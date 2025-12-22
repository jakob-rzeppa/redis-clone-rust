use std::io::{Read, Write};
use std::net::TcpStream;

const EOT: u8 = 0x04;

#[derive(Debug)]
pub struct Response {
    pub version: u8,
    pub command: u8,
    pub status_code: u16,
    pub content: Vec<u8>,
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:6379").expect("connect failed");

    println!("Successfully connected to server on port 6379");

    let mut msg: Vec<u8> = Vec::new();

    msg.push(1); // version
    msg.push(1); // set
    msg.extend_from_slice(9u16.to_be_bytes().as_slice()); // content_length

    // content
    msg.extend_from_slice(1u32.to_be_bytes().as_slice()); // id 1
    msg.extend_from_slice(b"hello");

    msg.push(EOT); // eot character

    println!("Set request for id 1");

    stream.write_all(&msg).expect("write failed");

    // Read fixed-size header
    let mut header = [0u8; 6];
    stream.read_exact(&mut header).expect("read header failed");

    let version = header[0];
    let command = header[1];
    let status_code = u16::from_be_bytes([header[2], header[3]]);
    let content_length = u16::from_be_bytes([header[4], header[5]]) as usize;

    // Read content
    let mut content = vec![0u8; content_length];
    stream.read_exact(&mut content).expect("read content failed");

    // Read EOT
    let mut eot = [0u8; 1];
    stream.read_exact(&mut eot).expect("read eot failed");

    if eot[0] != EOT {
        panic!("eot not match");
    }

    let response = Response {
        version,
        command,
        status_code,
        content,
    };

    println!("{:#?}", response);
}
use tokio::net::TcpStream;
use crate::connection::read_content::read_content;
use crate::connection::read_header::read_header;
use crate::controller::route_request;
use crate::types::{Command, Request};

pub(super) async fn handle_connection(mut stream: TcpStream) {
    loop {
        // read header
        let header_data = match read_header(&mut stream).await {
            Ok(header_data) => header_data,
            Err(e) => {
                eprintln!("read header failed: {}", e);
                println!("close connection");
                return;
            }
        };

        // read content
        let content: Option<Vec<u8>>;
        if header_data.command == Command::Set || header_data.command == Command::Insert {
            content = match read_content(&mut stream, header_data.content_length as usize).await {
                Ok(v) => Some(v),
                Err(e) => {
                    eprintln!("read content failed: {}", e);
                    println!("close connection");
                    return;
                }
            };
        } else {
            content = None;
        }

        let request = Request {
            version: header_data.version,
            command: header_data.command,
            content_length: header_data.content_length,
            content: match content {
                None => None,
                Some(v) => Some(v),
            },
        };

        route_request(request).await;
    }
}
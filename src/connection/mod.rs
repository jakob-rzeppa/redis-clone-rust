mod metadata;
mod header;
mod content;

use tokio::net::{TcpListener};

use header::{read_header, HeaderData};
use crate::connection::content::read_content;

enum Request<'a> {
    Get { id: u32 },
    Set { id: u32, content: &'a [u8] },
    Insert { content: &'a [u8] },
    Remove { id: u32 },
}

#[derive(Debug, PartialEq)]
enum Command {
    Get,
    Set,
    Insert,
    Remove
}

pub async fn listen_for_connections(tcp_listener: TcpListener) {
    loop {
        // accept connections and pass TcpStream to handle_connection
        let (mut stream, _) = match tcp_listener.accept().await {
            Ok(res) => res,
            Err(e) => {
                eprintln!("accept failed: {}", e);
                continue;
            }
        };

        tokio::spawn(async move {
            loop {
                let header_data = match read_header(&mut stream).await {
                    Ok(header_data) => header_data,
                    Err(e) => {
                        eprintln!("read header failed: {}", e);
                        println!("close connection");
                        return;
                    }
                };

                let request = match header_data {
                    HeaderData::Get { id } => {
                        Request::Get { id }
                    }
                    HeaderData::Remove { id } => {
                        Request::Remove { id }
                    }
                    HeaderData::Set { id, content_length } => {
                        // the request content still needs to be read
                        let content: Vec<u8> = match read_content(&mut stream, content_length as usize).await {
                            Ok(content) => content,
                            Err(e) => {
                                eprintln!("read set content failed: {}", e);
                                println!("close connection");
                                return;
                            }
                        };

                        Request::Set { id, content: &content.clone() }
                    },
                    HeaderData::Insert { content_length }  => {
                        // the request content still needs to be read
                        let content: Vec<u8> = match read_content(&mut stream, content_length as usize).await {
                            Ok(content) => content,
                            Err(e) => {
                                eprintln!("read insert content failed: {}", e);
                                println!("close connection");
                                return;
                            }
                        };

                        Request::Insert { content: &content.clone() }
                    }
                };

                // TODO handle the request
            }
        });
    }
}

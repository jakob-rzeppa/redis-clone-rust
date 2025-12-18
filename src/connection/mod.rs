mod metadata;
mod header;
mod content;

use tokio::net::{TcpListener};
use tokio::io::{AsyncReadExt, BufReader};

use header::{read_header, HeaderData};
use crate::connection::content::read_content;

enum RequestData<'a> {
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
        let (stream, _) = match tcp_listener.accept().await {
            Ok(res) => res,
            Err(e) => {
                eprintln!("accept failed: {}", e);
                continue;
            }
        };

        tokio::spawn(async move {
            // Wrap the stream in a BufReader, so we can use the BufRead methods
            let mut reader = BufReader::new(stream);

            loop {
                let header_data = match read_header(&mut reader).await {
                    Ok(header_data) => header_data,
                    Err(e) => {
                        eprintln!("read header failed: {}", e);
                        println!("close connection");
                        return;
                    }
                };

                let request = match header_data {
                    HeaderData::Get { id } => {
                        RequestData::Get { id }
                    }
                    HeaderData::Remove { id } => {
                        RequestData::Remove { id }
                    }
                    HeaderData::Set { id, content_length } => {
                        // the request content still needs to be read
                        let content: Vec<u8> = match read_content(&mut reader, content_length as usize).await {
                            Ok(content) => content,
                            Err(e) => {
                                eprintln!("read set content failed: {}", e);
                                println!("close connection");
                                return;
                            }
                        };

                        RequestData::Set { id, content: &content.clone() }
                    },
                    HeaderData::Insert { content_length }  => {
                        // the request content still needs to be read
                        let content: Vec<u8> = match read_content(&mut reader, content_length as usize).await {
                            Ok(content) => content,
                            Err(e) => {
                                eprintln!("read insert content failed: {}", e);
                                println!("close connection");
                                return;
                            }
                        };

                        RequestData::Insert { content: &content.clone() }
                    }
                };

                // TODO handle the request
            }
        });
    }
}

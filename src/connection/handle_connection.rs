use crate::connection::read_content::read_content;
use crate::connection::read_header::read_header;
use crate::connection::send_response::send_response;
use crate::controller::route_request;
use crate::types::{Command, Request, Response, StatusCode};

pub(super) async fn handle_connection<S>(mut stream: S)
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    loop {
        // read header
        let header_data = match read_header(&mut stream).await {
            Ok(header_data) => header_data,
            Err(e) => {
                eprintln!("read header failed: {}", e);
                // don't return a response and let the client reconnect with a new connection

                // break / close connection to make sure that there are no leftover bytes in the stream from this request
                break;
            }
        };

        // read content
        let content: Option<Vec<u8>>;
        if header_data.command == Command::Set || header_data.command == Command::Insert {
            content = match read_content(&mut stream, header_data.content_length as usize).await {
                Ok(v) => Some(v),
                Err(e) => {
                    eprintln!("read content failed: {}", e);
                    if let Err(e) = send_response(Response {
                        version: header_data.version,
                        command: header_data.command,
                        status_code: StatusCode::InvalidRequest,
                        content_length: 0,
                        content: None,
                    }, &mut stream).await {
                        eprintln!("sending response failed: {}", e);
                        continue; // skip to wait for next request
                    };
                    // break / close connection to make sure that there are no leftover bytes in the stream from this request
                    break;
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

        let response = route_request(request).await;

        match send_response(response, &mut stream).await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("sending response failed: {}", e);
                continue; // skip to wait for next request
            }
        };
    }
    println!("close connection");
}
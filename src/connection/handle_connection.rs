use crate::connection::read_content::read_content;
use crate::connection::read_header::read_header;
use crate::connection::send_response::send_response;
use crate::controller::route_request;
use crate::types::{Request};

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
        let content = match read_content(&mut stream, header_data.content_length as usize).await {
            Ok(v) => v,
            Err(e) => {
                eprintln!("read content failed: {}", e);
                // don't return a response and let the client reconnect with a new connection

                // break / close connection to make sure that there are no leftover bytes in the stream from this request
                break;
            }
        };

        let request = Request {
            version: header_data.version,
            command: header_data.command,
            content_length: header_data.content_length,
            content: if content.is_empty() { None } else { Some(content) },
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
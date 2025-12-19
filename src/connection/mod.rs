mod read_header;
mod read_content;
mod send_response;
mod handle_connection;

use tokio::net::{TcpListener};
use crate::connection::handle_connection::handle_connection;

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
            handle_connection(stream).await;
        });
    }
}

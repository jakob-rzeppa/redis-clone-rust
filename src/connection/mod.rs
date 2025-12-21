mod read_header;
mod read_content;
mod send_response;
mod handle_connection;

use tokio::net::{TcpListener};
use crate::connection::handle_connection::handle_connection;
use crate::repository::SharedRepository;

pub async fn listen_for_connections(tcp_listener: TcpListener, db: SharedRepository) {
    loop {
        // accept connections and pass TcpStream to handle_connection
        let (stream, _) = match tcp_listener.accept().await {
            Ok(res) => res,
            Err(e) => {
                eprintln!("accept failed: {}", e);
                continue;
            }
        };

        let local_db = db.clone();

        tokio::spawn(async move {
            handle_connection(stream, local_db).await;
        });
    }
}

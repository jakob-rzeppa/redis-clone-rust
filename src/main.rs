mod connection;
mod controller;
mod types;
mod util;
mod repository;

use tokio::net::{TcpListener};
use crate::connection::listen_for_connections;

#[tokio::main]
async fn main() {
    // panics if bind fails
    let listener = TcpListener::bind("127.0.0.1:6379").await.expect("bind failed");

    listen_for_connections(listener).await;
}
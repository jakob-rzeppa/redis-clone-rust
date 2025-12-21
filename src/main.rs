mod connection;
mod controller;
mod types;
mod util;
mod repository;

use std::sync::Arc;
use tokio::net::{TcpListener};
use crate::connection::listen_for_connections;
use crate::repository::{Repository, SharedRepository};

#[tokio::main]
async fn main() {
    let db: SharedRepository = Arc::new(Repository::new());
    // panics if bind fails
    let listener = TcpListener::bind("127.0.0.1:6379").await.expect("bind failed");

    listen_for_connections(listener, db).await;
}
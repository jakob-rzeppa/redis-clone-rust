use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use anyhow::Result;
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Command, Connection, Frame};
use bytes::Bytes;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    // panics if bind fails
    let listener = TcpListener::bind("127.0.0.1:6379").await.expect("bind failed");

    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // accept connections and pass TcpStream to handle_connection
        let (socket, _) = match listener.accept().await {
            Ok(res) => res,
            Err(e) => {
                println!("accept failed: {}", e);
                continue;
            }
        };

        // the db needs to be cloned before spawning the task so the task won't take ownership of the db
        let db = db.clone();
        tokio::spawn(async move {
            handle_connection(socket, db).await;
        });
    }
}

async fn handle_connection(socket: TcpStream, db: Db) {
    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);

    loop {
        // get the frame and exit the loop if connection closed or something goes wrong
        match connection.read_frame().await {
            Ok(Some(frame)) => {
                println!("GOT: {:?}", frame);

                let command = match Command::from_frame(frame) {
                    Ok(c) => c,
                    Err(e) => {
                        println!("invalid command: {:?}", e);
                        continue;
                    }
                };

                let response_frame = create_response(command, &db).unwrap_or_else(|e| {
                    Frame::Error(e.to_string())
                });

                // Write the response to the client
                match connection.write_frame(&response_frame).await {
                    Ok(_) => {
                        println!("SENT: {:?}", response_frame);
                    }
                    Err(e) => {
                        println!("sending response failed: {:?}", e);
                    }
                };
            }
            Ok(None) => {
                println!("connection closed");
                break;
            }
            _ => {
                println!("something went wrong when reading frame");
                break;
            }
        }
    }
}

fn create_response(command: Command, db: &Db) -> Result<Frame> {
    use mini_redis::Command::{Get, Set};

    // get access to the db and lock it for other tasks
    let mut db = match db.lock() {
        Ok(db) => db,
        Err(_) => {
            return Err(anyhow::Error::msg("something went wrong while locking db"));
        }
    };

    match command {
        Set(cmd) => {
            db.insert(cmd.key().to_string(), cmd.value().clone());
            Ok(Frame::Simple("OK".to_string()))
        },
        Get(cmd) => {
            if let Some(value) = db.get(cmd.key()) {
                // `Frame::Bulk` expects data to be of type `Bytes`. This
                // type will be covered later in the tutorial. For now,
                // `&Vec<u8>` is converted to `Bytes` using `into()`.
                Ok(Frame::Bulk(value.clone().into()))
            } else {
                Ok(Frame::Null)
            }
        },
        _ => {
            Err(anyhow::Error::msg("command not found"))
        }
    }
}
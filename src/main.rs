use std::collections::HashMap;
use anyhow::Result;
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Command, Connection, Frame};

#[tokio::main]
async fn main() {
    // panics if bind fails
    let listener = TcpListener::bind("127.0.0.1:6379").await.expect("bind failed");

    loop {
        // accept connections and pass TcpStream to handle_connection
        let (socket, _) = match listener.accept().await {
            Ok(res) => res,
            Err(e) => {
                println!("accept failed: {}", e);
                continue;
            }
        };

        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            // The move keyword moves the used variables (socket) to the task.
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(socket: TcpStream) {
    use std::collections::HashMap;

    // store data in hashmap
    let mut db: HashMap<String, Vec<u8>> = HashMap::new();

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

                let response_frame = create_response(command, &mut db).await.unwrap_or_else(|e| {
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

async fn create_response(command: Command, db: &mut HashMap<String, Vec<u8>>) -> Result<Frame> {
    use mini_redis::Command::{Get, Set};

    match command {
        Set(cmd) => {
            db.insert(cmd.key().to_string(), cmd.value().to_vec());
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
        cmd => {
            Err(anyhow::Error::msg("command not found"))
        }
    }
}
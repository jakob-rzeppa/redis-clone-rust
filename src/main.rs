use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

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
    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);

    match connection.read_frame().await {
        Ok(Some(frame)) => {
            println!("GOT: {:?}", frame);

            // Respond with an error
            let response = Frame::Error("unimplemented".to_string());
            connection.write_frame(&response).await.unwrap();
        }
        _ => {
            println!("something went wrong when reading frame");
        }
    }
}
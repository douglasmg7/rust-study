use bytes::Bytes;
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;
// type ShardedDb = Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;

#[tokio::main]
async fn main() {
    let mut listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // The second item contains the ip and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        // Clone the handle.
        let db = db.clone();

        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            proccess(socket, db).await;
        });
    }
}

async fn proccess(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    // The `Connection` lets us read/write redis **frames** instead of byte streams. The `Connection` type is defined by mini-redis.
    // Connection, provided by `mini-redis`, handles parsing frames from the socket.
    let mut connection = Connection::new(socket);

    // Use `read_frame` to receive a command from the connection.
    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);
        // // Respond with a error.
        // let response = Frame::Error("Unimplemented".to_string());
        // connection.write_frame(&response).await.unwrap();
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("uninplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}
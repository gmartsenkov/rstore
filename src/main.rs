use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::io::BufReader;
use rstore::db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = db::create();
    let mut listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        let db_clone = Arc::clone(&db);
        let mut socket = BufReader::new(socket);
        
        tokio::spawn(async move {
            let mut line = String::new();
            socket.read_line(&mut line).await.unwrap();

            match rstore::process(&db_clone, line.as_str()) {
                Err(_) => {},
                Ok(result) => {
                    socket.write_all(result.as_bytes()).await.unwrap();
                }
            }
        });
    }
}

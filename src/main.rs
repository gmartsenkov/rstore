use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::io::BufReader;
use std::sync::Mutex;
use std::sync::mpsc::{Sender, Receiver, channel};
use rstore::{db, logger};
use rstore::memory_spy::MemorySpy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::init();
    let (sender, receiver) : (Sender<db::DbStats>, Receiver<db::DbStats>) = channel();
    let stats_sender = Mutex::new(sender);
    let db = db::create(stats_sender);
    let memspy = MemorySpy::init();
    let mut listener = TcpListener::bind("127.0.0.1:8080").await?;
    logger::info!("Listening on port: 8080");

    loop {
        let (socket, _) = listener.accept().await?;
        let db_clone = Arc::clone(&db);
        let memspy_clone = Arc::clone(&memspy);
        let mut socket = BufReader::new(socket);
        
        tokio::spawn(async move {
            let mut line = String::new();
            socket.read_line(&mut line).await.unwrap();

            match rstore::process(&db_clone, line.as_str()) {
                Err(_) => {
                    logger::warn!("NOT FOUND: {}", line);
                },
                Ok(result) => {
                    logger::info!("REQUEST: {}", line);
                    socket.write_all(result.as_bytes()).await.unwrap();
                }
            }
        });
    }
}

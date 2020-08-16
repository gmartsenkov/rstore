use std::sync::{Arc, Mutex};
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::io::BufReader;


lazy_static! {
    static ref SET_REGEX : Regex = Regex::new("SET\\(KEY: \"(.*)\", VALUE: \"(.*)\"\\)").unwrap();
    static ref GET_REGEX : Regex = Regex::new("GET\\(KEY: \"(.*)\"\\)").unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db : Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        let db_clone = Arc::clone(&db);
        let mut socket = BufReader::new(socket);
        
        tokio::spawn(async move {
            let mut line = String::new();
            socket.read_line(&mut line).await.unwrap();

            if SET_REGEX.is_match(line.as_str()) {
                let captures = SET_REGEX.captures(line.as_str()).unwrap();
                let key = &captures[1];
                let value = &captures[2];

                db_clone.lock().unwrap().insert(String::from(key), String::from(value));
                socket.write_all("OK\n".as_bytes()).await.unwrap();
            };

            if GET_REGEX.is_match(line.as_str()) {
                let captures = GET_REGEX.captures(line.as_str()).unwrap();
                let key = &captures[1];
                let mut result = String::new();

                 match db_clone.lock().unwrap().get(key) {
                    None => {},
                    Some(value) => { result = String::from(value) }
                };

                socket.write_all(result.as_bytes()).await.unwrap();
            };
        });
    }
}

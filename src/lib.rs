pub mod logger;
pub mod db;
pub mod memory_spy;
mod commands;

pub fn process(database : &db::Db, data : &str) -> Result<String, u8> {
    for command in commands::COMMANDS.iter() {
        match command(database, data) {
            Some(v) => { return Ok(v); },
            None => {}
        }
    }

    Err(0)
}

#[cfg(test)]
mod tests {

    use std::sync::Mutex;
    use std::sync::mpsc::{Sender, Receiver, channel};
    use crate::db::{DbStats, create};
    use super::*;
    
    #[test]
    fn test_process_command_not_found() {
        let (sender, receiver) : (Sender<DbStats>, Receiver<DbStats>) = channel();
        let stats_sender = Mutex::new(sender);
        let db = create(stats_sender);

        assert_eq!(
            process(&db, "INVALID"),
            Err(0)
        )
    }

    #[test]
    fn test_process_get_command() {
        let (sender, receiver) : (Sender<DbStats>, Receiver<DbStats>) = channel();
        let stats_sender = Mutex::new(sender);
        let db = create(stats_sender);

        db::insert(&db, "TEST", "VALUE");

        assert_eq!(
            process(&db, "GET(KEY: \"TEST\")"),
            Ok("VALUE".to_string())
        )
    }

    #[test]
    fn test_process_set_command() {
        let (sender, receiver) : (Sender<DbStats>, Receiver<DbStats>) = channel();
        let stats_sender = Mutex::new(sender);
        let db = create(stats_sender);

        assert_eq!(
            process(&db, "SET(KEY: \"TEST\", VALUE: \"BMW\")"),
            Ok("BMW".to_string())
        )
    }
}

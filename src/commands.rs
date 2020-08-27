use regex::Regex;
use lazy_static::lazy_static;
use crate::db;

lazy_static! {
    static ref SET_REGEX : Regex = Regex::new("SET\\(KEY: \"(.*)\", VALUE: \"(.*)\"\\)").unwrap();
    static ref GET_REGEX : Regex = Regex::new("GET\\(KEY: \"(.*)\"\\)").unwrap();
}

type Command = fn(&db::Db, &str) -> Option<String>;
pub const COMMANDS : [Command; 2] = [
    get_command,
    set_command
];

fn get_command(database : &db::Db, data : &str) -> Option<String> {
    if GET_REGEX.is_match(data) {
        let captures = GET_REGEX.captures(data).unwrap();
        let key = &captures[1];

        return db::read(database, key);
    }

    None
}

fn set_command(database : &db::Db, data : &str) -> Option<String> {
    if SET_REGEX.is_match(data) {
        let captures = SET_REGEX.captures(data).unwrap();
        let key = &captures[1];
        let value = &captures[2];

        match db::insert(database, key, value) {
            None => { return Some(value.to_string()) }
            Some(v) => { return Some(v) },
        }
    };

    None
}

#[cfg(test)]
mod tests {

    use std::sync::Mutex;
    use std::sync::mpsc::{Sender, Receiver, channel};
    use crate::db::{DbStats, create};
    use super::*;
    
    #[test]
    fn test_get_command_when_regex_does_not_match() {
        let (sender, receiver) : (Sender<DbStats>, Receiver<DbStats>) = channel();
        let stats_sender = Mutex::new(sender);
        let db = create(stats_sender);

        assert_eq!(
            get_command(&db, "INVALID"),
            None
        )
    }

    #[test]
    fn test_get_command_when_key_does_not_exist() {
        let (sender, receiver) : (Sender<DbStats>, Receiver<DbStats>) = channel();
        let stats_sender = Mutex::new(sender);
        let db = create(stats_sender);

        assert_eq!(
            get_command(&db, "GET(KEY: \"TEST\")"),
            None
        )
    }

    #[test]
    fn test_get_command_when_key_exists() {
        let (sender, receiver) : (Sender<DbStats>, Receiver<DbStats>) = channel();
        let stats_sender = Mutex::new(sender);
        let db = create(stats_sender);

        db::insert(&db, "TEST", "VALUE");

        assert_eq!(
            get_command(&db, "GET(KEY: \"TEST\")"),
            Some("VALUE".to_string())
        )
    }

    #[test]
    fn test_set_command_when_regex_does_not_match() {
        let (sender, receiver) : (Sender<DbStats>, Receiver<DbStats>) = channel();
        let stats_sender = Mutex::new(sender);
        let db = create(stats_sender);

        assert_eq!(
            set_command(&db, "INVALID"),
            None
        )
    }

    #[test]
    fn test_set_command_writes_to_the_db() {
        let (sender, receiver) : (Sender<DbStats>, Receiver<DbStats>) = channel();
        let stats_sender = Mutex::new(sender);
        let db = create(stats_sender);

        assert_eq!(
            set_command(&db, "SET(KEY: \"TEST\", VALUE: \"BMW\")"),
            Some("BMW".to_string())
        );

        assert_eq!(
            db::read(&db, "TEST"),
            Some("BMW".to_string())
        )
    }
}

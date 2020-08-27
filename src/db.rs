use std::sync::{Arc,RwLock,Mutex};
use std::sync::mpsc::Sender;
use std::collections::HashMap;
use crate::logger;

pub (super) type Db = Arc<RwLock<DbEntity>>;

#[derive(Debug, PartialEq)]
pub struct DbStats {
    key_bytes : isize,
    value_bytes : isize
}

pub struct DbEntity {
    db : HashMap<String, String>,
    stats_sender : Mutex<Sender<DbStats>>
}

pub fn create(stats_sender : Mutex<Sender<DbStats>>) -> Db {
    Arc::new(
        RwLock::new(
            DbEntity{db: HashMap::new(), stats_sender}
        )
    )
}

pub (super) fn read(database : &Db, key : &str) -> Option<String> {
    match database.read().unwrap().db.get(key) {
        None => {
            logger::trace!("STORE READ: {}, VALUE: NOT FOUND", key);
            None
        },
        Some(v) => {
            logger::trace!("STORE READ: {}, VALUE: {}", key, v);
            Some(v.clone())
        }
    }
}

pub (super) fn insert(database : &Db, key : &str, value : &str) -> Option<String> {
    let mut db = database.write().unwrap();
    let result = match db.db.insert(key.to_string(), value.to_string()) {
        None => {
            db.stats_sender
              .lock()
              .unwrap()
              .send(DbStats{key_bytes: key.len() as isize, value_bytes: value.len() as isize})
              .unwrap();

            None
        },
        Some(v) => {
            db.stats_sender
              .lock()
              .unwrap()
              .send(DbStats{key_bytes: 0, value_bytes: (v.len() as isize - value.len() as isize)})
              .unwrap();
            Some(v.clone())
        }
    };

    logger::trace!("STORE WRITE: {}, VALUE: {}", key, value);
    result
}

#[cfg(test)]
mod tests {

    use std::sync::mpsc::{Sender, Receiver, channel};
    use super::*;

    #[test]
    fn test_read_when_entry_does_not_exist() {
        let (sender, _receiver) : (Sender<DbStats>, Receiver<DbStats>) = channel();
        let stats_sender = Mutex::new(sender);
        let db = create(stats_sender);

        assert_eq!(read(&db, "key"), None);
    }

    #[test]
    fn test_read_when_entry_exists() {
        let (sender, _receiver) : (Sender<DbStats>, Receiver<DbStats>) = channel();
        let stats_sender = Mutex::new(sender);
        let db = create(stats_sender);

        insert(&db, "key", "value");
        assert_eq!(read(&db, "key"), Some("value".to_string()));
    }

    #[test]
    fn test_write() {
        let (sender, receiver) : (Sender<DbStats>, Receiver<DbStats>) = channel();
        let stats_sender = Mutex::new(sender);
        let db = create(stats_sender);

        assert_eq!(
            insert(&db, "key", "value"),
            None
        );

        assert_eq!(
            receiver.recv().unwrap(),
            DbStats{key_bytes: 3, value_bytes: 5}
        )
    }

    #[test]
    fn test_write_overwrite() {
        let (sender, receiver) : (Sender<DbStats>, Receiver<DbStats>) = channel();
        let stats_sender = Mutex::new(sender);
        let db = create(stats_sender);

        insert(&db, "key", "value");

        assert_eq!(
            receiver.recv().unwrap(),
            DbStats{key_bytes: 3, value_bytes: 5}
        );

        assert_eq!(
            insert(&db, "key", "new value"),
            Some("value".to_string())
        );

        assert_eq!(
            receiver.recv().unwrap(),
            DbStats{key_bytes: 0, value_bytes: -4}
        );

        assert_eq!(read(&db, "key"), Some("new value".to_string()))
    }
}

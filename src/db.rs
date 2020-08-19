use std::sync::{Arc,RwLock};
use std::collections::HashMap;
use crate::logger;

pub (super) type Db = Arc<RwLock<HashMap<String, String>>>;
    
pub fn create() -> Db {
    Arc::new(RwLock::new(HashMap::new()))
}

pub (super) fn read(database : &Db, key : &str) -> Option<String> {
    match database.read().unwrap().get(key) {
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
    let result = match database.write().unwrap().insert(key.to_string(), value.to_string()) {
        None => None,
        Some(v) => Some(v.clone())
    };

    logger::trace!("STORE WRITE: {}, VALUE: {}", key, value);
    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read_when_entry_does_not_exist() {
        let db = create();

        assert_eq!(read(&db, "key"), None);
    }

    #[test]
    fn test_read_when_entry_exists() {
        let db = create();

        insert(&db, "key", "value");
        assert_eq!(read(&db, "key"), Some("value".to_string()));
    }

    #[test]
    fn test_write() {
        let db = create();

        assert_eq!(
            insert(&db, "key", "value"),
            None
        )
    }

    #[test]
    fn test_write_overwrite() {
        let db = create();

        insert(&db, "key", "value");
        assert_eq!(
            insert(&db, "key", "new value"),
            Some("value".to_string())
        );
        assert_eq!(read(&db, "key"), Some("new value".to_string()))
    }
}

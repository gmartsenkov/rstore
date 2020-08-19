use std::sync::{Arc,RwLock};
use std::collections::HashMap;

pub (super) type Db = Arc<RwLock<HashMap<String, String>>>;
    
pub fn create() -> Db {
    Arc::new(RwLock::new(HashMap::new()))
}

pub (super) fn read(database : &Db, key : &str) -> Option<String> {
    match database.read().unwrap().get(key) {
        None => return None,
        Some(v) => return Some(v.clone())
    }
}

pub (super) fn insert(database : &Db, key : &str, value : &str) -> Option<String> {
    match database.write().unwrap().insert(key.to_string(), value.to_string()) {
        None => return None,
        Some(v) => return Some(v.clone())
    }
}

use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref SET_REGEX : Regex = Regex::new("SET\\(KEY: \"(.*)\", VALUE: \"(.*)\"\\)").unwrap();
    static ref GET_REGEX : Regex = Regex::new("GET\\(KEY: \"(.*)\"\\)").unwrap();
}

pub mod db;

pub fn process(database : &db::Db, data : &str) -> Result<String, u8> {
    if SET_REGEX.is_match(data) {
        let captures = SET_REGEX.captures(data).unwrap();
        let key = &captures[1];
        let value = &captures[2];

        db::insert(database, key, value);
        return Ok("OK".to_string());
    };

    if GET_REGEX.is_match(data) {
        let captures = GET_REGEX.captures(data).unwrap();
        let key = &captures[1];

        match db::read(database, key) {
            None => {},
            Some(value) => {
                return Ok(value.clone());
            }
        };
    };


    return Err(0)
}

pub mod db;
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

    use super::*;
    
    #[test]
    fn test_process_command_not_found() {
        let db = db::create();

        assert_eq!(
            process(&db, "INVALID"),
            Err(0)
        )
    }

    #[test]
    fn test_process_get_command() {
        let db = db::create();

        db::insert(&db, "TEST", "VALUE");

        assert_eq!(
            process(&db, "GET(KEY: \"TEST\")"),
            Ok("VALUE".to_string())
        )
    }

    #[test]
    fn test_process_set_command() {
        let db = db::create();

        assert_eq!(
            process(&db, "SET(KEY: \"TEST\", VALUE: \"BMW\")"),
            Ok("BMW".to_string())
        )
    }
}

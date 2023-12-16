use crate::db::DataStore;
///
/// client repl
///
use anyhow::Result;
use std::io::{self, Write};

/// split the given message into two parts, i.e, cmd and params or key and value
pub fn split2(msg: &str) -> (String, String) {
    let mut split = msg.split_whitespace();
    let head = split.next().unwrap_or("");
    let mut tail = String::new();
    for s in split {
        tail.push_str(s);
        tail.push(' ');
    }

    (head.to_string(), tail.trim().to_string())
}

/// show the help for this repl
pub fn help(startup: bool) -> String {
    let mut buf = format!("{} Version: {}.\n\n", "Tiny K/V REPL", crate::VERSION);

    if startup {
        buf.push_str("Enter 'quit' to exit or 'help' for a list of commands.")
    } else {
        buf.push_str("Commands:\n");
        buf.push_str("  get key\n");
        buf.push_str("  set key value\n");
        buf.push_str("  remove key\n");
        buf.push_str("  dbsize\n");
        buf.push_str("  keys\n");
        buf.push_str("  loaddb filename\n");
        buf.push_str("  savedb filename\n");
        buf.push_str("  quit\n");
        buf.push_str("  help\n");
    }

    buf.push_str("\nNote: this REPL is for string types only...\n");

    buf
}

#[derive(Debug, Default)]
pub struct Client {
    db: DataStore,
}

impl Client {
    /// create with a data store, possibly loaded with data
    pub fn create(db: DataStore) -> Client {
        Client { db }
    }

    /// start the repl loop
    pub fn start(&mut self) -> Result<()> {
        println!("{}", help(true));
        let mut ln = 0;
        loop {
            ln += 1;
            print!("{} > ", ln);
            let _ = io::stdout().flush();
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");

            if input.starts_with("quit") {
                break;
            }

            let (cmd, params) = split2(&input);
            match cmd.as_str() {
                "help" => println!("{}", help(false)),
                "dbsize" => println!("{}", self.db.dbsize()),
                "keys" => println!("{:?}", self.db.keys()),
                "get" => {
                    if let Some(value) = self.db.get(&params) {
                        let val = value.clone();
                        let sval = String::from_utf8(val).unwrap();
                        println!("{:?} -> {}", value, sval);
                    } else {
                        println!("error");
                    }
                }
                "set" => {
                    let (key, value) = split2(&params);
                    if let Some(value) = self.db.set(&key, value.into_bytes()) {
                        let val = value.clone();
                        let sval = String::from_utf8(val).unwrap();
                        println!("{:?} -> {}", value, sval);
                    } else {
                        println!("ok");
                    }
                }
                "remove" => {
                    if let Some(value) = self.db.remove(&params) {
                        let val = value.clone();
                        let sval = String::from_utf8(val).unwrap();
                        println!("{:?} -> {}", value, sval);
                    } else {
                        println!("ok");
                    }
                }
                "loaddb" => {
                    if let Ok(sz) = self.db.loaddb(&params) {
                        println!("loaded {} records.", sz);
                    } else {
                        println!("error");
                    }
                }
                "savedb" => {
                    if let Ok(sz) = self.db.savedb(&params) {
                        println!("saved {} records to {}.", sz, &params);
                    } else {
                        println!("error");
                    }
                }
                _ => println!("not ready for {} yet", cmd),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_client() {
        let db = DataStore::create();
        let client = Client::create(db);
        println!("{:?}", client);
        assert_eq!(client.db.dbsize(), 0);
    }

    #[test]
    fn test_split2() {
        let ss = "set mykey my long value with other stuff";
        let (cmd, params) = split2(ss);
        assert_eq!(cmd, "set");
        assert!(params.starts_with("mykey"));
    }

    #[test]
    fn test_help() {
        let hlp = help(true);
        println!("{}", hlp);
        let hlp = help(false);
        println!("{}", hlp);
    }
}

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

/// write the prompt # > then read the next repl command from stdin
fn read_input(line_num: usize, prompt: &str) -> String {
    println!("{}{} ", line_num, prompt);
    let _ = io::stdout().flush();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}

#[derive(Debug, Clone)]
pub struct Client {
    db: DataStore,
    prompter: fn(usize, &str) -> String,
}

impl Client {
    /// create with a data store, possibly loaded with data
    pub fn create(db: DataStore) -> Client {
        Client {
            db,
            prompter: read_input,
        }
    }

    /// start the repl loop
    pub fn start(&mut self) -> Result<()> {
        println!("{}", help(true));
        let mut ln = 0;
        loop {
            ln += 1;
            let input = (self.prompter)(ln, " >");

            if input.starts_with("quit") {
                break;
            }

            self.process_command(&input);
        }

        Ok(())
    }

    /// process the command line from prompt
    fn process_command(&mut self, input: &str) {
        let (cmd, params) = split2(input);
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
}

#[cfg(test)]
mod tests {
    use super::*;

    // create a reader for each command
    fn mock_prompt(ln: usize, prompt: &str) -> String {
        println!("{}{}", ln, prompt);
        let _ = io::stdout().flush();

        std::thread::sleep(std::time::Duration::from_millis(10));

        match ln {
            1 => "set u999 dpw".to_string(),
            2 => "keys".to_string(),
            3 => "dbsize".to_string(),
            4 => "get u999".to_string(),
            5 => "remove u999".to_string(),
            6 => "dbsize".to_string(),
            _ => "quit".to_string(),
        }
    }

    #[test]
    fn create_client() {
        let db = DataStore::create();
        let client = Client::create(db);
        println!("{:?}", client);
        assert_eq!(client.db.dbsize(), 0);
    }

    #[test]
    fn test_loop() {
        let db = DataStore::create();
        let mut client = Client {
            db,
            prompter: mock_prompt,
        };

        let resp = client.start();
        assert!(resp.is_ok());
    }

    #[test]
    fn test_commands() {
        let db = DataStore::create();
        let mut client = Client::create(db);

        client.process_command("dbsize");
        client.process_command("keys");
        client.process_command("help");
        client.process_command("set u101");
        client.process_command("set u101 my thing");
        client.process_command("get u101");
        client.process_command("remove u101");
        client.process_command("remove u101");
        client.process_command("get u101");
        client.process_command("flarb");
        client.process_command("loaddb bad-file");
        client.process_command("loaddb tests/users-ref.kv");
        client.process_command("savedb /my/bad/file/thing.kv");
        client.process_command("savedb tests/repl-save-out.kv");
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

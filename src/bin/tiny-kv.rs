use std::io::{self, Write};
///
/// create a repl for set get remove keys dbsize loaddb savedb
///
use tiny_kv::{db::DataStore, VERSION};

fn split2(msg: &str) -> (String, String) {
    let mut split = msg.split_whitespace();
    let head = split.next().unwrap_or("");
    let mut tail = String::new();
    for s in split {
        tail.push_str(s);
        tail.push(' ');
    }

    (head.to_string(), tail.trim().to_string())
}

fn help(startup: bool) -> String {
    let mut buf = format!("{} Version: {}.\n\n", "Tiny K/V REPL", VERSION);

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

fn start() {
    let mut store = DataStore::create();

    let mut ln = 0;
    println!("{}", help(true));
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
            "dbsize" => println!("{}", store.dbsize()),
            "keys" => println!("{:?}", store.keys()),
            "get" => {
                if let Some(value) = store.get(&params) {
                    let val = value.clone();
                    let sval = String::from_utf8(val).unwrap();
                    println!("{:?} -> {}", value, sval);
                } else {
                    println!("error");
                }
            }
            "set" => {
                let (key, value) = split2(&params);
                if let Some(value) = store.set(&key, value.into_bytes()) {
                    let val = value.clone();
                    let sval = String::from_utf8(val).unwrap();
                    println!("{:?} -> {}", value, sval);
                } else {
                    println!("ok");
                }
            }
            "remove" => {
                if let Some(value) = store.remove(&params) {
                    let val = value.clone();
                    let sval = String::from_utf8(val).unwrap();
                    println!("{:?} -> {}", value, sval);
                } else {
                    println!("ok");
                }
            }
            "loaddb" => {
                if let Ok(sz) = store.loaddb(&params) {
                    println!("loaded {} records.", sz);
                } else {
                    println!("error");
                }
            }
            "savedb" => {
                if let Ok(sz) = store.savedb(&params) {
                    println!("saved {} records to {}.", sz, &params);
                } else {
                    println!("error");
                }
            }
            _ => println!("not ready for {} yet", cmd),
        }
    }
}

fn main() {
    start()
}

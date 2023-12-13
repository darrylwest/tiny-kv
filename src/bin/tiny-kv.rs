use std::io::{self, Write};
///
/// create a repl for set get remove keys dbsize loaddb savedb
///
use tiny_kv::db::DataStore;

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

fn start() {
    let mut store = DataStore::create();

    let mut ln = 0;
    println!("Enter 'quit' or ctrl-c to exit...");
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
            // remove
            // savedb
            // loaddb
            _ => println!("not ready for {} yet", cmd),
        }

        // println!("{}", input);
        // store.set("mykey", input.into_bytes());
        // let v = store.get("mykey").unwrap();
        // println!("{:?} -> {}", &v, String::from_utf8(v.clone()).unwrap());
    }
}

fn main() {
    start()
}

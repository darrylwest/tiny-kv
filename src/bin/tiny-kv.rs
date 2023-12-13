use std::io::{self, Write};
///
/// create a repl for set get remove keys dbsize loaddb savedb
///
use tiny_kv::db::DataStore;

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

        println!("{}", input);
        store.set("mykey", input.into_bytes());
        let v = store.get("mykey").unwrap();
        println!("{:?} -> {}", &v, String::from_utf8(v.clone()).unwrap());
    }
}

fn main() {
    start()
}

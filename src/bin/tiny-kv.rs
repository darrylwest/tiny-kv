///
/// create a repl for set get remove keys dbsize loaddb savedb
///
use tiny_kv::{db::DataStore, VERSION};
use clap::Parser;
use std::io::{self, Write};
use std::env;

#[derive(Debug, Default, Parser)]
#[command(
    name="udp-client",
    author,
    version,
    about="A repl client for udp-server backed by tiny-kv.",
    long_about=None,
)]
struct Cli {
    /// config filename to override default
    #[arg(short, long)]
    data_file: Option<String>,
}

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

fn start(args: Vec<String>) {
    let cli = Cli::parse_from(args);
    let mut store = DataStore::create();

    if cli.data_file.is_some() {
        let filename = cli.data_file.unwrap();
        let n = store.loaddb(filename.as_str()).expect("data file should exist...");
        println!("{:?} records loaded", n);
    }

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
    let args: Vec<String> = env::args().collect();
    start(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_client() {
        assert!(true)
    }

    #[test]
    fn test_split2() {
        let ss = "set mykey my long value with other stuff";
        let (cmd, params) = split2(ss);
        assert_eq!(cmd, "set");
        assert!(params.starts_with("mykey"));
    }
}

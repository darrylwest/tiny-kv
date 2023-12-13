use anyhow::Result;
use clap::Parser;
use std::env;
///
/// create a repl for set get remove keys dbsize loaddb savedb
///
use tiny_kv::{client::Client, db::DataStore};

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

/// create the client repl
fn create_client(args: Vec<String>) -> Result<Client> {
    let cli = Cli::parse_from(args);
    let store = DataStore::create();

    if cli.data_file.is_some() {
        let filename = cli.data_file.unwrap();
        let n = store.loaddb(filename.as_str())?;
        println!("Startup loaded {:?} items...", n);
    }

    Ok(Client::create(store))
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut repl = create_client(args)?;
    let _ = repl.start();

    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_create_client() {
        assert!(true)
    }
}

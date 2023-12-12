///
/// file operations
///
use anyhow::Result;
use hashbrown::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// read the k/v file contents and return a map
pub fn kv_file2map(filename: &str) -> Result<HashMap<String, String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut map: HashMap<String, String> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let split = line.split_once(' ').unwrap();
        let (key, value) = split;
        map.insert(key.to_string(), value.to_string());
    }

    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kv_file2map() {
        let filename = "./tests/users-ref.kv";
        let kv = kv_file2map(filename).unwrap();

        println!("kv: {:?}", kv);
        assert!(kv.len() >= 10);
    }
}

///
/// file operations
///
use anyhow::Result;
use hashbrown::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

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

/// write the map data to disk
pub fn map2kv_file(filename: &str, map: HashMap<String, String>) -> Result<usize> {
    let mut buf = File::create(filename)?;
    let mut sz: usize = 0;

    for (k, v) in map.iter() {
        let line = format!("{} {}\n", k, v);
        let resp = buf.write_all(line.as_bytes());
        if resp.is_ok() {
            sz += 1;
        }
    }

    Ok(sz)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_ref_kv_file() -> HashMap<String, String> {
        let filename = "./tests/users-ref.kv";
        kv_file2map(filename).unwrap()
    }

    #[test]
    fn test_kv_file2map() {
        let filename = "./tests/users-ref.kv";
        let kv = kv_file2map(filename).unwrap();

        println!("kv: {:?}", kv);
        assert!(kv.len() >= 10);
    }

    #[test]
    fn test_map2kv_file() {
        let kv = read_ref_kv_file();
        let kvsz = kv.len();

        let resp = map2kv_file("./tests/users-out.kv", kv);
        assert!(resp.is_ok());
        let sz = resp.unwrap();
        assert_eq!(kvsz, sz);
    }
}

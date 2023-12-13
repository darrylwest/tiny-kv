///
/// file operations
///
use anyhow::Result;
use hashbrown::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/// read the k/v file contents and return a map
pub fn kv_file2map(filename: &str) -> Result<HashMap<String, Vec<u8>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut map: HashMap<String, Vec<u8>> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let split = line.split_once(' ').unwrap();
        let (key, value) = split;
        let val = value.as_bytes().to_vec();
        map.insert(key.to_string(), val);
    }

    Ok(map)
}

/// write the map data to disk
pub fn map2kv_file(filename: &str, map: HashMap<String, Vec<u8>>) -> Result<usize> {
    let mut buf = File::create(filename)?;
    let mut sz: usize = 0;
    let space = 0x20_u8;

    for (k, v) in map.iter() {
        let _ = buf.write_all(k.as_bytes());
        let mut line: Vec<u8> = vec![space];
        for b in v {
            line.push(*b);
        }
        line.push(0x0D);
        line.push(0x0A);

        let resp = buf.write_all(line.as_slice());
        if resp.is_ok() {
            sz += 1;
        }
    }

    Ok(sz)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_ref_kv_file() -> HashMap<String, Vec<u8>> {
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

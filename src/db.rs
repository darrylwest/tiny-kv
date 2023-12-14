///
///
use anyhow::Result;
use hashbrown::HashMap;
// use std::thread;
use crate::file_ops;
use log::info;
use std::sync::{Arc, RwLock};

#[derive(Debug, Default, Clone)]
pub struct DataStore {
    map: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl DataStore {
    /// create teh data store
    pub fn create() -> DataStore {
        DataStore {
            map: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// set the value for this k/v pair
    pub fn set(&mut self, key: &str, value: Vec<u8>) -> Option<Vec<u8>> {
        // Get a write lock and insert the pair into the map
        let mut map = self.map.write().unwrap();
        map.insert(key.to_string(), value.to_owned())
    }

    /// return the value for the given key
    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        // Get a read lock and get the value from the map
        let map = self.map.read().unwrap();
        map.get(key).cloned()
    }

    /// remove the value for this key
    pub fn remove(&mut self, key: &str) -> Option<Vec<u8>> {
        // Get a write lock and remove the value from the map
        let mut map = self.map.write().unwrap();
        map.remove(key)
    }

    /// return a list of keys
    pub fn keys(&self) -> Vec<String> {
        let map = self.map.read().unwrap();
        map.keys().cloned().collect::<Vec<_>>()
    }

    /// return the number of elements
    pub fn dbsize(&self) -> usize {
        // Get a read lock and get the length of the map
        let map = self.map.read().unwrap();
        map.len()
    }

    /// save the database and return the file size
    pub fn savedb(&self, filename: &str) -> Result<usize> {
        info!("save db to {}", filename);
        let map = self.map.read().unwrap();
        let sz = file_ops::map2kv_file(filename, map.clone())?;

        Ok(sz)
    }

    /// load data from the specified filename; return the number of elements read in
    pub fn loaddb(&self, filename: &str) -> Result<usize> {
        info!("read db from {}", filename);
        let kv = file_ops::kv_file2map(filename)?;

        let mut map = self.map.write().unwrap();
        for (k, v) in kv.iter() {
            let _ = map.insert(k.to_string(), v.clone());
        }

        Ok(map.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_store() -> DataStore {
        DataStore::create()
    }

    #[test]
    fn loaddb() {
        let filename = "tests/users-ref.kv";
        let store = create_store();
        assert!(store.loaddb(filename).unwrap() >= 10);
    }

    #[test]
    fn savedb() {
        let ref_file = "tests/users-ref.kv";
        let store = create_store();
        assert!(store.loaddb(ref_file).unwrap() >= 10);
        let dbsize = store.dbsize();

        let sz = store.savedb("./tests/kvdb-out.kv");
        assert!(sz.is_ok());
        assert_eq!(dbsize, sz.unwrap());
    }

    #[test]
    fn set_get_remove() {
        let mut store = create_store();
        assert_eq!(store.dbsize(), 0);
        let key = "mykey";
        let value = "this is my value".as_bytes().to_vec();

        let resp = store.set(key, value.clone());
        assert!(resp.is_none());
        assert_eq!(store.dbsize(), 1);

        let keys = store.keys();
        println!("{:?}", keys);
        assert_eq!(keys.len(), store.dbsize());

        let resp = store.get(key).unwrap();
        assert_eq!(resp, value);

        let resp = store.get("not a valid key");
        assert!(resp.is_none());

        let resp = store.remove(key);
        assert!(resp.is_some());

        assert_eq!(store.dbsize(), 0);
    }

    #[test]
    fn create() {
        let store = DataStore::create();
        assert_eq!(store.dbsize(), 0);
    }
}

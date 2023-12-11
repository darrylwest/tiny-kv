///
///
use anyhow::Result;
use hashbrown::HashMap;
// use std::thread;
use log::info;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct DataStore {
    map: Arc<RwLock<HashMap<String, String>>>,
}

impl DataStore {
    /// create teh data store
    pub fn create() -> DataStore {
        DataStore {
            map: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// set the value for this k/v pair
    pub fn set(&mut self, key: &str, value: &str) -> Option<String> {
        // Get a write lock and insert the pair into the map
        let mut map = self.map.write().unwrap();
        map.insert(key.to_string(), value.to_string())
    }

    /// return the value for the given key
    pub fn get(&self, key: &str) -> Option<String> {
        // Get a read lock and get the value from the map
        let map = self.map.read().unwrap();
        map.get(key).cloned()
    }

    /// remove the value for this key
    pub fn remove(&mut self, key: &str) -> Option<String> {
        // Get a write lock and remove the value from the map
        let mut map = self.map.write().unwrap();
        map.remove(key)
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
        Ok(0_usize)
    }

    /// load data from the specified filename; return the number of elements read in
    pub fn loaddb(&self, filename: &str) -> Result<usize> {
        info!("read db from {}", filename);
        Ok(0_usize)
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
        let filename = "/tmp/tiny-kv.data";
        let store = create_store();
        assert_eq!(store.loaddb(filename).unwrap(), 0);
    }

    #[test]
    fn savedb() {
        let filename = "/tmp/tiny-kv.data";
        let store = create_store();
        assert_eq!(store.savedb(filename).unwrap(), 0);
    }

    #[test]
    fn set_get_remove() {
        let mut store = create_store();
        assert_eq!(store.dbsize(), 0);
        let key = "mykey";
        let value = "this is my value";

        let resp = store.set(key, value);
        assert!(resp.is_none());
        assert_eq!(store.dbsize(), 1);

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

///
///
// use anyhow::Result;
// use std::sync::{Arc, RwLock};
// use std::thread;
use hashbrown::HashMap;

pub struct DataStore {
    map: HashMap<String, String>,
}

impl DataStore {
    /// create teh data store
    pub fn create() -> DataStore {
        DataStore {
            map: HashMap::new(),
        }
    }

    /// set the value for this k/v pair
    pub fn set(&mut self, key: &str, value: &str) -> Option<String> {
        self.map.insert(key.to_string(), value.to_string())
    }

    /// return the value for the given key
    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    /// remove the value for this key
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.map.remove(key)
    }

    /// return the number of elements
    pub fn dbsize(&self) -> usize {
        self.map.len()
    }

    /// save the database and return the file size
    pub fn savedb(&self) -> usize {
        0_usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_store() -> DataStore {
        DataStore::create()
    }

    #[test]
    fn savedb() {
        let store = create_store();
        assert_eq!(store.savedb(), 0);
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
        assert_eq!(store.map.len(), 0);
        assert_eq!(store.dbsize(), 0);
    }
}

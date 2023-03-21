use std::collections::HashMap;

pub struct KvStore {
    map: HashMap<String, String>
}
impl KvStore {
    pub fn new() -> KvStore {
        KvStore { map: HashMap::new() }
    }

    pub fn set(&mut self, key: String, value: String) -> () {
        println!("Setting '{}' to '{}'", key, value);
        self.map.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        println!("Getting value for '{}'", key);
        self.map.get(&key).cloned()
    }

    pub fn remove(&mut self, key: String) -> () {
        println!("Removed value for '{}'", key);
        self.map.remove(&key);
    }
}
pub struct KvStore;
impl KvStore {
    pub fn new() -> KvStore {
        panic!("Abort, abort, abort");
    }

    pub fn set(&self, key: String, value: String) -> () {
        println!("Setting '{}' to '{}'", key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        println!("Getting value for '{}'", key);
        None
    }

    pub fn remove(&self, key: String) -> () {
        println!("Removed value for '{}'", key);
    }
}
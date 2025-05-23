use std::collections::HashMap;

pub struct KeyValueStore {
    store: HashMap<String, String>,
}

impl KeyValueStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn put(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    pub fn delete(&mut self, key: &str) {
        self.store.remove(key);
    }
}
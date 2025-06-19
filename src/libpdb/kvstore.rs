use std::collections::HashMap;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct KvStore {
	pub file: String,
    pub store: HashMap<String, String>,
}

#[allow(dead_code)]
impl KvStore {
    pub fn new(file: String) -> Self {
        Self {
        	file: file.to_string(),
        	store: HashMap::new() 
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    pub fn delete(&mut self, key: &str) -> Option<String> {
        self.store.remove(key)
    }
}

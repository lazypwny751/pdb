use std::{ io, collections::HashMap };
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
struct KvStore {
    store: HashMap<String, String>,
}

#[allow(dead_code)]
impl KvStore {
    fn new() -> Self {
        Self { store: HashMap::new() }
    }

    fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    fn delete(&mut self, key: &str) -> Option<String> {
        self.store.remove(key)
    }
}

fn main() -> io::Result<()> {
	Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get_delete() {
        let mut db = KvStore::new();

        db.set("foo".to_string(), "bar".to_string());
        db.set("baz".to_string(), "qux".to_string());

        assert_eq!(db.get("foo"), Some(&"bar".to_string()));
        assert_eq!(db.get("baz"), Some(&"qux".to_string()));

        let deleted = db.delete("foo");
        assert_eq!(deleted, Some("bar".to_string()));
        assert_eq!(db.get("foo"), None);
    }
}

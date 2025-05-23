mod persistence;
use persistence::key_value_store::KeyValueStore;

fn get_and_print(store: &KeyValueStore, key: &str) {
    match store.get("my_key") {
        Some(value) => println!("'{}' => '{}'", key, value),
        _ => println!("'{}' not found", key)
    }
}

fn main() {
    let mut kv_store = KeyValueStore::new();
    let my_key = "my_key";
    
    get_and_print(&kv_store, my_key);
    
    kv_store.put(String::from(my_key), String::from("my_value"));
    
    get_and_print(&kv_store, my_key);
    
    kv_store.delete(my_key);
    
    get_and_print(&kv_store, my_key);
}
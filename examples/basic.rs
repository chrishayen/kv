use kv::Store;

fn main() {
    let store = Store::new();
    store.put("one", "value").unwrap();
    store.put("butt", "butt".to_string()).unwrap();
    store.put("two", 100000).unwrap();
    store.put("one", "value").unwrap();
}

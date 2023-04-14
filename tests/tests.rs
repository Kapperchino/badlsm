use bytes::*;
use lsm::*;

#[test]
fn test() {
    let mut store = DiskStore::default();
    let table = vec![
        ("key1", new_value(42)),
        ("key2", new_value(52)),
        ("key3", new_value(62)),
        ("key5", Bytes::from(format!("{:0102400}", 1))),
        ("key4", new_value(72)),
    ];

    for (key, value) in &table {
        store.write(key_with_ts(key, 0), value.clone());
    }
    println!("Yoo {}", store.mem_table.len());
    store.flush();
    //TODO: fix search
    let val = store.read(Bytes::from("key1"));
    println!("Yoo {:?}", val.unwrap().to_ascii_lowercase());
}

fn key_with_ts(key: &str, ts: u64) -> Bytes {
    Bytes::from(format!("{}{:08}", key, ts))
}

fn new_value(v: usize) -> Bytes {
    Bytes::from(format!("{:05}", v))
}
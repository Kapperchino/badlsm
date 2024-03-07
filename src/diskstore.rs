use std::collections::BTreeMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use bytes::Bytes;

pub struct ShardedTree {
    trees: [BTreeMap<Bytes, Bytes>; 4],
}

impl ShardedTree {
    pub fn put(&self, key: Bytes, val: Bytes) {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let i = hasher.finish();
        self.trees[i % 4].insert(key, val);
    }
    pub fn get(&self, key: Bytes) -> Option<Bytes> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let i = hasher.finish();
        return self.trees[i % 4].get(key);
    }
}
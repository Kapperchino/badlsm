use std::collections::BTreeMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use bytes::Bytes;

pub struct ShardedTree {
    trees: Vec<BTreeMap<Bytes, Bytes>>,
    shards: usize,
}

impl ShardedTree {
    pub fn new(shards: usize) -> Self {
        let mut maps: Vec<BTreeMap<Bytes, Bytes>> = Vec::with_capacity(shards);
        for _ in 0..shards {
            maps.push(BTreeMap::new());
        }
        ShardedTree {
            shards,
            trees: maps,
        }
    }
    pub fn put(&mut self, key: Bytes, val: Bytes, shard: u32) {
        self.trees[shard].insert(key, val);
    }
    pub fn get(&self, key: Bytes, shard: u32) -> Option<Bytes> {
        return self.trees[shard].get(key);
    }
}
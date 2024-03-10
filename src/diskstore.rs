use std::collections::BTreeMap;

pub struct LSMTree {
    pub tree: BTreeMap<Vec<u8>, Vec<u8>>,
    shard_id: usize,
}

impl LSMTree {
    pub fn new(shard_id: usize) -> Self {
        LSMTree {
            shard_id,
            tree: BTreeMap::new(),
        }
    }
    pub fn put(&mut self, key: Vec<u8>, val: Vec<u8>) {
        self.tree.insert(key, val);
    }
}
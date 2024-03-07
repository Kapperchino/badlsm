use std::collections::BTreeMap;
use bytes::Bytes;

pub struct ShardedTree {
    trees: [BTreeMap<Bytes, Bytes>; 4],
}
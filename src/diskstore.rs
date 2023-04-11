use skiplist::{FixedLengthSuffixComparator, Skiplist};

pub trait LSM {
    fn get(&self, key: &[u8]);
    fn put(&self, key: &[u8], val: &[u8]);
}

pub struct DiskStore {
    pub mem_table: Skiplist<FixedLengthSuffixComparator>,
}

pub struct SSTable {}

const ARENA_SIZE: usize = 1 << 20;

impl Default for DiskStore {
    fn default() -> Self {
        let comp = FixedLengthSuffixComparator::new(8);
        let list = Skiplist::with_capacity(comp, ARENA_SIZE, true);
        Self {
            mem_table: list
        }
    }
}
use std::{fs::File, io::Write};

use bytes::{BufMut, Bytes};
use prost::Message;
use skiplist::{FixedLengthSuffixComparator, Skiplist};

use crate::protos;

const ARENA_SIZE: usize = 1 << 20;
const MEM_TABLE_SIZE: usize = 100;
const MAX_TABLE_LEVELS: usize = 3;

pub struct DiskStore {
    pub mem_table: Skiplist<FixedLengthSuffixComparator>,
    pub sstables: Vec<SSTable>,
    pub numLv: usize,
    chunkIndex: u32,
}

pub struct SSTable {
    path: String,
}

fn new_memtable() -> Skiplist<FixedLengthSuffixComparator>{
    let comp = FixedLengthSuffixComparator::new(8);
    return Skiplist::with_capacity(comp, ARENA_SIZE, true);
}

impl DiskStore {
    pub fn write(&mut self, key: Bytes, val: Bytes) {
        if self.sstables.len() >= MEM_TABLE_SIZE {
            self.flush();
        }
        self.mem_table.put(key, val);
    }

    //TODO
    pub fn read(&mut self, key: Bytes) -> Option<&Bytes>{
        return self.mem_table.get(&key);
    }
    

    pub fn flush(&mut self) {
        let mut iter = self.mem_table.iter();
        let mut chunk = protos::Chunk { list: Vec::new() };
        iter.seek_to_first();
        while iter.valid() {
            let key = iter.key();
            let val = iter.value();
            let kv = protos::Kv {
                key: key.to_vec(),
                val: val.to_vec(),
            };
            chunk.list.push(kv);
            iter.next();
        }
        // Create a new file or overwrite an existing one
        let mut file = match File::create(self.chunkIndex.to_string()) {
            Ok(file) => file,
            Err(error) => panic!("Unable to create file: {:?}", error),
        };
        let mut buf = Vec::new();
        buf.reserve(chunk.encoded_len());
        chunk.encode(&mut buf).unwrap();
        match file.write_all(&buf) {
            Ok(()) => println!("Data written to file successfully."),
            Err(error) => panic!("Unable to write to file: {:?}", error),
        };
        self.mem_table = new_memtable();
        self.chunkIndex += 1;
    }

}

impl Default for DiskStore {
    fn default() -> Self {
        let list = new_memtable();
        let mut vec: Vec<SSTable> = Vec::new();
        vec.push(SSTable {
            path: String::from("./0"),
        });
        Self {
            mem_table: list,
            sstables: Vec::new(),
            numLv: 1,
            chunkIndex: 0,
        }
    }
}

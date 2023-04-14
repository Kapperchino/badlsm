use std::{
    fs::File,
    io::{Read, Write},
};

use bytes::{buf, BufMut, Bytes, BytesMut};
use prost::Message;
use skiplist::{FixedLengthSuffixComparator, Skiplist};

use crate::protos;

const ARENA_SIZE: usize = 1 << 20;
const MEM_TABLE_SIZE: usize = 100;
const MAX_TABLE_LEVELS: usize = 3;

pub struct DiskStore {
    pub mem_table: Skiplist<FixedLengthSuffixComparator>,
    pub sstables: Vec<SSTable>,
    chunk_index: u32,
}

pub struct SSTable {
    path: String,
    file: File,
}

impl SSTable {
    fn new(path: String) -> Self {
        let file = match File::create(&path) {
            Ok(file) => file,
            Err(error) => panic!("Unable to create file: {:?}", error),
        };
        SSTable {
            path: path,
            file: file,
        }
    }

    fn write(&mut self, bytes: Bytes) {
        match self.file.write_all(&bytes) {
            Ok(()) => println!("Data written to file successfully."),
            Err(error) => panic!("Unable to write to file: {:?}", error),
        };
    }

    //TODO: use deliminated proto
    fn search(&self, key: &Bytes) -> Option<Bytes> {
        let mut buf = BytesMut::new();
        let mut file_ref = &self.file;
        let size = file_ref.read(&mut buf).ok()?;
        if size == 0 {
            return None;
        }
        let final_buf = buf.freeze();
        let chunk = protos::Chunk::decode(final_buf).unwrap();
        let index = chunk
            .list
            .binary_search_by(|kv| kv.key.cmp(&key.to_vec()))
            .ok()?;
        let res = &chunk.list[index].val;
        return Some(Bytes::from(res.to_owned()));
    }
}

fn new_memtable() -> Skiplist<FixedLengthSuffixComparator> {
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

    pub fn read(&mut self, key: Bytes) -> Option<Bytes> {
        let val = self.mem_table.get(&key);
        if !val.is_none() {
            return Some(val?.to_owned());
        }
        for table in self.sstables.iter().rev() {
            return Some(table.search(&key)?);
        }
        return None;
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
        let mut buffer = BytesMut::new();
        buffer.reserve(chunk.encoded_len());
        chunk.encode(&mut buffer).unwrap();
        let mut table = SSTable::new(self.chunk_index.to_string());
        table.write(buffer.freeze());
        self.mem_table = new_memtable();
        self.chunk_index += 1;
    }
}

impl Default for DiskStore {
    fn default() -> Self {
        let list = new_memtable();
        Self {
            mem_table: list,
            sstables: Vec::new(),
            chunk_index: 0,
        }
    }
}

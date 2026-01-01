use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Hash(Vec<u8>);

#[derive(Debug)]
pub enum G1t {
    Index { entries: Vec<Entry> },
}

#[derive(Debug)]
pub struct Entry {
    file_name: String,
}

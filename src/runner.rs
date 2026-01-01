use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Hash(Vec<u8>);

#[derive(Debug)]
pub struct BlobHash(Hash);

#[derive(Debug)]
pub struct G1t {
    index: Index,
    objects: Vec<Object>,
}

#[derive(Debug)]
pub struct Index {
    entries: Vec<Entry>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Entry {
    file_name: String,
    blob_hash: BlobHash,
}

#[derive(Debug)]
pub enum Object {
    Blob {
        hash: BlobHash,
        content: String,
    },
    Tree {
        hash: Hash,
        contents: Vec<(String, Hash)>,
    },
    Commit {
        hash: Hash,
        message: String,
        tree_hash: Hash,
        parent_commit: Option<Hash>,
    },
}

pub enum Cmd {
    Add { file_name: String },
}

pub struct Runner {
    g1t: G1t,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            g1t: G1t {
                index: Index::new(),
                objects: Vec::new(),
            },
        }
    }

    pub fn run(&mut self, cmd: Cmd) {
        match cmd {
            Cmd::Add { file_name } => {}
        }
    }
}

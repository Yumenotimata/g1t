use core::fmt;

use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use vfs::FileSystem;

#[derive(Clone)]
pub struct Hash(Vec<u8>);

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct BlobHash(Hash);

#[derive(Debug)]
pub struct G1t {
    index: Index,
    objects: Vec<Object>,
}

pub struct Content {
    file_name: String,
    content: String,
}

impl Content {
    pub fn new(file_name: String, content: String) -> Self {
        Self { file_name, content }
    }
}

pub trait Storage {
    fn index(&self) -> &Index;
    fn objects(&self) -> &Vec<Object>;
    fn modify_index(&mut self, modifier: Box<dyn FnOnce(&mut Index)>);
    fn add_entry(&mut self, entry: Entry);
    // return object hash and store object
    fn hash_object(&mut self, object: Object) -> Hash;

    // register file content to index, but not store object
    fn update_index(&mut self, content: Content);
}

pub struct JsonStorage {
    index: Index,
    objects: Vec<Object>,
    fs: Box<dyn FileSystem>,
}

impl JsonStorage {
    pub fn new(fs: Box<dyn FileSystem>) -> Self {
        Self {
            index: Index::new(),
            objects: Vec::new(),
            fs,
        }
    }
}

impl Storage for JsonStorage {
    fn index(&self) -> &Index {
        &self.index
    }

    fn objects(&self) -> &Vec<Object> {
        &self.objects
    }

    fn modify_index(&mut self, modifier: Box<dyn FnOnce(&mut Index)>) {
        modifier(&mut self.index);
    }

    fn add_entry(&mut self, entry: Entry) {
        self.index.entries.push(entry);
    }

    fn hash_object(&mut self, object: Object) -> Hash {
        let hash = object.hash();
        self.objects.push(object);
        hash
    }

    fn update_index(&mut self, content: Content) {
        let blob_hash = self.hash_object(Object::blob(content.content));

        self.index.entries.push(Entry {
            blob_hash: BlobHash(blob_hash),
            file_name: content.file_name,
        });
    }
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

impl Object {
    fn hash(&self) -> Hash {
        match self {
            Object::Blob { hash, .. } => hash.clone().0,
            Object::Tree { hash, .. } => hash.clone(),
            Object::Commit { hash, .. } => hash.clone(),
        }
    }

    fn blob(content: String) -> Self {
        let hash: Hash = Hash(Sha1::digest(content.clone()).to_vec());

        Object::Blob {
            hash: BlobHash(hash),
            content,
        }
    }
}

pub enum Cmd {
    Add { file_name: String },
}

pub struct Runner {
    pub storage: Box<dyn Storage>,
    fs: Box<dyn FileSystem>,
}

impl Runner {
    pub fn new(storage: Box<dyn Storage>, fs: Box<dyn FileSystem>) -> Self {
        Self { storage, fs }
    }

    pub fn run(&mut self, cmd: Cmd) {
        match cmd {
            Cmd::Add { file_name } => {
                if let Ok(mut file) = self.fs.open_file(&file_name) {
                    let mut content = String::new();
                    file.read_to_string(&mut content)
                        .unwrap();
                    println!("{:?}", content);

                    self.storage
                        .update_index(Content::new(file_name, content));
                } else {
                    eprintln!("File not found");
                }
            }
        }
    }
}

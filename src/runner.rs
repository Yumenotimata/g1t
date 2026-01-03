use core::fmt;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use vfs::FileSystem;

use crate::{FsMap, VfsWrapper};

#[derive(Clone, Serialize, Deserialize)]
pub struct Hash(pub Vec<u8>);

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobHash(Hash);

#[derive(Debug, Serialize, Deserialize)]
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
    pub fs: Box<dyn FileSystem>,
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

#[derive(Debug)]
pub struct FsMapedJson {
    pub index: Index,
    // pub objects: FsMap,
    pub fsw: VfsWrapper,
}

#[derive(Debug)]
pub enum FsMapedJsonError {
    AlreadyInitialized,
}

impl FsMapedJson {
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        // let index_path = self.mount.join("index.json");
        // let mut file = self.fs.create_file(index_path.to_str().unwrap()).unwrap();
        // file.write_all(serde_json::to_string(&self.index).unwrap().as_bytes())
        //     .unwrap();
        // Ok(())
        // let mut file =
        todo!()
    }

    pub fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // let index_path = self.mount.join("index.json");
        // let mut file = self.fs.open_file(index_path.to_str().unwrap()).unwrap();
        // let mut content = String::new();
        // file.read_to_string(&mut content).unwrap();
        // self.index = serde_json::from_str(&content).unwrap();
        Ok(())
    }

    pub fn new() -> Self {
        let fsw = VfsWrapper::new(vfs::PhysicalFS::new("./"));

        Self {
            index: Index::default(),
            // objects: FsMap::open("./.g1t/objects").unwrap(),
            fsw,
        }
    }

    pub fn init(&mut self) -> Result<(), FsMapedJsonError> {
        if self.fsw.exists("./.g1t") {
            return Err(FsMapedJsonError::AlreadyInitialized);
        }

        let mut index_file = self.fsw.create_file("./.g1t/index.json");
        index_file.write(serde_json::to_string(&Index::default()).unwrap().as_bytes());

        self.fsw.create_dir_all("./.g1t/objects");

        Ok(())
    }

    pub fn update_index(&mut self, content: Content) -> Result<(), Box<dyn std::error::Error>> {
        let hash = self.hash_object(Object::blob(content.content))?;
        self.index.entries.push(Entry {
            file_name: content.file_name,
            blob_hash: BlobHash(hash),
        });
        self.save()?;
        Ok(())
    }

    pub fn hash_object(&mut self, object: Object) -> Result<Hash, Box<dyn std::error::Error>> {
        // let hash = object.hash();
        // self.objects.insert(
        //     hash.clone(),
        //     serde_json::to_string(&object).unwrap(),
        //     &mut self.fs,
        // );
        // Ok(hash)
        todo!()
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Index {
    entries: Vec<Entry>,
}

impl Default for Index {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

impl Index {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    file_name: String,
    blob_hash: BlobHash,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Object {
    Blob {
        hash: BlobHash,
        content: String,
    },
    Tree {
        hash: Hash,
        contents: Vec<(PathBuf, ObjectMode, Hash)>,
    },
    Commit {
        hash: Hash,
        message: String,
        tree_hash: Hash,
        parent_commit: Option<Hash>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectMode {
    Blob,
    Tree,
    Commit,
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

    fn tree(contents: Vec<(PathBuf, Hash)>) -> Self {
        let hash: Hash = Hash(
            Sha1::digest(format!(
                "{:?}",
                contents.iter().map(|c| c.1.clone()).collect::<Vec<_>>()
            ))
            .to_vec(),
        );

        Object::Tree {
            hash: hash.clone(),
            contents: contents
                .iter()
                .map(|c| (c.0.clone(), ObjectMode::Blob, c.1.clone()))
                .collect(),
        }
    }

    fn commit(message: String, tree_hash: Hash) -> Self {
        let hash: Hash = Hash(Sha1::digest(message.clone()).to_vec());

        Object::Commit {
            hash: hash.clone(),
            message,
            tree_hash,
            parent_commit: None,
        }
    }
}

pub enum Cmd {
    Add { file_name: String },
    Commit { message: String },
    Init,
}

#[derive(Debug)]
pub struct Runner {
    // pub storage: Box<dyn Storage>,
    pub storage: FsMapedJson,
    fs: Box<dyn FileSystem>,
}

impl Runner {
    // pub fn new(storage: FsMapedJson, fs: Box<dyn FileSystem>) -> Self {
    //     Self { storage, fs }
    // }

    pub fn new() -> Self {
        Self {
            storage: FsMapedJson::new(),
            fs: Box::new(vfs::PhysicalFS::new("./")),
        }
    }

    pub fn run(&mut self, cmd: Cmd) -> Result<(), FsMapedJsonError> {
        match cmd {
            Cmd::Add { file_name } => {
                // if let Ok(mut file) = self.fs.open_file(&file_name) {
                //     let mut content = String::new();
                //     file.read_to_string(&mut content)?;
                //     println!("{}", content);

                //     self.storage
                //         .update_index(Content::new(file_name, content))?;
                // } else {
                //     println!("File not found");
                // }
            }
            Cmd::Commit { message } => {
                // let tree: Vec<(PathBuf, Hash)> = self
                //     .storage
                //     .index()
                //     .entries
                //     .iter()
                //     .map(|entry| {
                //         (
                //             PathBuf::from(&entry.file_name),
                //             entry.blob_hash.0.clone(),
                //         )
                //     })
                //     .collect();

                // let tree = Object::tree(tree);
                // self.storage.hash_object(tree.clone());

                // let commit = Object::commit(message, tree.hash());
                // self.storage.hash_object(commit);
            }
            Cmd::Init => {
                self.storage.init()?;
            }
        }

        Ok(())
    }
}

// impl Runner {
//     pub fn new(storage: Box<dyn Storage>, fs: Box<dyn FileSystem>) -> Self {
//         Self { storage, fs }
//     }

//     pub fn run(&mut self, cmd: Cmd) {
//         match cmd {
//             Cmd::Add { file_name } => {
//                 if let Ok(mut file) = self.fs.open_file(&file_name) {
//                     let mut content = String::new();
//                     file.read_to_string(&mut content)
//                         .unwrap();
//                     println!("{:?}", content);

//                     self.storage
//                         .update_index(Content::new(file_name, content));
//                 } else {
//                     eprintln!("File not found");
//                 }
//             }
//             Cmd::Commit { message } => {
//                 let tree: Vec<(PathBuf, Hash)> = self
//                     .storage
//                     .index()
//                     .entries
//                     .iter()
//                     .map(|entry| {
//                         (
//                             PathBuf::from(&entry.file_name),
//                             entry.blob_hash.0.clone(),
//                         )
//                     })
//                     .collect();

//                 let tree = Object::tree(tree);
//                 self.storage.hash_object(tree.clone());

//                 let commit = Object::commit(message, tree.hash());
//                 self.storage.hash_object(commit);
//             }
//         }
//     }
// }

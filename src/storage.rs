use std::{fmt, path::PathBuf};

use serde::{Deserialize, Serialize};
use sha1::{
    Digest, Sha1,
    digest::{
        consts::{U20, U64, U160},
        generic_array::GenericArray,
    },
};

trait Storage {
    fn write(&self, object: Object);
    fn read(&self, object: Object);
}

pub enum Header {}

#[derive(Clone, Deserialize, Serialize)]
struct Hash(Vec<u8>);

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Hash {
    fn from_vec(vec: Vec<u8>) -> Self {
        Hash(vec)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Object {
    Blob {
        hash: Hash,
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
    fn blob(content: String) -> Self {
        let hash: Hash = Hash(Sha1::digest(content.clone()).to_vec());

        Object::Blob { hash, content }
    }

    fn tree(contents: Vec<(String, Hash)>) -> Self {
        Object::Tree {
            contents: contents.clone(),

            hash: contents[0].1.clone(),
        }
    }

    fn commit(
        message: String,
        tree_hash: Hash,
        parent_commit: Option<Hash>,
    ) -> Self {
        Object::Commit {
            hash: Hash(
                Sha1::digest(format!("{} {:?}", message, tree_hash)).to_vec(),
            ),
            message,
            tree_hash,
            parent_commit,
        }
    }

    fn hash(&self) -> Hash {
        match self {
            Object::Blob { hash, .. } => hash.clone(),
            Object::Tree { hash, .. } => hash.clone(),
            Object::Commit { hash, .. } => hash.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    blob_hash: Hash,
    file_name: String,
}

#[derive(Debug)]
pub struct AbsStorage {
    objects: Vec<Object>,
    index: Index,
    head: Option<Hash>,
}

impl Storage for AbsStorage {
    fn write(&self, object: Object) {}

    fn read(&self, object: Object) {}
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

impl AbsStorage {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            index: Index::new(),
            head: None,
        }
    }

    fn hash_object(&mut self, object: Object) -> Hash {
        let hash = object.hash();
        self.objects.push(object);
        hash
    }

    pub fn update_index(&mut self, contents: Vec<Content>) {
        for content in contents {
            let blob_hash = self.hash_object(Object::blob(content.content));
            self.index.entries.push(Entry {
                blob_hash,
                file_name: content.file_name,
            });
        }
    }

    pub fn commit(&mut self, message: impl Into<String>) {
        let mut contents = Vec::new();

        for entry in self.index.entries.iter() {
            let blob_hash = entry.blob_hash.clone();
            let file_name = entry.file_name.clone();

            contents.push((file_name, blob_hash));
        }

        let tree = Object::tree(contents);
        let tree_hash = self.hash_object(tree);

        let commit =
            Object::commit(message.into(), tree_hash, self.head.clone());

        self.head = Some(commit.hash());

        self.hash_object(commit);
    }
}

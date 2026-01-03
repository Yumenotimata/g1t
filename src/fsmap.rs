use std::path::PathBuf;

use vfs::{FileSystem, VfsFileType};

use crate::Hash;

#[derive(Debug)]
pub struct FsMap {
    pub mount: PathBuf,
}

#[derive(Debug)]
pub enum FsMapError {
    MountDirNotFound,
}

impl FsMap {
    pub fn open(mount: impl Into<PathBuf>) -> Result<Self, FsMapError> {
        let mount = mount.into();

        if mount.exists() {
            Ok(Self { mount })
        } else {
            Err(FsMapError::MountDirNotFound)
        }
    }

    pub fn insert(&mut self, key: Hash, value: impl Into<String>, fs: &mut Box<dyn FileSystem>) {
        let (dir, file) = key.0.split_at(1);

        let dir_name = dir
            .to_vec()
            .into_iter()
            .map(|b| b.to_string())
            .collect::<String>();
        let file_name = file
            .to_vec()
            .into_iter()
            .map(|b| b.to_string())
            .collect::<String>();

        let dir_path = self.mount.join(dir_name);
        let file_path = &dir_path.join(file_name).to_str().unwrap().to_string();
        let dir_path = dir_path.to_str().unwrap();

        // キーのハッシュが同じならファイルが存在するなら、中身も同じなので上書き処理は必要ない
        if fs.exists(file_path).unwrap() {
            return;
        }

        fs.create_dir(dir_path).unwrap();

        let mut file = fs.create_file(file_path).expect("unreachable");

        file.write_all(value.into().as_bytes())
            .expect("unreachable");
    }

    pub fn get(&self, key: Hash, fs: &mut impl FileSystem) -> Option<String> {
        let (dir, file) = key.0.split_at(1);
        // let dir_name = String::from_utf8_lossy(dir).to_string();
        // let file_name = String::from_utf8_lossy(file).to_string();

        let dir_name = dir
            .to_vec()
            .into_iter()
            .map(|b| b.to_string())
            .collect::<String>();
        let file_name = file
            .to_vec()
            .into_iter()
            .map(|b| b.to_string())
            .collect::<String>();
        let path = self.mount.join(dir_name).join(file_name);

        let file = fs.open_file(path.to_str().unwrap());
        if let Ok(mut file) = file {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            Some(content)
        } else {
            None
        }
    }

    pub fn get_all(&self, fs: &mut Box<dyn FileSystem>) -> Vec<String> {
        let mut result = Vec::new();

        let dir = self.mount.to_str().unwrap();

        for entry in ls_files(fs, dir) {
            let mut file = fs.open_file(&entry).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            result.push(content);
        }

        result
    }
}

fn ls_files(fs: &mut Box<dyn FileSystem>, dir: &str) -> Vec<String> {
    let mut result = Vec::new();

    for entry in fs.read_dir(dir).unwrap() {
        let path = PathBuf::from(dir).join(entry);
        let meta = fs.metadata(path.to_str().unwrap()).unwrap();
        match meta.file_type {
            VfsFileType::Directory => {
                result.extend(ls_files(fs, path.to_str().unwrap()));
            }
            VfsFileType::File => {
                result.push(path.to_str().unwrap().to_string());
            }
        }
    }

    result
}

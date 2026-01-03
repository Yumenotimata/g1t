use std::{
    fs::File,
    path::{Path, PathBuf},
};

use vfs::{SeekAndRead, SeekAndWrite};

pub struct VfsWrapper {
    fs: Box<dyn vfs::FileSystem>,
}

impl VfsWrapper {
    pub fn new(fs: impl vfs::FileSystem) -> Self {
        let fs = Box::new(fs);
        Self { fs }
    }

    pub fn create_dir_all(&self, path: &Path) {
        let ancestors = path
            .iter()
            .scan(PathBuf::new(), |acc, x| {
                acc.push(x);
                Some(acc.clone())
            })
            .map(|x| x.to_string_lossy().to_string());

        for ancestor in ancestors {
            if !self.fs.exists(&ancestor).unwrap() {
                self.fs.create_dir(&ancestor).unwrap();
            }
        }
    }

    pub fn create_file(&self, path: &Path) -> FileWriter {
        let parent = path.parent().unwrap();
        let file = path.to_string_lossy();

        self.create_dir_all(parent);

        let mut already_created = true;

        if !self.fs.exists(&file).unwrap() {
            self.fs.create_file(&file).unwrap();
            already_created = false;
        }

        let file = self.fs.append_file(&file).unwrap();

        FileWriter {
            file,
            already_created,
        }
    }
}

pub struct FileWriter {
    file: Box<dyn SeekAndWrite>,
    already_created: bool,
}

impl FileWriter {
    pub fn write(&mut self, buf: &[u8]) {
        self.file.write(buf).unwrap();
        self.file.flush().unwrap();
    }

    pub fn already_created(&self) -> bool {
        self.already_created
    }
}

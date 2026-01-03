use std::{
    io::Write,
    path::{Path, PathBuf},
};

pub struct SafeFsBuilder {
    fs: Box<dyn vfs::FileSystem>,
}

impl SafeFsBuilder {
    pub fn new(fs: impl vfs::FileSystem) -> Self {
        Self { fs: Box::new(fs) }
    }

    pub fn open(path: &Path) -> Result<SafeFs, ()> {
        if !path.exists() {
            return Err(());
        }

        Ok(SafeFs {
            root: path.to_path_buf(),
        })
    }
}

pub struct SafeFs {
    root: PathBuf,
}

impl SafeFs {
    pub fn open_or_create_name(&self, file_name: &str, f: Box<dyn FnOnce(&mut FileHandler)>) {
        let path = self.root.join(file_name);

        if !path.exists() {
            std::fs::File::create(&path).unwrap();
        }

        let file = std::fs::File::open(&path).unwrap();

        let mut file_handler = FileHandler { file };

        f(&mut file_handler);
    }

    pub fn open_or_create_dir(&self, dir_name: &str, f: Box<dyn FnOnce(&mut DirHandler)>) {
        let path = self.root.join(dir_name);

        if !path.exists() {
            std::fs::create_dir(&path).unwrap();
        }

        let mut dir = DirHandler { path };

        f(&mut dir);
    }
}

pub struct FileHandler {
    file: std::fs::File,
}

impl FileHandler {
    pub fn write(&mut self, data: &[u8]) -> &mut Self {
        self.file.write(data).unwrap();
        self
    }
}

pub struct DirHandler {
    path: PathBuf,
}

impl DirHandler {
    pub fn open_or_create_file(&self, file_name: &str) -> FileHandler {
        let path = self.path.join(file_name);

        if !path.exists() {
            std::fs::File::create(&path).unwrap();
        }

        let file = std::fs::File::open(&path).unwrap();

        FileHandler { file }
    }

    pub fn open_or_create_dir(&self, dir_name: &str) -> DirHandler {
        let path = self.path.join(dir_name);

        if !path.exists() {
            std::fs::create_dir(&path).unwrap();
        }

        DirHandler { path }
    }
}

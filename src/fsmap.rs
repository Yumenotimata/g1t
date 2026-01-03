use std::path::PathBuf;

use vfs::FileSystem;

use crate::Hash;

#[derive(Debug)]
pub struct FsMap {
    mount: PathBuf,
}

impl FsMap {
    pub fn new(mount: impl Into<PathBuf>) -> Self {
        Self {
            mount: mount.into(),
        }
    }

    pub fn insert(
        &mut self,
        key: Hash,
        value: impl Into<String>,
        fs: &mut Box<dyn FileSystem>,
    ) {
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

        let path = self
            .mount
            .join(dir_name)
            .join(file_name);

        fs.create_dir(
            self.mount
                .join(path.parent().unwrap())
                .to_str()
                .unwrap(),
        )
        .unwrap();

        let mut file = fs
            .create_file(path.to_str().unwrap())
            .unwrap();

        file.write_all(value.into().as_bytes())
            .unwrap();
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
        let path = self
            .mount
            .join(dir_name)
            .join(file_name);

        let file = fs.open_file(path.to_str().unwrap());
        if let Ok(mut file) = file {
            let mut content = String::new();
            file.read_to_string(&mut content)
                .unwrap();
            Some(content)
        } else {
            None
        }
    }
}

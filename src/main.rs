use std::path::PathBuf;

use clap::{Parser, Subcommand};
use g1t::{JsonStorage, Runner};
use vfs::{FileSystem, MemoryFS, VfsFileType};

#[derive(Debug)]
pub struct FsBuilder {
    root: Directory,
}

impl FsBuilder {
    pub fn new() -> Self {
        Self {
            root: Directory {
                path: PathBuf::from("/root"),
                contents: Vec::new(),
            },
        }
    }
}

impl FsBuilder {
    pub fn mkdir(
        mut self,
        directory_name: impl Into<String>,
        child_builder: impl FnOnce(FsBuilder) -> FsBuilder,
    ) -> Self {
        let directory_name = directory_name.into();
        let child_file_system = child_builder(FsBuilder::new());

        self.root.contents.push(Data::directory(
            directory_name.into(),
            child_file_system.root.contents,
        ));

        self
    }

    pub fn touch(
        mut self,
        file_name: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        self.root
            .contents
            .push(Data::file(file_name.into().into(), content.into()));
        self
    }

    pub fn build(self, mount_point: impl Into<String>) -> Box<dyn FileSystem> {
        let fs: Box<dyn FileSystem> = Box::new(MemoryFS::new());

        Self::build_rec(fs, &self.root.path.clone(), self.root.into())
    }

    fn build_rec_in(
        root: &PathBuf,
    ) -> impl FnMut(Box<dyn FileSystem>, Data) -> Box<dyn FileSystem> {
        move |fs, data| Self::build_rec(fs, root, data)
    }

    fn build_rec(
        mut fs: Box<dyn FileSystem>,
        root: &PathBuf,
        data: Data,
    ) -> Box<dyn FileSystem> {
        match data {
            Data::File { path, content } => {
                fs.create_file(
                    root.join(path.clone())
                        .to_str()
                        .unwrap(),
                )
                .unwrap();
                fs.append_file(root.join(path).to_str().unwrap())
                    .unwrap()
                    .write_all(content.as_bytes())
                    .unwrap();
            }
            Data::Directory(directory) => {
                let path = directory.path.clone();
                let contents = directory.contents.clone();

                fs.create_dir(
                    root.join(directory.path.clone())
                        .to_str()
                        .unwrap(),
                )
                .unwrap();

                fs = contents
                    .into_iter()
                    .fold(fs, Self::build_rec_in(&root.join(path.clone())));
            }
        }

        fs
    }
}

#[derive(Debug, Clone)]
pub enum Data {
    File { path: PathBuf, content: String },
    Directory(Directory),
}

#[derive(Debug, Clone)]
pub struct Directory {
    path: PathBuf,
    contents: Vec<Data>,
}

impl Directory {
    pub fn new(path: PathBuf, contents: Vec<Data>) -> Self {
        Self { path, contents }
    }
}

impl From<Directory> for Data {
    fn from(directory: Directory) -> Self {
        Self::Directory(directory)
    }
}

impl Data {
    pub fn file(path: PathBuf, content: String) -> Self {
        Self::File { path, content }
    }

    pub fn directory(path: PathBuf, contents: Vec<Data>) -> Self {
        Self::Directory(Directory::new(path, contents))
    }
}

fn main() {
    let mut fs_builder = FsBuilder::new();
    let fs = fs_builder
        .mkdir("test_dir", |builder| {
            builder
                .touch("test_file", "test_content")
                .mkdir("test_dir2", |builder| {
                    builder.touch("test_file2", "test_content2")
                })
        })
        .touch("test_file3", "test_content3");
    println!("{:#?}", fs);
    let fs = fs.build("/root");

    into_data(fs, &PathBuf::from("/root"));

    // let mut runner = Runner::new(
    //     Box::new(JsonStorage::new(Box::new(MemoryFS::new()))),
    //     Box::new(mfs),
    // );

    // println!(
    //     "{:?}\n{:#?}",
    //     runner.storage.index(),
    //     runner.storage.objects()
    // );
    // runner.run(g1t::Cmd::Add {
    //     file_name: "/test_dir/test_file".to_string(),
    // });
    // println!(
    //     "== \n{:?}\n{:#?}",
    //     runner.storage.index(),
    //     runner.storage.objects()
    // );
}

fn pretty(fs: Box<dyn FileSystem>) {
    let mut entries = fs.read_dir("/test_dir").unwrap();
    for entry in entries {
        println!("{:?}", entry);
    }
}

fn into_data(fs: Box<dyn FileSystem>, root: &PathBuf) -> Data {
    let mut entries = fs
        .read_dir(root.to_str().unwrap())
        .unwrap();

    for entry in entries {
        println!("{:?}", entry);
        // match fs
        //     .metadata(entry.as_str())
        //     .unwrap()
        //     .file_type
        // {
        //     VfsFileType::File => {}
        //     VfsFileType::Directory => {}
        // }
    }

    todo!()
}

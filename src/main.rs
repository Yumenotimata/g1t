use std::path::PathBuf;

use clap::{Parser, Subcommand};
use g1t::{JsonStorage, Runner};
use vfs::{FileSystem, MemoryFS};

#[derive(Debug)]
pub struct FsBuilder {
    datas: Vec<Data>,
}

impl FsBuilder {
    pub fn new() -> Self {
        Self { datas: Vec::new() }
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

        self.datas.push(Data::directory(
            directory_name.into(),
            child_file_system.datas,
        ));

        self
    }

    pub fn touch(
        mut self,
        file_name: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        self.datas
            .push(Data::file(file_name.into().into(), content.into()));
        self
    }

    pub fn build(self) -> Box<dyn FileSystem> {
        let fs: Box<dyn FileSystem> = Box::new(MemoryFS::new());

        self.datas
            .into_iter()
            .fold(fs, Self::build_rec_pure_in(&PathBuf::from("/")))
    }

    fn build_rec_pure_in(
        root: &PathBuf,
    ) -> impl FnMut(Box<dyn FileSystem>, Data) -> Box<dyn FileSystem> {
        move |fs, data| Self::build_rec_pure(fs, root, data)
    }

    fn build_rec_pure(
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
            Data::Directory { path, contents } => {
                fs.create_dir(
                    root.join(path.clone())
                        .to_str()
                        .unwrap(),
                )
                .unwrap();

                fs = contents.into_iter().fold(
                    fs,
                    Self::build_rec_pure_in(&root.join(path.clone())),
                );
            }
        }

        fs
    }

    fn build_rec(fs: &mut dyn FileSystem, root: &PathBuf, data: Data) {
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
            Data::Directory { path, contents } => {
                fs.create_dir(
                    root.join(path.clone())
                        .to_str()
                        .unwrap(),
                )
                .unwrap();

                for data in contents {
                    Self::build_rec(fs, &root.join(path.clone()), data);
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum Data {
    File { path: PathBuf, content: String },
    Directory { path: PathBuf, contents: Vec<Data> },
}

impl Data {
    pub fn file(path: PathBuf, content: String) -> Self {
        Self::File { path, content }
    }

    pub fn directory(path: PathBuf, contents: Vec<Data>) -> Self {
        Self::Directory { path, contents }
    }
}

fn main() {
    let mut fs_builder = FsBuilder::new();
    let fs = fs_builder.mkdir("test_dir", |builder| {
        builder.touch("test_file", "test_content")
    });
    println!("{:#?}", fs);
    let fs = fs.build();

    pretty(fs);

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

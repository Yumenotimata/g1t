use std::path::PathBuf;

use clap::{Parser, Subcommand};
use g1t::{FsMap, FsMapedJson, Hash, JsonStorage, Runner, runner::Content};
use vfs::{FileSystem, MemoryFS, VfsFileType};

#[derive(Debug)]
enum Node {
    File { name: String, content: String },
    Dir { name: String, children: Vec<Node> },
}

#[derive(Debug)]
pub struct FsBuilder {
    nodes: Vec<Node>,
}

impl FsBuilder {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn touch(
        &mut self,
        name: impl Into<String>,
        content: impl Into<String>,
    ) -> &mut Self {
        self.nodes.push(Node::File {
            name: name.into(),
            content: content.into(),
        });
        self
    }

    pub fn mkdir(
        &mut self,
        name: impl Into<String>,
        f: impl FnOnce(&mut FsBuilder),
    ) -> &mut Self {
        let mut child = FsBuilder::new();
        f(&mut child);
        self.nodes.push(Node::Dir {
            name: name.into(),
            children: child.nodes,
        });
        self
    }

    pub fn execute(
        &mut self,
        mount: impl Into<PathBuf>,
        fs: &mut impl FileSystem,
    ) {
        let mount = mount.into();

        fs.create_dir(mount.to_str().unwrap())
            .unwrap();

        for node in self.nodes.iter() {
            build_node(fs, &mount, node);
        }
    }
}

fn build_node(fs: &mut impl FileSystem, base: &PathBuf, node: &Node) {
    match node {
        Node::File { name, content } => {
            let path = base.join(&name);
            fs.create_file(path.to_str().unwrap())
                .unwrap();
            fs.append_file(path.to_str().unwrap())
                .unwrap()
                .write_all(content.as_bytes())
                .unwrap();
        }
        Node::Dir { name, children } => {
            let path = base.join(&name);
            fs.create_dir(path.to_str().unwrap())
                .unwrap();
            for child in children {
                build_node(fs, &path, child);
            }
        }
    }
}

fn main() {
    // let mut fs_builder = FsBuilder::new();
    // fs_builder
    //     .mkdir("test_dir", |builder| {
    //         builder.touch("test_file", "test_content");
    //         builder.mkdir("test_dir2", |builder| {
    //             builder.touch("test_file2", "test_content2");
    //         });
    //     })
    //     .touch("test_file3", "test_content3");

    // let mut fs = MemoryFS::new();
    // fs_builder.execute("/root", &mut fs);

    // let mut runner = Runner::new(
    //     Box::new(JsonStorage::new(Box::new(MemoryFS::new()))),
    //     Box::new(fs),
    // );

    // println!(
    //     "{:?} \n{:?}",
    //     runner.storage.index(),
    //     runner.storage.objects()
    // );

    // runner.run(g1t::Cmd::Add {
    //     file_name: "/root/test_dir/test_file".to_string(),
    // });

    // runner.run(g1t::Cmd::Commit {
    //     message: "test commit".to_string(),
    // });

    // println!(
    //     "== after add {:#?}\n{:#?}",
    //     runner.storage.index(),
    //     runner.storage.objects()
    // );

    let mut fs_builder = FsBuilder::new();
    fs_builder.mkdir("g1t", |builder| {});

    let mut fs = MemoryFS::new();
    fs_builder.execute("/root", &mut fs);

    let mut fs_maped_json = FsMapedJson::new("/root/g1t".into(), Box::new(fs));
    println!("{:?}", fs_maped_json);

    // fs_maped_json.update_index(Content::new(
    //     "test_file".to_string(),
    //     "test_content".to_string(),
    // ));

    let infs = MemoryFS::new();

    let mut runner = Runner::new(fs_maped_json, Box::new(infs));

    runner.run(g1t::Cmd::Add {
        file_name: "/root/test_dir/test_file".to_string(),
    });

    // println!("{:?}", fs_maped_json);
    // into_data(&mut fs_maped_json.fs, &PathBuf::from("/root/g1t"));
    // let mut fs_map = FsMap::new("/root");
    // fs_map.insert(Hash(vec![1, 2, 3, 4, 5]), "test_content", &mut fs);

    // let mut entries = fs.read_dir("/root").unwrap();
    // for entry in entries {
    //     println!("{:?}", entry);
    // }
}

fn pretty(fs: Box<dyn FileSystem>) {
    let mut entries = fs.read_dir("/test_dir").unwrap();
    for entry in entries {
        println!("{:?}", entry);
    }
}

fn into_data(fs: &mut Box<dyn FileSystem>, root: &PathBuf) {
    let mut entries = fs
        .read_dir(root.to_str().unwrap())
        .unwrap();

    for entry in entries {
        let meta = fs
            .metadata(
                root.join(entry.clone())
                    .to_str()
                    .unwrap(),
            )
            .unwrap();

        match meta.file_type {
            VfsFileType::File => {
                println!("file: {:#?}", entry);
                println!("file: {:#?}", root.join(entry).to_str().unwrap());
            }
            VfsFileType::Directory => {
                println!("dir: {:#?}", entry);
                into_data(fs, &PathBuf::from(root).join(entry.clone()));
                println!("dir: {:#?}", root.join(entry).to_str().unwrap());
            }
        }
    }
}

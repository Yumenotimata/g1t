use clap::{Parser, Subcommand};

use g1t::{AbsStorage, Content, FileSystem, storage};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    Init,
}

#[derive(Debug, Clone)]
enum Cmd {
    Add(String, String),
    Commit(String),
}

fn main() {
    // let cli = Cli::parse();

    // match cli.command {
    //     SubCommand::Init => println!("Init"),
    // }

    // let mut file_system = FileSystem::new();
    // file_system.mkdir("test_dir", |dir| {
    //     dir.touch("test_file", "test_content");
    //     dir.mkdir("test_dir2", |dir| {
    //         dir.touch("test_file2", "test_content2");
    //     });
    //     dir.mkdir("test_dir3", |dir| {
    //         dir.touch("test_file3", "test_content3");
    //     });
    // });
    // let file_system = file_system;

    let mut abs_storage = AbsStorage::new();

    let cmds = vec![
        Cmd::Add("test_file".to_string(), "test_content".to_string()),
        Cmd::Add("test_file2".to_string(), "test_content2".to_string()),
        Cmd::Commit("test_message".to_string()),
        Cmd::Add("test_file3".to_string(), "test_content3".to_string()),
        Cmd::Commit("test_message2".to_string()),
    ];

    for cmd in cmds {
        match cmd.clone() {
            Cmd::Add(file_name, content) => {
                abs_storage
                    .update_index(vec![Content::new(file_name, content)]);
            }
            Cmd::Commit(message) => {
                abs_storage.commit(message);
            }
        }

        println!("=====after {:?}\n{:#?}\n", cmd, abs_storage);
    }
}

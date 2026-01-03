use clap::{CommandFactory, FromArgMatches, Parser};
use vfs::FileSystem;

use crate::{Cli, SubCommand};

#[derive(Debug)]
pub struct Runtime {
    fs: Box<dyn FileSystem>,
}

impl Runtime {
    pub fn new(fs: Box<dyn FileSystem>) -> Self {
        Self { fs }
    }

    pub fn run(&mut self, cmd: String) {
        // clapはstd::os::argsを入力としてとる前提で設計されているので
        // プログラム名argv[0]にあたるダミー引数を渡している
        let mut args = cmd.split_ascii_whitespace().collect::<Vec<&str>>();

        let arg_matches = Cli::command()
            .no_binary_name(true)
            .try_get_matches_from(args)
            .unwrap();

        let cli = Cli::from_arg_matches(&arg_matches).unwrap();

        match cli.command {
            SubCommand::Touch { path } => {
                self.fs.create_file(&path).unwrap();
            }
            SubCommand::Init => {
                println!("Initializing");
            }
        }
    }
}

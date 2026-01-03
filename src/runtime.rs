use clap::{CommandFactory, FromArgMatches, Parser};
use vfs::FileSystem;

use crate::{Cli, Cmd, Index, Runner, SubCommand, runner};

#[derive(Debug)]
pub struct Runtime {
    infs: Box<dyn FileSystem>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            infs: Box::new(vfs::PhysicalFS::new(".")),
        }
    }

    pub fn get_index(&self) -> Result<Index, Box<dyn std::error::Error>> {
        let mut runner = runner::Runner::new();
        runner.storage.load()?;
        Ok(runner.storage.index.clone())
    }

    pub fn get_objects(&self) -> Result<Vec<runner::Object>, Box<dyn std::error::Error>> {
        let mut runner = runner::Runner::new();

        let objects = runner.storage.objects.get_all(&mut runner.storage.fs);
        let objects = objects
            .into_iter()
            .map(|object| serde_json::from_str(&object).unwrap())
            .collect::<Vec<runner::Object>>();

        Ok(objects)
    }

    pub fn run(&mut self, cmd: String) -> Result<String, Box<dyn std::error::Error>> {
        let args = cmd.split_ascii_whitespace().collect::<Vec<&str>>();

        let arg_matches = Cli::command()
            .no_binary_name(true)
            .try_get_matches_from(args)?;

        let cli = Cli::from_arg_matches(&arg_matches)?;

        match cli.command {
            SubCommand::Init => {
                // let dir = self.infs.read_dir(".")?;
                // for entry in dir {
                //     println!("{}", entry);
                // }
                self.infs.create_dir(".g1t")?;
                self.infs.create_dir(".g1t/objects")?;
            }
            SubCommand::Reset => {
                self.infs.remove_dir(".g1t/objects")?;
                self.infs.remove_dir(".g1t")?;
            }
            SubCommand::Add { path } => {
                let mut runner = runner::Runner::new();
                let res = runner.run(runner::Cmd::Add { file_name: path })?;
                println!("{:?}", res);
            }
            SubCommand::Commit { message } => {
                let mut runner = runner::Runner::new();
                let res = runner.run(runner::Cmd::Commit { message })?;
                println!("{:?}", res);
            }
        }

        Ok("Success".to_string())
    }
}

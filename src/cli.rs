use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    Add {
        #[structopt()]
        text: String,
    },
    Done {
        #[structopt()]
        position: usize,
    },
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rust Todo",
    about = "A command line to-do app written in Rust."
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
    pub json_file: Option<PathBuf>,
}

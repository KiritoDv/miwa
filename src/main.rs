use clap::Parser;
use commands::{directory::DirectoryCommand, file::FileCommand};

pub mod commands;
pub mod preprocessor;

#[derive(Parser, Debug)]
#[clap(name = "miwa", version)]
pub enum App {
    File(FileCommand),
    Directory(DirectoryCommand),
}

#[tokio::main]
async fn main() {
    let args = App::parse();
    match args {
        App::File(cmd) => cmd.run().await,
        App::Directory(cmd) => cmd.run().await,
    }
}

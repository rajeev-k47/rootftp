use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rootftp")]
#[command(about = "FTP Server", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Start {
        #[arg(short, long)]
        daemon: bool,
    },
    Stop,
    Setdir {
        path: PathBuf,
    },
    Status,
}

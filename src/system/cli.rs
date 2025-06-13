use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rootftp")]
#[command(
    about = "A simple FTP Server",
    long_about = "
RootFTP is a lightweight, plugin-enabled FTP service that allows you to extend file handling capabilities.It lives on your private network so you can play with plugins and files.
    "
)]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the RootFTP service
    Start {
        /// Run as a daemon
        #[arg(short, long)]
        daemon: bool,
    },
    /// Stop the RootFTP service
    Stop,
    /// Set the FTP root directory
    Setdir {
        /// Path to the new root directory
        _path: PathBuf,
    },
    /// Show the status of the RootFTP service
    Status,
    /// Load a local plugin (.so file)
    Loadplugin {
        /// Path to the plugin shared object file
        _path: PathBuf,
    },
    /// Fetch available plugins from the plugin library
    Fetch,
    /// Install a plugin by name from the plugin library
    Install {
        /// Name of the plugin (case sensitive with extension (.so))
        plugin_name: String,
    },
    /// List all installed plugins
    List,
}

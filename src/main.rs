mod auth;
mod config;
mod constants;
mod helpers;
mod listeners;
mod plugin_handler;
mod system;

use clap::Parser;
use config::Config;
use constants::SimpleAuthenticator;
use helpers::server_handler::{start_server, stop_server};
use local_ip_address::local_ip;
use system::cli::{Cli, Commands};
use system::init;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::load();
    let path = config.root_dir.join("credentials.json");
    let auth = SimpleAuthenticator::new(path.clone());

    let cli = Cli::parse();
    let ip = local_ip().unwrap();
    let addr = format!("{}:{}", ip, 2121);

    match cli.command {
        Commands::Start { daemon } => {
            println!("Starting FTP server...");
            start_server(daemon, auth, config.root_dir.clone(), addr.clone()).await?;
        }
        Commands::Stop => {
            stop_server().await;
            println!("FTP server stopped...");
        }
        Commands::Setdir { _path } => {
            config.root_dir = _path;
            config.save();
            println!("Root directory set to: {}", config.root_dir.display());
        }
        Commands::Status => {
            helpers::command_helper::status().await;
        }
        Commands::Loadplugin { _path } => {
            helpers::load_plugin::load_plugin(&_path);
        }
        Commands::Fetch => {
            helpers::plugin_library_handler::load_plugins(true).await?;
        }
        Commands::Install { plugin_name } => {
            helpers::plugin_library_handler::install_plugin(&plugin_name).await?;
        }
        Commands::List => {
            helpers::plugin_library_handler::list_plugins().await;
        }
    }
    Ok(())
}

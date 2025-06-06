use libunftp::{auth::Authenticator, ServerBuilder};
use std::path::PathBuf;
use std::sync::Arc;
mod auth;
mod global_consts;
use global_consts::{SimpleAuthenticator, UserEntry};
use local_ip_address::local_ip;
mod init;
use init::init;
mod cli;
use clap::Parser;
use cli::{Cli, Commands};
use std::process::Command;
mod config;
use crate::global_consts::create_rooted_storage;
use config::Config;
use tokio::process::Command as TokioCommand;
#[path = "listeners/outbox_listener.rs"]
mod outbox_listener;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init()?;
    let mut config = Config::load();
    eprintln!("Using root_dir = {}", config.root_dir.display());

    //with direct json auth
    //let authenticator = JsonFileAuthenticator::from_file(String::from(std::env::current_dir().unwrap().join("credentials.json").to_str().unwrap()))?;
    let path = config.root_dir.join("credentials.json");
    println!("Using credentials file = {}", path.display());
    let auth = SimpleAuthenticator::new(path);

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
            println!("Stopping FTP server...");
        }
        Commands::Setdir { path } => {
            config.root_dir = path;
            config.save();
            println!("Root directory set to: {}", config.root_dir.display());
        }
        Commands::Status => {
            println!("Current configuration:");
            println!("Root Directory: {}", config.root_dir.display()); //TODO:ADD MORE INFO
        }
    }
    Ok(())
}

async fn start_server(
    daemon: bool,
    auth: SimpleAuthenticator,
    root_dir: PathBuf,
    addr: String,
) -> Result<(), Box<dyn std::error::Error>> {
    if daemon {
        Command::new("nohup")
            .arg("rootftp")
            .arg("start")
            .spawn()
            .expect("Failed to start daemon");
    } else {
        let dir = root_dir.join("ftpd");
        let storage = create_rooted_storage(dir.clone());

        let auth_arc: Arc<dyn Authenticator<UserEntry> + Send + Sync> = Arc::new(auth);
        let server = ServerBuilder::with_authenticator(storage, auth_arc)
            .build()
            .expect("Failed to build server");

        println!("Listening on {}", addr);
        server.listen(addr).await?;
    }

    Ok(())
}
pub async fn stop_server() {
    TokioCommand::new("pkill")
        .arg("-f")
        .arg("rootftp")
        .status()
        .await
        .expect("Failed to stop server");
}

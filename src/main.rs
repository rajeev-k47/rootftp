use std::sync::Arc;
use unftp_sbe_fs::ServerExt;
use std::path::PathBuf;
mod auth;
mod global_consts;
use global_consts::SimpleAuthenticator;
use local_ip_address::local_ip;
mod init;
use init::init;
mod cli;
use cli::{Cli, Commands};
use clap::{Parser};
use std::process::Command;
mod config;
use config::Config;
use tokio::process::Command as TokioCommand;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init()?;
    let mut config = Config::load();

    //with direct json auth
    //let authenticator = JsonFileAuthenticator::from_file(String::from(std::env::current_dir().unwrap().join("credentials.json").to_str().unwrap()))?;
    let path = config.root_dir.join("credentials.json");
    let auth = SimpleAuthenticator::new(path);
    

    let cli = Cli::parse();
    let ip = local_ip().unwrap();
    let addr = format!("{}:{}",ip,2121);

    match cli.command {
        Commands::Start{daemon} => {
             println!("Starting FTP server...");
            start_server(daemon, auth,config.root_dir.clone(),addr.clone()).await?;
        },
        Commands::Stop=> {
            stop_server().await;
            println!("Stopping FTP server...");
        },
        Commands::Setdir{path} => {
             config.root_dir = path;
            config.save();
            println!("Root directory set to: {}", config.root_dir.display());
        },
        Commands::Status => {
            println!("Current configuration:");
            println!("Root Directory: {}", config.root_dir.display());//TODO:ADD MORE INFO
        },
    }
    Ok(())
    
}

async fn start_server(daemon: bool, auth: SimpleAuthenticator,root_dir: PathBuf,addr:String) -> Result<(), Box<dyn std::error::Error>> {

        if daemon {
        Command::new("nohup")
            .arg("ftp-server")
            .arg("start")
            .spawn()
            .expect("Failed to start daemon");
         } else {
        
        let dir = root_dir.join("ftpd");
        let server = libunftp::Server::with_fs(dir)
            .authenticator(Arc::new(auth))
            .build()
            .unwrap();

        server.listen(addr.clone()).await?;
    }
    println!("Started FTP server on {}",addr.clone());


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

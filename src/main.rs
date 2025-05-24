use std::sync::Arc;
use unftp_auth_jsonfile::JsonFileAuthenticator;
use unftp_sbe_fs::ServerExt;
use std::path::PathBuf;
mod auth;
use auth::SimpleAuthenticator;
use local_ip_address::local_ip;


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let ip = local_ip().unwrap();
    //with direct json auth
    //let authenticator = JsonFileAuthenticator::from_file(String::from(std::env::current_dir().unwrap().join("credentials.json").to_str().unwrap()))?;
    let path = PathBuf::from(String::from(std::env::current_dir().unwrap().join("credentials.json").to_str().unwrap()));
    let auth = SimpleAuthenticator::new(path);


    let ftp_home = PathBuf::from("/home/rajeev/Documents/");

        let addr = format!("{}:{}",ip,2121);
        let server = libunftp::Server::with_fs(ftp_home)
            .authenticator(Arc::new(auth))
            .build()
            .unwrap();

        server.listen(addr).await?;

        Ok(())
}

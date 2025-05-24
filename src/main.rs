use std::sync::Arc;
use unftp_auth_jsonfile::JsonFileAuthenticator;
use unftp_sbe_fs::ServerExt;
use std::path::PathBuf;
mod auth;
use auth::SimpleAuthenticator;
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //pretty_env_logger::init();

 let authenticator = JsonFileAuthenticator::from_file(String::from(std::env::current_dir().unwrap().join("credentials.json").to_str().unwrap()))?;
let path = PathBuf::from(String::from(std::env::current_dir().unwrap().join("credentials.json").to_str().unwrap()));
    let auth = SimpleAuthenticator::new(path);
//for user in auth.list_users() {
//        println!("User: {:?}", user);
 //   }
 //   let auth
 let ftp_home = PathBuf::from("/home/rajeev/Documents/");
    let addr = "192.168.221.160:2121";
    let server = libunftp::Server::with_fs(ftp_home)
        .authenticator(Arc::new(auth))
        .build()
        .unwrap();

    server.listen(addr).await?;

    Ok(())
}

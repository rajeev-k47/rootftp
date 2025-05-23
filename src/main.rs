use unftp_sbe_fs::ServerExt;
use std::path::PathBuf;
#[tokio::main]
pub async fn main() {
      let ftp_home = PathBuf::from("/home/rajeev/Documents/");
    let server = libunftp::Server::with_fs(ftp_home)
        .greeting("Welcome to my FTP server")
        .passive_ports(50000..=65535)
        .build()
        .unwrap();

    let _= server.listen("192.168.221.160:2121").await;
}

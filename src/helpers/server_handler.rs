use crate::constants::{create_rooted_storage, SimpleAuthenticator, UserEntry};
use libunftp::auth::Authenticator;
use libunftp::ServerBuilder;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use tokio::process::Command as TokioCommand;

pub async fn start_server(
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
        crate::init::init().await?;

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

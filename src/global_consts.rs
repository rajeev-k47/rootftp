use libunftp::auth::UserDetail;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::path::PathBuf;
use std::sync::Mutex;
use unftp_sbe_fs::Filesystem;
use unftp_sbe_fs::Meta;
use unftp_sbe_rooter::RooterVfs;
use unftp_sbe_rooter::UserWithRoot;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserEntry {
    pub username: String,
    pub password: String,
    pub home_dir: Option<PathBuf>,
}
impl UserDetail for UserEntry {
    fn account_enabled(&self) -> bool {
        true
    }
}
impl std::fmt::Display for UserEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "User(username: {:?}", self.username)
    }
}
impl UserWithRoot for UserEntry {
    fn user_root(&self) -> Option<PathBuf> {
        self.home_dir.clone()
    }
}
#[derive(Debug)]
pub struct SimpleAuthenticator {
    pub path: PathBuf,
    pub users: Mutex<Vec<UserEntry>>,
}

pub fn create_rooted_storage(
    base_dir: PathBuf,
) -> Box<dyn Fn() -> RooterVfs<Filesystem, UserEntry, Meta> + Send + Sync> {
    Box::new(move || {
        let fs =
            Filesystem::new(base_dir.clone()).unwrap_or_else(|e| panic!("Filesystem err:{}", e));
        RooterVfs::new(fs)
    })
}

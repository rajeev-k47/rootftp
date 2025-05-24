use std::sync::Mutex;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserEntry {
    pub username: String,
    pub password: String,
    pub home_dir: PathBuf,
}

#[derive(Debug)]
pub struct SimpleAuthenticator {
    pub path: PathBuf,
    pub users: Mutex<Vec<UserEntry>>,
}




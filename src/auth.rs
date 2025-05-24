use libunftp::auth::{AuthenticationError, Authenticator, DefaultUser,Credentials};
use std::{fs, path::PathBuf, sync::Mutex};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserEntry {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct SimpleAuthenticator {
    pub path: PathBuf,
    pub users: Mutex<Vec<UserEntry>>,
}


impl SimpleAuthenticator {
    pub fn new(path: PathBuf) -> Self {
        let users = if let Ok(data) = fs::read_to_string(&path) {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            vec![]
        };

        Self {
            path,
            users: Mutex::new(users),
        }
    }

    fn save_users(&self) {
        if let Ok(users) = self.users.lock() {
            let json = serde_json::to_string_pretty(&*users).unwrap();
            fs::write(&self.path, json).unwrap();
        }
    }
    pub fn list_users(&self) -> Vec<UserEntry> {
        if let Ok(users) = self.users.lock() {
            users.clone()
        } else {
            vec![]
        }
    }
    //pub fn 
}

#[async_trait]
impl Authenticator<DefaultUser> for SimpleAuthenticator {
    async fn authenticate(&self, _username: &str, _password: &Credentials) -> Result<DefaultUser, AuthenticationError> {
         println!("Authenticating {}", _username);
         println!("Password: {}", _password.password.as_deref().ok_or(AuthenticationError::BadPassword)?);
  Ok(DefaultUser {})
    }
}

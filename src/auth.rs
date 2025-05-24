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
    pub fn add_user(&mut self, username: &str, password: &str) -> bool {
        let mut users = self.users.lock().unwrap();
        
        if users.iter().any(|u| u.username == username) {
            false
        } else {
            users.push(UserEntry {
                username: username.to_string(),
                password: password.to_string(),
            });
            self.save_users();
            true
        }
    }

    fn save_users(&self) {
        if let Ok(users) = self.users.lock() {
            let json = serde_json::to_string_pretty(&*users).unwrap();
            fs::write(&self.path, json).unwrap();
        }
    }
    pub fn user_exists(&self, username: &str) -> bool {
        self.users.lock().unwrap()
            .iter()
            .any(|u| u.username == username)
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
        if _username.is_empty() || _username.eq_ignore_ascii_case("anonymous") {
            return Err(AuthenticationError::BadUser);
        }

        let mut users = self.users.lock()
            .map_err(|_| AuthenticationError::new("Sys. error"))?;

        if let Some(user) = users.iter().find(|u| u.username == _username) {
            if user.password ==_password.password.as_deref().ok_or(AuthenticationError::BadPassword)?{
                Ok(DefaultUser {})
            } else {
                Err(AuthenticationError::BadPassword)
            }
        } 
         else {
            let password = _password.password.as_deref()
                .ok_or(AuthenticationError::BadPassword)?;
            users.push(UserEntry {
                username: _username.to_string(),
                password: password.to_string(),
                });
            let json = serde_json::to_string_pretty(&*users)
                    .map_err(|e| AuthenticationError::new(format!("fail:{}", e)))?;
            fs::write(&self.path, json)
                    .map_err(|e| AuthenticationError::new(format!("fail:{}", e)))?;
            Ok(DefaultUser {})
        }
    }
}

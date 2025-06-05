use crate::config::Config;
use crate::global_consts::{SimpleAuthenticator, UserEntry};
use async_trait::async_trait;
use libunftp::auth::{AuthenticationError, Authenticator, Credentials, DefaultUser};
use std::{fs, path::PathBuf, sync::Mutex};

use crate::outbox_listener;

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
    pub fn ensure_user_dirs(&self, username: &str) -> std::io::Result<()> {
        let config = Config::load();
        let base_path = config.root_dir.clone().join("ftpd");
        let user_dir = base_path.join(username);
        fs::create_dir_all(&user_dir)?;
        fs::create_dir_all(user_dir.join("uploads"))?;
        fs::create_dir_all(user_dir.join("home"))?;
        fs::create_dir_all(user_dir.join("inbox"))?;
        fs::create_dir_all(user_dir.join("outbox"))?;

        std::thread::spawn(move || {
            outbox_listener::start_outbox_watchers(base_path.clone());
        });

        Ok(())
    }
}
#[async_trait]
impl Authenticator<DefaultUser> for SimpleAuthenticator {
    async fn authenticate(
        &self,
        _username: &str,
        _creds: &Credentials,
    ) -> Result<DefaultUser, AuthenticationError> {
        Err(AuthenticationError::BadUser)
    }
}

#[async_trait]
impl Authenticator<UserEntry> for SimpleAuthenticator {
    async fn authenticate(
        &self,
        _username: &str,
        _password: &Credentials,
    ) -> Result<UserEntry, AuthenticationError> {
        if _username.is_empty() || _username.eq_ignore_ascii_case("anonymous") {
            return Err(AuthenticationError::BadUser);
        }

        let mut users = self
            .users
            .lock()
            .map_err(|_| AuthenticationError::new("Sys. error"))?;

        if let Some(user) = users.iter().find(|u| u.username == _username) {
            if user.password
                == _password
                    .password
                    .as_deref()
                    .ok_or(AuthenticationError::BadPassword)?
            {
                self.ensure_user_dirs(_username)
                    .map_err(|e| AuthenticationError::new(format!("Dir err.: {}", e)))?;
                Ok(UserEntry {
                    username: _username.to_string(),
                    password: user.password.clone(),
                    home_dir: Some(PathBuf::from(_username)),
                })
            } else {
                Err(AuthenticationError::BadPassword)
            }
        } else {
            let password = _password
                .password
                .as_deref()
                .ok_or(AuthenticationError::BadPassword)?;

            users.push(UserEntry {
                username: _username.to_string(),
                password: password.to_string(),
                home_dir: Some(PathBuf::from(_username)),
            });
            let json = serde_json::to_string_pretty(&*users)
                .map_err(|e| AuthenticationError::new(format!("fail:{}", e)))?;
            fs::write(&self.path, json)
                .map_err(|e| AuthenticationError::new(format!("fail:{}", e)))?;
            self.ensure_user_dirs(_username)
                .map_err(|e| AuthenticationError::new(format!("Dir err.: {}", e)))?;
            Ok(UserEntry {
                username: _username.to_string(),
                password: password.to_string(),
                home_dir: Some(PathBuf::from(_username)),
            })
        }
    }
}

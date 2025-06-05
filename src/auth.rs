use libunftp::auth::{AuthenticationError, Authenticator, DefaultUser,Credentials};
use std::{fs, path::PathBuf, sync::Mutex};
use async_trait::async_trait;
use crate::global_consts::{UserEntry,SimpleAuthenticator};
use crate::config::Config;

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
        let base_path = config.root_dir.clone()
            .join("ftpd");

        let user_dir = base_path.join(username);
        fs::create_dir_all(&user_dir)?;
        fs::create_dir_all(user_dir.join("uploads"))?;
        fs::create_dir_all(user_dir.join("private"))?;
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let permissions = fs::Permissions::from_mode(0o700);
            fs::set_permissions(&user_dir, permissions.clone())?;
            fs::set_permissions(user_dir.join("uploads"), permissions.clone())?;
            fs::set_permissions(user_dir.join("private"), permissions.clone())?;
        }
        Ok(())
    }
    
    //pub fn 
}
#[async_trait]
impl Authenticator<DefaultUser> for SimpleAuthenticator {
    async fn authenticate(&self, _username: &str, _creds: &Credentials) -> Result<DefaultUser, AuthenticationError> {
        Err(AuthenticationError::BadUser)
    }
}

#[async_trait]
impl Authenticator<UserEntry> for SimpleAuthenticator {

    async fn authenticate(&self, _username: &str, _password: &Credentials) -> Result<UserEntry, AuthenticationError> {

        if _username.is_empty() || _username.eq_ignore_ascii_case("anonymous") {
            return Err(AuthenticationError::BadUser);
        }

        let mut users = self.users.lock()
            .map_err(|_| AuthenticationError::new("Sys. error"))?;

        if let Some(user) = users.iter().find(|u| u.username == _username) {
            if user.password ==_password.password.as_deref().ok_or(AuthenticationError::BadPassword)?{
                self.ensure_user_dirs(_username)
                    .map_err(|e| AuthenticationError::new(format!("Dir err.: {}", e)))?;
                Ok(UserEntry {
                    username: _username.to_string(),
                    password: user.password.clone(),
                    home_dir: Some(PathBuf::from(_username)),
                })

             }else {
                Err(AuthenticationError::BadPassword)
            }
        } else {
            let password = _password.password.as_deref()
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
            self.ensure_user_dirs(_username).map_err(|e| AuthenticationError::new(format!("Dir err.: {}", e)))?;
            Ok(UserEntry{
                username: _username.to_string(),
                password: password.to_string(),
                home_dir: Some(PathBuf::from(_username)),
            })
        }
    }
}

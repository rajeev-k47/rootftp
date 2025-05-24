use libunftp::auth::{AuthenticationError, Authenticator, DefaultUser,Credentials};
use std::{fs, path::PathBuf, sync::Mutex};
use async_trait::async_trait;
use crate::global_consts::{UserEntry,SimpleAuthenticator};


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
        let base_path = PathBuf::from("/home/rajeev/ftpd");
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

    async fn authenticate(&self, _username: &str, _password: &Credentials) -> Result<DefaultUser, AuthenticationError> {

        if _username.is_empty() || _username.eq_ignore_ascii_case("anonymous") {
            return Err(AuthenticationError::BadUser);
        }

        let mut users = self.users.lock()
            .map_err(|_| AuthenticationError::new("Sys. error"))?;

        if let Some(user) = users.iter().find(|u| u.username == _username) {
            if user.password ==_password.password.as_deref().ok_or(AuthenticationError::BadPassword)?{
                self.ensure_user_dirs(_username)
                    .map_err(|e| AuthenticationError::new(format!("Dir err.: {}", e)))?;
                Ok(DefaultUser {})

             }else {
                Err(AuthenticationError::BadPassword)
            }
        } else {
            let password = _password.password.as_deref()
                .ok_or(AuthenticationError::BadPassword)?;
            let home_dir = PathBuf::from("/home/rajeev/ftpd/").join(_username);

            users.push(UserEntry {
                username: _username.to_string(),
                password: password.to_string(),
                home_dir: home_dir.clone(),
                });
            let json = serde_json::to_string_pretty(&*users)
                    .map_err(|e| AuthenticationError::new(format!("fail:{}", e)))?;
            fs::write(&self.path, json)
                    .map_err(|e| AuthenticationError::new(format!("fail:{}", e)))?;
            self.ensure_user_dirs(_username).map_err(|e| AuthenticationError::new(format!("Dir err.: {}", e)))?;
            Ok(DefaultUser {})
        }
    }
}

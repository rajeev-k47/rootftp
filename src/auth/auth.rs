use crate::config::Config;
use crate::constants::{SimpleAuthenticator, UserEntry};
use crate::listeners::outbox_listener;
use crate::plugin_handler;
use crate::plugin_handler::loader::PluginInstance;
use argon2::password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use async_trait::async_trait;
use libunftp::auth::{AuthenticationError, Authenticator, Credentials, DefaultUser};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::{fs, path::PathBuf};

static PLUGIN_CACHE: OnceLock<HashMap<String, Arc<PluginInstance>>> = OnceLock::new();
static INITIALIZED_USERS: Mutex<Option<HashSet<String>>> = Mutex::new(None);
static OUTBOX_STARTED: AtomicBool = AtomicBool::new(false);

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
}

fn verify_password(password: &str, stored: &str) -> bool {
    let Ok(parsed) = PasswordHash::new(stored) else {
        return false;
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

fn is_argon2_hash(s: &str) -> bool {
    s.starts_with("$argon2")
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
    pub fn ensure_user_dirs(&self, username: &str) -> std::io::Result<()> {
        let config = Config::load();
        let base_path = config.root_dir.clone().join("ftpd");
        let user_dir = base_path.join(username);
        fs::create_dir_all(&user_dir)?;
        fs::create_dir_all(user_dir.join("uploads"))?;
        fs::create_dir_all(user_dir.join("home"))?;
        fs::create_dir_all(user_dir.join("inbox"))?;
        fs::create_dir_all(user_dir.join("outbox"))?;

        if !OUTBOX_STARTED.swap(true, Ordering::Relaxed) {
            let ftpd = base_path.clone();
            std::thread::spawn(move || {
                outbox_listener::start_outbox_watchers(ftpd);
            });
        }

        let mut guard = INITIALIZED_USERS.lock().unwrap();
        let set = guard.get_or_insert_with(HashSet::new);
        if set.insert(username.to_string()) {
            let plugin_map = PLUGIN_CACHE
                .get_or_init(|| {
                    crate::plugin_handler::load_plugins(&config.root_dir.join("plugins"))
                })
                .clone();
            plugin_handler::start_watchers(base_path, username.to_string(), plugin_map);
        }

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

        if let Some(idx) = users.iter().position(|u| u.username == _username) {
            let password = _password
                .password
                .as_deref()
                .ok_or(AuthenticationError::BadPassword)?;

            if !verify_password(password, &users[idx].password) {
                return Err(AuthenticationError::BadPassword);
            }

            if !is_argon2_hash(&users[idx].password) {
                users[idx].password = hash_password(password);
                let json = serde_json::to_string_pretty(&*users)
                    .map_err(|e| AuthenticationError::new(format!("fail:{}", e)))?;
                fs::write(&self.path, json)
                    .map_err(|e| AuthenticationError::new(format!("fail:{}", e)))?;
            }

            self.ensure_user_dirs(_username)
                .map_err(|e| AuthenticationError::new(format!("Dir err.: {}", e)))?;
            Ok(UserEntry {
                username: _username.to_string(),
                password: users[idx].password.clone(),
                home_dir: Some(PathBuf::from(_username)),
            })
        } else {
            let password = _password
                .password
                .as_deref()
                .ok_or(AuthenticationError::BadPassword)?;

            let hashed = hash_password(password);

            users.push(UserEntry {
                username: _username.to_string(),
                password: hashed.clone(),
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
                password: hashed,
                home_dir: Some(PathBuf::from(_username)),
            })
        }
    }
}

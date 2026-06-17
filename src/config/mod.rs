use directories::ProjectDirs;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

static CONFIG_CACHE: Mutex<Option<Config>> = Mutex::new(None);

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub root_dir: PathBuf,
}

impl Config {
    pub fn load() -> Self {
        let mut cache = CONFIG_CACHE.lock().unwrap();
        if let Some(config) = cache.as_ref() {
            return config.clone();
        }
        let config = Self::load_inner();
        *cache = Some(config.clone());
        config
    }

    fn load_inner() -> Self {
        let default_config = Config {
            root_dir: home::home_dir()
                .unwrap()
                .join("ftproot")
        };

        if let Some(proj_dirs) = ProjectDirs::from("net", "runner", "rootftp") {
            let config_dir = proj_dirs.config_dir();
            let config_path = config_dir.join("config.json");
            match fs::read_to_string(&config_path) {
                Ok(contents) => serde_json::from_str(&contents).unwrap_or(default_config),
                Err(_) => {
                    fs::create_dir_all(config_dir).ok();
                    if let Err(e) = fs::write(&config_path, serde_json::to_string_pretty(&default_config).unwrap()) {
                        eprintln!("config save err: {}", e);
                    }
                    default_config
                }
            }
        } else {
            default_config
        }
    }

    pub fn save(&self) {
        if let Some(proj_dirs) = ProjectDirs::from("net", "runner", "rootftp") {
            let config_dir = proj_dirs.config_dir();
            fs::create_dir_all(config_dir).unwrap();
            let config_path = config_dir.join("config.json");
            fs::write(config_path, serde_json::to_string_pretty(self).unwrap()).unwrap();
        }
        let mut cache = CONFIG_CACHE.lock().unwrap();
        *cache = Some(self.clone());
    }
}

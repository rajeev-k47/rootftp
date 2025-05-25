use directories::ProjectDirs;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub root_dir: PathBuf,
}

impl Config {
    pub fn load() -> Self {
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
                    eprintln!("Warning: Couldn't create config: {}", e);
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
    }
}

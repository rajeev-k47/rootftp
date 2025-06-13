use std::fs;

use crate::config::Config;
use std::path::PathBuf;

pub fn load_plugin(_path: &PathBuf) {
    let config = Config::load();
    if _path.extension().and_then(|s| s.to_str()) == Some("so") {
        let plugins_dir = config.root_dir.join("plugins");
        if !plugins_dir.exists() {
            fs::create_dir_all(&plugins_dir).unwrap();
        }
        fs::copy(_path, plugins_dir.join(_path.file_name().unwrap())).unwrap();
        println!("Plugin loaded {}", _path.display());
    } else {
        println!("File is not a shared library (.so)");
    }
}

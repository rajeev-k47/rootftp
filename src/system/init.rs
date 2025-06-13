use crate::config::Config;
use crate::helpers::plugin_library_handler::load_plugins;
use std::fs;
use std::io;

pub async fn init() -> io::Result<()> {
    let config = Config::load();
    let root_dir = config.root_dir.clone();
    let ftpd = root_dir.join("ftpd");
    let plugins = root_dir.join("plugins");
    let credentials = root_dir.join("credentials.json");
    fs::create_dir_all(&root_dir)?;
    fs::create_dir_all(&ftpd)?;
    if !credentials.exists() {
        fs::File::create(&credentials)?;
    }
    fs::create_dir_all(&plugins)?;

    match load_plugins(false).await {
        Ok(_) => {
            println!("Plugins loaded from plugin library");
        }
        Err(_err) => println!("Warning: Failed to loading plugins."),
    };
    Ok(())
}

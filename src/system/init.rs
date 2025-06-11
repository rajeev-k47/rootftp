use crate::config::Config;
use std::fs;
use std::io;

pub fn init() -> io::Result<()> {
    let config = Config::load();
    let root_dir = config.root_dir.clone();
    let ftpd = root_dir.join("ftpd");
    let plugins = root_dir.join("plugins");
    let credentials = root_dir.join("credentials.json");
    fs::create_dir_all(&root_dir)?;
    fs::create_dir_all(&ftpd)?;
    fs::File::create(&credentials)?;
    fs::create_dir_all(&plugins)?;
    Ok(())
}

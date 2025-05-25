use std::fs;
use std::io;
use crate::config::Config;

pub fn init() -> io::Result<()> {
    
    let config = Config::load();
    let shared_dir = config.root_dir.clone()
    .join("ftpd")
    .join("shared");

    //print!("223423424 :{}", shared_dir.display());
    fs::create_dir_all(&shared_dir)?;

    Ok(())
}


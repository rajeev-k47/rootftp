use std::fs;
use std::io;


pub fn init() -> io::Result<()> {
    fs::create_dir_all("/home/rajeev/ftpd/shared")?;
    Ok(())
}


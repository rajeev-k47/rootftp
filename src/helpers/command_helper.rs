use crate::{config::Config, helpers::plugin_library_handler::list_plugins};

pub async fn status() {
    let config = Config::load();
    let path = config.root_dir.join("credentials.json");

    println!("Current configuration:");
    println!("Root Directory: {}", &config.root_dir.display());
    println!("Using credentials file = {}", path.clone().display());
    list_plugins().await;
}

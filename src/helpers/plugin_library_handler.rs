use anyhow::{Context, Result};
use config::Config;
use directories::ProjectDirs;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::{fs, path::Path};
use tokio::fs as async_fs;

use crate::config;

#[derive(Deserialize)]
struct Content {
    name: String,
    #[serde(rename = "type")]
    kind: String,
    download_url: Option<String>,
}

pub async fn load_plugins(deb: bool) -> Result<()> {
    let url = "https://api.github.com/repos/rajeev-k47/rootftp/contents/plugin_library";
    let client = Client::new();

    let resp = client
        .get(url)
        .header("User-Agent", "plugin-fetcher")
        .send()
        .await
        .with_context(|| format!("Failed to fetch plugin list"))?;

    let entries: Vec<Content> = resp.json().await.with_context(|| "Failed to parse")?;

    if deb {
        println!("Loading plugins :");
    }
    let plugins: HashMap<String, String> = entries
        .into_iter()
        .filter(|e| e.kind == "file" && e.name.ends_with(".so"))
        .map(|e| {
            let name = e.name.to_string();
            let url = e.download_url.unwrap_or_default();
            if deb {
                println!("{}", name);
            }
            return (name, url);
        })
        .collect();

    if let Some(proj_dirs) = ProjectDirs::from("net", "runner", "rootftp") {
        let config_dir = proj_dirs.config_dir();
        fs::create_dir_all(config_dir).unwrap();
        let config_path = config_dir.join("plugins.json");
        fs::write(config_path, serde_json::to_string_pretty(&plugins)?)?;
    }

    Ok(())
}

pub async fn install_plugin(_name: &str) -> Result<()> {
    if let Some(proj_dirs) = ProjectDirs::from("net", "runner", "rootftp") {
        let config_dir = proj_dirs.config_dir();
        let config_path = config_dir.join("plugins.json");
        let plugin_dir = Config::load().root_dir.join("plugins");

        match read_plugins(&config_path.to_string_lossy()) {
            Ok(plugins) => {
                let mut found = false;
                for (name, url) in plugins {
                    if name == _name {
                        download_plugin(_name, &url, &plugin_dir).await?;
                        found = true;
                    }
                }
                if !found {
                    println!("\nPlugin does not exist.\n");
                    list_plugins().await;
                }
            }
            Err(err) => {
                eprintln!("Failed to install plugin: {}", err);
            }
        }
    }
    Ok(())
}
pub async fn list_plugins() {
    if let Some(proj_dirs) = ProjectDirs::from("net", "runner", "rootftp") {
        let config_dir = proj_dirs.config_dir();
        let config_path = config_dir.join("plugins.json");

        match read_plugins(&config_path.to_string_lossy()) {
            Ok(plugins) => {
                println!("Current available plugins :");
                for (name, _) in plugins {
                    println!("{}", name);
                }
                println!("\nUse <rootftp fetch> to sync plugins with plugin library");
            }
            Err(err) => {
                eprintln!("Error : {}", err);
            }
        }
    }
}

fn read_plugins(path: &str) -> Result<HashMap<String, String>> {
    let content = fs::read_to_string(path)?;
    let plugins: HashMap<String, String> = serde_json::from_str(&content)?;
    Ok(plugins)
}

pub async fn download_plugin(name: &str, url: &str, plugin_dir: &Path) -> Result<()> {
    let dest_path = plugin_dir.join(name);

    let client = Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "plugin-downloader")
        .send()
        .await
        .context("Exception download")?
        .bytes()
        .await
        .context("Exception download")?;

    async_fs::write(&dest_path, &response)
        .await
        .with_context(|| "Failed to save plugin to")?;

    println!("Plugin installed {}", name);
    Ok(())
}

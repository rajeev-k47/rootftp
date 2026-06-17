use crate::plugin_handler::loader::PluginInstance;
use inotify::{EventMask, Inotify, WatchMask};
use std::sync::Arc;
use std::time::Duration;
use std::{collections::HashMap, fs, path::PathBuf, thread};

pub fn start_watchers(
    ftpd_root: PathBuf,
    user: String,
    plugin_map: HashMap<String, Arc<PluginInstance>>,
) {
    let plugins: Vec<(String, Arc<PluginInstance>)> = plugin_map.into_iter().collect();

    thread::spawn(move || {
        let mut inotify = match Inotify::init() {
            Ok(i) => i,
            Err(e) => {
                eprintln!("Failed to init inotify for plugin watcher: {}", e);
                return;
            }
        };
        let mut wd_map: HashMap<_, (String, String)> = HashMap::new();
        let mut buf = [0u8; 4096];

        for (plugin_name, _) in &plugins {
            let dir = ftpd_root
                .join(&user)
                .join("plugins")
                .join(plugin_name)
                .join("input");
            let out_dir = ftpd_root
                .join(&user)
                .join("plugins")
                .join(plugin_name)
                .join("output");

            fs::create_dir_all(&dir).ok();
            fs::create_dir_all(&out_dir).ok();
            match inotify.watches().add(
                &dir,
                WatchMask::CLOSE_WRITE | WatchMask::MOVED_TO,
            ) {
                Ok(wd) => {
                    wd_map.insert(wd.clone(), (user.clone(), plugin_name.clone()));
                }
                Err(e) => {
                    eprintln!("Failed to watch plugin dir {:?}: {}", dir, e);
                }
            }
        }

        loop {
            let events = match inotify.read_events_blocking(&mut buf) {
                Ok(ev) => ev,
                Err(e) => {
                    eprintln!("inotify read_events failed: {}", e);
                    thread::sleep(Duration::from_secs(1));
                    continue;
                }
            };

            for ev in events {
                if ev.mask.contains(EventMask::ISDIR) {
                    continue;
                }
                if !ev
                    .mask
                    .intersects(EventMask::CLOSE_WRITE | EventMask::MOVED_TO)
                {
                    continue;
                }
                let Some(name) = ev.name else { continue };
                let Some((user, plugin_name)) = wd_map.get(&ev.wd) else { continue };
                let filename = name.to_string_lossy();
                let path = ftpd_root
                    .join(user)
                    .join("plugins")
                    .join(plugin_name)
                    .join("input")
                    .join(&*filename);
                let out_path = ftpd_root
                    .join(user)
                    .join("plugins")
                    .join(plugin_name)
                    .join("output");
                let Some(instance) = plugins
                    .iter()
                    .find(|(n, _)| n == plugin_name)
                    .map(|(_, p)| p)
                else { continue };
                let Some(ext) = path.extension().and_then(|s| s.to_str()) else { continue };
                if !instance.exts.contains(&ext.to_string()) {
                    continue;
                }
                let instance = instance.clone();
                let path = path.clone();
                let out_path = out_path.clone();
                thread::spawn(move || {
                    instance.plugin.on_create(&path, &out_path);
                });
            }
        }
    });
}

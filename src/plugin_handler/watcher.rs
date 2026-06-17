use crate::plugin_handler::loader::PluginInstance;
use inotify::{EventMask, Inotify, WatchMask};
use std::sync::Arc;
use std::{collections::HashMap, fs, path::PathBuf, thread, time::Duration};

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
                WatchMask::CREATE | WatchMask::MOVED_TO | WatchMask::CLOSE_WRITE,
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
            thread::sleep(Duration::from_millis(1000));

            for ev in events {
                if !ev
                    .mask
                    .intersects(EventMask::CREATE | EventMask::MOVED_TO | EventMask::CLOSE_WRITE)
                {
                    continue;
                }
                if let Some(name) = ev.name {
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
                    if let Some(instance) = plugins
                        .iter()
                        .find(|(n, _)| n == plugin_name)
                        .map(|(_, p)| p)
                    {
                        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                            if instance.exts.contains(&ext.to_string()) {
                                instance.plugin.on_create(&path, &out_path)
                            }
                        }
                    }
                }
            }
        }
    });
}

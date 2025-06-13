use crate::plugin_handler::plugin_trait::Plugin;
use inotify::{EventMask, Inotify, WatchMask};
use std::{collections::HashMap, fs, path::PathBuf, thread, time::Duration};

pub fn start_watchers(
    ftpd_root: PathBuf,
    user: String,
    plugin_map: HashMap<String, (Box<dyn Plugin>, Vec<String>)>,
) {
    let plugins = plugin_map.into_iter().collect::<Vec<_>>();

    thread::spawn(move || {
        let mut inotify = Inotify::init().unwrap();
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
            let wd = inotify
                .watches()
                .add(
                    &dir,
                    WatchMask::CREATE | WatchMask::MOVED_TO | WatchMask::CLOSE_WRITE,
                )
                .unwrap();
            wd_map.insert(wd.clone(), (user.clone(), plugin_name.clone()));
        }

        loop {
            let events = inotify.read_events_blocking(&mut buf).unwrap();
            thread::sleep(Duration::from_millis(1000));

            for ev in events {
                if !ev
                    .mask
                    .intersects(EventMask::CREATE | EventMask::MOVED_TO | EventMask::CLOSE_WRITE)
                {
                    continue;
                }
                if let Some(name) = ev.name {
                    let (user, plugin_name) = &wd_map[&ev.wd];
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
                    if let Some((plugin, exts)) = plugins
                        .iter()
                        .find(|(n, _)| n == plugin_name)
                        .map(|(_, pe)| pe)
                    {
                        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                            if exts.contains(&ext.to_string()) {
                                plugin.on_create(&path, &out_path)
                            }
                        }
                    }
                }
            }
        }
    });
}

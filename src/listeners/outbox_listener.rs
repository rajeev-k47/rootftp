use inotify::{EventMask, Inotify, WatchDescriptor, WatchMask};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

pub fn start_outbox_watchers(ftpd_root: PathBuf) {
    let entries = match fs::read_dir(&ftpd_root) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Err ({:?}): {}", ftpd_root, e);
            return;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let username = match path.file_name() {
            Some(os) => os.to_string_lossy().into_owned(),
            None => continue,
        };

        let ftpd = ftpd_root.clone();
        let outbox_dir = path.join("outbox").clone();
        thread::spawn(move || {
            run_outbox_watcher(&ftpd, &username, &outbox_dir);
        });
    }
}

fn run_outbox_watcher(ftpd_root: &Path, username: &str, user_outbox: &Path) {
    let mut inotify = Inotify::init().expect("Error init inotify");
    let mut wd_map: HashMap<WatchDescriptor, PathBuf> = HashMap::new();

    match inotify.watches().add(
        user_outbox,
        WatchMask::CREATE | WatchMask::MOVED_TO | WatchMask::CLOSE_WRITE,
    ) {
        Ok(wd) => {
            wd_map.insert(wd.clone(), user_outbox.to_path_buf());
            println!("[{} watcher] Watching {:?}", username, user_outbox); //debug-o.
        }
        Err(e) => {
            eprintln!(
                "[{} watcher] add_watch failed on {:?}: {}", //debug-o.
                username, user_outbox, e
            );
            return;
        }
    }

    if let Ok(entries) = fs::read_dir(user_outbox) {
        for entry in entries.flatten() {
            let sub_path = entry.path();
            if sub_path.is_dir() {
                if let Ok(wd) = inotify.watches().add(
                    &sub_path,
                    WatchMask::CREATE | WatchMask::MOVED_TO | WatchMask::CLOSE_WRITE,
                ) {
                    wd_map.insert(wd.clone(), sub_path.clone());
                    // println!(
                    //   "[{} watcher] Also watching sub_dir {:?}",
                    // username, sub_path
                    // );
                    //debug-o.
                }
            }
        }
    }

    let mut buffer = [0u8; 4096];

    loop {
        let events = match inotify.read_events_blocking(&mut buffer) {
            Ok(ev) => ev,
            Err(e) => {
                eprintln!("[{} watcher] read_events failed: {}", username, e); //debug-o.
                thread::sleep(Duration::from_secs(1)); //dlay for retry
                continue;
            }
        };

        for event in events {
            if event.mask.contains(EventMask::CREATE) && event.mask.contains(EventMask::ISDIR) {
                //TODO Handle unknown mask panic
                if let Some(name_os) = event.name {
                    let subdir = user_outbox.join(name_os.to_string_lossy().into_owned());
                    if subdir.is_dir() {
                        if let Ok(wd) = inotify.watches().add(
                            &subdir,
                            WatchMask::CREATE | WatchMask::MOVED_TO | WatchMask::CLOSE_WRITE,
                        ) {
                            wd_map.insert(wd.clone(), subdir.clone());
                            //println!(
                            //  "[{} watcher] also watching new sub-dir {:?}", //debug-o.
                            //username, subdir
                            //);
                        }
                    }
                }
                continue;
            }

            if event
                .mask
                .intersects(EventMask::CREATE | EventMask::MOVED_TO | EventMask::CLOSE_WRITE)
            {
                let watched_path = match wd_map.get(&event.wd) {
                    Some(p) => p.clone(),
                    _none => {
                        eprintln!("[{} watcher] Unknown wd: {:?}", username, event.wd); //debug-o.
                        continue;
                    }
                };

                if let Some(name_os) = event.name {
                    let filename = name_os.to_string_lossy().into_owned();
                    let rel = match watched_path.strip_prefix(&ftpd_root.join(username)) {
                        Ok(r) => r,
                        Err(_) => continue,
                    }; //format-> /outbox/...
                    let comps: Vec<_> = rel
                        .components()
                        .map(|c| c.as_os_str().to_string_lossy().into_owned())
                        .collect();

                    if comps.len() >= 2 {
                        let part = &comps[1];
                        if let Some(target_user) = part.strip_prefix("share.") {
                            let src = watched_path.join(&filename);
                            let dest_dir = ftpd_root.join(target_user).join("inbox");
                            let dest = dest_dir.join(&filename);

                            match fs::rename(&src, &dest) {
                                Ok(()) => {
                                    // println!(
                                    //   "[{}→{}] Moved {:?} → {:?}",
                                    // username,
                                    //target_user,
                                    //src,
                                    //dest //debug-o.
                                    // );
                                }
                                Err(e) => {
                                    //eprintln!(
                                    //  "[{}→{}] Move error {:?} → {:?}: {}",
                                    // username,
                                    // target_user,
                                    //src,
                                    //dest,
                                    //e //debug-o.
                                    //);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

use crate::plugin_handler::plugin_trait::Plugin;
use libloading::Library;
use std::sync::Arc;
use std::{collections::HashMap, fs, path::Path};

pub struct PluginInstance {
    pub plugin: Box<dyn Plugin>,
    pub exts: Vec<String>,
    _lib: Library,
}

pub fn load_plugins(dir: &Path) -> HashMap<String, Arc<PluginInstance>> {
    let mut map = HashMap::new();

    let Ok(entries) = fs::read_dir(dir) else {
        return map;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("so") {
            continue;
        }
        let name = match path.file_stem() {
            Some(s) => s.to_string_lossy().into_owned(),
            None => continue,
        };

        let lib = match unsafe { Library::new(&path) } {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Failed to load plugin '{}': {}", path.display(), e);
                continue;
            }
        };

        let plugin = unsafe {
            let func: libloading::Symbol<unsafe extern "C" fn() -> *mut dyn Plugin> = match lib.get(b"register_plugin") {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Plugin '{}' missing register_plugin: {}", name, e);
                    continue;
                }
            };
            let raw = func();
            if raw.is_null() {
                eprintln!("Plugin '{}' returned null pointer from register_plugin", name);
                continue;
            }
            Box::from_raw(raw)
        };

        plugin.init();
        let exts = plugin.extensions().iter().map(|&s| s.to_string()).collect();
        map.insert(name, Arc::new(PluginInstance { plugin, exts, _lib: lib }));
    }
    map
}

use crate::plugin_handler::plugin_trait::Plugin;
use libloading::Library;
use std::{collections::HashMap, fs, path::Path};

pub fn load_plugins(dir: &Path) -> HashMap<String, (Box<dyn Plugin>, Vec<String>)> {
    let mut map = HashMap::new();

    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) == Some("so") {
            let name = path.file_stem().unwrap().to_string_lossy().into_owned();
            let lib = unsafe { Library::new(&path).unwrap() };

            unsafe {
                let func: libloading::Symbol<unsafe extern "C" fn() -> *mut dyn Plugin> =
                    lib.get(b"register_plugin").unwrap();
                let raw = func();
                let plugin = Box::from_raw(raw);
                plugin.init();
                let exts = plugin.extensions().iter().map(|&s| s.to_string()).collect();
                map.insert(name.clone(), (plugin, exts));
                std::mem::forget(lib);
            }
        }
    }
    map
}

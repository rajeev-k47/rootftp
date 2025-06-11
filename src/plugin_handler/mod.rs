pub mod loader;
pub mod plugin_trait;
pub mod watcher;

pub use loader::load_plugins;
pub use watcher::start_watchers;

pub trait Plugin: Send + Sync {
    fn init(&self);
    fn extensions(&self) -> &[&'static str];
    fn on_create(&self, input_path: &std::path::Path, output_path: &std::path::Path);
}

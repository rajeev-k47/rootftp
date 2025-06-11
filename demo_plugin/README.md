## Demo Plugin
This is an example of creating rootFTP plugin

- Import Plugin trait from rootFTP.
- Make a plugin struct
```rust
use rootftp::plugin_handler::plugin_trait::Plugin;

pub struct customPlugin;
  ```
- Implement methods on customPlugin struct.
```rust
impl Plugin for customPlugin {
    fn init(&self) {
        println!("CustomPlugin started!!");
    }

    fn extensions(&self) -> &[&'static str] {
        &["extensions"] // extensions => cpp,py,txt,rs etc.
    }

    fn on_create(&self, input_file: &Path, output_path: &Path) {
        //input_file => File which has to be placed inside input folder of that extension.
        //output_path => Output directory where you can create files as output.

        let input_path = input_file.parent().unwrap().join("input.in");

        //Since input.in file is optional you can check and do relative work with it.
        if input_path.exists() {
            match File::open(&input_path) {
                Ok(input_file) => {
                    //do something.
                }
                Err(_e) =>{ //do something }
            }
        }
    }
}

//For cpp example
#[unsafe(no_mangle)]
pub extern "C" fn register_plugin() -> *mut dyn Plugin { // register_plugin is case-sensitive.
    Box::into_raw(Box::new(customPlugin))
}
```
- Then build the plugin 😃.


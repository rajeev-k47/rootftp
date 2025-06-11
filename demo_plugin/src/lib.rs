use rootftp::plugin_handler::plugin_trait::Plugin;
use std::{
    fs::{self, File},
    path::Path,
    process::{Command, Stdio},
};

pub struct CppPlugin;

impl Plugin for CppPlugin {
    fn init(&self) {
        println!("CppPlugin started!!");
    }

    fn extensions(&self) -> &[&'static str] {
        &["cpp"]
    }

    fn on_create(&self, input_file: &Path, output_path: &Path) {
        let file_stem = input_file
            .file_stem()
            .expect("Naming error")
            .to_string_lossy();
        let input_path = input_file.parent().unwrap().join("input.in");
        let output_path = output_path.join(format!("{}.txt", file_stem));
        fs::File::create(&output_path).unwrap();

        let compile = Command::new("g++")
            .arg(input_file)
            .arg("-o")
            .arg("out")
            .status();

        if compile.is_err() {
            fs::write(&output_path, "Compilation failed").unwrap();
            return;
        }

        let mut cmd = Command::new(format!("./{}", "out"));

        if input_path.exists() {
            match File::open(&input_path) {
                Ok(input_file) => {
                    cmd.stdin(Stdio::from(input_file));
                }
                Err(_e) =>{  }
            }
        }

        match cmd.output() {
            Ok(output_exec) => {
                fs::write(&output_path, &output_exec.stdout).unwrap();
            }
            Err(e) => {
                fs::write(&output_path, e.to_string()).unwrap();
            }
        }

        let _ = fs::remove_file("out");
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn register_plugin() -> *mut dyn Plugin {
    Box::into_raw(Box::new(CppPlugin))
}

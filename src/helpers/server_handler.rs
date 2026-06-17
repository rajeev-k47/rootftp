use crate::constants::{create_rooted_storage, SimpleAuthenticator, UserEntry};
use directories::ProjectDirs;
use libunftp::auth::Authenticator;
use libunftp::ServerBuilder;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;

fn pid_path() -> Option<PathBuf> {
    ProjectDirs::from("net", "runner", "rootftp")
        .map(|d| d.config_dir().join("rootftp.pid"))
}

fn write_pid() {
    if let Some(path) = pid_path() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).ok();
        }
        fs::write(&path, std::process::id().to_string()).ok();
    }
}

fn remove_pid() {
    if let Some(path) = pid_path() {
        fs::remove_file(path).ok();
    }
}

/// Must be called before tokio runtime starts. Exits the parent process.
pub fn daemonize() -> Result<(), Box<dyn std::error::Error>> {
    use std::os::unix::io::AsRawFd;
    match unsafe { libc::fork() } {
        -1 => return Err("first fork failed".into()),
        0 => {}
        _ => std::process::exit(0),
    }
    unsafe { libc::setsid(); }
    match unsafe { libc::fork() } {
        -1 => return Err("second fork failed".into()),
        0 => {}
        _ => std::process::exit(0),
    }
    let devnull = fs::File::open("/dev/null")?;
    let fd = devnull.as_raw_fd();
    unsafe {
        libc::dup2(fd, 0);
        libc::dup2(fd, 1);
        libc::dup2(fd, 2);
    }
    Ok(())
}

pub async fn start_server(
    daemon: bool,
    auth: SimpleAuthenticator,
    root_dir: PathBuf,
    addr: String,
) -> Result<(), Box<dyn std::error::Error>> {
    if daemon {
        return Ok(());
    }

    let dir = root_dir.join("ftpd");
    let storage = create_rooted_storage(dir.clone());
    crate::init::init().await?;

    write_pid();

    let auth_arc: Arc<dyn Authenticator<UserEntry> + Send + Sync> = Arc::new(auth);
    let server = ServerBuilder::with_authenticator(storage, auth_arc)
        .passive_ports(50000u16..=50100u16)
        .build()
        .expect("Failed to build server");

    println!("Listening on {}", addr);

    let server_fut = server.listen(addr);
    let ctrl_c_fut = tokio::signal::ctrl_c();
    let mut term_signal = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
    let term_fut = term_signal.recv();

    tokio::select! {
        result = server_fut => {
            remove_pid();
            result?;
        }
        _ = ctrl_c_fut => {
            println!("\nShutting down...");
            remove_pid();
        }
        _ = term_fut => {
            println!("\nShutting down...");
            remove_pid();
        }
    }

    Ok(())
}

pub async fn stop_server() {
    let path = match pid_path() {
        Some(p) => p,
        None => {
            println!("Could not determine PID file location.");
            return;
        }
    };

    let pid_str = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(_) => {
            println!("Server PID file not found. Is the server running?");
            return;
        }
    };

    let pid: i32 = match pid_str.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid PID in file.");
            return;
        }
    };

    let status = Command::new("kill")
        .arg(pid.to_string())
        .status()
        .expect("Failed to run kill");

    if status.success() {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        fs::remove_file(&path).ok();
        println!("Server (PID {}) stopped.", pid);
    } else {
        println!("Failed to stop server (PID {}).", pid);
    }
}

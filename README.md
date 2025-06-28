[![Crates.io](https://img.shields.io/crates/v/rootftp.svg)](https://crates.io/crates/rootftp)
[![Rust Version](https://img.shields.io/badge/rust-stable-blue.svg)](https://www.rust-lang.org)
[![CI](https://github.com/rajeev-k47/rootftp/actions/workflows/rust.yml/badge.svg)](https://github.com/rajeev-k47/rootftp/actions)
![Termux Tested](https://img.shields.io/badge/Termux-v0.1.3_passing-brightgreen)

<div align="center">
  <img src="assets/logo.png" alt="Logo" width="150" height="200">
  <h1><b>RootFTP</b></h1>
</div>

## Introduction
Rust-based FTP server with with custom configurable, sharing directories across private networks. Built on the top of [libunftp](https://github.com/bolcom/libunftp).


### Features
- **FTP Service**: Enables file transfer over a local network without internet connection.
- **Authentication**: Authentication and persistence of the user crendential at the server over the ftp.
- **Filesystem**: Root each authenticated user to their own directory acts as a independent rooted user.
- **Messaging service**: Each user has an outbox/ and an inbox/ directory, through which they can interact with others.
- **Plugin System**: Write custom plugins to play with your files.
- **CLI**: Commands for everything from starting server to loading plugins.

### Overview
RootFTP provides a space where you can play with files- upload them, transform them, or run actions on them automatically. With plugin support, you can add custom features like code execution, compression, and more.
Interact with other users without internet. Also provides you a custom cloud storage.

## Installation

- Install via Crates.io
 ```bash
  cargo install rootftp
```
- Build from source (Recommanded) ``Additional systemd service``
```bash
git clone https://github.com/rajeev-k47/rootftp.git
cd rootftp
chmod +x ./scripts/install.sh
./scripts/install.sh
```

## Usage
- To start the server
```bash
rootftp start
```
  > Add the -d flag to launch the server in the background as a daemon process.

&nbsp;&nbsp; If installed via script

```bash
sudo systemctl start rootftp
sudo systemctl enable rootftp

```
- General usage
```
Usage: rootftp <COMMAND>

Commands:
  start       Start the RootFTP service
  stop        Stop the RootFTP service
  setdir      Set the FTP root directory
  status      Show the status of the RootFTP service
  loadplugin  Load a local plugin (.so file)
  fetch       Fetch available plugins from the plugin library
  install     Install a plugin by name from the plugin library
  list        List all installed plugins
  help        Print this message or the help of the given subcommand(s)
```
## Directory Layout
 By default (unless you override with setdir), rootftp chooses your home directory:

```bash
$HOME/ftproot/ftpd/
```
Under <root_dir>/ftpd/, each authenticated user automatically gets:
```bash
<root_dir>/ftpd/<username>/
├── home/
├── inbox/
├── outbox/
└── private/
```


```bash
- home/ – Radom dir.
- private/ – Random dir.
- outbox/ – where you place files to send to other users
- inbox/ – (See “Outbox Feature” below.)

```

## Outbox
For sharing files among users (user to user):

  - Create a folder with this convention
 ```bash
   <your_root>/outbox/share.<user_to_send>/ (e.g /outbox/share.xyz)
 ```
  - Put files to share in this folder
  - [inotify](https://docs.rs/inotify/latest/inotify/) watches **outbox/** and shares respective file.
  - Reciever will find these files in their **inbox/** directory
```bash
   /inbox/sender/xyz.txt
 ```

## Plugins

RootFTP has a plugin system that supports pluggable ```.so``` plugins. These plugins are dynamically loaded and automatically invoked when files are added to specific directories.

- Each plugin listens for input files in:
```
<root>/ftpd/<username>/<plugin_name>/input/
```
- When a new file (e.g., main.cpp) is added to the above path,the plugin matching the file extension (e.g. .cpp) is triggered.
- If an optional file named input.in exists in the same directory, it serves as an optional input for the file.
- Then output of that file is written to  
```
 <root>/ftpd/<username>/<plugin_name>/output/<filename>.txt
 ```

### Plugin development
 - Create a ``custom_plugin (or anything else)`` cargo package.
 - RootFTP provides a Plugin trait for development of the plugins. Explore [demo-example](https://github.com/rajeev-k47/rootftp/tree/main/demo_plugin) for more info.
 - Add rootFTP dependency inside plugin's ``Cargo.toml`` file.
```rust
   cargo add rootftp
   ```
   or
```rust
  [dependencies]
  rootftp = 0.1.4 //replace it with latest version
 ```
- Set the dynamic library compile type
```rust
  [lib]
  crate-type = ["cdylib"]
 ```
- Build your plugin as shared object and load it inside rootFTP server.
```rust
  cargo build --release
  //You can find release .so file in /target/release directory.
  rootftp loadplugin /path_to_.so_file
```

### Plugin library
- RootFTP provides a plugin library which contains free plugins. You can fetch any plugin and install it in your server.
- To fetch plugins from **library** use:

   ```rust
  rootftp fetch
   ```
- To get the list of available plugins use:
   ```rust
   rootftp list
   ```
- Install a plugin from library:
   ```rust
   rootftp install <plugin-name> //(Case sensitive with .so extension)
   ```
- To submit your own plugin in plugin library create a pull request.😃

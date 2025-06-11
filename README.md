
# rootftp

Rust-based FTP server with with custom configurable, sharing directories across private networks. Built on the top of [libunftp](https://github.com/bolcom/libunftp).


[![Crates.io](https://img.shields.io/crates/v/rootftp.svg)](https://crates.io/crates/rootftp)
[![Rust Version](https://img.shields.io/badge/rust-stable-blue.svg)](https://www.rust-lang.org)
[![CI](https://github.com/rajeev-k47/rootftp/actions/workflows/rust.yml/badge.svg)](https://github.com/rajeev-k47/rootftp/actions)
![Termux Tested](https://img.shields.io/badge/Termux-v0.1.3_passing-brightgreen)


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
- To stop the server
```bash
rootftp stop
```
- To set a custom root directory:
```bash
rootftp setdir /custom/dir # (e.g. /home/user/Documents)
```
- To check current status/config:
```bash
rootftp status

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
  - Reciever will found these files in his **inbox/** directory
```bash
   /inbox/sender/xyz.txt
 ```

## Plugins (Currently available only when build from source.)

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
 - Make ``custom_plugin (or anything else)`` dir in the root of cloned source code.
 - RootFTP provides a Plugin trait for development of the plugins. Explore [demo-example](https://github.com/rajeev-k47/rootftp/tree/main/demo_plugin) for more info.
 - Add rootFTP dependency inside plugin's ``Cargo.toml`` file.
```rust
  [lib]
  crate-type = ["cdylib"]

  [dependencies]
  rootftp = { path = "../", package = "rootftp" }
 ```
- Build your plugin as shared object and install it inside rootFTP server.
```rust
  cargo build --release
  cp /custom_plugin/target/releases/libcustom_plugin.so ~/<root>/plugins/
```
> Run server atleast once to ensure creating all necessary directories.


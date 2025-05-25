
# rootftp

Rust-based FTP server with with custom configurable, sharing directories across private networks. Built on the top of [libunftp](https://github.com/bolcom/libunftp).


[![Crates.io](https://img.shields.io/crates/v/rootftp.svg)](https://crates.io/crates/rootftp)
[![Rust Version](https://img.shields.io/badge/rust-stable-blue.svg)](https://www.rust-lang.org)
[![CI](https://github.com/rajeev-k47/rootftp/actions/workflows/rust.yml/badge.svg)](https://github.com/rajeev-k47/rootftp/actions)
![Termux Tested](https://img.shields.io/badge/Termux-v0.1.0_passing-brightgreen)


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




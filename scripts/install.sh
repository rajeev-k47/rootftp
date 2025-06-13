#!/bin/bash
cargo build --release
sudo cp target/release/rootftp /usr/local/bin/
sudo cp systemd/rootftp.service /etc/systemd/system/
sudo systemctl daemon-reload
echo "Installation complete. If you want to run rootftp using systemctl use :"
echo "  sudo systemctl start rootftp"
echo "  sudo systemctl enable rootftp"

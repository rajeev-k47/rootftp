#!/bin/bash
cargo build --release
sudo cp target/release/rootftp /usr/local/bin/
sudo cp systemd/rootftp.service /etc/systemd/system/
sudo systemctl daemon-reload
echo "Installation complete. Use:"
echo "  sudo systemctl start rootftp"
echo "  sudo systemctl enable rootftp"

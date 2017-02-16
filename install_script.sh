#!/bin/sh
# basic scripe to setup vanilla debian/ubuntu vm

apt-get update
apt-get upgrade

apt-get install tmux htop git
apt-get install gcc-multilib
curl https://sh.rustup.rs -sSf | sh
rustup upate

sudo apt-get install nginx
git clone https://github.com/stensonowen/wobsite
cd wobsite
cargo build && cargo build --release



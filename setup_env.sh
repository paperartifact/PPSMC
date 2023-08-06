#!/bin.sh
git submodule init
git submodule update
apt update
apt upgrade -y
apt install -y cmake
export RUSTUP_DIST_SERVER="https://rsproxy.cn"
export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
curl --proto '=https' --tlsv1.2 -sSf https://rsproxy.cn/rustup-init.sh | sh
source "$HOME/.cargo/env"
echo "[source.crates-io]
replace-with = 'rsproxy'
[source.rsproxy]
registry = \"https://rsproxy.cn/crates.io-index\"
[registries.rsproxy]
index = \"https://rsproxy.cn/crates.io-index\"
[net]
git-fetch-with-cli = true
" > ~/.cargo/config
export RUSTUP_DIST_SERVER="https://rsproxy.cn"
export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
rustup default nightly
cd ltl2ba
make -j
cd ..
cd ..
wget http://www.lrde.epita.fr/dload/spot/spot-2.11.5.tar.gz
tar -zxvf spot-2.11.5.tar.gz
cd spot-2.11.5
./configure
make -j
cd ..
cd pltlsmc

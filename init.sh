cargo install cargo-wasm
cargo install wapm-cli
rustup target add wasm32-unknown-unknown
curl https://get.wasmer.io -sSfL | sh
source /home/gitpod/.wasmer/wasmer.sh
wasmer login $(cat key)
wapm login $(cat key)
<h1 align=center>Rust-gRPC-Practice</h1>
practicing gRPC client/server implementation in Rust

# Dependencies
- tonic : gRPC implementation
- prost : generate Rust code from protobuf
- tonic-prost : prost codec implementation
- tonic-prost-build : build implementation for prost and tonic
# Usage
clone this repository
```zsh
git clone https://github.com/keeki-fami/Rust-gRPC-Practice.git
cd Rust-gRPC-Practice
```
build this project
```zsh
cargo build
```
open two terminals, run server and client
server
```zsh
cargo run --bin server
```
client
```zsh
cargo run --bin client
```

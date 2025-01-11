# rust_learning
Repo for trying out rust code while learning

# Notes
- If you want to run a different file as a standalone binary, you would need to configure that file as a separate binary in your Cargo.toml under the [bin] section or **put it in the src/bin directory**.

# Install
- Macos : curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
- Update Rust Compiler : rustup update

# Helpful Commands
- Get version : 
    - rustc --version
    - cargo --version
- Compile : rustc main.rs
- Cargo : 
    - Create : cargo new hello_cargo
    - Build only : cargo build
    - Buiold & Run : cargo run
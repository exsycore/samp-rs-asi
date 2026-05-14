# SA:MP Rust ASI Template
This template create for example for develop the ASI Plugin for SA:MP or GTASan

# Features
* Memory Safety: Write mods safely according to standards Rust
* Version Auto-Detection: Version checking system samp.dll (R1, R2, R4, DL etc)
* ASI Wrapper: Ready to compile into a Dynamic Link Library (.dll) with the file extension changed to .asi immediately.
* Windows API Integration: Use windows-rs for background system management.

# Getting Started
## 1. Requirement
* [Rust Toolchain](https://rustup.rs/) - We recommend the stable-x86_64-pc-windows-msvc version.
* **cargo-make**: For helping to build and deploy mods.
```bash
cargo install cargo-make
```
* [ASI Loader](https://github.com/ThirteenAG/Ultimate-ASI-Loader/releases) - Place it in the GTA San Andreas game folder.

## 2. Project installation
```bash
git clone https://github.com/exsycore/samp-rs-asi.git
cd samp-rs-asi
```

## 3. Building
```bash
cargo make
```
or
```bash
cargo build --release --target i686-pc-windows-msvc
```
The file will be located at target/i686-pc-windows-msvc/release/chatimgui_rust.dll. Change the file extension to .asi and place it in the game folder.

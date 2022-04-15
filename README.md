# Neptune

Neptune is a lightweight and speedy image viewer written in Rust, with support for as many image formats as Rust libraries currently exist for. 

## Compiling

### OSX

Make sure you have the Rust toolchain installed; see [https://rustup.rs/](https://rustup.rs/) for an easy-to-install script. 

Once both of the above have been installed, use the following instructions to compile:
```
git clone https://github.com/Pixadus/neptune
cd neptune
cargo build --release
```
To install Neptune, run
```
cargo install neptune
```
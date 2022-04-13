# Neptune

Neptune is a lightweight and speedy image viewer written in Rust, with support for as many image formats as Rust libraries currently exist for. 

## Compiling

### OSX

Make sure you have the Rust toolchain installed; see [https://rustup.rs/](https://rustup.rs/) for an easy-to-install script. 

Neptune also runs using the new GTK4 libraries. You can install these using
```
brew install gtk4
```

Once both of the above have been installed, use the following instructions to compile:
```
git clone https://github.com/Pixadus/neptune
cd Neptune
cargo build --release
```
To install Neptune, run
```
cargo install neptune
```
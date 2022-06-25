# Installation (macOS and Linux)

## Install Rust and Cargo
Install the Rustup toolchain installer using the [installation instructions][1] at the Rust language website. The Rustup installation will install the `rustc` compiler and `cargo` build management system.


## Get Source
Use `git` to clone the comp source code using the following command.
```
git clone https://github.com/usefulmove/comp.git
```

## Build
Build the comp binary for your operating system from source using `cargo`.
```
cd comp
cargo build --release
```

This will create a `comp` binary executable in a `target/release` folder. The executable can be run from this location by direct reference or added to your `/usr/local/bin` folder as a symbolic link to be accessed from the command line anywhere.
```
sudo ln -sr ./target/release/comp /usr/local/bin
```

You can check that the installation was successful by running the `comp version` command.
```
comp version
0.15.4
```


[1]: https://rust-lang.org/tools/install

# Installation (macOS and Linux)

## Install Rust and Cargo
Install the Rustup toolchain installer using the [installation instructions][1] at the Rust language website. The Rustup installation will install the `rustc` compiler and `cargo` build management system.


## Get source code
Use `git` to clone the comp source code using the following command.
```
git clone https://github.com/usefulmove/comp.git
```

## Build from source
Build the comp binary for your operating system from source using `cargo`.
```
cd comp
cargo build --release
```

This will create a `comp` executable binary in a `target/release` folder. The executable can be run from this location by direct reference or added to a folder in your $PATH to make it accessible from anywhere on the command line. An example of adding a symbolic link to the `/usr/local/bin` folder using the link (`ln`) command is shown below.

(Note that the -r option shown is not supported in the macOS version of the ln command, and the full path to the comp executable will have to be given instead of the relative path shown in the example.)
```
sudo ln -s -r ./target/release/comp /usr/local/bin
```

You can check that the installation was successful by running the `comp version` command.
```
comp version
0.23.5
```


[1]: https://rust-lang.org/tools/install

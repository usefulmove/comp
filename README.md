# comp

![](https://img.shields.io/badge/stable-0.18.4-green?style=plastic)
![](https://img.shields.io/badge/license-MIT-blue?style=plastic)

* [Usage instructions][1]
* [Installation][2]
* [License][3]

A simple and fast, reverse Polish notation (RPN) interpreter written in Rust. The interpreter implements a stack-based language similar to Forth that is inspired by the calculation interface of the original HP scientific calculators of the 1970s. The posfix language consists of lists of elements where each element is either a value to be added to the stack or an operation. As examples, `comp 3 4 +` adds the values 3 and 4 and `comp 3 dup x 4 dup x +` computes the sum of the squares of 3 and 4.

For more information, see the comp command [usage documentation][1].

---

## Usage
The basic usage of the comp interpreter can be accessed in the output of the `comp help` command. Detailed [usage information][1] and descriptions of each available command can be found in [`USAGE.md`][1].
```
comp help
```

![](https://raw.githubusercontent.com/usefulmove/comp/main/usage.png)

---

## Installation
The comp interpreter can be installed by installing the [`Rustup`][4] toolchain installer and using the `cargo` build management system to build comp from source on your system. Detailed [installation instructions][2] can be found in [`INSTALL.md`][2].

---

## License
The comp interpreter is available under the MIT License. The MIT License is a permissive free software license with very limited restrictions on reuse. The full license text can be found in the [`LICENSE.md`][3] file.

[1]: ./USAGE.md
[2]: ./INSTALL.md
[3]: ./LICENSE
[4]: https://rust-lang.org/tools/install

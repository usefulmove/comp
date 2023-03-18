# comp

<img src="https://raw.githubusercontent.com/usefulmove/comp/main/assets/system-preferences-icon-64x64.png" align="right"/>

![](https://img.shields.io/badge/stable-0.25.8-success?style=for-the-badge)
![](https://img.shields.io/badge/license-MIT-informational?style=for-the-badge)

* [Installation][2]
* [Usage Guide][1]
* [License][3]
* [Issues][5]
* [Feature Requests][6]

A streamlined, stack-based interpreter developed in Rust. The interpreter brings to life a high-level, reverse-Polish language, reminiscent of Forth, and draws inspiration from the command interface of the pioneering HP scientific calculators from the 1970s. The postfix language is composed of element lists, with each element representing either a value to be pushed onto the stack or an operation to be performed. For instance, 'comp 3 4 +' adds the values 3 and 4, while 'comp 3 dup x' calculates the square of 3.

For more information, see the comp command [usage documentation][1].

---

## Usage
The basic usage of the comp interpreter can be accessed in the output of the `comp help` command. A detailed [usage guide][1] with descriptions of each available command can be found in [`USAGE.md`][1].
```
comp help
```

![](https://raw.githubusercontent.com/usefulmove/comp/main/assets/usage.png)

---

## Installation
### Binaries
Release binaries are provided on the [Releases](https://github.com/usefulmove/comp/releases) page and can be installed manually.

### From source
The comp interpreter can be installed by installing the [`Rustup`][4] toolchain installer and using the `cargo` build management system to build comp from on your system from the source code. Detailed [installation instructions][2] can be found in [`INSTALL.md`][2].

---

## License
The comp interpreter is available under the MIT License. The MIT License is a permissive free software license with very limited restrictions on reuse. The full license text can be found in the [`LICENSE.md`][3] file.

[1]: ./USAGE.md
[2]: ./INSTALL.md
[3]: ./LICENSE
[4]: https://rust-lang.org/tools/install
[5]: https://github.com/usefulmove/comp/issues
[6]: https://github.com/usefulmove/comp/labels/feature%20request

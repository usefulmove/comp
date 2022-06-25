# comp
A simple and fast, reverse Polish notation (RPN) calculation interface. The interface was initially developed as a personal project to build a basic lisp interpreter in the Julia programming language. It was later rewritten in Rust. It implements a postfix notation similar to the Forth language and the interface implemented on the original Hewlett-Packard scientific calculators. The interface accepts a sequence of postfix operations. Each operation is either a value or a command. As examples, `comp 3 4 +` adds the values 3 and 4 and `comp 3 dup x 4 dup x +` computes the sum of the squares of 3 and 4.

For more information, see the comp command usage documentation below.


---

## License
The comp interpreter is available under the MIT License. The MIT License is a permissive free software license with very limited restrictions on reuse. The full license text can be found in the [`LICENSE.md`][1] file.


---

## Installation
The comp intepreter can be installed by copying the source code and installing [`Rust`][2] and the `rustup` installer and using the `cargo` build management system to build comp from source on your system. Detailed instructions can be found in [INSTALL.md][3].


---

## Usage
The basic usage of the comp intepreter can be accessed in the output of the `comp help` command. More detailed usage information and descriptions of each available command can be found in [USAGE.md][4].
```
comp help
```

![](https://raw.githubusercontent.com/usefulmove/comp/main/usage.png)


[1]: ./LICENSE
[2]: https://rust-lang.org/tools/install
[3]: ./INSTALL.md
[4]: ./USAGE.md

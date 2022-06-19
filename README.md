# comp
A clean, reverse Polish notation (RPN) calculation interface. The interface was initially developed as a personal project to build a basic Lisp interpreter in the Julia programming language. It implements a postfix notation similar to the Forth language and the interface implemented on the original Hewlett-Packard scientific calculators. The interface accepts a sequence of postfix operations. Each operation is either a value or a command. As examples, `comp 3 4 +` adds the values 3 and 4 and `comp 3 dup x 4 dup x +` computes the sum of the squares of 3 and 4.

For more information, see the comp reference documentation (in progress).

### Usage Information and Command List
```
comp help
```

![](https://raw.githubusercontent.com/usefulmove/comp/main/usage.png)

### License
The `comp` interpreter is available under the MIT License. The MIT License is a permissive free software license with very limited restrictions on reuse. The full license text can be found in the [`LICENSE.md`][1] file.

[1]: ./LICENSE.md

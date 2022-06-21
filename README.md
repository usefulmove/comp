# comp
A clean, reverse Polish notation (RPN) calculation interface. The interface was initially developed as a personal project to build a basic lisp interpreter in the Julia programming language. It implements a postfix notation similar to the Forth language and the interface implemented on the original Hewlett-Packard scientific calculators. The interface accepts a sequence of postfix operations. Each operation is either a value or a command. As examples, `comp 3 4 +` adds the values 3 and 4 and `comp 3 dup x 4 dup x +` computes the sum of the squares of 3 and 4.

For more information, see the comp command documentation below.

## License
The `comp` interpreter is available under the MIT License. The MIT License is a permissive free software license with very limited restrictions on reuse. The full license text can be found in the [`LICENSE.md`][1] file.

---

## Usage
```
comp help
```

![](https://raw.githubusercontent.com/usefulmove/comp/main/usage.png)

---

## Commands (stack manipulation)

### `push onto stack`
```
comp 3 4
3.0
4.0
```

### `drop`
```
comp 3 4 drop
3.0
```

### `duplicate`
duplicate last element
```
comp 3 4 dup
3.0
4.0
4.0
```

### `swap`
reverse order of last two elements
```
comp 1 2 3 4 swap
1.0
2.0
4.0
3.0
```

### `clear stack`
reverse order of last two elements
```
comp 1 2 3 4 cls

```

### `roll stack`
rotate stack elements such that the last element becomes the first
```
comp 1 2 3 4 roll
4.0
1.0
2.0
3.0
```


---

## Commands (memory usage)

### `store and retrieve`
The values `a b c` can be stored using the store command (e.g, `sa`) into memory for retrieval (e.g., `a`) in subsequent opertions. The stored value is removed from the stack when the store command is executed.
```
comp 1 2 3 sa drop a
1.0
3.0
```


---

## Commands (math operations)

### `add`
```
comp 3 4 +
7.0
```

### `subtract`
```
comp 3 4 -
-1.0
```

### `multiply`
```
comp 3 4 x
12.0
```

### `divide`
```
comp 3 4 /
0.75
```

### `add all`
```
comp 1 2 3 4 +_
10.0
```

### `multiply all`
```
comp 1 2 3 4 x_
24.0
```

### `change sign`
```
comp 3 chs
-3.0
```

### `absolute value`
```
comp -3 abs
3.0
```

### `round`
```
comp 10.2 round
10.0
```

### `invert (1/x)`
```
comp 3 inv
0.3333333333333333
```

### `square root`
```
comp 2 sqrt
1.4142135623730951
```

### `nth root`
```
comp 9 2 throot
3.0
```

### `find principal roots`
For this operation, the coefficients `a b c` of the quadratic equation `ax^2 + bx + c = 0` are pushed onto the stack. The real and imaginary components of the principal roots (root1 and root2) of the equation are returned to the stack in the order `real1 imag1 real2 imag2`. The example below finds the roots of the equation `x^2 - 9 = 0`.
```
comp 1 0 -9 proot
3.0
0.0
-3.0
0.0
```

### `exponetiation`
```
comp 2 4 ^
16.0
```

### `modulus`
```
comp 5 2 %
1.0
```

### `factorial`
```
comp 5 !
120.0
```

### `greatest common devisor`
```
comp 10 55 gcd
5.0
```

### `pi`
```
comp pi
3.141592653589793
```

### `Euler's number (e)`
```
comp e
2.718281828459045
```

### `convert degrees to radians / arcsine`
```
comp pi 2 /
1.5707963267948966

comp pi 2 / rtod
90.0

comp 90 dtor
1.5707963267948966
```

### `sine / arcsine`
```
comp pi 2 / sin
1.0

comp pi 2 / sin asin
1.5707963267948966
```

### `cosine / arcosine`
```
comp 0 cos
1.0

comp 0 cos acos
0.0
```

### `tangent / arctangent`
```
comp 4 pi / tan
0.9999999999999999

comp 4 pi / tan atan 4 x
3.141592653589793
```

### `log (base 10)`
```
comp 10 2 ^ log
2.0
```

### `natural log`
```
comp e ln
1.0
```


[1]: ./LICENSE

# Usage Guide

- [stack manipulation](#commands-stack-manipulation)
- [memory usage](#commands-memory-usage)
- [maths](#commands-maths)
- [conversion](#commands-conversion)
- [file usage](#commands-file-usage)
- [control flow](#commands-control-flow)
- [configuration]


## Commands (stack manipulation)

### push onto stack
```
% comp 3 4
  3
  4
```

### drop
```
% comp 3 4 drop
  3
```

### duplicate
duplicate last element
```
% comp 3 4 dup
  3
  4
  4
```

### swap
reverse order of last two elements
```
% comp 1 2 3 4 swap
  1
  2
  4
  3
```

### clear stack
reverse order of last two elements
```
% comp 1 2 3 4 cls

```

### roll stack
rotate stack elements such that the last element becomes the first
```
% comp 1 2 3 4 roll
  4
  1
  2
  3
```

### rotate stack
rotate stack elements such that the first element becomes the last (reverse direction from roll operation)
```
% comp 1 2 3 4 rot
  2
  3
  4
  1
```


---
## Commands (memory usage)

### store and retrieve
The values `a b c` can be stored using the store command (e.g, `sa`) into memory for retrieval (e.g., `a`) in subsequent operations. The stored value is removed from the stack when the store command is executed.
```
% comp 1 2 3 sa drop a
  1
  3
```


## Commands (maths)

### add
```
% comp 3 4 +
  7
```

### add all
```
% comp 1 2 3 4 +_
  10
```

### add one
```
% comp 1 ++
  2
```

### subtract
```
% comp 3 4 -
  -1
```

### subtract one
```
% comp 1 --
  0
```

### multiply
```
% comp 3 4 x
  12
```

### multiply all
```
% comp 1 2 3 4 x_
  24
```

### divide
```
% comp 3 4 /
  0.75
```

### change sign
```
% comp 3 chs
  -3
```

### absolute value
```
% comp -3 abs
  3
```

### round
```
% comp 10.2 round
  10
```

### invert (1/x)
```
% comp 3 inv
  0.3333333333333333
```

### square root
```
% comp 2 sqrt
  1.4142135623730951
```

### nth root
```
% comp 9 2 throot
  3
```

### find principal roots
For this operation, the coefficients `a b c` of the quadratic equation `ax^2 + bx + c = 0` are pushed onto the stack. The real and imaginary components of the principal roots (root1 and root2) of the equation are returned to the stack in the order `real1 imag1 real2 imag2`. The example below finds the roots of the equation `x^2 - 9 = 0`.
```
% comp 1 0 -9 proot
  3
  0
  -3
  0
```

### exponentiation
```
% comp 2 4 ^
  16
```

### modulus
```
% comp 5 2 %
  1
```

### factorial
```
% comp 5 !
  120
```

### greatest common divisor
```
% comp 10 55 gcd
  5
```

### pi
```
% comp pi
  3.141592653589793
```

### Euler's number (e)
```
% comp e
  2.718281828459045
```

### convert degrees to radians (and reverse)
```
% comp pi 2 /
  1.5707963267948966

% comp pi 2 / rad_deg
  90

% comp 90 deg_rad
  1.5707963267948966
```

### sine / arcsine
```
% comp pi 2 / sin
  1

% comp pi 2 / sin asin
  1.5707963267948966
```

### cosine / arcosine
```
% comp 0 cos
  1

% comp 0 cos acos
  0
```

### tangent / arctangent
```
% comp pi 4 / tan
  0.9999999999999999

% comp pi 4 / tan atan 4 x
  3.141592653589793
```

### log (base 10)
```
% comp 10 2 ^ log
  2
```

### log (base 2)
```
% comp 256 log2
  8
```

### log (base n)
```
% comp 256 2 logn
  8
```

### natural log
```
% comp e ln
  1
```

### max
```
% comp 1 2 3 4 max
  4
```

### min
```
% comp 1 2 3 4 min
  1
```

### rand
Reads positive integer (n) from stack and returns a random integer in the range 0 to n-1.
```
% comp 6 rand
  3

% comp 6 rand
  5
```


---
## Commands (conversion)

### convert between hexadecimal, binary, and decimal
```
% comp c0 hex_dec
  192

% comp 192 dec_hex
  c0

% comp 192 dec_bin
  1100000

% comp 11000000 bin_dec
  192

% comp 1100000 bin_hex
  c0

% comp c0 hex_bin
  11000000
```

### temperature conversion (Fahrenheit, Celsius)
```
% comp 212 F_C
  100
```

```
% comp 0 C_F
  32
```


---
## Commands (file usage)

### -f option (also --file)
The file flag allows the use of commands defined within a source file.
```
% comp -f <filepath>
```


---
## Commands (control flow)

### functions
User-defined functions can be defined by indicating the start of a function with an open parenthesis `(` symbol followed by the function name then a list of operations and terminated with the close parenthesis `)` symbol. The user function is executed by calling the function name as shown in the examples below.

( Note that on many systems, at the command prompt the parentheses must be escaped as shown the examples below. This is not necessary for functions defined within files. )
```
% comp \( square dup x \) 16 square
  256
```
```
% comp \( double 2 x \) 250 double
  500
```

Note that functions are most useful when combined with the `-f` file option. The cube function can be defined and executed in a source file and passed to the comp command using the file option.
```
{ cube.cm }
{ note - comments are identified inside curly brackets.
  there must be whitespace surrounding each bracket.
  multiline comments are supported. }

( cube
  3 ^
)

8 cube
```
```
% comp -f cube.cm
  512
```

Functions also can be defined in a file and used in operations passed after the file has been processed.
```
{ temperature.cm }

( ctof
  9 x 5 /
  32 +
)

( ftoc
  32 -
  5 x 9 /
)
```
```
% comp -f temperature.cm 0 ctof
  32
```

## Commands (configuration)

### save configuration
Save comp.toml configuration file in the home directory. This file can be used to configure some aspects of the behavior of the application like indicating the top of the stack and displaying monochrome output.
```
% comp save_cfg
```
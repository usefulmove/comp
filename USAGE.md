# Usage Guide

- [stack manipulation](#commands-stack-manipulation)
- [memory usage](#commands-memory-usage)
- [maths](#commands-maths)
- [conversion](#commands-conversion)
- [file usage](#commands-file-usage)
- [user-defined functions](#commands-user-defined-functions)
- [higher-order functions](#commands-higher-order-functions)
- [configuration](#commands-configuration)


---
## Commands (stack manipulation)

### push onto stack
```
% comp 3 4
  3
  4
```

### drop / dropn
drop element from the top of the stack
```
% comp 3 4 drop
  3
```
drop n elements from the top of the stack
```
% comp 1 2 3 4 2 dropn
  1
  2
```

### take / taken
take the element from the top of the stack
```
% comp 3 4 take
  4
```
take n elements from the top of the stack
```
% comp 1 2 3 4 2 taken
  3
  4
```

### duplicate (dup)
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

### clear stack (cls)
reverse order of last two elements
```
% comp 1 2 3 4 cls

```

### roll / rolln
roll stack elements such that the last element becomes the first
```
% comp 1 2 3 4 roll
  4
  1
  2
  3
```
roll stack n times
```
% comp 1 2 3 4 2 rolln
  3
  4
  1
  2
```

### rotate (rot / rotn)
rotate stack elements such that the first element becomes the last (reverse direction from roll operation)
```
% comp 1 2 3 4 rot
  2
  3
  4
  1
```
rotate stack n times
```
% comp 1 2 3 4 2 rotn
  3
  4
  1
  2
```

### iota (io)
push an integer range (starting at 1) onto the stack
```
% comp 8 io
  1
  2
  3
  4
  5
  6
  7
  8
```
push an integer range (starting at 0) onto the stack
```
% comp 3 i0
  0
  1
  2
  3
```

### range (to)
push a number range onto the stack by specifying the range start, end, and step size
```
% comp 0 10 2 to
  0
  2
  4
  6
  8
  10
```
```
% comp 20 -10 5 to
  20
  15
  10
  5
  0
  -5
  -10
```


---
## Commands (memory usage)

### store and retrieve
 Values can be stored using generic variables that are not associated with either built-in commands or user-defined functions using the `store` command as shown below.
```
% comp 1 2 3 saved_here store 4 5 6 cls saved_here
  3
```


---
## Commands (maths)

### add
```
% comp 3 4 +
  7
```

### sum (add all)
```
% comp 1 2 3 4 sum
  10
```
```
% comp 1 2 3 4 +_
  10
```

### subtract
```
% comp 3 4 -
  -1
```

### multiply
```
% comp 3 4 x
  12
```

### product (multiply all)
```
% comp 1 2 3 4 prod
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

### floor
```
% comp 10.2 floor
  10
```

### ceiling
```
% comp 10.2 ceil
  11
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
% comp 9 2 nroot
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

### max, max_all
return the maximum of the last two elements on the stack
```
% comp 1 2 3 4 max
  1
  2
  4
```
return the maximum of all elements on the stack
```
% comp 1 2 3 4 max_all
  4
```

### min, min_all
return the minimum of the last two elements on the stack
```
% comp 1 2 3 4 min
  1
  2
  3
```
return the minimum of all elements on the stack
```
% comp 1 2 3 4 min_all
  1
```

### avg, avg_all
return the average of the last two elements on the stack
```
% comp 1 2 3 4 avg
  1
  2
  3.5
```
return the average of all elements on the stack
```
% comp 1 2 3 4 avg_all
  2.5
```

### rand
read positive integer (n) from stack and returns a random integer in the range 0 to n-1
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

### length conversion (miles, kilometers) (feet, meters)
```
% comp 1 mi_km
  1.609344
```
```
% comp 1 m_ft
  3.281
```


---
## Commands (file usage)

### -f option (also --file)
The file flag allows the use of commands defined within a source file.
```
% comp -f <filepath>
```


---
## Commands (user-defined functions)

### function definition
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

A version of a recursive factorial function that takes an integer argument is below.
```
( factorial
  dup 1
  ifeq
    drop
    1
  else
    dup 1 - factorial x
  fi
)
```


---
## Commands (higher-order functions)

### map
map an anonymous function to each of the stack elements
```
% comp 1 2 3 4 5 [ 3 ^ ] map
  1
  8
  27
  64
  125
  216
  343
  512
  729
  1000
```

**Note: Commands that manipulate the entire stack (such as `sum`, `prod`, `cls`) cannot be used in anonymous functions passed to higher-order functions.

### fold (reduce)
use an anonymous function to collapse the values on the stack into a single value
```
{ Compute the sum of the squares of the numbers from 1 to 5. (The 0 passed
  into the fold command is the initial value of the accumulator.) }

% comp 10 11 12 13 14 15 [ + ] 0 fold
  75
```
```
{ Compute the product of the squares of the values from 1 to 5. }

% comp 1 2 3 4 5 [ dup x x ] 1 fold
  14400
```

As another example, `fold` be used with the `to range operator to calculate the sum of the reciprocals of powers of 2 as shown below.
<img src="https://raw.githubusercontent.com/usefulmove/comp/main/assets/series-of-reciprocals-of-powers-of-2.jpg" align="center"/>
<br>
```
% comp 0 100 1 to [ 2 swap ^ inv + ] 0 fold
  2
```

### scan
use an anonymous function and the state of the previous item on the stack to update each stack item
```
% comp 8 io [ + ] scan
  1
  3
  6
  10
  15
  21
  28
  36
```

---
## Commands (configuration)

### save configuration
Save comp.toml configuration file in the home directory. This file can be used to configure some aspects of the behavior of the application like indicating the top of the stack and displaying monochrome output. If any problems are encountered with loading the configuration file, the file can be deleted from the home directory, and the default configuration will be loaded.
```
% comp save_config
```

### show configuration
```
% comp show_config

show_stack_level = true
conversion_constant = 1.61803
monochrome = false
tip_percentage = 0.15
show_warnings = true

```
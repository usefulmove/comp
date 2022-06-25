# Usage


## Commands (stack manipulation)

### push onto stack
```
comp 3 4
3
4
```

### drop
```
comp 3 4 drop
3
```

### duplicate
duplicate last element
```
comp 3 4 dup
3
4
4
```

### swap
reverse order of last two elements
```
comp 1 2 3 4 swap
1
2
4
3pm 
```

### clear stack
reverse order of last two elements
```
comp 1 2 3 4 cls

```

### roll stack
rotate stack elements such that the last element becomes the first
```
comp 1 2 3 4 roll
4
1
2
3
```


---
## Commands (memory usage)

### store and retrieve
The values `a b c` can be stored using the store command (e.g, `sa`) into memory for retrieval (e.g., `a`) in subsequent opertions. The stored value is removed from the stack when the store command is executed.
```
comp 1 2 3 sa drop a
1
3
```


## Commands (math operations)

### add
```
comp 3 4 +
7
```

### subtract
```
comp 3 4 -
-1
```

### multiply
```
comp 3 4 x
12
```

### divide
```
comp 3 4 /
0.75
```

### add all
```
comp 1 2 3 4 +_
10
```

### multiply all
```
comp 1 2 3 4 x_
24
```

### change sign
```
comp 3 chs
-3
```

### absolute value
```
comp -3 abs
3
```

### round
```
comp 10.2 round
10
```

### invert (1/x)
```
comp 3 inv
0.3333333333333333
```

### square root
```
comp 2 sqrt
1.4142135623730951
```

### nth root
```
comp 9 2 throot
3
```

### find principal roots
For this operation, the coefficients `a b c` of the quadratic equation `ax^2 + bx + c = 0` are pushed onto the stack. The real and imaginary components of the principal roots (root1 and root2) of the equation are returned to the stack in the order `real1 imag1 real2 imag2`. The example below finds the roots of the equation `x^2 - 9 = 0`.
```
comp 1 0 -9 proot
3
0
-3
0
```

### exponetiation
```
comp 2 4 ^
16
```

### modulus
```
comp 5 2 %
1
```

### factorial
```
comp 5 !
120
```

### greatest common divisor
```
comp 10 55 gcd
5
```

### pi
```
comp pi
3.141592653589793
```

### Euler's number (e)
```
comp e
2.718281828459045
```

### convert degrees to radians / arcsine
```
comp pi 2 /
1.5707963267948966

comp pi 2 / rtod
90

comp 90 dtor
1.5707963267948966
```

### sine / arcsine
```
comp pi 2 / sin
1

comp pi 2 / sin asin
1.5707963267948966
```

### cosine / arcosine
```
comp 0 cos
1

comp 0 cos acos
0
```

### tangent / arctangent
```
comp pi 4 / tan
0.9999999999999999

comp pi 4 / tan atan 4 x
3.141592653589793
```

### log (base 10)
```
comp 10 2 ^ log
2
```

### natural log
```
comp e ln
1
```


---
## Commands (file usage)

### --file flag
The file flag allows the use of commands defined within a source file.
```
comp --file <filename>
```
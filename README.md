# meowmeow

A simple, dynyamically-typed cat-oriented programming language.

## Basic Syntax

meowmeow uses prefix notation, like other languages such as [uiua](https://uiua.org) and [lisp](https://lisp-lang.org). That means instead of typing things like `variable = 2 + 2`, you would type something like `= variable + 2 2`.

### Naming Scheme

All variables and functions must be a meow decided by the regex `mr+[iye]?[aoe]*[wu]*r?~*`. Variables end with a tilde (~) while functions do not.
- meow
- mreow
- mraow~
- miaou
- mrawr~
- mrrriaaaoooeeuuwwwr~

All labels and control flow functions must be a nya. Labels end with a tilde (~) while control flow functions do not.
- nya
- nyan
- nyyaaaa~
- nyyyaaaaan~

All opurrators are a purr.
- prrr
- purrr
- puurr

### Numbers

Numbers can either be written in unary or concatenated decimal.

Writing a number in unary is simple, write X amount of rs after an m.
- `mrrrr` = 4
- `mrrrrrrrrr` = 9
- `m` = 0

You can also make a number negative by adding a p at the end.
- `mrrp` = -2

Writing a number in concatenated decimal is also simple. For each digit of its decimal representation, write that amount of rs, then follow it with a w.
- `mrrrrwrr` = 42
- `mrrrrrrrrrwwwrp` = -9001
- `mrwrrwrrrrrrrrwrrrrrwrrrrrrrwrr` = 128572

Concatenated decimal was a suggestion from early feedback to help write large numbers instead of having to use arithmetic operators to compute large numbers without concatenated decimal.

`pur purrr pur purrr mrrrrrrrrrrrrrr pur purrr mrrrr mrrrrr mr mrrrrr pur purrr pur purrr mrrrr mrrrr mr purrr mrrrrr mrrrrr mrrrrr mrr` = (14 * (4 * 5 + 1) + 5) * ((4 * 4 + 1) * (5 * 5) + 5) + 2 = 128572

### Functions and Opurrators

Functions and opurrators are syntactically the same, but functions tend to have side effects whereas opurrators do not. They take their arguments after their function call. Each function/opurrator has a set amount of arguments that can be seen in the list of functions and operators below.

For example `mreow` is the print number function, and it takes in one argument. So, the code `mreow mrr` would print `2` to the screen.

You can use the result of operators and functions and pass them into more functions on the same instruction/line.
- `mreow pur mrr mrr`: `printn + 2 2`, prints 2 + 2 (pur is the opurrator for addition)
- `mew meow~ pur mr purrr mrrr meow~`: `set meow~ + 1 * 3 meow~`, sets the variable `meow~` to 3 * `meow~` + 1

Each expression that isn't used as an argument into another expression is called an instruction. You can see the grouping of each instruction by using the `--debug` setting when running your code using the interpreter.

This means that whitespace/indentation doesn't matter in meowmeow, and you may format your code any way you like, and it'll act the same way as long as each symbol is seperated by spaces on both sides.

```py
mew meow~ mrrrr
mew meoww~ mrrrrr
mreowr meow~
meowr " * "
mreowr meoww~
meowr " = "
mreow purrr meow~ meoww~

# These two programs do the same thing
mew meow~ mrrrr mew meoww~ mrrrrr mreowr meow~ meowr " * " mreowr meoww~ meowr " = " mreow purrr meow~ meoww~
```

### Variables

Variables in meowmeow, as mentioned before, must follow the meow regex listed in the *Naming Scheme* section.

All variables are a meow ending with a tilde (~). Variables can be set to using the `mew` function.

```py
mew meow~ pur mrr mrr       # meow~ = 2 + 2
mreow meow~                 # printn meow~
```

Variables can be set to any type listed in the *Types* section. Different types can be set to the same variable.

```py
mew meow~ "Hello World!"    # meow~ = "Hello World!" (strings are automatically converted into arrays)
mew meow~ mrrrrrrrrr        # meow~ = 9
mew meow~ nyull             # meow~ = nyull
```

#### Indexing into Arrays

If the variable is an array, you can index into the array by prepending the variable name with a number.

```py
mew meow~ "chaotic"         # meow~ = "chaotic"

meowr mmeow~                # prints "chaotic"[0] = c
meowr mrrmeow~              # prints "chaotic"[2] = a
meowr mrrrrmeow~            # prints "chaotic"[4] = t

mew mrrrmeow~ miaor "😼"    # sets the 4th item in meow~ to 😼
meow mrrrmeow~              # prints 😼
```

#### Variable Variables

> "Sometimes it is convenient to be able to have variable variable names. That is, a variable name which can be set and used dynamically." - PHP Manual

```py
mew mraow~ "Hello World!"
mew mreow~ "mraow"
mew meow~ "mreow"

meow meow~~~                    # Hello World!

mew meow~~~ "kitty meow meow"
meow mreow~~                    # kitty meow meow
```

#### Types

meowmeow has 3 types: nyull, numbers, and arrays.

nyull is similar to null in other languages. If a function can't take nyull, it will throw an error.

Numbers are always signed 64-bit integers.

Arrays are lists of 64-bit integers. You can create an array by either typing it in as a string (e.g. `"Cat"` -> \[67, 97, 116\]) or converting another value into an array using the `puurrr [value]` operator (e.g. `puurrr mrrr` -> \[3\])

Some functions/opurrators will expect certain arguments to be certain types, and the code will throw an error if they aren't correct. Some functions/opurrators will have different behaviors if you input different types. For example, the `pur` operator, which does addition, can also combine arrays.

```py
mreow pur mrrr mrrp         # -6
mreow pur "kitty " "cat"    # kitty cat
```

### Comments

Comments start with a `#` and last until the end of the line.

```py
meow "This will print out" # I am a comment and will do nothing
meow "This will also print out"
```

## Operators

### Boolean (prr)
- `pr [value]`: converts \[value\] to either 0 or 1
- `prr [value]`: if \[value\] is a number greater than 0, return 0, else return 1
- `prrr [a] [b]`: returns 1 if \[a\] and \[b\] are equal
- `prrrr [a] [b]`: boolean AND between \[a\] and \[b\]
- `prrrrr [a] [b]`: boolean OR between \[a\] and \[b\]
- `prrrrrr [a] [b]`: boolean XOR between \[a\] and \[b\]

### Arithmetic (purr)
- `pur [a] [b]`: adds \[a\] and \[b\]
- `purr [a] [b]`: subtracts \[a\] from \[b\] (NOTE: computes b - a)
- `purrr [a] [b]`: multiplies \[a\] and \[b\]
- `purrrr [a] [b]`: divides \[b\] by \[a\] (NOTE: computes b / a)
- `purrrrr [a] [b]`: modulo, finds the remainder of \[b\] / \[a\] (NOTE: computes b % a)
- `purrrrrr [a] [b]`: raises \[b\] to the power of \[a\] (NOTE: computes b ^ a)
- `purrrrrrr [value]`: gets the square root of \[value\]
- `purrrrrrrr [value]`: gets the absolute value of \[value\]

### Array (puurr)
- `puur [i] [arr]`: gets the i+1th item in \[arr\]
- `puurr [arr]`: gets the length of the array \[arr\]
- `puurrr [value]`: converts \[value\] into an array
    - if \[value\] is a number, put the number in an empty array and return it
    - if \[value\] is an array, return \[value\]
    - if \[value\] is nyull, return an empty array

## Functions
### Input/Variable Setting

All functions return either the previous value of the variable or `nyull`.

- `mew [variable~] [value]`: Sets a variable to a value
- `miaw [variable~]`: Sets a variable to the first character inputted
- `miawr [variable~]`: Sets a variable to an array of the inputted characters
- `mriaw [variable~]`: Sets a variable to the number inputted
- `mriawr [variable~]`: Sets a variable to the array of numbers inputted

### Output

All functions return their outputted variable.

- `meow [value]`: Outputs a value to the console as its unicode character
- `mreow [value]`: Outputs a value to the console as a number
- `meowr [value]`: Same as `meow` but without a newline
- `mreowr [value]`: Same as `mreow` but without a newline

### Array Manipulation

- `miao [value] [variable~]`: Pushes a value to the variable and returns that value
- `miaor [variable~]`: Pops a value from the end of the variable

### Control Flow

- `nya [label~]`: Jumps to the instruction with the label
- `nyan [value] [label~]`: If value > 0, jumps to the instruction with the label

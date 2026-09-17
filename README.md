# Introduction  
-C (Negative C) is a low level programming language that compiles to PIKU assembly
(used in my custom cpu, [PIKU](https://github.com/kirbytheburger/PIKU)).
It's based on the popular C programming language, but it's just a little worse (hence the name Negative C).
Just like C, -C is statically typed and has manual memory management.  

# Overall project structure
The stages that code goes through during compilation are the following:
```
source code -> lexer -> parser -> IR compiler -> writer
```
The lexer produces tokens and passes it to the parser, which transforms them into an AST,
which the IR compiler compiles into an IR enum (with infinite virtual registers),
and the writer then transforms that into actual PIKU assembly while managing the registers.
Note that the writer produces PIKU assembly code and not bytecode for ease of debugging.

# Documentation  
-C is still very much in progress, so the documentation is far from complete.
Below are the current features the language offers:  

## Data types
-C currently has 2 data types, `int` and `char`.
An int is a 16-bit unsigned integer (because PIKU is a 16-bit CPU),
and a char is an 8-bit number which represents an ASCII value.
```
// This is an int
5

// This is a char
'a'
```

## Printing
the `print` keyword will output values to the terminal, just like any other language:
```
// Output: 12
print 12

// Output: e
print 'e'
```

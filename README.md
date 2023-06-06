# mindheck

![Image](image/mindheck.png)

Mindheck is a language designed to be easier than brainfuck and compile to it.
The compiler and brainfuck runner are written in Rust.

## Syntax

To write a command, type the command.
```
print("Hello World!")
```

No semicolons here! We are doing it the Python way.

## Functions

There are currently ... functions.

### print
```
print("<string>")
```
prints \<string>

```
print(100)
```
prints whatever is in the tape at place 100.

### input
```
input("Input something: ")
```
takes the first character and stores it in the tape.

### move_to
```
move_to(30)
```
sets the pointer to 30.

### add
```
add(10)
```
adds 10 to whatever.

### sub
```
sub(10)
```
add, but in reverse.

### mult
```
mult(3, 4)
```
calculates 3 * 4.

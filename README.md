bf-rs
=====

A brainfuck interpreter and debugger written in rust. See [the Esolang Wiki entry](https://esolangs.org/wiki/Brainfuck) for more information on the language.

Usage
=====

To execute a program, use the `bf-rs` binary:

```
bf-rs [program file]
```

To provide input, you can either pipe data into the interpreter process or pass the `-i` flag

```
echo "input" | bf-rs [program file]
bf-rs -i input.txt [program file]
```

Debugger
========

The `-d` flag enables basic debugging functionality, including stepping through the program and breaking on the special break character `#`.

```
bf-rs -d -i input.txt [program file]
```

Some of the comamnds are:
`re` - run to the end of the program
`rle` - run to the end of the current loop
`rli` - run to the end of the current loop iteration
`r<#>` - run the specified number of steps before breaking. For exmaple, `r30` would run 30 steps.
`s` - step to the next command.

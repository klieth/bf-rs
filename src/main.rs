use clap::{
    App,
    //Arg,
};

#[derive(Debug)]
enum Command {
    Plus,
    Minus,
    Right,
    Left,
    Input,
    Output,
    Loop(Program),
}

struct Program {
    commands: Vec<Command>,
}

impl std::fmt::Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<Program cmds={}>", self.commands.len())
    }
}

impl Program {
    fn parse(input: Vec<u8>) -> Self {
        let mut iter = input.into_iter().peekable();
        Self::parse_internal(&mut iter)
    }

    fn parse_internal<I>(input: &mut std::iter::Peekable<I>) -> Self where I: Iterator<Item = u8> {
        let mut commands = Vec::new();

        while let Some(chr) = input.next() {
            let command = match chr as char {
                '+' => Command::Plus,
                '-' => Command::Minus,
                '>' => Command::Right,
                '<' => Command::Left,
                '[' => Command::Loop(Self::parse_internal(input)),
                ']' => break,
                ',' => Command::Input,
                '.' => Command::Output,
                _ => continue,
            };
            commands.push(command);
        }

        Program {
            commands,
        }
    }

    fn run<I: Iterator<Item = u8>>(&self, tape: &mut Vec<u8>, ptr: &mut usize, stdin: &mut I, stdout: &mut String) {
        for command in self.commands.iter() {
            // ensure ptr is pointing to valid memory
            // TODO: move this and the check inside the Loop into a tape wrapper so that this is
            // automatically checked on a get() or get_mut() call
            if *ptr >= tape.len() {
                tape.resize(*ptr + 1, 0);
            }

            match command {
                Command::Plus => {
                    if let Some(val) = tape.get_mut(*ptr) {
                        *val = val.wrapping_add(1);
                    } else {
                        panic!("memory location invalid: {}", ptr);
                    }
                }
                Command::Minus => {
                    if let Some(val) = tape.get_mut(*ptr) {
                        *val = val.wrapping_sub(1);
                    } else {
                        panic!("memory location invalid: {}", ptr);
                    }
                },
                Command::Right => {
                    *ptr += 1;
                },
                Command::Left => {
                    *ptr -= 1;
                },
                Command::Input => {
                    if let Some(val) = tape.get_mut(*ptr) {
                        if let Some(i) = stdin.next() {
                            *val = i;
                        } else {
                            panic!("EOF on stdin");
                        }
                    } else {
                        panic!("memory location invalid: {}", ptr);
                    }
                },
                Command::Output => {
                    if let Some(val) = tape.get(*ptr) {
                        stdout.push(*val as char);
                    } else {
                        panic!("memory location invalid: {}", ptr);
                    }
                },
                Command::Loop(prog) => {
                    loop {
                        if let Some(val) = tape.get(*ptr) {
                            if *val != 0 {
                                prog.run(tape, ptr, stdin, stdout);
                            } else {
                                break;
                            }
                        } else {
                            panic!("memory location invalid: {}", ptr);
                        }

                        // It's possible for the loop to end on an uninitialized cell, so we have
                        // to check this again before the next loop instance.
                        if *ptr >= tape.len() {
                            tape.resize(*ptr + 1, 0);
                        }
                    }
                },
            }
        }
    }
}

struct Interpreter {
    program: Program,
    tape: Vec<u8>,
    ptr: usize,
}

impl Interpreter {
    fn new(code: Vec<u8>) -> Self {
        let program = Program::parse(code);

        Interpreter {
            program,
            tape: vec![],
            ptr: 0,
        }
    }

    fn run<I: Iterator<Item = u8>>(&mut self, mut stdin: I) -> String {
        let mut output = String::new();

        self.program.run(&mut self.tape, &mut self.ptr, &mut stdin, &mut output);

        output
    }

}

fn main() -> Result<(), String> {
    let _ = App::new("Brainfuck interpreter")
        .version("0.1")
        .get_matches();

    use std::io::Read;
    let mut code = Vec::new();
    let bytes_read = std::io::stdin().read_to_end(&mut code)
        .map_err(|e| format!("{:?}", e))?;
    println!("Read {} bytes of code", bytes_read);

    let mut interpreter = Interpreter::new(code);

    let output = interpreter.run(std::iter::empty());

    println!("{}", output);
    Ok( () )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_works() {
        let input = b"+[-[<<[+[--->]-[<<<]]]>>>-]>-.---.>..>.<<<<-.<+.>>>>>.>.<<.<-.";
        let mut interpreter = Interpreter::new(input.to_vec());
        let output = interpreter.run(std::iter::empty());
        assert_eq!(output, "hello world");
    }
}

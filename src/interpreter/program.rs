use std::{
    fmt,
    io::Write,
};
use super::storage::Storage;
use crate::error::{
    ProgramError,
    ProgramResult,
};

#[derive(Debug)]
pub enum Command {
    Plus,
    Minus,
    Right,
    Left,
    Input,
    Output,
    Loop(Program),
    Debugger,
}

#[derive(Debug)]
pub enum StepResult {
    Continue,
    Debugger,
    LoopIteration,
    LoopEnd,
}

pub struct RunInstance<'a, 'b, 'c, 'd, I: Iterator<Item=u8>, W: Write> {
    stack: std::collections::LinkedList<(&'a Program, usize)>,
    storage: &'b mut Storage,
    stdin: &'c mut I,
    stdout: &'d mut W,
}

impl<'a, 'b, 'c, 'd, I: Iterator<Item=u8>, W: Write> Iterator for RunInstance<'a, 'b, 'c, 'd, I, W> {
    type Item = ProgramResult<StepResult>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((prog, pc)) = self.stack.back_mut() {
            match prog.step(*pc, self.storage, self.stdin, self.stdout) {
                Ok(ProgramStepResult::Continue) => {
                    *pc += 1;
                    Some(Ok(StepResult::Continue))
                }
                Ok(ProgramStepResult::LoopEnd) => {
                    self.stack.pop_back();
                    let result = if *self.storage.get() == 0 {
                        StepResult::LoopEnd
                    } else {
                        StepResult::LoopIteration
                    };
                    Some(Ok(result))
                }
                Ok(ProgramStepResult::LoopEnter(prog)) => {
                    if *self.storage.get() == 0 {
                        *pc += 1;
                    } else {
                        self.stack.push_back((prog, 0));
                    }
                    Some(Ok(StepResult::Continue))
                }
                Ok(ProgramStepResult::Debugger) => {
                    *pc += 1;
                    Some(Ok(StepResult::Debugger))
                }
                Err(e) => Some(Err(e))
            }
        } else {
            None
        }
    }
}

impl<'a, 'b, 'c, 'd, I: Iterator<Item=u8>, W: Write> fmt::Display for RunInstance<'a, 'b, 'c, 'd, I, W> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.storage)
    }
}

enum ProgramStepResult<'a> {
    Continue,
    Debugger,
    LoopEnter(&'a Program),
    LoopEnd,
}

pub struct Program {
    commands: Vec<Command>,
}

impl std::fmt::Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<Program cmds={}>", self.commands.len())
    }
}

impl Program {
    pub fn parse<T: IntoIterator<Item = u8>>(input: T) -> Self {
        let mut iter = input.into_iter();
        Self::parse_internal(&mut iter)
    }

    fn parse_internal<I: Iterator<Item = u8>>(input: &mut I) -> Self {
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
                '#' => Command::Debugger,
                _ => continue,
            };
            commands.push(command);
        }

        Program {
            commands,
        }
    }

    pub fn run<'a, 'b, 'c, 'd, I: Iterator<Item=u8>, W: Write>(&'a self, storage: &'b mut Storage, stdin: &'c mut I, stdout: &'d mut W) -> RunInstance<'a, 'b, 'c, 'd, I, W> {
        let mut stack = std::collections::LinkedList::new();
        stack.push_back((self, 0));
        RunInstance {
            stack,
            storage,
            stdin,
            stdout,
        }
    }

    fn step<I: Iterator<Item=u8>, W: Write>(&self, pc: usize, storage: &mut Storage, stdin: &mut I, stdout: &mut W) -> ProgramResult<ProgramStepResult> {
        if let Some(command) = self.commands.get(pc) {
            match command {
                Command::Input => {
                    if let Some(i) = stdin.next() {
                        storage.set(i);
                        Ok(ProgramStepResult::Continue)
                    } else {
                        Err(ProgramError::Eof)
                    }
                }
                Command::Output => {
                    let val = storage.get();
                    stdout.write(&[*val])
                        .map_err(|_| ProgramError::Eof)
                        .map(|_| ProgramStepResult::Continue)
                },
                Command::Loop(prog) => Ok(ProgramStepResult::LoopEnter(prog)),
                Command::Debugger => Ok(ProgramStepResult::Debugger),
                cmd => {
                    storage.command(cmd);
                    Ok(ProgramStepResult::Continue)
                }
            }
        } else {
            Ok(ProgramStepResult::LoopEnd)
        }
    }
}

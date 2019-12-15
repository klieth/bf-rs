use std::{
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
}

#[derive(Debug)]
pub enum ProgramState {
    Running(usize),
    Stopped,
}

pub struct Program {
    commands: Vec<Command>,
    state: ProgramState,
}

impl std::fmt::Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<Program cmds={} state={:?}>", self.commands.len(), self.state)
    }
}

impl Program {
    pub fn parse<T: IntoIterator<Item = u8>>(input: T) -> Self {
        let mut iter = input.into_iter().peekable();
        Self::parse_internal(&mut iter)
    }

    fn parse_internal<I: Iterator<Item = u8>>(input: &mut std::iter::Peekable<I>) -> Self {
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
            state: ProgramState::Stopped,
        }
    }

    pub fn is_running(&self) -> bool {
        match self.state {
            ProgramState::Running(_) => true,
            ProgramState::Stopped => false,
        }
    }

    pub fn start(&mut self) {
        self.state = ProgramState::Running(0);
    }

    pub fn step<I: Iterator<Item=u8>, W: Write>(&mut self, storage: &mut Storage, stdin: &mut I, stdout: &mut W) -> ProgramResult<bool> {
        if let ProgramState::Stopped = self.state {
            self.start();
        }

        if let ProgramState::Running(ref mut pc) = self.state {
            let command = self.commands.get_mut(*pc)
                .ok_or(ProgramError::PcOutOfBounds)?;

            match command {
                Command::Input => {
                    if let Some(i) = stdin.next() {
                        storage.set(i)?;
                    } else {
                        return Err(ProgramError::Eof)
                    }
                    *pc += 1;
                },
                Command::Output => {
                    storage.get()
                        .ok_or(ProgramError::MemoryError)
                        .and_then(|val| stdout.write(&[*val])
                            .map_err(|_| ProgramError::Eof))?;
                    *pc += 1;
                },
                Command::Loop(prog) => {
                    if !prog.is_running() {
                        if let Some(val) = storage.get() {
                            if *val == 0 {
                                *pc += 1;
                            } else {
                                prog.start();
                            }
                        }
                    }

                    if prog.is_running() {
                        prog.step(storage, stdin, stdout)?;
                    }
                },
                cmd => {
                    storage.command(cmd)?;
                    *pc += 1;
                }
            }

            if *pc == self.commands.len() {
                self.state = ProgramState::Stopped;
            }
        }

        Ok( self.is_running() )
    }
}


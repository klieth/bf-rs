mod program;
mod storage;

use std::io::{
    Read,
    Write,
};

use program::{
    Program,
    StepResult,
};
use storage::Storage;

use crate::{
    error::{
        ProgramError,
        ProgramResult,
    },
    input::InputSource,
};

pub trait Stepper {
    fn new() -> Self;

    fn run_to_end<I: Iterator<Item = u8>, W: Write>(&mut self, program: &mut Program, storage: &mut Storage, stdin: &mut I, stdout: &mut W) -> ProgramResult<()>;
}

pub struct Normal;

impl Stepper for Normal {
    fn new() -> Self {
        Self
    }

    fn run_to_end<I: Iterator<Item = u8>, W: Write>(&mut self, program: &mut Program, storage: &mut Storage, stdin: &mut I, stdout: &mut W) -> ProgramResult<()> {
        // TODO: can this be done with iterator adapters?
        //program.run(storage, stdin, stdout).fold(StepResult::Continue, |_, obj| obj);
        let mut instance = program.run(storage, stdin, stdout);
        while let Some(state) = instance.next() { let _ = state?; }
        Ok( () )
    }
}

enum DebugCommand {
    Run(usize),
    RunToEnd,
    RunToEndWithoutBreak,
    RunLoopIteration,
    RunToAfterLoop,
    Step,
    Break,
}

impl std::str::FromStr for DebugCommand {
    type Err = ProgramError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.trim().chars().peekable();
        chars.next()
            .ok_or(ProgramError::BadCommand)
            .and_then(|c| {
                match c {
                    'r' => {
                        if let Some(p) = chars.peek() {
                            match p {
                                'l' => {
                                    chars.next();
                                    if let Some(ch) = chars.peek() {
                                        match ch {
                                            'e' => Ok(DebugCommand::RunToAfterLoop),
                                            'i' => Ok(DebugCommand::RunLoopIteration),
                                            _ => Err(ProgramError::BadCommand)
                                        }
                                    } else {
                                        Err(ProgramError::BadCommand)
                                    }
                                }
                                'e' => Ok(DebugCommand::RunToEnd),
                                '0'..='9' => {
                                    let rest = chars.collect::<String>();
                                    if rest.len() == 0 {
                                        Ok(DebugCommand::RunToEnd)
                                    } else {
                                        rest.parse()
                                            .map(|amt| DebugCommand::Run(amt))
                                            .map_err(|_| ProgramError::BadCommand)
                                    }
                                }
                                _ => Err(ProgramError::BadCommand)
                            }
                        } else {
                            Err(ProgramError::BadCommand)
                        }
                    },
                    's' => Ok(DebugCommand::Step),
                    _ => Err(ProgramError::BadCommand),
                }
            })
    }
}

pub struct Debug {
    state: DebugCommand,
}

impl Debug {
    fn read_cmd(&self) -> ProgramResult<DebugCommand> {
        println!("Command:");
        let mut raw = String::new();
        std::io::stdin().read_line(&mut raw)
            .map_err(|_e| unimplemented!())?;

        raw.parse()
    }
}

impl Stepper for Debug {
    fn new() -> Self {
        Self {
            state: DebugCommand::Break,
        }
    }

    fn run_to_end<I: Iterator<Item = u8>, W: Write>(&mut self, program: &mut Program, storage: &mut Storage, stdin: &mut I, stdout: &mut W) -> ProgramResult<()> {
        let mut instance = program.run(storage, stdin, stdout);
        while let Some(step_result) = instance.next() {
            let step_result = step_result?;
            println!("{:?}", step_result);
            println!("{}", instance);
            if let StepResult::Debugger = step_result {
                if let DebugCommand::RunToEndWithoutBreak = self.state {
                    continue;
                } else {
                    self.state = DebugCommand::Break;
                }
            }
            match self.state {
                DebugCommand::Run(ref mut amt) => {
                    if *amt == 0 {
                        self.state = DebugCommand::Break;
                    } else {
                        *amt -= 1;
                    }
                }
                DebugCommand::RunLoopIteration => {
                    if let StepResult::LoopIteration = step_result {
                        self.state = DebugCommand::Break;
                    }
                }
                DebugCommand::RunToAfterLoop => {
                    if let StepResult::LoopEnd = step_result {
                        self.state = DebugCommand::Break;
                    }
                }
                DebugCommand::RunToEnd | DebugCommand::RunToEndWithoutBreak => {}
                DebugCommand::Step | DebugCommand::Break => {
                    self.state = self.read_cmd()?;
                }
            }
        }
        Ok( () )
    }
}

pub struct Interpreter {
    program: Program,
    storage: Storage,
}

impl Interpreter {
    pub fn new<T: IntoIterator<Item = u8>>(code: T) -> Self {
        Interpreter {
            program: Program::parse(code),
            storage: Storage::new(),
        }
    }

    pub fn run<S: Stepper>(&mut self, stdin: &mut InputSource) -> ProgramResult<String> {
        let mut output = Vec::new();
        let mut stdin = stdin.bytes().map(|b| b.expect("failed to read from stdin"));

        let mut stepper = S::new();
        stepper.run_to_end(&mut self.program, &mut self.storage, &mut stdin, &mut output)?;

        Ok(String::from_utf8(output).expect("Program wrote invalid utf8 value"))
    }
}

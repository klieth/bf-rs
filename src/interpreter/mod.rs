mod program;
mod storage;

use std::io::Read;

use program::{
    Program,
};
use storage::Storage;

use crate::{
    error::ProgramResult,
    input::InputSource,
};

pub trait Interpreter {
    fn run(&mut self, stdin: &mut InputSource) -> ProgramResult<String>;
}

pub struct DebugInterpreter;

impl DebugInterpreter {
    pub fn new<T: IntoIterator<Item = u8>>(_: T) -> Self {
        DebugInterpreter
    }
}

impl Interpreter for DebugInterpreter {
    fn run(&mut self, stdin: &mut InputSource) -> ProgramResult<String> {
        unimplemented!()
    }
}

pub struct OptimizingInterpreter {
    program: Program,
    storage: Storage,
}

impl OptimizingInterpreter {
    pub fn new<T: IntoIterator<Item = u8>>(code: T) -> Self {
        let program = Program::parse(code);
        OptimizingInterpreter {
            program,
            storage: Storage::new(),
        }
    }
}

impl Interpreter for OptimizingInterpreter {
    fn run(&mut self, stdin: &mut InputSource) -> ProgramResult<String> {
        let mut output = Vec::new();
        let mut stdin = stdin.bytes().map(|b| b.expect("failed to read from stdin"));

        while self.program.step(&mut self.storage, &mut stdin, &mut output)? {}

        Ok(String::from_utf8(output).expect("Program wrote invalid utf8 value"))
    }
}

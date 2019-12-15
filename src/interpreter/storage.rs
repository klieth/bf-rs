use std::fmt::{
    self,
    Display,
    Formatter,
};

use super::program::Command;

use crate::{
    error::{
        ProgramError,
        ProgramResult,
    },
};

const SANITY_LIMIT: usize = 4096;

fn ensure_sized(vec: &mut Vec<u8>, size: usize) {
    if size > 4096 {
        panic!("sanit limit reached: tape grown too big {}/{}", size, SANITY_LIMIT);
    }

    if size >= vec.len() {
        vec.resize(size + 1, 0);
    }
}

pub struct Storage {
    tape: Vec<u8>,
    ptr: usize,
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            tape: Vec::new(),
            ptr: 0,
        }
    }

    pub fn get_mut(&mut self) -> Option<&mut u8> {
        ensure_sized(&mut self.tape, self.ptr);

        self.tape.get_mut(self.ptr)
    }

    pub fn get(&mut self) -> Option<&u8> {
        ensure_sized(&mut self.tape, self.ptr);

        self.tape.get(self.ptr)
    }

    pub fn set(&mut self, val: u8) -> ProgramResult<()> {
        self.get_mut().map(|v| *v = val)
            .ok_or(ProgramError::MemoryError)
    }

    pub fn command(&mut self, command: &Command) -> ProgramResult<()> {
        match command {
            Command::Plus => self.inc_mem(),
            Command::Minus => self.dec_mem(),
            Command::Right => self.inc_ptr(),
            Command::Left => self.dec_ptr(),
            _ => panic!("storage doesn't handle this type of command! {:?}", command),
        }
    }

    pub fn inc_mem(&mut self) -> ProgramResult<()> {
        self.get_mut().map(|v| *v = v.wrapping_add(1))
            .ok_or(ProgramError::MemoryError)
    }

    pub fn dec_mem(&mut self) -> ProgramResult<()> {
        self.get_mut().map(|v| *v = v.wrapping_sub(1))
            .ok_or(ProgramError::MemoryError)
    }

    pub fn inc_ptr(&mut self) -> ProgramResult<()> {
        self.ptr += 1;
        Ok( () )
    }

    pub fn dec_ptr(&mut self) -> ProgramResult<()> {
        self.ptr -= 1;
        Ok( () )
    }
}

impl Display for Storage {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[Storage size={} ptr={}]", self.tape.len(), self.ptr)
    }
}

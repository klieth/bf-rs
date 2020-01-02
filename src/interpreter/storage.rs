use std::fmt::{
    self,
    Display,
    Formatter,
};

use super::program::Command;

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

    pub fn get_mut(&mut self) -> &mut u8 {
        ensure_sized(&mut self.tape, self.ptr);

        self.tape.get_mut(self.ptr).expect("Uncaught memory overflow in get_mut()")
    }

    pub fn get(&mut self) -> &u8 {
        ensure_sized(&mut self.tape, self.ptr);

        self.tape.get(self.ptr).expect("Uncaught memory overflow in get()")
    }

    pub fn set(&mut self, val: u8) {
        let v = self.get_mut();
        *v = val;
    }

    pub fn command(&mut self, command: &Command) {
        match command {
            Command::Plus => self.inc_mem(),
            Command::Minus => self.dec_mem(),
            Command::Right => self.inc_ptr(),
            Command::Left => self.dec_ptr(),
            _ => panic!("storage doesn't handle this type of command! {:?}", command),
        }
    }

    pub fn inc_mem(&mut self) {
        let v = self.get_mut();
        *v = v.wrapping_add(1);
    }

    pub fn dec_mem(&mut self) {
        let v = self.get_mut();
        *v = v.wrapping_sub(1);
    }

    pub fn inc_ptr(&mut self) {
        self.ptr += 1;
    }

    pub fn dec_ptr(&mut self) {
        self.ptr -= 1;
    }
}

const VIEW_OFFSET: usize = 10;

impl Display for Storage {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "[Storage size={} ptr={}]", self.tape.len(), self.ptr)?;

        let min = if self.ptr >= VIEW_OFFSET { self.ptr - VIEW_OFFSET } else { 0 };
        let max = if self.tape.len() > VIEW_OFFSET && self.ptr <= self.tape.len() - VIEW_OFFSET {
            self.ptr + VIEW_OFFSET
        } else {
            self.tape.len()
        };
        let indicator = self.ptr - min;

        for item in min..max {
            let prefix = if indicator == item { '>' } else { ' ' };
            write!(f, "{}{:^4} ", prefix, item)?;
        }
        write!(f, "\n")?;
        for item in self.tape[min..max].iter() {
            write!(f, " {:^4} ", item)?;
        }
        Ok(())
    }
}

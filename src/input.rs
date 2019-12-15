use std::io::Read;

#[cfg(test)]
use crate::{
    error::ProgramError,
};

pub enum InputSource {
    Stdin,
    File(std::fs::File),
    //Str(String),
    #[cfg(test)]
    Empty,
}

impl Read for InputSource {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            InputSource::Stdin => std::io::stdin().read(buf),
            InputSource::File(file) => file.read(buf),
            #[cfg(test)]
            InputSource::Empty => Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, ProgramError::EmptyInputSource))
        }
    }
}

pub fn parse_input_source(value: Option<&str>) -> Result<InputSource, String> {
    match value {
        Some("-") | None => Ok(InputSource::Stdin),
        Some(path) => std::fs::File::open(path).map_err(|e| format!("failed to open file: {}", e)).map(|f| InputSource::File(f))
    }
}


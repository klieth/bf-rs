#[derive(Debug)]
pub enum ProgramError {
    PcOutOfBounds,
    Eof,
    MemoryError,
    #[cfg(test)]
    EmptyInputSource,
}

impl std::fmt::Display for ProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProgramError::PcOutOfBounds => write!(f, "Program Counter out of bounds"),
            ProgramError::Eof => write!(f, "Eof"),
            ProgramError::MemoryError => write!(f, "Memory Error"),
            #[cfg(test)]
            ProgramError::EmptyInputSource => write!(f, "InputSource was Empty"),
        }
    }
}

impl std::error::Error for ProgramError {}

pub type ProgramResult<T> = Result<T, ProgramError>;

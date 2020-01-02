use clap::{
    App,
    Arg,
};
use std::io::Read;

mod error;
mod input;
mod interpreter;

use input::InputSource;
use interpreter::{
    Interpreter,
};

fn main() -> Result<(), String> {
    let matches = App::new("Brainfuck interpreter")
        .version("0.1")
        .arg(Arg::with_name("debug")
            .short("d")
            .long("debug"))
        .arg(Arg::with_name("infile")
            .short("i")
            .long("infile")
            .help("File used as stdin for the running program. Default or '-' uses stdin from the interpreter.")
            .takes_value(true)
            .value_name("INPUT FILE"))
        .arg(Arg::with_name("program")
            .help("File containing the program to run.")
            .takes_value(true)
            .required(true)
            .value_name("PROGRAM FILE"))
        .get_matches();

    let mut code_source = input::parse_input_source(matches.value_of("program"))?;
    let mut stdin_source = input::parse_input_source(matches.value_of("infile"))?;

    if let InputSource::Stdin = code_source {
        return Err("Must specify filename for code. Can't read from stdin".to_string());
    }

    let mut code = Vec::new();
    let bytes_read = code_source.read_to_end(&mut code)
        .map_err(|e| format!("{:?}", e))?;
    println!("Read {} bytes of code", bytes_read);

    let mut interpreter = Interpreter::new(code);

    let output = if matches.is_present("debug") {
        if let InputSource::Stdin = stdin_source {
            return Err("Can't use stdin for program in debug mode".to_string());
        }
        interpreter.run::<interpreter::Debug>(&mut stdin_source).map_err(|e| format!("{}", e))?
    } else {
        interpreter.run::<interpreter::Normal>(&mut stdin_source).map_err(|e| format!("{}", e))?
    };
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
        let output = interpreter.run::<interpreter::Normal>(&mut InputSource::Empty).expect("Failed to run");
        assert_eq!(output, "hello world");
    }
}

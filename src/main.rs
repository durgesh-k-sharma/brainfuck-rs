use brainfuck::{Program, TerminalIo, Vm};
use std::{
    env, fs,
    io::{self, Read},
};

fn main() {
    if let Err(error) = run_cli() {
        eprintln!("brainfuck: {error}");
        std::process::exit(1);
    }
}

fn run_cli() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let source = if let Some(path) = args.next() {
        if args.next().is_some() {
            return Err("usage: brainfuck [FILE]".into());
        }
        fs::read_to_string(path)?
    } else {
        let mut source = String::new();
        io::stdin().read_to_string(&mut source)?;
        source
    };
    let program = Program::parse(&source)?;
    let mut vm = Vm::new(program, TerminalIo::new());
    vm.run()?;
    Ok(())
}

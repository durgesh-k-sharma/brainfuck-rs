use brainfuck::{BrainfuckIo, Error, MemoryIo, Program, RunOutcome, Vm};
use std::io;

fn execute(source: &str, input: &[u8]) -> (Vm<MemoryIo>, Vec<u8>) {
    let program = Program::parse(source).unwrap();
    let mut vm = Vm::new(program, MemoryIo::new(input.to_vec()));
    vm.run().unwrap();
    let output = vm.io().output().to_vec();
    (vm, output)
}

#[test]
fn instructions_pointer_movement_wrapping_and_ignored_characters() {
    let (vm, output) = execute("comment ++++++++[>++++++++<-]>+. ignored", b"");
    assert_eq!(output, b"A");
    assert_eq!(vm.snapshot().data_pointer, 1);
    assert_eq!(vm.snapshot().tape, vec![0, 65]);
}

#[test]
fn all_eight_instructions_and_both_wrapping_directions() {
    // Comma reads 0xff; the pointer moves right and back, while cell
    // arithmetic wraps in both directions and dot writes the current byte.
    let (_, output) = execute(",>+.-.<.", &[u8::MAX]);
    assert_eq!(output, [1, 0, u8::MAX]);
}

#[test]
fn decrement_wraps_zero_to_255() {
    let (vm, output) = execute("-.", b"");
    assert_eq!(output, [u8::MAX]);
    assert_eq!(vm.snapshot().tape, [u8::MAX]);
}

#[test]
fn nested_loops_and_eof() {
    let (_, output) = execute("++[>++[>+<-]<-]>>.,.,.", b"Z");
    assert_eq!(output, [4, b'Z', 0]);
}

#[test]
fn unmatched_brackets_include_locations() {
    assert!(
        matches!(Program::parse("+\n["), Err(Error::UnmatchedOpen { location } ) if location.line == 2 && location.column == 1)
    );
    assert!(
        matches!(Program::parse("\n]"), Err(Error::UnmatchedClose { location } ) if location.line == 2 && location.column == 1)
    );
}

#[test]
fn parser_discards_non_commands_and_keeps_original_locations() {
    let program = Program::parse("é\n +x").unwrap();
    assert_eq!(program.len(), 1);
    let operation = program.operations()[0];
    assert_eq!(operation.location.line, 2);
    assert_eq!(operation.location.column, 2);
    assert_eq!(operation.location.offset, 4);
}

#[test]
fn moving_left_from_zero_is_an_error() {
    let mut vm = Vm::new(Program::parse("<").unwrap(), MemoryIo::default());
    assert!(
        matches!(vm.step(), Err(Error::PointerUnderflow { location }) if location.line == 1 && location.column == 1)
    );
}

struct BrokenIo;

impl BrainfuckIo for BrokenIo {
    fn read_byte(&mut self) -> io::Result<Option<u8>> {
        Err(io::Error::new(io::ErrorKind::Other, "read failed"))
    }
    fn write_byte(&mut self, _: u8) -> io::Result<()> {
        Err(io::Error::new(io::ErrorKind::Other, "write failed"))
    }
}

#[test]
fn io_failures_are_structured_and_located() {
    let mut input_vm = Vm::new(Program::parse("\n,").unwrap(), BrokenIo);
    assert!(
        matches!(input_vm.step(), Err(Error::Io { location: Some(location), .. }) if location.line == 2)
    );
    let mut output_vm = Vm::new(Program::parse(".").unwrap(), BrokenIo);
    assert!(
        matches!(output_vm.step(), Err(Error::Io { location: Some(location), .. }) if location.column == 1)
    );
}

#[test]
fn run_until_limits_additional_instructions() {
    let mut vm = Vm::new(Program::parse("+++.").unwrap(), MemoryIo::default());
    assert_eq!(vm.run_until(2).unwrap(), RunOutcome::InstructionLimit);
    assert_eq!(vm.run_until(2).unwrap(), RunOutcome::Halted);
    assert_eq!(vm.io().output(), &[3]);
}

#[test]
fn hello_world_program() {
    let source = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
    let (_, output) = execute(source, b"");
    assert_eq!(output, b"Hello World!\n");
}

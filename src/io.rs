use std::io::{self, Read, Write};

/// Byte-oriented I/O used by the VM. A `None` input byte represents EOF.
pub trait BrainfuckIo {
    fn read_byte(&mut self) -> io::Result<Option<u8>>;
    fn write_byte(&mut self, byte: u8) -> io::Result<()>;
}

/// In-memory I/O for embedding and tests.
#[derive(Debug, Default)]
pub struct MemoryIo {
    input: Vec<u8>,
    input_position: usize,
    output: Vec<u8>,
}

impl MemoryIo {
    pub fn new(input: impl Into<Vec<u8>>) -> Self {
        Self {
            input: input.into(),
            input_position: 0,
            output: Vec::new(),
        }
    }

    pub fn output(&self) -> &[u8] {
        &self.output
    }
    pub fn into_output(self) -> Vec<u8> {
        self.output
    }
}

impl BrainfuckIo for MemoryIo {
    fn read_byte(&mut self) -> io::Result<Option<u8>> {
        let byte = self.input.get(self.input_position).copied();
        if byte.is_some() {
            self.input_position += 1;
        }
        Ok(byte)
    }
    fn write_byte(&mut self, byte: u8) -> io::Result<()> {
        self.output.push(byte);
        Ok(())
    }
}

/// Terminal byte I/O. Output is flushed after each `.` instruction.
pub struct TerminalIo<R = io::Stdin, W = io::Stdout> {
    input: R,
    output: W,
}

impl TerminalIo<io::Stdin, io::Stdout> {
    pub fn new() -> Self {
        Self {
            input: io::stdin(),
            output: io::stdout(),
        }
    }
}

impl<R, W> TerminalIo<R, W> {
    pub fn with_streams(input: R, output: W) -> Self {
        Self { input, output }
    }
}

impl<R: Read, W: Write> BrainfuckIo for TerminalIo<R, W> {
    fn read_byte(&mut self) -> io::Result<Option<u8>> {
        let mut byte = [0];
        match self.input.read(&mut byte)? {
            0 => Ok(None),
            _ => Ok(Some(byte[0])),
        }
    }
    fn write_byte(&mut self, byte: u8) -> io::Result<()> {
        self.output.write_all(&[byte])?;
        self.output.flush()
    }
}

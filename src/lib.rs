//! A faithful, small Brainfuck implementation.
//!
//! The parser removes non-instructions and pairs brackets before execution.
//! The VM uses an unsigned 8-bit, wrapping tape and grows it to the right.

pub mod error;
pub mod io;
pub mod parser;
pub mod vm;

pub use error::{Error, Result, SourceLocation};
pub use io::{BrainfuckIo, MemoryIo, TerminalIo};
pub use parser::{Instruction, Program};
pub use vm::{RunOutcome, Snapshot, Vm};

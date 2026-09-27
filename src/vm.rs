use crate::{BrainfuckIo, Error, Program, Result};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snapshot {
    pub instruction_pointer: usize,
    pub data_pointer: usize,
    pub tape: Vec<u8>,
    pub halted: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunOutcome {
    Halted,
    InstructionLimit,
}

pub struct Vm<I> {
    program: Program,
    io: I,
    tape: Vec<u8>,
    data_pointer: usize,
    instruction_pointer: usize,
    executed: usize,
}

impl<I: BrainfuckIo> Vm<I> {
    pub fn new(program: Program, io: I) -> Self {
        Self {
            program,
            io,
            tape: vec![0],
            data_pointer: 0,
            instruction_pointer: 0,
            executed: 0,
        }
    }

    /// Execute one instruction; return `false` when the program has halted.
    pub fn step(&mut self) -> Result<bool> {
        let Some(operation) = self
            .program
            .operations()
            .get(self.instruction_pointer)
            .copied()
        else {
            return Ok(false);
        };
        use crate::Instruction::*;
        match operation.instruction {
            Right => {
                self.data_pointer += 1;
                if self.data_pointer == self.tape.len() {
                    self.tape.push(0);
                }
                self.instruction_pointer += 1;
            }
            Left => {
                if self.data_pointer == 0 {
                    return Err(Error::PointerUnderflow {
                        location: operation.location,
                    });
                }
                self.data_pointer -= 1;
                self.instruction_pointer += 1;
            }
            Increment => {
                self.tape[self.data_pointer] = self.tape[self.data_pointer].wrapping_add(1);
                self.instruction_pointer += 1;
            }
            Decrement => {
                self.tape[self.data_pointer] = self.tape[self.data_pointer].wrapping_sub(1);
                self.instruction_pointer += 1;
            }
            Output => {
                self.io
                    .write_byte(self.tape[self.data_pointer])
                    .map_err(|source| Error::Io {
                        location: Some(operation.location),
                        source,
                    })?;
                self.instruction_pointer += 1;
            }
            Input => {
                self.tape[self.data_pointer] = self
                    .io
                    .read_byte()
                    .map_err(|source| Error::Io {
                        location: Some(operation.location),
                        source,
                    })?
                    .unwrap_or(0);
                self.instruction_pointer += 1;
            }
            LoopStart => {
                if self.tape[self.data_pointer] == 0 {
                    self.instruction_pointer =
                        operation.jump.expect("parsed loop has a target") + 1;
                } else {
                    self.instruction_pointer += 1;
                }
            }
            LoopEnd => {
                if self.tape[self.data_pointer] != 0 {
                    self.instruction_pointer =
                        operation.jump.expect("parsed loop has a target") + 1;
                } else {
                    self.instruction_pointer += 1;
                }
            }
        }
        self.executed += 1;
        Ok(true)
    }

    pub fn run(&mut self) -> Result<()> {
        while self.step()? {}
        Ok(())
    }

    /// Run at most `limit` further instructions and report whether the VM halted.
    pub fn run_until(&mut self, limit: usize) -> Result<RunOutcome> {
        for _ in 0..limit {
            if !self.step()? {
                return Ok(RunOutcome::Halted);
            }
        }
        Ok(if self.instruction_pointer >= self.program.len() {
            RunOutcome::Halted
        } else {
            RunOutcome::InstructionLimit
        })
    }

    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            instruction_pointer: self.instruction_pointer,
            data_pointer: self.data_pointer,
            tape: self.tape.clone(),
            halted: self.instruction_pointer >= self.program.len(),
        }
    }

    pub fn io(&self) -> &I {
        &self.io
    }

    pub fn io_mut(&mut self) -> &mut I {
        &mut self.io
    }

    pub fn executed_instructions(&self) -> usize {
        self.executed
    }
}

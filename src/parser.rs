use crate::{Error, Result, SourceLocation};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    Right,
    Left,
    Increment,
    Decrement,
    Output,
    Input,
    LoopStart,
    LoopEnd,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Operation {
    pub instruction: Instruction,
    pub location: SourceLocation,
    /// Matching bracket index for loop instructions.
    pub jump: Option<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    operations: Vec<Operation>,
}

impl Program {
    pub fn parse(source: &str) -> Result<Self> {
        let mut operations = Vec::new();
        let mut stack: Vec<(usize, SourceLocation)> = Vec::new();
        let (mut line, mut column) = (1, 1);
        for (offset, ch) in source.char_indices() {
            let location = SourceLocation {
                line,
                column,
                offset,
            };
            let instruction = match ch {
                '>' => Some(Instruction::Right),
                '<' => Some(Instruction::Left),
                '+' => Some(Instruction::Increment),
                '-' => Some(Instruction::Decrement),
                '.' => Some(Instruction::Output),
                ',' => Some(Instruction::Input),
                '[' => Some(Instruction::LoopStart),
                ']' => Some(Instruction::LoopEnd),
                _ => None,
            };
            if let Some(instruction) = instruction {
                let index = operations.len();
                operations.push(Operation {
                    instruction,
                    location,
                    jump: None,
                });
                match instruction {
                    Instruction::LoopStart => stack.push((index, location)),
                    Instruction::LoopEnd => {
                        let Some((start, _)) = stack.pop() else {
                            return Err(Error::UnmatchedClose { location });
                        };
                        operations[start].jump = Some(index);
                        operations[index].jump = Some(start);
                    }
                    _ => {}
                }
            }
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        if let Some((_, location)) = stack.pop() {
            return Err(Error::UnmatchedOpen { location });
        }
        Ok(Self { operations })
    }

    pub fn operations(&self) -> &[Operation] {
        &self.operations
    }

    pub fn len(&self) -> usize {
        self.operations.len()
    }

    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }
}

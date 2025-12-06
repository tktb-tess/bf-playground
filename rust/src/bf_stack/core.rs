use anyhow::{Result, anyhow, bail};

#[derive(Debug, Default, Clone)]
pub struct BFMemory {
    stack: Vec<u8>,
    index: usize,
}

#[derive(Debug, Clone)]
pub enum BFCommand {
    Next,
    Prev,
    Increment,
    Decrement,
    Read,
    Write,
    LoopStart,
    LoopEnd,
}

impl BFCommand {
    fn from(value: u8) -> Option<Self> {
        use BFCommand::*;
        match value {
            0x3e => Some(Next),
            0x3c => Some(Prev),
            0x2b => Some(Increment),
            0x2d => Some(Decrement),
            0x2e => Some(Read),
            0x2c => Some(Write),
            0x5b => Some(LoopStart),
            0x5d => Some(LoopEnd),
            _ => None,
        }
    }
}

impl BFMemory {
    const DEFAULT_LEN: usize = 0x8000;
    pub fn new() -> Self {
        let stack = vec![0; Self::DEFAULT_LEN];
        Self { stack, index: 0 }
    }

    pub fn get_value(&self) -> Result<u8> {
        self.stack
            .get(self.index)
            .cloned()
            .ok_or_else(|| anyhow!("out of range"))
    }

    fn realloc(&mut self, new_len: usize) -> Result<()> {
        let len = self.stack.len();

        if len > new_len {
            bail!("new length is shorter than current length");
        }

        self.stack.resize(new_len, 0);

        Ok(())
    }
}

use super::error::BFRuntimeError;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

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
    fn new(value: u8) -> Option<Self> {
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

#[derive(Debug)]
pub struct BFCode {
    commands: Box<[BFCommand]>,
    pub maps: Box<[[usize; 2]]>,
}

impl Deref for BFCode {
    type Target = Box<[BFCommand]>;
    fn deref(&self) -> &Self::Target {
        &self.commands
    }
}

impl DerefMut for BFCode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.commands
    }
}

impl FromStr for BFCode {
    type Err = BFRuntimeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = Vec::with_capacity(s.len());

        for c in s.as_bytes() {
            if let Some(c) = BFCommand::new(*c) {
                v.push(c);
            }
        }

        let v = v.into_boxed_slice();
        let maps = detect_loop(&v)?;

        Ok(BFCode { commands: v, maps })
    }
}

fn detect_loop(v: &[BFCommand]) -> Result<Box<[[usize; 2]]>, BFRuntimeError> {
    use BFCommand::*;
    let mut maps = vec![];
    let mut idxs = vec![];
    for i in 0..v.len() {
        let com = v.get(i).ok_or_else(|| BFRuntimeError::OutOfRange {
            index: i,
            len: v.len(),
        })?;

        if let LoopStart = com {
            idxs.push(i);
        } else if let LoopEnd = com {
            let start = idxs.pop().ok_or_else(|| {
                BFRuntimeError::FailedToParseCode("no corresponding LoopStart".into())
            })?;
            maps.push([start, i]);
        }
    }

    if idxs.len() > 0 {
        Err(BFRuntimeError::FailedToParseCode(
            "no corresponding LoopEnd".into(),
        ))?;
    }

    Ok(maps.into())
}

use super::error::BFRuntimeError;
use std::{
    ops::{Deref, DerefMut},
    result,
    str::FromStr,
};

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

impl TryFrom<u8> for BFCommand {
    type Error = ();

    fn try_from(value: u8) -> result::Result<Self, Self::Error> {
        use BFCommand::*;
        match value {
            0x3e => Ok(Next),
            0x3c => Ok(Prev),
            0x2b => Ok(Increment),
            0x2d => Ok(Decrement),
            0x2e => Ok(Read),
            0x2c => Ok(Write),
            0x5b => Ok(LoopStart),
            0x5d => Ok(LoopEnd),
            _ => Err(()),
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
            if let Ok(c) = BFCommand::try_from(*c) {
                v.push(c);
            }
        }

        v.shrink_to_fit();
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
        let com = v.get(i).ok_or_else(|| {
            BFRuntimeError::new(&format!("unexpected error: index was out of range: {}", i))
        })?;

        if let LoopStart = com {
            idxs.push(i);
        } else if let LoopEnd = com {
            let start = idxs
                .pop()
                .ok_or_else(|| BFRuntimeError::new("invalid code: no corresponding LoopStart"))?;
            maps.push([start, i]);
        }
    }

    if idxs.len() > 0 {
        Err(BFRuntimeError::new(
            "invalid code: no corresponding LoopEnd",
        ))?;
    }

    Ok(maps.into())
}

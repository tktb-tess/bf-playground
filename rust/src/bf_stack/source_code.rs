use std::{
    ops::{Deref, DerefMut},
    result,
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

// impl BFCommand {
//     pub fn as_code_point(&self) -> u8 {
//         use BFCommand::*;
//         match self {
//             Next => 0x3e,
//             Prev => 0x3c,
//             Increment => 0x2b,
//             Decrement => 0x2d,
//             Read => 0x2e,
//             Write => 0x2c,
//             LoopStart => 0x5b,
//             LoopEnd => 0x5d,
//         }
//     }
// }

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

impl From<&str> for BFCode {
    fn from(value: &str) -> Self {
        let mut v = Vec::with_capacity(value.len());

        for c in value.as_bytes() {
            if let Ok(c) = BFCommand::try_from(*c) {
                v.push(c);
            }
        }

        v.shrink_to_fit();
        let v = v.into_boxed_slice();
        let maps = detect_loop(&v);

        BFCode { commands: v, maps }
    }
}

fn detect_loop(v: &[BFCommand]) -> Box<[[usize; 2]]> {
    use BFCommand::*;
    let mut maps = vec![];
    let mut idxs = vec![];
    for i in 0..v.len() {
        let com = v
            .get(i)
            .unwrap_or_else(|| unreachable!("unexpected error: index was out of range: {}", i));

        if let LoopStart = com {
            idxs.push(i);
        } else if let LoopEnd = com {
            let start = idxs
                .pop()
                .expect("invalid code: no corresponding LoopStart");
            maps.push([start, i]);
        }
    }

    if idxs.len() > 0 {
        panic!("invalid code: no corresponding LoopEnd");
    }

    maps.into()
}

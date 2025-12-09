mod bf_stack;
use bf_stack::BFCommand::*;
use bf_stack::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct BFExecOptions {
    #[tsify(optional)]
    pub input: Option<String>,
    #[tsify(optional)]
    pub init_buff_len: Option<usize>,
}

#[wasm_bindgen]
pub fn exec(code: &str, options: BFExecOptions) -> String {
    let code: BFCode = code.into();

    let mut mem = match options.init_buff_len {
        Some(len) => BFMemory::with_len(len),
        None => BFMemory::new(),
    };

    let input = options.input.unwrap_or("".into());
    let mut input: VecDeque<_> = input.into_bytes().into_iter().collect();
    let mut output = vec![];
    let mut i: usize = 0;

    while i < code.len() {
        let c = code
            .get(i)
            .unwrap_or_else(|| panic!("Unexpected error: index was out of range {} {:?}", i, code));
        match c {
            Next => {
                mem.move_next();
            }
            Prev => {
                mem.move_prev();
            }
            Increment => {
                mem.increment();
            }
            Decrement => {
                mem.decrement();
            }
            Read => {
                let read = *mem.get_value();
                output.push(read);
            }
            Write => {
                let write = input.pop_front().expect("No input");
                *mem.get_value_mut() = write;
            }
            LoopStart => {
                let mut iter = code.maps.iter();
                if *mem.get_value() == 0 {
                    let e = iter
                        .find(|[s, _]| i == *s)
                        .expect("Unexpected error: loop not found")[1];
                    i = e + 1;
                }
            }
            LoopEnd => {
                let mut iter = code.maps.iter();
                if *mem.get_value() > 0 {
                    let s = iter
                        .find(|[_, e]| i == *e)
                        .expect("Unexpected error: loop not found")[0];
                    i = s;
                }
            }
        }
        i += 1;
    }
    let output: String = output.try_into().unwrap_or_else(|e| panic!("{}", e));
    output
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let str = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        let options = BFExecOptions { input: None, init_buff_len: None };
        let result = exec(str, options);
        assert_eq!(result, "Hello World!\n");
    }
}
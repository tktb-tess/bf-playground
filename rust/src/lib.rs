mod bf_stack;
use bf_stack::BFCommand::*;
use bf_stack::*;
use std::collections::VecDeque;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn exec(code: &str, options: BFOptions) -> Result<String, JsError> {
    let code: BFCode = code.parse()?;

    let mut mem = match options.init_buff_len {
        Some(len) => BFMemory::with_len(len),
        None => BFMemory::new(),
    };

    let mut input: VecDeque<_> = options
        .input
        .unwrap_or("".into())
        .into_bytes()
        .into_iter()
        .collect();

    let mut output = vec![];
    let mut i: usize = 0;
    let mut count: u32 = 0;

    while i < code.len() {
        if count > CYCLE_LIMIT {
            Err(BFRuntimeError::new("Exceeded limit of loop"))?;
        }

        let c = code.get(i).ok_or_else(|| {
            BFRuntimeError::new(&format!(
                "Unexpected error: index was out of range {} {:?}",
                i, code
            ))
        })?;

        match c {
            Next => {
                mem.move_next()?;
            }
            Prev => {
                mem.move_prev();
            }
            Increment => {
                mem.increment()?;
            }
            Decrement => {
                mem.decrement()?;
            }
            Read => {
                let read = *mem.get_value()?;
                output.push(read);
            }
            Write => {
                let write = input
                    .pop_front()
                    .ok_or_else(|| BFRuntimeError::new("Ran out of input"))?;
                *mem.get_value_mut()? = write;
            }
            LoopStart => {
                let mut iter = code.maps.iter();
                if *mem.get_value()? == 0 {
                    let e = iter.find(|[s, _]| i == *s).ok_or_else(|| {
                        BFRuntimeError::new("Invalid source code: loop not found")
                    })?[1];
                    i = e;
                }
            }
            LoopEnd => {
                let mut iter = code.maps.iter();
                if *mem.get_value()? > 0 {
                    let s = iter.find(|[_, e]| i == *e).ok_or_else(|| {
                        BFRuntimeError::new("Invalid source code: loop not found")
                    })?[0];
                    i = s;
                }
            }
        }

        i += 1;
        count += 1;
    }

    let output = String::try_from(output).map_err(|e| BFRuntimeError::new(&e.to_string()))?;

    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let str = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.";
        let options = BFOptions {
            input: None,
            init_buff_len: None,
        };
        let result = exec(str, options).unwrap();
        assert_eq!(&result, "Hello World!");
    }
}

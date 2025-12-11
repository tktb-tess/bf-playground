use crate::bf_stack::BFRuntimeError;

type BFResult<T> = Result<T, BFRuntimeError>;

#[derive(Debug, Clone)]
pub struct BFMemory {
    stack: Vec<u8>,
    index: usize,
}

impl BFMemory {
    const DEFAULT_LEN: usize = 0x8000;
    const LIMIT: usize = 1 << 28;

    pub fn with_len(len: usize) -> Self {
        let stack = vec![0; len];
        Self { stack, index: 0 }
    }

    pub fn new() -> Self {
        Self::with_len(Self::DEFAULT_LEN)
    }

    pub fn get_value(&self) -> BFResult<&u8> {
        let len = self.stack.len();
        self.stack.get(self.index).ok_or_else(|| {
            BFRuntimeError::new(&format!(
                "out of range\nindex: {}\nlen: {}",
                self.index, len
            ))
        })
    }

    pub fn get_value_mut(&mut self) -> BFResult<&mut u8> {
        let len = self.stack.len();
        self.stack.get_mut(self.index).ok_or_else(|| {
            BFRuntimeError::new(&format!(
                "out of range\nindex: {}\nlen: {}",
                self.index, len
            ))
        })
    }

    fn realloc(&mut self, new_len: usize) -> BFResult<()> {
        let len = self.stack.len();

        if len > new_len {
            Err(BFRuntimeError::new(
                "New length is shorter than current length",
            ))?;
        }

        if new_len > Self::LIMIT {
            Err(BFRuntimeError::new("exceeded limit of memory size"))?;
        }

        self.stack.resize(new_len, 0);
        Ok(())
    }

    pub fn move_next(&mut self) -> BFResult<()> {
        let len = self.stack.len();
        if self.index == (len - 1) {
            self.realloc(len * 2)?;
        }

        self.index += 1;
        Ok(())
    }

    pub fn move_prev(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }

    pub fn increment(&mut self) -> BFResult<()> {
        let n = *self.get_value_mut()?;
        *self.get_value_mut()? = n.wrapping_add(1);
        Ok(())
    }

    pub fn decrement(&mut self) -> BFResult<()> {
        let n = *self.get_value_mut()?;
        *self.get_value_mut()? = n.wrapping_sub(1);
        Ok(())
    }
}

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

    pub fn get_value(&self) -> &u8 {
        self.stack.get(self.index).unwrap_or_else(|| {
            panic!(
                "out of range\nindex: {}\nlen: {}",
                self.index,
                self.stack.len()
            )
        })
    }

    pub fn get_value_mut(&mut self) -> &mut u8 {
        let len = self.stack.len();
        self.stack
            .get_mut(self.index)
            .unwrap_or_else(|| panic!("out of range\nindex: {}\nlen: {}", self.index, len))
    }

    fn realloc(&mut self, new_len: usize) {
        let len = self.stack.len();

        if len > new_len {
            panic!("new length is shorter than current length");
        }

        if new_len > Self::LIMIT {
            panic!("exceeded limit of memory size");
        }

        self.stack.resize(new_len, 0);
    }

    pub fn move_next(&mut self) {
        let len = self.stack.len();
        if self.index == (len - 1) {
            self.realloc(len * 2);
        }

        self.index += 1;
    }

    pub fn move_prev(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }

    pub fn increment(&mut self) {
        let n = *self.get_value_mut();
        *self.get_value_mut() = n.wrapping_add(1);
    }

    pub fn decrement(&mut self) {
        let n = *self.get_value_mut();
        *self.get_value_mut() = n.wrapping_sub(1);
    }
}

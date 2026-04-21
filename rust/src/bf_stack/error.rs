use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum BFRuntimeError {
    #[error("out of range\nindex: {index}, len: {len}")]
    OutOfRange { index: usize, len: usize },
    #[error("tried to allocate shorter memory than current")]
    AllocShorterMemory,
    #[error("exceeded limit of memory size: {0}")]
    ExceededLimitMemSize(usize),
    #[error("exceeded limit of loop: {0}")]
    ExceededLimitLoopSize(usize),
    #[error("fail to parse code: {0}")]
    FailedToParseCode(String),
    #[error("ran out of input")]
    RanOutOfInput,
}

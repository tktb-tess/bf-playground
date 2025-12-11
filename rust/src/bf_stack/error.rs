use serde::{Deserialize, Serialize};
use std::{error, fmt};
use tsify::Tsify;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct BFRuntimeError {
    pub message: String,
}

impl BFRuntimeError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for BFRuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}

impl error::Error for BFRuntimeError {}



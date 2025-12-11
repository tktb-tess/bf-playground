use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Clone, Default, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct BFOptions {
    #[tsify(optional)]
    pub input: Option<String>,
    #[tsify(optional)]
    pub init_buff_len: Option<usize>,
}

pub const CYCLE_LIMIT: u32 = 1 << 28;

pub mod cfmm;
pub mod cost_function;
pub mod utils;

pub mod dto;
pub mod entity;

use amplify::{From, Wrapper};
use noisy_float::types::R64;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, Wrapper)]
pub struct AssetId([u8; 32]);

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AssetInfo {
    id: AssetId,
    amount: R64,
    ticker: String,
}

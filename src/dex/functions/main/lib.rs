pub(crate) mod lib;
pub mod amm_manager;
pub mod utils;


pub mod functions;

use amplify::{From, Wrapper};
use noisy_float::types::R64;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, Wrapper)]
pub struct AssetId([u8; 32]);

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AssetInfo {
    id: AssetId,
    amount: Number,
    genesis: String,
    schema: String,
}

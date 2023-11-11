pub mod lib;
pub mod amm_mangaer;
pub mod utils;


pub mod funtcions;

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

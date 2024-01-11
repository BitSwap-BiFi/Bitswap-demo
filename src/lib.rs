// Bitswap Core

#[macro_use]
extern crate rgb;
#[macro_use]
extern crate bifrost;
#[macro_use]
extern crate error;
#[macro_use]
extern crate lightning;
#[macro_use]
extern crate struct;
#[macro_use]
extern crate proxy;
#[macro_use]
extern crate bitcoin;
#[macro_use]
extern crate util;
#[macro_use]
extern crate testnet;
#[macro_use]
extern crate validators;
#[macro_use]
extern crate web;
#[macro_use]
extern crate constants;

mod rgb;
mod proxy;
mod bifrost;
mod testnet;
mod validators;
mod util;
mod constant;
mod struct;
mod web;

pub use rgb::rgb_core;


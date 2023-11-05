use std::collections::BTreeMap;

use bitcoin::secp256k1::schnorr::Signature;
use bitcoin::secp256k1::XOnlyPublicKey;
use lnpbp::chain::Chain;
use psbt::Psbt;

use crate::bifrost::{ChannelId, ChannelProposal, ProtocolName};
use crate::bolt;

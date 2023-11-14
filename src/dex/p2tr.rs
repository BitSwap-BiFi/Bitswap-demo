
use std::borrow::{Borrow, BorrowMut};
use std::cmp::Ordering;
use std::fmt::{self, Debug, Display, Formatter};
use std::io::{Read, Write};
use std::ops::{Deref, Not};
use std::str::FromStr;

use rgb_node::{Daemon, Mod, Opts, Service};
use bitswap_core::{Aluvm,PayJoin,Swap};
use rust_dlc::{Message};
use rust_payjoin::{Sender, Receiver, Input, Output};
use amplify::Wrapper;
use bitcoin::hashes::Hash;
use bitcoin::psbt::TapTree;
use bitcoin::util::taproot::{LeafVersion, TapBranchHash, TapLeafHash, TaprootBuilder};
use bitcoin::Script;
use strict_encoding::{StrictDecode, StrictEncode}


use crate::types::IntoNodeHash;
use crate::{LeafScript, TapNodeHash, TapScript};

impl TaprootScriptTree {

fn taproot = {
  let leafversion =
fn bitswap
fn bitcoin


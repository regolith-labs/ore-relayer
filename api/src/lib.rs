pub mod consts;
pub mod error;
pub mod instruction;
pub mod loaders;
pub mod state;

pub(crate) use ore_utils as utils;

use solana_program::declare_id;

// TODO
declare_id!("DhG9f9AqK6uRaAVvGd5nBuLyjMr9U2oLSPEBe3Jb3SL4");

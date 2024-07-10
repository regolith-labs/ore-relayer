pub mod consts;
pub mod error;
pub mod instruction;
pub mod loaders;
pub mod state;

pub(crate) use ore_utils as utils;

use solana_program::declare_id;

// TODO
// declare_id!("stakeHF5r6S7HyD9SppBfVMXMavDkJsxwGesEvxZr2A");
declare_id!("3Esu6Fg26wmJeeswBUCUhXfV2NLJf3vwMt2S6mo8rRRH");

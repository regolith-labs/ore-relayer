pub mod consts;
pub mod error;
pub mod instruction;
pub mod loaders;
pub mod state;

pub(crate) use ore_utils as utils;

use solana_program::declare_id;

// TODO
// declare_id!("stakeHF5r6S7HyD9SppBfVMXMavDkJsxwGesEvxZr2A");
declare_id!("25C1Mc8fuirWVR6PiNUFnQavLG7hetCr5iU3AY94pqbt");

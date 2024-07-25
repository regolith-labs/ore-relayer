pub mod consts;
pub mod error;
pub mod instruction;
pub mod loaders;
pub mod state;

pub(crate) use ore_utils as utils;

use solana_program::declare_id;

// TODO
declare_id!("DMeEBqSV9ccdmus3NyGYYf5W4N4jJaMWXMoRrHSAKHwE");

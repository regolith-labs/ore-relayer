pub mod consts;
pub mod error;
pub mod instruction;
pub mod loaders;
pub mod state;

pub(crate) use ore_utils as utils;

use solana_program::declare_id;

// TODO
declare_id!("5k7MGcRF3HNhC6HCGUE8aVnMGj94z4p8CV2fY5hpWyoH");

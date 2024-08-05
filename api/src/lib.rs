pub mod consts;
pub mod error;
pub mod instruction;
pub mod loaders;
pub mod state;

pub(crate) use ore_utils as utils;

use solana_program::declare_id;

declare_id!("DdWBvLQxbyPcry5G8T3arkgcsYYRzYfdZiCXTevxT6vP");

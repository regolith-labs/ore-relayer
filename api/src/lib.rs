pub mod consts;
pub mod error;
pub mod instruction;
pub mod loaders;
pub mod state;

pub(crate) use ore_utils as utils;

use solana_program::declare_id;

// TODO
declare_id!("AyYunJ29iysRy7Guw2vZ6Be7nS3QzuuTUuJtZAYSt1BZ");

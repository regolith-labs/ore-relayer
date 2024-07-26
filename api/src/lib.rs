pub mod consts;
pub mod error;
pub mod instruction;
pub mod loaders;
pub mod state;

pub(crate) use ore_utils as utils;

use solana_program::declare_id;

// TODO
declare_id!("CqSALibGBk2FuCbHbJHJhTGV1WktJ6uR55Bga2eouQ9m");

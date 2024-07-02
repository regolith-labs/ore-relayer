mod escrow;
mod relayer;

pub use escrow::*;
pub use relayer::*;

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum AccountDiscriminator {
    Escrow = 100,
    Relayer = 101,
}

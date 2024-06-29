mod delegate;
mod stake;

pub use delegate::*;
pub use stake::*;

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum AccountDiscriminator {
    Delegate = 100,
    Stake = 101,
}

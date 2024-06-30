mod delegate;
mod pool;

pub use delegate::*;
pub use pool::*;

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum AccountDiscriminator {
    Delegate = 100,
    Pool = 101,
}

use bytemuck::{Pod, Zeroable};
use shank::ShankAccount;
use utils::{impl_account_from_bytes, impl_to_bytes, Discriminator};

use super::AccountDiscriminator;

/// Board account
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, ShankAccount, Zeroable)]
pub struct Board {
    /// The bump used for signing CPIs.
    pub bump: u64,

    /// The current board state.
    pub state: [u8; 1024],
}

impl Discriminator for Board {
    fn discriminator() -> u8 {
        AccountDiscriminator::Board.into()
    }
}

impl_to_bytes!(Board);
impl_account_from_bytes!(Board);

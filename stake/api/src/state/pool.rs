use bytemuck::{Pod, Zeroable};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use crate::utils::{impl_account_from_bytes, impl_to_bytes, Discriminator};

use super::AccountDiscriminator;

/// Pool accounts can receive ORE deposits from delegators to stake in the mining protocol for a rewards multiplier.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, ShankAccount, Zeroable)]
pub struct Pool {
    /// The signer authorized to use this stake account.
    pub authority: Pubkey,

    /// The aggregate amount of ORE delegated to this stake account.
    pub balance: u64,

    /// The account bump used for signing CPIs.
    pub bump: u64,

    /// Flag indicating whether or not this stake account is accepting new deposits.
    pub is_open: u64,
}

impl Discriminator for Pool {
    fn discriminator() -> u8 {
        AccountDiscriminator::Pool.into()
    }
}

impl_to_bytes!(Pool);
impl_account_from_bytes!(Pool);

use bytemuck::{Pod, Zeroable};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use crate::utils::{impl_account_from_bytes, impl_to_bytes, Discriminator};

use super::AccountDiscriminator;

/// Stake accounts can receive ORE deposits from delegators to stake in the mining protocol for a rewards multiplier.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, ShankAccount, Zeroable)]
pub struct Stake {
    /// The signer authorized to use this stake account.
    pub authority: Pubkey,

    /// The account bump used for signing CPIs.
    pub bump: u64,

    /// Flag indicating whether or not delegated stake in this account is liquid.
    pub is_liquid: u64,

    /// Flag indicating whether or not this stake account is accepting new deposits.
    pub is_open: u64,
}

impl Discriminator for Stake {
    fn discriminator() -> u8 {
        AccountDiscriminator::Stake.into()
    }
}

impl_to_bytes!(Stake);
impl_account_from_bytes!(Stake);

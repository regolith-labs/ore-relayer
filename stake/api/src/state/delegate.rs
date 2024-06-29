use bytemuck::{Pod, Zeroable};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use crate::utils::{impl_account_from_bytes, impl_to_bytes, Discriminator};

use super::AccountDiscriminator;

/// Delegate can delegated ORE to a stake account to be used for mining.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, ShankAccount, Zeroable)]
pub struct Delegate {
    /// The signer authorized to use this delegate account.
    pub authority: Pubkey,

    /// The quantity of tokens this delegator has deposited with the staker.
    pub balance: u64,

    /// The stake account this delegate is assocated with.
    pub stake: Pubkey,
}

impl Discriminator for Delegate {
    fn discriminator() -> u8 {
        AccountDiscriminator::Delegate.into()
    }
}

impl_to_bytes!(Delegate);
impl_account_from_bytes!(Delegate);

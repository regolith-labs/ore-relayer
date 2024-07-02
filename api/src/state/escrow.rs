use bytemuck::{Pod, Zeroable};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;
use utils::{impl_account_from_bytes, impl_to_bytes, Discriminator};

use super::AccountDiscriminator;

/// Escrow account
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, ShankAccount, Zeroable)]
pub struct Escrow {
    /// The signer authorized to use this relay account.
    pub authority: Pubkey,

    /// The bump used for signing CPIs.
    pub bump: u64,

    /// The relayer this escrow account is associated with.
    pub relayer: Pubkey,
}

impl Discriminator for Escrow {
    fn discriminator() -> u8 {
        AccountDiscriminator::Escrow.into()
    }
}

impl_to_bytes!(Escrow);
impl_account_from_bytes!(Escrow);

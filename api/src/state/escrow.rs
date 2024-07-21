use bytemuck::{Pod, Zeroable};
use ore_utils::{impl_account_from_bytes, impl_to_bytes, Discriminator};
use solana_program::pubkey::Pubkey;

use super::AccountDiscriminator;

/// Escrow account
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Escrow {
    /// The signer authorized to use this relay account.
    pub authority: Pubkey,

    /// The bump used for signing CPIs.
    pub bump: u64,

    /// The last hash this relayer has collected commission on.
    pub last_hash: [u8; 32],
}

impl Default for Escrow {
    fn default() -> Self {
        Escrow {
            authority: Pubkey::new_from_array([0; 32]),
            bump: 0,
            last_hash: [0; 32],
        }
    }
}

impl Discriminator for Escrow {
    fn discriminator() -> u8 {
        AccountDiscriminator::Escrow.into()
    }
}

impl_to_bytes!(Escrow);
impl_account_from_bytes!(Escrow);

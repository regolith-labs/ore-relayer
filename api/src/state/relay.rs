use bytemuck::{Pod, Zeroable};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;
use utils::{impl_account_from_bytes, impl_to_bytes, Discriminator};

use super::AccountDiscriminator;

// TODO Authorized depositors?
// TODO Commission?

/// Relay accounts can receive ORE deposits from delegators to stake in the mining protocol for a rewards multiplier.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, ShankAccount, Zeroable)]
pub struct Relay {
    /// The signer authorized to use this relay account.
    pub authority: Pubkey,

    /// The bump used for signing CPIs.
    pub bump: u64,

    /// The address of the proof account this relayer manages
    pub proof: Pubkey,
}

impl Discriminator for Relay {
    fn discriminator() -> u8 {
        AccountDiscriminator::Relay.into()
    }
}

impl_to_bytes!(Relay);
impl_account_from_bytes!(Relay);

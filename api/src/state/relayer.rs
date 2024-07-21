use bytemuck::{Pod, Zeroable};
use ore_utils::{impl_account_from_bytes, impl_to_bytes, Discriminator};
use solana_program::pubkey::Pubkey;

use super::AccountDiscriminator;

/// Relay accounts have
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Relayer {
    /// The signer authorized to use this relay account.
    pub authority: Pubkey,

    /// The bump used for signing CPIs.
    pub bump: u64,

    /// The fixed commission of ORE the relayer is authorized to take per hash.
    pub commission: u64,

    /// The miner address users should authorize for signing.
    pub miner: Pubkey,

    /// The url where users can submit txs for signing.
    pub url: [u32; 64],
}

impl Discriminator for Relayer {
    fn discriminator() -> u8 {
        AccountDiscriminator::Relayer.into()
    }
}

impl_to_bytes!(Relayer);
impl_account_from_bytes!(Relayer);

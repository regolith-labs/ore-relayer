use bytemuck::{Pod, Zeroable};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;
use utils::{impl_account_from_bytes, impl_to_bytes, Discriminator};

use super::AccountDiscriminator;

// Relayer: Owned by relay operator
// Escrow: Shared ownership between user and relayer
// Proof: ORE hash chain

// User -> Escrow -> Proof
// Relayer -> Escrow -> Proof

// Relayer is assumed to have miner authority on proof account.
// Relayer runs a node where users can submit txs. Relayer signs user txs with miner keypair and pays tx fee.
// Relayer collects commission via Escrow account.
// Relayer publishes the URL where user can submit transactions for signing

/// Relay accounts have
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, ShankAccount, Zeroable)]
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

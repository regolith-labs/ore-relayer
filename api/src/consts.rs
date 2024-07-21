use solana_program::{pubkey, pubkey::Pubkey};

/// Fee collection address
pub const COLLECTOR_ADDRESS: Pubkey = pubkey!("HBUh9g46wk2X89CvaNN15UmsznP59rh6od1h8JwYAopk");

/// Miner pubkey
pub const MINER_PUBKEY: Pubkey = pubkey!("F7coAFJKxeo1btofymv6f6KFmN5LUC9JEGRATRqwQCXL");

/// The seed of the escrow account PDA.
pub const ESCROW: &[u8] = b"escrow";

/// The ore commission the relayer is allowed to collect
pub const COMMISSION: u64 = 100;

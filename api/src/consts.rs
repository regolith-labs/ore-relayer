use solana_program::{pubkey, pubkey::Pubkey};

/// The admin allowed open new relayers.
// pub const AUTHORIZED_RELAYER: Pubkey = pubkey!("HBUh9g46wk2X89CvaNN15UmsznP59rh6od1h8JwYAopk");
pub const AUTHORIZED_RELAYER: Pubkey = pubkey!("DEuG4JnzvMVxMFPoBVvf2GH38mn3ybunMxtfmVU3ms86");

/// The seed of the escrow account PDA.
pub const ESCROW: &[u8] = b"escrow";

/// The seed of the pool account PDA.
pub const RELAYER: &[u8] = b"relayer";

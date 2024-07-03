use solana_program::{pubkey, pubkey::Pubkey};

/// The admin allowed open new relayers.
pub const AUTHORIZED_RELAYER: Pubkey = pubkey!("HBUh9g46wk2X89CvaNN15UmsznP59rh6od1h8JwYAopk");

/// The seed of the board account PDA.
pub const BOARD: &[u8] = b"board";

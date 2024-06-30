use solana_program::{pubkey, pubkey::Pubkey};

/// The admin allowed to initialize the program.
pub const INITIAL_ADMIN: Pubkey = pubkey!("HBUh9g46wk2X89CvaNN15UmsznP59rh6od1h8JwYAopk");

/// The seed of the delegate account PDA.
pub const DELEGATE: &[u8] = b"delegate";

/// The seed of the pool account PDA.
pub const POOL: &[u8] = b"pool";

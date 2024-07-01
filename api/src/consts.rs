use solana_program::{pubkey, pubkey::Pubkey};

/// The admin allowed to initialize the program.
pub const INITIAL_ADMIN: Pubkey = pubkey!("HBUh9g46wk2X89CvaNN15UmsznP59rh6od1h8JwYAopk");

/// The commission to collect from mining (1%)
pub const COMISSION: u64 = 10_000;

/// The commission to collect from mining
pub const COMISSION_DENOMINATOR: u64 = 1_000_000;

/// The seed of the pool account PDA.
pub const RELAY: &[u8] = b"relay";

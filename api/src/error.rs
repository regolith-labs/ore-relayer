use num_enum::IntoPrimitive;
use solana_program::program_error::ProgramError;
use thiserror::Error;

// TODO Enumerate error types

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum RelayError {
    #[error("This is a placeholder error")]
    Dummy = 0,
}

impl From<RelayError> for ProgramError {
    fn from(e: RelayError) -> Self {
        let f = (e as u32) + 200;
        ProgramError::Custom(f)
    }
}

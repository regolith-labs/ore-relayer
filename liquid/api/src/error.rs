use num_enum::IntoPrimitive;
use solana_program::program_error::ProgramError;
use thiserror::Error;

// TODO Enumerate error types

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum StakeError {
    #[error("This is a placeholder error")]
    Dummy = 0,
}

impl From<StakeError> for ProgramError {
    fn from(e: StakeError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

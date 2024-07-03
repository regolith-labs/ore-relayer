use num_enum::IntoPrimitive;
use solana_program::program_error::ProgramError;
use thiserror::Error;

// TODO Enumerate error types

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum CheckoresError {
    #[error("This is a placeholder error")]
    Dummy = 0,
}

impl From<CheckoresError> for ProgramError {
    fn from(e: CheckoresError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};
use utils::{AccountDeserialize, Discriminator};

pub use crate::state::*;
pub use ore_api::loaders::*;

/// Errors if:
/// - Owner is not relay program.
/// - Data is empty.
/// - Account cannot be parsed to a relayer account.
/// - Relayer authority is not expected value.
/// - Expected to be writable, but is not.
pub fn load_relayer<'a, 'info>(
    info: &'a AccountInfo<'info>,
    authority: Pubkey,
    is_writable: bool,
) -> Result<(), ProgramError> {
    if info.owner.ne(&ore_api::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    let relayer_data = info.data.borrow();
    let relayer = Relayer::try_from_bytes(&relayer_data)?;

    if relayer.authority.ne(&authority) {
        return Err(ProgramError::InvalidAccountData);
    }

    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// Errors if:
/// - Owner is not relay program.
/// - Data is empty.
/// - Account discriminator is not a relayer account.
/// - Expected to be writable, but is not.
pub fn load_any_relayer<'a, 'info>(
    info: &'a AccountInfo<'info>,
    is_writable: bool,
) -> Result<(), ProgramError> {
    if info.owner.ne(&ore_api::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    if info.data.borrow()[0].ne(&(Relayer::discriminator() as u8)) {
        return Err(solana_program::program_error::ProgramError::InvalidAccountData);
    }

    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

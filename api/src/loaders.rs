use ore_utils::AccountDeserialize;
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

pub use crate::state::*;
pub use ore_api::loaders::*;

/// Errors if:
/// - Owner is not relay program.
/// - Data is empty.
/// - Account cannot be parsed to a escrow account.
/// - Escrow authority is not expected value.
/// - Expected to be writable, but is not.
pub fn load_escrow<'a, 'info>(
    info: &'a AccountInfo<'info>,
    authority: &Pubkey,
    is_writable: bool,
) -> Result<(), ProgramError> {
    if info.owner.ne(&crate::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    let escrow_data = info.data.borrow();
    let escrow = Escrow::try_from_bytes(&escrow_data)?;

    if escrow.authority.ne(&authority) {
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
/// - Account cannot be parsed to a escrow account.
/// - Expected to be writable, but is not.
pub fn load_any_escrow<'a, 'info>(
    info: &'a AccountInfo<'info>,
    is_writable: bool,
) -> Result<(), ProgramError> {
    if info.owner.ne(&crate::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    let escrow_data = info.data.borrow();
    let _ = Escrow::try_from_bytes(&escrow_data)?;

    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

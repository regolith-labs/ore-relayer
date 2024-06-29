use ore_stake_api::state::{Delegate, Stake};
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};
use utils::{AccountDeserialize, Discriminator};

pub use crate::utils::loaders::*;

/// Errors if:
/// - Owner is not ORE stake program.
/// - Data is empty.
/// - Delegate authority is not valid.
/// - Expected to be writable, but is not.
pub fn load_delegate<'a, 'info>(
    info: &'a AccountInfo<'info>,
    authority: Pubkey,
    stake: Pubkey,
    is_writable: bool,
) -> Result<(), ProgramError> {
    if info.owner.ne(&ore_api::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    let delegate_data = info.data.borrow();
    let delegate = Delegate::try_from_bytes(&delegate_data)?;

    if delegate.authority.ne(&authority) {
        return Err(ProgramError::InvalidAccountData);
    }

    if delegate.stake.ne(&stake) {
        return Err(ProgramError::InvalidAccountData);
    }

    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// Errors if:
/// - Owner is not ORE stake program.
/// - Data is empty.
/// - Account discriminator is not a stake account.
/// - Expected to be writable, but is not.
pub fn load_any_stake<'a, 'info>(
    info: &'a AccountInfo<'info>,
    is_writable: bool,
) -> Result<(), ProgramError> {
    if info.owner.ne(&ore_api::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    if info.data.borrow()[0].ne(&(Stake::discriminator() as u8)) {
        return Err(solana_program::program_error::ProgramError::InvalidAccountData);
    }

    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

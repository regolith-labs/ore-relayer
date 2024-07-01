use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};
use utils::AccountDeserialize;

pub use crate::state::*;
pub use ore_api::loaders::*;

/// Errors if:
/// - Owner is not ORE stake program.
/// - Data is empty.
/// - Account discriminator is not a stake account.
/// - Expected to be writable, but is not.
pub fn load_relay<'a, 'info>(
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

    let relay_data = info.data.borrow();
    let relay = Relay::try_from_bytes(&relay_data)?;

    if relay.authority.ne(&authority) {
        return Err(ProgramError::InvalidAccountData);
    }

    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

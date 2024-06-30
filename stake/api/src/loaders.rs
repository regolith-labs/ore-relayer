use ore_api::consts::MINT;
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};
use utils::Discriminator;

pub use crate::{state::*, utils::loaders::*};

/// Errors if:
/// - Owner is not ORE stake program.
/// - Data is empty.
/// - Account discriminator is not a stake account.
/// - Expected to be writable, but is not.
pub fn load_any_pool<'a, 'info>(
    info: &'a AccountInfo<'info>,
    is_writable: bool,
) -> Result<(), ProgramError> {
    if info.owner.ne(&ore_api::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    if info.data.borrow()[0].ne(&(Pool::discriminator() as u8)) {
        return Err(solana_program::program_error::ProgramError::InvalidAccountData);
    }

    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// Errors if:
/// - Mint address is not expected.
/// - Account is not a valid mint.
pub fn load_pool_mint<'a, 'info>(
    info: &'a AccountInfo<'info>,
    pool: Pubkey,
    is_writable: bool,
) -> Result<(), ProgramError> {
    let mint_pda = Pubkey::find_program_address(&[MINT, pool.as_ref()], &crate::id());
    load_mint(info, mint_pda.0, is_writable)
}

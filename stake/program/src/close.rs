use ore_stake_api::{error::StakeError, loaders::*, state::Delegate};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    system_program,
};

use crate::utils::AccountDeserialize;

/// Closes a delegate account.
pub fn process_close<'a, 'info>(accounts: &'a [AccountInfo<'info>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer, delegate_info, pool_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_delegate(delegate_info, *signer.key, *pool_info.key, true)?;
    load_any_pool(pool_info, false)?;
    load_program(system_program, system_program::id())?;

    // Error if delegate balance is non-zero.
    let delegate_data = delegate_info.data.borrow();
    let delegate = Delegate::try_from_bytes(&delegate_data)?;
    if delegate.balance.ne(&0) {
        return Err(StakeError::Dummy.into());
    }
    drop(delegate_data);

    // Realloc data to zero.
    delegate_info.realloc(0, true)?;

    // Send lamports to signer.
    **signer.lamports.borrow_mut() += delegate_info.lamports();
    **delegate_info.lamports.borrow_mut() = 0;

    Ok(())
}

use std::mem::size_of;

use ore_stake_api::{consts::*, instruction::OpenArgs, loaders::*, state::Delegate};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    system_program,
};

use crate::utils::{create_pda, AccountDeserialize, Discriminator};

/// Opens a new delegate account.
pub fn process_open<'a, 'info>(accounts: &'a [AccountInfo<'info>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = OpenArgs::try_from_bytes(data)?;

    // Load accounts.
    let [signer, delegate_info, pool_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_uninitialized_pda(
        delegate_info,
        &[DELEGATE, signer.key.as_ref(), pool_info.key.as_ref()],
        args.bump,
        &ore_api::id(),
    )?;
    load_any_pool(pool_info, false)?;
    load_program(system_program, system_program::id())?;

    // Initialize the delegate account.
    create_pda(
        delegate_info,
        &ore_stake_api::id(),
        8 + size_of::<Delegate>(),
        &[
            DELEGATE,
            signer.key.as_ref(),
            pool_info.key.as_ref(),
            &[args.bump],
        ],
        system_program,
        signer,
    )?;
    let mut delegate_data = delegate_info.data.borrow_mut();
    delegate_data[0] = Delegate::discriminator() as u8;
    let delegate = Delegate::try_from_bytes_mut(&mut delegate_data)?;
    delegate.authority = *signer.key;
    delegate.balance = 0;
    delegate.stake = *pool_info.key;

    Ok(())
}

use std::mem::size_of;

use ore_relayer_api::{
    consts::*, error::RelayError, instruction::OpenRelayerArgs, loaders::*, state::Relayer,
};
use ore_utils::{create_pda, AccountDeserialize, Discriminator};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    system_program,
};

/// Opens a new relay account.
pub fn process_open_relayer<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    data: &[u8],
) -> ProgramResult {
    // Parse args
    let args = OpenRelayerArgs::try_from_bytes(data)?;

    // Load accounts.
    let [signer, miner_info, relayer_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_system_account(miner_info, false)?;
    load_uninitialized_pda(
        relayer_info,
        &[RELAYER, signer.key.as_ref()],
        args.bump,
        &ore_relayer_api::id(),
    )?;
    load_program(system_program, system_program::id())?;

    // Error if non-authorized party tries to create a relayer.
    if signer.key.ne(&AUTHORIZED_RELAYER) {
        return Err(RelayError::Dummy.into());
    }

    // Initialize relay account.
    create_pda(
        relayer_info,
        &ore_relayer_api::id(),
        8 + size_of::<Relayer>(),
        &[RELAYER, signer.key.as_ref(), &[args.bump]],
        system_program,
        signer,
    )?;
    let mut relayer_data = relayer_info.data.borrow_mut();
    relayer_data[0] = Relayer::discriminator() as u8;
    let relayer = Relayer::try_from_bytes_mut(&mut relayer_data)?;
    relayer.authority = *signer.key;
    relayer.bump = args.bump as u64;
    relayer.commission = 1_100;
    relayer.miner = *miner_info.key;

    Ok(())
}

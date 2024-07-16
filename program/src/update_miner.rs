use ore_relayer_api::{consts::*, error::RelayError, loaders::*};
use ore_utils::AccountDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

/// Updates the miner authority for a particular proof account.
pub fn process_update_miner<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    _data: &[u8],
) -> ProgramResult {
    // Load accounts.
    let [signer, escrow_info, miner_info, proof_info, ore_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_any_escrow(escrow_info, false)?;
    load_system_account(miner_info, false)?;
    load_proof(proof_info, escrow_info.key, true)?;
    load_program(ore_program, ore_api::id())?;

    // Error if signer is not valid
    if signer.key.ne(&MINER_PUBKEY) {
        return Err(RelayError::Dummy.into());
    }

    // Update the miner keypair on the proof account.
    let escrow_data = escrow_info.data.borrow();
    let escrow = Escrow::try_from_bytes(&escrow_data)?;
    let escrow_authority = escrow.authority;
    let escrow_bump = escrow.bump as u8;
    drop(escrow_data);
    solana_program::program::invoke_signed(
        &ore_api::instruction::update(*escrow_info.key, *miner_info.key),
        &[
            ore_program.clone(),
            escrow_info.clone(),
            miner_info.clone(),
            proof_info.clone(),
        ],
        &[&[ESCROW, escrow_authority.as_ref(), &[escrow_bump]]],
    )?;

    Ok(())
}

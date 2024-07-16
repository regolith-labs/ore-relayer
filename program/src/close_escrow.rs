use ore_relayer_api::{consts::*, loaders::*};
use ore_utils::AccountDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    system_program,
};

/// Closes an escrow account.
pub fn process_close_escrow<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    _data: &[u8],
) -> ProgramResult {
    // Load accounts.
    let [signer, escrow_info, proof_info, ore_program, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_escrow(escrow_info, signer.key, true)?;
    load_proof(proof_info, escrow_info.key, true)?;
    load_program(ore_program, ore_api::id())?;
    load_program(system_program, system_program::id())?;

    // Close the proof account.
    let escrow_data = escrow_info.data.borrow();
    let escrow = Escrow::try_from_bytes(&escrow_data)?;
    let escrow_bump = escrow.bump as u8;
    drop(escrow_data);
    solana_program::program::invoke_signed(
        &ore_api::instruction::close(*escrow_info.key),
        &[
            escrow_info.clone(),
            proof_info.clone(),
            system_program.clone(),
        ],
        &[&[ESCROW, signer.key.as_ref(), &[escrow_bump]]],
    )?;

    // Realloc data to zero
    escrow_info.realloc(0, true)?;

    // Send lamports to signer
    **signer.lamports.borrow_mut() += escrow_info.lamports();
    **escrow_info.lamports.borrow_mut() = 0;

    Ok(())
}

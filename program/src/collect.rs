use ore_api::{consts::MINT_ADDRESS, state::Proof};
use ore_relayer_api::{consts::*, error::RelayError, instruction::CollectArgs, loaders::*};
use ore_utils::AccountDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    system_program,
};

/// Collects commission from a miner.
pub fn process_collect<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    data: &[u8],
) -> ProgramResult {
    // Parse args
    let args = CollectArgs::try_from_bytes(data)?;
    let fee = u64::from_le_bytes(args.fee);

    // Load accounts.
    let [signer, beneficiary_info, escrow_info, proof_info, treasury_info, treasury_tokens_info, system_program, ore_program, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_token_account(beneficiary_info, None, &MINT_ADDRESS, true)?;
    load_any_escrow(escrow_info, true)?;
    load_proof(proof_info, escrow_info.key, true)?;
    load_treasury(treasury_info, false)?;
    load_treasury_tokens(treasury_tokens_info, true)?;
    load_program(system_program, system_program::id())?;
    load_program(ore_program, ore_api::id())?;
    load_program(token_program, spl_token::id())?;

    // Verify signer
    if signer.key.ne(&MINER_PUBKEY) {
        return Err(RelayError::Dummy.into());
    }

    // Error if the last hash is the same (don't allow double collections)
    let mut escrow_data = escrow_info.data.borrow_mut();
    let escrow = Escrow::try_from_bytes_mut(&mut escrow_data)?;
    let proof_data = proof_info.data.borrow();
    let proof = Proof::try_from_bytes(&proof_data)?;
    if escrow.last_hash.eq(&proof.last_hash) {
        return Err(RelayError::Dummy.into());
    }

    // Update the last hash
    escrow.last_hash = proof.last_hash;

    // Return early if proof doesn't have sufficient balance
    if proof.balance.lt(&COMMISSION) {
        return Ok(());
    }

    // Claim commission
    let escrow_authority = escrow.authority;
    let escrow_bump = escrow.bump as u8;
    drop(escrow_data);
    drop(proof_data);
    solana_program::program::invoke_signed(
        &ore_api::instruction::claim(*escrow_info.key, *beneficiary_info.key, COMMISSION),
        &[
            escrow_info.clone(),
            beneficiary_info.clone(),
            proof_info.clone(),
            treasury_info.clone(),
            treasury_tokens_info.clone(),
            token_program.clone(),
        ],
        &[&[ESCROW, escrow_authority.as_ref(), &[escrow_bump]]],
    )?;

    // Send transaction fee to miner
    **signer.lamports.borrow_mut() += fee;
    **escrow_info.lamports.borrow_mut() -= fee;

    Ok(())
}

use ore_api::consts::MINT_ADDRESS;
use ore_relay_api::{consts::*, instruction::ClaimArgs, loaders::*};
use ore_utils::AccountDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

/// Claims ORE from a user proof account.
pub fn process_claim<'a, 'info>(accounts: &'a [AccountInfo<'info>], data: &[u8]) -> ProgramResult {
    // Parse args
    let args = ClaimArgs::try_from_bytes(data)?;

    // Load accounts.
    let [signer, beneficiary_info, escrow_info, proof_info, treasury_info, treasury_tokens_info, ore_program, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_token_account(beneficiary_info, None, &MINT_ADDRESS, true)?;
    load_escrow(escrow_info, signer.key, true)?;
    load_proof(proof_info, escrow_info.key, true)?;
    load_treasury(treasury_info, true)?;
    load_treasury_tokens(treasury_tokens_info, true)?;
    load_program(ore_program, ore_api::id())?;
    load_program(token_program, spl_token::id())?;

    // Claim stake to beneficiary
    let escrow_data = escrow_info.data.borrow();
    let escrow = Escrow::try_from_bytes(&escrow_data)?;
    let escrow_bump = escrow.bump as u8;
    let escrow_relayer = escrow.relayer;
    drop(escrow_data);
    solana_program::program::invoke_signed(
        &ore_api::instruction::claim(*escrow_info.key, *beneficiary_info.key, args.amount),
        &[
            escrow_info.clone(),
            beneficiary_info.clone(),
            proof_info.clone(),
            treasury_info.clone(),
            treasury_tokens_info.clone(),
            token_program.clone(),
        ],
        &[&[
            ESCROW,
            signer.key.as_ref(),
            escrow_relayer.as_ref(),
            &[escrow_bump],
        ]],
    )?;

    Ok(())
}

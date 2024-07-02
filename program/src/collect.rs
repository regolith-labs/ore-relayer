use ore_api::consts::MINT_ADDRESS;
use ore_relay_api::{consts::*, loaders::*};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};
use utils::AccountDeserialize;

/// Collects commission from a miner.
pub fn process_collect<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    _data: &[u8],
) -> ProgramResult {
    // Load accounts.
    let [signer, beneficiary_info, escrow_info, proof_info, relayer_info, treasury_info, treasury_tokens_info, ore_program, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_token_account(beneficiary_info, None, &MINT_ADDRESS, true)?;
    load_escrow_with_relayer(escrow_info, relayer_info.key, true)?;
    load_proof(proof_info, escrow_info.key, true)?;
    load_relayer(relayer_info, signer.key, true)?;
    load_treasury(treasury_info, true)?;
    load_token_account(
        treasury_tokens_info,
        Some(treasury_info.key),
        &MINT_ADDRESS,
        true,
    )?;
    load_program(ore_program, ore_api::id())?;
    load_program(token_program, spl_token::id())?;

    // TODO Get amount to claim
    let amount = 10;

    // Claim commission
    let escrow_data = escrow_info.data.borrow();
    let escrow = Escrow::try_from_bytes(&escrow_data)?;
    let escrow_bump = escrow.bump as u8;
    drop(escrow_data);
    solana_program::program::invoke_signed(
        &ore_api::instruction::claim(*escrow_info.key, *beneficiary_info.key, amount),
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
            relayer_info.key.as_ref(),
            &[escrow_bump],
        ]],
    )?;

    Ok(())
}

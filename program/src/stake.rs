use ore_api::{consts::MINT_ADDRESS, instruction::StakeArgs};
use ore_relay_api::{consts::*, loaders::*};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};
use utils::AccountDeserialize;

/// Stakes ORE with the user's proof account.
pub fn process_stake<'a, 'info>(accounts: &'a [AccountInfo<'info>], data: &[u8]) -> ProgramResult {
    // Parse args
    let args = StakeArgs::try_from_bytes(data)?;

    // Load accounts.
    let [signer, escrow_info, escrow_tokens_info, proof_info, treasury_tokens_info, ore_program, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_escrow(escrow_info, signer.key, true)?;
    load_token_account(
        escrow_tokens_info,
        Some(escrow_info.key),
        &MINT_ADDRESS,
        true,
    )?;
    load_proof(proof_info, escrow_info.key, true)?;
    load_treasury_tokens(treasury_tokens_info, true)?;
    load_program(ore_program, ore_api::id())?;
    load_program(token_program, spl_token::id())?;

    // Get amount to stake
    let amount = u64::from_le_bytes(args.amount);

    // TODO Transfer tokens from sender to escrow account

    // Stake ORE from escrow account
    let escrow_data = escrow_info.data.borrow();
    let escrow = Escrow::try_from_bytes(&escrow_data)?;
    let escrow_bump = escrow.bump as u8;
    let escrow_relayer = escrow.relayer;
    drop(escrow_data);
    solana_program::program::invoke_signed(
        &ore_api::instruction::stake(*escrow_info.key, *escrow_tokens_info.key, amount),
        &[
            ore_program.clone(),
            escrow_info.clone(),
            proof_info.clone(),
            escrow_tokens_info.clone(),
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

use ore_api::{
    consts::{MINT_ADDRESS, TREASURY_ADDRESS},
    loaders::load_proof,
    state::Proof,
};
use ore_relayer_api::{consts::POOL, instruction::WithdrawArgs, loaders::*, state::Pool};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    program_pack::Pack,
};
use spl_token::state::Mint;

use crate::utils::AccountDeserialize;

/// Withdraw ORE from a delegated stake account.
pub fn process_withdraw<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    data: &[u8],
) -> ProgramResult {
    // Parse args.
    let args = WithdrawArgs::try_from_bytes(data)?;

    // Load accounts.
    let [signer, pool_info, pool_mint_info, pool_tokens_info, proof_info, recipient_info, sender_info, treasury_tokens_info, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_any_pool(pool_info, false)?;
    load_pool_mint(pool_mint_info, pool_info.key, true)?;
    load_token_account(pool_tokens_info, Some(pool_info.key), &MINT_ADDRESS, true)?;
    load_proof(proof_info, pool_info.key, true)?;
    load_token_account(recipient_info, Some(signer.key), &MINT_ADDRESS, true)?;
    load_token_account(sender_info, Some(signer.key), pool_mint_info.key, true)?;
    load_token_account(
        treasury_tokens_info,
        Some(&TREASURY_ADDRESS),
        &MINT_ADDRESS,
        true,
    )?;
    load_program(token_program, spl_token::id())?;

    // Calculate claim amount and update balances.
    let proof_data = proof_info.data.borrow();
    let proof = Proof::try_from_bytes(&proof_data)?;
    let pool_mint_data = pool_mint_info.data.borrow();
    let pool_mint = Mint::unpack(&pool_mint_data).expect("Failed to parse mint");
    let claim_amount = proof
        .balance
        .saturating_mul(args.amount)
        .saturating_div(pool_mint.supply);

    // Claim ORE stake from core contract.
    let pool_data = pool_info.data.borrow();
    let pool = Pool::try_from_bytes(&pool_data)?;
    let pool_bump = pool.bump as u8;
    let pool_authority = pool.authority;
    drop(proof_data);
    drop(pool_data);
    solana_program::program::invoke_signed(
        &ore_api::instruction::claim(*pool_info.key, *recipient_info.key, claim_amount),
        &[
            pool_info.clone(),
            proof_info.clone(),
            pool_tokens_info.clone(),
            treasury_tokens_info.clone(),
            token_program.clone(),
        ],
        &[&[POOL, pool_authority.as_ref(), &[pool_bump]]],
    )?;

    // Burn pool tokens.
    drop(pool_mint_data);
    solana_program::program::invoke(
        &spl_token::instruction::burn(
            &spl_token::id(),
            sender_info.key,
            pool_mint_info.key,
            signer.key,
            &[signer.key],
            args.amount,
        )?,
        &[
            token_program.clone(),
            sender_info.clone(),
            pool_mint_info.clone(),
            signer.clone(),
        ],
    )?;

    Ok(())
}

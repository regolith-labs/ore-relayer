use ore_api::{
    consts::{MINT_ADDRESS, TREASURY_ADDRESS},
    loaders::load_proof,
    state::Proof,
};
use ore_stake_api::{
    consts::POOL,
    error::StakeError,
    instruction::WithdrawArgs,
    loaders::*,
    state::{Delegate, Pool},
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::utils::AccountDeserialize;

/// Withdraw ORE from a delegated stake account.
pub fn process_withdraw<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    data: &[u8],
) -> ProgramResult {
    // Parse args.
    let args = WithdrawArgs::try_from_bytes(data)?;

    // Load accounts.
    let [signer, beneficiary_info, delegate_info, pool_info, pool_tokens_info, proof_info, treasury_tokens_info, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_token_account(beneficiary_info, None, &MINT_ADDRESS, true)?;
    load_delegate(delegate_info, *signer.key, *pool_info.key, true)?;
    load_any_pool(pool_info, true)?;
    load_token_account(pool_tokens_info, Some(pool_info.key), &MINT_ADDRESS, true)?;
    load_proof(proof_info, pool_info.key, true)?;
    load_token_account(
        treasury_tokens_info,
        Some(&TREASURY_ADDRESS),
        &MINT_ADDRESS,
        true,
    )?;
    load_program(token_program, spl_token::id())?;

    // Error if withdraw amount is too large.
    let mut delegate_data = delegate_info.data.borrow_mut();
    let delegate = Delegate::try_from_bytes_mut(&mut delegate_data)?;
    if args.amount.gt(&delegate.balance) {
        return Err(StakeError::Dummy.into());
    }

    // Calculate claim amount and update balances.
    let mut pool_data = pool_info.data.borrow_mut();
    let pool = Pool::try_from_bytes_mut(&mut pool_data)?;
    let proof_data = proof_info.data.borrow();
    let proof = Proof::try_from_bytes(&proof_data)?;
    let claim_amount = proof
        .balance
        .saturating_mul(args.amount)
        .saturating_div(pool.balance);
    delegate.balance = delegate.balance.saturating_sub(args.amount);
    pool.balance = pool.balance.saturating_sub(args.amount);

    // Claim ORE stake from core contract.
    let pool_bump = pool.bump as u8;
    let pool_authority = pool.authority;
    drop(proof_data);
    drop(pool_data);
    solana_program::program::invoke_signed(
        &ore_api::instruction::claim(*pool_info.key, *beneficiary_info.key, claim_amount),
        &[
            pool_info.clone(),
            proof_info.clone(),
            pool_tokens_info.clone(),
            treasury_tokens_info.clone(),
            token_program.clone(),
        ],
        &[&[POOL, pool_authority.as_ref(), &[pool_bump]]],
    )?;

    // TODO If liquid, burn liquid tokens

    Ok(())
}

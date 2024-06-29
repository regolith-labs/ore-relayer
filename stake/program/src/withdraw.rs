use ore_api::{
    consts::{MINT_ADDRESS, TREASURY_ADDRESS},
    loaders::load_proof,
    state::Proof,
};
use ore_stake_api::{
    consts::STAKE,
    error::StakeError,
    instruction::WithdrawArgs,
    loaders::*,
    state::{Delegate, Stake},
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::utils::AccountDeserialize;

/// Withdraw ORE from a delegated stake account.
pub fn process_withdraw<'a, 'info>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'info>],
    data: &[u8],
) -> ProgramResult {
    // Parse args.
    let args = WithdrawArgs::try_from_bytes(data)?;

    // Load accounts.
    let [signer, beneficiary_info, delegate_info, proof_info, stake_info, stake_tokens_info, treasury_tokens_info, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_delegate(delegate_info, *signer.key, *stake_info.key, true)?;
    load_token_account(beneficiary_info, None, &MINT_ADDRESS, true)?;
    load_proof(proof_info, stake_info.key, true)?;
    load_any_stake(stake_info, true)?;
    load_token_account(stake_tokens_info, Some(stake_info.key), &MINT_ADDRESS, true)?;
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
    let mut stake_data = stake_info.data.borrow_mut();
    let stake = Stake::try_from_bytes_mut(&mut stake_data)?;
    let proof_data = proof_info.data.borrow();
    let proof = Proof::try_from_bytes(&proof_data)?;
    let claim_amount = proof
        .balance
        .saturating_mul(args.amount)
        .saturating_div(stake.balance);
    delegate.balance = delegate.balance.saturating_sub(args.amount);
    stake.balance = stake.balance.saturating_sub(args.amount);

    // Claim ORE stake from core contract.
    let stake_bump = stake.bump as u8;
    let stake_authority = stake.authority;
    drop(proof_data);
    drop(stake_data);
    solana_program::program::invoke_signed(
        &ore_api::instruction::claim(*stake_info.key, *beneficiary_info.key, claim_amount),
        &[
            stake_info.clone(),
            proof_info.clone(),
            stake_tokens_info.clone(),
            treasury_tokens_info.clone(),
            token_program.clone(),
        ],
        &[&[STAKE, stake_authority.as_ref(), &[stake_bump]]],
    )?;

    // TODO If liquid, burn liquid tokens

    Ok(())
}

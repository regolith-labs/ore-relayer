use ore_api::{consts::MINT_ADDRESS, loaders::*};
use ore_stake_api::{
    consts::*,
    instruction::DelegateArgs,
    loaders::*,
    state::{Delegate, Stake},
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::utils::AccountDeserialize;

// TODO Hold ORE in escrow until it is manually activated.
//      Otherwise the delegate function can maliciously be used to nullify a miner's multiplier (one-minute warmup requirement).
//      Figure out how to safely withdraw given some stake might not be activated yet.

/// Delegates ORE to a stake account.
pub fn process_delegate<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    data: &[u8],
) -> ProgramResult {
    // Parse args.
    let args = DelegateArgs::try_from_bytes(data)?;

    // Load accounts.
    let [signer, delegate_info, proof_info, sender_info, stake_info, stake_tokens_info, treasury_tokens_info, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_delegate(delegate_info, *signer.key, *stake_info.key, true)?;
    load_token_account(sender_info, Some(signer.key), &MINT_ADDRESS, true)?;
    load_any_stake(stake_info, true)?;
    load_token_account(stake_tokens_info, Some(stake_info.key), &MINT_ADDRESS, true)?;

    // Update balances.
    let mut delegate_data = delegate_info.data.borrow_mut();
    let delegate = Delegate::try_from_bytes_mut(&mut delegate_data)?;
    let mut stake_data = stake_info.data.borrow_mut();
    let stake = Stake::try_from_bytes_mut(&mut stake_data)?;
    delegate.balance = delegate.balance.saturating_add(args.amount);
    stake.balance = stake.balance.saturating_add(args.amount);

    // Transfer tokens from sender to escrow account.
    solana_program::program::invoke(
        &spl_token::instruction::transfer(
            &spl_token::id(),
            sender_info.key,
            stake_tokens_info.key,
            signer.key,
            &[signer.key],
            args.amount,
        )?,
        &[
            token_program.clone(),
            sender_info.clone(),
            stake_tokens_info.clone(),
            signer.clone(),
        ],
    )?;

    // Stake tokens from escrow account into ORE contract.
    let stake_bump = stake.bump as u8;
    let stake_authority = stake.authority;
    drop(stake_data);
    solana_program::program::invoke_signed(
        &ore_api::instruction::stake(*stake_info.key, *stake_tokens_info.key, args.amount),
        &[
            stake_info.clone(),
            proof_info.clone(),
            stake_tokens_info.clone(),
            treasury_tokens_info.clone(),
            token_program.clone(),
        ],
        &[&[STAKE, stake_authority.as_ref(), &[stake_bump]]],
    )?;

    // TODO If not accepting new stake, then error out
    // TODO If liquid, mint liquid tokens

    Ok(())
}

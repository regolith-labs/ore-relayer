use ore_api::{
    consts::{MINT_ADDRESS, TREASURY_ADDRESS},
    loaders::*,
    state::Proof,
};
use ore_stake_api::{
    consts::*, error::StakeError, instruction::DelegateArgs, loaders::*, state::Pool,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    program_pack::Pack,
};
use spl_token::state::Mint;

use crate::utils::AccountDeserialize;

/// Delegates ORE to a stake account.
pub fn process_delegate<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    data: &[u8],
) -> ProgramResult {
    // Parse args.
    let args = DelegateArgs::try_from_bytes(data)?;

    // Load accounts.
    let [signer, ore_mint_info, pool_info, pool_mint_info, pool_tokens_info, proof_info, recipient_info, sender_info, treasury_tokens_info, ore_program, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_mint(ore_mint_info, MINT_ADDRESS, true)?;
    load_any_pool(pool_info, false)?;
    load_pool_mint(pool_mint_info, pool_info.key, true)?;
    load_token_account(pool_tokens_info, Some(pool_info.key), &MINT_ADDRESS, true)?;
    load_proof(proof_info, pool_info.key, true)?;
    load_token_account(recipient_info, Some(signer.key), pool_mint_info.key, true)?;
    load_token_account(sender_info, Some(signer.key), &MINT_ADDRESS, true)?;
    load_token_account(
        treasury_tokens_info,
        Some(&TREASURY_ADDRESS),
        &MINT_ADDRESS,
        true,
    )?;
    load_program(ore_program, ore_api::id())?;
    load_program(token_program, spl_token::id())?;

    // If not accepting new stake, then error out
    let pool_data = pool_info.data.borrow();
    let pool = Pool::try_from_bytes(&pool_data)?;
    if pool.is_open.eq(&0) {
        return Err(StakeError::Dummy.into());
    }

    // Calculate correct amount of pool tokens to mint to user.
    // x / (pool_mint.supply + x) = args.amount / (proof.balance + args.amount)
    // x = (pool_mint.supply)(args.amount) / proof.balance
    let proof_data = proof_info.data.borrow();
    let proof = Proof::try_from_bytes(&proof_data)?;
    let pool_mint_data = pool_mint_info.data.borrow();
    let pool_mint = Mint::unpack(&pool_mint_data).expect("Failed to parse mint");
    let amount_to_mint = if pool_mint.supply.eq(&0) || proof.balance.eq(&0) {
        args.amount
    } else {
        pool_mint
            .supply
            .saturating_mul(args.amount)
            .saturating_div(proof.balance)
    };

    // Transfer tokens from sender to escrow account.
    solana_program::program::invoke(
        &spl_token::instruction::transfer(
            &spl_token::id(),
            sender_info.key,
            pool_tokens_info.key,
            signer.key,
            &[signer.key],
            args.amount,
        )?,
        &[
            token_program.clone(),
            sender_info.clone(),
            pool_tokens_info.clone(),
            signer.clone(),
        ],
    )?;

    // Send tokens from escrow account into ORE contract.
    let pool_bump = pool.bump as u8;
    let pool_authority = pool.authority;
    drop(pool_data);
    solana_program::program::invoke_signed(
        &ore_api::instruction::stake(*pool_info.key, *pool_tokens_info.key, args.amount),
        &[
            pool_info.clone(),
            proof_info.clone(),
            pool_tokens_info.clone(),
            treasury_tokens_info.clone(),
            token_program.clone(),
            ore_program.clone(),
        ],
        &[&[POOL, pool_authority.as_ref(), &[pool_bump]]],
    )?;

    // Mint new pool tokens to the recipient to represent their share of the pool.
    drop(pool_mint_data);
    solana_program::program::invoke_signed(
        &spl_token::instruction::mint_to(
            &spl_token::id(),
            pool_mint_info.key,
            recipient_info.key,
            signer.key,
            &[pool_info.key],
            amount_to_mint,
        )?,
        &[
            token_program.clone(),
            pool_mint_info.clone(),
            recipient_info.clone(),
            pool_info.clone(),
        ],
        &[&[POOL, pool_authority.as_ref(), &[pool_bump]]],
    )?;

    Ok(())
}

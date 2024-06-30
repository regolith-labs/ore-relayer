use std::mem::size_of;

use ore_api::consts::*;
use ore_stake_api::{consts::*, instruction::InitializeArgs, loaders::*, state::Pool};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    system_program,
    sysvar::{self},
};

use crate::utils::{create_pda, AccountDeserialize, Discriminator};

// TODO Create a new mint for this Pool account
// TODO Create metadata for the token
// TODO Delegate mint authority to the pool authority

/// Initializes a new stake account.
pub fn process_initialize<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    data: &[u8],
) -> ProgramResult {
    // Parse args.
    let args = InitializeArgs::try_from_bytes(data)?;

    // Load accounts.
    let [signer, miner_info, mint_info, pool_info, pool_tokens_info, proof_info, system_program, token_program, associated_token_program, slot_hashes_info] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_system_account(miner_info, false)?;
    load_mint(mint_info, MINT_ADDRESS, false)?;
    load_uninitialized_pda(
        pool_info,
        &[POOL, signer.key.as_ref()],
        args.pool_bump,
        &ore_api::id(),
    )?;
    load_system_account(pool_tokens_info, true)?;
    load_uninitialized_pda(
        proof_info,
        &[PROOF, pool_info.key.as_ref()],
        args.proof_bump,
        &ore_api::id(),
    )?;
    load_program(system_program, system_program::id())?;
    load_program(token_program, spl_token::id())?;
    load_program(associated_token_program, spl_associated_token_account::id())?;
    load_sysvar(slot_hashes_info, sysvar::slot_hashes::id())?;

    // Initialize the stake account.
    create_pda(
        pool_info,
        &ore_stake_api::id(),
        8 + size_of::<Pool>(),
        &[POOL, signer.key.as_ref(), &[args.pool_bump]],
        system_program,
        signer,
    )?;
    let mut pool_data = pool_info.data.borrow_mut();
    pool_data[0] = Pool::discriminator() as u8;
    let stake = Pool::try_from_bytes_mut(&mut pool_data)?;
    stake.authority = *signer.key;
    stake.bump = args.pool_bump as u64;
    stake.is_open = 0;
    drop(pool_data);

    // Initialize a token account to escrow stake.
    solana_program::program::invoke(
        &spl_associated_token_account::instruction::create_associated_token_account(
            signer.key,
            pool_info.key,
            mint_info.key,
            &spl_token::id(),
        ),
        &[
            associated_token_program.clone(),
            signer.clone(),
            pool_tokens_info.clone(),
            pool_info.clone(),
            mint_info.clone(),
            system_program.clone(),
            token_program.clone(),
        ],
    )?;

    // Open a proof account for mining.
    solana_program::program::invoke_signed(
        &ore_api::instruction::open(*pool_info.key, *miner_info.key),
        &[
            pool_info.clone(),
            miner_info.clone(),
            proof_info.clone(),
            system_program.clone(),
            slot_hashes_info.clone(),
        ],
        &[&[POOL, signer.key.as_ref(), &[args.pool_bump]]],
    )?;

    Ok(())
}

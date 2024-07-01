use std::mem::size_of;

use ore_api::consts::PROOF;
use ore_relay_api::{consts::*, instruction::OpenArgs, loaders::*, state::Relay};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    system_program, sysvar,
};
use utils::{create_pda, AccountDeserialize, Discriminator};

/// Opens a new relay account.
pub fn process_open<'a, 'info>(accounts: &'a [AccountInfo<'info>], data: &[u8]) -> ProgramResult {
    // Parse args
    let args = OpenArgs::try_from_bytes(data)?;

    // Load accounts.
    let [signer, miner_info, proof_info, relay_info, ore_program, system_program, slot_hashes_sysvar] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_uninitialized_pda(
        proof_info,
        &[PROOF, relay_info.key.as_ref()],
        args.proof_bump,
        &ore_api::id(),
    )?;
    load_system_account(miner_info, false)?;
    load_uninitialized_pda(
        relay_info,
        &[RELAY, signer.key.as_ref()],
        args.relay_bump,
        &ore_relay_api::id(),
    )?;
    load_program(ore_program, ore_api::id())?;
    load_program(system_program, system_program::id())?;
    load_sysvar(slot_hashes_sysvar, sysvar::slot_hashes::id())?;

    // Initialize relay account.
    create_pda(
        relay_info,
        &ore_relay_api::id(),
        8 + size_of::<Relay>(),
        &[RELAY, proof_info.key.as_ref(), &[args.relay_bump]],
        system_program,
        signer,
    )?;
    let mut relay_data = relay_info.data.borrow_mut();
    relay_data[0] = Relay::discriminator() as u8;
    let relay = Relay::try_from_bytes_mut(&mut relay_data)?;
    relay.authority = *signer.key;
    relay.bump = args.relay_bump as u64;
    relay.proof = *proof_info.key;

    // Open a proof account for mining.
    drop(relay_data);
    solana_program::program::invoke_signed(
        &ore_api::instruction::open(*relay_info.key, *miner_info.key),
        &[
            relay_info.clone(),
            miner_info.clone(),
            proof_info.clone(),
            system_program.clone(),
            slot_hashes_sysvar.clone(),
        ],
        &[&[RELAY, signer.key.as_ref(), &[args.relay_bump]]],
    )?;

    Ok(())
}

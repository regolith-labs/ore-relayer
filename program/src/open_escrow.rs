use std::mem::size_of;

use ore_api::consts::PROOF;
use ore_relay_api::{consts::*, instruction::OpenEscrowArgs, loaders::*};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    system_program, sysvar,
};
use utils::{create_pda, AccountDeserialize, Discriminator};

/// Opens a new escrow account.
pub fn process_open_escrow<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    data: &[u8],
) -> ProgramResult {
    // Parse args
    let args = OpenEscrowArgs::try_from_bytes(data)?;

    // Load accounts.
    let [signer, escrow_info, escrow_tokens, miner_info, proof_info, relayer_info, ore_program, system_program, slot_hashes_sysvar] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_uninitialized_pda(
        proof_info,
        &[PROOF, escrow_info.key.as_ref()],
        args.proof_bump,
        &ore_api::id(),
    )?;
    load_system_account(escrow_tokens, true)?;
    load_system_account(miner_info, false)?;
    load_uninitialized_pda(
        escrow_info,
        &[ESCROW, signer.key.as_ref(), relayer_info.key.as_ref()],
        args.escrow_bump,
        &ore_relay_api::id(),
    )?;
    load_any_relayer(relayer_info, true)?;
    load_program(ore_program, ore_api::id())?;
    load_program(system_program, system_program::id())?;
    load_sysvar(slot_hashes_sysvar, sysvar::slot_hashes::id())?;

    // Initialize escrow account.
    create_pda(
        escrow_info,
        &ore_relay_api::id(),
        8 + size_of::<Escrow>(),
        &[
            ESCROW,
            signer.key.as_ref(),
            relayer_info.key.as_ref(),
            &[args.escrow_bump],
        ],
        system_program,
        signer,
    )?;
    let mut escrow_data = escrow_info.data.borrow_mut();
    escrow_data[0] = Escrow::discriminator() as u8;
    let escrow = Escrow::try_from_bytes_mut(&mut escrow_data)?;
    escrow.authority = *signer.key;
    escrow.bump = args.escrow_bump as u64;
    escrow.relayer = *relayer_info.key;

    // TODO Initialize escrow tokens account

    // Open a proof account for mining.
    drop(escrow_data);
    solana_program::program::invoke_signed(
        &ore_api::instruction::open(*escrow_info.key, *miner_info.key),
        &[
            escrow_info.clone(),
            miner_info.clone(),
            proof_info.clone(),
            system_program.clone(),
            slot_hashes_sysvar.clone(),
        ],
        &[&[
            ESCROW,
            signer.key.as_ref(),
            relayer_info.key.as_ref(),
            &[args.escrow_bump],
        ]],
    )?;

    Ok(())
}

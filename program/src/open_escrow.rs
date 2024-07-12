use std::mem::size_of;

use ore_api::{
    consts::{MINT_ADDRESS, PROOF},
    state::Proof,
};
use ore_relay_api::{consts::*, error::RelayError, instruction::OpenEscrowArgs, loaders::*};
use ore_utils::{create_pda, spl::create_ata, AccountDeserialize, Discriminator};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    system_program, sysvar,
};

/// Opens a new escrow account.
pub fn process_open_escrow<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    data: &[u8],
) -> ProgramResult {
    // Parse args
    let args = OpenEscrowArgs::try_from_bytes(data)?;

    // Load accounts
    let [signer, miner_info, escrow_info, escrow_tokens, mint_info, proof_info, relayer_info, ore_program, system_program, token_program, associated_token_program, slot_hashes_sysvar] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_signer(miner_info)?;
    load_uninitialized_pda(
        proof_info,
        &[PROOF, escrow_info.key.as_ref()],
        args.proof_bump,
        &ore_api::id(),
    )?;
    load_system_account(escrow_tokens, true)?;
    load_mint(mint_info, MINT_ADDRESS, false)?;
    load_uninitialized_pda(
        escrow_info,
        &[ESCROW, signer.key.as_ref(), relayer_info.key.as_ref()],
        args.escrow_bump,
        &ore_relay_api::id(),
    )?;
    load_relayer(relayer_info, &AUTHORIZED_RELAYER, true)?;
    load_program(ore_program, ore_api::id())?;
    load_program(token_program, spl_token::id())?;
    load_program(associated_token_program, spl_associated_token_account::id())?;
    load_program(system_program, system_program::id())?;
    load_sysvar(slot_hashes_sysvar, sysvar::slot_hashes::id())?;

    // Load the relay account
    let relayer_data = relayer_info.data.borrow();
    let relayer = Relayer::try_from_bytes(&relayer_data)?;

    // validate miner against relayer
    if !miner_info.key.eq(&relayer.miner) {
        return Err(RelayError::Dummy.into());
    }

    // Create escrow account
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

    //// Open a proof account for mining
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

    // Load the proof account
    let proof_data = proof_info.data.borrow();
    let proof = Proof::try_from_bytes(&proof_data)?;

    // Initialize escrow account
    let mut escrow_data = escrow_info.data.borrow_mut();
    escrow_data[0] = Escrow::discriminator() as u8;
    let escrow = Escrow::try_from_bytes_mut(&mut escrow_data)?;
    escrow.authority = *signer.key;
    escrow.bump = args.escrow_bump as u64;
    escrow.last_hash = proof.last_hash;
    escrow.relayer = *relayer_info.key;

    // // Initialize escrow tokens account
    // create_ata(
    //     signer,
    //     escrow_info,
    //     escrow_tokens,
    //     mint_info,
    //     system_program,
    //     token_program,
    //     associated_token_program,
    // )?;

    Ok(())
}

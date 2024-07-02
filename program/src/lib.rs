mod claim;
mod collect;
mod open_escrow;
mod open_relayer;
mod stake;
mod update_miner;

use claim::*;
use collect::*;
use open_escrow::*;
use open_relayer::*;
use stake::*;
use update_miner::*;

use ore_relay_api::instruction::*;
use solana_program::{
    self, account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if program_id.ne(&ore_relay_api::id()) {
        return Err(ProgramError::IncorrectProgramId);
    }

    let (tag, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match RelayInstruction::try_from(*tag).or(Err(ProgramError::InvalidInstructionData))? {
        // User ixs
        RelayInstruction::Claim => process_claim(accounts, data)?,
        RelayInstruction::Stake => process_stake(accounts, data)?,
        RelayInstruction::OpenEscrow => process_open_escrow(accounts, data)?,

        // Relayer ixs
        RelayInstruction::OpenRelayer => process_open_relayer(accounts, data)?,
        RelayInstruction::Collect => process_collect(accounts, data)?,
        RelayInstruction::UpdateMiner => process_update_miner(accounts, data)?,

        _ => {}
    }

    Ok(())
}

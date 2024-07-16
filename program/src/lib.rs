mod claim;
mod close_escrow;
mod collect;
mod open_escrow;
mod stake;
mod update_miner;

use claim::*;
use close_escrow::*;
use collect::*;
use open_escrow::*;
use stake::*;
use update_miner::*;

use ore_relayer_api::instruction::*;
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
    if program_id.ne(&ore_relayer_api::id()) {
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
        RelayInstruction::CloseEscrow => process_close_escrow(accounts, data)?,

        // Relayer ixs
        RelayInstruction::Collect => process_collect(accounts, data)?,
        RelayInstruction::UpdateMiner => process_update_miner(accounts, data)?,
    }

    Ok(())
}

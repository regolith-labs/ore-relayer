mod collect;
mod open_escrow;
mod open_relayer;

use collect::*;
use open_escrow::*;
use open_relayer::*;

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
        RelayInstruction::Collect => process_collect(accounts, data)?,
        RelayInstruction::OpenEscrow => process_open_escrow(accounts, data)?,
        RelayInstruction::OpenRelayer => process_open_relayer(accounts, data)?,
        _ => {}
    }

    Ok(())
}

mod delegate;
mod initialize;
mod open;
mod withdraw;

use delegate::*;
use initialize::*;
use open::*;
use withdraw::*;

use ore_stake_api::instruction::*;
use solana_program::{
    self, account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

pub(crate) use utils;

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if program_id.ne(&ore_stake_api::id()) {
        return Err(ProgramError::IncorrectProgramId);
    }

    let (tag, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match StakeInstruction::try_from(*tag).or(Err(ProgramError::InvalidInstructionData))? {
        StakeInstruction::Initialize => process_initialize(program_id, accounts, data)?,
        StakeInstruction::Open => process_open(program_id, accounts, data)?,
        StakeInstruction::Delegate => process_delegate(program_id, accounts, data)?,
        StakeInstruction::Withdraw => process_withdraw(program_id, accounts, data)?,
    }

    Ok(())
}

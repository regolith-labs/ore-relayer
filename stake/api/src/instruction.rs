use bytemuck::{Pod, Zeroable};
use num_enum::TryFromPrimitive;
use ore_api::consts::*;
use shank::ShankInstruction;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program, sysvar,
};

use crate::{
    consts::*,
    utils::{impl_instruction_from_bytes, impl_to_bytes},
};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, ShankInstruction, TryFromPrimitive)]
#[rustfmt::skip]
pub enum StakeInstruction {
    #[account(0, name = "stake_program", desc = "ORE stake program")]
    #[account(1, name = "signer", desc = "Signer", signer)]
    #[account(2, name = "miner", desc = "Miner authority")]
    #[account(3, name = "proof", desc = "ORE proof account", writable)]
    #[account(4, name = "stake", desc = "ORE stake account", writable)]
    #[account(5, name = "system_program", desc = "Solana system program")]
    #[account(6, name = "slot_hashes", desc = "Solana slot hashes sysvar")]
    Open = 0,
}

impl StakeInstruction {
    pub fn to_vec(&self) -> Vec<u8> {
        vec![*self as u8]
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct OpenArgs {
    pub proof_bump: u8,
    pub stake_bump: u8,
}

impl_to_bytes!(OpenArgs);

impl_instruction_from_bytes!(OpenArgs);

/// Builds an open instruction.
pub fn open(signer: Pubkey) -> Instruction {
    let stake_pda = Pubkey::find_program_address(&[STAKE, signer.as_ref()], &crate::id());
    let proof_pda = Pubkey::find_program_address(&[PROOF, stake_pda.0.as_ref()], &ore_api::id());
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(stake_pda.0, false),
            AccountMeta::new(proof_pda.0, false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(sysvar::slot_hashes::id(), false),
        ],
        data: [
            StakeInstruction::Open.to_vec(),
            OpenArgs {
                proof_bump: proof_pda.1,
                stake_bump: stake_pda.1,
            }
            .to_bytes()
            .to_vec(),
        ]
        .concat(),
    }
}

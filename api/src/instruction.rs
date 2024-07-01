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
pub enum RelayInstruction {
    #[account(0, name = "relay_program", desc = "Relay program")]
    #[account(1, name = "signer", desc = "Signer", signer)]
    #[account(2, name = "miner", desc = "Mining authority", writable)]
    #[account(3, name = "proof", desc = "ORE proof account", writable)]
    #[account(4, name = "relay", desc = "Relay account", writable)]
    #[account(5, name = "ore_program", desc = "ORE program")]
    #[account(6, name = "system_program", desc = "Solana system program")]
    #[account(7, name = "slot_hash_sysvar", desc = "Solana slot hash sysvar")]
    Open,

    // TODO Claim
    // TODO Close
    // TODO Collect
    // TODO Open
    // TODO Stake
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct OpenArgs {
    pub proof_bump: u8,
    pub relay_bump: u8,
}

impl RelayInstruction {
    pub fn to_vec(&self) -> Vec<u8> {
        vec![*self as u8]
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct WithdrawArgs {
    pub amount: u64,
}

impl_to_bytes!(OpenArgs);

impl_instruction_from_bytes!(OpenArgs);

// Builds an initialize instruction.
pub fn open(signer: Pubkey, miner: Pubkey) -> Instruction {
    let relay_pda = Pubkey::find_program_address(&[RELAY, signer.as_ref()], &crate::id());
    let proof_pda = Pubkey::find_program_address(&[PROOF, relay_pda.0.as_ref()], &ore_api::id());
    // let pool_tokens_address =
    //     spl_associated_token_account::get_associated_token_address(&pool_pda.0, &MINT_ADDRESS);
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(miner, false),
            AccountMeta::new(proof_pda.0, false),
            AccountMeta::new(relay_pda.0, false),
            AccountMeta::new_readonly(ore_api::id(), false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(sysvar::slot_hashes::id(), false),
        ],
        data: [
            RelayInstruction::Open.to_vec(),
            OpenArgs {
                relay_bump: relay_pda.1,
                proof_bump: proof_pda.1,
            }
            .to_bytes()
            .to_vec(),
        ]
        .concat(),
    }
}

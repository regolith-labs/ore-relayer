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
    #[account(5, name = "stake_tokens", desc = "ORE stake escrow account", writable)]
    #[account(6, name = "system_program", desc = "Solana system program")]
    #[account(7, name = "token_program", desc = "SPL token program")]
    #[account(8, name = "associated_token_program", desc = "SPL associated token program")]
    #[account(9, name = "slot_hashes", desc = "Solana slot hashes sysvar")]
    Initialize = 0,

    #[account(0, name = "stake_program", desc = "ORE stake program")]
    #[account(1, name = "signer", desc = "Signer", signer)]
    #[account(2, name = "delegate", desc = "ORE stake delegate account", writable)]
    #[account(3, name = "stake", desc = "ORE stake account")]
    #[account(4, name = "system_program", desc = "Solana system program")]
    Open = 1,

    #[account(0, name = "stake_program", desc = "ORE stake program")]
    #[account(1, name = "signer", desc = "Signer", signer)]
    #[account(2, name = "delegate", desc = "ORE stake delegate account", writable)]
    #[account(3, name = "proof", desc = "ORE proof account", writable)]
    #[account(4, name = "sender", desc = "Signer token account", writable)]
    #[account(5, name = "stake", desc = "ORE stake account", writable)]
    #[account(6, name = "stake_tokens", desc = "ORE stake escrow account", writable)]
    #[account(7, name = "treasury_tokens", desc = "ORE treasury token account", writable)]
    #[account(8, name = "token_program", desc = "SPL token program")]
    Delegate = 2,

    #[account(0, name = "stake_program", desc = "ORE stake program")]
    #[account(1, name = "signer", desc = "Signer", signer)]
    #[account(2, name = "delegate", desc = "ORE stake delegate account", writable)]
    #[account(3, name = "proof", desc = "ORE proof account", writable)]
    #[account(4, name = "beneficiary", desc = "Beneficiary token account", writable)]
    #[account(5, name = "stake", desc = "ORE stake account", writable)]
    #[account(6, name = "stake_tokens", desc = "ORE stake escrow account", writable)]
    #[account(7, name = "treasury_tokens", desc = "ORE treasury token account", writable)]
    #[account(8, name = "token_program", desc = "SPL token program")]
    Withdraw = 3,

    #[account(0, name = "stake_program", desc = "ORE stake program")]
    #[account(1, name = "signer", desc = "Signer", signer)]
    #[account(2, name = "delegate", desc = "ORE stake delegate account", writable)]
    #[account(3, name = "stake", desc = "ORE stake account")]
    #[account(4, name = "system_program", desc = "Solana system program")]
    Close = 4,

    // TODO Update stake account
}

impl StakeInstruction {
    pub fn to_vec(&self) -> Vec<u8> {
        vec![*self as u8]
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct InitializeArgs {
    pub proof_bump: u8,
    pub stake_bump: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct OpenArgs {
    pub bump: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct DelegateArgs {
    pub amount: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct WithdrawArgs {
    pub amount: u64,
}

impl_to_bytes!(OpenArgs);
impl_to_bytes!(DelegateArgs);
impl_to_bytes!(InitializeArgs);
impl_to_bytes!(WithdrawArgs);

impl_instruction_from_bytes!(OpenArgs);
impl_instruction_from_bytes!(DelegateArgs);
impl_instruction_from_bytes!(InitializeArgs);
impl_instruction_from_bytes!(WithdrawArgs);

/// Builds an initialize instruction.
pub fn initialize(signer: Pubkey) -> Instruction {
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
            InitializeArgs {
                proof_bump: proof_pda.1,
                stake_bump: stake_pda.1,
            }
            .to_bytes()
            .to_vec(),
        ]
        .concat(),
    }
}

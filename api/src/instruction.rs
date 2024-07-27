use bytemuck::{Pod, Zeroable};
use num_enum::TryFromPrimitive;
use ore_api::consts::*;
use ore_utils::{impl_instruction_from_bytes, impl_to_bytes};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program, sysvar,
};

use crate::consts::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq,  TryFromPrimitive)]
#[rustfmt::skip]
pub enum RelayInstruction {
    Claim = 0, 
    CloseEscrow = 1,
    OpenEscrow = 2,
    Stake = 3,

    Collect = 101, 
    UpdateMiner = 102, 
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ClaimArgs {
    pub amount: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CollectArgs {
    pub fee: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct OpenEscrowArgs {
    pub escrow_bump: u8,
    pub proof_bump: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct StakeArgs {
    pub amount: [u8; 8],
}

impl RelayInstruction {
    pub fn to_vec(&self) -> Vec<u8> {
        vec![*self as u8]
    }
}

impl_to_bytes!(ClaimArgs);
impl_to_bytes!(CollectArgs);
impl_to_bytes!(OpenEscrowArgs);
impl_to_bytes!(StakeArgs);

impl_instruction_from_bytes!(ClaimArgs);
impl_instruction_from_bytes!(CollectArgs);
impl_instruction_from_bytes!(OpenEscrowArgs);
impl_instruction_from_bytes!(StakeArgs);

// Builds a collect instruction.
pub fn collect(
    signer: Pubkey,
    escrow_authority: Pubkey,
    beneficiary: Pubkey,
    sol_fee: u64,
) -> Instruction {
    let (escrow_pda, _) =
        Pubkey::find_program_address(&[ESCROW, escrow_authority.as_ref()], &crate::id());
    let (proof_pda, _) =
        Pubkey::find_program_address(&[PROOF, escrow_pda.as_ref()], &ore_api::id());
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(beneficiary, false),
            AccountMeta::new(escrow_pda, false),
            AccountMeta::new(proof_pda, false),
            AccountMeta::new_readonly(ore_api::consts::TREASURY_ADDRESS, false),
            AccountMeta::new(ore_api::consts::TREASURY_TOKENS_ADDRESS, false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(ore_api::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: vec![
            RelayInstruction::Collect.to_vec(),
            CollectArgs {
                fee: sol_fee.to_le_bytes(),
            }
            .to_bytes()
            .to_vec(),
        ]
        .concat(),
    }
}

// Builds a claim instruction.
pub fn claim(signer: Pubkey, beneficiary: Pubkey, amount: u64) -> Instruction {
    let (escrow_pda, _) = Pubkey::find_program_address(&[ESCROW, signer.as_ref()], &crate::id());
    let (proof_pda, _) =
        Pubkey::find_program_address(&[PROOF, escrow_pda.as_ref()], &ore_api::id());
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(beneficiary, false),
            AccountMeta::new(escrow_pda, false),
            AccountMeta::new(proof_pda, false),
            AccountMeta::new_readonly(ore_api::consts::TREASURY_ADDRESS, false),
            AccountMeta::new(ore_api::consts::TREASURY_TOKENS_ADDRESS, false),
            AccountMeta::new_readonly(ore_api::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: [
            RelayInstruction::Claim.to_vec(),
            ClaimArgs {
                amount: amount.to_le_bytes(),
            }
            .to_bytes()
            .to_vec(),
        ]
        .concat(),
    }
}

// Builds a stake instruction.
pub fn stake(signer: Pubkey, sender: Pubkey, amount: u64) -> Instruction {
    let (escrow_pda, _) = Pubkey::find_program_address(&[ESCROW, signer.as_ref()], &crate::id());
    let escrow_tokens =
        spl_associated_token_account::get_associated_token_address(&escrow_pda, &MINT_ADDRESS);
    let (proof_pda, _) =
        Pubkey::find_program_address(&[PROOF, escrow_pda.as_ref()], &ore_api::id());
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(escrow_pda, false),
            AccountMeta::new(escrow_tokens, false),
            AccountMeta::new(proof_pda, false),
            AccountMeta::new(sender, false),
            AccountMeta::new(ore_api::consts::TREASURY_TOKENS_ADDRESS, false),
            AccountMeta::new_readonly(ore_api::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: [
            RelayInstruction::Stake.to_vec(),
            StakeArgs {
                amount: amount.to_le_bytes(),
            }
            .to_bytes()
            .to_vec(),
        ]
        .concat(),
    }
}

// Builds an open_escrow instruction.
pub fn open_escrow(signer: Pubkey, payer: Pubkey) -> Instruction {
    let escrow_pda = Pubkey::find_program_address(&[ESCROW, signer.as_ref()], &crate::id());
    let proof_pda = Pubkey::find_program_address(&[PROOF, escrow_pda.0.as_ref()], &ore_api::id());
    let escrow_tokens_address =
        spl_associated_token_account::get_associated_token_address(&escrow_pda.0, &MINT_ADDRESS);
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(MINER_PUBKEY, false),
            AccountMeta::new(payer, true),
            AccountMeta::new(escrow_pda.0, false),
            AccountMeta::new(escrow_tokens_address, false),
            AccountMeta::new_readonly(MINT_ADDRESS, false),
            AccountMeta::new(proof_pda.0, false),
            AccountMeta::new_readonly(ore_api::id(), false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(spl_associated_token_account::id(), false),
            AccountMeta::new_readonly(sysvar::slot_hashes::id(), false),
        ],
        data: [
            RelayInstruction::OpenEscrow.to_vec(),
            OpenEscrowArgs {
                escrow_bump: escrow_pda.1,
                proof_bump: proof_pda.1,
            }
            .to_bytes()
            .to_vec(),
        ]
        .concat(),
    }
}

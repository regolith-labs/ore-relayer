use bytemuck::{Pod, Zeroable};
use num_enum::TryFromPrimitive;
use ore_api::consts::*;
use ore_utils::{impl_instruction_from_bytes, impl_to_bytes};
use shank::ShankInstruction;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program, sysvar,
};

use crate::{consts::*, state::Escrow};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, ShankInstruction, TryFromPrimitive)]
#[rustfmt::skip]
pub enum RelayInstruction {
    #[account(0, name = "relay_program", desc = "Relay program")]
    #[account(1, name = "signer", desc = "Signer", signer)]
    #[account(2, name = "beneficiary", desc = "Beneficiary token account", writable)]
    #[account(3, name = "escrow", desc = "Escrow account", writable)]
    #[account(4, name = "proof", desc = "ORE proof account", writable)]
    #[account(5, name = "treasury", desc = "ORE treasury account", writable)]
    #[account(6, name = "treasury_tokens", desc = "ORE treasury token account", writable)]
    #[account(7, name = "ore_program", desc = "ORE program")]
    #[account(8, name = "token_program", desc = "SPL token program")]
    Claim = 0, 

    #[account(0, name = "relay_program", desc = "Relay program")]
    #[account(1, name = "signer", desc = "Signer", signer)]
    #[account(2, name = "proof", desc = "ORE proof account", writable)]
    #[account(3, name = "ore_program", desc = "ORE program")]
    #[account(4, name = "system_program", desc = "Solana system program")]
    CloseEscrow = 1,

    #[account(0, name = "relay_program", desc = "Relay program")]
    #[account(1, name = "signer", desc = "Signer", signer)]
    #[account(2, name = "escrow", desc = "Escrow account to be created", writable)]
    #[account(3, name = "escrow_tokens", desc = "Escrow token account to be created", writable)]
    #[account(4, name = "miner", desc = "Miner authority")]
    #[account(5, name = "proof", desc = "ORE proof account to be created", writable)]
    #[account(6, name = "ore_program", desc = "ORE program")]
    #[account(7, name = "system_program", desc = "Solana system program")]
    #[account(8, name = "slot_hash_sysvar", desc = "Solana slot hash sysvar")]
    OpenEscrow = 2,

    #[account(0, name = "relay_program", desc = "Relay program")]
    #[account(1, name = "signer", desc = "Signer", signer)]
    #[account(2, name = "escrow", desc = "Escrow account", writable)]
    #[account(3, name = "escrow_tokens", desc = "Escrow token account", writable)]
    #[account(4, name = "proof", desc = "ORE proof account", writable)]
    #[account(5, name = "treasury_tokens", desc = "ORE treasury token account", writable)]
    #[account(6, name = "ore_program", desc = "ORE program")]
    #[account(7, name = "token_program", desc = "SPL token program")]
    Stake = 3,

    #[account(0, name = "relay_program", desc = "Relay program")]
    #[account(1, name = "signer", desc = "Signer", signer)]
    #[account(2, name = "beneficiary", desc = "Beneficiary token account", writable)]
    #[account(3, name = "escrow", desc = "Escrow account", writable)]
    #[account(4, name = "proof", desc = "ORE proof account", writable)]
    #[account(5, name = "treasury", desc = "ORE treasury account", writable)]
    #[account(6, name = "treasury_tokens", desc = "ORE treasury token account", writable)]
    #[account(7, name = "ore_program", desc = "ORE program")]
    #[account(8, name = "token_program", desc = "SPL token program")]
    Collect = 101, 

    #[account(0, name = "relay_program", desc = "Relay program")]
    #[account(1, name = "signer", desc = "Signer", signer)]
    #[account(2, name = "miner", desc = "Miner authority")]
    #[account(3, name = "proof", desc = "ORE proof account", writable)]
    #[account(4, name = "ore_program", desc = "ORE program")]
    UpdateMiner = 102, 
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ClaimArgs {
    pub amount: [u8; 8],
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
impl_to_bytes!(OpenEscrowArgs);
impl_to_bytes!(StakeArgs);

impl_instruction_from_bytes!(ClaimArgs);
impl_instruction_from_bytes!(OpenEscrowArgs);
impl_instruction_from_bytes!(StakeArgs);

// Builds a collect instruction.
pub fn collect(signer: Pubkey, escrow: Escrow, beneficiary: Pubkey) -> Instruction {
    let (escrow_pda, _) =
        Pubkey::find_program_address(&[ESCROW, escrow.authority.as_ref()], &crate::id());
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
        data: RelayInstruction::Collect.to_vec(),
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

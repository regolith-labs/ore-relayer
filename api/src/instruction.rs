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
    #[account(2, name = "beneficiary", desc = "Beneficiary token account", writable)]
    #[account(3, name = "escrow", desc = "Escrow account", writable)]
    #[account(4, name = "proof", desc = "ORE proof account", writable)]
    #[account(5, name = "treasury", desc = "ORE treasury account", writable)]
    #[account(6, name = "treasury_tokens", desc = "ORE treasury token account", writable)]
    #[account(7, name = "ore_program", desc = "ORE program")]
    #[account(8, name = "token_program", desc = "SPL token program")]
    Claim = 0, 

    // TODO CloseEscrow
    CloseEscrow = 1,

    #[account(0, name = "relay_program", desc = "Relay program")]
    #[account(1, name = "signer", desc = "Signer", signer)]
    #[account(2, name = "escrow", desc = "Escrow account to be created", writable)]
    #[account(3, name = "escrow_tokens", desc = "Escrow token account to be created", writable)]
    #[account(4, name = "miner", desc = "Miner authority")]
    #[account(5, name = "proof", desc = "ORE proof account to be created", writable)]
    #[account(6, name = "relayer", desc = "Relayer account", writable)]
    #[account(7, name = "ore_program", desc = "ORE program")]
    #[account(8, name = "system_program", desc = "Solana system program")]
    #[account(9, name = "slot_hash_sysvar", desc = "Solana slot hash sysvar")]
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
    #[account(2, name = "relayer", desc = "Relayer account to be created", writable)]
    #[account(3, name = "system_program", desc = "Solana system program")]
    OpenRelayer = 100,

    #[account(0, name = "relay_program", desc = "Relay program")]
    #[account(1, name = "signer", desc = "Signer", signer)]
    #[account(2, name = "beneficiary", desc = "Beneficiary token account", writable)]
    #[account(3, name = "escrow", desc = "Escrow account", writable)]
    #[account(4, name = "proof", desc = "ORE proof account", writable)]
    #[account(5, name = "relayer", desc = "Relayer account", writable)]
    #[account(6, name = "treasury", desc = "ORE treasury account", writable)]
    #[account(7, name = "treasury_tokens", desc = "ORE treasury token account", writable)]
    #[account(8, name = "ore_program", desc = "ORE program")]
    #[account(9, name = "token_program", desc = "SPL token program")]
    Collect = 101, 

    #[account(0, name = "relay_program", desc = "Relay program")]
    #[account(1, name = "signer", desc = "Signer", signer)]
    #[account(2, name = "miner", desc = "Miner authority")]
    #[account(3, name = "proof", desc = "ORE proof account", writable)]
    #[account(4, name = "relayer", desc = "Relayer account")]
    #[account(5, name = "ore_program", desc = "ORE program")]
    UpdateMiner = 102, 

    // TODO UpdateRelayer
    UpdateRelayer = 103,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ClaimArgs {
    pub amount: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct OpenEscrowArgs {
    pub escrow_bump: u8,
    pub proof_bump: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct OpenRelayerArgs {
    pub bump: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct StakeArgs {
    pub amount: u64,
}

impl RelayInstruction {
    pub fn to_vec(&self) -> Vec<u8> {
        vec![*self as u8]
    }
}

impl_to_bytes!(ClaimArgs);
impl_to_bytes!(OpenEscrowArgs);
impl_to_bytes!(OpenRelayerArgs);
impl_to_bytes!(StakeArgs);

impl_instruction_from_bytes!(ClaimArgs);
impl_instruction_from_bytes!(OpenEscrowArgs);
impl_instruction_from_bytes!(OpenRelayerArgs);
impl_instruction_from_bytes!(StakeArgs);

// Builds an open_escrow instruction.
pub fn open_escrow(signer: Pubkey, miner: Pubkey, relayer: Pubkey) -> Instruction {
    let escrow_pda =
        Pubkey::find_program_address(&[ESCROW, signer.as_ref(), relayer.as_ref()], &crate::id());
    let proof_pda = Pubkey::find_program_address(&[PROOF, escrow_pda.0.as_ref()], &ore_api::id());
    let escrow_tokens_address =
        spl_associated_token_account::get_associated_token_address(&escrow_pda.0, &MINT_ADDRESS);
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(escrow_pda.0, false),
            AccountMeta::new(escrow_tokens_address, false),
            AccountMeta::new_readonly(miner, false),
            AccountMeta::new(proof_pda.0, false),
            AccountMeta::new(relayer, false),
            AccountMeta::new_readonly(ore_api::id(), false),
            AccountMeta::new_readonly(system_program::id(), false),
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

// Builds an open_relayer instruction.
pub fn open_relayer(signer: Pubkey) -> Instruction {
    let relayer_pda = Pubkey::find_program_address(&[RELAYER, signer.as_ref()], &crate::id());
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(relayer_pda.0, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: [
            RelayInstruction::OpenRelayer.to_vec(),
            OpenRelayerArgs {
                bump: relayer_pda.1,
            }
            .to_bytes()
            .to_vec(),
        ]
        .concat(),
    }
}

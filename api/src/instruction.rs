use bytemuck::{Pod, Zeroable};
use num_enum::TryFromPrimitive;
use shank::ShankInstruction;

use crate::utils::{impl_instruction_from_bytes, impl_to_bytes};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, ShankInstruction, TryFromPrimitive)]
#[rustfmt::skip]
pub enum CheckoresInstruction {
    NewGame =  0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct NewGameArgs {
    pub bump: u8,
}

impl CheckoresInstruction {
    pub fn to_vec(&self) -> Vec<u8> {
        vec![*self as u8]
    }
}

impl_to_bytes!(NewGameArgs);

impl_instruction_from_bytes!(NewGameArgs);

// Builds an open_escrow instruction.
// pub fn new_game(signer: Pubkey) -> Instruction {
//     let escrow_pda =
//         Pubkey::find_program_address(&[ESCROW, signer.as_ref(), relayer.as_ref()], &crate::id());
//     let proof_pda = Pubkey::find_program_address(&[PROOF, escrow_pda.0.as_ref()], &ore_api::id());
//     let escrow_tokens_address =
//         spl_associated_token_account::get_associated_token_address(&escrow_pda.0, &MINT_ADDRESS);
//     Instruction {
//         program_id: crate::id(),
//         accounts: vec![
//             AccountMeta::new(signer, true),
//             AccountMeta::new(escrow_pda.0, false),
//             AccountMeta::new(escrow_tokens_address, false),
//             AccountMeta::new_readonly(miner, false),
//             AccountMeta::new(proof_pda.0, false),
//             AccountMeta::new(relayer, false),
//             AccountMeta::new_readonly(ore_api::id(), false),
//             AccountMeta::new_readonly(system_program::id(), false),
//             AccountMeta::new_readonly(sysvar::slot_hashes::id(), false),
//         ],
//         data: [
//             CheckoresInstruction::OpenEscrow.to_vec(),
//             OpenEscrowArgs {
//                 escrow_bump: escrow_pda.1,
//                 proof_bump: proof_pda.1,
//             }
//             .to_bytes()
//             .to_vec(),
//         ]
//         .concat(),
//     }
// }

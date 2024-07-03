use std::mem::size_of;

use checkores_api::{consts::*, instruction::NewGameArgs, loaders::*, state::Board};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    system_program,
};
use utils::{create_pda, AccountDeserialize, Discriminator};

/// Starts a new game.
pub fn process_new_game<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    data: &[u8],
) -> ProgramResult {
    // Parse args
    let args = NewGameArgs::try_from_bytes(data)?;

    // Load accounts.
    let [signer, board_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_uninitialized_pda(
        board_info,
        &[BOARD, signer.key.as_ref()],
        args.bump,
        &checkores_api::id(),
    )?;
    load_program(system_program, system_program::id())?;

    // Initialize board account.
    create_pda(
        board_info,
        &checkores_api::id(),
        8 + size_of::<Board>(),
        &[BOARD, &[args.bump]],
        system_program,
        signer,
    )?;
    let mut board_data = board_info.data.borrow_mut();
    board_data[0] = Board::discriminator() as u8;
    let board = Board::try_from_bytes_mut(&mut board_data)?;
    board.bump = args.bump as u64;
    board.state = [0; 1024];

    Ok(())
}

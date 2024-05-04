use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::instruction::MplBufferInstruction;
pub mod allocate;
pub mod clear_data;
pub mod close;
pub mod create;
pub mod write_data;
pub(crate) use allocate::process_allocate;
pub(crate) use clear_data::process_clear_data;
pub(crate) use close::process_close;
pub(crate) use create::process_create;
pub(crate) use write_data::process_write_data;

pub(crate) fn process_instruction<'a>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction: MplBufferInstruction = MplBufferInstruction::try_from_slice(instruction_data)?;
    match instruction {
        MplBufferInstruction::Create => {
            msg!("Instruction: Create");
            process_create(accounts)
        }
        MplBufferInstruction::Close => {
            msg!("Instruction: Close");
            process_close(accounts)
        }
        MplBufferInstruction::Allocate(args) => {
            msg!("Instruction: Allocate");
            process_allocate(accounts, args)
        }
        MplBufferInstruction::WriteData(args) => {
            msg!("Instruction: WriteData");
            process_write_data(accounts, args)
        }
        MplBufferInstruction::ClearData => {
            msg!("Instruction: ClearData");
            process_clear_data(accounts)
        }
    }
}

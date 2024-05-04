use borsh::BorshSerialize;
use mpl_utils::{assert_derivation, assert_signer, create_or_allocate_account_raw};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke,
    program_memory::sol_memcpy, rent::Rent, system_instruction, system_program, sysvar::Sysvar,
};

use crate::{
    error::MplBufferError,
    instruction::accounts::CreateAccounts,
    state::{BufferMetadata, Key, PREFIX},
};

pub(crate) fn process_create<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    let ctx = &CreateAccounts::context(accounts)?;

    // Check that the account isn't already initialized.
    if (ctx.accounts.buffer.owner != &system_program::ID) || !ctx.accounts.buffer.data_is_empty() {
        return Err(MplBufferError::AlreadyInitialized.into());
    }

    // Check that the account isn't already initialized.
    if (ctx.accounts.buffer_metadata.owner != &system_program::ID)
        || !ctx.accounts.buffer_metadata.data_is_empty()
    {
        return Err(MplBufferError::AlreadyInitialized.into());
    }
    // Verify that the derived address is correct for the metadata account.
    let bump = assert_derivation(
        &crate::ID,
        ctx.accounts.buffer_metadata,
        &[
            PREFIX.as_bytes(),
            crate::ID.as_ref(),
            ctx.accounts.buffer.key.as_ref(),
        ],
        MplBufferError::DerivedKeyInvalid,
    )?;

    // The payer must sign as well as the authority, if present.
    let authority = match ctx.accounts.authority {
        Some(authority) => {
            assert_signer(authority)?;
            authority
        }
        None => ctx.accounts.payer,
    };
    assert_signer(ctx.accounts.payer)?;

    if ctx.accounts.system_program.key != &system_program::ID {
        return Err(MplBufferError::InvalidSystemProgram.into());
    }

    // Initialize the buffer account.
    let rent = Rent::get()?;
    let rent_amount = rent.minimum_balance(0);
    invoke(
        &system_instruction::create_account(
            ctx.accounts.payer.key,
            ctx.accounts.buffer.key,
            rent_amount,
            0,
            &crate::ID,
        ),
        &[
            ctx.accounts.payer.clone(),
            ctx.accounts.buffer.clone(),
            ctx.accounts.system_program.clone(),
        ],
    )?;

    // Initialize the buffer metadata.
    let buffer_metadata = BufferMetadata {
        key: Key::BufferMetadata,
        buffer: *ctx.accounts.buffer.key,
        bump,
        authority: *authority.key,
    };

    let serialized_metadata = &buffer_metadata.try_to_vec()?;

    // Initialize the buffer metadata account.
    create_or_allocate_account_raw(
        crate::ID,
        ctx.accounts.buffer_metadata,
        ctx.accounts.system_program,
        ctx.accounts.payer,
        serialized_metadata.len(),
        &[
            PREFIX.as_bytes(),
            crate::ID.as_ref(),
            ctx.accounts.buffer.key.as_ref(),
            &[bump],
        ],
    )?;

    // Write the buffer metadata to the metadata account.
    sol_memcpy(
        &mut ctx.accounts.buffer_metadata.try_borrow_mut_data()?,
        serialized_metadata,
        serialized_metadata.len(),
    );

    Ok(())
}

use borsh::BorshDeserialize;
use mpl_utils::{assert_derivation, assert_signer, resize_or_reallocate_account_raw};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_memory::sol_memcpy,
    system_program,
};

use crate::{
    error::MplBufferError,
    instruction::{accounts::WriteDataAccounts, WriteDataArgs},
    state::{BufferMetadata, PREFIX},
};

pub(crate) fn process_write_data<'a>(
    accounts: &'a [AccountInfo<'a>],
    args: WriteDataArgs,
) -> ProgramResult {
    let ctx = &mut WriteDataAccounts::context(accounts)?;

    // Check that the buffer account is already initialized.
    if ctx.accounts.buffer.owner != &crate::ID {
        return Err(MplBufferError::NotInitialized.into());
    }

    // Check that the metadata account is already initialized.
    if (ctx.accounts.buffer_metadata.owner != &crate::ID)
        || ctx.accounts.buffer_metadata.data_is_empty()
    {
        return Err(MplBufferError::NotInitialized.into());
    }
    let buffer_metadata =
        BufferMetadata::try_from_slice(&ctx.accounts.buffer_metadata.data.borrow())?;

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
    if bump != buffer_metadata.bump {
        return Err(MplBufferError::DerivedKeyInvalid.into());
    }

    // The payer must sign as well as the authority, if present.
    let authority = match ctx.accounts.authority {
        Some(authority) => {
            assert_signer(authority)?;
            authority
        }
        None => ctx.accounts.payer,
    };

    assert_signer(ctx.accounts.payer)?;

    if buffer_metadata.authority != *authority.key {
        return Err(MplBufferError::InvalidAuthority.into());
    }

    if ctx.accounts.system_program.key != &system_program::ID {
        return Err(MplBufferError::InvalidSystemProgram.into());
    }

    let old_size = ctx.accounts.buffer.data_len();
    let write_end = args
        .offset
        .checked_add(args.value.len())
        .ok_or(MplBufferError::NumericalOverflow)?;

    // Resize the account to fit the new data if necessary.
    if write_end > old_size {
        resize_or_reallocate_account_raw(
            ctx.accounts.buffer,
            ctx.accounts.payer,
            ctx.accounts.system_program,
            write_end,
        )?;
    }

    // Write the buffer metadata to the metadata account.
    sol_memcpy(
        &mut ctx.accounts.buffer.try_borrow_mut_data()?[args.offset..],
        &args.value,
        args.value.len(),
    );

    Ok(())
}

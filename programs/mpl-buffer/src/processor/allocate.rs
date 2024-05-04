use borsh::BorshDeserialize;
use mpl_utils::{assert_derivation, assert_signer, resize_or_reallocate_account_raw};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::{ProgramResult, MAX_PERMITTED_DATA_INCREASE},
    system_program,
};

use crate::{
    error::MplBufferError,
    instruction::{accounts::AllocateAccounts, AllocateArgs},
    state::{BufferMetadata, PREFIX},
};

pub(crate) fn process_allocate<'a>(
    accounts: &'a [AccountInfo<'a>],
    args: AllocateArgs,
) -> ProgramResult {
    let ctx = &mut AllocateAccounts::context(accounts)?;

    // Check that the buffer account is already initialized.
    if ctx.accounts.buffer.owner != &crate::ID {
        return Err(MplBufferError::NotInitialized.into());
    }

    // Check that the metadata account is already initialized.
    if ctx.accounts.buffer_metadata.owner != &crate::ID {
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

    let max_realloc_size = ctx
        .accounts
        .buffer
        .data_len()
        .checked_add(MAX_PERMITTED_DATA_INCREASE)
        .ok_or(MplBufferError::NumericalOverflow)?;

    let new_size = std::cmp::min(args.target_size, max_realloc_size);

    // Resize the account to fit the new authority.
    resize_or_reallocate_account_raw(
        ctx.accounts.buffer,
        ctx.accounts.payer,
        ctx.accounts.system_program,
        new_size,
    )?;

    Ok(())
}

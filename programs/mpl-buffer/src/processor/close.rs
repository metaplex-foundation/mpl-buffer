use borsh::BorshDeserialize;
use mpl_utils::{assert_derivation, assert_signer, close_account_raw};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, system_program};

use crate::{
    error::MplBufferError,
    instruction::accounts::CloseAccounts,
    state::{BufferMetadata, PREFIX},
};

pub(crate) fn process_close<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    let ctx = &CloseAccounts::context(accounts)?;

    // Check that the account is already initialized.
    if ctx.accounts.buffer.owner != &crate::ID {
        return Err(MplBufferError::NotInitialized.into());
    }

    // Check that the account is already initialized.
    if (ctx.accounts.buffer_metadata.owner != &crate::ID)
        || ctx.accounts.buffer_metadata.data_is_empty()
    {
        return Err(MplBufferError::NotInitialized.into());
    }

    let buffer_metadata =
        BufferMetadata::try_from_slice(&ctx.accounts.buffer_metadata.data.borrow())?;

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

    // Close both accounts
    close_account_raw(ctx.accounts.payer, ctx.accounts.buffer)?;
    close_account_raw(ctx.accounts.payer, ctx.accounts.buffer_metadata)?;

    Ok(())
}

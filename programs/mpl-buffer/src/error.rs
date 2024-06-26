use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Error, Clone, Debug, Eq, PartialEq, FromPrimitive)]
pub enum MplBufferError {
    /// 0 - Invalid System Program
    #[error("Invalid System Program")]
    InvalidSystemProgram,
    /// 1 - Error deserializing account
    #[error("Error deserializing account")]
    DeserializationError,
    /// 2 - Error serializing account
    #[error("Error serializing account")]
    SerializationError,
    /// 3 - The account passed in was already initialized.
    #[error("The account has already been initialized")]
    AlreadyInitialized,
    /// 4 - The key for the JSON metadata account is invalid.
    #[error("The key for the account is invalid.")]
    DerivedKeyInvalid,
    /// 5 - The account passed isn't initialized.
    #[error("The account has not yet been initialized")]
    NotInitialized,
    /// 6 - The payer does not have authority to perform this action.
    #[error("The payer does not have authority to perform this action.")]
    InvalidAuthority,
    /// 7 - Numerical Overflow
    #[error("Numerical Overflow")]
    NumericalOverflow,
}

impl PrintProgramError for MplBufferError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<MplBufferError> for ProgramError {
    fn from(e: MplBufferError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for MplBufferError {
    fn type_of() -> &'static str {
        "Mpl Buffer Error"
    }
}

use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankContext, ShankInstruction)]
#[rustfmt::skip]
pub enum MplBufferInstruction {
    /// Create an account to be used as a buffer.
    #[account(0, writable, signer, name="buffer", desc = "The account where data is stored.")]
    #[account(1, writable, name="buffer_metadata", desc = "The account to store the buffer account's metadata in.")]
    #[account(2, writable, signer, name="payer", desc = "The account paying for the storage fees.")]
    #[account(3, optional, signer, name="authority", desc="The authority of the buffer account.")]
    #[account(4, name="system_program", desc = "The system program")]
    Create,

    /// Close the Buffer and Metadata accounts.
    #[account(0, writable, name="buffer", desc = "The account where data is stored.")]
    #[account(1, writable, name="buffer_metadata", desc = "The account to store the buffer account's metadata in.")]
    #[account(2, writable, signer, name="payer", desc="The account that will pay for the rent.")]
    #[account(3, optional, signer, name="authority", desc="The authority of the buffer account.")]
    #[account(4, name="system_program", desc = "System program")]
    Close,

    /// Allocate additional space for the buffer account.
    #[account(0, writable, name="buffer", desc = "The account where data is stored.")]
    #[account(1, writable, name="buffer_metadata", desc = "The account to store the buffer account's metadata in.")]
    #[account(2, writable, signer, name="payer", desc="The account that will pay for the rent.")]
    #[account(3, optional, signer, name="authority", desc="The authority of the buffer account.")]
    #[account(4, name="system_program", desc = "System program")]
    Allocate(AllocateArgs),

    /// Write data to the buffer account.
    #[account(0, writable, name="buffer", desc = "The account where data is stored.")]
    #[account(1, writable, name="buffer_metadata", desc = "The account to store the buffer account's metadata in.")]
    #[account(2, writable, signer, name="payer", desc="The account that will pay for the rent.")]
    #[account(3, optional, signer, name="authority", desc="The authority of the buffer account.")]
    #[account(4, name="system_program", desc = "System program")]
    WriteData(WriteDataArgs),

    /// Clear the buffer account.
    #[account(0, writable, name="buffer", desc = "The account where data is stored.")]
    #[account(1, writable, name="buffer_metadata", desc = "The account to store the buffer account's metadata in.")]
    #[account(2, writable, signer, name="payer", desc="The account that will pay for the rent.")]
    #[account(3, optional, signer, name="authority", desc="The authority of the buffer account.")]
    #[account(4, name="system_program", desc = "System program")]
    ClearData,
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct AllocateArgs {
    pub target_size: usize,
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct WriteDataArgs {
    pub offset: usize,
    pub value: Vec<u8>,
}

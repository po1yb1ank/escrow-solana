use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Debug, Error, Clone, Copy)]
pub enum EscrowError {
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("Not Rent Exempt")]
    NotRentExempt,
    #[error("Mismatch")]
    ExpectedAmountMismatch,
}

impl From<EscrowError> for ProgramError{
    fn from(e:EscrowError) -> Self{
        ProgramError::Custom(e as u32)
    }
}
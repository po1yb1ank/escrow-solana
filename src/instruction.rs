use std::convert::TryInto;
use solana_program::program_error::ProgramError;

use crate::error::EscrowError::InvalidInstruction;
pub enum EscrowInstruction{
    /// Accs expected:
    /// 0: signer - initiator of the escrow
    /// 1: writable - temp token acc that should be created from this instr and owned by initializer
    /// 2. The initializer's token account for the token they will receive should the trade go through
    /// 3. writable - The escrow account, it will hold all necessary info about the trade.
    /// 4.  The rent sysvar
    /// 5.  The token program
    InitEscrow{
        /// The amount party A expects to receive of token Y
        amount : u64
    }
}

impl EscrowInstruction {
    pub fn unpack(input: &[u8]) ->Result<Self, ProgramError>{
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag{
            0 => Self::InitEscrow{
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into())
        })
    }

    fn unpack_amount(input: &[u8]) ->Result<u64, ProgramError>{
        let amount = input
        .get(..8) // took subslice
        .and_then(|slice| slice.try_into().ok()) // lambda || as an input
        .map(u64::from_le_bytes)
        .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}
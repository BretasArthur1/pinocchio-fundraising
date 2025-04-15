use pinocchio::program_error::ProgramError;

pub mod check;
pub mod contribute;
pub mod refund;
pub mod initialize;


pub use check::*;
pub use contribute::*;
pub use refund::*;
pub use initialize::*;


#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FundraiserInstruction {
    Initialize = 0,
    Contribute = 1,
    Refund = 2,
    Check = 3,
}

impl TryFrom<u8> for FundraiserInstruction{
    type Error = ProgramError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Initialize),
            1 => Ok(Self::Contribute),
            2 => Ok(Self::Refund),
            3 => Ok(Self::Check),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
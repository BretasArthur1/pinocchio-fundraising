

use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey, ProgramResult
};

mod instructions;
mod state;
mod constants;

use instructions::{
    FundraiserInstruction,
    process_initialize_instruction,
    process_contribute_instruction,
    process_check_instruction, 
    process_refund_instruction
};

pinocchio_pubkey::declare_id!("55555555555555555555555555555555555555555555");
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    assert_eq!(program_id, &ID);

    let (instruction_discriminant, instruction_data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match FundraiserInstruction::try_from(*instruction_discriminant)? {
        FundraiserInstruction::Initialize => {
            process_initialize_instruction(accounts, instruction_data)?
        }
        FundraiserInstruction::Contribute => {
            process_contribute_instruction(accounts, instruction_data)?
        }
        FundraiserInstruction::Check => process_check_instruction(accounts)?,
        FundraiserInstruction::Refund => process_refund_instruction(accounts)?,
    }

    Ok(())
}


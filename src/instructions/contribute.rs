use crate::constants::*;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_token::instructions::Transfer;


pub fn process_contribute_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Extract the contribution amount from instruction data
    let amount: u64 = unsafe { *(data.as_ptr() as *const u64) };
    
    // Validate contribution amount is within acceptable limits
    assert!(amount >= MIN_RAISE, "Amount too low");
    assert!(amount <= MAX_RAISE, "Amount too high");
    
    // Destructure required accounts 
    let [signer, contributor, signer_ta, fundraiser, vault, _token_program, _remaining @ ..] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Transfer tokens from contributor to the vault account
    Transfer {
        from: signer_ta,
        to: vault,
        authority: signer,
        amount,
    }
    .invoke()?;
    
    unsafe {
        // Update fundraiser's current raised amount at byte offset 0
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(0) as *mut u64) += amount;
        
        // Update contributor's total contribution amount at byte offset 0
        *(contributor.borrow_mut_data_unchecked().as_mut_ptr().add(0) as *mut u64) += amount;
    }
    
    Ok(())
}
    
    
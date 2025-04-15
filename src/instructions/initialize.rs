use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, sysvars::clock::Clock,
    ProgramResult,
};

pub fn process_initialize_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {

    let [fundraiser, clock, _remaining @ ..] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Check if the clock account is the sysvar clock
    assert_eq!(
        *clock.key(),
        Pubkey::from(five8_const::decode_32_const(
            "SysvarC1ock11111111111111111111111111111111"
        )),
        "The clock account is not the sysvar clock"
    );

    unsafe{
        // Get clock sysvar to access current timestamp
        let clock = &*(clock.borrow_data_unchecked().as_ptr() as *const Clock);
        
        // Initialize the fundraiser data fields at their respective byte offsets
        // The fundraiser account has the following memory layout:
        // - bytes 0-7:   current_amount (u64) - initialized to 0 implicitly
        // - bytes 8-15:  time_started (i64) - initialize with current timestamp
        // - bytes 16-47: maker (Pubkey) - creator of the fundraiser
        // - bytes 48-79: mint_to_raise (Pubkey) - token mint being raised
        // - bytes 80-87: amount_to_raise (u64) - fundraising goal
        // - byte 88:     duration (u8) - fundraising period in days
        // - byte 89:     bump (u8) - PDA bump seed for signing
        
        let fundraiser_account = fundraiser.borrow_mut_data_unchecked().as_mut_ptr();
        *(fundraiser_account.add(8) as *mut i64) = clock.unix_timestamp;
        *(fundraiser_account.add(16) as *mut Pubkey) = *((data.as_ptr()).add(0) as *const Pubkey); 
        *(fundraiser_account.add(48) as *mut Pubkey) = *((data.as_ptr()).add(32) as *const Pubkey); 
        *(fundraiser_account.add(80) as *mut u64) = *((data.as_ptr()).add(64) as *const u64); 
        *(fundraiser_account.add(88) as *mut u64) = *((data.as_ptr()).add(72) as *const u64); 
        *(fundraiser_account.add(89)) = *((data.as_ptr()).add(73)); 
    }
    Ok(())
}

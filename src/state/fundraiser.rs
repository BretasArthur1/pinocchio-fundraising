use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

// Wrapper for fundraiser account data stored as raw bytes
pub struct Fundraiser(*mut u8);

impl Fundraiser {

    // account fields used in the program and now we are going to create a struct to represent them
    //  maker
    //  mint_to_raise
    //  amount_to_raise
    //  current_amount
    //  time_started
    //  duration
    //  bump

     // Total account data size: 90 bytes
     pub const LEN: usize = 8 // current_amount
     + 8  // time_started
     + 32 // maker
     + 32 // mint_to_raise
     + 8  // amount_to_raise
     + 1  // duration
     + 1; // bump 

     // Creates Fundraiser without validation checks
     #[inline(always)]
     pub fn from_account_info_unchecked(account_info: &AccountInfo) -> Self {
         unsafe { Self(account_info.borrow_mut_data_unchecked().as_mut_ptr()) }
     }
    
    // Creates Fundraiser with program ID and size validation
    #[inline(always)]
    pub fn from_account_info(account_info: &AccountInfo) -> Result<Self, ProgramError> {
        unsafe {
            assert_eq!(*account_info.owner(), crate::ID);
            assert_eq!(account_info.data_len(), Self::LEN);
        }
        Ok(Self::from_account_info_unchecked(account_info))
    }

    // Gets amount of funds currently raised (bytes 0-7)
    #[inline(always)]
    pub fn current_amount(&self) -> u64 {
        unsafe { *(self.0 as *const u64) } 
    }
    
    // Gets timestamp when fundraiser started (bytes 8-15)
    // #[inline(always)]
    // pub fn time_started(&self) -> i64 {
    //     unsafe { *(self.0.add(8) as *const i64) } 
    // }
    
    // Gets public key of fundraiser creator (bytes 16-47)
    #[inline(always)]
    pub fn maker(&self) -> Pubkey {
        unsafe { *(self.0.add(16) as *const Pubkey) } 
    }
    
    // Gets duration of fundraiser in days (byte 88)
    // #[inline(always)]
    // pub fn duration(&self) -> u8 {
    //     unsafe { *(self.0.add(88) as *const u8) } 
    // }
    
    // Gets PDA bump seed (byte 89)
    #[inline(always)]
    pub fn bump(&self) -> u8 {
        unsafe { *(self.0.add(89) as *const u8) } 
    }
}
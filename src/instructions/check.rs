use crate::state::Fundraiser;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_token::{instructions::Transfer, ID};
use pinocchio::instruction::{Seed, Signer};
use pinocchio::pubkey::find_program_address;

pub fn process_check_instruction(accounts: &[AccountInfo]) -> ProgramResult {
    let [signer, signer_ta, fundraiser, vault, _token_program, _remaining @ ..] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate program ID
    unsafe {
        assert_eq!(
            *vault.owner(),
            ID,
            "Token program is not the owner of the vault"
        );
    }

    let fundraiser_account = Fundraiser::from_account_info_unchecked(fundraiser);
    
    // 1. Verify the fundraiser PDA with find_program_address
    let seeds = [(b"fundraiser"), signer.key().as_slice(), &[fundraiser_account.bump()]];
    let fundraiser_pda = find_program_address(&seeds, &crate::ID).0;
    assert_eq!(*fundraiser.key(), fundraiser_pda, "Invalid fundraiser PDA");
    
    // 2. Perform the token transfer with signed authority
    let bump = [fundraiser_account.bump()];
    let seed = [
        Seed::from(b"fundraiser"), 
        Seed::from(signer.key()), 
        Seed::from(&bump)
    ];
    let seeds = Signer::from(&seed);
    
    Transfer {
        from: vault,
        to: signer_ta,
        authority: fundraiser,
        amount: fundraiser_account.current_amount(),
    }.invoke_signed(&[seeds.clone()])?;
    
    Ok(())
}
    
    

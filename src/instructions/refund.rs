use crate::state::{Contributor, Fundraiser};
use pinocchio::{account_info::AccountInfo, msg, program_error::ProgramError, ProgramResult};
use pinocchio_token::instructions::Transfer;
use pinocchio::instruction::{Seed, Signer};
use pinocchio::pubkey::find_program_address;

pub fn process_refund_instruction(accounts: &[AccountInfo]) -> ProgramResult {
    let [fundraiser, contributor_account, contributor_ta, vault, _token_program, _remaining @ ..] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let fundraiser_account = Fundraiser::from_account_info(fundraiser)?;
    let contributor = Contributor::from_account_info(contributor_account)?;

    msg!("Processing refund");

    assert!(contributor.amount() > 0, "No amount to refund");
    
    // 1. Verify the fundraiser PDA with find_program_address
    let maker = fundraiser_account.maker();
    let seeds = [(b"fundraiser"), maker.as_ref(), &[fundraiser_account.bump()]];
    let fundraiser_pda = find_program_address(&seeds, &crate::ID).0;
    assert_eq!(*fundraiser.key(), fundraiser_pda, "Invalid fundraiser PDA");
    
    // 2. Prepare seeds for signing
    let bump = [fundraiser_account.bump()];
    let seed = [
        Seed::from(b"fundraiser"), 
        Seed::from(&maker), 
        Seed::from(&bump)
    ];
    let seeds = Signer::from(&seed);
    
    // 3. Execute token transfer with PDA signature
    Transfer {
        from: vault,
        to: contributor_ta,
        authority: fundraiser,
        amount: contributor.amount(),
    }.invoke_signed(&[seeds.clone()])?;
    
    Ok(())
}
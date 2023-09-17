use borsh::{BorshDeserialize, BorshSerialize};
use rand::Rng;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// Random number
    pub random_number: u32,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");

    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to store the random number
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Generate a random number
    let mut rng = rand::thread_rng();
    let random_number = rng.gen::<u32>();

    // Create a GreetingAccount with the random number and serialize it into the account data
    let greeting_account = GreetingAccount { random_number };
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Random number stored: {}", random_number);

    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data: Vec<u8> = Vec::new();

        process_instruction(&program_id, &[account], &instruction_data).unwrap();

        // Deserialize the account data to get the stored random number
        let stored_random_number = GreetingAccount::try_from_slice(&account.data.borrow())
            .unwrap()
            .random_number;

        assert_ne!(stored_random_number, 0);
        println!("Stored Random Number: {}", stored_random_number);
    }
}

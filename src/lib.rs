use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Define el tipo de estado almacenado en las cuentas
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// número de saludos
    pub counter: u32,
}

// Declarar y exportar el punto de entrada del programa
entrypoint!(process_instruction);

// Implementación del punto de entrada del programa
pub fn process_instruction(
    program_id: &Pubkey, // Clave pública de la cuenta en la que se cargó el programa de saludo
    accounts: &[AccountInfo], // La cuenta a la que se va a saludar
    _instruction_data: &[u8], // Ignorado, todas las instrucciones son saludos
) -> ProgramResult {
    msg!("Hola, nuevo libro de saludos");

    // Iterar las cuentas es más seguro que indexarlas
    let accounts_iter = &mut accounts.iter();

    // Obtener la cuenta a la que se va a saludar
    let account = next_account_info(accounts_iter)?;

    // La cuenta debe ser propiedad del programa para poder modificar sus datos
    if account.owner != program_id {
        msg!("La cuenta saludada no tiene la identificación de programa correcta");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Incrementar y almacenar la cantidad de veces que se ha saludado a la cuenta
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    greeting_account.counter += 1;
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("¡Saludada {} vez(es)!", greeting_account.counter);

    Ok(())
}

// Pruebas de integridad
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

        let accounts = vec![account];

        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            1
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            2
        );
    }
}


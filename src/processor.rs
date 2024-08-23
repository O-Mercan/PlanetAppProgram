use std::str::FromStr;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{self, next_account_info, Account, AccountInfo}, 
    entrypoint::ProgramResult, 
    msg, 
    program_error::ProgramError, 
    pubkey::Pubkey
};

use crate::{error::PlanetAppError, 
    instruction::PlanetAppInstruction, 
    state::{Explorer, Planet}
};


pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {

        let instruction = PlanetAppInstruction::unpack(instruction_data)?;
        msg!("unpack called");

        match instruction {
            
            PlanetAppInstruction::NewExplorer{data} => {
                msg!("NewExplorer Case ");
                Self::new_explorer_record(program_id, accounts, data)},

            PlanetAppInstruction::NewPlanet{data} => {
                msg!("NewPlanet Case");
                Self::new_planet_record(program_id, accounts, data)
            },
            PlanetAppInstruction::Confirme => {
                msg!("Confirme Case");
                Self::confirme(program_id, accounts)
            }
            PlanetAppInstruction::AddIfConfirme => {
                Self::add_if_confirme(program_id, accounts)
            },
            
        }
    }

    pub fn new_explorer_record(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: Explorer
    ) -> ProgramResult {

        msg!("new_explorer");
        let account_info_iter = &mut accounts.iter();
        let explorer_account = next_account_info(account_info_iter)?; //kullanici hesap bilgileri
        let payer = next_account_info(account_info_iter)?; 
        if !payer.is_signer{
            panic!()
        }

        if explorer_account.owner != program_id{panic!()}

        let explorer = Explorer{
            name: instruction_data.name,
            surname: instruction_data.surname,
            age: instruction_data.age,
            planet_amount: 0,
            explorer_account: payer.key.to_bytes(),
        };
               
        explorer.serialize(&mut &mut explorer_account.data.borrow_mut()[..])?;

        Ok(())
    }

    pub fn new_planet_record(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: Planet
    ) -> ProgramResult {

        let account_info_iter = &mut accounts.iter();
        let new_planet_account = next_account_info(account_info_iter)?; 
        let payer = next_account_info(account_info_iter)?; //explorer wallet
        let explorer_account  = next_account_info(account_info_iter)?; //explorer account info's
        msg!("1");
        if !payer.is_signer{
            panic!()
        }

        msg!("2");
        let explorer = Explorer::try_from_slice(&explorer_account.data.borrow())?;
        msg!("3");

        let payer_from_bytes = Pubkey::new_from_array(explorer.explorer_account);
        msg!("4");
        if payer.key != &payer_from_bytes {
            panic!()
        }

        let planet = Planet{
            name: instruction_data.name,
            planet_age: instruction_data.planet_age,
            explorer_account: explorer_account.key.to_bytes(),
            is_confirmed: 0,
            
        };

        planet.serialize(&mut &mut new_planet_account.data.borrow_mut()[..])?;
            
        Ok(())
    }

    pub fn confirme(
        program_id: &Pubkey,
        accounts: &[AccountInfo], 
    ) -> ProgramResult {

        msg!("confirme case");
        let account_info_iter = &mut accounts.iter();

        let explorered_planet_account = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;
        let authority_address = Pubkey::from_str("FhU6kagtkkcWV4gmaDoAaQYQcxXRPakwwfDbHkMFjSGV").unwrap();
        msg!("{}", authority.key.to_string());
        if &authority_address != authority.key {
            msg!("1");
           panic!()

        }

        if !authority.is_signer{
            msg!("2");
            panic!()
        }

       let mut planet_info = Planet::try_from_slice(&explorered_planet_account.data.borrow())?;
        planet_info.is_confirmed = 1;
        planet_info.serialize(&mut &mut explorered_planet_account.data.borrow_mut()[..])?;

        Ok(())
    }

    pub fn add_if_confirme(
        program_id: &Pubkey,
        accounts: &[AccountInfo]
    ) -> ProgramResult{

        let account_iter = &mut accounts.iter();
        let explorer_account = next_account_info(account_iter)?;
        let explored_planet_account = next_account_info(account_iter)?;

        if  explored_planet_account.owner != program_id {
            panic!()
        }

        let mut planet_info = Planet::try_from_slice(&explored_planet_account.data.borrow())?;

        if planet_info.is_confirmed != 1 {
            panic!()
        }

        planet_info.is_confirmed = 2;

        let mut explorer_info = Explorer::try_from_slice(&explorer_account.data.borrow())?;
        explorer_info.planet_amount += 1;

        explorer_info.serialize(&mut &mut explorer_account.data.borrow_mut()[..])?;
        planet_info.serialize(&mut &mut explored_planet_account.data.borrow_mut()[..])?;

        Ok(())
    }

}

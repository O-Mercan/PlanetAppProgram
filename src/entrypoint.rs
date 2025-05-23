use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};
use crate::processor::Processor;


entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],)
    -> ProgramResult {
        msg!("entrypoint");
        Processor::process(program_id, accounts, instruction_data)?;
        
        Ok(())
    }

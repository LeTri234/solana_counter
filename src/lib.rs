use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};
use crate::instruction::CounterInstruction;

pub mod instruction;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct CounterAccount {
    pub counter: u32,
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Counter program entry point");

    let instruction: CounterInstruction = CounterInstruction::unpack(instruction_data)?;

    let account_iter = &mut accounts.iter();

    let account = next_account_info(account_iter)?;

    let mut counter_account = CounterAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        CounterInstruction::Increment(args) => {
            counter_account.counter += args.value;
        }
        CounterInstruction::Decrement(args) => {
            counter_account.counter -= args.value;
            counter_account.counter = match counter_account.counter.checked_sub(args.value){
                Some(diff) => diff,
                None => 0,
            }
        }
        CounterInstruction::Reset => {
            counter_account.counter = 0;
        }
        CounterInstruction::Update(args) => {
            counter_account.counter = args.value;
        }
    }

    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::clock::Epoch;
    use solana_sdk::lamports;
    use std::mem;

    #[test]
    fn test_counter() {
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

        let accounts = vec![account];
    }
    
}
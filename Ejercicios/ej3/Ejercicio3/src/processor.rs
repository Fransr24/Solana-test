use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_pack::{Pack},
};

use crate::{instruction::ContadorInstruction, state::Counter};

pub struct Processor;
impl Processor
{
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult 
    {
        let instruction = ContadorInstruction::unpack(instruction_data)?;
        match instruction{
            ContadorInstruction::Aumentar{amount} => {
                msg!("Instruction: Aumentar");
                Self::process_increase(accounts, amount, program_id)
            }
        }
    }
    fn process_increase(accounts: &[AccountInfo], amount: u32, program_id: &Pubkey,) -> ProgramResult
    {
        let account_info_iter = &mut accounts.iter();
        let counter_acc = next_account_info(account_info_iter)?;
        if counter_acc.owner == program_id
        {
            let mut counter_data = Counter::unpack_unchecked(&counter_acc.try_borrow_data()?)?;
            aumentar(& mut counter_data, amount);
            Counter::pack(counter_data,&mut counter_acc.try_borrow_mut_data()?,
        )?;
        }
        Ok(())
    }
}

fn aumentar (sumo: & mut Counter, amount: u32)
{
    sumo.cantidad += amount;
}
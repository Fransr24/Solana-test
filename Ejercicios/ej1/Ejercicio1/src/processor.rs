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
            ContadorInstruction::Aumentar{} => {
                msg!("Instruction: Aumentar");
                Self::process_increase(accounts, program_id)
            }
        }
    }
    fn process_increase(accounts: &[AccountInfo], program_id: &Pubkey,) -> ProgramResult
    {
        let account_info_iter = &mut accounts.iter();
        let counter_acc = next_account_info(account_info_iter)?; //Cuenta contador
        if counter_acc.owner == program_id //El ownership de la account que tiene el contador debe ser el programa que se envió por la transaccion
        {
            let mut counter_data = Counter::unpack_unchecked(&counter_acc.try_borrow_data()?)?; //obtengo el dato del contador

            //Llamo a la función, no me interesa el contenido de la instriccion ni nada mas ya que con saber que la instruccion es 0, aumento el contador
            //Solo le paso por donde va el contador
            aumentar(& mut counter_data); //referencia para que no haga borrow de ownership

            //Devuelvo el dato
            Counter::pack(counter_data,&mut counter_acc.try_borrow_mut_data()?,
        )?;
        }
        Ok(())
    }
}

fn aumentar (sumo: & mut Counter)
{
    sumo.cantidad += 1; //En rust no hace falta hacer -> para referenciar un elemento de la struct 
}
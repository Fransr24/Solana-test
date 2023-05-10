use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
    program_pack::{Pack},
    rent::Rent,
    sysvar::Sysvar,
    system_instruction::create_account,
};

use crate::{instruction::ContadorInstruction, state::Counter, error::Errores};

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
        let signer = next_account_info(account_info_iter)?;//Cuenta signer
        let system_program = next_account_info(account_info_iter)?;//Cuenta signer
        if !signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        let (pda, _bump_seed) = Pubkey::find_program_address(&[b"counter"], program_id);
        //Verificacion para ver si es la misma direccion
        if pda != *counter_acc.key{
            return Err(Errores::NotExpectedAddress.into());
        }
        //Como verifico si una cuenta esta inicializada o no? veo si el owner es el systemProgram y sus lamports son 0
        if counter_acc.lamports() == 0
        && *counter_acc.owner == solana_program::system_program::id()
        {
            
            let space = Counter::LEN;
            let rent = Rent::get()?; //obtengo datos de Rent, como hacer los calculos y eso para la siguiente linea
            let rent_lamports = rent.minimum_balance(space);
            //rent_lamports debo buscar lamports minimos para la renta, ni idea
            //Ahora debo llamar a la funcion Invoke_signed para mandar la transaccion al program que se envió por parametro (el que tiene el ownership de counter)
            //La transaccion será que cree una cuenta a su nombre con las seeds de la pda

            invoke_signed(
                //creo cuenta con el signer (para que firme la transaccion), counter 
                &create_account(
                    &signer.key, //Signer, será el feepayer
                    &counter_acc.key, //recordar que counter no está creado, esta será su clave publica
                    rent_lamports, //cantidad de lamports que tendrá al ser inicializada (para la rent)
                    space as u64, //Tamaño de la cuenta (tomar como referencia la estructura Contador en state)
                    program_id, //programa owner de la cuenta
                ),
                &[signer.clone(), counter_acc.clone()], //array de cuentas
                &[&[b"counter", &[_bump_seed]]], //Signer para la transaccion de invoke_signed
            )?;
        }

        let mut counter_data = Counter::unpack_unchecked(&counter_acc.try_borrow_data()?)?; //obtengo el dato del contador

        //Llamo a la función, no me interesa el contenido de la instriccion ni nada mas ya que con saber que la instruccion es 0, aumento el contador
        //Solo le paso por donde va el contador
        increase(& mut counter_data); //referencia para que no haga borrow de ownership

        //Devuelvo el dato
        Counter::pack(counter_data,&mut counter_acc.try_borrow_mut_data()?,)?;
        Ok(())
    }
}

fn increase (sumo: & mut Counter)
{
    sumo.cantidad += 1; //En rust no hace falta hacer -> para referenciar un elemento de la struct 
}
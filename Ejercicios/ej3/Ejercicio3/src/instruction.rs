use solana_program::program_error::ProgramError;
use solana_program::msg;

use crate::error::Errores::InvalidInstruction;

pub enum ContadorInstruction
{
    // Accounts expected: 
    //0. Cuenta Externa donde se guarda el contador se debe poder escribir en ella para aumentarlo
    Aumentar{
        amount: u32
    },
}

impl ContadorInstruction 
{
    pub fn unpack(input: &[u8]) -> Result <Self, ProgramError> 
    {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?; 
        msg!("{}", tag);
        Ok(match tag{
            0 => Self::Aumentar {amount: Self::unpack_amount(rest)?,},
            _ => return Err(InvalidInstruction.into()),
        })
    }
    fn unpack_amount(input: &[u8]) -> Result<u32, ProgramError> {

        let amount :u32 = input[0] as u32; //Solo me queda 1 byte en el array, lo convierto de esta manera y listo
        Ok(amount)//retorno amount dentro de un result, que indica que no se produjo un error
    }
}


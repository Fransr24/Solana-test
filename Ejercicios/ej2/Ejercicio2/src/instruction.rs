use solana_program::program_error::ProgramError;

use crate::error::Errores::InvalidInstruction;

pub enum ContadorInstruction
{
    // Accounts expected: 
    //0. Cuenta pda donde se guarda el contador se debe poder escribir en ella para aumentarlo
    //1. Cuenta signer donde se firma la transaccion [signer]
    //2. System program (si no lo envio tengo error al crear cuenta)
    Aumentar{},
}

impl ContadorInstruction 
{
    pub fn unpack(input: &[u8]) -> Result <Self, ProgramError> 
    {
        let (tag, _rest) = input.split_first().ok_or(InvalidInstruction)?; //separo la instruccion en tag y rest
    //Match, solo tengo 1 caso de instruccion asi que va a ser 0, si es 0 llamo a la funcion Aumentar (que va a aumentar el contador), si es otra cosa, error
        Ok(match tag{
            0 => Self::Aumentar {},
            _ => return Err(InvalidInstruction.into()),
        })
    }
}

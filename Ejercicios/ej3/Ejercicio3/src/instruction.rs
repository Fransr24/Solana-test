use solana_program::program_error::ProgramError;

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
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?; //separo la instruccion en tag y rest
    //Match, solo tengo 1 caso de instruccion asi que va a ser 0, si es 0 llamo a la funcion Aumentar (que va a aumentar el contador), si es otra cosa, error
        Ok(match tag{
            0 => Self::Aumentar {amount: Self::unpack_amount(rest)?,},
            _ => return Err(InvalidInstruction.into()),
        })
    }
    fn unpack_amount(input: &[u8]) -> Result<u32, ProgramError> {
        let amount = input
            .get(..8) //con get obtengo una referencia a los primeros 8 bytes de la entrada input
            .and_then(|slice| slice.try_into().ok()) //Con andthen hago que el resultado anterior(referencia de 8 bytes) lo convierto a un vector fijo de 8 bytes [u8;8]
        //Con tryinto hago cada conversion
            .map(u32::from_le_bytes)//convierto de u8 a u32 con little endian
            .ok_or(InvalidInstruction)?;//devuelvo error
        Ok(amount)//retorno amount dentro de un result, que indica que no se produjo un error
    }
}

use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum Errores {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    // Not expected Adress
    #[error("The pda Adress is not the expected")]
    NotExpectedAddress,
    
}
//Convierto EscrowError a ProgramError, porque el entrypoint retorna un result de tipo ProgramError 
impl From<Errores> for ProgramError {
    fn from(e: Errores) -> Self {
        ProgramError::Custom(e as u32)
    }
}
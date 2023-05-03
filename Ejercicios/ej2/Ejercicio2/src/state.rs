use solana_program::{
    program_error::ProgramError,
    program_pack::{Pack, Sealed},
};
#[derive(Debug, Default)]
pub struct Counter {
    pub cantidad :u32,
}
impl Counter {}
impl Sealed for Counter {}

impl Pack for Counter
{
    const LEN: usize = 4; //Tamaño de estructura 4 bytes
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> { //como todos los unpack, te transforma el array de bytes en datos
        let cantidad:u32;
        //Tengo que hacer un unpack de un array de 8 bits en un u32
        (_, cantidad) = unpack_u32(src);
        Ok(Counter { cantidad })
    } 
    fn pack_into_slice(&self, src: &mut[u8]){
        //Tengo que hacer un pack de un u32 a un array de 8 bits
        pack_u32( src ,self.cantidad);
    } 
}

fn unpack_u32(buf: &[u8]) -> (&[u8], u32) {
    let mut data_src: [u8; 4] = [0 as u8; 4];
    data_src.copy_from_slice(&buf[..4 as usize]);
    return (&buf[4..], u32::from_le_bytes(data_src));
}
fn pack_u32(buf: &mut [u8], data: u32) -> &mut [u8] {
    buf[0..4].copy_from_slice(&data.to_le_bytes());
    &mut buf[4..]
}
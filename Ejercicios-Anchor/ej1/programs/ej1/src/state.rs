use anchor_lang::prelude::*;

#[account]
pub struct Counter
{
    pub number: u32,
}
impl Counter {
    pub const LEN: usize = 8 + 4;
}
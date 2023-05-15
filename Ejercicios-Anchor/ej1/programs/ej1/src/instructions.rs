use anchor_lang::{prelude::*};
use crate::{state::*};

#[derive(Accounts)]
pub struct Increase <'info>
{
    #[account(mut)]
    pub signer: Signer<'info>,
    //#[account(init, payer = signer, space= 8+4)]
    #[account(
        init_if_needed,
        space = Counter::LEN,
        seeds = ["counter".as_ref()],
        bump,
        payer = signer
    )]
    pub counter: Account<'info, Counter>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
pub fn handler(ctx: Context<Increase>) -> Result<()>
{
    ctx.accounts.counter.number += 1;
    Ok(())
}
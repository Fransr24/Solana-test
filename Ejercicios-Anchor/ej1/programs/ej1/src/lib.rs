use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;
pub use crate ::{instructions::*, state::*};

declare_id!("GQ5Wi1iyf328x9fMyahYV8ErWkLfX2Hav3qKJ7e5w3Kz");

#[program]
pub mod ej1 {
    use super::*;

    pub fn increase(ctx: Context<Increase>) -> Result<()> {
        handler(ctx)
    }
}



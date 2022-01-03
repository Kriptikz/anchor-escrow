use anchor_lang::prelude::*;

declare_id!("CGcc7NHAsJ1zdbx47F8vcQ4qh8Z5JzXnsbNgSjcQh2kp");

#[program]
pub mod anchor_escrow {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

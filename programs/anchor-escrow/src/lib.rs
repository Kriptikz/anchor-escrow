use anchor_lang::prelude::*;

declare_id!("CGcc7NHAsJ1zdbx47F8vcQ4qh8Z5JzXnsbNgSjcQh2kp");

#[program]
pub mod anchor_escrow {
    use super::*;
    pub fn initialize(
        ctx: Context<Initialize>,
        _vault_account_bump: u8,
        initializer_amount: u64,
        taker_amount: u64,
    ) -> ProgramResult {
        // TODO
        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>) -> ProgramResult {
        // TODO
        Ok(())
    }

    pub fn exchange(ctx: Context<Exchange>) -> ProgramResult {
        // TODO
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Mint, SetAuthority, TokenAccount, Transfer};
use spl_token::instruction::AuthorityType;

declare_id!("CGcc7NHAsJ1zdbx47F8vcQ4qh8Z5JzXnsbNgSjcQh2kp");

#[program]
pub mod anchor_escrow {
    use super::*;

    const ESCROW_PDA_SEED: &[u8] = b"escrow";

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
pub struct Initialize<'info> {
    pub initializer: AccountInfo<'info>,
    pub mint: Account<'info, Mint>,
    pub vault_account: Account<'info, TokenAccount>,
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    pub initializer_receive_token_account: Account<'info, TokenAccount>,
    pub escrow_account: Box<Account<'info, EscrowAccount>>,
    pub system_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Exchange<'info> {
    pub taker: AccountInfo<'info>,
    pub taker_deposit_token_account: Account<'info, TokenAccount>,
    pub taker_receive_token_account: Account<'info, TokenAccount>,
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    pub initializer_receive_token_account: Account<'info, TokenAccount>,
    pub initializer: AccountInfo<'info>,
    pub escrow_account: Box<Account<'info, EscrowAccount>>,
    pub vault_account: Account<'info, TokenAccount>,
    pub vault_authority: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Cancel<'info> {
    pub initializer: AccountInfo<'info>,
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    pub vault_account: Account<'info, TokenAccount>,
    pub vault_authority: AccountInfo<'info>,
    pub escrow_account: Box<Account<'info, EscrowAccount>>,
    pub token_program: AccountInfo<'info>,
}

#[account]
pub struct EscrowAccount {
    pub initializer_key: Pubkey,
    pub initializer_deposit_token_account: Pubkey,
    pub initializer_receive_token_account: Pubkey,
    pub initializer_amount: u64,
    pub taker_amount:  u64,
}


use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use std::str::FromStr;


declare_id!("Fzp3NWwFAUu31ofQ4qgrmkRvrgUU9cSAtBkeJGfvMSFA");

const MINIMUM_SLOT:u64 = 100;
const TOKEN_MINIMUM_BALANCE:u64 = 100_000_000_000;
const USDC_MINT:&str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

#[program]
pub mod check_token {
    use super::*;

    pub fn initialize_data_account(ctx: Context<InitializedDataAccount>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializedDataAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + DataAccount::INIT_SPACE,
    )]
    pub data_account:Account<'info, DataAccount>,

    pub some_user_account: Signer<'info>,

    system_program: Program<'info, System>,
}

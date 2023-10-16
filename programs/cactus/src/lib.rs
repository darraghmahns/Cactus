mod trade;
use trade::*;

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, mint_to};
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("6CDCUqzb1MQnEXaxhHNqXV2s1f9qChJ3GyXQwcH9Fp93");

//Define main program module
#[program]
pub mod cactus {
   

    use super::*;

    //function to initialize registry with a name
    pub fn initialize_registry(ctx: Context<InitializeRegistry>, registry_name: String) -> ProgramResult {

        //obtain a mutable reference to the registry account from the context
        let registry = &mut ctx.accounts.registry;
        registry.name = registry_name;
        Ok(())

    }

    //function to issue credits to a speecific token account
    pub fn issue_credits(ctx: Context<IssueCredits>, amount: u64) -> ProgramResult {
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let mint = ctx.accounts.mint.to_account_info();
        let destination = ctx.accounts.destination.to_account_info();
        let authority = ctx.accounts.authority.to_account_info();
        let cpi_accounts = anchor_spl::token::MintTo {
            mint: mint.clone(),
            to: destination.clone(),
            authority: authority.clone(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        mint_to(cpi_ctx, amount).map_err(|e| ProgramError::AccountDataTooSmall)?;
        Ok(())
    }

    pub fn buy_credits(ctx: Context<BuyCredits>, amount: u64) -> ProgramResult {
        trade::buy_credits(ctx, amount)
    }

    pub fn sell_credits(ctx: Context<SellCredits>, amount: u64) -> ProgramResult {
        trade::sell_credits(ctx, amount)
    }

    pub fn trade_credits(ctx: Context<TradeCredits>, amount: u64) -> ProgramResult {
        trade::trade_credits(ctx, amount)
    }

}


//define the account struct needed for each method
#[derive(Accounts)]
pub struct InitializeRegistry<'info> {
    #[account(init, payer = user, space = 8  + 32)]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct IssueCredits<'info> {
    pub token_program: AccountInfo<'info>,
    pub mint: Account<'info, Mint>,
    pub destination: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Registry {
    pub name: String,
    // SPL Token account to hold the carbon credits
    pub credits_token_account: Pubkey,
}

// #[error]
// pub enum ErrorCode {
//     #[msg("Overflow")]
//     Overflow,
//}

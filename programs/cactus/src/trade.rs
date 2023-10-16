use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Token, Transfer as SPLTransfer};
use solana_program::entrypoint::ProgramResult;
use solana_program::system_instruction;

#[derive(Accounts)]
pub struct BuyCredits<'info> {
    pub token_program: AccountInfo<'info>,
    #[account(mut)]
    pub registry_credits_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub buyer_account: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct TradeCredits<'info> {
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub buyer_account: Account<'info, TokenAccount>,
    pub seller: Signer<'info>,
    #[account(mut)]
    pub seller_account: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct SellCredits<'info> {
    pub seller: Signer<'info>,
    #[account(mut)]
    pub seller_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub registry_credits_account: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct TransferSpl<'info> {
    pub from: Signer<'info>,
    #[account(mut)]
    pub from_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn transfer_spl_tokens(ctx: Context<TransferSpl>, amount: u64) -> Result<()> {
    let destination = &ctx.accounts.to_ata;
    let source = &ctx.accounts.from;
    let token_program = &ctx.accounts.token_program;
    let authority = &ctx.accounts.from;

    // Transfer tokens from taker to initializer
    let cpi_accounts = TransferSpl {




        from: source.to_account_info().clone(),
        from_ata: source.to_account_info().clone(),

        to_ata: destination.to_account_info().clone(),
        authority: authority.to_account_info().clone(),
        token_program: ctx.accounts.token_program.to_account_info().clone(),
    };
    let cpi_program = token_program.to_account_info();
    
    token::transfer(
        CpiContext::new(cpi_program, cpi_accounts),
        amount)?;
    Ok(())
}

pub fn buy_credits(ctx: Context<BuyCredits>, amount: u64) -> ProgramResult {
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let from = ctx.accounts.registry_credits_account.to_account_info();
    let to = ctx.accounts.buyer_account.to_account_info();
    let authority = ctx.accounts.authority.to_account_info();

    let cpi_ctx = CpiContext::new(cpi_program)
        .with_accounts(transfer {
            from: from.clone(),
            to: to.clone(),
            authority: authority.clone(),
        });
    
    token::transfer(cpi_ctx, amount)
}


pub fn trade_credits(ctx: Context<TradeCredits>, amount: u64) -> ProgramResult {
    
    //Transfer tokens from the seller's account to the registry;s account
    token::transfer(
        ctx.accounts.into(), 
        amount,
    )?;
    Ok(())

}

pub fn sell_credits(ctx: Context<SellCredits>, amount: u64) -> ProgramResult {
    
    //transfer tokens from the seller's account to the registry's account
    token::transfer(
        ctx.accounts, 
        amount,
    )?;
    Ok(())

}
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct Loan<'info> {
    #[account(mut)]
    pub borrower: Signer<'info>,
    #[account(
        init_if_needed,
        payer = borrower,
        associated_token::mint = mint,
        associated_token::authority = borrower
    )]
    pub borrower_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [b"protocol"],
        bump
    )]
    pub protocol: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = borrower,
        associated_token::mint = mint,
        associated_token::authority = protocol
    )]
    pub protocol_ata: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub instructions: AccountInfo<'info>,
}
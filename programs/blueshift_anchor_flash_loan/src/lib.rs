#![warn(unexpected_cfgs)]
use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod states;

use instructions::*;
use states::*;

declare_id!("22222222222222222222222222222222222222222222");

#[program]
pub mod blueshift_anchor_flash_loan {
    use super::*;

    pub fn borrow(ctx: Context<Loan>, borrow_amount: u64) -> Result<()> {
        instructions::borrow(ctx, borrow_amount)
    }

    pub fn repay(ctx: Context<Loan>) -> Result<()> {
        instructions::repay(ctx)
    }
}
use crate::{errors::TaskTraderError, state::admin::Admin};
use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct UpdateAdmin<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"admin"],
        bump,
        constraint = admin.signer == *payer.key @ TaskTraderError::Unauthorized
    )]
    pub admin: Account<'info, Admin>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn update_admin(ctx: Context<UpdateAdmin>, signer: Pubkey, fee_receiver: Pubkey) -> Result<()> {
    msg!("Updating admin...");

    ctx.accounts.admin.signer = signer;
    ctx.accounts.admin.fee_receiver = fee_receiver;

    Ok(())
}
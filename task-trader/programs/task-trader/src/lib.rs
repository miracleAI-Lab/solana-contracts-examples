mod errors;
mod instructions;
mod state;
mod utils;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("DSyKrLRc83jxeEUiUJdsyePRcreQ2dkXj3vdpggH8wd1");

#[program]
pub mod task_trader {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        signer: Pubkey,
        fee_receiver: Pubkey,
    ) -> Result<()> {
        msg!("Initializing...");

        instructions::initialize::initialize(ctx, signer, fee_receiver)
    }

    pub fn update_admin(
        ctx: Context<UpdateAdmin>,
        signer: Pubkey,
        fee_receiver: Pubkey,
    ) -> Result<()> {
        msg!("Updating admin...");

        instructions::update_admin::update_admin(ctx, signer, fee_receiver)
    }

    pub fn create_task(
        ctx: Context<CreateTask>,
        task_id: u64,
        task_amount: u64,
        taker_num: u64,
        coin_mint: Pubkey, // usdt, mai
        rewards: u64,      // mai
        expire_time: i64,
    ) -> Result<()> {
        msg!("Creating Task Trader...");

        instructions::create_task::create_task(
            ctx,
            task_id,
            task_amount,
            taker_num,
            coin_mint,
            rewards,
            expire_time,
        )
    }

    pub fn apply_task(ctx: Context<ApplyTask>) -> Result<()> {
        msg!("Applying Task Trader...");
        instructions::apply_task::apply_task(ctx)
    }

    pub fn approve_application(ctx: Context<ApproveApplication>) -> Result<()> {
        msg!("Approving Application...");
        instructions::approve_application::approve_application(ctx)
    }

    pub fn reject_application(ctx: Context<RejectApplication>) -> Result<()> {
        msg!("Rejecting Application...");
        instructions::reject_application::reject_application(ctx)
    }

    pub fn submit_acceptance(ctx: Context<SubmitAcceptance>) -> Result<()> {
        msg!("Submitting Acceptance...");
        instructions::submit_acceptance::submit_acceptance(ctx)
    }

    pub fn open_task(ctx: Context<SetupTaskState>) -> Result<()> {
        msg!("Open Task ...");
        instructions::setup_task_state::open_task(ctx)
    }

    pub fn close_task(ctx: Context<SetupTaskState>) -> Result<()> {
        msg!("Close Task ...");
        instructions::setup_task_state::close_task(ctx)
    }

    pub fn verify_task_application(
        ctx: Context<VerifyTaskApplication>,
        is_accepted: bool,
    ) -> Result<()> {
        msg!("Verifying task application...");
        instructions::verify_task_application::verify_task_application(ctx, is_accepted)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        msg!("Withdrawing rewards...");
        instructions::withdraw::withdraw(ctx)
    }

    pub fn update_task_support_coin(
        ctx: Context<UpdateTaskSupportCoin>,
        coin_mints: Vec<Pubkey>,
    ) -> Result<()> {
        msg!("Updating task support coin...");
        instructions::update_task_support_coin::update_task_support_coin(ctx, coin_mints)
    }
}

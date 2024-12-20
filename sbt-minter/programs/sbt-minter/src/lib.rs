mod instructions;
mod state;
mod errors;

use anchor_lang::prelude::*;

use instructions::*;
use state::*;

declare_id!("GwvQ53QTu1xz3XXYfG5m5jEqwhMBvVBudPS8TUuFYnhT");

#[program]
pub mod sbt_minter {
    use super::*;

    pub fn create_sbt_token_mint(
        ctx: Context<CreateSbtMint>,
        token_name: String,
        token_symbol: String,
        token_uri: String,
        signer: Pubkey,
        fee_account: Pubkey
    ) -> Result<()> {
        msg!("Creating SBT mint...");

        sbt_create::create_sbt_token_mint(ctx, token_name, token_symbol, token_uri, signer, fee_account);

        msg!("SBT mint created successfully.");

        Ok(())
    }

    pub fn mint_sbt_token_paid(
        ctx: Context<SbtMint>,
        name: String,
        photo: String,
        twitter_id: String,
        discord_id: String,
        telegram_id: String,
        score: u64,
        signature: [u8; 64],
        recovery_id: u8
    ) -> Result<()> {
        msg!("Paid minting SBT token...");
        
        sbt_mint::mint_sbt_token_paid(
            ctx,
            name,
            photo,
            twitter_id,
            discord_id,
            telegram_id,
            score,
            signature,
            recovery_id
        );

        msg!("Paid minting SBT token successfully.");

        Ok(())
    }

    pub fn mint_sbt_token_free(
        ctx: Context<SbtMint>,
        name: String,
        photo: String,
        twitter_id: String,
        discord_id: String,
        telegram_id: String,
        score: u64,
        signature: [u8; 64],
        recovery_id: u8
    ) -> Result<()> {
        msg!("Free minting SBT token...");

        sbt_mint::mint_sbt_token_free(
            ctx,
            name,
            photo,
            twitter_id,
            discord_id,
            telegram_id,
            score,
            signature,
            recovery_id
        );

        msg!("Free minting SBT token successfully.");

        Ok(())
    }

    pub fn update_sbt_info(
        ctx: Context<SbtMint>,
        name: String,
        photo: String,
        twitter_id: String,
        discord_id: String,
        telegram_id: String,
        score: u64,
        signature: [u8; 64],
        recovery_id: u8
    ) -> Result<()> {
        msg!("Updating SBT info...");

        sbt_mint::update_sbt_info(
            ctx,
            name,
            photo,
            twitter_id,
            discord_id,
            telegram_id,
            score,
            signature,
            recovery_id
        );

        msg!("SBT info updated successfully.");

        Ok(())
    }
}

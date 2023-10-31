use anchor_lang::prelude::*;

declare_id!("2y4X3bWvSQPMGcvfMxGPM23fCQNHv6M4F5d5pJ9V3daG");

const COUNTER_SEED: &str = "oodles_technologies_counter";

#[program]
pub mod solana_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;

        counter_account.owner = ctx.accounts.initializer.key();
        counter_account.counter = 0;
        counter_account.bump = *ctx.bumps.get("counter_account").unwrap();

        Ok(())
    }

    pub fn increment_counter(ctx: Context<Update>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;

        counter_account.counter += 1;

        Ok(())
    }

    pub fn decrement_counter(ctx: Context<Update>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;

        require!(counter_account.counter > 0, ErrorCode::InvalidCount);

        counter_account.counter -= 1;

        Ok(())
    }

    pub fn remove_counter(_ctx: Context<Remove>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = initializer,
        seeds = [
            COUNTER_SEED.as_bytes(),
            initializer.key.as_ref()
        ],
        bump,
        space = 8 + Counter::INIT_SPACE
      )]
    pub counter_account: Account<'info, Counter>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(
        mut,
        seeds = [
            COUNTER_SEED.as_bytes(),
            updater.key.as_ref()
        ],
        bump = counter_account.bump,
      )]
    pub counter_account: Account<'info, Counter>,

    #[account(
        mut,
        constraint = updater.key() == counter_account.owner @ ErrorCode::AccessDenied
    )]
    pub updater: Signer<'info>,
}

#[derive(Accounts)]
pub struct Remove<'info> {
    #[account(
        mut,
        seeds = [
            COUNTER_SEED.as_bytes(),
            remover.key.as_ref()
        ],
        bump = counter_account.bump,
        close = remover
      )]
    pub counter_account: Account<'info, Counter>,

    #[account(
        mut,
        constraint = remover.key() == counter_account.owner @ ErrorCode::AccessDenied
    )]
    pub remover: Signer<'info>,
}

#[account]
#[derive(Default, InitSpace)]
pub struct Counter {
    pub owner: Pubkey,
    pub counter: u64,
    pub bump: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Access Denied")]
    AccessDenied,
    #[msg("Invalid Count")]
    InvalidCount,
}

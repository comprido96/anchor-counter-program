use anchor_lang::prelude::*;

declare_id!("7fFioY3q8JCPsPCrZ44hVE8x28YoQHch7KCJ8svyDCZW");

#[program]
pub mod counter_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        counter.count = 0;

        msg!("Counter initialized!");
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        msg!("Previous counter: {}", counter.count);

        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Updated counter: {}", counter.count);

        Ok(())
    }

    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        msg!("Previous counter: {}", counter.count);

        counter.count = counter.count.checked_sub(1).unwrap();
        msg!("Updated counter: {}", counter.count);

        Ok(())
    }
}

#[account]
pub struct Counter {
    pub count: u64
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8+8)]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,
}
#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("6coNTHYvSbigb5M9sjynfpxp6hCvpCjySr4tmsRmtr1a");

#[program]
pub mod crud_dapp {

    use super::*;

    pub fn create_journal_entry(ctx: Context<CreateEntry>, title: String, message: String) -> Result<()> {
      let journal_entry = &mut ctx.accounts.journal_entry;
      journal_entry.owner = ctx.accounts.owner.key();
      journal_entry.title = title;
      journal_entry.message = message;
      Ok(())
    }

    pub fn update_journal_entry(ctx: Context<UpdateEntry>, title: String, message: String) -> Result<()> {
      let journal_entry = &mut ctx.accounts.journal_entry;
      journal_entry.title = title;
      journal_entry.message = message;
      Ok(())
    }

    pub fn delete_journal_entry(_ctx: Context<DeleteEntry>) -> Result<()> {
      Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String, message: String)]
pub struct CreateEntry<'info> {
    #[account(
        init,
        seeds = [title.as_bytes(), owner.key().as_ref()], 
        bump, 
        payer = owner, 
        space = 8 + 32 + 4 + title.len() + 4 + message.len()
    )]
    pub journal_entry: Account<'info, JournalEntryState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String, message: String)]
pub struct UpdateEntry<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()], 
        bump, 
        realloc = 8 + 32 + 4 + title.len() + 4 + message.len(),
        realloc::payer = owner, 
        realloc::zero = true, 
    )]
    pub journal_entry: Account<'info, JournalEntryState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteEntry<'info> {
    #[account( 
        mut, 
        seeds = [title.as_bytes(), owner.key().as_ref()], 
        bump, 
        close= owner,
    )]
    pub journal_entry: Account<'info, JournalEntryState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct JournalEntryState {
    pub owner: Pubkey,
    #[max_len(50)]
    pub title: String,
    #[max_len(1000)]
    pub message: String,
}

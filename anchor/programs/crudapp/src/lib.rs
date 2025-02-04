#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("EU4HRU8XEvF3HhzmUvJ9n4o8oNxRAF9TLMHB8Yhytb9d");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod crudapp {
  use super::*;

  // Create a new journal entry
  pub fn create_journal_entry(
    context: Context<CreateJournalEntry>,
    title: String,
    message: String
  ) -> Result<()> {
    let journal_entry = &mut context.accounts.journal_entry;
    journal_entry.owner = *context.accounts.owner.key;
    journal_entry.title = title;
    journal_entry.message = message;

    Ok(())
  }

  // Update a journal entry
  pub fn update_journal_entry(
    context: Context<UpdateJournalEntry>,
    _title: String,
    message: String
  ) -> Result<()> {
    let journal_entry = &mut context.accounts.journal_entry;
    journal_entry.message = message;

    Ok(())
  }

  // Delete a journal entry
  pub fn delete_journal_entry(
    _context: Context<DeleteJournalEntry>,
    _title: String
  ) -> Result<()> {
    // context.accounts.journal_entry.owner = Pubkey::default();
    // context.accounts.journal_entry.title = "".to_string();
    // context.accounts.journal_entry.message = "".to_string();

    Ok(())
  }
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateJournalEntry<'info> {
  #[account(
    init,
    payer = owner,
    space = ANCHOR_DISCRIMINATOR_SIZE + JournalEntryState::INIT_SPACE,
    seeds = [title.as_bytes(), owner.key().as_ref()],
    bump,
  )]
  pub journal_entry: Account<'info, JournalEntryState>,

  #[account(mut)]
  pub owner: Signer<'info>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateJournalEntry<'info> {
  #[account(
    mut,
    seeds = [title.as_bytes(), owner.key().as_ref()],
    bump,
    realloc = ANCHOR_DISCRIMINATOR_SIZE + JournalEntryState::INIT_SPACE,
    realloc::payer = owner,
    // INFO: To change the original space back to zero and do reallocation of the space
    realloc::zero = true,
  )]
  pub journal_entry: Account<'info, JournalEntryState>,

  #[account(mut)]
  pub owner: Signer<'info>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteJournalEntry<'info> {
  #[account(
    mut,
    seeds = [title.as_bytes(), owner.key().as_ref()],
    bump,
    // INFO: To close the account if the public key that is specified in the seeds is the same as the signer of the instruction
    close = owner,
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
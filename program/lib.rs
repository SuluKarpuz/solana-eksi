use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use solana_program::entrypoint::ProgramResult;

declare_id!("7z6RVtwLczdagawf34Hh3QdpfjQLZ3ydDXrr9rkSUHXq");

#[program]
pub mod solana_eksi {
    use super::*;
    pub fn send_entry(ctx: Context<SendEntry>, topic: String, content: String) -> ProgramResult {
        let entry: &mut Account<Tweet> = &mut ctx.accounts.tweet;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();

        entry.author = *author.key;
        entry.timestamp = clock.unix_timestamp;
        entry.topic = topic;
        entry.content = content;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendEntry<'info> {
    #[account(init, payer = author, space = Entry::LEN)]
    pub entry: Account<'info, Tweet>,
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[account]
pub struct Entry {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4; 

impl Entry {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH 
        + TIMESTAMP_LENGTH 
        + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH 
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH; 
}

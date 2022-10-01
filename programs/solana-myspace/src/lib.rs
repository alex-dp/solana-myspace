use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_myspace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn make_post(ctx: Context<MakePost>, text:String) -> Result<()> {
        let new_post = &mut ctx.accounts.new_post;
        let account = &mut ctx.accounts.personal_account;
        new_post.author = account.authority;
        new_post.text = text;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    personal_account: Account<'info, PersonalAccount>,
}

#[derive(Accounts)]
pub struct MakePost<'info> {
    personal_account: Account<'info, PersonalAccount>,
    new_post: Account<'info, Post>
}

#[account]
pub struct PersonalAccount {
    authority: Pubkey,
    most_recent_post: Post
}


#[account]
pub struct Post {
    author: Pubkey,
    text: String,
    next_post: Pubkey
}
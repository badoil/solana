use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod programs {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateState<'info> {
    #[account(
        init,
        seeds = [b"state".as_ref()],
        bump,
        payer = authority,
        space = size_of::<StateAccount>() + 8
    )]
    pub state: Account<'info, StateAccount>, 

    //Authority, this is signer who paid transaction fee
    #[account(mut)]
    pub authority: Signer<'info>,

    //system program
    pub system_program: UncheckedAccount<'info>,

    //token program
    #[account(constraint = token_program.key == &token::Id)]
    pub token_program: Program<'info, Token>,
}  

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(
        mut,
        seeds = [b"state".as_ref()],
        bump,
    )]
    pub state: Account<'info, StateAccount>, 

    // authenticate post account
    #[account(
        init,
        seeds = [b"post".as_ref(), post_count.to_be_bytes().as_ref()],
        bump,
        payer = authority,
        space = size_of::<PostAccount>() + 8
    )]
    

}

#[account]
pub struct StateAccount {
    pub authority: Pubkey,
    pub post_count: u64,
}


// post account structure
#[account]
pub struct PostAccount {
    pub authority: Pubkey,

    pub text: String,

    pub poster_name: String,

    pub poster_url: String,

    pub comment_count: u64, 

    pub index: u64,

    pub post_time: i64,
}
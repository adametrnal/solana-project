use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_project {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        //Get a reference to the account
        let base_account = &mut ctx.accounts.base_account;
        // Initialize total_gifs
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>) -> ProgramResult {
        //Get a reference to the account and increment total_gifs
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs += 1;
        Ok(())
    }
}

// Attach variables to the context
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

//Tell Solana what we want to store on this account
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
}

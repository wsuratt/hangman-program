use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use std::str::FromStr;
use anchor_lang::solana_program::system_instruction;

declare_id!("8VxWJzmYtVrC755tFjQGMLhAN3hgPfCNPReEtN3wBzYz");

#[program]
pub mod hangman_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        ctx.accounts.pool.win = 1;
        ctx.accounts.pool.loss = 1;
        Ok(())
    }

    pub fn wager(ctx: Context<Wager>) -> ProgramResult {
        let owner: &Signer = &ctx.accounts.owner;
        let pool_amount: u64 = 50000000;
        // let fee_amount: u64 = 10000000;
        let target_word: String = String::from("********");
        let word: String = String::from("********");

        // wager pool
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.owner.key(), // from
            &ctx.accounts.pool.key(), // to
            pool_amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.owner.to_account_info(), //from
                ctx.accounts.pool.to_account_info(), //to
            ],
        );

        // fee
        // let ix = anchor_lang::solana_program::system_instruction::transfer(
        //     &ctx.accounts.owner.key(), // from
        //     &ctx.accounts.admin.key(), // to
        //     fee_amount,
        // );
        // anchor_lang::solana_program::program::invoke(
        //     &ix,
        //     &[
        //         ctx.accounts.owner.to_account_info(), //from
        //         ctx.accounts.admin.to_account_info(), //to
        //     ],
        // );

        Ok(())
    }

    pub fn end_game(ctx: Context<EndGame>, won: bool) -> ProgramResult {
        let pool: &mut Account<Pool> = &mut ctx.accounts.pool;
        let from_account = pool.to_account_info();
        let player_account = ctx.accounts.owner.to_account_info();
        let admin_account = ctx.accounts.admin.to_account_info();

        // if **from_account.try_borrow_lamports()? < amount_of_lamports {
        //     return Err(ErrorCode::ContentTooLong.into());
        // }

        // send winnings
        if (won)
        {
            let wins = pool.win as f64;
            let losses = pool.loss as f64;
            let lamports = 50000000 as f64;

            pool.win += 1;
            let fee_amount: u64 = ((((losses/wins) * lamports) * 0.8) * 0.1) as u64;
            let win_amount: u64 = (((((losses/wins) * lamports) * 0.8) + lamports) - fee_amount as f64) as u64;
            **from_account.try_borrow_mut_lamports()? -= win_amount;
            **player_account.try_borrow_mut_lamports()? += win_amount;

            // fees
            **from_account.try_borrow_mut_lamports()? -= fee_amount;
            **admin_account.try_borrow_mut_lamports()? += fee_amount;
        }
        else
        {
            pool.loss += 1;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, seeds = [b"hangman_solwager".as_ref()], space = 8 + 8 + 8, bump, payer = owner)]
    pool: Account<'info, Pool>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Wager<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    // #[account(mut, address = Pubkey::from_str("8WnqfBUM4L13fBUudvjstHBEmUcxTPPX7DGkg3iyMmc8").unwrap())]
    // pub admin: AccountInfo<'info>,
    #[account(mut, seeds = [b"hangman_solwager".as_ref()], bump)]
    pub pool: Account<'info, Pool>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EndGame<'info> {
    #[account(mut, seeds = [b"hangman_solwager".as_ref()], bump)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub owner: AccountInfo<'info>,
    #[account(mut, address = Pubkey::from_str("8WnqfBUM4L13fBUudvjstHBEmUcxTPPX7DGkg3iyMmc8").unwrap())]
    pub admin: AccountInfo<'info>,
    #[account(mut, address = Pubkey::from_str("8ECeRHmzdQKE3sBLZ5r8wAoEWfAVNEupJRu3EtkXvA4Q").unwrap())]
    pub server: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct Pool {
    pub win: i64,
    pub loss: i64,
}

#[error]
pub enum ErrorCode {
    #[msg("The provided topic should be 50 characters long maximum.")]
    TopicTooLong,
    #[msg("The provided content should be 280 characters long maximum.")]
    ContentTooLong,
}

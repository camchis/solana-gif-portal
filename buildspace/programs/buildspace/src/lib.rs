use anchor_lang::prelude::*;

declare_id!("EEgTUKc9SVhcq1BG1ruWDB6Mgt1MFGrmXe4DVnpZchkj");

#[program]
pub mod buildspace {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    base_account.total_gifs = 0;
    Ok(())
  }

  pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

    let len = base_account.gif_list.len();

    let item = ItemStruct {
      gif_link: gif_link.to_string(),
      votes: 0,
      user_address: *user.to_account_info().key,
      id: len as u32,
    };
    base_account.gif_list.push(item);
    base_account.total_gifs += 1;
    Ok(())
  }

  pub fn add_vote(ctx: Context<AddVote>, id: u32) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    let _user = &mut ctx.accounts.user;

    let mut gif = &mut base_account.gif_list[id as usize];
    gif.votes += 1;

    Ok(())
  }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
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
  #[account(mut)]
  pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct AddVote<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
  pub gif_link: String,
  pub votes: u64,
  pub user_address: Pubkey,
  pub id: u32,
}

#[account]
pub struct BaseAccount {
  pub total_gifs: u64,
  pub gif_list: Vec<ItemStruct>,
}
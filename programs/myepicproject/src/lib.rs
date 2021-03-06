use anchor_lang::prelude::*;

declare_id!("9HpujVCStmUC4o7jk4UtZuf3JcirjcRWgwRW6aGfVXpb");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    base_account.total_gifs = 0;
    Ok(())
  }

  // The function now accepts a gif_link param from the user. We also reference the user from the Context
  pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

	// Build the struct.
    let item = ItemStruct {
      gif_link: gif_link.to_string(),
      user_address: *user.to_account_info().key,
      up_voters: Vec::new(),
      down_voters: Vec::new()
    };
		
	// Add it to the gif_list vector.
    base_account.gif_list.push(item);
    base_account.total_gifs += 1;
    Ok(())
  }

  pub fn upvote_gif(ctx: Context<VoteGif>, gif_link: String, gif_uploader: Pubkey) -> ProgramResult {
      let base_account = &mut ctx.accounts.base_account;
      let user = &mut ctx.accounts.user;
      
      for gif in &mut base_account.gif_list {
          if gif.gif_link == gif_link && gif.user_address == gif_uploader{

              if gif.up_voters.contains(&*user.to_account_info().key) {
                panic!("You already upvoted this gif")
              }

              if gif.down_voters.contains(&*user.to_account_info().key){
                gif.down_voters.retain(|&x| x != *user.to_account_info().key);
              }
              gif.up_voters.push(*user.to_account_info().key) 
          }
      }

      Ok(())
  }

    pub fn downvote_gif(ctx: Context<VoteGif>, gif_link: String, gif_uploader: Pubkey) -> ProgramResult {
      let base_account = &mut ctx.accounts.base_account;
      let user = &mut ctx.accounts.user;
      
      for gif in &mut base_account.gif_list {
          if gif.gif_link == gif_link && gif.user_address == gif_uploader{

              if gif.down_voters.contains(&*user.to_account_info().key) {
                panic!("You already downvoted this gif")
              }

              if gif.up_voters.contains(&*user.to_account_info().key){
                gif.up_voters.retain(|&x| x != *user.to_account_info().key);
              }

              gif.down_voters.push(*user.to_account_info().key) 
          }
      }

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

// Add the signer who calls the AddGif method to the struct so that we can save it
#[derive(Accounts)]
pub struct AddGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct VoteGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub up_voters: Vec<Pubkey>,
    pub down_voters: Vec<Pubkey>,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
	// Attach a Vector of type ItemStruct to the account.
    pub gif_list: Vec<ItemStruct>,
}
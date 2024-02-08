use anchor_lang::prelude::*;
use anchor_lang::prelude::Pubkey;
use anchor_spl::token::{TokenAccount, Token, Mint, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use std::mem::size_of;

use crate::account::*;
use crate::constants::*;


#[derive(Accounts)]
pub struct CreateUserContext<'info> {
  #[account(mut)]
  pub owner: Signer<'info>,
  #[account(init, seeds = [
    b"user_", 
    owner.key().as_ref()], 
    bump, 
    payer = owner, 
    space = size_of::<User>() + 8
  )]
  pub user: AccountLoader<'info, User>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct CreateAdminContext<'info> {
  #[account(mut, constraint = owner.key() == ADMIN_KEY)]
  pub owner: Signer<'info>,
  #[account(mut)]
    /// CHECK: it's not dangerous
  pub admin: AccountInfo<'info>,
  #[account(mut)]
  pub user: AccountLoader<'info, User>,
  pub system_program: Program<'info, System>
}


#[derive(Accounts)]
pub struct DeleteAdminContext<'info> {
  #[account(mut, constraint = owner.key() == ADMIN_KEY)]
  pub owner: Signer<'info>,
  #[account(mut)]
    /// CHECK: it's not dangerous
  pub admin: AccountInfo<'info>,
  #[account(mut)]
  pub user: AccountLoader<'info, User>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct FeeContext<'info> {
    #[account(init_if_needed, seeds = [b"raffle_fee"], bump, payer = admin, space = 8 + 304)]
    pub fee_account: Account<'info, RaffleState>,
    #[account(mut)]
    admin: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(raffle_id: u64)]
pub struct CreateRaffleContext<'info> {
  // #[account(mut, constraint = admin.key() == ADMIN_KEY)]
  #[account(mut)]
  pub admin: Signer<'info>,
  #[account(init, seeds = [
    POOL_SEED.as_bytes(), 
    &raffle_id.to_le_bytes(), 
    mint.key().as_ref()], 
    bump, 
    payer = admin, 
    space = size_of::<Pool>() + 8
  )]
  pub pool: AccountLoader<'info, Pool>,
  /// CHECK: This is not dangerous because we don't read or write from this account
  #[account(mut)] //  constraint = treasury.key() == WITHDRAW_KEY
  pub treasury: AccountInfo<'info>,
  pub fee_account: Account<'info, RaffleState>,
  pub mint: Account<'info, Mint>,
  #[account(mut, constraint = ata_from.mint == mint.key() && ata_from.owner == admin.key())]
  pub ata_from: Account<'info, TokenAccount>,
  #[account(
    init_if_needed,
    payer = admin,
    associated_token::mint = mint,
    associated_token::authority = pool
  )]
  pub ata_to: Account<'info, TokenAccount>,
  pub token_program: Program<'info, Token>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>
}

impl<'info> CreateRaffleContext<'info> {
  pub fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
      let cpi_accounts = Transfer {
          from: self.ata_from.to_account_info().clone(),
          to: self.ata_to.to_account_info().clone(),
          authority: self.admin.to_account_info().clone(),
      };
      CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
  }
}

#[derive(Accounts)]
pub struct EditRaffleContext<'info> {
  #[account(mut, constraint = admin.key() == ADMIN_KEY)]
  pub admin: Signer<'info>,
  #[account(mut)]
  pub pool: AccountLoader<'info, Pool>
}


#[derive(Accounts)]
pub struct DeleteRaffleContext<'info> {
  #[account(mut, constraint = admin.key() == ADMIN_KEY)]
  pub admin: Signer<'info>,
  #[account(mut)]
  pub pool: AccountLoader<'info, Pool>,
  pub mint: Account<'info, Mint>,
  #[account(mut, constraint = ata_from.mint == mint.key() && ata_from.owner == pool.key())]
  pub ata_from: Account<'info, TokenAccount>,
  #[account(
    init_if_needed,
    payer = admin,
    associated_token::mint = mint,
    associated_token::authority = admin
  )]
  pub ata_to: Account<'info, TokenAccount>,
  pub token_program: Program<'info, Token>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>
}

impl<'info> DeleteRaffleContext<'info> {
  pub fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
      let cpi_accounts = Transfer {
          from: self.ata_from.to_account_info().clone(),
          to: self.ata_to.to_account_info().clone(),
          authority: self.pool.to_account_info().clone(),
      };
      CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
  }
}

#[derive(Accounts)]
pub struct BuyTicketContext<'info> {
  #[account(mut)]
  pub buyer: Signer<'info>,
    /// CHECK: it's not dangerous
  #[account(mut, constraint = community.key() == COMMUNITY_KEY)]
  pub community: AccountInfo<'info>,
  #[account(mut)]
  pub pool: AccountLoader<'info, Pool>,
  /// CHECK: This is not dangerous because we don't read or write from this account
  #[account(mut)] //  constraint = treasury.key() == WITHDRAW_KEY)
  pub treasury: AccountInfo<'info>,
  pub fee_account: Account<'info, RaffleState>,
  pub mint: Account<'info, Mint>,
  #[account(constraint = pay_mint.key() == PAY_TOKEN)]
  pub pay_mint: Account<'info, Mint>,
  #[account(mut, constraint = mint_token.mint == mint.key() && mint_token.owner == buyer.key())]
  pub mint_token: Account<'info, TokenAccount>,
  #[account(mut, constraint = ata_from.mint == pay_mint.key() && ata_from.owner == buyer.key())]
  pub ata_from: Account<'info, TokenAccount>,
  #[account(
    init_if_needed,
    payer = buyer,
    associated_token::mint = pay_mint,
    associated_token::authority = community
  )]
  pub ata_to: Box<Account<'info, TokenAccount>>,
  /// CHECK: it's not dangerous
  pub metadata: AccountInfo<'info>,
  pub token_program: Program<'info, Token>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent> 
}

impl<'info> BuyTicketContext<'info> {
  pub fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
      let cpi_accounts = Transfer {
          from: self.ata_from.to_account_info().clone(),
          to: self.ata_to.to_account_info().clone(),
          authority: self.buyer.to_account_info().clone(),
      };
      CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
  }
}

#[derive(Accounts)]
pub struct SetWinnerContext<'info> {
  #[account(mut)]
  pub admin: Signer<'info>,
  #[account(mut)]
  pub pool: AccountLoader<'info, Pool>,
  /// CHECK: it's not dangerous
  #[account(constraint = slothash.key() == SLOTHASH_PUBKEY)]
  pub slothash: AccountInfo<'info>
}

#[derive(Accounts)]
pub struct ClaimPrizeContext<'info> {
  #[account(mut)]
  pub buyer: Signer<'info>,
  #[account(mut)]
  pub pool: AccountLoader<'info, Pool>,
  /// CHECK: This is not dangerous because we don't read or write from this account
  #[account(mut)] //  constraint = treasury.key() == WITHDRAW_KEY
  pub treasury: AccountInfo<'info>,
  pub fee_account: Account<'info, RaffleState>,
  pub mint: Account<'info, Mint>,
  #[account(mut, constraint = ata_from.mint == mint.key() && ata_from.owner == pool.key())]
  pub ata_from: Account<'info, TokenAccount>,
  #[account(
    init_if_needed,
    payer = buyer,
    associated_token::mint = mint,
    associated_token::authority = buyer
  )]
  pub ata_to: Account<'info, TokenAccount>,
  pub token_program: Program<'info, Token>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent> 
}

impl<'info> ClaimPrizeContext<'info> {
  pub fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
    let cpi_accounts = Transfer {
        from: self.ata_from.to_account_info().clone(),
        to: self.ata_to.to_account_info().clone(),
        authority: self.pool.to_account_info().clone(),
    };
    CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
  }
}

#[derive(Accounts)]
pub struct ClaimCoodeContext<'info> {
  #[account(mut, constraint = admin.key() == ADMIN_KEY)]
  pub admin: Signer<'info>,
  #[account(mut)]
  pub pool: AccountLoader<'info, Pool>,
  /// CHECK: This is not dangerous because we don't read or write from this account
  #[account(mut)] //  constraint = treasury.key() == WITHDRAW_KEY
  pub treasury: AccountInfo<'info>,
  pub pay_mint: Account<'info, Mint>,
  #[account(mut, constraint = ata_from.mint == pay_mint.key() && ata_from.owner == pool.key())]
  pub ata_from: Account<'info, TokenAccount>,
  #[account(
    init_if_needed,
    payer = admin,
    associated_token::mint = pay_mint,
    associated_token::authority = admin
  )]
  pub ata_to: Account<'info, TokenAccount>,
  pub token_program: Program<'info, Token>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent> 
}

impl<'info> ClaimCoodeContext<'info> {
  pub fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
    let cpi_accounts = Transfer {
        from: self.ata_from.to_account_info().clone(),
        to: self.ata_to.to_account_info().clone(),
        authority: self.pool.to_account_info().clone(),
    };
    CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
  }
}

#[derive(Accounts)]
pub struct SendBackNftContext<'info> {
  #[account(mut)]
  pub partner: Signer<'info>,
  /// CHECK: it's not dangerous
  #[account(mut)]
  pub admin: AccountInfo<'info>,
  #[account(mut)]
  pub pool: AccountLoader<'info, Pool>,
  pub mint: Account<'info, Mint>,
  #[account(mut, constraint = ata_from.mint == mint.key() && ata_from.owner == pool.key())]
  pub ata_from: Account<'info, TokenAccount>,
  #[account(
    init_if_needed,
    payer = partner,
    associated_token::mint = mint,
    associated_token::authority = admin
  )]
  pub ata_to: Account<'info, TokenAccount>,
  pub token_program: Program<'info, Token>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent> 
}

impl<'info> SendBackNftContext<'info> {
  pub fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
    let cpi_accounts = Transfer {
        from: self.ata_from.to_account_info().clone(),
        to: self.ata_to.to_account_info().clone(),
        authority: self.pool.to_account_info().clone(),
    };
    CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
  }
}

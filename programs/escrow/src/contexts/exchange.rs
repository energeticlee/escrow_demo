use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::state::Escrow;

#[derive(Accounts)]
pub struct Exchange<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub initializer: SystemAccount<'info>,
    pub offering_mint: Box<Account<'info, Mint>>,
    pub asking_mint: Box<Account<'info, Mint>>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = offering_mint,
        associated_token::authority = taker
    )]
    pub taker_offer_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = asking_mint,
        associated_token::authority = taker
    )]
    pub taker_asking_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = asking_mint,
        associated_token::authority = initializer
    )]
    pub initializer_asking_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        has_one = asking_mint,
        constraint = taker_asking_ata.amount >= escrow.asking_amount,
        seeds = [Escrow::seed(), random_seed.key().as_ref()],
        bump,
        close = initializer,
    )]
    pub escrow: Account<'info, Escrow>,
    /// CHECK: Escrow seed
    pub random_seed: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint = offering_mint,
        associated_token::authority = escrow
    )]
    pub vault: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

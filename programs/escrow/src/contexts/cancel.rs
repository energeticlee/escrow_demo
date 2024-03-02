use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::state::Escrow;

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub offering_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = offering_mint,
        associated_token::authority = initializer
    )]
    pub initializer_offer_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        has_one = initializer,
        has_one = offering_mint,
        close = initializer,
        seeds = [Escrow::seed(), random_seed.key().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,
    /// CHECK: Escrow seed
    pub random_seed: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint = offering_mint,
        associated_token::authority = escrow,
    )]
    pub vault: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

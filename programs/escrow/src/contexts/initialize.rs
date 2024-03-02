use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(offering_mint_amount: u64)]
pub struct Initialize<'info> {
    // initializer & payer for any rent and transaction fee
    #[account(mut)]
    pub initializer: Signer<'info>,
    // What the initializer is offering
    pub offering_mint: Account<'info, Mint>,
    // What the initializer want
    pub asking_mint: Account<'info, Mint>,
    // Initializer offer ATA, to deposit to vault
    #[account(
        mut,
        constraint = initializer_offer_ata.amount >= offering_mint_amount,
        associated_token::mint = offering_mint,
        associated_token::authority = initializer
    )]
    pub initializer_offer_ata: Account<'info, TokenAccount>,
    // Escrow that holds state
    #[account(
        init_if_needed,
        payer = initializer,
        seeds = [Escrow::seed(), random_seed.key().as_ref()],
        bump,
        space = 8 + std::mem::size_of::<Escrow>(),
    )]
    pub escrow: Account<'info, Escrow>,
    /// CHECK: Escrow seed
    pub random_seed: AccountInfo<'info>,
    // Vault is an offer ATA that belongs to the escrow PDA.
    #[account(
        init_if_needed,
        payer = initializer,
        associated_token::mint = offering_mint,
        associated_token::authority = escrow
    )]
    pub vault: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

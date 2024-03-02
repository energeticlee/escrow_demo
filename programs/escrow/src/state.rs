use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    // Escrow Seed
    pub random_seed: Pubkey,
    // The initializer pubkey
    pub initializer: Pubkey,
    // Initializer offer
    pub offering_mint: Pubkey,
    // What the initializer is offering
    pub offering_amount: u64,
    // What the initializer want
    pub asking_mint: Pubkey,
    // What the initializer want for the offered amount
    pub asking_amount: u64,
    // Initialize date
    pub initialize_date: u64,
}

impl Escrow {
    pub fn seed<'s>() -> &'s [u8] {
        b"escrow"
    }
}

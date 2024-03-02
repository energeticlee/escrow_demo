use anchor_lang::prelude::*;
use anchor_spl::token::transfer_checked;

mod contexts;
mod state;

use contexts::*;

declare_id!("BzQwpkP1J3CjpUqJttYnzPLhoeskbt6jcMtehCVf73Cb");

#[program]
pub mod escrow {
    use anchor_spl::token::{close_account, CloseAccount, TransferChecked};

    use self::state::Escrow;

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        offering_amount: u64,
        asking_amount: u64,
    ) -> Result<()> {
        let initializer = &mut ctx.accounts.initializer;
        let offering_mint = &ctx.accounts.offering_mint;
        let escrow = &mut ctx.accounts.escrow;

        escrow.random_seed = ctx.accounts.random_seed.key();
        escrow.initializer = initializer.key();
        escrow.offering_mint = offering_mint.key();
        escrow.offering_amount = offering_amount;
        escrow.asking_mint = ctx.accounts.asking_mint.key();
        escrow.asking_amount = asking_amount;
        escrow.initialize_date = Clock::get()?.unix_timestamp as u64;

        let cpi_accounts = TransferChecked {
            from: ctx.accounts.initializer_offer_ata.to_account_info(),
            mint: offering_mint.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: initializer.to_account_info(),
        };
        let transfer_ctx =
            CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

        transfer_checked(transfer_ctx, offering_amount, offering_mint.decimals)?;
        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        let initializer = &ctx.accounts.initializer;
        let escrow = &ctx.accounts.escrow;
        let vault = &ctx.accounts.vault;
        let offering_mint = &ctx.accounts.offering_mint;

        // PDA REQUIRE SEED SIGNER
        let signer_seeds: &[&[&[u8]]] = &[&[
            Escrow::seed(),
            escrow.random_seed.as_ref(),
            &[ctx.bumps.escrow],
        ]];

        // TRANSNFER VALUE BACK FROM VAULT TO INITIALIZER
        let transfer_cpi_accounts = TransferChecked {
            from: vault.to_account_info(),
            mint: offering_mint.to_account_info(),
            to: ctx.accounts.initializer_offer_ata.to_account_info(),
            authority: escrow.to_account_info(),
        };
        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_cpi_accounts,
            signer_seeds,
        );
        transfer_checked(transfer_ctx, escrow.offering_amount, offering_mint.decimals)?;

        // CLOSE VAULT AND TRANSFER RENT TO INITIALIZER
        let close_cpi_accounts = CloseAccount {
            account: vault.to_account_info(),
            destination: initializer.to_account_info(),
            authority: escrow.to_account_info(),
        };
        let close_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            close_cpi_accounts,
            signer_seeds,
        );
        close_account(close_ctx)?;

        Ok(())
    }

    pub fn exchange(ctx: Context<Exchange>) -> Result<()> {
        let initializer = &ctx.accounts.initializer;
        let escrow = &ctx.accounts.escrow;
        let vault = &ctx.accounts.vault;
        let offering_mint = &ctx.accounts.offering_mint;
        let asking_mint = &ctx.accounts.asking_mint;
        let token_program = &ctx.accounts.token_program;
        // PDA REQUIRE SEED SIGNER
        let signer_seeds: &[&[&[u8]]] = &[&[
            Escrow::seed(),
            escrow.random_seed.as_ref(),
            &[ctx.bumps.escrow],
        ]];

        msg!("TEST 1");
        // BUYER TRANSFER ASKING_AMOUNT ASKING_MINT TO INITIALIZER
        let taker_transfer_accounts = TransferChecked {
            from: ctx.accounts.taker_asking_ata.to_account_info(),
            mint: asking_mint.to_account_info(),
            to: ctx.accounts.initializer_asking_ata.to_account_info(),
            authority: ctx.accounts.taker.to_account_info(),
        };
        msg!("TEST 1.1");
        let taker_transfer_ctx =
            CpiContext::new(token_program.to_account_info(), taker_transfer_accounts);
        msg!("TEST 1.2");
        transfer_checked(
            taker_transfer_ctx,
            escrow.asking_amount,
            asking_mint.decimals,
        )?;
        msg!("TEST 2");

        // TRANSFER OFFER_AMOUNT OFFER_MINT TO BUYER
        let vault_transfer_accounts = TransferChecked {
            from: vault.to_account_info(),
            mint: offering_mint.to_account_info(),
            to: ctx.accounts.taker_offer_ata.to_account_info(),
            authority: escrow.to_account_info(),
        };
        let taker_transfer_ctx = CpiContext::new_with_signer(
            token_program.to_account_info(),
            vault_transfer_accounts,
            signer_seeds,
        );
        transfer_checked(
            taker_transfer_ctx,
            escrow.offering_amount,
            offering_mint.decimals,
        )?;

        msg!("TEST 3");
        // CLOSE VAULT AND TRANSFER RENT TO INITIALIZER
        let close_cpi_accounts = CloseAccount {
            account: vault.to_account_info(),
            destination: initializer.to_account_info(),
            authority: escrow.to_account_info(),
        };
        let close_ctx = CpiContext::new_with_signer(
            token_program.to_account_info(),
            close_cpi_accounts,
            signer_seeds,
        );
        close_account(close_ctx)?;

        Ok(())
    }
}

use anchor_lang::prelude::*;

use solana_address_lookup_table_program;

declare_id!("GCxLFEE3BkEUdEnPyWMwZpQWQP9sXSq5wAsq7yR3Rrh");

#[program]
pub mod anchor_address_lookup_table_example {
    use super::*;

    pub fn create_raffle(ctx: Context<CreateRaffle>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateRaffle {
    #[account(init)]
    pub raffle: ProgramAccount<Raffle>,
    pub rent: Sysvar<Rent>,
}

#[account]
pub struct Raffle {
    admin: Pubkey,
    alt: Pubkey,
}

impl Space for Escrow {
    const INIT_SPACE: usize = 8 + 32 + 32 + 8 + 1;
}

#[error_code]
pub enum EscrowError {
    #[msg("Invalid instruction")]
    InvalidIx,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Invalid program")]
    InvalidProgram,
    #[msg("Invalid Maker ATA")]
    Inval
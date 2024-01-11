use anchor_lang::{
    prelude::*,
    solana_program::program::invoke,
};

use solana_address_lookup_table_program::{
    instruction::{create_lookup_table_signed, freeze_lookup_table, extend_lookup_table, deactivate_lookup_table, close_lookup_table},
    ID as ADDRESS_LOOKUP_TABLE_PROGRAM_ID,
};

declare_id!("ALT4bMa1vKx8fAEbrTYFE1cx86HyKV49LpaNHMBcQZTJ");

#[program]
pub mod anchor_address_lookup_table_example {
    use super::*;

    /// This function constructs an instruction to create an address lookup table account and returns
    /// the instruction and the table account's derived address.
    pub fn create_address_lookup_table(ctx: Context<CreateAddressLookupTable>, recent_slot: u64) -> Result<()> {

        let (create_table_instruction, table_key) =
        create_lookup_table_signed(
            ctx.accounts.authority.key(),
            ctx.accounts.payer.key(),
            // ALTs are made with recent_slot as part of the "seeds" because if it can be closed and re-initialized with new addresses, 
            // any client which is unaware of the change could inadvertently lookup unexpected addresses. To avoid this, all address lookup 
            // tables must be initialized at an address derived from a recent slot and they cannot be closed until the slot used for 
            // deactivation is no longer in the slot hashes sysvar.
            recent_slot, 
        );

        require_keys_eq!(ctx.accounts.lookup_table.key(), table_key, AltError::InvalidAddress);

        invoke(
            &create_table_instruction,
            &[
                ctx.accounts.lookup_table.to_account_info(),
                ctx.accounts.authority.to_account_info(),
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.address_lookup_table_program.to_account_info(),
            ],
        )?;

        Ok(())
    }
    
    /// This function constructs an instruction that permanently freeze an address lookup table so that 
    /// it can never be closed or extended again and making it immutable.
    /// NB: Empty lookup tables cannot be frozen.
    pub fn freeze_address_lookup_table(ctx: Context<FreezeAddressLookupTable>) -> Result<()> {

        let freeze_table_instruction = freeze_lookup_table(ctx.accounts.lookup_table.key(), ctx.accounts.authority.key());

        invoke(
            &freeze_table_instruction,
            &[
                ctx.accounts.lookup_table.to_account_info(),
                ctx.accounts.authority.to_account_info(),
                ctx.accounts.address_lookup_table_program.to_account_info(),
            ],
        )?;

        Ok(())
    }

    /// This function constructs an instruction that extends an address lookup table account with new addresses.
    /// NB: Funding account and system program account references are only required if the lookup table
    /// account requires additional lamports to cover the rent-exempt balance after being extended.
    pub fn extend_address_lookup_table(ctx: Context<ExtendAddressLookupTable>) -> Result<()> {

        let extend_table_instruction = extend_lookup_table(
            ctx.accounts.lookup_table.key(),
            ctx.accounts.authority.key(),
            Some(ctx.accounts.payer.key()),
            // NB: You can add max 20 address at a Time
            vec![ctx.accounts.publickey_1.key(), ctx.accounts.publickey_2.key(), ctx.accounts.publickey_3.key()], 
        );

        invoke(
            &extend_table_instruction,
            &[
                ctx.accounts.lookup_table.to_account_info(),
                ctx.accounts.authority.to_account_info(),
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.address_lookup_table_program.to_account_info(),
            ],
        )?;

        Ok(())
    }

    /// This function constructs an instruction that deactivates an address lookup table so that it cannot be extended again and 
    /// will be unusable and eligible for closure after a short amount of time.
    pub fn deactivate_address_lookup_table(ctx: Context<DeactivateAddressLookupTable>) -> Result<()> {

        let deactivate_table_instruction = deactivate_lookup_table(
            ctx.accounts.lookup_table.key(),
            ctx.accounts.authority.key(),
        );

        invoke(
            &deactivate_table_instruction,
            &[
                ctx.accounts.lookup_table.to_account_info(),
                ctx.accounts.authority.to_account_info(),
                ctx.accounts.address_lookup_table_program.to_account_info(),
            ],
        )?;

        Ok(())
    }

    /// This function constructs an instruction that closes an address lookup table account. 
    /// The account will be deallocated and the lamports will be drained to the recipient address.
    pub fn close_address_lookup_table(ctx: Context<CloseAddressLookupTable>) -> Result<()> {

        let close_lookup_table_instruction = close_lookup_table(
            ctx.accounts.lookup_table.key(),
            ctx.accounts.authority.key(),
            ctx.accounts.recipient.key(),
        );

        invoke(
            &close_lookup_table_instruction,
            &[
                ctx.accounts.lookup_table.to_account_info(),
                ctx.accounts.authority.to_account_info(),
                ctx.accounts.recipient.to_account_info(),
                ctx.accounts.address_lookup_table_program.to_account_info(),
            ],
        )?;

        Ok(())
    }
}


#[derive(Accounts)]
pub struct CreateAddressLookupTable<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    /// CHECK: the account will be validated by the lookup table program
    pub lookup_table: AccountInfo<'info>,

    #[account(address = ADDRESS_LOOKUP_TABLE_PROGRAM_ID)]
    /// CHECK: the account will be validated by the lookup table program
    pub address_lookup_table_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FreezeAddressLookupTable<'info> {
    pub authority: Signer<'info>,

    #[account(mut)]
    /// CHECK: the account will be validated by the lookup table program
    pub lookup_table: AccountInfo<'info>,

    #[account(address = ADDRESS_LOOKUP_TABLE_PROGRAM_ID)]
    /// CHECK: the account will be validated by the lookup table program
    pub address_lookup_table_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExtendAddressLookupTable<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    /// CHECK: the account will be validated by the lookup table program
    pub lookup_table: AccountInfo<'info>,

    /// CHECK: We don't care to check it since we're just putting it in the lookup table
    pub publickey_1: AccountInfo<'info>,
    /// CHECK: We don't care to check it since we're just putting it in the lookup table
    pub publickey_2: AccountInfo<'info>,
    /// CHECK: We don't care to check it since we're just putting it in the lookup table
    pub publickey_3: AccountInfo<'info>,

    #[account(address = ADDRESS_LOOKUP_TABLE_PROGRAM_ID)]
    /// CHECK: the account will be validated by the lookup table program
    pub address_lookup_table_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeactivateAddressLookupTable<'info> {
    pub authority: Signer<'info>,

    #[account(mut)]
    /// CHECK: the account will be validated by the lookup table program
    pub lookup_table: AccountInfo<'info>,

    #[account(address = ADDRESS_LOOKUP_TABLE_PROGRAM_ID)]
    /// CHECK: the account will be validated by the lookup table program
    pub address_lookup_table_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseAddressLookupTable<'info> {
    pub authority: Signer<'info>,

    #[account(mut)]
    /// CHECK: the account will be validated by the lookup table program
    pub lookup_table: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: We don't care to check it since it's just getting the funds from the closure of the ALT
    pub recipient: AccountInfo<'info>,

    #[account(address = ADDRESS_LOOKUP_TABLE_PROGRAM_ID)]
    /// CHECK: the account will be validated by the lookup table program
    pub address_lookup_table_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum AltError {
    #[msg("The address lookup table passed in as account doesn't match the one derived in the program")]
    InvalidAddress,
}
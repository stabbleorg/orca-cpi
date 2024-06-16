use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Swap<'info> {
    pub token_program: AccountInfo<'info>,
    #[account(mut)]
    pub token_authority: AccountInfo<'info>,
    #[account(mut)]
    pub whirlpool: AccountInfo<'info>,
    #[account(mut)]
    pub token_owner_account_a: AccountInfo<'info>,
    #[account(mut)]
    pub token_vault_a: AccountInfo<'info>,
    #[account(mut)]
    pub token_owner_account_b: AccountInfo<'info>,
    #[account(mut)]
    pub token_vault_b: AccountInfo<'info>,
    #[account(mut)]
    pub tick_array_0: AccountInfo<'info>,
    #[account(mut)]
    pub tick_array_1: AccountInfo<'info>,
    #[account(mut)]
    pub tick_array_2: AccountInfo<'info>,
    #[account(mut)]
    pub oracle: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TwoHopSwap<'info> {
    pub token_program: AccountInfo<'info>,
    #[account(mut)]
    pub token_authority: AccountInfo<'info>,
    #[account(mut)]
    pub whirlpool_one: AccountInfo<'info>,
    #[account(mut)]
    pub whirlpool_two: AccountInfo<'info>,
    #[account(mut)]
    pub token_owner_account_one_a: AccountInfo<'info>,
    #[account(mut)]
    pub token_vault_one_a: AccountInfo<'info>,
    #[account(mut)]
    pub token_owner_account_one_b: AccountInfo<'info>,
    #[account(mut)]
    pub token_vault_one_b: AccountInfo<'info>,
    #[account(mut)]
    pub token_owner_account_two_a: AccountInfo<'info>,
    #[account(mut)]
    pub token_vault_two_a: AccountInfo<'info>,
    #[account(mut)]
    pub token_owner_account_two_b: AccountInfo<'info>,
    #[account(mut)]
    pub token_vault_two_b: AccountInfo<'info>,
    #[account(mut)]
    pub tick_array_one_0: AccountInfo<'info>,
    #[account(mut)]
    pub tick_array_one_1: AccountInfo<'info>,
    #[account(mut)]
    pub tick_array_one_2: AccountInfo<'info>,
    #[account(mut)]
    pub tick_array_two_0: AccountInfo<'info>,
    #[account(mut)]
    pub tick_array_two_1: AccountInfo<'info>,
    #[account(mut)]
    pub tick_array_two_2: AccountInfo<'info>,
    #[account(mut)]
    pub oracle_one: AccountInfo<'info>,
    #[account(mut)]
    pub oracle_two: AccountInfo<'info>,
}

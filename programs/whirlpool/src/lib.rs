#![allow(unused)]

pub mod context;

use crate::context::*;
use anchor_lang::prelude::*;

declare_id!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");

#[program]
pub mod whirlpool {
    use super::*;

    pub fn swap(
        ctx: Context<Swap>,
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit: u128,
        amount_specified_is_input: bool,
        a_to_b: bool,
    ) -> Result<()> { Ok(()) }

    pub fn two_hop_swap(
        ctx: Context<TwoHopSwap>,
        amount: u64,
        other_amount_threshold: u64,
        amount_specified_is_input: bool,
        a_to_b_one: bool,
        a_to_b_two: bool,
        sqrt_price_limit_one: u128,
        sqrt_price_limit_two: u128,
    ) -> Result<()> { Ok(()) }
}

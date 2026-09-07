use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Deposit amount must be greater than zero")]
    InvalidAmount,
    #[msg("The vault does not have enough withdrawable lamports")]
    InsufficientFunds,
}

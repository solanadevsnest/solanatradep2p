use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("The stage specified is not valid for an exchange or cancellation.")]
    InvalidStage,
    #[msg("There are insufficient funds available for this action.")]
    InsufficientFunds,
    #[msg("The provided mint account is not appropriate for this trade.")]
    InvalidMint,
    #[msg("A necessary mint is absent for this trade.")]
    MissingMint,
    #[msg("The trade type is invalid, possibly due to missing mint addresses.")]
    InvalidTradeType,
    #[msg("There is an invalid mint association between the token accounts.")]
    InvalidAccount,
    #[msg("Duplicate mint accounts are not permitted.")]
    DuplicateMint,
    #[msg("The account does not possess a valid owner.")]
    InvalidOwner,
    #[msg("The specified partner is not suitable for this trade.")]
    InvalidPartner,
    #[msg("Both trade value and receive value must exceed zero.")]
    ZeroValue,
    #[msg("Required instruction parameters are not provided.")]
    MissingParams,
}

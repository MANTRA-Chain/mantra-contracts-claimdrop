use cosmwasm_std::{Decimal, OverflowError, StdError, Uint128};
use cw_ownable::OwnershipError;
use cw_utils::PaymentError;
use hex::FromHexError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("{0}")]
    OwnershipError(#[from] OwnershipError),

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("{0}")]
    PaymentError(#[from] PaymentError),

    //todo try reusing the Uint* errors
    #[error("An overflow has occurred")]
    Overflow,

    #[error("Invalid distribution percentage, expected: {expected}, actual: {actual}")]
    InvalidDistributionPercentage { expected: Decimal, actual: Decimal },

    #[error("Invalid distribution percentage, cannot be zero")]
    ZeroDistributionPercentage,

    #[error("Invalid reward amount, expected: {expected}, actual: {actual}")]
    InvalidRewardAmount { expected: Uint128, actual: Uint128 },

    #[error("{0}")]
    FromHexError(#[from] FromHexError),

    #[error("Invalid campaign param {param}, reason: {reason}")]
    InvalidCampaignParam { param: String, reason: String },

    #[error("Campaign with id {campaign_id} not found")]
    CampaignNotFound { campaign_id: u64 },
}

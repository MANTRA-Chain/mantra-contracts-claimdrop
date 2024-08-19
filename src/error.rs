use cosmwasm_std::{Decimal, OverflowError, StdError};
use cw_ownable::OwnershipError;
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

    //todo try reusing the Uint* errors
    #[error("An overflow has occurred")]
    Overflow,

    #[error("Invalid distribution percentage, expected: {expected}, actual: {actual}")]
    InvalidDistributionPercentage {
        expected: Decimal,
        actual: Decimal,
    },

    #[error("Invalid distribution percentage, cannot be zero")]
    ZeroDistributionPercentage,

    #[error("Invalid campaign times, please check start_time and end_time of your campaign")]
    InvalidCampaignTimes,
}

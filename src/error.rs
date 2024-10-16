use cosmwasm_std::{Decimal, OverflowError, StdError, Uint128};
use cw_migrate_error_derive::cw_migrate_invalid_version_error;
use cw_ownable::OwnershipError;
use cw_utils::PaymentError;
use hex::FromHexError;
use thiserror::Error;

#[cw_migrate_invalid_version_error]
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Semver parsing error: {0}")]
    SemVer(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("{0}")]
    OwnershipError(#[from] OwnershipError),

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("{0}")]
    PaymentError(#[from] PaymentError),

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

    #[error("Wrong hash length")]
    WrongHashLength,

    #[error("Merkle root verification failed")]
    MerkleRootVerificationFailed,

    #[error("Claim amount exceeds the maximum claimable amount")]
    ExceededMaxClaimAmount,

    #[error("Campaign error: {reason}")]
    CampaignError { reason: String },

    #[error("Invalid distribution times, start time: {start_time}, end time: {end_time}")]
    InvalidDistributionTimes { start_time: u64, end_time: u64 },

    #[error("Invalid start distribution time, start time: {start_time}, current time: {current_time}. The start time needs to be in the future.")]
    InvalidStartDistributionTime { start_time: u64, current_time: u64 },

    #[error("There's nothing to claim for the given address")]
    NothingToClaim,

    #[error("The cliff period has not passed yet")]
    CliffPeriodNotPassed,
}

impl From<semver::Error> for ContractError {
    fn from(err: semver::Error) -> Self {
        Self::SemVer(err.to_string())
    }
}

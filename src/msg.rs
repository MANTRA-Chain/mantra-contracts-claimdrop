use std::fmt::{Display, Formatter};

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{ensure, Addr, Coin, Decimal, Timestamp, Uint128};
use cw_ownable::{cw_ownable_execute, cw_ownable_query};

use crate::error::ContractError;

#[cw_serde]
pub struct InstantiateMsg {
    /// Owner of the contract. If not set, it is the sender of the Instantiate message.
    pub owner: Option<String>,
    /// If set, the address that is allowed to execute the Claim message.
    pub proxy: Option<String>,
}

#[cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {
    /// Manages campaigns based on the action, defined by [CampaignAction].
    ManageCampaign { action: CampaignAction },
    /// Claims rewards from a campaign
    Claim {
        /// The total claimable amount from the campaign
        total_claimable_amount: Uint128,
        /// The receiver address of the claimed rewards. If not set, the sender of the message will be the receiver.
        /// This is useful for allowing a contract to do the claim operation on behalf of a user.
        receiver: Option<String>,
        /// A Vector of all necessary proofs for the merkle root verification, hex-encoded.
        proof: Vec<String>,
    },
    UpdateProxy {
        /// The new proxy address
        proxy: String,
    },
}

#[cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(CampaignResponse)]
    /// Get the airdrop campaign
    Campaign {},
    #[returns(RewardsResponse)]
    /// Get the rewards for a specific campaign and receiver address.
    Rewards {
        /// The total claimable amount for the campaign.
        total_claimable_amount: Uint128,
        /// The address to get the rewards for.
        receiver: String,
        /// A Vector with the necessary proofs for the merkle root verification, hex-encoded.
        proof: Vec<String>,
    },
    #[returns(ClaimedResponse)]
    /// Get the total amount of tokens claimed on the campaign.
    Claimed {
        /// If provided, it will return the tokens claimed by the specified address.
        address: Option<String>,
        /// The address to start querying from. Used for paginating results.
        start_from: Option<String>,
        /// The maximum number of items to return. If not set, the default value is used. Used for paginating results.
        limit: Option<u8>,
    },
}

#[cw_serde]
pub struct MigrateMsg {}

pub type CampaignResponse = Campaign;

/// Response to the Rewards query.
#[cw_serde]
pub struct RewardsResponse {
    /// The tokens that have been claimed by the address.
    pub claimed: Vec<Coin>,
    /// The total amount of tokens that is pending to be claimed by the address.
    pub pending: Vec<Coin>,
    /// The tokens that are available to be claimed by the address.
    pub available_to_claim: Vec<Coin>,
}

/// Response to the Claimed query.
#[cw_serde]
pub struct ClaimedResponse {
    /// Contains a vector with a tuple with (address, coin) that have been claimed
    pub claimed: Vec<(String, Coin)>,
}

/// The campaign action that can be executed with the [ExecuteMsg::ManageCampaign] message.
#[cw_serde]
pub enum CampaignAction {
    /// Creates a new campaign
    CreateCampaign {
        /// The parameters to create a campaign with
        params: Box<CampaignParams>,
    },
    /// Tops up the campaign
    TopUpCampaign {},
    /// Closes the campaign
    CloseCampaign {},
}

/// Represents a campaign.
#[cw_serde]
pub struct Campaign {
    /// The campaign owner
    pub owner: Addr,
    /// The campaign name
    pub name: String,
    /// The campaign description
    pub description: String,
    /// The asset to be distributed as reward by the campaign
    pub reward_asset: Coin,
    /// The amount of the reward asset that has been claimed
    pub claimed: Coin,
    /// The ways the reward is distributed, which are defined by the [DistributionType].
    /// The sum of the percentages must be 100.
    pub distribution_type: Vec<DistributionType>,
    /// The duration of the cliff, in seconds
    pub cliff_duration: Option<u64>,
    /// The campaign start time (unix timestamp), in seconds
    pub start_time: u64,
    /// The campaign end time (unix timestamp), in seconds
    pub end_time: u64,
    /// The campaign merkle root for the airdrop
    pub merkle_root: String,
    /// The timestamp at which the campaign was closed, in seconds
    pub closed: Option<u64>,
}

impl Display for Campaign {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Campaign {{ owner: {}, name: {}, description: {}, reward_asset: {}, claimed: {}, distribution_type: {:?}, cliff_duration: {:?}, start_time: {}, end_time: {}, merkle_root: {} }}",
            self.owner,
            self.name,
            self.description,
            self.reward_asset,
            self.claimed,
            self.distribution_type,
            self.cliff_duration,
            self.start_time,
            self.end_time,
            self.merkle_root
        )
    }
}

impl Campaign {
    /// Creates a new campaign from the given parameters
    pub fn from_params(params: CampaignParams, owner: Addr) -> Self {
        let reward_denom = params.reward_asset.denom.clone();

        Campaign {
            owner,
            name: params.name,
            description: params.description,
            reward_asset: params.reward_asset,
            claimed: Coin {
                denom: reward_denom,
                amount: Uint128::zero(),
            },
            distribution_type: params.distribution_type,
            cliff_duration: params.cliff_duration,
            start_time: params.start_time,
            end_time: params.end_time,
            merkle_root: params.merkle_root,
            closed: None,
        }
    }

    /// Checks if the campaign has started
    pub fn has_started(&self, current_time: &Timestamp) -> bool {
        current_time.seconds() >= self.start_time
    }

    /// Checks if the campaign has ended
    pub fn has_ended(&self, current_time: &Timestamp) -> bool {
        current_time.seconds() >= self.end_time
    }

    /// Checks if the campaign has funds available
    pub fn has_funds_available(&self) -> bool {
        self.claimed.amount < self.reward_asset.amount
    }
}

/// Represents the parameters to create a campaign with.
#[cw_serde]
pub struct CampaignParams {
    /// The campaign owner. If none is provided, the sender of the message will the owner.
    pub owner: Option<String>,
    /// The campaign name
    pub name: String,
    /// The campaign description
    pub description: String,
    /// The asset to be distributed as reward by the campaign
    pub reward_asset: Coin,
    /// The ways the reward is distributed, which are defined by the [DistributionType].
    /// The sum of the percentages must be 100.
    pub distribution_type: Vec<DistributionType>,
    /// The duration of the cliff, in seconds
    pub cliff_duration: Option<u64>,
    /// The campaign start time (unix timestamp), in seconds
    pub start_time: u64,
    /// The campaign end timestamp (unix timestamp), in seconds
    pub end_time: u64,
    /// The campaign merkle root
    pub merkle_root: String,
}

impl CampaignParams {
    /// Validates the campaign name and description
    pub fn validate_campaign_name_description(&self) -> Result<(), ContractError> {
        ensure!(
            !self.name.is_empty(),
            ContractError::InvalidCampaignParam {
                param: "name".to_string(),
                reason: "cannot be empty".to_string(),
            }
        );

        ensure!(
            self.name.len() <= 50usize,
            ContractError::InvalidCampaignParam {
                param: "name".to_string(),
                reason: "cannot be longer than 50 characters".to_string(),
            }
        );

        ensure!(
            !self.description.is_empty(),
            ContractError::InvalidCampaignParam {
                param: "description".to_string(),
                reason: "cannot be empty".to_string(),
            }
        );

        ensure!(
            self.description.len() <= 500usize,
            ContractError::InvalidCampaignParam {
                param: "description".to_string(),
                reason: "cannot be longer than 500 characters".to_string(),
            }
        );

        Ok(())
    }

    /// Validates the start and end times of a campaign
    pub fn validate_campaign_times(&self, current_time: Timestamp) -> Result<(), ContractError> {
        ensure!(
            self.start_time < self.end_time,
            ContractError::InvalidCampaignParam {
                param: "start_time".to_string(),
                reason: "cannot be greater or equal than end_time".to_string(),
            }
        );
        ensure!(
            self.start_time >= current_time.seconds(),
            ContractError::InvalidCampaignParam {
                param: "start_time".to_string(),
                reason: "cannot be less than the current time".to_string(),
            }
        );

        Ok(())
    }

    /// Validates the cliff duration
    pub fn validate_cliff_duration(&self) -> Result<(), ContractError> {
        if let Some(cliff_duration) = self.cliff_duration {
            ensure!(
                cliff_duration > 0,
                ContractError::InvalidCampaignParam {
                    param: "cliff_duration".to_string(),
                    reason: "cannot be zero".to_string(),
                }
            );

            ensure!(
                cliff_duration < self.end_time - self.start_time,
                ContractError::InvalidCampaignParam {
                    param: "cliff_duration".to_string(),
                    reason: "cannot be greater or equal than the campaign duration".to_string(),
                }
            );
        }

        Ok(())
    }

    /// Ensures the distribution type parameters are correct
    pub fn validate_campaign_distribution(
        &self,
        current_time: Timestamp,
    ) -> Result<(), ContractError> {
        let mut total_percentage = Decimal::zero();

        ensure!(
            !self.distribution_type.is_empty() && self.distribution_type.len() <= 2,
            ContractError::InvalidCampaignParam {
                param: "distribution_type".to_string(),
                reason: "invalid number of distribution types, should be at least 1, maximum 2"
                    .to_string(),
            }
        );

        for dist in self.distribution_type.iter() {
            let (percentage, start_time, end_time) = match dist {
                DistributionType::LinearVesting {
                    percentage,
                    start_time,
                    end_time,
                } => (percentage, start_time, end_time),
                DistributionType::LumpSum {
                    percentage,
                    start_time,
                    end_time,
                } => (percentage, start_time, end_time),
            };

            ensure!(
                percentage != Decimal::zero(),
                ContractError::ZeroDistributionPercentage
            );

            total_percentage = total_percentage.checked_add(*percentage)?;

            ensure!(
                *start_time >= current_time.seconds(),
                ContractError::InvalidStartDistributionTime {
                    start_time: *start_time,
                    current_time: current_time.seconds(),
                }
            );

            ensure!(
                end_time > start_time,
                ContractError::InvalidDistributionTimes {
                    start_time: *start_time,
                    end_time: *end_time,
                }
            );

            ensure!(
                *end_time <= self.end_time,
                ContractError::InvalidEndDistributionTime {
                    end_time: *end_time,
                    campaign_end_time: self.end_time,
                }
            );
        }

        ensure!(
            total_percentage == Decimal::percent(100),
            ContractError::InvalidDistributionPercentage {
                expected: Decimal::percent(100),
                actual: total_percentage,
            }
        );

        Ok(())
    }
}

#[cw_serde]
pub enum DistributionType {
    /// The distribution is done in a linear vesting schedule
    LinearVesting {
        /// The percentage of the total reward to be distributed with a linear vesting schedule
        percentage: Decimal,
        /// The unix timestamp when this distribution type starts, in seconds
        start_time: u64,
        /// The unix timestamp when this distribution type ends, in seconds
        end_time: u64,
    },
    /// The distribution is done in a single lump sum, i.e. no vesting period
    LumpSum {
        percentage: Decimal,
        /// The unix timestamp when this distribution type starts, in seconds
        start_time: u64,
        /// The unix timestamp when this distribution type ends, in seconds
        end_time: u64,
    },
}

impl DistributionType {
    pub fn has_started(&self, current_time: &Timestamp) -> bool {
        let start_time = match self {
            DistributionType::LinearVesting { start_time, .. } => start_time,
            DistributionType::LumpSum { start_time, .. } => start_time,
        };

        current_time.seconds() >= *start_time
    }
}

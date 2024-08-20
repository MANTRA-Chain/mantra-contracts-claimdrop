use std::fmt::{Display, Formatter};

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{ensure, Addr, Coin, Decimal, Timestamp, Uint128};
use cw_ownable::{cw_ownable_execute, cw_ownable_query};

use crate::error::ContractError;

#[cw_serde]
pub struct InstantiateMsg {
    /// Owner of the contract. If not set, it is the sender of the Instantiate message.
    pub owner: Option<String>,
}

#[cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {
    /// Manages campaigns based on the action, defined by [CampaignAction].
    ManageCampaign { action: CampaignAction },
}

#[cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

#[cw_serde]
pub enum CampaignAction {
    /// Creates a new campaign
    CreateCampaign {
        /// The parameters to create a campaign with
        params: CampaignParams,
    },
    /// Ends a campaign
    EndCampaign {
        /// The campaign id to end
        campaign_id: u64,
    },
}

#[cw_serde]
pub struct Campaign {
    /// The campaign id
    pub id: u64,
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
    /// The distribution types are applied in the order they are defined.
    pub distribution_type: Vec<DistributionType>,
    /// The campaign start time (unix timestamp)
    pub start_time: u64,
    /// The campaign end time (unix timestamp)
    pub end_time: u64,
    /// The campaign merkle root
    pub merkle_root: String,
}

impl Display for Campaign {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Campaign {{ id: {}, owner: {}, name: {}, description: {}, reward_asset: {}, claimed: {}, distribution_type: {:?}, start_time: {}, end_time: {}, merkle_root: {} }}",
            self.id, self.owner, self.name, self.description, self.reward_asset, self.claimed, self.distribution_type, self.start_time, self.end_time, self.merkle_root
        )
    }
}

impl Campaign {
    /// Creates a new campaign from the given parameters
    pub fn from_params(params: CampaignParams, id: u64, owner: Addr) -> Self {
        let reward_denom = params.reward_asset.denom.clone();

        Campaign {
            id,
            owner,
            name: params.name,
            description: params.description,
            reward_asset: params.reward_asset,
            claimed: Coin {
                denom: reward_denom,
                amount: Uint128::zero(),
            },
            distribution_type: params.distribution_type,
            start_time: params.start_time,
            end_time: params.end_time,
            merkle_root: params.merkle_root,
        }
    }

    /// Checks if the campaign has ended
    pub fn has_ended(&self, current_time: Timestamp) -> bool {
        current_time.seconds() >= self.end_time || self.claimed.amount == self.reward_asset.amount
    }
}

#[cw_serde]
pub struct CampaignParams {
    /// The campaign owner. If none is provided, the sender of the message will the owner.
    pub owner: Option<String>,
    /// The campaign name
    pub name: String,
    /// The campaign description
    pub description: String,
    //todo vector of coins? what about cw20 tokens?
    /// The asset to be distributed as reward by the campaign
    pub reward_asset: Coin,
    /// The ways the reward is distributed, which are defined by the [DistributionType].
    /// The sum of the percentages must be 100.
    /// The distribution types are applied in the order they are defined.
    pub distribution_type: Vec<DistributionType>,
    /// The campaign start timestamp, in seconds
    pub start_time: u64,
    /// The campaign end timestamp, in seconds
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
            self.name
                .chars()
                .all(|c| c.is_alphanumeric() || c.is_whitespace()),
            ContractError::InvalidCampaignParam {
                param: "name".to_string(),
                reason: "can only contain alphanumeric characters and spaces".to_string(),
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

    /// Validates the merkle root, i.e. checks if it is a valid SHA-256 hash
    pub fn validate_merkle_root(&self) -> Result<(), ContractError> {
        let mut merkle_root_buf: [u8; 32] = [0; 32];
        hex::decode_to_slice(&self.merkle_root, &mut merkle_root_buf)?;

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
        ensure!(
            self.end_time > current_time.seconds(),
            ContractError::InvalidCampaignParam {
                param: "end_time".to_string(),
                reason: "cannot be less or equal than the current time".to_string(),
            }
        );

        Ok(())
    }

    /// Ensures the distribution type parameters are correct
    pub fn validate_campaign_distribution(
        &self,
        _current_time: Timestamp,
    ) -> Result<(), ContractError> {
        let mut total_percentage = Decimal::zero();

        for dist in self.distribution_type.iter() {
            let percentage = match dist {
                DistributionType::LinearVesting { percentage, .. } => percentage,
                DistributionType::LumpSum { percentage, .. } => percentage,
            };

            //todo validate times

            //todo check if the vestings are repeated? i.e. imagine 2 lumpsum distributions of 50% each
            // with the exact same start and end times?

            ensure!(
                percentage != Decimal::zero(),
                ContractError::ZeroDistributionPercentage
            );

            total_percentage = total_percentage.checked_add(*percentage)?;
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

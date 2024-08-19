use std::fmt::{Display, Formatter};

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Decimal, Uint128};
use cw_ownable::{cw_ownable_execute, cw_ownable_query};

#[cw_serde]
pub struct InstantiateMsg {
    /// Owner of the contract. If not set, it is the sender of the Instantiate message.
    pub owner: Option<String>,
}

#[cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {
    /// Create a new campaign
    CreateCampaign { params: CampaignParams },
}

#[cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

#[cw_serde]
pub struct Campaign {
    /// The campaign id
    pub id: u64,
    /// The campaign name
    pub name: String,
    /// The campaign description
    pub description: String,
    /// The asset to be distributed as reward by the campaign
    pub reward_asset: Coin,
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
        write!(f, "Campaign {{ id: {}, name: {}, description: {}, reward_asset: {}, distribution_type: {:?}, start_time: {}, end_time: {}, merkle_root: {} }}",
               self.id, self.name, self.description, self.reward_asset, self.distribution_type, self.start_time, self.end_time, self.merkle_root)
    }
}

impl Default for Campaign {
    fn default() -> Self {
        Campaign {
            id: 0,
            name: "".to_string(),
            description: "".to_string(),
            reward_asset: Coin {
                denom: "".to_string(),
                amount: Uint128::zero(),
            },
            distribution_type: vec![],
            start_time: 0,
            end_time: 0,
            merkle_root: "".to_string(),
        }
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

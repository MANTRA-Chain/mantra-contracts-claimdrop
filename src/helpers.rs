use cosmwasm_std::{ensure, MessageInfo, Timestamp};

use crate::error::ContractError;
use crate::msg::CampaignParams;

/// Validates the campaign parameters
pub(crate) fn validate_campaign_params(
    current_time: Timestamp,
    info: &MessageInfo,
    campaign_params: &CampaignParams,
) -> Result<(), ContractError> {
    campaign_params.validate_campaign_name_description()?;
    campaign_params.validate_merkle_root()?;
    campaign_params.validate_campaign_distribution(current_time)?;
    campaign_params.validate_campaign_times(current_time)?;

    let reward_amount = cw_utils::must_pay(info, &campaign_params.reward_asset.denom)?;
    ensure!(
        reward_amount == campaign_params.reward_asset.amount,
        ContractError::InvalidRewardAmount {
            expected: campaign_params.reward_asset.amount,
            actual: reward_amount
        }
    );

    Ok(())
}

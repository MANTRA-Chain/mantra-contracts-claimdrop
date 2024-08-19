use cosmwasm_std::{Decimal, ensure, MessageInfo, Timestamp};

use crate::error::ContractError;
use crate::msg::{CampaignParams, DistributionType};

pub(crate) fn validate_campaign_params(
    current_time: Timestamp,
    info: &MessageInfo,
    campaign_params: &CampaignParams,
) -> Result<(), ContractError> {
    validate_campaign_distribution(current_time, &campaign_params.distribution_type)?;
    // todo validate name, description?

    validate_campaign_times(current_time, campaign_params.start_time, campaign_params.end_time)?;


    Ok(())
}

/// Validates the start and end times of a campaign
fn validate_campaign_times(current_time: Timestamp, start_time: u64, end_time: u64) -> Result<(), ContractError> {
    ensure!(start_time < end_time, ContractError::InvalidCampaignTimes);
    ensure!(start_time >= current_time.seconds(), ContractError::InvalidCampaignTimes);
    ensure!(end_time > current_time.seconds(), ContractError::InvalidCampaignTimes);
    Ok(())
}


/// Validates the distribution percentages of a campaign
fn validate_campaign_distribution(
    current_time: Timestamp,
    distribution: &Vec<DistributionType>,
) -> Result<(), ContractError> {
    let mut total_percentage = Decimal::zero();

    for dist in distribution.iter() {
        let percentage = match dist {
            DistributionType::LinearVesting { percentage, .. } => percentage,
            DistributionType::LumpSum { percentage, .. } => percentage,
        };

        //todo validate times

        //todo check if the vestings are repeated? i.e. imagine 2 lumpsum distributions of 50% each
        // with the exact same start and end times?

        ensure!(percentage != Decimal::zero(), ContractError::ZeroDistributionPercentage);

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

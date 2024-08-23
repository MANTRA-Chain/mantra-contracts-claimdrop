use cosmwasm_std::Deps;

use crate::error::ContractError;
use crate::msg::{CampaignFilter, CampaignsResponse};
use crate::state::{get_campaign_by_id, get_campaigns, get_campaigns_by_owner};

/// Returns a list of campaigns based on the provided filter.
pub(crate) fn query_campaigns(
    deps: Deps,
    campaign_filter: Option<CampaignFilter>,
    start_from: Option<u64>,
    limit: Option<u8>,
) -> Result<CampaignsResponse, ContractError> {
    //do the same as above but matching if campaign_filter is some
    let campaigns = if let Some(campaign_filter) = campaign_filter {
        match campaign_filter {
            CampaignFilter::Owner { owner } => {
                deps.api.addr_validate(&owner)?;
                get_campaigns_by_owner(deps.storage, owner)?
            }
            CampaignFilter::CampaignId { campaign_id } => {
                vec![get_campaign_by_id(deps.storage, campaign_id)?]
            }
        }
    } else {
        get_campaigns(deps.storage, start_from, limit)?
    };

    Ok(CampaignsResponse { campaigns })
}

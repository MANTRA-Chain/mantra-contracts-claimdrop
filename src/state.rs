use std::collections::HashMap;

use cosmwasm_std::{Addr, Deps, Order, StdResult, Storage, Uint128};
use cw_storage_plus::{Bound, Index, IndexList, IndexedMap, Map, MultiIndex};

use crate::error::ContractError;
use crate::msg::Campaign;

/// Indexed Map of campaigns. The key is the campaign id, the value is the [Campaign].
pub const CAMPAIGNS: IndexedMap<String, Campaign, CampaignIndexes> = IndexedMap::new(
    "campaigns",
    CampaignIndexes {
        owner: MultiIndex::new(
            |_pk, c| c.owner.to_string(),
            "campaigns",
            "campaigns__owner",
        ),
        merkle_root: MultiIndex::new(
            |_pk, c| c.merkle_root.clone(),
            "campaigns",
            "campaigns__merkle_root",
        ),
    },
);

pub struct CampaignIndexes<'a> {
    pub owner: MultiIndex<'a, String, Campaign, String>,
    pub merkle_root: MultiIndex<'a, String, Campaign, String>,
}

impl<'a> IndexList<Campaign> for CampaignIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Campaign>> + '_> {
        let v: Vec<&dyn Index<Campaign>> = vec![&self.owner, &self.merkle_root];
        Box::new(v.into_iter())
    }
}

//todo probably need to add time when it was claimed last, so the linear vesting can be calculated properly

pub type Claim = (Uint128, u64);
pub type DistributionSlot = usize;

pub const CLAIMS: Map<(String, String), HashMap<DistributionSlot, Claim>> = Map::new("claims");

/// Returns a campaign by its id
pub fn get_campaign_by_id(
    storage: &dyn Storage,
    campaign_id: &str,
) -> Result<Campaign, ContractError> {
    CAMPAIGNS
        .may_load(storage, campaign_id.to_string())?
        .ok_or(ContractError::CampaignNotFound {
            campaign_id: campaign_id.to_string(),
        })
}

// settings for pagination
pub(crate) const MAX_LIMIT: u8 = 50;
const DEFAULT_LIMIT: u8 = 10;

/// Returns campaigns by owner
pub fn get_campaigns_by_owner(storage: &dyn Storage, owner: String) -> StdResult<Vec<Campaign>> {
    let limit = MAX_LIMIT as usize;
    // no way to paginate this easily. It will return the last [MAX_LIMIT] campaigns created by the owner
    CAMPAIGNS
        .idx
        .owner
        .prefix(owner)
        .range(storage, None, None, Order::Descending)
        .take(limit)
        .map(|item| {
            let (_, campaign) = item?;
            Ok(campaign)
        })
        .collect()
}

/// Returns a list of campaigns. Supports pagination with `start_from` and `limit`.
pub fn get_campaigns(
    storage: &dyn Storage,
    start_from: Option<String>,
    limit: Option<u8>,
) -> StdResult<Vec<Campaign>> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_from.map(Bound::exclusive);

    CAMPAIGNS
        .range(storage, start, None, Order::Descending)
        .take(limit)
        .map(|item| {
            let (_, campaign) = item?;
            Ok(campaign)
        })
        .collect()
}

/// Returns the claims that an address has made for a campaign
pub fn get_claims_for_address(
    deps: Deps,
    campaign_id: &str,
    address: &Addr,
) -> Result<HashMap<DistributionSlot, Claim>, ContractError> {
    let claimed = CLAIMS.may_load(deps.storage, (address.to_string(), campaign_id.to_string()))?;
    Ok(claimed.unwrap_or_default())
}

/// Returns the claims that an address has made for a campaign
pub fn get_total_claims_amount_for_address(
    deps: Deps,
    campaign_id: &str,
    address: &Addr,
) -> Result<Uint128, ContractError> {
    let claimed = get_claims_for_address(deps, campaign_id, address)?;
    let mut total = Uint128::zero();
    for (_, (amount, _)) in claimed.iter() {
        total = total.checked_add(*amount)?;
    }

    Ok(total)
}

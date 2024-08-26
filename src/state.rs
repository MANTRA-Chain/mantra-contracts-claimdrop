use cosmwasm_std::{Order, StdResult, Storage};
use cw_storage_plus::{Bound, Index, IndexList, IndexedMap, Item, Map, MultiIndex, UniqueIndex};

use crate::error::ContractError;
use crate::msg::Campaign;

/// The number of campaigns created
pub const CAMPAIGN_COUNT: Item<u64> = Item::new("campaign_count");

/// Indexed Map of campaigns. The key is the campaign id, the value is the [Campaign].
pub const CAMPAIGNS: IndexedMap<u64, Campaign, CampaignIndexes> = IndexedMap::new(
    "campaigns",
    CampaignIndexes {
        owner: MultiIndex::new(
            |_pk, c| c.owner.to_string(),
            "campaigns",
            "campaigns__owner",
        ),
        merkle_root: UniqueIndex::new(
            |c: &Campaign| c.merkle_root.clone(),
            "campaigns__merkle_root",
        ),
    },
);

pub struct CampaignIndexes<'a> {
    pub owner: MultiIndex<'a, String, Campaign, String>,
    pub merkle_root: UniqueIndex<'a, String, Campaign, String>,
}

impl<'a> IndexList<Campaign> for CampaignIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Campaign>> + '_> {
        let v: Vec<&dyn Index<Campaign>> = vec![&self.owner, &self.merkle_root];
        Box::new(v.into_iter())
    }
}

//todo likely to change, bool works fine when you have a single reward token with lumpsum distribution only
pub const CLAIMS: Map<(String, u64), ()> = Map::new("claims");

/// Returns a campaign by its id
pub fn get_campaign_by_id(
    storage: &dyn Storage,
    campaign_id: u64,
) -> Result<Campaign, ContractError> {
    CAMPAIGNS
        .may_load(storage, campaign_id)?
        .ok_or(ContractError::CampaignNotFound { campaign_id })
}

// settings for pagination
pub(crate) const MAX_LIMIT: u8 = 50;
const DEFAULT_LIMIT: u8 = 10;

/// Returns a campaign by owner
pub fn get_campaigns_by_owner(storage: &dyn Storage, owner: String) -> StdResult<Vec<Campaign>> {
    let limit = MAX_LIMIT as usize;
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

pub fn get_campaigns(
    storage: &dyn Storage,
    start_from: Option<u64>,
    limit: Option<u8>,
) -> StdResult<Vec<Campaign>> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = if start_from.is_some() {
        Some(Bound::exclusive(start_from.unwrap()))
    } else {
        None
    };

    CAMPAIGNS
        .range(storage, start, None, Order::Descending)
        .take(limit)
        .map(|item| {
            let (_, campaign) = item?;
            Ok(campaign)
        })
        .collect()
}

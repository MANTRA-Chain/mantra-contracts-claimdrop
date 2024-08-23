use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex, UniqueIndex};

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

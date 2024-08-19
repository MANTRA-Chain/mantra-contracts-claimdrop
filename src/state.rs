use cw_storage_plus::{Item, Map};

use crate::msg::Campaign;

/// The number of campaigns created
pub const CAMPAIGN_COUNT: Item<u64> = Item::new("campaign_count");

/// The list of campaigns. The key is the campaign id, the value is the campaign data.
pub const CAMPAIGNS: Map<u64, Campaign> = Map::new("campaigns");

/// The merkle root for a given campaign. The key is the campaign id, the value is the root hash.
pub const MERKLE_ROOT: Map<u64, String> = Map::new("merkle_root");

//todo likely to change, bool works fine when you have a single reward token with lumpsum distribution only
pub const CLAIMS: Map<(String, u64), bool> = Map::new("claims");

use std::collections::HashMap;

use cosmwasm_std::{Addr, Deps, Uint128};
use cw_storage_plus::{Item, Map};

use crate::error::ContractError;
use crate::msg::Campaign;

/// The campaign item
pub const CAMPAIGN: Item<Campaign> = Item::new("campaign");

/// Contains information about how much has an address claimed for a given distribution type.
/// The key is the address and the value is a hashmap where the key is the distribution_type index,
/// and the value is a tuple with the amount claimed and the timestamp at which it was claimed.
/// Used primarily to compute the correct claim amounts when doing the linear vesting.
pub const CLAIMS: Map<String, HashMap<DistributionSlot, Claim>> = Map::new("claims");

/// The claim is a tuple of the amount and the timestamp when it was claimed.
pub type Claim = (Uint128, u64);
/// The distribution slot is the index of DistributionType on the campaign.
pub type DistributionSlot = usize;

/// Returns the claims that an address has made for a campaign
pub fn get_claims_for_address(
    deps: Deps,
    address: &Addr,
) -> Result<HashMap<DistributionSlot, Claim>, ContractError> {
    let claimed = CLAIMS.may_load(deps.storage, address.to_string())?;
    Ok(claimed.unwrap_or_default())
}

/// Returns the claims that an address has made for the campaign
pub fn get_total_claims_amount_for_address(
    deps: Deps,
    address: &Addr,
) -> Result<Uint128, ContractError> {
    let claimed = get_claims_for_address(deps, address)?;
    let mut total = Uint128::zero();
    for (_, (amount, _)) in claimed.iter() {
        total = total.checked_add(*amount)?;
    }

    Ok(total)
}

use std::collections::HashMap;

use cosmwasm_std::{Deps, Uint128};
use cw_storage_plus::{Item, Map};

use crate::error::ContractError;
use crate::helpers;
use crate::msg::Campaign;

/// The campaign item that stores the current active campaign
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

/// Stores the allocation for each address in the airdrop. This is set before the campaign starts
/// and cannot be modified after that.
pub const ALLOCATIONS: Map<&str, Uint128> = Map::new("allocations");

/// Stores blacklisted addresses. Blacklisted addresses cannot claim their allocations.
pub const BLACKLIST: Map<&str, bool> = Map::new("blacklist");

/// Returns the claims that an address has made for a campaign
///
/// # Arguments
/// * `deps` - The dependencies
/// * `address` - The address to get claims for
///
/// # Returns
/// * `Result<HashMap<DistributionSlot, Claim>, ContractError>` - The claims for the address
pub fn get_claims_for_address(
    deps: Deps,
    address: String,
) -> Result<HashMap<DistributionSlot, Claim>, ContractError> {
    let claimed = CLAIMS.may_load(deps.storage, helpers::validate_raw_address(deps, &address)?)?;
    Ok(claimed.unwrap_or_default())
}

/// Returns the total amount of tokens claimed by an address
///
/// # Arguments
/// * `deps` - The dependencies
/// * `address` - The address to get total claims for
///
/// # Returns
/// * `Result<Uint128, ContractError>` - The total amount claimed
pub fn get_total_claims_amount_for_address(
    deps: Deps,
    address: &str,
) -> Result<Uint128, ContractError> {
    let claimed = get_claims_for_address(deps, address.to_string())?;
    let mut total = Uint128::zero();
    for (_, (amount, _)) in claimed.iter() {
        total = total.checked_add(*amount)?;
    }

    Ok(total)
}

/// Returns the allocation for an address
///
/// # Arguments
/// * `deps` - The dependencies
/// * `address` - The address to get allocation for
///
/// # Returns
/// * `Result<Option<Uint128>, ContractError>` - The allocation amount if it exists
pub fn get_allocation(deps: Deps, address: &str) -> Result<Option<Uint128>, ContractError> {
    Ok(ALLOCATIONS.may_load(
        deps.storage,
        helpers::validate_raw_address(deps, address)?.as_str(),
    )?)
}

/// Returns whether an address is blacklisted
///
/// # Arguments
/// * `deps` - The dependencies
/// * `address` - The address to check
///
/// # Returns
/// * `Result<bool, ContractError>` - Whether the address is blacklisted
pub fn is_blacklisted(deps: Deps, address: &str) -> Result<bool, ContractError> {
    Ok(BLACKLIST
        .may_load(
            deps.storage,
            helpers::validate_raw_address(deps, address)?.as_str(),
        )?
        .unwrap_or(false))
}

use claimdrop_contract::{
    error::ContractError,
    helpers::{validate_raw_address, MAX_PLACEHOLDER_ADDRESS_LEN},
};
use cosmwasm_std::testing::mock_dependencies;

#[test]
fn valid_bech32_address() {
    let deps = mock_dependencies();
    let address_raw = "cosmos1AABBccddeeffgghhii001122334455667789";
    let result = validate_raw_address(deps.as_ref(), address_raw);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), address_raw.to_lowercase());
}

#[test]
fn invalid_bech32_but_valid_placeholder() {
    let deps = mock_dependencies();
    let address_raw = "example.eth";
    let result = validate_raw_address(deps.as_ref(), address_raw);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "example.eth");
}

#[test]
fn invalid_bech32_and_invalid_placeholder() {
    let deps = mock_dependencies();
    let address_raw = "invalid#address";
    let result = validate_raw_address(deps.as_ref(), address_raw);
    assert!(matches!(result, Err(ContractError::InvalidInput { .. })));
}

#[test]
fn uppercase_bech32_address() {
    let deps = mock_dependencies();
    let address_raw = "Cosmos1aabbccddeeffgghhii001122334455667789";
    let result = validate_raw_address(deps.as_ref(), address_raw);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), address_raw.to_lowercase());
}

#[test]
fn empty_string() {
    let deps = mock_dependencies();
    let address_raw = "";
    let result = validate_raw_address(deps.as_ref(), address_raw);
    assert!(matches!(result, Err(ContractError::InvalidInput { .. })));
}

#[test]
fn placeholder_too_long() {
    let deps = mock_dependencies();
    let placeholder = "a".repeat(MAX_PLACEHOLDER_ADDRESS_LEN + 1);
    let result = validate_raw_address(deps.as_ref(), &placeholder);
    assert!(matches!(result, Err(ContractError::InvalidInput { .. })));
}

#[test]
fn placeholder_with_invalid_chars() {
    let deps = mock_dependencies();
    let address_raw = "invalid_address!";
    let result = validate_raw_address(deps.as_ref(), address_raw);
    assert!(matches!(result, Err(ContractError::InvalidInput { .. })));
}

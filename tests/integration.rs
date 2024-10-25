use std::str::FromStr;

use cosmwasm_std::{coin, coins, Decimal, Uint128};
use cw_multi_test::AppResponse;
use cw_ownable::OwnershipError;
use cw_utils::PaymentError;

use crate::hashes::{
    ALICE_PROOFS, ALICE_PROOFS_X, BOB_PROOFS, BROKEN_PROOFS, CAROL_PROOFS, DAN_PROOFS, EVA_PROOFS,
    MERKLE_ROOT, MERKLE_ROOT_X,
};
use claimdrop_contract::error::ContractError;
use claimdrop_contract::msg::{CampaignAction, CampaignParams, DistributionType, RewardsResponse};

use crate::suite::TestingSuite;

mod hashes;
mod suite;

#[test]
fn instantiate_claimdrop_contract() {
    let mut suite = TestingSuite::default_with_balances(vec![coin(1_000_000_000, "uom")]);
    suite.instantiate_claimdrop_contract(None);
}

#[test]
fn create_multiple_campaigns_fails() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);

    let alice = &suite.senders[0].clone();
    let current_time = &suite.get_time();

    suite
        .instantiate_claimdrop_contract(Some(alice.to_string()))
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                    }],
                    cliff_duration: None,
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: "87bb9bf2d62ff8430e314a0d18d3134dd01afd98b75b487337a677322d20ad3d"
                        .to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                    }],
                    cliff_duration: None,
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: "87bb9bf2d62ff8430e314a0d18d3134dd01afd98b75b487337a677322d20ad3d"
                        .to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::CampaignError { reason } => {
                        assert_eq!(reason, "existing campaign");
                    }
                    _ => panic!("Wrong error type, should return ContractError::CampaignError"),
                }
            },
        )
        .query_campaign(|result| {
            let campaign = result.unwrap();
            assert_eq!(campaign.name, "Test Airdrop I".to_string());
        })
        .manage_campaign(
            alice,
            CampaignAction::CloseCampaign {},
            &[],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop II".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                    }],
                    cliff_duration: None,
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: "87bb9bf2d62ff8430e314a0d18d3134dd01afd98b75b487337a677322d20ad3d"
                        .to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::CampaignError { reason } => {
                        assert_eq!(reason, "existing campaign");
                    }
                    _ => panic!("Wrong error type, should return ContractError::CampaignError"),
                };
            },
        );
}

#[test]
fn cant_create_campaign_if_not_owner() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);

    let alice = &suite.senders[0].clone();
    let bob = &suite.senders[1].clone();
    let current_time = &suite.get_time();

    suite
        .instantiate_claimdrop_contract(Some(alice.to_string()))
        .manage_campaign(
            bob,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                    }],
                    cliff_duration: None,
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: "a2835f4d5c3b7f9f58ec60e85a52e6e24985777540a214ee4080431bacf4882a"
                        .to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::OwnershipError(e) => match e {
                        OwnershipError::NotOwner => {}
                        _ => panic!("Wrong error type, should return OwnershipError::NotOwner"),
                    },
                    _ => panic!("Wrong error type, should return ContractError::OwnershipError"),
                }
            },
        );
}

#[test]
fn validate_campaign_params() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);

    let alice = &suite.senders[0].clone();
    let current_time = &suite.get_time();

    suite
        .instantiate_claimdrop_contract(Some(alice.to_string()))
        // name & description
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                    }],
                    cliff_duration: None,
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: "a2835f4d5c3b7f9f58ec60e85a52e6e24985777540a214ee4080431bacf4882a"
                        .to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidCampaignParam { param, reason, } => {
                        assert_eq!(param, "name");
                        assert_eq!(reason, "cannot be empty");
                    }
                    _ => panic!("Wrong error type, should return ContractError::InvalidCampaignParam"),
                }
            },
        ).manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "%WEmpcQxf5ONYNy1Gj2m#w7oauP6E5OUoZM1n7AnTRDX9sSd5DK".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                    }],
                    cliff_duration: None,
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: "a2835f4d5c3b7f9f58ec60e85a52e6e24985777540a214ee4080431bacf4882a"
                        .to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidCampaignParam { param, reason } => {
                        assert_eq!(param, "name");
                        assert_eq!(reason, "cannot be longer than 50 characters");
                    }
                    _ => panic!("Wrong error type, should return ContractError::InvalidCampaignParam"),
                }
            },
        )
        .manage_campaign(
        alice,
        CampaignAction::CreateCampaign {
            params: Box::new(CampaignParams {
                owner: None,
                name: "Test Airdrop I".to_string(),
                description: "".to_string(),
                reward_asset: coin(100_000, "uom"),
                distribution_type: vec![DistributionType::LumpSum {
                    percentage: Decimal::one(),
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                }],
                cliff_duration: None,
                start_time: current_time.seconds() + 1,
                end_time: current_time.seconds() + 172_800,
                merkle_root: "a79197d1f2f90797e65c545a99662630da89baf820af47b0044cd910339cb4fd"
                    .to_string(),
            }),
        },
        &coins(100_000, "uom"),
        |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();
            match err {
                ContractError::InvalidCampaignParam { param, reason } => {
                    assert_eq!(param, "description");
                    assert_eq!(reason, "cannot be empty");
                }
                _ => panic!("Wrong error type, should return ContractError::InvalidCampaignParam"),
            }
        },
    )
        .manage_campaign(
        alice,
        CampaignAction::CreateCampaign {
            params: Box::new(CampaignParams {
                owner: None,
                name: "Test Airdrop I".to_string(),
                description: "bxOmDxCEaGwURwYOretJXoSBZxKtqvRzQYFUzBvzrYHqVGywHYeWvApTYETxADYdHDeBDTbKXeAzCVPhvxzTBxNtFhUzPYCgrEBP\
                bxOmDxCEaGwURwYOretJXoSBZxKtqvRzQYFUzBvzrYHqVGywHYeWvApTYETxADYdHDeBDTbKXeAzCVPhvxzTBxNtFhUzPYCgrEBP\
                bxOmDxCEaGwURwYOretJXoSBZxKtqvRzQYFUzBvzrYHqVGywHYeWvApTYETxADYdHDeBDTbKXeAzCVPhvxzTBxNtFhUzPYCgrEBP\
                bxOmDxCEaGwURwYOretJXoSBZxKtqvRzQYFUzBvzrYHqVGywHYeWvApTYETxADYdHDeBDTbKXeAzCVPhvxzTBxNtFhUzPYCgrEBP\
                bxOmDxCEaGwURwYOretJXoSBZxKtqvRzQYFUzBvzrYHqVGywHYeWvApTYETxADYdHDeBDTbKXeAzCVPhvxzTBxNtFhUzPYCgrEBP1".to_string(),
                reward_asset: coin(100_000, "uom"),
                distribution_type: vec![DistributionType::LumpSum {
                    percentage: Decimal::one(),
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                }],
                cliff_duration: None,
                start_time: current_time.seconds() + 1,
                end_time: current_time.seconds() + 172_800,
                merkle_root: "a79197d1f2f90797e65c545a99662630da89baf820af47b0044cd910339cb4fd"
                    .to_string(),
            }),
        },
        &coins(100_000, "uom"),
        |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();
            match err {
                ContractError::InvalidCampaignParam { param, reason } => {
                    assert_eq!(param, "description");
                    assert_eq!(reason, "cannot be longer than 500 characters");
                }
                _ => panic!("Wrong error type, should return ContractError::InvalidCampaignParam"),
            }
        },
    )
        // merkle root
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params:Box::new( CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(5_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                    }],
                    cliff_duration: None,
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: ""
                        .to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::FromHexError { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::FromHexError"),
                }
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params:Box::new( CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(5_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                    }],
                    cliff_duration: None,
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: "a79197d1f2f90797e65c545a99662630da89baf820af47b0044cd910339cb4fda"
                        .to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::FromHexError { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::FromHexError"),
                }
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params:Box::new( CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(5_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                    }],
                    cliff_duration: None,
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: "a79197d1f2f90797ebaf820af47b0044cd910339cb4fda"
                        .to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::FromHexError { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::FromHexError"),
                }
            },
        )
        // campaign times
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                    }],
                    cliff_duration: None,
                    start_time: current_time.seconds() + 172_800,
                    end_time: current_time.seconds() + 1,
                    merkle_root: "a79197d1f2f90797e65c545a99662630da89baf820af47b0044cd910339cb4fd"
                        .to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidCampaignParam { param, reason } => {
                        assert_eq!(param, "start_time");
                        assert_eq!(reason, "cannot be greater or equal than end_time");
                    }
                    _ => panic!("Wrong error type, should return ContractError::InvalidCampaignParam"),
                }
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                    }],
                    cliff_duration: None,
                    start_time: current_time.seconds() - 100,
                    end_time: current_time.seconds() + 1,
                    merkle_root: "a79197d1f2f90797e65c545a99662630da89baf820af47b0044cd910339cb4fd"
                        .to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidCampaignParam { param, reason, } => {
                        assert_eq!(param, "start_time");
                        assert_eq!(reason, "cannot be less than the current time");
                    }
                    _ => panic!("Wrong error type, should return ContractError::InvalidCampaignParam"),
                }
            },
        )
        // distribution types
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![],
                    cliff_duration: None,
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: "a79197d1f2f90797e65c545a99662630da89baf820af47b0044cd910339cb4fd"
                        .to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidCampaignParam { param, reason } => {
                        assert_eq!(param, "distribution_type");
                        assert_eq!(reason, "invalid number of distribution types, should be at least 1, maximum 2");
                    }
                    _ => panic!("Wrong error type, should return ContractError::InvalidCampaignParam"),
                }
            },
        ).manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![
                        DistributionType::LumpSum {
                        percentage: Decimal::from_str("2").unwrap(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                    },
                        DistributionType::LumpSum {
                        percentage: Decimal::from_str("2").unwrap(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                    },
                        DistributionType::LumpSum {
                        percentage: Decimal::from_str("2").unwrap(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                    },
                    ],
                    cliff_duration: None,
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: "a79197d1f2f90797e65c545a99662630da89baf820af47b0044cd910339cb4fd"
                        .to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidCampaignParam { param, reason } => {
                        assert_eq!(param, "distribution_type");
                        assert_eq!(reason, "invalid number of distribution types, should be at least 1, maximum 2");
                    }
                    _ => panic!("Wrong error type, should return ContractError::InvalidCampaignParam"),
                }
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::from_str("2").unwrap(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                    }],
                    cliff_duration: None,
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: "a79197d1f2f90797e65c545a99662630da89baf820af47b0044cd910339cb4fd"
                        .to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidDistributionPercentage { expected, actual } => {
                        assert_eq!(expected, Decimal::one());
                        assert_eq!(actual, Decimal::from_str("2").unwrap());
                    }
                    _ => panic!("Wrong error type, should return ContractError::InvalidDistributionPercentage"),
                }
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::from_str("0.2").unwrap(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                    }],
                    cliff_duration: None,
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: "a79197d1f2f90797e65c545a99662630da89baf820af47b0044cd910339cb4fd"
                        .to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidDistributionPercentage { expected, actual } => {
                        assert_eq!(expected, Decimal::one());
                        assert_eq!(actual, Decimal::from_str("0.2").unwrap());
                    }
                    _ => panic!("Wrong error type, should return ContractError::InvalidDistributionPercentage"),
                }
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::zero(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                    }],
                    cliff_duration: None,
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: "a79197d1f2f90797e65c545a99662630da89baf820af47b0044cd910339cb4fd"
                        .to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::ZeroDistributionPercentage  => {}
                    _ => panic!("Wrong error type, should return ContractError::ZeroDistributionPercentage"),
                }
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() - 1,
                        end_time: current_time.seconds() + 172_800,
                    }],
                    cliff_duration: None,
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: "a79197d1f2f90797e65c545a99662630da89baf820af47b0044cd910339cb4fd"
                        .to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidStartDistributionTime {..} => {}
                    _ => panic!("Wrong error type, should return ContractError::InvalidStartDistributionTime"),
                }
            },
        )

        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds(),
                        end_time: current_time.seconds() - 1,
                    }],
                    cliff_duration: None,
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: "a79197d1f2f90797e65c545a99662630da89baf820af47b0044cd910339cb4fd"
                        .to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidDistributionTimes {..} => {}
                    _ => panic!("Wrong error type, should return ContractError::InvalidDistributionTimes"),
                }
            },
        )

        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![
                        DistributionType::LumpSum {
                        percentage: Decimal::from_str("0.5").unwrap(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                    },

                        DistributionType::LumpSum {
                            percentage: Decimal::one(),
                            start_time: current_time.seconds() + 1,
                            end_time: current_time.seconds() + 172_800,
                        }
                    ],
                    cliff_duration: None,
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: "a79197d1f2f90797e65c545a99662630da89baf820af47b0044cd910339cb4fd"
                        .to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidDistributionPercentage { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::InvalidDistributionPercentage"),
                }
            },
        ).manage_campaign(
        alice,
        CampaignAction::CreateCampaign {
            params: Box::new(CampaignParams {
                owner: None,
                name: "Test Airdrop I".to_string(),
                description: "This is an airdrop, 土金, ك".to_string(),
                reward_asset: coin(100_000, "uom"),
                distribution_type: vec![DistributionType::LumpSum {
                    percentage: Decimal::one(),
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                }],
                cliff_duration: None,
                start_time: current_time.seconds() + 1,
                end_time: current_time.seconds() + 172_800,
                merkle_root: "a79197d1f2f90797e65c545a99662630da89baf820af47b0044cd910339cb4fd"
                    .to_string(),
            }),
        },
        &[],
        |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();
            match err {
                ContractError::PaymentError { .. } => {}
                _ => panic!("Wrong error type, should return ContractError::PaymentError"),
            }
        },
    )
        // cliff duration
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                    }],
                    cliff_duration: Some(0u64),
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: "a79197d1f2f90797e65c545a99662630da89baf820af47b0044cd910339cb4fd"
                        .to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidCampaignParam { param, reason } => {
                        assert_eq!(param, "cliff_duration");
                        assert_eq!(reason, "cannot be zero");
                    }
                    _ => panic!("Wrong error type, should return ContractError::InvalidCampaignParam"),
                }
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                    }],
                    cliff_duration: Some(172_801u64),
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: "a79197d1f2f90797e65c545a99662630da89baf820af47b0044cd910339cb4fd"
                        .to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidCampaignParam { param, reason } => {
                        assert_eq!(param, "cliff_duration");
                        assert_eq!(reason, "cannot be greater or equal than the campaign duration");
                    }
                    _ => panic!("Wrong error type, should return ContractError::InvalidCampaignParam"),
                }
            },
        )
        // reward
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params:Box::new( CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(5_000, "uusdc"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                    }],
                    cliff_duration: None,
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: "a79197d1f2f90797e65c545a99662630da89baf820af47b0044cd910339cb4fd"
                        .to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::PaymentError(e) => {
                        match e {
                            PaymentError::MissingDenom(_) => {}
                            _ => panic!("Wrong error type, should return PaymentError::MissingDenom"),
                        }
                    }
                    _ => panic!("Wrong error type, should return ContractError::PaymentError"),
                }
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params:Box::new( CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(5_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                    }],
                    cliff_duration: None,
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: "a79197d1f2f90797e65c545a99662630da89baf820af47b0044cd910339cb4fd"
                        .to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidRewardAmount { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::InvalidRewardAmount"),
                }
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params:Box::new( CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_001, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                    }],
                    cliff_duration: None,
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: "a79197d1f2f90797e65c545a99662630da89baf820af47b0044cd910339cb4fd"
                        .to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidRewardAmount { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::InvalidRewardAmount"),
                }
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                    }],
                    cliff_duration: None,
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: "87bb9bf2d62ff8430e314a0d18d3134dd01afd98b75b487337a677322d20ad3d"
                        .to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );
}

#[test]
fn cant_claim_without_campaign() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);

    let alice = &suite.senders[0].clone();

    suite
        .instantiate_claimdrop_contract(Some(alice.to_string()))
        .claim(
            alice,
            Uint128::new(20_000u128),
            None,
            ALICE_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();

                match err {
                    ContractError::CampaignError { reason } => {
                        assert_eq!(reason, "there's not an active campaign");
                    }
                    _ => panic!("Wrong error type, should return ContractError::CampaignError"),
                }
            },
        );
}

#[test]
fn create_campaign_and_claim_single_distribution_type() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);

    let alice = &suite.senders[0].clone();
    let bob = &suite.senders[1].clone();
    let carol = &suite.senders[2].clone();
    let current_time = &suite.get_time();

    suite.instantiate_claimdrop_contract(Some(alice.to_string()));

    suite.manage_campaign(
        alice,
        CampaignAction::CreateCampaign {
            params: Box::new(CampaignParams {
                owner: None,
                name: "Test Airdrop I".to_string(),
                description: "This is an airdrop, 土金, ك".to_string(),
                reward_asset: coin(100_000, "uom"),
                distribution_type: vec![DistributionType::LumpSum {
                    percentage: Decimal::one(),
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                }],
                cliff_duration: None,
                start_time: current_time.seconds() + 1,
                end_time: current_time.seconds() + 172_800,
                merkle_root: MERKLE_ROOT.to_string(),
            }),
        },
        &coins(100_000, "uom"),
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    // claim
    suite.claim(
        alice,
        Uint128::new(20_000u128),
        None,
        ALICE_PROOFS,
        |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();

            match err {
                ContractError::CampaignError { reason } => {
                    assert_eq!(reason, "not started");
                }
                _ => panic!("Wrong error type, should return ContractError::CampaignError"),
            }
        },
    );

    suite.add_day();

    suite
        .claim(
            alice,
            // pretending to be entitled to more tokens than the campaign has to offer for this user
            Uint128::new(20_000u128),
            None,
            ALICE_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();

                match err {
                    ContractError::MerkleRootVerificationFailed { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::MerkleRootVerificationFailed"),
                }
            },
        )
        .claim(
            alice,
            Uint128::new(10_000u128),
            None,
            // provide wrong proofs
            BROKEN_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();

                match err {
                    ContractError::MerkleRootVerificationFailed { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::MerkleRootVerificationFailed"),
                }
            },
        )
        .claim(
            alice,
            Uint128::new(10_000u128),
            // try claiming for someone else, with the wrong proofs
            Some(bob.to_string()),
            ALICE_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();

                match err {
                    ContractError::MerkleRootVerificationFailed { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::MerkleRootVerificationFailed"),
                }
            },
        )
        .query_balance("uom", alice, |balance| {
            assert_eq!(balance, Uint128::new(999_900_000));
        })
        .query_balance("uom", bob, |balance| {
            assert_eq!(balance, Uint128::new(1_000_000_000));
        })
        // bob claims for alice
        .claim(
            bob,
            Uint128::new(10_000u128),
            Some(alice.to_string()),
            ALICE_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_balance("uom", alice, |balance| {
            assert_eq!(balance, Uint128::new(999_910_000));
        })
        .query_balance("uom", bob, |balance| {
            assert_eq!(balance, Uint128::new(1_000_000_000));
        })
        .claim(
            bob,
            Uint128::new(10_000u128),
            None,
            BOB_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_balance("uom", bob, |balance| {
            assert_eq!(balance, Uint128::new(1_000_010_000));
        })
        .query_campaign(|result| {
                let campaign = result.unwrap();
                assert_eq!(campaign.claimed, coin(20_000u128, "uom"));
            }
        );

    suite
        .add_week()
        .add_week()
        .add_week()
        .query_balance("uom", carol, |balance| {
            assert_eq!(balance, Uint128::new(1_000_000_000));
        })
        .claim(
            carol,
            Uint128::new(20_000u128),
            None,
            CAROL_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_balance("uom", carol, |balance| {
            assert_eq!(balance, Uint128::new(1_000_020_000));
        });
}

#[test]
fn claim_ended_campaign() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);

    let alice = &suite.senders[0].clone();
    let bob = &suite.senders[1].clone();
    let carol = &suite.senders[2].clone();
    let dan = &suite.senders[3].clone();
    let current_time = &suite.get_time();

    suite
        .instantiate_claimdrop_contract(Some(dan.to_string()))
        .manage_campaign(
            dan,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                    }],
                    cliff_duration: None,
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: MERKLE_ROOT.to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_campaign(|result| {
            let campaign = result.unwrap();
            assert_eq!(campaign.name, "Test Airdrop I");
        });

    // claim
    suite
        .add_day()
        .claim(
            bob,
            Uint128::new(10_000u128),
            Some(alice.to_string()),
            ALICE_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_balance("uom", bob, |balance| {
            assert_eq!(balance, Uint128::new(1_000_000_000));
        })
        .query_balance("uom", alice, |balance| {
            assert_eq!(balance, Uint128::new(1_000_010_000));
        })
        .claim(
            bob,
            Uint128::new(10_000u128),
            None,
            BOB_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_balance("uom", bob, |balance| {
            assert_eq!(balance, Uint128::new(1_000_010_000));
        })
        .query_balance("uom", alice, |balance| {
            assert_eq!(balance, Uint128::new(1_000_010_000));
        });

    suite
        .manage_campaign(
            // bob tries to end the campaign
            bob,
            CampaignAction::CloseCampaign {},
            &vec![coin(100_000, "uom")],
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::PaymentError { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::PaymentError"),
                }
            },
        )
        .manage_campaign(
            // bob tries to end the campaign
            bob,
            CampaignAction::CloseCampaign {},
            &[],
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::Unauthorized { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::Unauthorized"),
                }
            },
        )
        .query_campaign(|result| {
            let campaign = result.unwrap();
            assert_eq!(campaign.claimed, coin(20_000u128, "uom"));
        })
        .query_balance("uom", dan, |balance| {
            assert_eq!(balance, Uint128::new(999_900_000));
        })
        // change ownership of the campaign
        .update_ownership(
            dan,
            cw_ownable::Action::TransferOwnership {
                new_owner: alice.to_string(),
                expiry: None,
            },
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .update_ownership(
            alice,
            cw_ownable::Action::AcceptOwnership {},
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .manage_campaign(
            // alice should be able to, since she is the owner of the contract now
            alice,
            CampaignAction::CloseCampaign {},
            &[],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        // the owner of the campaign, dan, got the remaining tokens back, which were 80k as 20k
        // were claimed by bob and alice
        .query_balance("uom", dan, |balance| {
            assert_eq!(balance, Uint128::new(999_980_000));
        });

    // now carol tries to claim but it's too late
    suite.claim(
        carol,
        Uint128::new(20_000u128),
        None,
        CAROL_PROOFS,
        |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();

            match err {
                ContractError::CampaignError { reason } => {
                    assert_eq!(reason, "has been closed, cannot claim");
                }
                _ => panic!("Wrong error type, should return ContractError::CampaignError"),
            }
        },
    );
}

#[test]
fn query_claimed() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);

    let alice = &suite.senders[0].clone();
    let bob = &suite.senders[1].clone();
    let carol = &suite.senders[2].clone();
    let dan = &suite.senders[3].clone();
    let eva = &suite.senders[4].clone();
    let current_time = &suite.get_time();

    suite
        .instantiate_claimdrop_contract(Some(dan.to_string()))
        .manage_campaign(
            dan,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![
                        DistributionType::LumpSum {
                            percentage: Decimal::percent(25),
                            start_time: current_time.seconds(),
                            end_time: current_time.plus_days(7).seconds(),
                        },
                        DistributionType::LinearVesting {
                            percentage: Decimal::percent(75),
                            start_time: current_time.plus_days(7).seconds(),
                            end_time: current_time.plus_days(14).seconds(),
                        },
                    ],
                    cliff_duration: None,
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(14).seconds(),
                    merkle_root: MERKLE_ROOT.to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_campaign(|result| {
            let campaign = result.unwrap();
            assert_eq!(campaign.name, "Test Airdrop I");
        });

    suite.add_week();

    // claim
    // alice gets 2500 in lump sump and 7500 linearly.
    // of those 7500 linear tokens, she gets 1071 per day as the distribution lasts 7 days

    suite
        .claim(
            bob, //bob claims for alice
            Uint128::new(10_000u128),
            Some(alice.to_string()),
            ALICE_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_claimed(Some(alice), None, None, |result| {
            let claimed_response = result.unwrap();
            assert_eq!(claimed_response.claimed.len(), 1usize);
            assert_eq!(
                claimed_response.claimed[0],
                (alice.to_string(), coin(2_500u128, "uom"))
            );
        })
        .query_claimed(Some(bob), None, None, |result| {
            let claimed_response = result.unwrap();
            assert!(claimed_response.claimed.is_empty());
        });

    suite
        .add_day()
        .add_day()
        .claim(
            alice,
            Uint128::new(10_000u128),
            Some(alice.to_string()),
            ALICE_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_claimed(Some(alice), None, None, |result| {
            let claimed_response = result.unwrap();
            assert_eq!(claimed_response.claimed.len(), 1usize);
            assert_eq!(
                claimed_response.claimed[0],
                (alice.to_string(), coin(2_500u128 + 1071u128 * 2, "uom"))
            );
        })
        .claim(
            bob,
            Uint128::new(10_000u128),
            None,
            BOB_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_claimed(Some(bob), None, None, |result| {
            let claimed_response = result.unwrap();
            assert_eq!(claimed_response.claimed.len(), 1usize);
            assert_eq!(
                claimed_response.claimed[0],
                (bob.to_string(), coin(2_500u128 + 1071u128 * 2, "uom"))
            );
        });

    // move a week, the campaign should be over by now and everybody who claims will get the full amount
    // they were entitled to
    suite.add_week();

    suite
        .claim(
            alice,
            Uint128::new(10_000u128),
            Some(alice.to_string()),
            ALICE_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_claimed(Some(alice), None, None, |result| {
            let claimed_response = result.unwrap();
            assert_eq!(claimed_response.claimed.len(), 1usize);
            assert_eq!(
                claimed_response.claimed[0],
                (alice.to_string(), coin(10_000u128, "uom"))
            );
        })
        .claim(
            bob,
            Uint128::new(10_000u128),
            None,
            BOB_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .claim(
            carol,
            Uint128::new(20_000u128),
            None,
            CAROL_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .claim(
            dan,
            Uint128::new(35_000u128),
            None,
            DAN_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .claim(
            eva,
            Uint128::new(25_000u128),
            None,
            EVA_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_claimed(Some(eva), None, None, |result| {
            let claimed_response = result.unwrap();
            assert_eq!(claimed_response.claimed.len(), 1usize);
            assert_eq!(
                claimed_response.claimed[0],
                (eva.to_string(), coin(25_000u128, "uom"))
            );
        });

    // test pagination

    suite
        .query_claimed(None, None, None, |result| {
            let claimed_response = result.unwrap();
            assert_eq!(claimed_response.claimed.len(), 5usize);
            assert_eq!(
                claimed_response.claimed,
                vec![
                    (alice.to_string(), coin(10_000u128, "uom")),
                    (carol.to_string(), coin(20_000u128, "uom")),
                    (eva.to_string(), coin(25_000u128, "uom")),
                    (bob.to_string(), coin(10_000u128, "uom")),
                    (dan.to_string(), coin(35_000u128, "uom")),
                ]
            );
        })
        .query_claimed(None, Some(eva), None, |result| {
            let claimed_response = result.unwrap();
            assert_eq!(claimed_response.claimed.len(), 2usize);
            assert_eq!(
                claimed_response.claimed,
                vec![
                    (bob.to_string(), coin(10_000u128, "uom")),
                    (dan.to_string(), coin(35_000u128, "uom")),
                ]
            );
        })
        .query_claimed(None, None, Some(2), |result| {
            let claimed_response = result.unwrap();
            assert_eq!(claimed_response.claimed.len(), 2usize);
            assert_eq!(
                claimed_response.claimed,
                vec![
                    (alice.to_string(), coin(10_000u128, "uom")),
                    (carol.to_string(), coin(20_000u128, "uom")),
                ]
            );
        })
        .query_claimed(None, Some(carol), Some(2), |result| {
            let claimed_response = result.unwrap();
            assert_eq!(claimed_response.claimed.len(), 2usize);
            assert_eq!(
                claimed_response.claimed,
                vec![
                    (eva.to_string(), coin(25_000u128, "uom")),
                    (bob.to_string(), coin(10_000u128, "uom")),
                ]
            );
        });
}

#[test]
fn create_campaign_and_claim_multiple_distribution_types() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);

    let alice = &suite.senders[0].clone();
    let dan = &suite.senders[3].clone();
    let current_time = &suite.get_time();

    suite
        .instantiate_claimdrop_contract(Some(alice.to_string()))
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![
                        DistributionType::LumpSum {
                            percentage: Decimal::percent(25),
                            start_time: current_time.seconds(),
                            end_time: current_time.plus_days(7).seconds(),
                        },
                        DistributionType::LinearVesting {
                            percentage: Decimal::percent(75),
                            start_time: current_time.plus_days(7).seconds(),
                            end_time: current_time.plus_days(14).seconds(),
                        },
                    ],
                    cliff_duration: None,
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(14).seconds(),
                    merkle_root: MERKLE_ROOT.to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );

    suite.add_day();

    suite
        .claim(
            alice,
            Uint128::new(10_000u128),
            None,
            ALICE_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_campaign(|result| {
            let campaign = result.unwrap();
            assert_eq!(campaign.claimed, coin(2_500u128, "uom"));
        })
        // trying to claim again without moving time, should err
        .query_rewards(Uint128::new(10_000u128), alice, ALICE_PROOFS, |result| {
            assert_eq!(
                result.unwrap(),
                RewardsResponse {
                    claimed: coins(2_500u128, "uom"),
                    pending: coins(10_000u128 - 2_500u128, "uom"),
                    available_to_claim: vec![],
                }
            );
        })
        .claim(
            alice,
            Uint128::new(10_000u128),
            None,
            ALICE_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::NothingToClaim { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::NothingToClaim"),
                }
            },
        )
        .add_week()
        .claim(
            alice,
            Uint128::new(10_000u128),
            None,
            ALICE_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        // trying to claim again without moving time, should err
        .claim(
            alice,
            Uint128::new(10_000u128),
            None,
            ALICE_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::NothingToClaim { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::NothingToClaim"),
                }
            },
        )
        .add_day()
        .add_day()
        .add_day()
        .add_day()
        .query_rewards(Uint128::new(10_000u128), alice, ALICE_PROOFS, |result| {
            assert_eq!(
                result.unwrap(),
                RewardsResponse {
                    claimed: coins(3_571u128, "uom"),
                    pending: coins(10_000u128 - 3_571u128, "uom"),
                    available_to_claim: coins(4_285u128, "uom"),
                }
            );
        })
        .claim(
            alice,
            Uint128::new(10_000u128),
            None,
            ALICE_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        // add 2 more weeks and claim, the campaign should have finished by then
        .query_rewards(Uint128::new(10_000u128), alice, ALICE_PROOFS, |result| {
            assert_eq!(
                result.unwrap(),
                RewardsResponse {
                    claimed: coins(7_856u128, "uom"),
                    pending: coins(10_000u128 - 7_856u128, "uom"),
                    available_to_claim: vec![],
                }
            );
        })
        .add_week()
        .add_week()
        .claim(
            alice,
            Uint128::new(10_000u128),
            None,
            ALICE_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        // add a day and try claiming again, should err
        .add_day()
        .claim(
            alice,
            Uint128::new(10_000u128),
            None,
            ALICE_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::NothingToClaim { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::NothingToClaim"),
                }
            },
        )
        .query_campaign(|result| {
            let campaign = result.unwrap();
            assert_eq!(campaign.claimed, coin(10_000u128, "uom"));
        })
        // dan claiming all at once
        .query_rewards(Uint128::new(35_000u128), dan, DAN_PROOFS, |result| {
            assert_eq!(
                result.unwrap(),
                RewardsResponse {
                    claimed: vec![],
                    pending: coins(35_000u128, "uom"),
                    available_to_claim: coins(35_000u128, "uom"),
                }
            );
        })
        .claim(
            dan,
            Uint128::new(35_000u128),
            None,
            DAN_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_rewards(Uint128::new(35_000u128), dan, DAN_PROOFS, |result| {
            assert_eq!(
                result.unwrap(),
                RewardsResponse {
                    claimed: coins(35_000u128, "uom"),
                    pending: vec![],
                    available_to_claim: vec![],
                }
            );
        })
        .query_campaign(|result| {
            let campaign = result.unwrap();
            assert_eq!(campaign.claimed, coin(45_000u128, "uom"));
        });
}

#[test]
fn claim_campaign_with_cliff() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);

    let alice = &suite.senders[0].clone();
    let current_time = &suite.get_time();

    suite.instantiate_claimdrop_contract(None).manage_campaign(
        alice,
        CampaignAction::CreateCampaign {
            params: Box::new(CampaignParams {
                owner: None,
                name: "Test Airdrop I".to_string(),
                description: "This is an airdrop with cliff".to_string(),
                reward_asset: coin(100_000, "uom"),
                distribution_type: vec![DistributionType::LinearVesting {
                    percentage: Decimal::percent(100),
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(1460).seconds(), // 4 years
                }],
                cliff_duration: Some(86_400 * 365), // 1 year cliff
                start_time: current_time.seconds(),
                end_time: current_time.plus_days(1460).seconds(),
                merkle_root: MERKLE_ROOT.to_string(),
            }),
        },
        &coins(100_000, "uom"),
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    // can't claim before the cliff period is over
    suite.claim(
        alice,
        Uint128::new(10_000u128),
        None,
        ALICE_PROOFS,
        |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();
            match err {
                ContractError::CliffPeriodNotPassed { .. } => {}
                _ => panic!("Wrong error type, should return ContractError::CliffPeriodNotPassed"),
            }
        },
    );

    // move a few days to pass the cliff

    // move the remaining of the year - 1 day, 365 - 1 day
    for _ in 0..364 {
        suite.add_day();
    }

    suite.claim(
        alice,
        Uint128::new(10_000u128),
        None,
        ALICE_PROOFS,
        |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();
            match err {
                ContractError::CliffPeriodNotPassed { .. } => {}
                _ => panic!("Wrong error type, should return ContractError::CliffPeriodNotPassed"),
            }
        },
    );

    // add another day, total days passed 365, ready to claim some
    suite.add_day();

    suite.claim(
        alice,
        Uint128::new(10_000u128),
        None,
        ALICE_PROOFS,
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    suite
        .query_campaign(|result| {
            assert_eq!(result.unwrap().claimed, coin(10_000 / 4, "uom"));
        })
        .query_rewards(Uint128::new(10_000u128), alice, ALICE_PROOFS, |result| {
            assert_eq!(
                result.unwrap(),
                RewardsResponse {
                    claimed: coins(10_000 / 4, "uom"),
                    pending: coins(10_000u128 - (10_000 / 4), "uom"),
                    available_to_claim: vec![],
                }
            );
        });

    // advance another year
    for _ in 0..365 {
        suite.add_day();
    }

    suite
        .claim(
            alice,
            Uint128::new(10_000u128),
            None,
            ALICE_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_campaign(|result| {
            assert_eq!(result.unwrap().claimed, coin((10_000 / 4) * 2, "uom"));
        });

    // advance two more years, so the vesting period should be over
    for _ in 0..730 {
        suite.add_day();
    }

    suite
        .query_rewards(Uint128::new(10_000u128), alice, ALICE_PROOFS, |result| {
            assert_eq!(
                result.unwrap(),
                RewardsResponse {
                    claimed: coins((10_000 / 4) * 2, "uom"),
                    pending: coins(10_000u128 - ((10_000 / 4) * 2), "uom"),
                    available_to_claim: coins((10_000 / 4) * 2, "uom"),
                }
            );
        })
        .claim(
            alice,
            Uint128::new(10_000u128),
            None,
            ALICE_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        // add a week and claim
        .query_campaign(|result| {
            assert_eq!(result.unwrap().claimed, coin(10_000u128, "uom"));
        })
        .query_rewards(Uint128::new(10_000u128), alice, ALICE_PROOFS, |result| {
            assert_eq!(
                result.unwrap(),
                RewardsResponse {
                    claimed: coins(10_000u128, "uom"),
                    pending: vec![],
                    available_to_claim: vec![],
                }
            );
        });
}

#[test]
fn topup_campaigns_with_and_without_cliff() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);

    let alice = &suite.senders[0].clone();
    let bob = &suite.senders[1].clone();
    let carol = &suite.senders[2].clone();
    let dan = &suite.senders[3].clone();
    let eva = &suite.senders[4].clone();
    let current_time = &suite.get_time();

    suite.instantiate_claimdrop_contract(None).manage_campaign(
        alice,
        CampaignAction::CreateCampaign {
            params: Box::new(CampaignParams {
                owner: None,
                name: "Test Airdrop I".to_string(),
                description: "This is an airdrop with cliff".to_string(),
                reward_asset: coin(30_000, "uom"),
                distribution_type: vec![DistributionType::LinearVesting {
                    percentage: Decimal::percent(100),
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(30).seconds(), // a month
                }],
                cliff_duration: Some(86_400 * 7), // 7 days cliff
                start_time: current_time.seconds(),
                end_time: current_time.plus_days(30).seconds(),
                merkle_root: MERKLE_ROOT.to_string(),
            }),
        },
        &coins(30_000, "uom"),
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    suite.query_campaign(|result| {
        let campaign = result.unwrap();
        assert_eq!(campaign.reward_asset, coin(30_000, "uom"));
        assert!(campaign.cliff_duration.is_some());
    });

    for _ in 0..7 {
        suite.add_day();
    }

    // everybody claims
    suite
        .claim(
            alice,
            Uint128::new(10_000u128),
            None,
            ALICE_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .claim(
            bob,
            Uint128::new(10_000u128),
            None,
            BOB_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .claim(
            carol,
            Uint128::new(20_000u128),
            None,
            CAROL_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .claim(
            dan,
            Uint128::new(35_000u128),
            None,
            DAN_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .claim(
            eva,
            Uint128::new(25_000u128),
            None,
            EVA_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );

    suite.query_campaign(|result| {
        let campaign = result.unwrap();
        assert_eq!(campaign.claimed, coin(23_331u128, "uom"));
    });

    // topup campaign
    suite
        .manage_campaign(
            bob,
            CampaignAction::TopUpCampaign {},
            &coins(70_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::Unauthorized { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::Unauthorized"),
                }
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::TopUpCampaign {},
            &coins(70_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_campaign(|result| {
            let campaign = result.unwrap();
            assert_eq!(campaign.reward_asset, coin(100_000, "uom"));
        });

    // go to the time when the campaign already finished. Should fail topping up
    for _ in 0..23 {
        suite.add_day();
    }

    suite.manage_campaign(
        alice,
        CampaignAction::TopUpCampaign {},
        &coins(90_000, "uom"),
        |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();
            match err {
                ContractError::CampaignError { reason } => {
                    assert_eq!(reason, "campaign has ended");
                }
                _ => panic!("Wrong error type, should return ContractError::CampaignError"),
            }
        },
    );

    let current_time = &suite.get_time();

    // create a new contract / campaign

    suite
        .instantiate_claimdrop_contract(None)
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop II".to_string(),
                    description: "This is an airdrop without cliff".to_string(),
                    reward_asset: coin(50_000, "uom"),
                    distribution_type: vec![DistributionType::LinearVesting {
                        percentage: Decimal::percent(100),
                        start_time: current_time.seconds(),
                        end_time: current_time.plus_days(30).seconds(), // a month
                    }],
                    cliff_duration: None, // no cliff
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(30).seconds(),
                    merkle_root: MERKLE_ROOT.to_string(),
                }),
            },
            &coins(50_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_campaign(|result| {
            let campaign = result.unwrap();
            assert_eq!(campaign.reward_asset, coin(50_000, "uom"));
            assert!(campaign.cliff_duration.is_none());
        })
        .manage_campaign(
            alice,
            CampaignAction::TopUpCampaign {},
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_campaign(|result| {
            let campaign = result.unwrap();
            assert_eq!(campaign.reward_asset, coin(150_000, "uom"));
        });
}

#[test]
fn topup_campaign_with_more_funds_than_the_merkle_proof_dictates() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);

    let alice = &suite.senders[0].clone();
    let bob = &suite.senders[1].clone();
    let carol = &suite.senders[2].clone();
    let dan = &suite.senders[3].clone();
    let eva = &suite.senders[4].clone();
    let current_time = &suite.get_time();

    suite.instantiate_claimdrop_contract(None);

    let contract = suite.claimdrop_contract_addr.clone();

    suite
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop with cliff".to_string(),
                    reward_asset: coin(10_000, "uom"),
                    distribution_type: vec![DistributionType::LinearVesting {
                        percentage: Decimal::percent(100),
                        start_time: current_time.seconds(),
                        end_time: current_time.plus_days(30).seconds(), // a month
                    }],
                    cliff_duration: Some(86_400 * 7), // 7 days cliff
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(30).seconds(),
                    merkle_root: MERKLE_ROOT.to_string(),
                }),
            },
            &coins(10_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        // topup campaign
        .manage_campaign(
            alice,
            CampaignAction::TopUpCampaign {},
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );

    suite.query_campaign(|result| {
        let campaign = result.unwrap();
        assert_eq!(campaign.reward_asset, coin(110_000, "uom"));
    });

    // move a month
    for _ in 0..30 {
        suite.add_day();
    }

    // everybody claims they whole amount

    suite
        .claim(
            alice,
            Uint128::new(10_000u128),
            None,
            ALICE_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .claim(
            bob,
            Uint128::new(10_000u128),
            None,
            BOB_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .claim(
            carol,
            Uint128::new(20_000u128),
            None,
            CAROL_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .claim(
            dan,
            Uint128::new(35_000u128),
            None,
            DAN_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .claim(
            eva,
            Uint128::new(25_000u128),
            None,
            EVA_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );

    suite.query_campaign(|result| {
        let campaign = result.unwrap();
        // this is the most that can be claimed according to the merkle root
        assert_eq!(campaign.claimed, coin(100_000, "uom"));
    });

    suite.add_week();

    suite.query_balance("uom", &contract, |balance| {
        assert_eq!(balance, Uint128::new(10_000));
    });

    // trying to claim more, even though there are funds in the contract, should fail
    suite
        .claim(
            alice,
            Uint128::new(10_000u128),
            None,
            ALICE_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::NothingToClaim { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::NothingToClaim"),
                }
            },
        )
        .claim(
            bob,
            Uint128::new(10_000u128),
            None,
            BOB_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::NothingToClaim { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::NothingToClaim"),
                }
            },
        )
        .claim(
            carol,
            Uint128::new(20_000u128),
            None,
            CAROL_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::NothingToClaim { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::NothingToClaim"),
                }
            },
        )
        .claim(
            dan,
            Uint128::new(35_000u128),
            None,
            DAN_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::NothingToClaim { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::NothingToClaim"),
                }
            },
        )
        .claim(
            eva,
            Uint128::new(25_000u128),
            None,
            EVA_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::NothingToClaim { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::NothingToClaim"),
                }
            },
        );

    suite.query_balance("uom", &contract, |balance| {
        assert_eq!(balance, Uint128::new(10_000));
    });
}

#[test]
fn query_rewards() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);

    let alice = &suite.senders[0].clone();
    let current_time = &suite.get_time();

    suite.instantiate_claimdrop_contract(None).manage_campaign(
        alice,
        CampaignAction::CreateCampaign {
            params: Box::new(CampaignParams {
                owner: None,
                name: "Test Airdrop I".to_string(),
                description: "This is an airdrop with cliff".to_string(),
                reward_asset: coin(100_000, "uom"),
                distribution_type: vec![
                    DistributionType::LumpSum {
                        percentage: Decimal::percent(25),
                        start_time: current_time.seconds(),
                        end_time: current_time.plus_days(7).seconds(),
                    },
                    DistributionType::LinearVesting {
                        percentage: Decimal::percent(75),
                        start_time: current_time.plus_days(7).seconds(),
                        end_time: current_time.plus_days(14).seconds(),
                    },
                ],
                cliff_duration: None,
                start_time: current_time.seconds(),
                end_time: current_time.plus_days(14).seconds(),
                merkle_root: MERKLE_ROOT.to_string(),
            }),
        },
        &coins(100_000, "uom"),
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    suite
        .claim(
            alice,
            Uint128::new(10_000u128),
            None,
            ALICE_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_rewards(Uint128::new(35_000u128), alice, ALICE_PROOFS, |result| {
            let err = result.unwrap_err().to_string();

            assert_eq!(
                err,
                "Generic error: Querier contract error: Merkle root verification failed"
            );
        })
        .query_rewards(Uint128::new(20_000u128), alice, ALICE_PROOFS, |result| {
            let err = result.unwrap_err().to_string();

            assert_eq!(
                err,
                "Generic error: Querier contract error: Merkle root verification failed"
            );
        })
        .query_rewards(Uint128::new(10_000u128), alice, ALICE_PROOFS, |result| {
            assert_eq!(
                result.unwrap(),
                RewardsResponse {
                    claimed: coins(2_500u128, "uom"),
                    pending: coins(10_000u128 - 2_500u128, "uom"),
                    available_to_claim: vec![],
                }
            );
        })
        .manage_campaign(
            alice,
            CampaignAction::CloseCampaign {},
            &[],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_rewards(Uint128::new(10_000u128), alice, ALICE_PROOFS, |result| {
            let rewards_response = result.unwrap();
            assert_eq!(rewards_response.claimed, coins(2_500u128, "uom"));
            assert!(rewards_response.pending.is_empty());
            assert!(rewards_response.available_to_claim.is_empty());
        });
}

#[test]
fn query_rewards_fails_when_campaign_has_not_started() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);

    let alice = &suite.senders[0].clone();
    let current_time = &suite.get_time();

    suite.instantiate_claimdrop_contract(None).manage_campaign(
        alice,
        CampaignAction::CreateCampaign {
            params: Box::new(CampaignParams {
                owner: None,
                name: "Test Airdrop I".to_string(),
                description: "This is an airdrop with cliff".to_string(),
                reward_asset: coin(100_000, "uom"),
                distribution_type: vec![
                    DistributionType::LumpSum {
                        percentage: Decimal::percent(25),
                        start_time: current_time.seconds(),
                        end_time: current_time.plus_days(7).seconds(),
                    },
                    DistributionType::LinearVesting {
                        percentage: Decimal::percent(75),
                        start_time: current_time.plus_days(7).seconds(),
                        end_time: current_time.plus_days(14).seconds(),
                    },
                ],
                cliff_duration: None,
                start_time: current_time.plus_days(1).seconds(),
                end_time: current_time.plus_days(14).seconds(),
                merkle_root: MERKLE_ROOT.to_string(),
            }),
        },
        &coins(100_000, "uom"),
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    suite
        .claim(
            alice,
            Uint128::new(10_000u128),
            None,
            ALICE_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();

                match err {
                    ContractError::CampaignError { reason } => {
                        assert_eq!(reason, "not started");
                    }
                    _ => panic!("Wrong error type, should return ContractError::CampaignError"),
                }
            },
        )
        .query_rewards(Uint128::new(10_000u128), alice, ALICE_PROOFS, |result| {
            let err = result.unwrap_err().to_string();
            assert!(err.contains("not started"));
        });

    // move some epochs to make campaign active
    // the query is successful
    suite
        .add_week()
        .query_rewards(Uint128::new(10_000u128), alice, ALICE_PROOFS, |result| {
            result.unwrap();
        });
}

#[test]
fn close_campaigns() {
    let mut suite = TestingSuite::default_with_balances(vec![coin(1_000_000_000, "uom")]);

    let alice = &suite.senders[0].clone();
    let bob = &suite.senders[1].clone();
    let carol = &suite.senders[2].clone();
    let dan = &suite.senders[3].clone();
    let current_time = &suite.get_time();

    suite.instantiate_claimdrop_contract(None).manage_campaign(
        alice,
        CampaignAction::CreateCampaign {
            params: Box::new(CampaignParams {
                owner: Some(dan.to_string()),
                name: "Test Airdrop I".to_string(),
                description: "This is an airdrop with cliff".to_string(),
                reward_asset: coin(100_000, "uom"),
                distribution_type: vec![
                    DistributionType::LumpSum {
                        percentage: Decimal::percent(25),
                        start_time: current_time.seconds(),
                        end_time: current_time.plus_days(7).seconds(),
                    },
                    DistributionType::LinearVesting {
                        percentage: Decimal::percent(75),
                        start_time: current_time.plus_days(7).seconds(),
                        end_time: current_time.plus_days(14).seconds(),
                    },
                ],
                cliff_duration: None,
                start_time: current_time.seconds(),
                end_time: current_time.plus_days(14).seconds(),
                merkle_root: MERKLE_ROOT.to_string(),
            }),
        },
        &coins(100_000, "uom"),
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    //end campaign
    suite
        .manage_campaign(
            bob,
            CampaignAction::CloseCampaign {},
            &[],
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::Unauthorized { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::Unauthorized"),
                }
            },
        )
        .manage_campaign(
            carol,
            CampaignAction::CloseCampaign {},
            &[],
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::Unauthorized { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::Unauthorized"),
                }
            },
        )
        .query_campaign(|result| {
            let campaign = result.unwrap();
            assert!(campaign.closed.is_none());
        })
        .manage_campaign(
            dan, // dan can end the campaign since it's the owner of the campaign
            CampaignAction::CloseCampaign {},
            &[],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .manage_campaign(
            dan, // dan tries closing the campaign again
            CampaignAction::CloseCampaign {},
            &[],
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::CampaignError { reason } => {
                        assert_eq!(reason, "campaign has already been closed");
                    }
                    _ => panic!("Wrong error type, should return ContractError::CampaignError"),
                }
            },
        )
        .query_campaign(|result| {
            let campaign = result.unwrap();
            assert!(campaign.closed.is_some());
        })
        // try top up, should fail
        .manage_campaign(
            dan,
            CampaignAction::TopUpCampaign {},
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::CampaignError { reason } => {
                        assert_eq!(reason, "campaign has been closed");
                    }
                    _ => panic!("Wrong error type, should return ContractError::CampaignError"),
                }
            },
        );

    // let's create a new campaign
    suite.instantiate_claimdrop_contract(None).manage_campaign(
        alice,
        CampaignAction::CreateCampaign {
            params: Box::new(CampaignParams {
                owner: Some(dan.to_string()),
                name: "Test Airdrop I".to_string(),
                description: "This is an airdrop with cliff".to_string(),
                reward_asset: coin(100_000, "uom"),
                distribution_type: vec![
                    DistributionType::LumpSum {
                        percentage: Decimal::percent(25),
                        start_time: current_time.seconds(),
                        end_time: current_time.plus_days(7).seconds(),
                    },
                    DistributionType::LinearVesting {
                        percentage: Decimal::percent(75),
                        start_time: current_time.plus_days(7).seconds(),
                        end_time: current_time.plus_days(14).seconds(),
                    },
                ],
                cliff_duration: None,
                start_time: current_time.seconds(),
                end_time: current_time.plus_days(14).seconds(),
                merkle_root: MERKLE_ROOT.to_string(),
            }),
        },
        &coins(100_000, "uom"),
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    suite.manage_campaign(
        carol, // carol can't since it's not the owner of the contract nor the owner of the campaign
        CampaignAction::CloseCampaign {},
        &[],
        |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();
            match err {
                ContractError::Unauthorized { .. } => {}
                _ => panic!("Wrong error type, should return ContractError::Unauthorized"),
            }
        },
    );

    // let's update the ownership of the contract
    suite
        .update_ownership(
            alice,
            cw_ownable::Action::TransferOwnership {
                new_owner: carol.to_string(),
                expiry: None,
            },
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .update_ownership(
            carol,
            cw_ownable::Action::AcceptOwnership {},
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );

    // now only dan or carol can end the new campaign.
    suite
        .manage_campaign(
            alice, // alice can't since it renounce the ownership of the contract
            CampaignAction::CloseCampaign {},
            &[],
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::Unauthorized { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::Unauthorized"),
                }
            },
        )
        .manage_campaign(
            carol, // alice can't since it renounce the ownership of the contract
            CampaignAction::CloseCampaign {},
            &[],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );
}

#[test]
fn can_query_claims_after_campaign_is_closed() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);

    let alice = &suite.senders[0].clone();
    let bob = &suite.senders[1].clone();
    let dan = &suite.senders[3].clone();
    let current_time = &suite.get_time();

    suite
        .instantiate_claimdrop_contract(Some(dan.to_string()))
        .manage_campaign(
            dan,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![
                        DistributionType::LumpSum {
                            percentage: Decimal::percent(25),
                            start_time: current_time.seconds(),
                            end_time: current_time.plus_days(7).seconds(),
                        },
                        DistributionType::LinearVesting {
                            percentage: Decimal::percent(75),
                            start_time: current_time.plus_days(7).seconds(),
                            end_time: current_time.plus_days(14).seconds(),
                        },
                    ],
                    cliff_duration: None,
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(14).seconds(),
                    merkle_root: MERKLE_ROOT.to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_campaign(|result| {
            let campaign = result.unwrap();
            assert_eq!(campaign.name, "Test Airdrop I");
        });

    suite.add_week();

    // claim
    // alice gets 2500 in lump sump and 7500 linearly.
    // of those 7500 linear tokens, she gets 1071 per day as the distribution lasts 7 days

    suite
        .claim(
            bob, //bob claims for alice
            Uint128::new(10_000u128),
            Some(alice.to_string()),
            ALICE_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_claimed(Some(alice), None, None, |result| {
            let claimed_response = result.unwrap();
            assert_eq!(claimed_response.claimed.len(), 1usize);
            assert_eq!(
                claimed_response.claimed[0],
                (alice.to_string(), coin(2_500u128, "uom"))
            );
        })
        .query_claimed(Some(bob), None, None, |result| {
            let claimed_response = result.unwrap();
            assert!(claimed_response.claimed.is_empty());
        });

    suite.add_week();

    suite.query_rewards(Uint128::new(10_000u128), alice, ALICE_PROOFS, |result| {
        assert_eq!(
            result.unwrap(),
            RewardsResponse {
                claimed: coins(2_500u128, "uom"),
                pending: coins(10_000u128 - 2_500u128, "uom"),
                available_to_claim: vec![coin(7_500, "uom")],
            }
        );
    });

    // closing campaign
    suite
        .query_campaign(|result| {
            let campaign = result.unwrap();
            assert_eq!(campaign.claimed, coin(2_500u128, "uom"));
            assert_eq!(
                campaign
                    .reward_asset
                    .amount
                    .saturating_sub(campaign.claimed.amount),
                Uint128::new(100_000 - 2_500)
            );
        })
        .query_balance("uom", dan, |result| {
            assert_eq!(result, Uint128::new(1_000_000_000 - 100_000));
        })
        .manage_campaign(
            dan,
            CampaignAction::CloseCampaign {},
            &[],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_balance("uom", dan, |result| {
            assert_eq!(result, Uint128::new(1_000_000_000 - 2_500));
        })
        .query_rewards(Uint128::new(10_000u128), alice, ALICE_PROOFS, |result| {
            let rewards_response = result.unwrap();
            assert_eq!(rewards_response.claimed, coins(2_500u128, "uom"));
            assert!(rewards_response.pending.is_empty());
            assert!(rewards_response.available_to_claim.is_empty());
        })
        .query_claimed(None, None, None, |result| {
            let claimed_response = result.unwrap();
            assert_eq!(claimed_response.claimed.len(), 1usize);
            assert_eq!(
                claimed_response.claimed[0],
                (alice.to_string(), coin(2_500u128, "uom"))
            );
        });
}

#[test]
fn renouncing_contract_owner_makes_prevents_creating_campaigns() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);

    let alice = &suite.senders[0].clone();
    let bob = &suite.senders[1].clone();
    let current_time = &suite.get_time();

    suite.instantiate_claimdrop_contract(None).manage_campaign(
        alice,
        CampaignAction::CreateCampaign {
            params: Box::new(CampaignParams {
                owner: None,
                name: "Test Airdrop I".to_string(),
                description: "This is an airdrop, 土金, ك".to_string(),
                reward_asset: coin(100_000, "uom"),
                distribution_type: vec![
                    DistributionType::LumpSum {
                        percentage: Decimal::percent(25),
                        start_time: current_time.seconds(),
                        end_time: current_time.plus_days(7).seconds(),
                    },
                    DistributionType::LinearVesting {
                        percentage: Decimal::percent(75),
                        start_time: current_time.plus_days(7).seconds(),
                        end_time: current_time.plus_days(14).seconds(),
                    },
                ],
                cliff_duration: None,
                start_time: current_time.seconds(),
                end_time: current_time.plus_days(14).seconds(),
                merkle_root: MERKLE_ROOT.to_string(),
            }),
        },
        &coins(100_000, "uom"),
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    suite.add_week();

    // can claim
    suite.claim(
        bob,
        Uint128::new(10_000u128),
        None,
        BOB_PROOFS,
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    suite
        .update_ownership(
            bob,
            cw_ownable::Action::RenounceOwnership {},
            |result: Result<AppResponse, anyhow::Error>| {
                //error
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::OwnershipError(e) => match e {
                        OwnershipError::NotOwner => {}
                        _ => panic!("Wrong error type, should return OwnershipError::NotOwner"),
                    },
                    _ => panic!("Wrong error type, should return ContractError::OwnershipError"),
                }
            },
        )
        .update_ownership(
            alice,
            cw_ownable::Action::RenounceOwnership {},
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        // can claim
        .claim(
            alice,
            Uint128::new(10_000u128),
            None,
            ALICE_PROOFS,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        //end campaign
        .manage_campaign(
            alice,
            CampaignAction::CloseCampaign {},
            &[],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        // intended
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop II".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![
                        DistributionType::LumpSum {
                            percentage: Decimal::percent(25),
                            start_time: current_time.seconds(),
                            end_time: current_time.plus_days(7).seconds(),
                        },
                        DistributionType::LinearVesting {
                            percentage: Decimal::percent(75),
                            start_time: current_time.plus_days(7).seconds(),
                            end_time: current_time.plus_days(14).seconds(),
                        },
                    ],
                    cliff_duration: None,
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(14).seconds(),
                    merkle_root: MERKLE_ROOT.to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::OwnershipError(e) => match e {
                        OwnershipError::NoOwner => {}
                        _ => panic!("Wrong error type, should return OwnershipError::NoOwner"),
                    },
                    _ => panic!("Wrong error type, should return ContractError::OwnershipError"),
                }
            },
        );
}

#[test]
fn can_claim_dust_without_new_claims() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);

    let alice = &suite.senders[0].clone();
    let current_time = &suite.get_time();

    suite
        .instantiate_claimdrop_contract(Some(alice.to_string()))
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(23, "uom"),
                    distribution_type: vec![DistributionType::LinearVesting {
                        percentage: Decimal::percent(100),
                        start_time: current_time.seconds(),
                        end_time: current_time.plus_days(60).seconds(),
                    }],
                    cliff_duration: None,
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(60).seconds(),
                    merkle_root: MERKLE_ROOT_X.to_string(),
                }),
            },
            &coins(23, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_campaign(|result| {
            let campaign = result.unwrap();
            assert_eq!(campaign.name, "Test Airdrop I");
        });

    for _ in 0..59 {
        suite.add_day();
    }

    suite
        .claim(
            alice,
            Uint128::new(17u128),
            Some(alice.to_string()),
            ALICE_PROOFS_X,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_claimed(Some(alice), None, None, |result| {
            let claimed_response = result.unwrap();
            assert_eq!(claimed_response.claimed.len(), 1usize);
            assert_eq!(
                claimed_response.claimed[0],
                (alice.to_string(), coin(16u128, "uom"))
            );
        });

    suite.add_week();

    // executing the claiming here, will result on the compute_claimable_amount::new_claims being empty,
    // as the claim_amount will be zero, while the rounding_error_compensation_amount will be 1.
    suite
        .claim(
            alice,
            Uint128::new(17u128),
            Some(alice.to_string()),
            ALICE_PROOFS_X,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_claimed(Some(alice), None, None, |result| {
            let claimed_response = result.unwrap();
            assert_eq!(claimed_response.claimed.len(), 1usize);
            assert_eq!(
                claimed_response.claimed[0],
                (alice.to_string(), coin(17u128, "uom"))
            );
        });
}

#[test]
fn cant_end_distribution_type_after_campaign() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);
    let alice = &suite.senders[0].clone();
    let current_time = &suite.get_time();

    suite
        .instantiate_claimdrop_contract(None)
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LinearVesting {
                        percentage: Decimal::percent(100),
                        start_time: current_time.seconds(),
                        end_time: current_time.plus_days(30).seconds(),
                    }],
                    cliff_duration: None,
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(7).seconds(),
                    merkle_root: MERKLE_ROOT.to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidEndDistributionTime { .. } => {}
                    _ => panic!(
                        "Wrong error type, should return ContractError::InvalidEndDistributionTime"
                    ),
                }
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    owner: None,
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LinearVesting {
                        percentage: Decimal::percent(100),
                        start_time: current_time.seconds(),
                        end_time: current_time.plus_days(7).seconds(),
                    }],
                    cliff_duration: None,
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(7).seconds(),
                    merkle_root: MERKLE_ROOT.to_string(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );

    suite.instantiate_claimdrop_contract(None).manage_campaign(
        alice,
        CampaignAction::CreateCampaign {
            params: Box::new(CampaignParams {
                owner: None,
                name: "Test Airdrop I".to_string(),
                description: "This is an airdrop, 土金, ك".to_string(),
                reward_asset: coin(100_000, "uom"),
                distribution_type: vec![DistributionType::LinearVesting {
                    percentage: Decimal::percent(100),
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(6).seconds(),
                }],
                cliff_duration: None,
                start_time: current_time.seconds(),
                end_time: current_time.plus_days(7).seconds(),
                merkle_root: MERKLE_ROOT.to_string(),
            }),
        },
        &coins(100_000, "uom"),
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );
}

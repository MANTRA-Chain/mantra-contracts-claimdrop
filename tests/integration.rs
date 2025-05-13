use std::str::FromStr;

use cosmwasm_std::{coin, coins, Decimal, Uint128};
use cw_multi_test::AppResponse;
use cw_ownable::OwnershipError;
use cw_utils::PaymentError;

use crate::suite::TestingSuite;
use claimdrop_contract::error::ContractError;
use claimdrop_contract::msg::{CampaignAction, CampaignParams, DistributionType, RewardsResponse};
use claimdrop_contract::queries::query_allocation;

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
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                    }],
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
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
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                    }],
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
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
                    name: "Test Airdrop II".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                    }],
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
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
                    name: "".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                    }],
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
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
                    name: "".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                    }],
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
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
                name: "%WEmpcQxf5ONYNy1Gj2m#w7oauP6E5OUoZM1n7AnTRDX9sSd5DK%WEmpcQxf5ONYNy1Gj2m#\
                    w7oauP6E5OUoZM1n7AnTRDX9sSd5D%WEmpcQxf5ONYNy1Gj2m#w7oauP6E5OUoZM1n7AnTRDX9sSd5D\
                    %WEmpcQxf5ONYNy1Gj2m#w7oauP6E5OUoZM1n7AnTRDX9sSd5D".to_string(),
                description: "This is an airdrop, 土金, ك".to_string(),
                reward_denom: "uom".to_string(),
                total_reward: coin(100_000, "uom"),
                distribution_type: vec![DistributionType::LumpSum {
                    percentage: Decimal::one(),
                    start_time: current_time.seconds() + 1,
                }],
                start_time: current_time.seconds() + 1,
                end_time: current_time.seconds() + 172_800,
            }),
        },
        &coins(100_000, "uom"),
        |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();
            match err {
                ContractError::InvalidCampaignParam { param, reason } => {
                    assert_eq!(param, "name");
                    assert_eq!(reason, "cannot be longer than 200 characters");
                }
                _ => panic!("Wrong error type, should return ContractError::InvalidCampaignParam"),
            }
        },
    )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop I".to_string(),
                    description: "".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                    }],
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
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
                    name: "Test Airdrop I".to_string(),
                    description: "bxOmDxCEaGwURwYOretJXoSBZxKtqvRzQYFUzBvzrYHqVGywHYeWvApTYETxADYdHDeBDTbKXeAzCVPhvxzTBxNtFhUzPYCgrEBP\
                bxOmDxCEaGwURwYOretJXoSBZxKtqvRzQYFUzBvzrYHqVGywHYeWvApTYETxADYdHDeBDTbKXeAzCVPhvxzTBxNtFhUzPYCgrEBP\
                bxOmDxCEaGwURwYOretJXoSBZxKtqvRzQYFUzBvzrYHqVGywHYeWvApTYETxADYdHDeBDTbKXeAzCVPhvxzTBxNtFhUzPYCgrEBP\
                bxOmDxCEaGwURwYOretJXoSBZxKtqvRzQYFUzBvzrYHqVGywHYeWvApTYETxADYdHDeBDTbKXeAzCVPhvxzTBxNtFhUzPYCgrEBP\
                bxOmDxCEaGwURwYOretJXoSBZxKtqvRzQYFUzBvzrYHqVGywHYeWvApTYETxADYdHDeBDTbKXeAzCVPhvxzTBxNtFhUzPYCgrEBP\
                bxOmDxCEaGwURwYOretJXoSBZxKtqvRzQYFUzBvzrYHqVGywHYeWvApTYETxADYdHDeBDTbKXeAzCVPhvxzTBxNtFhUzPYCgrEBP\
                bxOmDxCEaGwURwYOretJXoSBZxKtqvRzQYFUzBvzrYHqVGywHYeWvApTYETxADYdHDeBDTbKXeAzCVPhvxzTBxNtFhUzPYCgrEBP\
                bxOmDxCEaGwURwYOretJXoSBZxKtqvRzQYFUzBvzrYHqVGywHYeWvApTYETxADYdHDeBDTbKXeAzCVPhvxzTBxNtFhUzPYCgrEBP\
                bxOmDxCEaGwURwYOretJXoSBZxKtqvRzQYFUzBvzrYHqVGywHYeWvApTYETxADYdHDeBDTbKXeAzCVPhvxzTBxNtFhUzPYCgrEBP\
                bxOmDxCEaGwURwYOretJXoSBZxKtqvRzQYFUzBvzrYHqVGywHYeWvApTYETxADYdHDeBDTbKXeAzCVPhvxzTBxNtFhUzPYCgrEBP\
                bxOmDxCEaGwURwYOretJXoSBZxKtqvRzQYFUzBvzrYHqVGywHYeWvApTYETxADYdHDeBDTbKXeAzCVPhvxzTBxNtFhUzPYCgrEBP\
                bxOmDxCEaGwURwYOretJXoSBZxKtqvRzQYFUzBvzrYHqVGywHYeWvApTYETxADYdHDeBDTbKXeAzCVPhvxzTBxNtFhUzPYCgrEBP\
                bxOmDxCEaGwURwYOretJXoSBZxKtqvRzQYFUzBvzrYHqVGywHYeWvApTYETxADYdHDeBDTbKXeAzCVPhvxzTBxNtFhUzPYCgrEBP\
                bxOmDxCEaGwURwYOretJXoSBZxKtqvRzQYFUzBvzrYHqVGywHYeWvApTYETxADYdHDeBDTbKXeAzCVPhvxzTBxNtFhUzPYCgrEBP\
                bxOmDxCEaGwURwYOretJXoSBZxKtqvRzQYFUzBvzrYHqVGywHYeWvApTYETxADYdHDeBDTbKXeAzCVPhvxzTBxNtFhUzPYCgrEBP\
                bxOmDxCEaGwURwYOretJXoSBZxKtqvRzQYFUzBvzrYHqVGywHYeWvApTYETxADYdHDeBDTbKXeAzCVPhvxzTBxNtFhUzPYCgrEBP\
                bxOmDxCEaGwURwYOretJXoSBZxKtqvRzQYFUzBvzrYHqVGywHYeWvApTYETxADYdHDeBDTbKXeAzCVPhvxzTBxNtFhUzPYCgrEBP\
                bxOmDxCEaGwURwYOretJXoSBZxKtqvRzQYFUzBvzrYHqVGywHYeWvApTYETxADYdHDeBDTbKXeAzCVPhvxzTBxNtFhUzPYCgrEBP\
                bxOmDxCEaGwURwYOretJXoSBZxKtqvRzQYFUzBvzrYHqVGywHYeWvApTYETxADYdHDeBDTbKXeAzCVPhvxzTBxNtFhUzPYCgrEBP\
                bxOmDxCEaGwURwYOretJXoSBZxKtqvRzQYFUzBvzrYHqVGywHYeWvApTYETxADYdHDeBDTbKXeAzCVPhvxzTBxNtFhUzPYCgrEBP1".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                    }],
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidCampaignParam { param, reason } => {
                        assert_eq!(param, "description");
                        assert_eq!(reason, "cannot be longer than 2000 characters");
                    }
                    _ => panic!("Wrong error type, should return ContractError::InvalidCampaignParam"),
                }
            },
        )
        // campaign times
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                    }],
                    start_time: current_time.seconds() + 172_800,
                    end_time: current_time.seconds() + 1,
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
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                    }],
                    start_time: current_time.seconds() - 100,
                    end_time: current_time.seconds() + 1,
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
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![],
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
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
                name: "Test Airdrop I".to_string(),
                description: "This is an airdrop, 土金, ك".to_string(),
                reward_denom: "uom".to_string(),
                total_reward: coin(100_000, "uom"),
                distribution_type: vec![
                    DistributionType::LumpSum {
                        percentage: Decimal::from_str("2").unwrap(),
                        start_time: current_time.seconds() + 1,
                    },
                    DistributionType::LumpSum {
                        percentage: Decimal::from_str("2").unwrap(),
                        start_time: current_time.seconds() + 1,
                    },
                    DistributionType::LumpSum {
                        percentage: Decimal::from_str("2").unwrap(),
                        start_time: current_time.seconds() + 1,
                    },
                ],
                start_time: current_time.seconds() + 1,
                end_time: current_time.seconds() + 172_800,
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
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::from_str("2").unwrap(),
                        start_time: current_time.seconds() + 1,
                    }],
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
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
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::from_str("0.2").unwrap(),
                        start_time: current_time.seconds() + 1,
                    }],
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
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
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::zero(),
                        start_time: current_time.seconds() + 1,
                    }],
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::ZeroDistributionPercentage => {}
                    _ => panic!("Wrong error type, should return ContractError::ZeroDistributionPercentage"),
                }
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() - 1,
                    }],
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidStartDistributionTime { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::InvalidStartDistributionTime"),
                }
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds(),
                    }],
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidStartDistributionTime { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::InvalidStartDistributionTime"),
                }
            },
        )

        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![
                        DistributionType::LumpSum {
                            percentage: Decimal::from_str("0.5").unwrap(),
                            start_time: current_time.seconds() + 1,
                        },
                        DistributionType::LumpSum {
                            percentage: Decimal::one(),
                            start_time: current_time.seconds() + 1,
                        }
                    ],
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,

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
        )
        //cliff
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LinearVesting {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                        end_time: current_time.seconds() + 172_800,
                        cliff_duration: Some(0u64),
                    }],
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,

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
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LinearVesting {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds(),
                        end_time: current_time.plus_days(7).seconds(),
                        cliff_duration: Some(7 * 86_400u64),
                    }],
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(7).seconds(),

                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidCampaignParam { param, reason } => {
                        assert_eq!(param, "cliff_duration");
                        assert_eq!(reason, "cannot be greater or equal than the distribution duration");
                    }
                    _ => panic!("Wrong error type, should return ContractError::InvalidCampaignParam"),
                }
            },
        )
        // rewards
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop II".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uosmo".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                    }],
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidCampaignParam { reason, param } => {
                        assert_eq!(param, "reward_denom");
                        assert_eq!(reason, "reward denom mismatch");
                    }
                    _ => panic!("Wrong error type, should return ContractError::InvalidCampaignParam"),
                };
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop II".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(0, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                    }],
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidCampaignParam { reason, param } => {
                        assert_eq!(param, "total_reward");
                        assert_eq!(reason, "cannot be zero");
                    }
                    _ => panic!("Wrong error type, should return ContractError::CampaignError"),
                };
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                    }],
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,

                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )

    ;
}

#[test]
fn cannot_start_distribution_in_past() {
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
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LinearVesting {
                        percentage: Decimal::percent(100),
                        start_time: current_time.minus_days(10).seconds(),
                        end_time: current_time.plus_days(60).seconds(),
                        cliff_duration: None,
                    }],
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(60).seconds(),
                }),
            },
            &[],
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidStartDistributionTime { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::InvalidStartDistributionTime"),
                }
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
        .claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();

            match err {
                ContractError::CampaignError { reason } => {
                    assert_eq!(reason, "there's not an active campaign");
                }
                _ => panic!("Wrong error type, should return ContractError::CampaignError"),
            }
        });
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

    let allocations = &vec![
        (alice.to_string(), Uint128::new(10_000)),
        (bob.to_string(), Uint128::new(10_000)),
        (carol.to_string(), Uint128::new(20_000)),
    ];

    suite
        .instantiate_claimdrop_contract(Some(alice.to_string()))
        .add_allocations(
            alice,
            allocations,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );

    suite.manage_campaign(
        alice,
        CampaignAction::CreateCampaign {
            params: Box::new(CampaignParams {
                name: "Test Airdrop I".to_string(),
                description: "This is an airdrop, 土金, ك".to_string(),
                reward_denom: "uom".to_string(),
                total_reward: coin(100_000, "uom"),
                distribution_type: vec![DistributionType::LumpSum {
                    percentage: Decimal::one(),
                    start_time: current_time.seconds() + 1,
                }],
                start_time: current_time.seconds() + 1,
                end_time: current_time.seconds() + 172_800,
            }),
        },
        &coins(100_000, "uom"),
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    // claim
    suite.claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
        let err = result.unwrap_err().downcast::<ContractError>().unwrap();

        match err {
            ContractError::CampaignError { reason } => {
                assert_eq!(reason, "not started");
            }
            _ => panic!("Wrong error type, should return ContractError::CampaignError"),
        }
    });

    suite.add_day();

    suite
        .query_balance("uom", alice, |balance| {
            assert_eq!(balance, Uint128::new(999_900_000));
        })
        .query_balance("uom", bob, |balance| {
            assert_eq!(balance, Uint128::new(1_000_000_000));
        })
        // bob claims for alice
        .claim(
            bob,
            Some(alice.to_string()),
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
        .claim(bob, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
        .query_balance("uom", bob, |balance| {
            assert_eq!(balance, Uint128::new(1_000_010_000));
        })
        .query_campaign(|result| {
            let campaign = result.unwrap();
            assert_eq!(campaign.claimed, coin(20_000u128, "uom"));
        });

    suite
        .add_week()
        .add_week()
        .add_week()
        .query_balance("uom", carol, |balance| {
            assert_eq!(balance, Uint128::new(1_000_000_000));
        })
        .claim(carol, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
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

    let allocations = &vec![
        (alice.to_string(), Uint128::new(10_000)),
        (bob.to_string(), Uint128::new(10_000)),
        (carol.to_string(), Uint128::new(20_000)),
    ];

    suite
        .instantiate_claimdrop_contract(Some(dan.to_string()))
        .add_allocations(
            dan,
            allocations,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .manage_campaign(
            dan,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::one(),
                        start_time: current_time.seconds() + 1,
                    }],
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
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
            Some(alice.to_string()),
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
        .claim(bob, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
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
            &[coin(100_000, "uom")],
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
                    ContractError::OwnershipError { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::OwnershipError"),
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
        // Since dan gave ownership to alice, he doesn't get the remaining tokens back.
        .query_balance("uom", dan, |balance| {
            assert_eq!(balance, Uint128::new(999_900_000));
        })
        // alice got the remaining tokens back, which were 80k as 20k as were claimed by bob and herself
        .query_balance("uom", alice, |balance| {
            assert_eq!(balance, Uint128::new(1_000_090_000));
        });

    // now carol tries to claim but it's too late
    suite.claim(carol, None, |result: Result<AppResponse, anyhow::Error>| {
        let err = result.unwrap_err().downcast::<ContractError>().unwrap();

        match err {
            ContractError::CampaignError { reason } => {
                assert_eq!(reason, "has been closed, cannot claim");
            }
            _ => panic!("Wrong error type, should return ContractError::CampaignError"),
        }
    });
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

    let allocations = &vec![
        (alice.to_string(), Uint128::new(10_000)),
        (bob.to_string(), Uint128::new(10_000)),
        (carol.to_string(), Uint128::new(20_000)),
        (dan.to_string(), Uint128::new(35_000)),
        (eva.to_string(), Uint128::new(25_000)),
    ];

    suite
        .instantiate_claimdrop_contract(Some(dan.to_string()))
        .add_allocations(
            dan,
            allocations,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .manage_campaign(
            dan,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![
                        DistributionType::LumpSum {
                            percentage: Decimal::percent(25),
                            start_time: current_time.seconds(),
                        },
                        DistributionType::LinearVesting {
                            percentage: Decimal::percent(75),
                            start_time: current_time.plus_days(7).seconds(),
                            end_time: current_time.plus_days(14).seconds(),
                            cliff_duration: None,
                        },
                    ],
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(14).seconds(),
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
            Some(alice.to_string()),
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
            Some(alice.to_string()),
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
        .claim(bob, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
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
            Some(alice.to_string()),
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
        .claim(bob, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
        .claim(carol, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
        .claim(dan, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
        .claim(eva, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
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

    let allocations = &vec![
        (alice.to_string(), Uint128::new(10_000)),
        (dan.to_string(), Uint128::new(35_000)),
    ];

    suite
        .instantiate_claimdrop_contract(Some(alice.to_string()))
        .add_allocations(
            alice,
            allocations,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![
                        DistributionType::LumpSum {
                            percentage: Decimal::percent(25),
                            start_time: current_time.seconds(),
                        },
                        DistributionType::LinearVesting {
                            percentage: Decimal::percent(75),
                            start_time: current_time.plus_days(7).seconds(),
                            end_time: current_time.plus_days(14).seconds(),
                            cliff_duration: None,
                        },
                    ],
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(14).seconds(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );

    suite.add_day();

    suite
        .claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
        .query_campaign(|result| {
            let campaign = result.unwrap();
            assert_eq!(campaign.claimed, coin(2_500u128, "uom"));
        })
        // trying to claim again without moving time, should err
        .query_rewards(alice, |result| {
            assert_eq!(
                result.unwrap(),
                RewardsResponse {
                    claimed: coins(2_500u128, "uom"),
                    pending: coins(10_000u128 - 2_500u128, "uom"),
                    available_to_claim: vec![],
                }
            );
        })
        .claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();
            match err {
                ContractError::NothingToClaim { .. } => {}
                _ => panic!("Wrong error type, should return ContractError::NothingToClaim"),
            }
        })
        .add_week()
        .claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
        // trying to claim again without moving time, should err
        .claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();
            match err {
                ContractError::NothingToClaim { .. } => {}
                _ => panic!("Wrong error type, should return ContractError::NothingToClaim"),
            }
        })
        .add_day()
        .add_day()
        .add_day()
        .add_day()
        .query_rewards(alice, |result| {
            assert_eq!(
                result.unwrap(),
                RewardsResponse {
                    claimed: coins(3_571u128, "uom"),
                    pending: coins(10_000u128 - 3_571u128, "uom"),
                    available_to_claim: coins(4_285u128, "uom"),
                }
            );
        })
        .claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
        // add 2 more weeks and claim, the campaign should have finished by then
        .query_rewards(alice, |result| {
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
        .claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
        // add a day and try claiming again, should err
        .add_day()
        .claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();
            match err {
                ContractError::NothingToClaim { .. } => {}
                _ => panic!("Wrong error type, should return ContractError::NothingToClaim"),
            }
        })
        .query_campaign(|result| {
            let campaign = result.unwrap();
            assert_eq!(campaign.claimed, coin(10_000u128, "uom"));
        })
        // dan claiming all at once
        .query_rewards(dan, |result| {
            assert_eq!(
                result.unwrap(),
                RewardsResponse {
                    claimed: vec![],
                    pending: coins(35_000u128, "uom"),
                    available_to_claim: coins(35_000u128, "uom"),
                }
            );
        })
        .claim(dan, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
        .query_rewards(dan, |result| {
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

    let allocations = &vec![(alice.to_string(), Uint128::new(10_000))];

    suite
        .instantiate_claimdrop_contract(None)
        .add_allocations(
            alice,
            allocations,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop with cliff".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LinearVesting {
                        percentage: Decimal::percent(100),
                        start_time: current_time.seconds(),
                        end_time: current_time.plus_days(1460).seconds(), // 4 years
                        cliff_duration: Some(86_400 * 365),               // 1 year cliff
                    }],
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(1460).seconds(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );

    // can't claim before the cliff period is over
    suite.claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
        let err = result.unwrap_err().downcast::<ContractError>().unwrap();
        match err {
            ContractError::NothingToClaim { .. } => {}
            _ => panic!("Wrong error type, should return ContractError::NothingToClaim"),
        }
    });

    // move a few days to pass the cliff

    // move the remaining of the year - 1 day, 365 - 1 day
    for _ in 0..364 {
        suite.add_day();
    }

    suite.claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
        let err = result.unwrap_err().downcast::<ContractError>().unwrap();
        match err {
            ContractError::NothingToClaim { .. } => {}
            _ => panic!("Wrong error type, should return ContractError::NothingToClaim"),
        }
    });

    // add another day, total days passed 365, ready to claim some
    suite.add_day();

    suite.claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
        result.unwrap();
    });

    suite
        .query_campaign(|result| {
            assert_eq!(result.unwrap().claimed, coin(10_000 / 4, "uom"));
        })
        .query_rewards(alice, |result| {
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
        .claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
        .query_campaign(|result| {
            assert_eq!(result.unwrap().claimed, coin((10_000 / 4) * 2, "uom"));
        });

    // advance two more years, so the vesting period should be over
    for _ in 0..730 {
        suite.add_day();
    }

    suite
        .query_rewards(alice, |result| {
            assert_eq!(
                result.unwrap(),
                RewardsResponse {
                    claimed: coins((10_000 / 4) * 2, "uom"),
                    pending: coins(10_000u128 - ((10_000 / 4) * 2), "uom"),
                    available_to_claim: coins((10_000 / 4) * 2, "uom"),
                }
            );
        })
        .claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
        // add a week and claim
        .query_campaign(|result| {
            assert_eq!(result.unwrap().claimed, coin(10_000u128, "uom"));
        })
        .query_rewards(alice, |result| {
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
fn claim_campaign_with_vesting_cliff_and_lump_sum() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);

    let alice = &suite.senders[0].clone();
    let current_time = &suite.get_time();

    let allocations = &vec![(alice.to_string(), Uint128::new(10_000))];

    suite
        .instantiate_claimdrop_contract(None)
        .add_allocations(
            alice,
            allocations,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop with cliff".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![
                        DistributionType::LinearVesting {
                            percentage: Decimal::percent(50),
                            start_time: current_time.seconds(),
                            end_time: current_time.plus_days(1460).seconds(), // 4 years
                            cliff_duration: Some(86_400 * 365),               // 1 year cliff
                        },
                        DistributionType::LumpSum {
                            percentage: Decimal::percent(50),
                            start_time: current_time.seconds(),
                        },
                    ],
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(1460).seconds(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );

    // The lump sum should be claimable right away, as it is not affected by the vesting cliff
    suite
        .claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
        .query_claimed(Some(alice), None, None, |result| {
            let claimed_response = result.unwrap();
            assert_eq!(claimed_response.claimed.len(), 1usize);
            assert_eq!(
                claimed_response.claimed[0],
                (alice.to_string(), coin(5_000u128, "uom"))
            );
        });

    // move a few days to pass the cliff

    // move the remaining of the year - 1 day, 365 - 1 day
    for _ in 0..364 {
        suite.add_day();
    }

    suite.claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
        let err = result.unwrap_err().downcast::<ContractError>().unwrap();
        match err {
            ContractError::NothingToClaim { .. } => {}
            _ => panic!("Wrong error type, should return ContractError::NothingToClaim"),
        }
    });

    // add another day, total days passed 365, ready to claim some
    suite.add_day();

    suite.claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
        result.unwrap();
    });

    suite
        .query_campaign(|result| {
            assert_eq!(
                result.unwrap().claimed,
                coin(5_000u128 + 5_000u128 / 4, "uom")
            );
        })
        .query_rewards(alice, |result| {
            assert_eq!(
                result.unwrap(),
                RewardsResponse {
                    claimed: coins(5_000u128 + 5_000u128 / 4, "uom"),
                    pending: coins(10_000u128 - (5_000u128 + 5_000u128 / 4), "uom"),
                    available_to_claim: vec![],
                }
            );
        });

    // advance another year
    for _ in 0..365 {
        suite.add_day();
    }

    suite
        .claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
        .query_campaign(|result| {
            assert_eq!(
                result.unwrap().claimed,
                coin(5_000u128 + (2 * 5_000u128 / 4), "uom")
            );
        });

    // advance two more years, so the vesting period should be over
    for _ in 0..730 {
        suite.add_day();
    }

    suite
        .query_rewards(alice, |result| {
            assert_eq!(
                result.unwrap(),
                RewardsResponse {
                    claimed: coins(5_000u128 + (2 * 5_000u128 / 4), "uom"),
                    pending: coins(10_000u128 - (5_000u128 + (2 * 5_000u128 / 4)), "uom"),
                    available_to_claim: coins(2 * 5_000u128 / 4, "uom"),
                }
            );
        })
        .claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
        // add a week and claim
        .query_campaign(|result| {
            assert_eq!(result.unwrap().claimed, coin(10_000u128, "uom"));
        })
        .query_rewards(alice, |result| {
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
fn claim_campaign_with_vesting_cliff_in_future_and_lump_sum() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);

    let alice = &suite.senders[0].clone();
    let current_time = &suite.get_time();

    let allocations = &vec![(alice.to_string(), Uint128::new(10_000))];

    suite
        .instantiate_claimdrop_contract(None)
        .add_allocations(
            alice,
            allocations,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop with cliff".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![
                        DistributionType::LinearVesting {
                            percentage: Decimal::percent(50),
                            start_time: current_time.plus_days(30).seconds(),
                            end_time: current_time.plus_days(60).seconds(), // 30 days duration
                            cliff_duration: Some(86_400 * 7),               // 7 days cliff
                        },
                        DistributionType::LumpSum {
                            percentage: Decimal::percent(50),
                            start_time: current_time.seconds(),
                        },
                    ],
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(90).seconds(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );

    // The lump sum should be claimable right away, as it is not affected by the vesting cliff
    suite
        .claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
        .query_claimed(Some(alice), None, None, |result| {
            let claimed_response = result.unwrap();
            assert_eq!(claimed_response.claimed.len(), 1usize);
            assert_eq!(
                claimed_response.claimed[0],
                (alice.to_string(), coin(5_000u128, "uom"))
            );
        });

    // move a few days to pass the cliff

    // move the remaining of the 30 days - 1 day
    for _ in 0..29 {
        suite.add_day();
    }

    suite.claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
        let err = result.unwrap_err().downcast::<ContractError>().unwrap();
        match err {
            ContractError::NothingToClaim { .. } => {}
            _ => panic!("Wrong error type, should return ContractError::NothingToClaim"),
        }
    });

    // add another day, total days passed 30, now the cliff starts ticking
    suite.add_day();

    suite.claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
        let err = result.unwrap_err().downcast::<ContractError>().unwrap();
        match err {
            ContractError::NothingToClaim { .. } => {}
            _ => panic!("Wrong error type, should return ContractError::NothingToClaim"),
        }
    });

    // move mid-way the cliff, there should be nothing to claim yet
    for _ in 0..6 {
        suite.add_day();
    }

    suite.query_rewards(alice, |result| {
        assert_eq!(
            result.unwrap(),
            RewardsResponse {
                claimed: coins(5_000u128, "uom"),
                pending: coins(10_000u128 - 5_000u128, "uom"),
                available_to_claim: vec![],
            }
        );
    });

    // move a day to pass the cliff
    suite.add_day();

    suite.query_rewards(alice, |result| {
        assert_eq!(
            result.unwrap(),
            RewardsResponse {
                claimed: coins(5_000u128, "uom"),
                pending: coins(10_000u128 - 5_000u128, "uom"),
                available_to_claim: coins(7 * 5_000u128 / 30u128, "uom"),
            }
        );
    });

    // advance another week
    for _ in 0..7 {
        suite.add_day();
    }

    suite
        .claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
        .query_campaign(|result| {
            assert_eq!(
                result.unwrap().claimed,
                coin(5_000u128 + (2 * 7 * 5_000u128 / 30u128), "uom")
            );
        });

    // advance two more weeks, so the linear vesting should be over
    for _ in 0..16 {
        suite.add_day();
    }

    suite
        .query_rewards(alice, |result| {
            assert_eq!(
                result.unwrap(),
                RewardsResponse {
                    claimed: coins(5_000u128 + (2 * 7 * 5_000u128 / 30u128), "uom"),
                    pending: coins(
                        10_000u128 - (5_000u128 + (2 * 7 * 5_000u128 / 30u128)),
                        "uom"
                    ),
                    available_to_claim: coins(
                        10_000u128 - (5_000u128 + (2 * 7 * 5_000u128 / 30u128)),
                        "uom"
                    ),
                }
            );
        })
        .claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
        // add a week and claim
        .query_campaign(|result| {
            assert_eq!(result.unwrap().claimed, coin(10_000u128, "uom"));
        })
        .query_rewards(alice, |result| {
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

    let allocations = &vec![
        (alice.to_string(), Uint128::new(10_000)),
        (bob.to_string(), Uint128::new(10_000)),
        (carol.to_string(), Uint128::new(20_000)),
        (dan.to_string(), Uint128::new(35_000)),
        (eva.to_string(), Uint128::new(25_000)),
    ];

    suite
        .instantiate_claimdrop_contract(None)
        .add_allocations(
            alice,
            allocations,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop with cliff".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LinearVesting {
                        percentage: Decimal::percent(100),
                        start_time: current_time.seconds(),
                        end_time: current_time.plus_days(30).seconds(), // a month
                        cliff_duration: Some(86_400 * 7),               // 7 days cliff
                    }],
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(30).seconds(),
                }),
            },
            &coins(30_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );

    suite.query_campaign(|result| {
        let campaign = result.unwrap();
        assert_eq!(campaign.total_reward, coin(100_000, "uom"));
    });

    for _ in 0..7 {
        suite.add_day();
    }

    // everybody claims
    suite
        .claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
        .claim(bob, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
        .claim(carol, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
        .claim(dan, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
        .claim(eva, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        });

    suite.query_campaign(|result| {
        let campaign = result.unwrap();
        assert_eq!(campaign.claimed, coin(23_331u128, "uom"));
    });

    let contract = suite.claimdrop_contract_addr.clone();
    // topup campaign
    suite
        .top_up_campaign(
            alice,
            &coins(70_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_balance("uom", &contract, |result| {
            assert_eq!(result, Uint128::new(100_000u128 - 23_331u128));
        });

    // go to the time when the campaign already finished
    for _ in 0..23 {
        suite.add_day();
    }

    let current_time = &suite.get_time();

    // create a new contract / campaign
    suite.instantiate_claimdrop_contract(None);

    let contract = suite.claimdrop_contract_addr.clone();

    suite
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop II".to_string(),
                    description: "This is an airdrop without cliff".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LinearVesting {
                        percentage: Decimal::percent(100),
                        start_time: current_time.seconds(),
                        end_time: current_time.plus_days(30).seconds(), // a month
                        cliff_duration: None,                           // no cliff
                    }],
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(30).seconds(),
                }),
            },
            &coins(50_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_balance("uom", &contract, |result| {
            assert_eq!(result, Uint128::new(50_000u128));
        })
        .top_up_campaign(
            alice,
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_balance("uom", &contract, |result| {
            assert_eq!(result, Uint128::new(150_000u128));
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

    let allocations = &vec![(alice.to_string(), Uint128::new(10_000))];

    suite
        .instantiate_claimdrop_contract(None)
        .add_allocations(
            alice,
            allocations,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop with cliff".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![
                        DistributionType::LumpSum {
                            percentage: Decimal::percent(25),
                            start_time: current_time.seconds(),
                        },
                        DistributionType::LinearVesting {
                            percentage: Decimal::percent(75),
                            start_time: current_time.plus_days(7).seconds(),
                            end_time: current_time.plus_days(14).seconds(),
                            cliff_duration: None,
                        },
                    ],
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(14).seconds(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );

    suite
        .claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
        .query_rewards(alice, |result| {
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
        .query_rewards(alice, |result| {
            let rewards_response = result.unwrap();
            assert_eq!(rewards_response.claimed, coins(2_500u128, "uom"));
            assert!(rewards_response.pending.is_empty());
            assert!(rewards_response.available_to_claim.is_empty());
        });
}

#[test]
fn query_rewards_single_user() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);

    let alice = &suite.senders[0].clone();
    let current_time = &suite.get_time();

    let allocations = &vec![(alice.to_string(), Uint128::new(100))];

    suite
        .instantiate_claimdrop_contract(None)
        .add_allocations(
            alice,
            allocations,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop with cliff".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::percent(100),
                        start_time: current_time.seconds(),
                    }],
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(14).seconds(),
                }),
            },
            &coins(100, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );

    suite
        .query_rewards(alice, |result| {
            let rewards_response = result.unwrap();
            println!("{:?}", rewards_response);
        })
        .claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
        .query_rewards(alice, |result| {
            let rewards_response = result.unwrap();
            println!("{:?}", rewards_response);
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

    let allocations = &vec![(alice.to_string(), Uint128::new(10_000))];

    suite
        .instantiate_claimdrop_contract(None)
        .add_allocations(
            alice,
            allocations,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop with cliff".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![
                        DistributionType::LumpSum {
                            percentage: Decimal::percent(25),
                            start_time: current_time.plus_days(1).seconds(),
                        },
                        DistributionType::LinearVesting {
                            percentage: Decimal::percent(75),
                            start_time: current_time.plus_days(8).seconds(),
                            end_time: current_time.plus_days(15).seconds(),
                            cliff_duration: None,
                        },
                    ],
                    start_time: current_time.plus_days(1).seconds(),
                    end_time: current_time.plus_days(15).seconds(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );

    suite
        .claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();

            match err {
                ContractError::CampaignError { reason } => {
                    assert_eq!(reason, "not started");
                }
                _ => panic!("Wrong error type, should return ContractError::CampaignError"),
            }
        })
        .query_rewards(alice, |result| {
            let err = result.unwrap_err().to_string();
            assert!(err.contains("not started"));
        });

    // move some epochs to make campaign active
    // the query is successful
    suite.add_week().query_rewards(alice, |result| {
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
                name: "Test Airdrop I".to_string(),
                description: "This is an airdrop with cliff".to_string(),
                reward_denom: "uom".to_string(),
                total_reward: coin(100_000, "uom"),
                distribution_type: vec![
                    DistributionType::LumpSum {
                        percentage: Decimal::percent(25),
                        start_time: current_time.seconds(),
                    },
                    DistributionType::LinearVesting {
                        percentage: Decimal::percent(75),
                        start_time: current_time.plus_days(7).seconds(),
                        end_time: current_time.plus_days(14).seconds(),
                        cliff_duration: None,
                    },
                ],
                start_time: current_time.seconds(),
                end_time: current_time.plus_days(14).seconds(),
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
                    ContractError::OwnershipError { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::OwnershipError"),
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
                    ContractError::OwnershipError { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::OwnershipError"),
                }
            },
        )
        .query_campaign(|result| {
            let campaign = result.unwrap();
            assert!(campaign.closed.is_none());
        })
        .manage_campaign(
            alice, // alice can end the campaign since it's the owner
            CampaignAction::CloseCampaign {},
            &[],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .manage_campaign(
            alice, // alice tries closing the campaign again
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
        });

    // let's create a new campaign
    suite.instantiate_claimdrop_contract(None).manage_campaign(
        alice,
        CampaignAction::CreateCampaign {
            params: Box::new(CampaignParams {
                name: "Test Airdrop I".to_string(),
                description: "This is an airdrop with cliff".to_string(),
                reward_denom: "uom".to_string(),
                total_reward: coin(100_000, "uom"),
                distribution_type: vec![
                    DistributionType::LumpSum {
                        percentage: Decimal::percent(25),
                        start_time: current_time.seconds(),
                    },
                    DistributionType::LinearVesting {
                        percentage: Decimal::percent(75),
                        start_time: current_time.plus_days(7).seconds(),
                        end_time: current_time.plus_days(14).seconds(),
                        cliff_duration: None,
                    },
                ],
                start_time: current_time.seconds(),
                end_time: current_time.plus_days(14).seconds(),
            }),
        },
        &coins(100_000, "uom"),
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    suite.manage_campaign(
        carol, // carol can't since it's not the owner
        CampaignAction::CloseCampaign {},
        &[],
        |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();
            match err {
                ContractError::OwnershipError { .. } => {}
                _ => panic!("Wrong error type, should return ContractError::OwnershipError"),
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
            alice, // alice can't since it renounce the ownership
            CampaignAction::CloseCampaign {},
            &[],
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::OwnershipError { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::OwnershipError"),
                }
            },
        )
        .manage_campaign(
            carol, // alice can't since it renounce the ownership
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

    let allocations = &vec![(alice.to_string(), Uint128::new(10_000))];

    suite
        .instantiate_claimdrop_contract(Some(dan.to_string()))
        .add_allocations(
            dan,
            allocations,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .manage_campaign(
            dan,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![
                        DistributionType::LumpSum {
                            percentage: Decimal::percent(25),
                            start_time: current_time.seconds(),
                        },
                        DistributionType::LinearVesting {
                            percentage: Decimal::percent(75),
                            start_time: current_time.plus_days(7).seconds(),
                            end_time: current_time.plus_days(14).seconds(),
                            cliff_duration: None,
                        },
                    ],
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(14).seconds(),
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
            Some(alice.to_string()),
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

    suite.query_rewards(alice, |result| {
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
        .query_rewards(alice, |result| {
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

    let allocations = &vec![
        (alice.to_string(), Uint128::new(10_000)),
        (bob.to_string(), Uint128::new(10_000)),
    ];

    suite
        .instantiate_claimdrop_contract(None)
        .add_allocations(
            alice,
            allocations,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![
                        DistributionType::LumpSum {
                            percentage: Decimal::percent(25),
                            start_time: current_time.seconds(),
                        },
                        DistributionType::LinearVesting {
                            percentage: Decimal::percent(75),
                            start_time: current_time.plus_days(7).seconds(),
                            end_time: current_time.plus_days(14).seconds(),
                            cliff_duration: None,
                        },
                    ],
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(14).seconds(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );

    suite.add_week();

    // can claim
    suite.claim(bob, None, |result: Result<AppResponse, anyhow::Error>| {
        result.unwrap();
    });

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
        .claim(alice, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
        //end campaign
        .manage_campaign(
            alice,
            CampaignAction::CloseCampaign {},
            &[],
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
        )
        // intended
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop II".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![
                        DistributionType::LumpSum {
                            percentage: Decimal::percent(25),
                            start_time: current_time.seconds(),
                        },
                        DistributionType::LinearVesting {
                            percentage: Decimal::percent(75),
                            start_time: current_time.plus_days(7).seconds(),
                            end_time: current_time.plus_days(14).seconds(),
                            cliff_duration: None,
                        },
                    ],
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(14).seconds(),
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

    let allocations = &vec![(alice.to_string(), Uint128::new(17))];

    suite
        .instantiate_claimdrop_contract(Some(alice.to_string()))
        .add_allocations(
            alice,
            allocations,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(23, "uom"),
                    distribution_type: vec![DistributionType::LinearVesting {
                        percentage: Decimal::percent(100),
                        start_time: current_time.seconds(),
                        end_time: current_time.plus_days(60).seconds(),
                        cliff_duration: None,
                    }],
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(60).seconds(),
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
            Some(alice.to_string()),
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
            Some(alice.to_string()),
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
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LinearVesting {
                        percentage: Decimal::percent(100),
                        start_time: current_time.seconds(),
                        end_time: current_time.plus_days(30).seconds(),
                        cliff_duration: None,
                    }],
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(7).seconds(),
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
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![DistributionType::LinearVesting {
                        percentage: Decimal::percent(100),
                        start_time: current_time.seconds(),
                        end_time: current_time.plus_days(7).seconds(),
                        cliff_duration: None,
                    }],
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(7).seconds(),
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
                name: "Test Airdrop I".to_string(),
                description: "This is an airdrop, 土金, ك".to_string(),
                reward_denom: "uom".to_string(),
                total_reward: coin(100_000, "uom"),
                distribution_type: vec![DistributionType::LinearVesting {
                    percentage: Decimal::percent(100),
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(6).seconds(),
                    cliff_duration: None,
                }],
                start_time: current_time.seconds(),
                end_time: current_time.plus_days(7).seconds(),
            }),
        },
        &coins(100_000, "uom"),
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );
}

#[test]
fn test_add_allocations() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);
    let alice = &suite.senders[0].clone();
    let bob = &suite.senders[1].clone();
    let carol = &suite.senders[2].clone();
    let current_time = &suite.get_time();

    suite.instantiate_claimdrop_contract(None).manage_campaign(
        alice,
        CampaignAction::CreateCampaign {
            params: Box::new(CampaignParams {
                name: "Test Airdrop I".to_string(),
                description: "This is an airdrop with cliff".to_string(),
                reward_denom: "uom".to_string(),
                total_reward: coin(100_000, "uom"),
                distribution_type: vec![
                    DistributionType::LumpSum {
                        percentage: Decimal::percent(25),
                        start_time: current_time.plus_days(1).seconds(),
                    },
                    DistributionType::LinearVesting {
                        percentage: Decimal::percent(75),
                        start_time: current_time.plus_days(8).seconds(),
                        end_time: current_time.plus_days(15).seconds(),
                        cliff_duration: None,
                    },
                ],
                start_time: current_time.plus_days(1).seconds(),
                end_time: current_time.plus_days(15).seconds(),
            }),
        },
        &coins(100_000, "uom"),
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    // add allocations
    let allocations = &vec![
        (alice.to_string(), Uint128::new(100_000)),
        (bob.to_string(), Uint128::new(200_000)),
        (carol.to_string(), Uint128::new(300_000)),
    ];

    suite
        .query_allocation(alice, |result| {
            let allocation = result.unwrap();
            assert_eq!(allocation.allocation, None);
        })
        .query_allocation(bob, |result| {
            let allocation = result.unwrap();
            assert_eq!(allocation.allocation, None);
        })
        .query_allocation(carol, |result| {
            let allocation = result.unwrap();
            assert_eq!(allocation.allocation, None);
        })
        .add_allocations(
            bob,
            allocations,
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::OwnershipError(e) => {}
                    _ => panic!("Wrong error type, should return ContractError::OwnershipError"),
                }
            },
        )
        .add_allocations(
            alice,
            allocations,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_allocation(alice, |result| {
            let allocation = result.unwrap();
            assert_eq!(allocation.allocation, Some(Uint128::new(100_000)));
        })
        .query_allocation(bob, |result| {
            let allocation = result.unwrap();
            assert_eq!(allocation.allocation, Some(Uint128::new(200_000)));
        })
        .query_allocation(carol, |result| {
            let allocation = result.unwrap();
            assert_eq!(allocation.allocation, Some(Uint128::new(300_000)));
        });
}

#[test]
fn test_cannot_add_allocations_after_campaign_start() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);
    let alice = &suite.senders[0].clone();
    let bob = &suite.senders[1].clone();
    let carol = &suite.senders[2].clone();
    let current_time = &suite.get_time();

    suite.instantiate_claimdrop_contract(None).manage_campaign(
        alice,
        CampaignAction::CreateCampaign {
            params: Box::new(CampaignParams {
                name: "Test Airdrop I".to_string(),
                description: "This is an airdrop with cliff".to_string(),
                reward_denom: "uom".to_string(),
                total_reward: coin(100_000, "uom"),
                distribution_type: vec![DistributionType::LumpSum {
                    percentage: Decimal::percent(100),
                    start_time: current_time.plus_days(1).seconds(),
                }],
                start_time: current_time.plus_days(1).seconds(),
                end_time: current_time.plus_days(14).seconds(),
            }),
        },
        &coins(100_000, "uom"),
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    // Upload allocations
    let allocations = &vec![
        (alice.to_string(), Uint128::new(100_000)),
        (bob.to_string(), Uint128::new(200_000)),
        (carol.to_string(), Uint128::new(300_000)),
    ];

    suite.add_day().add_day();

    suite.add_allocations(
        alice,
        allocations,
        |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();
            match err {
                ContractError::CampaignError { reason } => {
                    assert_eq!(
                        reason,
                        "cannot upload allocations after campaign has started"
                    );
                }
                _ => panic!("Wrong error type, should return ContractError::CampaignError"),
            }
        },
    );
}

#[test]
fn test_replace_address() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);
    let alice = &suite.senders[0].clone();
    let bob = &suite.senders[1].clone();
    let carol = &suite.senders[2].clone();
    let current_time = &suite.get_time();

    // Upload allocations
    let allocations = &vec![(bob.to_string(), Uint128::new(100_000))];

    suite
        .instantiate_claimdrop_contract(None)
        .add_allocations(
            alice,
            allocations,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop with cliff".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![
                        DistributionType::LinearVesting {
                            percentage: Decimal::percent(50),
                            start_time: current_time.plus_days(7).seconds(),
                            end_time: current_time.plus_days(14).seconds(),
                            cliff_duration: None,
                        },
                        DistributionType::LumpSum {
                            percentage: Decimal::percent(50),
                            start_time: current_time.seconds(),
                        },
                    ],
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(14).seconds(),
                }),
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );

    suite.add_day();

    suite
        .claim(bob, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        })
        .query_claimed(Some(bob), None, None, |result| {
            let claimed_response = result.unwrap();
            assert_eq!(claimed_response.claimed.len(), 1usize);
            assert_eq!(
                claimed_response.claimed[0],
                (bob.to_string(), coin(50_000u128, "uom"))
            );
        })
        .query_allocation(bob, |result| {
            let allocation = result.unwrap();
            assert_eq!(allocation.allocation, Some(Uint128::new(100_000)));
        });

    // replace address

    suite
        .replace_address(
            carol,
            bob,
            carol,
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
        )
        .replace_address(
            alice,
            bob,
            carol,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_claimed(Some(bob), None, None, |result| {
            let claimed_response = result.unwrap();
            assert!(claimed_response.claimed.is_empty());
        })
        .query_allocation(bob, |result| {
            let allocation = result.unwrap();
            assert_eq!(allocation.allocation, None);
        })
        .query_claimed(Some(carol), None, None, |result| {
            let claimed_response = result.unwrap();
            assert_eq!(claimed_response.claimed.len(), 1usize);
            assert_eq!(
                claimed_response.claimed[0],
                (carol.to_string(), coin(50_000u128, "uom"))
            );
        })
        .query_allocation(carol, |result| {
            let allocation = result.unwrap();
            assert_eq!(allocation.allocation, Some(Uint128::new(100_000)));
        });
}

#[test]
fn test_blacklist_address() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);
    let alice = &suite.senders[0].clone();
    let bob = &suite.senders[1].clone();
    let carol = &suite.senders[2].clone();
    let current_time = &suite.get_time();

    suite.instantiate_claimdrop_contract(None).manage_campaign(
        alice,
        CampaignAction::CreateCampaign {
            params: Box::new(CampaignParams {
                name: "Test Airdrop I".to_string(),
                description: "This is an airdrop with cliff".to_string(),
                reward_denom: "uom".to_string(),
                total_reward: coin(600_000, "uom"),
                distribution_type: vec![DistributionType::LumpSum {
                    percentage: Decimal::percent(100),
                    start_time: current_time.plus_days(1).seconds(),
                }],
                start_time: current_time.plus_days(1).seconds(),
                end_time: current_time.plus_days(14).seconds(),
            }),
        },
        &coins(300_000, "uom"),
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    // Upload allocations
    let allocations = &vec![
        (alice.to_string(), Uint128::new(100_000)),
        (bob.to_string(), Uint128::new(200_000)),
        (carol.to_string(), Uint128::new(300_000)),
    ];

    suite.add_allocations(
        alice,
        allocations,
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    suite
        .blacklist_address(
            bob,
            carol,
            true,
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
        )
        .query_is_blacklisted(carol, |result| {
            let blacklist_status = result.unwrap();
            assert_eq!(blacklist_status.is_blacklisted, false);
        })
        .blacklist_address(
            alice,
            carol,
            true,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_is_blacklisted(carol, |result| {
            let blacklist_status = result.unwrap();
            assert_eq!(blacklist_status.is_blacklisted, true);
        });

    suite.add_week();
    //claiming should fail

    suite.claim(carol, None, |result: Result<AppResponse, anyhow::Error>| {
        let err = result.unwrap_err().downcast::<ContractError>().unwrap();
        match err {
            ContractError::AddressBlacklisted => {}
            _ => panic!("Wrong error type, should return ContractError::AddressBlacklisted"),
        }
    });

    // remove from blacklist, claiming should work
    suite
        .blacklist_address(
            alice,
            carol,
            false,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .claim(carol, None, |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        });
}

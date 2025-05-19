use std::str::FromStr;

use cosmwasm_std::{coin, coins, Decimal, Timestamp, Uint128};
use cw_multi_test::AppResponse;
use cw_ownable::OwnershipError;

use crate::suite::TestingSuite;
use claimdrop_contract::error::ContractError;
use claimdrop_contract::msg::{CampaignAction, CampaignParams, DistributionType, RewardsResponse};
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
        .claim(
            alice,
            None,
            None,
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
    suite.claim(
        alice,
        None,
        None,
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
            None,
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
            None,
            None,
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
        });

    suite
        .add_week()
        .add_week()
        .add_week()
        .query_balance("uom", carol, |balance| {
            assert_eq!(balance, Uint128::new(1_000_000_000));
        })
        .claim(
            carol,
            None,
            None,
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
            None,
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
            None,
            None,
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
    suite.claim(
        carol,
        None,
        None,
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
            None,
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
            None,
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
            None,
            None,
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
            Some(alice.to_string()),
            None,
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
            None,
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .claim(
            carol,
            None,
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .claim(
            dan,
            None,
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .claim(
            eva,
            None,
            None,
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
            // Note: Order might vary based on internal HashMap iteration order before sorting in query if not explicitly sorted by address string
            // For robust test, sort here or ensure query always sorts. Assuming query_claimed sorts by address for pagination.
            // Let's assume the test had this order before, if it fails, it's due to HashMap iter order.
            let mut actual_claims = claimed_response.claimed;
            actual_claims.sort_by(|a, b| a.0.cmp(&b.0)); // Sort by address for consistent comparison

            let mut expected_claims = vec![
                (alice.to_string(), coin(10_000u128, "uom")),
                (bob.to_string(), coin(10_000u128, "uom")),
                (carol.to_string(), coin(20_000u128, "uom")),
                (dan.to_string(), coin(35_000u128, "uom")),
                (eva.to_string(), coin(25_000u128, "uom")),
            ];
            expected_claims.sort_by(|a, b| a.0.cmp(&b.0));
            assert_eq!(actual_claims, expected_claims);
        })
        .query_claimed(None, Some(carol), None, |result| {
            // start_from carol
            let claimed_response = result.unwrap();
            let mut actual_claims = claimed_response.claimed;
            actual_claims.sort_by(|a, b| a.0.cmp(&b.0));

            // Expected: addresses alphabetically after carol
            let mut expected_after_carol = vec![
                (bob.to_string(), coin(10_000u128, "uom")),
                (dan.to_string(), coin(35_000u128, "uom")),
                (eva.to_string(), coin(25_000u128, "uom")),
            ];
            expected_after_carol.sort_by(|a, b| a.0.cmp(&b.0));

            assert_eq!(actual_claims.len(), 3usize);
            assert_eq!(actual_claims, expected_after_carol);
        })
        .query_claimed(None, None, Some(2), |result| {
            let claimed_response = result.unwrap();
            // Expects first 2 alphabetically: alice, bob
            assert_eq!(claimed_response.claimed.len(), 2usize);
            assert_eq!(
                claimed_response.claimed,
                vec![
                    (alice.to_string(), coin(10_000u128, "uom")),
                    (carol.to_string(), coin(20_000u128, "uom")),
                ]
            );
        })
        .query_claimed(None, Some(alice), Some(2), |result| {
            // After alice, limit 2: bob, carol
            let claimed_response = result.unwrap();
            assert_eq!(claimed_response.claimed.len(), 2usize);
            assert_eq!(
                claimed_response.claimed,
                vec![
                    (carol.to_string(), coin(20_000u128, "uom")),
                    (eva.to_string(), coin(25_000u128, "uom")),
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
        .claim(
            alice,
            None,
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
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
        .claim(
            alice,
            None,
            None,
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
            None,
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        // trying to claim again without moving time, should err
        .claim(
            alice,
            None,
            None,
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
        .query_rewards(alice, |result| {
            assert_eq!(
                result.unwrap(),
                RewardsResponse {
                    claimed: coins(3_571u128, "uom"), // 2500 + (7500 * 4/7 * 1/4) approx. This needs re-check based on actual vesting.
                    // Original test had 3_571. This is lump sum (2500) + 4 days of vesting of (7500 over 7 days)
                    // 2500 + (7500 * 4/7) = 2500 + 4285.71 = 6785.
                    // Let's re-evaluate the original test's numbers.
                    // After 1 day: claim 2500 (lump sum). Available = 0.
                    // Add 7 days (total 8 days from start). Vesting started at day 7. So 1 day of vesting.
                    // Vesting is 7500 over 7 days = 1071.42 per day.
                    // So, at day 8, Alice claims 1071. Total claimed = 2500 + 1071 = 3571. OK.
                    // Available: 0 (claimed all current vesting)
                    // Add 4 more days (total 12 days from start). 4 more days of vesting.
                    // Available: 1071 * 4 = 4284
                    // Claimed: 3571. Pending: 10000 - 3571 = 6429
                    pending: coins(10_000u128 - 3_571u128, "uom"),
                    available_to_claim: coins(4_286u128, "uom"), // 4 days * (7500/7)
                }
            );
        })
        .claim(
            alice,
            None,
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap(); // Claims 4285. Total claimed = 3571 + 4285 = 7856
            },
        )
        // add 2 more weeks and claim, the campaign should have finished by then
        .query_rewards(alice, |result| {
            assert_eq!(
                result.unwrap(),
                RewardsResponse {
                    claimed: coins(7_857u128, "uom"),
                    pending: coins(10_000u128 - 7_857u128, "uom"), // 2143
                    available_to_claim: vec![], // All currently vested is claimed.
                }
            );
        })
        .add_week() // total 12 days + 7 days = 19 days. Campaign ends day 14. Vesting ends day 14.
        .add_week() // total 19 days + 7 days = 26 days.
        .claim(
            alice,
            None,
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                // Claims remaining 2144
                result.unwrap();
            },
        )
        // add a day and try claiming again, should err
        .add_day()
        .claim(
            alice,
            None,
            None,
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
        .query_rewards(dan, |result| {
            assert_eq!(
                result.unwrap(),
                RewardsResponse {
                    claimed: vec![],
                    pending: coins(35_000u128, "uom"),
                    available_to_claim: coins(35_000u128, "uom"), // All available as campaign ended
                }
            );
        })
        .claim(
            dan,
            None,
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
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
            assert_eq!(campaign.claimed, coin(45_000u128, "uom")); // 10000 (alice) + 35000 (dan)
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
    suite.claim(
        alice,
        None,
        None,
        |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();
            match err {
                ContractError::NothingToClaim { .. } => {}
                _ => panic!("Wrong error type, should return ContractError::NothingToClaim"),
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
        None,
        None,
        |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();
            match err {
                ContractError::NothingToClaim { .. } => {}
                _ => panic!("Wrong error type, should return ContractError::NothingToClaim"),
            }
        },
    );

    // add another day, total days passed 365, ready to claim some
    suite.add_day();

    suite.claim(
        alice,
        None,
        None,
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

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
        .claim(
            alice,
            None,
            None,
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
        .claim(
            alice,
            None,
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
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
        .claim(
            alice,
            None,
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
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

    suite.claim(
        alice,
        None,
        None,
        |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();
            match err {
                ContractError::NothingToClaim { .. } => {}
                _ => panic!("Wrong error type, should return ContractError::NothingToClaim"),
            }
        },
    );

    // add another day, total days passed 365, ready to claim some
    suite.add_day();

    suite.claim(
        alice,
        None,
        None,
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

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
        .claim(
            alice,
            None,
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
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
        .claim(
            alice,
            None,
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
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
        .claim(
            alice,
            None,
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
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

    suite.claim(
        alice,
        None,
        None,
        |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();
            match err {
                ContractError::NothingToClaim { .. } => {}
                _ => panic!("Wrong error type, should return ContractError::NothingToClaim"),
            }
        },
    );

    // add another day, total days passed 30, now the cliff starts ticking
    suite.add_day();

    suite.claim(
        alice,
        None,
        None,
        |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();
            match err {
                ContractError::NothingToClaim { .. } => {}
                _ => panic!("Wrong error type, should return ContractError::NothingToClaim"),
            }
        },
    );

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
                available_to_claim: coins(7 * 5_000u128 / 30u128, "uom"), // 7 days (cliff) out of 30 day vesting period of 5000 tokens = 1166
            }
        );
    });

    // advance another week
    for _ in 0..7 {
        suite.add_day();
    }

    suite
        .claim(
            alice,
            None,
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                // Claims available 1166 + 7 more days = 1166 + 1166 = 2332. Total claimed = 5000 + 2332 = 7332
                result.unwrap();
            },
        )
        .query_campaign(|result| {
            assert_eq!(
                result.unwrap().claimed, // Lump sum + ( (cliff_duration + 7 days) / total_vesting_duration_for_slot ) * slot_total_reward
                // 5000 + ( (7+7) / 30 ) * 5000 = 5000 + (14/30)*5000 = 5000 + 2333 = 7333. Close enough to 7332 due to previous rounding.
                coin(5_000u128 + (2 * 7 * 5_000u128 / 30u128), "uom") // Original: 5000 + 2332 = 7332
            );
        });

    // advance two more weeks, so the linear vesting should be over (14 more days. Total days passed cliff = 7+7+14 = 28. Vesting duration = 30 days)
    // Remaining 2 days of vesting will be claimable.
    for _ in 0..16 {
        // cliff (7) + 7 + 16 = 30 days from vesting start. Vesting ends.
        suite.add_day();
    }

    suite
        .query_rewards(alice, |result| {
            let previous_total_claimed = 5_000u128 + (2 * 7 * 5_000u128 / 30u128); // 7332
            let total_vesting_slot_amount = 5_000u128;
            let already_claimed_from_vesting = 2 * 7 * 5_000u128 / 30u128; // 2332
            let remaining_to_claim_from_vesting =
                total_vesting_slot_amount.saturating_sub(already_claimed_from_vesting); // 5000 - 2332 = 2668

            assert_eq!(
                result.unwrap(),
                RewardsResponse {
                    claimed: coins(previous_total_claimed, "uom"),
                    pending: coins(10_000u128 - previous_total_claimed, "uom"),
                    available_to_claim: coins(remaining_to_claim_from_vesting, "uom"),
                }
            );
        })
        .claim(
            alice,
            None,
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
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
            &coins(30_000, "uom"), // Initial funding less than total_reward
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );

    suite.query_campaign(|result| {
        let campaign = result.unwrap();
        assert_eq!(campaign.total_reward, coin(100_000, "uom"));
    });

    for _ in 0..7 {
        // Pass cliff
        suite.add_day();
    }

    // everybody claims
    // Total allocation = 10k+10k+20k+35k+25k = 100k
    // At 7 days (cliff end) into 30-day vesting: 7/30 of total is claimable.
    // 100_000 * 7 / 30 = 23333.33. Rounded down.
    // Alice: 10000 * 7/30 = 2333
    // Bob: 10000 * 7/30 = 2333
    // Carol: 20000 * 7/30 = 4666
    // Dan: 35000 * 7/30 = 8166
    // Eva: 25000 * 7/30 = 5833
    // Total claimed = 2333+2333+4666+8166+5833 = 23331 (Matches original test)
    suite
        .claim(
            alice,
            None,
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .claim(
            bob,
            None,
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .claim(
            carol,
            None,
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .claim(
            dan,
            None,
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .claim(
            eva,
            None,
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );

    suite.query_campaign(|result| {
        let campaign = result.unwrap();
        assert_eq!(campaign.claimed, coin(23_331u128, "uom"));
    });

    let contract = suite.claimdrop_contract_addr.clone();
    // topup campaign
    suite
        .top_up_campaign(
            alice,
            &coins(70_000, "uom"), // Top up to meet total_reward
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_balance("uom", &contract, |result| {
            // Initial 30k + 70k topup - 23331 claimed = 76669
            assert_eq!(result, Uint128::new(30_000u128 + 70_000u128 - 23_331u128));
        });

    // go to the time when the campaign already finished
    for _ in 0..23 {
        // 7 days passed + 23 days = 30 days. Campaign ends.
        suite.add_day();
    }

    let current_time_after_first_campaign = &suite.get_time(); // Save time for next campaign

    // create a new contract / campaign (by re-instantiating)
    suite.instantiate_claimdrop_contract(None); // New contract instance for Test Airdrop II

    let contract_2 = suite.claimdrop_contract_addr.clone(); // Get new contract address

    suite
        .manage_campaign(
            alice, // Alice is owner of this new contract instance by default if None passed to instantiate
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop II".to_string(),
                    description: "This is an airdrop without cliff".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"), // Total intended for this campaign
                    distribution_type: vec![DistributionType::LinearVesting {
                        percentage: Decimal::percent(100),
                        start_time: current_time_after_first_campaign.seconds(),
                        end_time: current_time_after_first_campaign.plus_days(30).seconds(), // a month
                        cliff_duration: None, // no cliff
                    }],
                    start_time: current_time_after_first_campaign.seconds(),
                    end_time: current_time_after_first_campaign.plus_days(30).seconds(),
                }),
            },
            &coins(50_000, "uom"), // Initial funding
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_balance("uom", &contract_2, |result| {
            assert_eq!(result, Uint128::new(50_000u128));
        })
        .top_up_campaign(
            // Top up more funds
            alice,
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_balance("uom", &contract_2, |result| {
            // Total funds in contract_2 = 50k + 100k = 150k
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
        .claim(
            alice,
            None,
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap(); // Claims 2500 (LumpSum)
            },
        )
        .query_rewards(alice, |result| {
            assert_eq!(
                result.unwrap(),
                RewardsResponse {
                    claimed: coins(2_500u128, "uom"),
                    pending: coins(10_000u128 - 2_500u128, "uom"), // 7500 pending from vesting
                    available_to_claim: vec![],                    // Vesting not started yet
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
            assert!(rewards_response.pending.is_empty()); // Campaign closed, no more pending
            assert!(rewards_response.available_to_claim.is_empty()); // Campaign closed
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
                    total_reward: coin(100_000, "uom"), // Contract has 100k, user allocated 100
                    distribution_type: vec![DistributionType::LumpSum {
                        percentage: Decimal::percent(100),
                        start_time: current_time.seconds(),
                    }],
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(14).seconds(),
                }),
            },
            &coins(100, "uom"), // Fund with exact user allocation for simplicity here
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );

    suite
        .query_rewards(alice, |result| {
            let rewards_response = result.unwrap();
            // Expected before claim:
            assert_eq!(rewards_response.claimed, vec![]);
            assert_eq!(rewards_response.pending, coins(100, "uom"));
            assert_eq!(rewards_response.available_to_claim, coins(100, "uom"));
            println!("{:?}", rewards_response);
        })
        .claim(
            alice,
            None,
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_rewards(alice, |result| {
            let rewards_response = result.unwrap();
            // Expected after claim:
            assert_eq!(rewards_response.claimed, coins(100, "uom"));
            assert_eq!(rewards_response.pending, vec![]);
            assert_eq!(rewards_response.available_to_claim, vec![]);
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
                    start_time: current_time.plus_days(1).seconds(), // Campaign starts in 1 day
                    end_time: current_time.plus_days(15).seconds(),
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
            None,
            None,
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
        .query_rewards(alice, |result| {
            let err = result.unwrap_err().to_string();
            assert!(err.contains("not started"));
        });

    // move some epochs to make campaign active
    suite.add_day(); // Advance 1 day, campaign starts
    suite.query_rewards(alice, |result| {
        // Now campaign has started
        let rewards = result.unwrap();
        // Lump sum (25% of 10k = 2500) should be available as its start time matches campaign start time
        assert_eq!(rewards.claimed, vec![]);
        assert_eq!(rewards.pending, coins(10_000, "uom"));
        assert_eq!(rewards.available_to_claim, coins(2500, "uom"));
    });
}

#[test]
fn close_campaigns() {
    let mut suite = TestingSuite::default_with_balances(vec![coin(1_000_000_000, "uom")]);

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
        // Alice is owner of new contract
        alice,
        CampaignAction::CreateCampaign {
            params: Box::new(CampaignParams {
                name: "Test Airdrop I".to_string(), // Name can be same as closed one
                description: "This is an airdrop with cliff".to_string(),
                reward_denom: "uom".to_string(),
                total_reward: coin(100_000, "uom"),
                distribution_type: vec![
                    DistributionType::LumpSum {
                        percentage: Decimal::percent(25),
                        start_time: current_time.seconds(), // Use a fresh current_time if logic depends on it relative to now
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
        carol, // carol can't since it's not the owner of this new contract instance
        CampaignAction::CloseCampaign {},
        &[],
        |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();
            match err {
                ContractError::OwnershipError { .. } => {} // Should be NotOwner
                _ => panic!("Wrong error type, should return ContractError::OwnershipError"),
            }
        },
    );

    // let's update the ownership of the contract
    suite
        .update_ownership(
            alice, // Current owner of the new contract
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

    // now only carol can end the new campaign.
    suite
        .manage_campaign(
            alice, // alice can't since it renounced the ownership for this instance
            CampaignAction::CloseCampaign {},
            &[],
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::OwnershipError { .. } => {} // Should be NotOwner
                    _ => panic!("Wrong error type, should return ContractError::OwnershipError"),
                }
            },
        )
        .manage_campaign(
            carol, // Carol is the new owner
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
    let dan = &suite.senders[3].clone(); // Dan is owner
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
                    total_reward: coin(100_000, "uom"), // Contract funded with 100k
                    distribution_type: vec![
                        DistributionType::LumpSum {
                            percentage: Decimal::percent(25), // Alice gets 2500 from this
                            start_time: current_time.seconds(),
                        },
                        DistributionType::LinearVesting {
                            percentage: Decimal::percent(75), // Alice gets 7500 from this
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

    // suite.add_week(); // Time passes, vesting might start if campaign start_time allows

    // claim
    // alice gets 2500 in lump sum if campaign started.
    // Here, campaign starts immediately (current_time.seconds())
    // Lump sum starts immediately.
    suite
        .claim(
            bob, //bob claims for alice
            Some(alice.to_string()),
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap(); // Alice claims 2500
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

    suite.add_week(); // Advance 7 days. Vesting starts. All of lump sum available.
                      // Vesting also starts if its start_time is <= current_time + 7 days.
                      // Vesting start_time is current_time + 7 days. So vesting just begins.
                      // Nothing from vesting available yet on day 7 itself, needs 1 sec more for 1st portion.
                      // Let's advance one more day to be sure some vesting happened.
    suite.add_day(); // Total 8 days passed. 1 day of vesting (7500/7 = 1071)

    suite.query_rewards(alice, |result| {
        // Claimed: 2500 (lump sum)
        // Available from vesting: 1 day = 1071 (approx)
        // Pending: 10000 - 2500 = 7500
        // Available to claim now = 1071 from vesting
        assert_eq!(
            result.unwrap(),
            RewardsResponse {
                claimed: coins(2_500u128, "uom"),
                pending: coins(7500, "uom"),
                available_to_claim: coins(1071, "uom"), // 7500 / 7 days for 1 day
            }
        );
    });

    // closing campaign by Dan (owner)
    suite
        .query_campaign(|result| {
            let campaign = result.unwrap();
            assert_eq!(campaign.claimed, coin(2_500u128, "uom")); // Only Alice's initial claim
        })
        .query_balance("uom", dan, |result| {
            // Dan's balance before refund
            assert_eq!(result, Uint128::new(1_000_000_000 - 100_000)); // Initial funding
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
            // Dan gets refund of (100_000 initial_fund - 2_500 claimed_by_alice) = 97_500
            assert_eq!(result, Uint128::new(1_000_000_000 - 100_000 + 97_500));
        })
        .query_rewards(alice, |result| {
            // After campaign close
            let rewards_response = result.unwrap();
            assert_eq!(rewards_response.claimed, coins(2_500u128, "uom")); // What was claimed remains
            assert!(rewards_response.pending.is_empty()); // No more pending
            assert!(rewards_response.available_to_claim.is_empty()); // Nothing available
        })
        .query_claimed(None, None, None, |result| {
            // Query all claims
            let claimed_response = result.unwrap();
            assert_eq!(claimed_response.claimed.len(), 1usize); // Only Alice claimed
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

    let alice = &suite.senders[0].clone(); // Default owner
    let bob = &suite.senders[1].clone();
    let current_time = &suite.get_time();

    let allocations = &vec![
        (alice.to_string(), Uint128::new(10_000)),
        (bob.to_string(), Uint128::new(10_000)),
    ];

    suite
        .instantiate_claimdrop_contract(None) // Alice is owner
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

    suite.add_week(); // Advance time

    // can claim
    suite.claim(
        bob,
        None,
        None,
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    suite
        .update_ownership(
            bob, // Bob is not owner
            cw_ownable::Action::RenounceOwnership {},
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
        .update_ownership(
            alice, // Alice is owner
            cw_ownable::Action::RenounceOwnership {},
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap(); // Alice renounces
            },
        )
        // can claim even if owner renounced
        .claim(
            alice,
            None,
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        //end campaign fails as no owner
        .manage_campaign(
            alice, // Alice tries, but is no longer owner
            CampaignAction::CloseCampaign {},
            &[],
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::OwnershipError(e) => match e {
                        OwnershipError::NoOwner => {} // Correct: No owner to perform this action
                        _ => panic!(
                            "Wrong error type, should return OwnershipError::NoOwner but got {:?}",
                            e
                        ),
                    },
                    _ => panic!("Wrong error type, should return ContractError::OwnershipError"),
                }
            },
        )
        // Creating new campaign also fails
        .manage_campaign(
            alice, // Alice tries, but no owner
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop II".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_denom: "uom".to_string(),
                    total_reward: coin(100_000, "uom"),
                    distribution_type: vec![ /* ... */ ],
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
                        _ => panic!(
                            "Wrong error type, should return OwnershipError::NoOwner but got {:?}",
                            e
                        ),
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
                    end_time: current_time.plus_days(60).seconds(), // Campaign ends when vesting ends
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
        // Go to day 59 (0-indexed, so 59th day after start)
        suite.add_day();
    }
    // At day 59, 59/60 of 17 should be vested. 17 * 59 / 60 = 16.71 => 16
    suite
        .claim(
            alice,
            Some(alice.to_string()),
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_claimed(Some(alice), None, None, |result| {
            let claimed_response = result.unwrap();
            assert_eq!(claimed_response.claimed.len(), 1usize);
            assert_eq!(
                claimed_response.claimed[0],
                (alice.to_string(), coin(16u128, "uom")) // Rounded down
            );
        });

    suite.add_day(); // Day 60. Vesting ends. Campaign ends.
                     // All 17 should be claimable. 16 already claimed. 1 remaining (dust).

    // executing the claiming here, will result on the compute_claimable_amount::new_claims being empty,
    // as the claim_amount will be zero, while the rounding_error_compensation_amount will be 1.
    // This relies on get_compensation_for_rounding_errors.
    suite
        .claim(
            alice,
            Some(alice.to_string()),
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_claimed(Some(alice), None, None, |result| {
            let claimed_response = result.unwrap();
            assert_eq!(claimed_response.claimed.len(), 1usize);
            assert_eq!(
                claimed_response.claimed[0],
                (alice.to_string(), coin(17u128, "uom")) // Total 17 claimed
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
                        end_time: current_time.plus_days(30).seconds(), // Dist ends after campaign
                        cliff_duration: None,
                    }],
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(7).seconds(), // Campaign ends before dist
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
            // Valid case: dist ends with campaign
            alice,
            CampaignAction::CreateCampaign {
                params: Box::new(CampaignParams {
                    name: "Test Airdrop II".to_string(), // Changed name
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

    // Must instantiate a new contract or close the previous one before creating another "Test Airdrop I"
    // Assuming we want a new contract for this next test case:
    suite.instantiate_claimdrop_contract(None);
    suite.manage_campaign(
        alice,
        CampaignAction::CreateCampaign {
            params: Box::new(CampaignParams {
                name: "Test Airdrop III".to_string(), // Changed name
                description: "This is an airdrop, 土金, ك".to_string(),
                reward_denom: "uom".to_string(),
                total_reward: coin(100_000, "uom"),
                distribution_type: vec![DistributionType::LinearVesting {
                    percentage: Decimal::percent(100),
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(6).seconds(), // Dist ends before campaign here, which is fine.
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

    suite.instantiate_claimdrop_contract(None); // Alice is owner by default

    // Add allocations BEFORE creating campaign
    let allocations = &vec![
        (alice.to_string(), Uint128::new(100_000)),
        (bob.to_string(), Uint128::new(200_000)),
        (carol.to_string(), Uint128::new(300_000)),
    ];
    suite.add_allocations(
        // Owner adds allocations
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
                description: "This is an airdrop with cliff".to_string(),
                reward_denom: "uom".to_string(),
                total_reward: coin(600_000, "uom"), // Sum of allocations
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
        &coins(600_000, "uom"),
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    suite
        .query_allocations(Some(alice), None, None, |result| {
            let allocation = result.unwrap();
            assert_eq!(allocation.allocations.len(), 1);
            assert_eq!(
                allocation.allocations[0],
                (alice.to_string(), Uint128::new(100_000))
            );
        })
        .query_allocations(Some(bob), None, None, |result| {
            let allocation = result.unwrap();
            assert_eq!(allocation.allocations.len(), 1);
            assert_eq!(
                allocation.allocations[0],
                (bob.to_string(), Uint128::new(200_000))
            );
        })
        .query_allocations(Some(carol), None, None, |result| {
            let allocation = result.unwrap();
            assert_eq!(allocation.allocations.len(), 1);
            assert_eq!(
                allocation.allocations[0],
                (carol.to_string(), Uint128::new(300_000))
            );
        })
        .add_allocations(
            // Try adding by non-owner
            bob,
            allocations,
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

    // test allocations pagination
    // Note: Map iteration order means we should sort for robust tests if order matters.
    // The addresses are alice, bob, carol.
    suite
        .query_allocations(None, None, Some(2u16), |result| {
            let allocation_resp = result.unwrap();
            let mut allocations_vec = allocation_resp.allocations;
            allocations_vec.sort_by(|a, b| a.0.cmp(&b.0)); // Sort by address
            assert_eq!(allocations_vec.len(), 2);
            assert_eq!(
                allocations_vec,
                vec![
                    (alice.to_string(), Uint128::new(100_000)),
                    (carol.to_string(), Uint128::new(300_000))
                ]
            );
        })
        .query_allocations(None, Some(carol), None, |result| {
            // Start after bob
            let allocation_resp = result.unwrap();
            let mut allocations_vec = allocation_resp.allocations;
            allocations_vec.sort_by(|a, b| a.0.cmp(&b.0));
            assert_eq!(
                allocations_vec,
                vec![(bob.to_string(), Uint128::new(200_000))]
            );
        });
}
#[test]
fn test_add_duplicated_allocation() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);
    let alice = &suite.senders[0].clone();
    let bob = &suite.senders[1].clone();
    // let carol = &suite.senders[2].clone(); // Carol not used here
    // let current_time = &suite.get_time(); // current_time not strictly needed if campaign creation is not the focus

    suite.instantiate_claimdrop_contract(None); // Alice is owner

    // add allocations first
    let allocations_initial = &vec![
        (alice.to_string(), Uint128::new(100_000)),
        (bob.to_string(), Uint128::new(200_000)),
    ];
    suite.add_allocations(
        alice,
        allocations_initial,
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    // Try to add allocations with a duplicate
    let allocations_duplicate = &vec![
        (bob.to_string(), Uint128::new(50_000)), // Bob is a duplicate
        (suite.senders[2].clone().to_string(), Uint128::new(300_000)), // New address (Carol)
    ];

    suite.add_allocations(
        alice,
        allocations_duplicate,
        |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();
            match err {
                ContractError::AllocationAlreadyExists { address } => {
                    assert_eq!(address, bob.to_string()); // Fails on Bob
                }
                _ => {
                    panic!("Wrong error type, should return ContractError::AllocationAlreadyExists")
                }
            }
        },
    );
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

    suite.instantiate_claimdrop_contract(None); // Alice is owner

    suite.manage_campaign(
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
                start_time: current_time.plus_days(1).seconds(), // Campaign starts in 1 day
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
        (alice.to_string(), Uint128::new(10_000)), // Reduced for test clarity
        (bob.to_string(), Uint128::new(20_000)),
        (carol.to_string(), Uint128::new(300_000)),
    ];

    suite.add_day(); // Advance 1 day, campaign is not yet started because start_time is current_time.plus_days(1)
                     // To make campaign started, we need to advance past its start_time.
                     // If campaign_start_time = current_time_at_setup.plus_days(1).seconds(),
                     // then suite.add_day() makes current_time = current_time_at_setup + 1 day.
                     // So, campaign has just started.

    //Let's add one more day to be sure campaign has started
    suite.add_day();

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
    let alice = &suite.senders[0].clone(); // Owner
    let bob = &suite.senders[1].clone(); // Old address
    let carol = &suite.senders[2].clone(); // New address
    let current_time = &suite.get_time();

    // Upload initial allocation for Bob
    let allocations = &vec![(bob.to_string(), Uint128::new(100_000))];

    suite
        .instantiate_claimdrop_contract(None) // Alice is owner
        .add_allocations(
            alice,
            allocations,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );
    // Create campaign AFTER allocations are set, so replace_address can be tested before start
    suite.manage_campaign(
        alice,
        CampaignAction::CreateCampaign {
            params: Box::new(CampaignParams {
                name: "Test Airdrop I".to_string(),
                description: "Test replace address".to_string(),
                reward_denom: "uom".to_string(),
                total_reward: coin(100_000, "uom"), // Matches Bob's allocation
                distribution_type: vec![DistributionType::LumpSum {
                    percentage: Decimal::percent(100),               // All at once
                    start_time: current_time.plus_days(1).seconds(), // Starts tomorrow
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

    // At this point, campaign hasn't started. Bob has an allocation. Carol doesn't.
    // Bob has made no claims.

    // Replace Bob with Carol
    suite
        .replace_address(
            // Non-owner tries
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
            // Owner replaces
            alice,
            bob,
            carol,
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );

    // Verify Bob has no allocation or claims, Carol has Bob's original allocation
    suite
        .query_allocations(Some(bob), None, None, |result| {
            let allocation = result.unwrap();
            assert!(allocation.allocations.is_empty());
        })
        .query_claimed(Some(bob), None, None, |result| {
            let claimed_response = result.unwrap();
            assert!(claimed_response.claimed.is_empty());
        })
        .query_allocations(Some(carol), None, None, |result| {
            let allocation = result.unwrap();
            assert_eq!(allocation.allocations.len(), 1);
            assert_eq!(
                allocation.allocations[0],
                (carol.to_string(), Uint128::new(100_000))
            );
        })
        .query_claimed(Some(carol), None, None, |result| {
            let claimed_response = result.unwrap();
            assert!(claimed_response.claimed.is_empty()); // Carol shouldn't have claims yet
        });

    // Now, let's test scenario where Bob *had* claims
    // Re-setup: New contract, Bob gets allocation, claims some, then replace.
    suite.instantiate_claimdrop_contract(None); // Alice is owner
    suite.add_allocations(
        alice,
        allocations,
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    ); // Bob gets 100k again
    suite.manage_campaign(
        alice,
        CampaignAction::CreateCampaign {
            params: Box::new(CampaignParams {
                name: "Test Airdrop II".to_string(),
                description: "Test replace address with claims".to_string(),
                reward_denom: "uom".to_string(),
                total_reward: coin(100_000, "uom"),
                distribution_type: vec![
                    DistributionType::LumpSum {
                        percentage: Decimal::percent(50),   // 50% now
                        start_time: current_time.seconds(), // Starts now
                    },
                    DistributionType::LumpSum {
                        // Remaining 50% later
                        percentage: Decimal::percent(50),
                        start_time: current_time.plus_days(5).seconds(),
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

    // Bob claims the first 50% (50_000)
    suite.claim(
        bob,
        None,
        None,
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );
    suite.query_claimed(Some(bob), None, None, |res| {
        let claim = res.unwrap();
        assert_eq!(claim.claimed[0].1, coin(50_000, "uom"));
    });

    // Now replace Bob (who has claims) with Carol (who has no allocation yet in this campaign)
    // Before campaign starts for the *second* distribution type.
    suite.replace_address(
        alice,
        bob,
        carol,
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    // Check states: Bob should have no allocation/claims. Carol should have Bob's allocation and his claims.
    suite.query_allocations(Some(bob), None, None, |result| {
        let allocation = result.unwrap();
        assert!(allocation.allocations.is_empty());
    });
    suite.query_claimed(Some(bob), None, None, |result| {
        let claimed_response = result.unwrap();
        assert!(claimed_response.claimed.is_empty());
    });

    suite.query_allocations(Some(carol), None, None, |result| {
        let allocation = result.unwrap();
        assert_eq!(allocation.allocations[0].1, Uint128::new(100_000));
    });
    suite.query_claimed(Some(carol), None, None, |result| {
        let claim = result.unwrap();
        assert_eq!(claim.claimed[0].1, coin(50_000, "uom")); // Carol now has Bob's claim
    });

    // Advance time so Carol can claim the second part
    for _ in 0..6 {
        suite.add_day();
    }
    suite.claim(
        carol,
        None,
        None,
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    ); // Carol claims the next 50k
    suite.query_claimed(Some(carol), None, None, |result| {
        let claim = result.unwrap();
        assert_eq!(claim.claimed[0].1, coin(100_000, "uom")); // Carol has total 100k
    });
}

#[test]
fn test_blacklist_address() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);
    let alice = &suite.senders[0].clone(); // Owner
    let bob = &suite.senders[1].clone();
    let carol = &suite.senders[2].clone(); // Address to be blacklisted
    let current_time = &suite.get_time();

    suite.instantiate_claimdrop_contract(None); // Alice is owner

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

    suite.manage_campaign(
        alice,
        CampaignAction::CreateCampaign {
            params: Box::new(CampaignParams {
                name: "Test Airdrop I".to_string(),
                description: "This is an airdrop with cliff".to_string(),
                reward_denom: "uom".to_string(),
                total_reward: coin(600_000, "uom"), // Sum of allocations
                distribution_type: vec![DistributionType::LumpSum {
                    percentage: Decimal::percent(100),
                    start_time: current_time.plus_days(1).seconds(), // Starts tomorrow
                }],
                start_time: current_time.plus_days(1).seconds(),
                end_time: current_time.plus_days(14).seconds(),
            }),
        },
        &coins(600_000, "uom"),
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    suite
        .blacklist_address(
            // Non-owner fails
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
            // Owner succeeds
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

    suite.add_day(); // Advance 1 day, campaign starts

    // Carol (blacklisted) tries to claim, should fail
    suite.claim(
        carol,
        None,
        None,
        |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();
            match err {
                ContractError::AddressBlacklisted => {}
                _ => panic!("Wrong error type, should return ContractError::AddressBlacklisted"),
            }
        },
    );

    // remove from blacklist, claiming should work
    suite
        .blacklist_address(
            alice,
            carol,
            false, // Unblacklist
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .claim(
            carol,
            None,
            None,
            |result: Result<AppResponse, anyhow::Error>| {
                // Carol claims successfully
                result.unwrap();
            },
        );
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
///
#[test]
fn test_partial_claim_lump_sum() {
    let mut suite = TestingSuite::default_with_balances(vec![coin(1_000_000_000, "umantra")]);
    let user_total_allocation = Uint128::new(1000);
    let reward_denom = "umantra";

    let admin = suite.admin(); // Get admin from suite
    suite.instantiate_claimdrop_contract(Some(admin.to_string())); // Instantiate contract first
    let user = suite.senders[1].clone(); // Define user for the test

    suite.setup_campaign_for_partial_claims(user_total_allocation, reward_denom, &user);

    // Advance time to make lump sum available
    // Campaign starts at current_time_secs + 100
    // Lump sum (dist_lump_sum_start_time) starts at campaign_start_time + 50 = current_time_secs + 150
    suite.add_seconds(150 + 1);

    let lump_sum_share = Uint128::new(500); // 50%
    let partial_claim_amount = Uint128::new(100);

    assert!(partial_claim_amount < lump_sum_share);

    // Query rewards before claim
    suite.query_rewards(&user, |res| {
        let rewards = res.unwrap();
        assert_eq!(
            rewards.available_to_claim,
            vec![coin(lump_sum_share.u128(), reward_denom)]
        );
        assert_eq!(rewards.claimed, vec![]);
        assert_eq!(
            rewards.pending,
            vec![coin(user_total_allocation.u128(), reward_denom)]
        );
    });

    // User performs a partial claim
    suite.claim(
        &user,
        Some(user.to_string()),
        Some(partial_claim_amount),
        |res: Result<AppResponse, anyhow::Error>| {
            res.unwrap();
        },
    );

    // Check user balance
    suite.query_balance(reward_denom, &user, |bal| {
        assert_eq!(bal, partial_claim_amount);
    });

    // Query rewards after partial claim
    let remaining_lump_sum = lump_sum_share.checked_sub(partial_claim_amount).unwrap();
    suite.query_rewards(&user, |res| {
        let rewards = res.unwrap();
        assert_eq!(
            rewards.available_to_claim,
            vec![coin(remaining_lump_sum.u128(), reward_denom)]
        );
        assert_eq!(
            rewards.claimed,
            vec![coin(partial_claim_amount.u128(), reward_denom)]
        );
        let total_pending = user_total_allocation
            .checked_sub(partial_claim_amount)
            .unwrap();
        assert_eq!(
            rewards.pending,
            vec![coin(total_pending.u128(), reward_denom)]
        );
    });

    // User claims the rest of the lump sum
    suite.claim(
        &user,
        Some(user.to_string()),
        Some(remaining_lump_sum),
        |res: Result<AppResponse, anyhow::Error>| {
            res.unwrap();
        },
    );
    suite.query_balance(reward_denom, &user, |bal| {
        assert_eq!(bal, lump_sum_share);
    });
    suite.query_rewards(&user, |res| {
        let rewards = res.unwrap();
        // Vesting part might be available if time advanced enough for its start, but not cliff yet
        // For this specific test, focusing on lump sum, let's assume vesting not claimable yet.
        // If dist_vesting_start_time (current_time_secs + 200) is hit, it might show up.
        // Current block time is current_time_secs + 151. Vesting starts at +200. So, still 0 from vesting.
        assert_eq!(rewards.available_to_claim, vec![]);
        assert_eq!(
            rewards.claimed,
            vec![coin(lump_sum_share.u128(), reward_denom)]
        );
        let total_pending_after_full_lump =
            user_total_allocation.checked_sub(lump_sum_share).unwrap();
        assert_eq!(
            rewards.pending,
            vec![coin(total_pending_after_full_lump.u128(), reward_denom)]
        );
    });
}

#[test]
fn test_partial_claim_linear_vesting() {
    let mut suite = TestingSuite::default_with_balances(vec![coin(1_000_000_000, "umantra")]);
    let user_total_allocation = Uint128::new(1000);
    let reward_denom = "umantra";

    let admin = suite.admin();
    suite.instantiate_claimdrop_contract(Some(admin.to_string()));
    let user = suite.senders[1].clone();
    suite.setup_campaign_for_partial_claims(user_total_allocation, reward_denom, &user);

    let initial_block_time_secs = suite.get_time().seconds();
    // Campaign starts at initial_block_time_secs + 100
    // Vesting (dist_vesting_start_time) starts at campaign_start_time + 100 = initial_block_time_secs + 200
    // Cliff (dist_vesting_cliff_duration) is 50s, so cliff ends at initial_block_time_secs + 200 + 50 = initial_block_time_secs + 250
    // Vesting duration is 500s. Vesting ends at initial_block_time_secs + 200 + 500 = initial_block_time_secs + 700
    // Let's go to initial_block_time_secs + 250 (cliff end) + 100 (1/5th into vesting period post-cliff) = initial_block_time_secs + 350

    let target_time_secs = initial_block_time_secs + 350;
    let time_to_advance = target_time_secs - initial_block_time_secs;

    let mut block = suite.add_seconds(350);

    let vesting_share_total = Uint128::new(500); // 50%
                                                 // let time_into_vesting_post_cliff = 100u64; // We advanced 100s after cliff_end (which is 250s from campaign start + 100s for vesting start)
    let vesting_duration = 500u64;
    // Expected vested from linear part: (time_into_vesting_post_cliff / vesting_duration) * vesting_share_total
    // No, it's ( (current_time - vesting_start_time_abs).min(vesting_duration) / vesting_duration ) * vesting_share_total
    // current_time = initial_block_time_secs + 350
    // vesting_start_time_abs = initial_block_time_secs + 200
    // effective_time_passed_in_vesting = 350 - 200 = 150. This includes the cliff period.
    // The contract logic calculates vested amount based on time passed since vesting_start_time,
    // but only makes it available *after* cliff_end_time.
    // At cliff_end_time (initial_block_time_secs + 250), amount for (250-200)=50s duration is released.
    // So, at initial_block_time_secs + 350, effectively 150s of vesting has occurred.
    let effective_time_in_vesting = target_time_secs - (initial_block_time_secs + 200); // 350 - 200 = 150
    let expected_vested_amount = (Decimal::from_ratio(vesting_share_total, Uint128::one())
        * Decimal::from_ratio(
            Uint128::new(effective_time_in_vesting as u128),
            Uint128::new(vesting_duration as u128),
        ))
    .to_uint_floor(); // 500 * (150/500) = 150

    let lump_sum_share = Uint128::new(500); // 50% (available because target_time_secs > dist_lump_sum_start_time)
    let total_currently_available = lump_sum_share + expected_vested_amount; // 500 + 150 = 650

    suite.query_rewards(&user, |res| {
        let rewards = res.unwrap();
        assert_eq!(
            rewards.available_to_claim,
            vec![coin(total_currently_available.u128(), reward_denom)]
        );
    });

    let partial_claim_vesting_only = Uint128::new(50);
    assert!(partial_claim_vesting_only < expected_vested_amount);

    // User makes a claim. This will first take from lump sum.
    // To test partial vesting claim, let's first claim entire lump sum.
    suite.claim(
        &user,
        Some(user.to_string()),
        Some(lump_sum_share),
        |res: Result<AppResponse, anyhow::Error>| {
            res.unwrap();
        },
    );

    suite.query_rewards(&user, |res| {
        let rewards = res.unwrap();
        assert_eq!(
            rewards.available_to_claim,
            vec![coin(expected_vested_amount.u128(), reward_denom)]
        );
        assert_eq!(
            rewards.claimed,
            vec![coin(lump_sum_share.u128(), reward_denom)]
        );
    });

    // Now claim a part of the vested amount
    suite.claim(
        &user,
        Some(user.to_string()),
        Some(partial_claim_vesting_only),
        |res: Result<AppResponse, anyhow::Error>| {
            res.unwrap();
        },
    );

    let total_claimed_after_partial_vesting = lump_sum_share + partial_claim_vesting_only;
    suite.query_balance(reward_denom, &user, |bal| {
        assert_eq!(bal, total_claimed_after_partial_vesting);
    });

    let remaining_vested_available = expected_vested_amount
        .checked_sub(partial_claim_vesting_only)
        .unwrap();
    suite.query_rewards(&user, |res| {
        let rewards = res.unwrap();
        assert_eq!(
            rewards.available_to_claim,
            vec![coin(remaining_vested_available.u128(), reward_denom)]
        );
        assert_eq!(
            rewards.claimed,
            vec![coin(
                total_claimed_after_partial_vesting.u128(),
                reward_denom
            )]
        );
    });
}

#[test]
fn test_partial_claim_prioritization_lump_sum_then_vesting() {
    let mut suite = TestingSuite::default_with_balances(vec![coin(1_000_000_000, "umantra")]);
    let user_total_allocation = Uint128::new(1000);
    let reward_denom = "umantra";

    let admin = suite.admin();
    suite.instantiate_claimdrop_contract(Some(admin.to_string()));
    let user = suite.senders[1].clone();
    suite.setup_campaign_for_partial_claims(user_total_allocation, reward_denom, &user);

    let initial_block_time_secs = suite.get_time().seconds();
    // Target time: initial_block_time_secs + 350 (as in previous test, so lump sum + 150s of vesting available)
    let target_time_secs = initial_block_time_secs + 350;
    let time_to_advance = target_time_secs - initial_block_time_secs;
    suite.add_seconds(time_to_advance);

    let lump_sum_share = Uint128::new(500); // 50%
    let vesting_share_total = Uint128::new(500); // 50%
    let vesting_duration = 500u64;
    // effective_time_in_vesting = 350 (target) - 200 (vesting start from initial) = 150
    let effective_time_in_vesting = target_time_secs - (initial_block_time_secs + 200);
    let currently_vested_amount = (Decimal::from_ratio(vesting_share_total, Uint128::one())
        * Decimal::from_ratio(
            Uint128::new(effective_time_in_vesting as u128),
            Uint128::new(vesting_duration as u128),
        ))
    .to_uint_floor(); // 150

    let total_available = lump_sum_share + currently_vested_amount; // 500 + 150 = 650

    // Attempt to claim an amount that is more than lump sum but less than total available
    let amount_to_take_from_vesting = Uint128::new(50);
    let claim_amount = lump_sum_share + amount_to_take_from_vesting; // 500 + 50 = 550
    assert!(claim_amount <= total_available);

    suite.claim(
        &user,
        Some(user.to_string()),
        Some(claim_amount),
        |res: Result<AppResponse, anyhow::Error>| {
            res.unwrap();
        },
    );

    suite.query_balance(reward_denom, &user, |bal| {
        assert_eq!(bal, claim_amount);
    });

    // Check rewards: lump sum should be fully claimed, vesting partially.
    let remaining_vested_available = currently_vested_amount
        .checked_sub(amount_to_take_from_vesting)
        .unwrap(); // 150 - 50 = 100
    suite.query_rewards(&user, |res| {
        let rewards = res.unwrap();
        assert_eq!(
            rewards.available_to_claim,
            vec![coin(remaining_vested_available.u128(), reward_denom)]
        );
        assert_eq!(
            rewards.claimed,
            vec![coin(claim_amount.u128(), reward_denom)]
        );
    });
}

#[test]
fn test_claim_zero_amount_fails() {
    let mut suite = TestingSuite::default_with_balances(vec![coin(1_000_000_000, "umantra")]);
    let user_total_allocation = Uint128::new(1000);
    let reward_denom = "umantra";

    let admin = suite.admin();
    suite.instantiate_claimdrop_contract(Some(admin.to_string()));
    let user = suite.senders[1].clone();
    suite.setup_campaign_for_partial_claims(user_total_allocation, reward_denom, &user);

    // Advance time for lump sum to be available
    let initial_block_time = suite.get_time().seconds();
    let time_to_advance = (initial_block_time + 150 + 1) - initial_block_time;
    suite.add_seconds(time_to_advance);

    suite.claim(
        &user,
        Some(user.to_string()),
        Some(Uint128::zero()),
        |res: Result<AppResponse, anyhow::Error>| {
            let err = res.unwrap_err();
            assert!(err.to_string().contains("amount must be greater than zero"));
        },
    );
}

#[test]
fn test_claim_more_than_currently_available_fails() {
    let mut suite = TestingSuite::default_with_balances(vec![coin(1_000_000_000, "umantra")]);
    let user_total_allocation = Uint128::new(1000);
    let reward_denom = "umantra";

    let admin = suite.admin();
    suite.instantiate_claimdrop_contract(Some(admin.to_string()));
    let user = suite.senders[1].clone();
    suite.setup_campaign_for_partial_claims(user_total_allocation, reward_denom, &user);

    // Advance time for lump sum to be available
    let initial_block_time = suite.get_time().seconds();
    let time_to_advance = (initial_block_time + 150 + 1) - initial_block_time;
    suite.add_seconds(time_to_advance);

    let lump_sum_share = Uint128::from(500u128); // 50%
                                                 // At this point, only lump_sum_share is available. Vesting hasn't started/cliffed.
    let excessive_amount = lump_sum_share + Uint128::new(1); // 501

    suite.claim(
        &user,
        Some(user.to_string()),
        Some(excessive_amount),
        |res: Result<AppResponse, anyhow::Error>| {
            let err = res.unwrap_err();
            assert!(err
                .to_string()
                .contains("exceeds available claimable amount"));
        },
    );
}

#[test]
fn test_claim_full_amount_when_none_specified_after_partial_claims() {
    let mut suite = TestingSuite::default_with_balances(vec![coin(1_000_000_000, "umantra")]);
    let user_total_allocation = Uint128::new(1000);
    let reward_denom = "umantra";

    let admin = suite.admin();
    suite.instantiate_claimdrop_contract(Some(admin.to_string()));
    let user = suite.senders[1].clone();
    suite.setup_campaign_for_partial_claims(user_total_allocation, reward_denom, &user);

    let initial_block_time_secs = suite.get_time().seconds();
    // Advance time for lump sum and some vesting (target: initial_block_time_secs + 350)
    let target_time_secs_1 = initial_block_time_secs + 350;
    let time_to_advance_1 = target_time_secs_1 - initial_block_time_secs;
    suite.add_seconds(time_to_advance_1);

    let lump_sum_share = (Decimal::from_ratio(user_total_allocation, Uint128::from(1u128))
        * Decimal::percent(50))
    .to_uint_floor(); // 500
    let vesting_share_total = (Decimal::from_ratio(user_total_allocation, Uint128::from(1u128))
        * Decimal::percent(50))
    .to_uint_floor(); // 500
    let vesting_duration = 500u64;
    let effective_time_in_vesting_1 = target_time_secs_1 - (initial_block_time_secs + 200); // 150
    let currently_vested_amount_1 = (Decimal::from_ratio(vesting_share_total, Uint128::one())
        * Decimal::from_ratio(
            Uint128::new(effective_time_in_vesting_1 as u128),
            Uint128::new(vesting_duration as u128),
        ))
    .to_uint_floor(); // 150
    let total_currently_available_1 = lump_sum_share + currently_vested_amount_1; // 650

    // First, a partial claim
    let partial_amount = Uint128::new(150); // Takes from lump sum
    suite.claim(
        &user,
        Some(user.to_string()),
        Some(partial_amount),
        |res: Result<AppResponse, anyhow::Error>| {
            res.unwrap();
        },
    );

    let amount_claimed_so_far = partial_amount;
    let remaining_available_now = total_currently_available_1
        .checked_sub(partial_amount)
        .unwrap(); // 650 - 150 = 500

    suite.query_rewards(&user, |res| {
        let r = res.unwrap();
        assert_eq!(
            r.available_to_claim,
            vec![coin(remaining_available_now.u128(), reward_denom)]
        );
        assert_eq!(
            r.claimed,
            vec![coin(amount_claimed_so_far.u128(), reward_denom)]
        );
    });

    // Now, claim with amount = None (should claim all remaining available now)
    suite.claim(
        &user,
        Some(user.to_string()),
        None,
        |res: Result<AppResponse, anyhow::Error>| {
            res.unwrap();
        },
    );

    suite.query_balance(reward_denom, &user, |bal| {
        assert_eq!(bal, total_currently_available_1); // 150 + 500 = 650
    });

    suite.query_rewards(&user, |res| {
        let r = res.unwrap();
        assert_eq!(r.available_to_claim, vec![]);
        assert_eq!(
            r.claimed,
            vec![coin(total_currently_available_1.u128(), reward_denom)]
        );
    });

    // Advance time to make everything vested (campaign_end_time is initial_block_time_secs + 100 + 1000 = initial_block_time_secs + 1100)
    let target_time_secs_2 = initial_block_time_secs + 1100 + 1;
    let time_to_advance_2 = target_time_secs_2 - initial_block_time_secs;
    suite.add_seconds(time_to_advance_2);

    // let _amount_already_claimed_total = total_currently_available_1; // 650
    // Total vesting share is 500. Amount vested from previous stage was 150. So 500-150=350 is newly available from vesting.
    let newly_available_from_vesting_completion =
        vesting_share_total.saturating_sub(currently_vested_amount_1); // 500 - 150 = 350

    suite.query_rewards(&user, |res| {
        let r = res.unwrap();
        assert_eq!(
            r.available_to_claim,
            vec![coin(
                newly_available_from_vesting_completion.u128(),
                reward_denom
            )]
        );
    });

    // Claim the final remainder
    suite.claim(
        &user,
        Some(user.to_string()),
        None,
        |res: Result<AppResponse, anyhow::Error>| {
            res.unwrap();
        },
    );
    suite.query_balance(reward_denom, &user, |bal| {
        assert_eq!(bal, user_total_allocation);
    });
    suite.query_rewards(&user, |res| {
        let r = res.unwrap();
        assert_eq!(r.available_to_claim, vec![]);
        assert_eq!(
            r.claimed,
            vec![coin(user_total_allocation.u128(), reward_denom)]
        );
        assert_eq!(r.pending, vec![]);
    });
}

// Make sure this is the last part of the file, or adjust accordingly

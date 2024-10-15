use std::cell::RefCell;
use std::str::FromStr;

use cosmwasm_std::{coin, coins, Decimal, Uint128};
use cw_multi_test::AppResponse;

use airdrop_manager::error::ContractError;
use airdrop_manager::msg::{
    CampaignAction, CampaignFilter, CampaignParams, DistributionType, RewardsResponse,
};

use crate::suite::TestingSuite;

mod suite;

#[test]
fn instantiate_airdrop_manager() {
    let mut suite = TestingSuite::default_with_balances(vec![coin(1_000_000_000, "uom")]);
    suite.instantiate_airdrop_manager(None);
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

    let campaign_id = RefCell::new("".to_string());

    suite
        .instantiate_airdrop_manager(None)
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
                    owner: None,
                    salt: "zkzv117igbvuwqk12a68kx2zj823v7rg".to_string(),
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
                },
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidCampaignParam { param, .. } => { assert_eq!(param, "name"); }
                    _ => panic!("Wrong error type, should return ContractError::InvalidCampaignParam"),
                }
            },
        ).manage_campaign(
        alice,
        CampaignAction::CreateCampaign {
            params: CampaignParams {
                owner: None,
                salt: "".to_string(),
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
            },
        },
        &coins(100_000, "uom"),
        |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();
            match err {
                ContractError::InvalidCampaignParam { param, .. } => { assert_eq!(param, "description"); }
                _ => panic!("Wrong error type, should return ContractError::InvalidCampaignParam"),
            }
        },
    )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
                    owner: None,
                    salt: "".to_string(),
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
                },
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                println!("{:?}", result);
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidCampaignParam { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::InvalidRewardAmount"),
                }
            },
        ).manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
                    owner: None,
                    salt: "gm7ki1rn5wo4x5105j8uclbh99s9".to_string(),
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
                },
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidCampaignParam { param, .. } => { assert_eq!(param, "salt"); }
                    _ => panic!("Wrong error type, should return ContractError::InvalidCampaignParam"),
                }
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
                    owner: None,
                    salt: "gm7ki1rn5wo4x5105j8uclbh99s9db7v".to_string(),
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
                },
            },
            &coins(100_000, "uusdc"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::PaymentError { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::PaymentError"),
                }
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
                    owner: None,
                    salt: "gm7ki1rn5wo4x5105j8uclbh99s9db7v".to_string(),
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
                },
            },
            &vec![coin(100_000, "uom"), coin(100_000, "uusdc")],
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::PaymentError { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::PaymentError"),
                }
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
                    owner: None,
                    salt: "gm7ki1rn5wo4x5105j8uclbh99s9db7v".to_string(),
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
                },
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
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
                    owner: None,
                    salt: "gm7ki1rn5wo4x5105j8uclbh99s9db7v".to_string(),
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![],
                    cliff_duration: None,
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: "a79197d1f2f90797e65c545a99662630da89baf820af47b0044cd910339cb4fd"
                        .to_string(),
                },
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidCampaignParam { param, .. } => { assert_eq!(param, "distribution_type"); }
                    _ => panic!("Wrong error type, should return ContractError::InvalidCampaignParam"),
                }
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
                    owner: None,
                    salt: "gm7ki1rn5wo4x5105j8uclbh99s9db7v".to_string(),
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
                },
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
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
                    owner: None,
                    salt: "gm7ki1rn5wo4x5105j8uclbh99s9db7v".to_string(),
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
                },
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
        //todo missing to validate the times of the distributions are correct
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
                    owner: None,
                    salt: "gm7ki1rn5wo4x5105j8uclbh99s9db7v".to_string(),
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
                },
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidCampaignParam { param, .. } => { assert_eq!(param, "cliff_duration"); }
                    _ => panic!("Wrong error type, should return ContractError::InvalidCampaignParam"),
                }
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
                    owner: None,
                    salt: "gm7ki1rn5wo4x5105j8uclbh99s9db7v".to_string(),
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
                },
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidCampaignParam { param, .. } => { assert_eq!(param, "cliff_duration"); }
                    _ => panic!("Wrong error type, should return ContractError::InvalidCampaignParam"),
                }
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
                    owner: None,
                    salt: "gm7ki1rn5wo4x5105j8uclbh99s9db7v".to_string(),
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
                },
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidCampaignParam { param, .. } => { assert_eq!(param, "start_time"); }
                    _ => panic!("Wrong error type, should return ContractError::InvalidCampaignParam"),
                }
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
                    owner: None,
                    salt: "gm7ki1rn5wo4x5105j8uclbh99s9db7v".to_string(),
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
                },
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::InvalidCampaignParam { param, .. } => { assert_eq!(param, "start_time"); }
                    _ => panic!("Wrong error type, should return ContractError::InvalidCampaignParam"),
                }
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
                    owner: None,
                    salt: "gm7ki1rn5wo4x5105j8uclbh99s9db7v".to_string(),
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
                    merkle_root: "".to_string(),
                },
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
                params: CampaignParams {
                    owner: None,
                    salt: "zkzv117igbvuwqk12a68kx2zj823v7rg".to_string(),
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
                    merkle_root: "a79197d1f2f9baf820af47b0044cd910339cb4fd".to_string(),
                },
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                println!("{:?}", result);
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
                params: CampaignParams {
                    owner: None,
                    salt: "zkzv117igbvuwqk12a68kx2zj823v7rg".to_string(),
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
                },
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_campaigns(None, None, None, {
            |result| {
                let result = result.unwrap();
                assert_eq!(result.campaigns.len(), 1);
                campaign_id.replace(result.campaigns[0].id.clone());
            }
        });

    println!("campaign_id: {:?}", campaign_id.borrow());
    // claim
    suite.claim(
        alice,
        &campaign_id.borrow(),
        Uint128::new(20_000u128),
        None,
        vec![
            "4fdd358f8c2cb6bf7d151577abc60e39309ea9a7ad12d0105cbb3fe9b43d7369".to_string(),
            "db9ef98f27db88bdeb6bd0d59f1e704d122e6f0744308fa9a271d1ab71b6bdc7".to_string(),
            "f64300b3dcdec17cc2ddd3d7af23a11c5111bd78c1227a4932f468d11b1e850e".to_string(),
        ],
        |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();

            match err {
                ContractError::CampaignError { .. } => {}
                _ => panic!("Wrong error type, should return ContractError::CampaignError"),
            }
        },
    );

    suite.add_day();

    suite
        .claim(
            alice,
            // trying to claim a campaign that doesn't exist
            "2",
            Uint128::new(10_000u128),
            None,
            vec![
                "4fdd358f8c2cb6bf7d151577abc60e39309ea9a7ad12d0105cbb3fe9b43d7369".to_string(),
                "db9ef98f27db88bdeb6bd0d59f1e704d122e6f0744308fa9a271d1ab71b6bdc7".to_string(),
                "f64300b3dcdec17cc2ddd3d7af23a11c5111bd78c1227a4932f468d11b1e850e".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::CampaignNotFound { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::CampaignNotFound"),
                }
            },
        )
        .claim(
            alice,
            &campaign_id.borrow(),
            // pretending to be entitled to more tokens than the campaign has to offer for this user
            Uint128::new(20_000u128),
            None,
            vec![
                "4fdd358f8c2cb6bf7d151577abc60e39309ea9a7ad12d0105cbb3fe9b43d7369".to_string(),
                "db9ef98f27db88bdeb6bd0d59f1e704d122e6f0744308fa9a271d1ab71b6bdc7".to_string(),
                "f64300b3dcdec17cc2ddd3d7af23a11c5111bd78c1227a4932f468d11b1e850e".to_string(),
            ],
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
            &campaign_id.borrow(),
            Uint128::new(10_000u128),
            None,
            // provide wrong proofs
            vec![
                "4fdd358f8c2cb6bf7d151577abc60e39309ea9a7ad12d0105cbb3fe9b43d7369".to_string(),
                "f64300b3dcdec17cc2ddd3d7af23a11c5111bd78c1227a4932f468d11b1e850e".to_string(),
            ],
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
            &campaign_id.borrow(),
            Uint128::new(10_000u128),
            // try claiming for someone else, with the wrong proofs
            Some(bob.to_string()),
            vec![
                "4fdd358f8c2cb6bf7d151577abc60e39309ea9a7ad12d0105cbb3fe9b43d7369".to_string(),
                "db9ef98f27db88bdeb6bd0d59f1e704d122e6f0744308fa9a271d1ab71b6bdc7".to_string(),
                "f64300b3dcdec17cc2ddd3d7af23a11c5111bd78c1227a4932f468d11b1e850e".to_string(),
            ],
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
            &campaign_id.borrow(),
            Uint128::new(10_000u128),
            Some(alice.to_string()),
            vec![
                "4fdd358f8c2cb6bf7d151577abc60e39309ea9a7ad12d0105cbb3fe9b43d7369".to_string(),
                "db9ef98f27db88bdeb6bd0d59f1e704d122e6f0744308fa9a271d1ab71b6bdc7".to_string(),
                "f64300b3dcdec17cc2ddd3d7af23a11c5111bd78c1227a4932f468d11b1e850e".to_string(),
            ],
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
            &campaign_id.borrow(),
            Uint128::new(10_000u128),
            None,
            vec![
                "e417db26095b3788271fbe52e9eaa09ee713c4b04e2027fd1fae7b370da0dd41".to_string(),
                "105345c5c8dec04489b07c716e7dad74585a25ab4f3944595e0c538350073e37".to_string(),
                "f64300b3dcdec17cc2ddd3d7af23a11c5111bd78c1227a4932f468d11b1e850e".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_balance("uom", bob, |balance| {
            assert_eq!(balance, Uint128::new(1_000_010_000));
        })
        .query_campaigns(None, None, None, {
            |result| {
                let response = result.unwrap();

                assert_eq!(response.campaigns.len(), 1);
                assert_eq!(response.campaigns[0].claimed, coin(20_000u128, "uom"));
            }
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
            &campaign_id.borrow(),
            Uint128::new(20_000u128),
            None,
            vec![
                "a2fb8c4ce922bfee77b860eac06c725f5869b63af67edeac09161599d455f230".to_string(),
                "105345c5c8dec04489b07c716e7dad74585a25ab4f3944595e0c538350073e37".to_string(),
                "f64300b3dcdec17cc2ddd3d7af23a11c5111bd78c1227a4932f468d11b1e850e".to_string(),
            ],
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
        .instantiate_airdrop_manager(None)
        .manage_campaign(
            dan,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
                    owner: None,
                    salt: "zkzv117igbvuwqk12a68kx2zj823v7rg".to_string(),
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
                    merkle_root: "e0c8ef9777e5561011c106768f2a444a68134ca44ccf76521c6a150add289813"
                        .to_string(),
                },
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_campaigns(None, None, None, {
            |result| {
                println!(">>> {:?}", result);
                assert_eq!(result.unwrap().campaigns.len(), 1);
            }
        });

    // claim
    suite
        .add_day()
        .claim(
            bob,
            "e49bb620abaaba52e721976ead1f8da2b9bfb8622ea4bfcc21e0a1a9d0e1675f",
            Uint128::new(10_000u128),
            Some(alice.to_string()),
            vec![
                "b86a1eb1de627ad64975c6dbf2e997d4c700f36a26738596d8bfb39263a47f4e".to_string(),
                "f7e94fe76a6bb12ad769fe17cbe9e9de73983f9e462f32d4dd8a49f1c73c8670".to_string(),
                "f28ab71cb3f3a8837ab8ffc1f02e9a1b9250b73044407d574aed938ddf82355d".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .claim(
            bob,
            "e49bb620abaaba52e721976ead1f8da2b9bfb8622ea4bfcc21e0a1a9d0e1675f",
            Uint128::new(10_000u128),
            None,
            vec!["ea87adeadd21f8d75a4b30cbce54c1043e9a646ec74c7cdce4e6a16a694f6add".to_string()],
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
            CampaignAction::EndCampaign {
                campaign_id: "e49bb620abaaba52e721976ead1f8da2b9bfb8622ea4bfcc21e0a1a9d0e1675f"
                    .to_string(),
            },
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
            CampaignAction::EndCampaign {
                campaign_id: "nonexistent_campaign".to_string(),
            },
            &[],
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::CampaignNotFound { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::CampaignNotFound"),
                }
            },
        )
        .manage_campaign(
            // bob tries to end the campaign
            bob,
            CampaignAction::EndCampaign {
                campaign_id: "e49bb620abaaba52e721976ead1f8da2b9bfb8622ea4bfcc21e0a1a9d0e1675f"
                    .to_string(),
            },
            &[],
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::Unauthorized { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::Unauthorized"),
                }
            },
        )
        .query_campaigns(None, None, None, {
            |result| {
                println!("{:?}", result);
                let response = result.unwrap();

                assert_eq!(response.campaigns.len(), 1);
                assert_eq!(response.campaigns[0].claimed, coin(20_000u128, "uom"));
            }
        })
        .query_balance("uom", dan, |balance| {
            assert_eq!(balance, Uint128::new(999_900_000));
        })
        .manage_campaign(
            // alice should be able to, since she is the owner of the contract
            alice,
            CampaignAction::EndCampaign {
                campaign_id: "e49bb620abaaba52e721976ead1f8da2b9bfb8622ea4bfcc21e0a1a9d0e1675f"
                    .to_string(),
            },
            &[],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        // the owner of the campaign, dan, got the remaining tokens back, which were 80k as 20k
        // were claimed by bob and alice
        .query_balance("uom", dan, |balance| {
            assert_eq!(balance, Uint128::new(999_980_000));
        })
        .query_campaigns(None, None, None, {
            |result| {
                println!("{:?}", result);
                let response = result.unwrap();

                assert_eq!(response.campaigns.len(), 1);
                assert_eq!(
                    response.campaigns[0].claimed,
                    response.campaigns[0].reward_asset
                );
            }
        });

    // now carol tries to claim but it's too late
    suite.claim(
        carol,
        "e49bb620abaaba52e721976ead1f8da2b9bfb8622ea4bfcc21e0a1a9d0e1675f",
        Uint128::new(20_000u128),
        None,
        vec![
            "0708aeaab00071d640329f00e69fab58e7778f418937410595c2aa7bb2a30f49".to_string(),
            "88aabb1557935bf7e6382ca27d8c16271b2ce3a412468bc8c98791991aefec4b".to_string(),
            "f28ab71cb3f3a8837ab8ffc1f02e9a1b9250b73044407d574aed938ddf82355d".to_string(),
        ],
        |result: Result<AppResponse, anyhow::Error>| {
            let err = result.unwrap_err().downcast::<ContractError>().unwrap();

            match err {
                ContractError::CampaignError { .. } => {}
                _ => panic!("Wrong error type, should return ContractError::CampaignError"),
            }
        },
    );
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

    suite.instantiate_airdrop_manager(None).manage_campaign(
        alice,
        CampaignAction::CreateCampaign {
            params: CampaignParams {
                owner: None,
                salt: "zkzv117igbvuwqk12a68kx2zj823v7rg".to_string(),
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
                merkle_root: "a2835f4d5c3b7f9f58ec60e85a52e6e24985777540a214ee4080431bacf4882a"
                    .to_string(),
            },
        },
        &coins(100_000, "uom"),
        |result: Result<AppResponse, anyhow::Error>| {
            println!("{:?}", result);
            result.unwrap();
        },
    );

    suite.add_day();

    println!(">>>>> LumpSum claiming");
    suite
        .claim(
            alice,
            "8032e08c50569671b6c2a6b782f2622e761f421f753a29d34c50668fb1b18880",
            Uint128::new(10_000u128),
            None,
            vec![
                "a761a28228a04d0052587cb11a3a380644545aca9b4953ccb386ac307d458cb2".to_string(),
                "f8fe416e0991cb111c9282cd9b31fff4d298a038b2852666e70b5abc25a3b32c".to_string(),
                "f4474b4e3cb6bcc1996bd91287efd372dad310250953ddfea64f0d552ab8cd2f".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_campaigns(None, None, None, {
            |result| {
                println!("{:?}", result);
                let response = result.unwrap();
                assert_eq!(response.campaigns.len(), 1);
                assert_eq!(response.campaigns[0].claimed, coin(2_500u128, "uom"));

                println!(">>>>> trying to claim again without moving time, should err");
            }
        })
        .query_rewards(
            "8032e08c50569671b6c2a6b782f2622e761f421f753a29d34c50668fb1b18880",
            Uint128::new(10_000u128),
            alice.to_string(),
            vec![
                "a761a28228a04d0052587cb11a3a380644545aca9b4953ccb386ac307d458cb2".to_string(),
                "f8fe416e0991cb111c9282cd9b31fff4d298a038b2852666e70b5abc25a3b32c".to_string(),
                "f4474b4e3cb6bcc1996bd91287efd372dad310250953ddfea64f0d552ab8cd2f".to_string(),
            ],
            |result| {
                assert_eq!(
                    result.unwrap(),
                    RewardsResponse {
                        claimed: coins(2_500u128, "uom"),
                        pending: coins(10_000u128 - 2_500u128, "uom"),
                        available_to_claim: vec![],
                    }
                );
            },
        )
        .claim(
            alice,
            "8032e08c50569671b6c2a6b782f2622e761f421f753a29d34c50668fb1b18880",
            Uint128::new(10_000u128),
            None,
            vec![
                "a761a28228a04d0052587cb11a3a380644545aca9b4953ccb386ac307d458cb2".to_string(),
                "f8fe416e0991cb111c9282cd9b31fff4d298a038b2852666e70b5abc25a3b32c".to_string(),
                "f4474b4e3cb6bcc1996bd91287efd372dad310250953ddfea64f0d552ab8cd2f".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                println!("{:?}", result);
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::NothingToClaim { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::NothingToClaim"),
                }

                println!(">>>>> add a week and claim");
            },
        )
        .add_week()
        .claim(
            alice,
            "8032e08c50569671b6c2a6b782f2622e761f421f753a29d34c50668fb1b18880",
            Uint128::new(10_000u128),
            None,
            vec![
                "a761a28228a04d0052587cb11a3a380644545aca9b4953ccb386ac307d458cb2".to_string(),
                "f8fe416e0991cb111c9282cd9b31fff4d298a038b2852666e70b5abc25a3b32c".to_string(),
                "f4474b4e3cb6bcc1996bd91287efd372dad310250953ddfea64f0d552ab8cd2f".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                println!("{:?}", result);
                result.unwrap();
                println!(">>>>> try claiming again without moving time, should err");
            },
        )
        .claim(
            alice,
            "8032e08c50569671b6c2a6b782f2622e761f421f753a29d34c50668fb1b18880",
            Uint128::new(10_000u128),
            None,
            vec![
                "a761a28228a04d0052587cb11a3a380644545aca9b4953ccb386ac307d458cb2".to_string(),
                "f8fe416e0991cb111c9282cd9b31fff4d298a038b2852666e70b5abc25a3b32c".to_string(),
                "f4474b4e3cb6bcc1996bd91287efd372dad310250953ddfea64f0d552ab8cd2f".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                println!("result: {:?}", err);
                match err {
                    ContractError::NothingToClaim { .. } => {}
                    _ => panic!("Wrong error type, should return ContractError::NothingToClaim"),
                }

                println!(">>>>> add 4 days and claim");
            },
        )
        .add_day()
        .add_day()
        .add_day()
        .add_day()
        .query_rewards(
            "8032e08c50569671b6c2a6b782f2622e761f421f753a29d34c50668fb1b18880",
            Uint128::new(10_000u128),
            alice.to_string(),
            vec![
                "a761a28228a04d0052587cb11a3a380644545aca9b4953ccb386ac307d458cb2".to_string(),
                "f8fe416e0991cb111c9282cd9b31fff4d298a038b2852666e70b5abc25a3b32c".to_string(),
                "f4474b4e3cb6bcc1996bd91287efd372dad310250953ddfea64f0d552ab8cd2f".to_string(),
            ],
            |result| {
                assert_eq!(
                    result.unwrap(),
                    RewardsResponse {
                        claimed: coins(3_571u128, "uom"),
                        pending: coins(10_000u128 - 3_571u128, "uom"),
                        available_to_claim: coins(4_285u128, "uom"),
                    }
                );
            },
        )
        .claim(
            alice,
            "8032e08c50569671b6c2a6b782f2622e761f421f753a29d34c50668fb1b18880",
            Uint128::new(10_000u128),
            None,
            vec![
                "a761a28228a04d0052587cb11a3a380644545aca9b4953ccb386ac307d458cb2".to_string(),
                "f8fe416e0991cb111c9282cd9b31fff4d298a038b2852666e70b5abc25a3b32c".to_string(),
                "f4474b4e3cb6bcc1996bd91287efd372dad310250953ddfea64f0d552ab8cd2f".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                println!("{:?}", result);
                result.unwrap();
                println!(
                    ">>>>> add 2 more weeks and claim, the campaign should have finished by then"
                );
            },
        )
        .query_rewards(
            "8032e08c50569671b6c2a6b782f2622e761f421f753a29d34c50668fb1b18880",
            Uint128::new(10_000u128),
            alice.to_string(),
            vec![
                "a761a28228a04d0052587cb11a3a380644545aca9b4953ccb386ac307d458cb2".to_string(),
                "f8fe416e0991cb111c9282cd9b31fff4d298a038b2852666e70b5abc25a3b32c".to_string(),
                "f4474b4e3cb6bcc1996bd91287efd372dad310250953ddfea64f0d552ab8cd2f".to_string(),
            ],
            |result| {
                assert_eq!(
                    result.unwrap(),
                    RewardsResponse {
                        claimed: coins(7_856u128, "uom"),
                        pending: coins(10_000u128 - 7_856u128, "uom"),
                        available_to_claim: vec![],
                    }
                );
            },
        )
        .add_week()
        .add_week()
        .claim(
            alice,
            "8032e08c50569671b6c2a6b782f2622e761f421f753a29d34c50668fb1b18880",
            Uint128::new(10_000u128),
            None,
            vec![
                "a761a28228a04d0052587cb11a3a380644545aca9b4953ccb386ac307d458cb2".to_string(),
                "f8fe416e0991cb111c9282cd9b31fff4d298a038b2852666e70b5abc25a3b32c".to_string(),
                "f4474b4e3cb6bcc1996bd91287efd372dad310250953ddfea64f0d552ab8cd2f".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                println!("{:?}", result);
                println!(">>>>> add a day and try claiming again, should err");
            },
        )
        .add_day()
        .claim(
            alice,
            "8032e08c50569671b6c2a6b782f2622e761f421f753a29d34c50668fb1b18880",
            Uint128::new(10_000u128),
            None,
            vec![
                "a761a28228a04d0052587cb11a3a380644545aca9b4953ccb386ac307d458cb2".to_string(),
                "f8fe416e0991cb111c9282cd9b31fff4d298a038b2852666e70b5abc25a3b32c".to_string(),
                "f4474b4e3cb6bcc1996bd91287efd372dad310250953ddfea64f0d552ab8cd2f".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                println!("{:?}", result);
                println!(">>>>> query campaigns");
            },
        )
        .query_campaigns(None, None, None, {
            |result| {
                println!("{:?}", result);
                let response = result.unwrap();
                assert_eq!(response.campaigns.len(), 1);
                assert_eq!(response.campaigns[0].claimed, coin(10_000u128, "uom"));
                println!(">>>>> dan claiming all at once");
            }
        })
        .query_rewards(
            "8032e08c50569671b6c2a6b782f2622e761f421f753a29d34c50668fb1b18880",
            Uint128::new(35_000u128),
            dan.to_string(),
            vec![
                "8ea89eba5efd17edc104d54946785b5dfa0b9ee420283fd91553dd11e6912f28".to_string(),
                "390a946104c5f84c48e822b2bff8bf3cedffa95f7b44f8e2d48d9d5b2e4a8dd8".to_string(),
                "f4474b4e3cb6bcc1996bd91287efd372dad310250953ddfea64f0d552ab8cd2f".to_string(),
            ],
            |result| {
                assert_eq!(
                    result.unwrap(),
                    RewardsResponse {
                        claimed: vec![],
                        pending: coins(35_000u128, "uom"),
                        available_to_claim: coins(35_000u128, "uom"),
                    }
                );
            },
        )
        .claim(
            dan,
            "8032e08c50569671b6c2a6b782f2622e761f421f753a29d34c50668fb1b18880",
            Uint128::new(35_000u128),
            None,
            vec![
                "8ea89eba5efd17edc104d54946785b5dfa0b9ee420283fd91553dd11e6912f28".to_string(),
                "390a946104c5f84c48e822b2bff8bf3cedffa95f7b44f8e2d48d9d5b2e4a8dd8".to_string(),
                "f4474b4e3cb6bcc1996bd91287efd372dad310250953ddfea64f0d552ab8cd2f".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                println!("{:?}", result);
                println!(">>>>> query campaigns");
            },
        )
        .query_rewards(
            "8032e08c50569671b6c2a6b782f2622e761f421f753a29d34c50668fb1b18880",
            Uint128::new(35_000u128),
            dan.to_string(),
            vec![
                "8ea89eba5efd17edc104d54946785b5dfa0b9ee420283fd91553dd11e6912f28".to_string(),
                "390a946104c5f84c48e822b2bff8bf3cedffa95f7b44f8e2d48d9d5b2e4a8dd8".to_string(),
                "f4474b4e3cb6bcc1996bd91287efd372dad310250953ddfea64f0d552ab8cd2f".to_string(),
            ],
            |result| {
                assert_eq!(
                    result.unwrap(),
                    RewardsResponse {
                        claimed: coins(35_000u128, "uom"),
                        pending: vec![],
                        available_to_claim: vec![],
                    }
                );
            },
        )
        .query_campaigns(None, None, None, {
            |result| {
                println!("{:?}", result);
                let response = result.unwrap();
                assert_eq!(response.campaigns.len(), 1);
                assert_eq!(response.campaigns[0].claimed, coin(45_000u128, "uom"));
            }
        });
}

#[test]
fn claim_campaigns_with_cliff() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);

    let alice = &suite.senders[0].clone();
    let current_time = &suite.get_time();
    let campaign_id_1 = RefCell::new("".to_string());
    let campaign_id_2 = RefCell::new("".to_string());

    suite
        .instantiate_airdrop_manager(None)
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
                    owner: None,
                    salt: "zkzv117igbvuwqk12a68kx2zj823v7rg".to_string(),
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
                    merkle_root: "a79197d1f2f90797e65c545a99662630da89baf820af47b0044cd910339cb4fd"
                        .to_string(),
                },
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_campaigns(None, None, None, |result| {
            *campaign_id_1.borrow_mut() = result.unwrap().campaigns[0].id.clone();
        })
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
                    owner: None,
                    salt: "zkzv117igbvuwqk12a68kx2zj823v7rg".to_string(),
                    name: "Test Airdrop II".to_string(),
                    description: "This is an airdrop with cliff".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![
                        DistributionType::LumpSum {
                            percentage: Decimal::percent(10),
                            start_time: current_time.seconds(),
                            end_time: current_time.plus_days(30).seconds(), // 1 month
                        },
                        DistributionType::LinearVesting {
                            percentage: Decimal::percent(90),
                            start_time: current_time.seconds(),
                            end_time: current_time.plus_days(30).seconds(), // 1 month
                        },
                    ],
                    cliff_duration: Some(86_400 * 7), // 7 days cliff
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(30).seconds(),
                    merkle_root: "3c4097aa688dc231127b4bf9b7451f22c4855eb2afaa7b46db6cf9c4c653b1ff"
                        .to_string(),
                },
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_campaigns(None, None, None, |result| {
            let campaigns = result.unwrap().campaigns;
            println!("{:?}", campaigns);
            let filtered_campaigns: Vec<_> = campaigns
                .into_iter()
                .filter(|c| c.id != *campaign_id_1.borrow())
                .collect();
            *campaign_id_2.borrow_mut() = filtered_campaigns[0].id.clone();
        });

    println!("campaign_id_1: {:?}", campaign_id_1.borrow());
    println!("campaign_id_2: {:?}", campaign_id_2.borrow());

    suite
        .claim(
            alice,
            &campaign_id_1.borrow(),
            Uint128::new(10_000u128),
            None,
            vec![
                "612dd1c48419ca3dcc9a3d8ee87b769544f4c9542faebd5a61810bbbb01b62eb".to_string(),
                "9c47eb9c03236dd26e132cb10c615055a8fb21c153a3fcb42726c9f339cf7f4d".to_string(),
                "f38693e403da2d14b81c246dc040c5dbafaffad7d3a45fbb7ae401b868e2d7a8".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                println!("{:?}", result);
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::CliffPeriodNotPassed { .. } => {}
                    _ => panic!(
                        "Wrong error type, should return ContractError::CliffPeriodNotPassed"
                    ),
                }
            },
        )
        .claim(
            alice,
            &campaign_id_2.borrow(),
            Uint128::new(10_000u128),
            None,
            vec![
                "7fb7cee0673f366d2f73790d2f6c89b6a9116fdf20a5359407990b2c024574ab".to_string(),
                "884c810285f8b0f09322d493e0bd69e65daad31206b3bbd2ac1eef5dbbe8d2dc".to_string(),
                "e741afbec3d922f5dd23e2d30879fec675ee41d73cc3bbe5b9eaf72fbd1941fb".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                match err {
                    ContractError::CliffPeriodNotPassed { .. } => {}
                    _ => panic!(
                        "Wrong error type, should return ContractError::CliffPeriodNotPassed"
                    ),
                }
            },
        );

    // move a few days to pass the cliff of campaign 2

    for _ in 0..7 {
        suite.add_day();
    }

    suite
        .claim(
            alice,
            &campaign_id_2.borrow(),
            Uint128::new(10_000u128),
            None,
            vec![
                "7fb7cee0673f366d2f73790d2f6c89b6a9116fdf20a5359407990b2c024574ab".to_string(),
                "884c810285f8b0f09322d493e0bd69e65daad31206b3bbd2ac1eef5dbbe8d2dc".to_string(),
                "e741afbec3d922f5dd23e2d30879fec675ee41d73cc3bbe5b9eaf72fbd1941fb".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_rewards(
            &campaign_id_2.borrow(),
            Uint128::new(10_000u128),
            alice.to_string(),
            vec![
                "7fb7cee0673f366d2f73790d2f6c89b6a9116fdf20a5359407990b2c024574ab".to_string(),
                "884c810285f8b0f09322d493e0bd69e65daad31206b3bbd2ac1eef5dbbe8d2dc".to_string(),
                "e741afbec3d922f5dd23e2d30879fec675ee41d73cc3bbe5b9eaf72fbd1941fb".to_string(),
            ],
            |result| {
                assert_eq!(
                    result.unwrap(),
                    RewardsResponse {
                        claimed: coins(1_000u128 + 2_099u128 /*7 * 9_000u128 / 30*/, "uom"),
                        pending: coins(10_000u128 - 3_099u128 /*7 * 9_000u128 / 30*/, "uom"),
                        available_to_claim: vec![],
                    }
                );
            },
        );

    // move 23 more days to pass the end time of campaign 2
    for _ in 0..23 {
        suite.add_day();
    }

    suite
        .query_rewards(
            &campaign_id_2.borrow(),
            Uint128::new(10_000u128),
            alice.to_string(),
            vec![
                "7fb7cee0673f366d2f73790d2f6c89b6a9116fdf20a5359407990b2c024574ab".to_string(),
                "884c810285f8b0f09322d493e0bd69e65daad31206b3bbd2ac1eef5dbbe8d2dc".to_string(),
                "e741afbec3d922f5dd23e2d30879fec675ee41d73cc3bbe5b9eaf72fbd1941fb".to_string(),
            ],
            |result| {
                assert_eq!(
                    result.unwrap(),
                    RewardsResponse {
                        claimed: coins(1_000u128 + 2_099u128 /*7 * 9_000u128 / 30*/, "uom"),
                        pending: coins(10_000u128 - 3_099u128 /*7 * 9_000u128 / 30*/, "uom"),
                        available_to_claim: coins(10_000u128 - 3_099u128, "uom"),
                    }
                );
            },
        )
        .claim(
            alice,
            &campaign_id_2.borrow(),
            Uint128::new(10_000u128),
            None,
            vec![
                "7fb7cee0673f366d2f73790d2f6c89b6a9116fdf20a5359407990b2c024574ab".to_string(),
                "884c810285f8b0f09322d493e0bd69e65daad31206b3bbd2ac1eef5dbbe8d2dc".to_string(),
                "e741afbec3d922f5dd23e2d30879fec675ee41d73cc3bbe5b9eaf72fbd1941fb".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_rewards(
            &campaign_id_2.borrow(),
            Uint128::new(10_000u128),
            alice.to_string(),
            vec![
                "7fb7cee0673f366d2f73790d2f6c89b6a9116fdf20a5359407990b2c024574ab".to_string(),
                "884c810285f8b0f09322d493e0bd69e65daad31206b3bbd2ac1eef5dbbe8d2dc".to_string(),
                "e741afbec3d922f5dd23e2d30879fec675ee41d73cc3bbe5b9eaf72fbd1941fb".to_string(),
            ],
            |result| {
                assert_eq!(
                    result.unwrap(),
                    RewardsResponse {
                        claimed: coins(10_000u128, "uom"),
                        pending: vec![],
                        available_to_claim: vec![],
                    }
                );
            },
        )
        .query_campaigns(
            Some(CampaignFilter::CampaignId(
                campaign_id_2.borrow().to_string(),
            )),
            None,
            None,
            {
                |result| {
                    let response = result.unwrap();
                    assert_eq!(response.campaigns.len(), 1);
                    assert_eq!(response.campaigns[0].claimed, coin(10_000u128, "uom"));
                }
            },
        );

    // move the remaining of the year - 1 day, 341 - 30 - 1 days
    for _ in 0..334 {
        suite.add_day();
    }

    suite.claim(
        alice,
        &campaign_id_1.borrow(),
        Uint128::new(10_000u128),
        None,
        vec![
            "612dd1c48419ca3dcc9a3d8ee87b769544f4c9542faebd5a61810bbbb01b62eb".to_string(),
            "9c47eb9c03236dd26e132cb10c615055a8fb21c153a3fcb42726c9f339cf7f4d".to_string(),
            "f38693e403da2d14b81c246dc040c5dbafaffad7d3a45fbb7ae401b868e2d7a8".to_string(),
        ],
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
        &campaign_id_1.borrow(),
        Uint128::new(10_000u128),
        None,
        vec![
            "612dd1c48419ca3dcc9a3d8ee87b769544f4c9542faebd5a61810bbbb01b62eb".to_string(),
            "9c47eb9c03236dd26e132cb10c615055a8fb21c153a3fcb42726c9f339cf7f4d".to_string(),
            "f38693e403da2d14b81c246dc040c5dbafaffad7d3a45fbb7ae401b868e2d7a8".to_string(),
        ],
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    suite
        .query_campaigns(
            Some(CampaignFilter::CampaignId(
                campaign_id_1.borrow().to_string(),
            )),
            None,
            None,
            {
                |result| {
                    println!("{:?}", result);
                    assert_eq!(
                        result.unwrap().campaigns[0].claimed,
                        coin(10_000 / 4, "uom")
                    );
                }
            },
        )
        .query_rewards(
            &campaign_id_1.borrow(),
            Uint128::new(10_000u128),
            alice.to_string(),
            vec![
                "612dd1c48419ca3dcc9a3d8ee87b769544f4c9542faebd5a61810bbbb01b62eb".to_string(),
                "9c47eb9c03236dd26e132cb10c615055a8fb21c153a3fcb42726c9f339cf7f4d".to_string(),
                "f38693e403da2d14b81c246dc040c5dbafaffad7d3a45fbb7ae401b868e2d7a8".to_string(),
            ],
            |result| {
                assert_eq!(
                    result.unwrap(),
                    RewardsResponse {
                        claimed: coins(10_000 / 4, "uom"),
                        pending: coins(10_000u128 - (10_000 / 4), "uom"),
                        available_to_claim: vec![],
                    }
                );
            },
        );

    // advance another year
    for _ in 0..365 {
        suite.add_day();
    }

    suite
        .claim(
            alice,
            &campaign_id_1.borrow(),
            Uint128::new(10_000u128),
            None,
            vec![
                "612dd1c48419ca3dcc9a3d8ee87b769544f4c9542faebd5a61810bbbb01b62eb".to_string(),
                "9c47eb9c03236dd26e132cb10c615055a8fb21c153a3fcb42726c9f339cf7f4d".to_string(),
                "f38693e403da2d14b81c246dc040c5dbafaffad7d3a45fbb7ae401b868e2d7a8".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_campaigns(
            Some(CampaignFilter::CampaignId(
                campaign_id_1.borrow().to_string(),
            )),
            None,
            None,
            {
                |result| {
                    println!("{:?}", result);
                    assert_eq!(
                        result.unwrap().campaigns[0].claimed,
                        coin((10_000 / 4) * 2, "uom")
                    );
                }
            },
        );

    // advance two more years, so the vesting period should be over
    for _ in 0..730 {
        suite.add_day();
    }

    suite
        .query_rewards(
            &campaign_id_1.borrow(),
            Uint128::new(10_000u128),
            alice.to_string(),
            vec![
                "612dd1c48419ca3dcc9a3d8ee87b769544f4c9542faebd5a61810bbbb01b62eb".to_string(),
                "9c47eb9c03236dd26e132cb10c615055a8fb21c153a3fcb42726c9f339cf7f4d".to_string(),
                "f38693e403da2d14b81c246dc040c5dbafaffad7d3a45fbb7ae401b868e2d7a8".to_string(),
            ],
            |result| {
                assert_eq!(
                    result.unwrap(),
                    RewardsResponse {
                        claimed: coins((10_000 / 4) * 2, "uom"),
                        pending: coins(10_000u128 - ((10_000 / 4) * 2), "uom"),
                        available_to_claim: coins((10_000 / 4) * 2, "uom"),
                    }
                );
            },
        )
        .claim(
            alice,
            &campaign_id_1.borrow(),
            Uint128::new(10_000u128),
            None,
            vec![
                "612dd1c48419ca3dcc9a3d8ee87b769544f4c9542faebd5a61810bbbb01b62eb".to_string(),
                "9c47eb9c03236dd26e132cb10c615055a8fb21c153a3fcb42726c9f339cf7f4d".to_string(),
                "f38693e403da2d14b81c246dc040c5dbafaffad7d3a45fbb7ae401b868e2d7a8".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();

                println!(">>>>> add a week and claim");
            },
        )
        .query_campaigns(
            Some(CampaignFilter::CampaignId(
                campaign_id_1.borrow().to_string(),
            )),
            None,
            None,
            {
                |result| {
                    println!("{:?}", result);
                    assert_eq!(
                        result.unwrap().campaigns[0].claimed,
                        coin(10_000u128, "uom")
                    );
                }
            },
        )
        .query_rewards(
            &campaign_id_1.borrow(),
            Uint128::new(10_000u128),
            alice.to_string(),
            vec![
                "612dd1c48419ca3dcc9a3d8ee87b769544f4c9542faebd5a61810bbbb01b62eb".to_string(),
                "9c47eb9c03236dd26e132cb10c615055a8fb21c153a3fcb42726c9f339cf7f4d".to_string(),
                "f38693e403da2d14b81c246dc040c5dbafaffad7d3a45fbb7ae401b868e2d7a8".to_string(),
            ],
            |result| {
                assert_eq!(
                    result.unwrap(),
                    RewardsResponse {
                        claimed: coins(10_000u128, "uom"),
                        pending: vec![],
                        available_to_claim: vec![],
                    }
                );
            },
        );
}

#[test]
fn topup_campaigns_with_cliff() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);

    let alice = &suite.senders[0].clone();
    let current_time = &suite.get_time();

    suite
        .instantiate_airdrop_manager(None)
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
                    owner: None,
                    salt: "gm7ki1rn5wo4x5105j8uclbh99s9db7v".to_string(),
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop without cliff".to_string(),
                    // it will be 100_000 in total after topping up
                    reward_asset: coin(10_000, "uom"),
                    distribution_type: vec![DistributionType::LinearVesting {
                        percentage: Decimal::percent(100),
                        start_time: current_time.seconds(),
                        end_time: current_time.plus_days(1460).seconds(), // 4 years
                    }],
                    cliff_duration: None,
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(1460).seconds(),
                    merkle_root: "a79197d1f2f90797e65c545a99662630da89baf820af47b0044cd910339cb4fd"
                        .to_string(),
                },
            },
            &coins(10_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
                    owner: None,
                    salt: "gm7ki1rn5wo4x5105j8uclbh99s9db7v".to_string(),
                    name: "Test Airdrop II".to_string(),
                    description: "This is an airdrop with cliff".to_string(),
                    // it will be 100_000 in total after topping up
                    reward_asset: coin(10_000, "uom"),
                    distribution_type: vec![
                        DistributionType::LumpSum {
                            percentage: Decimal::percent(10),
                            start_time: current_time.seconds(),
                            end_time: current_time.plus_days(30).seconds(), // 1 month
                        },
                        DistributionType::LinearVesting {
                            percentage: Decimal::percent(90),
                            start_time: current_time.seconds(),
                            end_time: current_time.plus_days(30).seconds(), // 1 month
                        },
                    ],
                    cliff_duration: Some(86_400 * 7), // 7 days cliff
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(30).seconds(),
                    merkle_root: "158f7d8f16fb97d0cdc2e04abb304035dc94dff5b9adcb45930539302367e9da"
                        .to_string(),
                },
            },
            &coins(10_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );

    suite.query_campaigns(None, None, None, {
        |result| {
            let response = result.unwrap();
            assert_eq!(response.campaigns.len(), 2);

            assert_eq!(
                response.campaigns[0].id,
                "c10f8f00b23b5f734b0e247387104781f0e39f8d120d8a2902c8acb73f913eaa"
            );
            assert_eq!(response.campaigns[0].reward_asset, coin(10_000, "uom"));
        }
    });

    for _ in 0..7 {
        suite.add_day();
    }

    // topup campaign

    suite
        .manage_campaign(
            alice,
            CampaignAction::TopUpCampaign {
                campaign_id: "c10f8f00b23b5f734b0e247387104781f0e39f8d120d8a2902c8acb73f913eaa"
                    .to_string(),
            },
            &coins(90_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_campaigns(
            Some(CampaignFilter::CampaignId(
                "c10f8f00b23b5f734b0e247387104781f0e39f8d120d8a2902c8acb73f913eaa".to_string(),
            )),
            None,
            None,
            {
                |result| {
                    let response = result.unwrap();
                    assert_eq!(response.campaigns[0].reward_asset, coin(100_000, "uom"));
                }
            },
        );

    // go to the time when the campaign already finished. Should fail topping up
    for _ in 0..23 {
        suite.add_day();
    }

    suite.manage_campaign(
        alice,
        CampaignAction::TopUpCampaign {
            campaign_id: "c10f8f00b23b5f734b0e247387104781f0e39f8d120d8a2902c8acb73f913eaa"
                .to_string(),
        },
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
}

#[test]
fn query_rewards() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);

    let alice = &suite.senders[0].clone();
    let current_time = &suite.get_time();

    suite.instantiate_airdrop_manager(None).manage_campaign(
        alice,
        CampaignAction::CreateCampaign {
            params: CampaignParams {
                owner: None,
                salt: "zkzv117igbvuwqk12a68kx2zj823v7rg".to_string(),
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
                merkle_root: "a2835f4d5c3b7f9f58ec60e85a52e6e24985777540a214ee4080431bacf4882a"
                    .to_string(),
            },
        },
        &coins(100_000, "uom"),
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    suite
        .claim(
            alice,
            "8032e08c50569671b6c2a6b782f2622e761f421f753a29d34c50668fb1b18880",
            Uint128::new(10_000u128),
            None,
            vec![
                "a761a28228a04d0052587cb11a3a380644545aca9b4953ccb386ac307d458cb2".to_string(),
                "f8fe416e0991cb111c9282cd9b31fff4d298a038b2852666e70b5abc25a3b32c".to_string(),
                "f4474b4e3cb6bcc1996bd91287efd372dad310250953ddfea64f0d552ab8cd2f".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_rewards(
            "8032e08c50569671b6c2a6b782f2622e761f421f753a29d34c50668fb1b18880",
            Uint128::new(35_000u128),
            alice.to_string(),
            vec![
                "a761a28228a04d0052587cb11a3a380644545aca9b4953ccb386ac307d458cb2".to_string(),
                "f8fe416e0991cb111c9282cd9b31fff4d298a038b2852666e70b5abc25a3b32c".to_string(),
                "f4474b4e3cb6bcc1996bd91287efd372dad310250953ddfea64f0d552ab8cd2f".to_string(),
            ],
            |result| {
                let err = result.unwrap_err().to_string();

                assert_eq!(
                    err,
                    "Generic error: Querier contract error: Merkle root verification failed"
                );
            },
        )
        .query_rewards(
            "8032e08c50569671b6c2a6b782f2622e761f421f753a29d34c50668fb1b18880",
            Uint128::new(20_000u128),
            alice.to_string(),
            vec![
                "a761a28228a04d0052587cb11a3a380644545aca9b4953ccb386ac307d458cb2".to_string(),
                "f8fe416e0991cb111c9282cd9b31fff4d298a038b2852666e70b5abc25a3b32c".to_string(),
                "f4474b4e3cb6bcc1996bd91287efd372dad310250953ddfea64f0d552ab8cd2f".to_string(),
            ],
            |result| {
                let err = result.unwrap_err().to_string();

                assert_eq!(
                    err,
                    "Generic error: Querier contract error: Merkle root verification failed"
                );
            },
        )
        .query_rewards(
            "8032e08c50569671b6c2a6b782f2622e761f421f753a29d34c50668fb1b18880",
            Uint128::new(10_000u128),
            alice.to_string(),
            vec![
                "a761a28228a04d0052587cb11a3a380644545aca9b4953ccb386ac307d458cb2".to_string(),
                "f8fe416e0991cb111c9282cd9b31fff4d298a038b2852666e70b5abc25a3b32c".to_string(),
                "f4474b4e3cb6bcc1996bd91287efd372dad310250953ddfea64f0d552ab8cd2f".to_string(),
            ],
            |result| {
                assert_eq!(
                    result.unwrap(),
                    RewardsResponse {
                        claimed: coins(2_500u128, "uom"),
                        pending: coins(10_000u128 - 2_500u128, "uom"),
                        available_to_claim: vec![],
                    }
                );
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::EndCampaign {
                campaign_id: "8032e08c50569671b6c2a6b782f2622e761f421f753a29d34c50668fb1b18880"
                    .to_string(),
            },
            &[],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_rewards(
            "8032e08c50569671b6c2a6b782f2622e761f421f753a29d34c50668fb1b18880",
            Uint128::new(10_000u128),
            alice.to_string(),
            vec![
                "a761a28228a04d0052587cb11a3a380644545aca9b4953ccb386ac307d458cb2".to_string(),
                "f8fe416e0991cb111c9282cd9b31fff4d298a038b2852666e70b5abc25a3b32c".to_string(),
                "f4474b4e3cb6bcc1996bd91287efd372dad310250953ddfea64f0d552ab8cd2f".to_string(),
            ],
            |result| {
                assert_eq!(
                    result.unwrap(),
                    RewardsResponse {
                        claimed: coins(2_500u128, "uom"),
                        pending: vec![],
                        available_to_claim: vec![],
                    }
                );
            },
        );
}

#[test]
fn end_campaigns() {
    let mut suite = TestingSuite::default_with_balances(vec![coin(1_000_000_000, "uom")]);

    let alice = &suite.senders[0].clone();
    let bob = &suite.senders[1].clone();
    let dan = &suite.senders[3].clone();
    let current_time = &suite.get_time();

    suite
        .instantiate_airdrop_manager(None)
        .manage_campaign(
            dan,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
                    owner: None,
                    salt: "zkzv117igbvuwqk12a68kx2zj823v7rg".to_string(),
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
                    merkle_root: "fcd009d9c1fdeb7016c6c1093a225111dcc1190f57f9c49e10798a681b943ed7"
                        .to_string(),
                },
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .manage_campaign(
            dan,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
                    owner: None,
                    salt: "zkzv117igbvuwqk12a68kx2zj823v7rg".to_string(),
                    name: "Test Airdrop II".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(50_000, "uom"),
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
                    merkle_root: "8e867c026989b53d6c2dfacce4ec213a12377aa039eb36ebb9c955e89f4f25b0"
                        .to_string(),
                },
            },
            &coins(50_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                print!("{:?}", result);
                result.unwrap();
            },
        )
        .query_campaigns(None, None, None, {
            |result| {
                assert_eq!(result.unwrap().campaigns.len(), 2);
            }
        });

    let campaign_1_id =
        "cc94d804b854f38dc73639eacd42cae9cff4081c1aececefa9248f2587b403e6".to_string();
    let campaign_2_id =
        "0b9cb6c64fdb2ed4e0b8d496988c889466d1bd88f7794f2efe3b3170b1607b05".to_string();

    //end campaign
    suite
        .manage_campaign(
            bob,
            CampaignAction::EndCampaign {
                campaign_id: campaign_1_id.clone(),
            },
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
            bob,
            CampaignAction::EndCampaign {
                campaign_id: campaign_2_id.clone(),
            },
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
            dan,
            CampaignAction::EndCampaign {
                campaign_id: campaign_1_id.clone(),
            },
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
            dan,
            CampaignAction::EndCampaign {
                campaign_id: campaign_1_id.clone(),
            },
            &[],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .manage_campaign(
            alice, //alice can end the campaign since it's the owner of the contract
            CampaignAction::EndCampaign {
                campaign_id: campaign_2_id.clone(),
            },
            &[],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );
}

#[test]
fn query_campaigns() {
    let mut suite = TestingSuite::default_with_balances(vec![coin(1_000_000_000, "uom")]);

    let alice = &suite.senders[0].clone();
    let dan = &suite.senders[3].clone();
    let current_time = &suite.get_time();

    suite.instantiate_airdrop_manager(None);

    for i in 0..100 {
        suite.manage_campaign(
            dan,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
                    owner: None,
                    salt: "zkzv117igbvuwqk12a68kx2zj823v7rg".to_string(),
                    name: format!("Test Airdrop {i}"),
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
                    merkle_root: "a79197d1f2f90797e65c545a99662630da89baf820af47b0044cd910339cb4fd"
                        .to_string(),
                },
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                println!("i: {}", i);
                println!("{:?}", result);
                println!("---");
                result.unwrap();
            },
        );
    }
    //todo fix
    // suite
    //     .query_campaigns(
    //         Some(CampaignFilter::Owner(alice.to_string())),
    //         None,
    //         None,
    //         {
    //             |result| {
    //                 let response = result.unwrap();
    //
    //                 assert!(response.campaigns.is_empty());
    //             }
    //         },
    //     )
    //     .query_campaigns(
    //         Some(CampaignFilter::CampaignId("0819b6bc079397161bb78ec4029235713a8df8d80a85a83b383fba8b4e43fe7f".to_string())),
    //         None,
    //         None,
    //         {
    //             |result| {
    //                 let response = result.unwrap();
    //
    //                 assert_eq!(response.campaigns.len(), 1);
    //                 assert_eq!(response.campaigns[0].name, "Test Airdrop 0");
    //             }
    //         },
    //     )
    //     .query_campaigns(
    //         Some(CampaignFilter::CampaignId("2b6f4e5a107d9390a945596e8ec05b1f2feae892818fa8ddddc798608749f515".to_string())),
    //         None,
    //         None,
    //         {
    //             |result| {
    //                 let response = result.unwrap();
    //
    //                 assert_eq!(response.campaigns.len(), 1);
    //                 assert_eq!(response.campaigns[0].name, "Test Airdrop 79");
    //             }
    //         },
    //     )
    //     .query_campaigns(
    //         Some(CampaignFilter::Owner(dan.to_string())),
    //         None,
    //         None,
    //         |result| {
    //             let response = result.unwrap();
    //
    //             println!(">>>>> {:?}", response);
    //
    //             assert_eq!(response.campaigns.len(), 50);
    //             assert_eq!(response.campaigns.last().unwrap().id, "795e9e6ddba229cbdadd8a5d52dbbac939d28aca76c69aab94a1fec5b7c5384d".to_string());
    //             assert_eq!(response.campaigns.first().unwrap().id, "fd708985acda0acce96d23a0c66d32f6ff5a28315a69635404cbee5ec557c20a".to_string());
    //         },
    //     )
    //     .query_campaigns(None, Some("22533927e5b061fa142edfc93e2b356c193086a34b35995d0f52bc40564e311f".to_string()), None, |result| {
    //         let response = result.unwrap();
    //
    //         assert_eq!(response.campaigns.len(), 10);
    //         assert_eq!(response.campaigns.last().unwrap().id, "c85fcaeac36628fc3d4283ca7910714a247ab4cef817208887ac0534313c79f7".to_string());
    //         assert_eq!(response.campaigns.first().unwrap().id, "9b3dd22d26a4bfed3c39c30401d8d70f19b1dab110fdf91b4055178044e56b06".to_string());
    //     })
    //     .query_campaigns(None, Some("22533927e5b061fa142edfc93e2b356c193086a34b35995d0f52bc40564e311f".to_string()), Some(30u8), |result| {
    //         let response = result.unwrap();
    //
    //         assert_eq!(response.campaigns.len(), 30);
    //         assert_eq!(response.campaigns.last().unwrap().id, "48a96fa284ad4c157ca66fba1538fcefeb69cbba99a1a8aedf1108a7f567a3ec".to_string());
    //         assert_eq!(response.campaigns.first().unwrap().id, "9b3dd22d26a4bfed3c39c30401d8d70f19b1dab110fdf91b4055178044e56b06".to_string());
    //     });
}

use std::str::FromStr;

use cosmwasm_std::{coin, coins, Decimal, Uint128};
use cw_multi_test::AppResponse;

use airdrop_manager::error::ContractError;
use airdrop_manager::msg::{CampaignAction, CampaignParams, DistributionType};

use crate::suite::TestingSuite;

mod suite;

#[test]
fn instantiate_airdrop_manager() {
    let mut suite = TestingSuite::default_with_balances(vec![coin(1_000_000_000, "uom")]);

    let alice = &suite.senders[0].clone();
    let current_time = &suite.get_time();

    suite.instantiate_airdrop_manager(None);
}

#[test]
fn create_campaign_and_claim() {
    let mut suite = TestingSuite::default_with_balances(vec![
        coin(1_000_000_000, "uom"),
        coin(1_000_000_000, "uusdc"),
    ]);

    let alice = &suite.senders[0].clone();
    let current_time = &suite.get_time();

    suite
        .instantiate_airdrop_manager(None)
        .query_campaigns(None, None, None, {
            |result| {
                assert_eq!(result.unwrap().campaigns.len(), 0);
            }
        })
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
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
                    merkle_root: "3bbbd2c479fc54a483b3417a25417d2b71dc11a60b32d014ccfaccc8d878ce60"
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
                merkle_root: "3bbbd2c479fc54a483b3417a25417d2b71dc11a60b32d014ccfaccc8d878ce60"
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
                    merkle_root: "3bbbd2c479fc54a483b3417a25417d2b71dc11a60b32d014ccfaccc8d878ce60"
                        .to_string(),
                },
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
                params: CampaignParams {
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
                    merkle_root: "3bbbd2c479fc54a483b3417a25417d2b71dc11a60b32d014ccfaccc8d878ce60"
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
                    merkle_root: "3bbbd2c479fc54a483b3417a25417d2b71dc11a60b32d014ccfaccc8d878ce60"
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
                    name: "Test Airdrop I".to_string(),
                    description: "This is an airdrop, 土金, ك".to_string(),
                    reward_asset: coin(100_000, "uom"),
                    distribution_type: vec![],
                    cliff_duration: None,
                    start_time: current_time.seconds() + 1,
                    end_time: current_time.seconds() + 172_800,
                    merkle_root: "3bbbd2c479fc54a483b3417a25417d2b71dc11a60b32d014ccfaccc8d878ce60"
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
                    merkle_root: "3bbbd2c479fc54a483b3417a25417d2b71dc11a60b32d014ccfaccc8d878ce60"
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
                    merkle_root: "3bbbd2c479fc54a483b3417a25417d2b71dc11a60b32d014ccfaccc8d878ce60"
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
                    merkle_root: "3bbbd2c479fc54a483b3417a25417d2b71dc11a60b32d014ccfaccc8d878ce60"
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
                    merkle_root: "3bbbd2c479fc54a483b3417a25417d2b71dc11a60b32d014ccfaccc8d878ce60"
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
                    merkle_root: "3bbbd2c479fc54a483b3417a25417d2b71dc11a60b32d014ccfaccc8d878ce60"
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
                    merkle_root: "3bbbd2c479fc54a483b3417a25417d2b71dc11a60b32d014ccfaccc8d878ce60"
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
                    merkle_root: "3bbbd2c479fc54a48b3417a25417d2b71dc11a60b32d014ccfaccc8d878ce60".to_string(),
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
                    merkle_root: "3bbbd2c479fc54a483b3417a25417d2b71dc11a60b32d014ccfaccc8d878ce60"
                        .to_string(),
                },
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {},
        )


        .query_campaigns(None, None, None, {
            |result| {
                println!("{:?}", result);
                assert_eq!(result.unwrap().campaigns.len(), 1);
            }
        });

    //todo test for failing cases as well
    //claim
    // suite.claim(
    //     alice,
    //     1,
    //     Uint128::new(10_000u128),
    //     vec![
    //         "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
    //         "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
    //         "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
    //     ],
    //     |result: Result<AppResponse, anyhow::Error>| {
    //         // let err = result.unwrap_err().downcast::<ContractError>().unwrap();
    //         //
    //         // match err {
    //         //     ContractError::OwnershipError { .. } => {}
    //         //     _ => panic!("Wrong error type, should return ContractError::OwnershipError"),
    //         // }
    //
    //         println!("{:?}", result);
    //     },
    // );

    suite.add_day();

    suite
        .claim(
            alice,
            1,
            Uint128::new(10_000u128),
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                // let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                //
                // match err {
                //     ContractError::OwnershipError { .. } => {}
                //     _ => panic!("Wrong error type, should return ContractError::OwnershipError"),
                // }

                println!("{:?}", result);
            },
        )
        .query_campaigns(None, None, None, {
            |result| {
                println!("{:?}", result);
                assert_eq!(result.unwrap().campaigns.len(), 1);
            }
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

    suite.instantiate_airdrop_manager(None).manage_campaign(
        alice,
        CampaignAction::CreateCampaign {
            params: CampaignParams {
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
                merkle_root: "3bbbd2c479fc54a483b3417a25417d2b71dc11a60b32d014ccfaccc8d878ce60"
                    .to_string(),
            },
        },
        &coins(100_000, "uom"),
        |result: Result<AppResponse, anyhow::Error>| {},
    );

    suite.add_day();

    println!(">>>>> LumpSum claiming");
    suite
        .claim(
            alice,
            1,
            Uint128::new(10_000u128),
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                // let err = result.unwrap_err().downcast::<ContractError>().unwrap();
                //
                // match err {
                //     ContractError::OwnershipError { .. } => {}
                //     _ => panic!("Wrong error type, should return ContractError::OwnershipError"),
                // }

                println!("{:?}", result);
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
        .claim(
            alice,
            1,
            Uint128::new(10_000u128),
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
            1,
            Uint128::new(10_000u128),
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                println!("{:?}", result);
                println!(">>>>> try claiming again without moving time, should err");
            },
        )
        .claim(
            alice,
            1,
            Uint128::new(10_000u128),
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
        .claim(
            alice,
            1,
            Uint128::new(10_000u128),
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                println!("{:?}", result);
                println!(
                    ">>>>> add 2 more weeks and claim, the campaign should have finished by then"
                );
            },
        )
        .add_week()
        .add_week()
        .claim(
            alice,
            1,
            Uint128::new(10_000u128),
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                println!("{:?}", result);
                println!(">>>>> add a day and try claiming again, should err");
            },
        )
        .add_day()
        .claim(
            alice,
            1,
            Uint128::new(10_000u128),
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
        .claim(
            dan,
            1,
            Uint128::new(35_000u128),
            vec![
                "8799448ea6334a9b96f60f63ef2e568be364c340fb1a189262d6d7955bce300b".to_string(),
                "34424a2e4bdc8c8e9c3fb3e4743fbc0abba484737ac49f195100d7b8133cf5be".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
                assert_eq!(response.campaigns[0].claimed, coin(45_000u128, "uom"));
            }
        });
}

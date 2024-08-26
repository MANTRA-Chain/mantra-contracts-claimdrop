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

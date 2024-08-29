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

    suite
        .instantiate_airdrop_manager(None)
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
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_campaigns(None, None, None, {
            |result| {
                assert_eq!(result.unwrap().campaigns.len(), 1);
            }
        });

    // claim
    suite.claim(
        alice,
        1,
        Uint128::new(20_000u128),
        None,
        vec![
            "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
            "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
            "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
            2,
            Uint128::new(10_000u128),
            None,
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
            1,
            // pretending to be entitled to more tokens than the campaign has to offer for this user
            Uint128::new(20_000u128),
            None,
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
            1,
            Uint128::new(10_000u128),
            None,
            // provide wrong proofs
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
            1,
            Uint128::new(10_000u128),
            // try claiming for someone else, with the wrong proofs
            Some(bob.to_string()),
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
            1,
            Uint128::new(10_000u128),
            Some(alice.to_string()),
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
            1,
            Uint128::new(10_000u128),
            None,
            vec!["267bf7a7b8f52ece6b04cbddf77c0d0bbc1fc0544e8f68923f95fdd7b9121316".to_string()],
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
            1,
            Uint128::new(20_000u128),
            None,
            vec![
                "7a012b86f12743c59d9382d2be117e9362ba3210ed53dbdfefaaf556306c6d1e".to_string(),
                "34424a2e4bdc8c8e9c3fb3e4743fbc0abba484737ac49f195100d7b8133cf5be".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_campaigns(None, None, None, {
            |result| {
                println!("{:?}", result);
                assert_eq!(result.unwrap().campaigns.len(), 1);
            }
        });

    // claim
    suite
        .add_day()
        .claim(
            bob,
            1,
            Uint128::new(10_000u128),
            Some(alice.to_string()),
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .claim(
            bob,
            1,
            Uint128::new(10_000u128),
            None,
            vec!["267bf7a7b8f52ece6b04cbddf77c0d0bbc1fc0544e8f68923f95fdd7b9121316".to_string()],
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
            CampaignAction::EndCampaign { campaign_id: 1 },
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
            CampaignAction::EndCampaign { campaign_id: 2 },
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
            CampaignAction::EndCampaign { campaign_id: 1 },
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
            CampaignAction::EndCampaign { campaign_id: 1 },
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
        1,
        Uint128::new(20_000u128),
        None,
        vec![
            "7a012b86f12743c59d9382d2be117e9362ba3210ed53dbdfefaaf556306c6d1e".to_string(),
            "34424a2e4bdc8c8e9c3fb3e4743fbc0abba484737ac49f195100d7b8133cf5be".to_string(),
            "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    suite.add_day();

    println!(">>>>> LumpSum claiming");
    suite
        .claim(
            alice,
            1,
            Uint128::new(10_000u128),
            None,
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
            1,
            Uint128::new(10_000u128),
            alice.to_string(),
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
            1,
            Uint128::new(10_000u128),
            None,
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
            None,
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                println!("{:?}", result);
                result.unwrap();
                println!(">>>>> try claiming again without moving time, should err");
            },
        )
        .claim(
            alice,
            1,
            Uint128::new(10_000u128),
            None,
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
        .query_rewards(
            1,
            Uint128::new(10_000u128),
            alice.to_string(),
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
            1,
            Uint128::new(10_000u128),
            None,
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
            1,
            Uint128::new(10_000u128),
            alice.to_string(),
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
            1,
            Uint128::new(10_000u128),
            None,
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
            None,
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
        .query_rewards(
            1,
            Uint128::new(35_000u128),
            dan.to_string(),
            vec![
                "8799448ea6334a9b96f60f63ef2e568be364c340fb1a189262d6d7955bce300b".to_string(),
                "34424a2e4bdc8c8e9c3fb3e4743fbc0abba484737ac49f195100d7b8133cf5be".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
            1,
            Uint128::new(35_000u128),
            None,
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
        .query_rewards(
            1,
            Uint128::new(35_000u128),
            dan.to_string(),
            vec![
                "8799448ea6334a9b96f60f63ef2e568be364c340fb1a189262d6d7955bce300b".to_string(),
                "34424a2e4bdc8c8e9c3fb3e4743fbc0abba484737ac49f195100d7b8133cf5be".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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

    suite
        .instantiate_airdrop_manager(None)
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
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
                    merkle_root: "3bbbd2c479fc54a483b3417a25417d2b71dc11a60b32d014ccfaccc8d878ce60"
                        .to_string(),
                },
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .manage_campaign(
            alice,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
                    owner: None,
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
                    merkle_root: "158f7d8f16fb97d0cdc2e04abb304035dc94dff5b9adcb45930539302367e9da"
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
            1,
            Uint128::new(10_000u128),
            None,
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
        )
        .claim(
            alice,
            2,
            Uint128::new(10_000u128),
            None,
            vec![
                "301f69b76517f75653649c2d61206c8a8ad885f6733b3a3abe9f3ebfbcf3cb03".to_string(),
                "68c3150317d0d7fa222307b6f3fac98568d6732ad09f363aba90e15e95a77d1d".to_string(),
                "eb51f76124a113a2686d550b459e64248bfe364c280de44364010acf99ba6492".to_string(),
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
            2,
            Uint128::new(10_000u128),
            None,
            vec![
                "301f69b76517f75653649c2d61206c8a8ad885f6733b3a3abe9f3ebfbcf3cb03".to_string(),
                "68c3150317d0d7fa222307b6f3fac98568d6732ad09f363aba90e15e95a77d1d".to_string(),
                "eb51f76124a113a2686d550b459e64248bfe364c280de44364010acf99ba6492".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_rewards(
            2,
            Uint128::new(10_000u128),
            alice.to_string(),
            vec![
                "301f69b76517f75653649c2d61206c8a8ad885f6733b3a3abe9f3ebfbcf3cb03".to_string(),
                "68c3150317d0d7fa222307b6f3fac98568d6732ad09f363aba90e15e95a77d1d".to_string(),
                "eb51f76124a113a2686d550b459e64248bfe364c280de44364010acf99ba6492".to_string(),
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
            2,
            Uint128::new(10_000u128),
            alice.to_string(),
            vec![
                "301f69b76517f75653649c2d61206c8a8ad885f6733b3a3abe9f3ebfbcf3cb03".to_string(),
                "68c3150317d0d7fa222307b6f3fac98568d6732ad09f363aba90e15e95a77d1d".to_string(),
                "eb51f76124a113a2686d550b459e64248bfe364c280de44364010acf99ba6492".to_string(),
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
            2,
            Uint128::new(10_000u128),
            None,
            vec![
                "301f69b76517f75653649c2d61206c8a8ad885f6733b3a3abe9f3ebfbcf3cb03".to_string(),
                "68c3150317d0d7fa222307b6f3fac98568d6732ad09f363aba90e15e95a77d1d".to_string(),
                "eb51f76124a113a2686d550b459e64248bfe364c280de44364010acf99ba6492".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_rewards(
            2,
            Uint128::new(10_000u128),
            alice.to_string(),
            vec![
                "301f69b76517f75653649c2d61206c8a8ad885f6733b3a3abe9f3ebfbcf3cb03".to_string(),
                "68c3150317d0d7fa222307b6f3fac98568d6732ad09f363aba90e15e95a77d1d".to_string(),
                "eb51f76124a113a2686d550b459e64248bfe364c280de44364010acf99ba6492".to_string(),
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
        .query_campaigns(Some(CampaignFilter::CampaignId(2)), None, None, {
            |result| {
                let response = result.unwrap();
                assert_eq!(response.campaigns.len(), 1);
                assert_eq!(response.campaigns[0].claimed, coin(10_000u128, "uom"));
            }
        });

    // move the remaining of the year - 1 day, 341 - 30 - 1 days
    for _ in 0..334 {
        suite.add_day();
    }

    suite.claim(
        alice,
        1,
        Uint128::new(10_000u128),
        None,
        vec![
            "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
            "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
            "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
        1,
        Uint128::new(10_000u128),
        None,
        vec![
            "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
            "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
            "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
        ],
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    suite
        .query_campaigns(Some(CampaignFilter::CampaignId(1)), None, None, {
            |result| {
                println!("{:?}", result);
                assert_eq!(
                    result.unwrap().campaigns[0].claimed,
                    coin(10_000 / 4, "uom")
                );
            }
        })
        .query_rewards(
            1,
            Uint128::new(10_000u128),
            alice.to_string(),
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
            1,
            Uint128::new(10_000u128),
            None,
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_campaigns(Some(CampaignFilter::CampaignId(1)), None, None, {
            |result| {
                println!("{:?}", result);
                assert_eq!(
                    result.unwrap().campaigns[0].claimed,
                    coin((10_000 / 4) * 2, "uom")
                );
            }
        });

    // advance two more years, so the vesting period should be over
    for _ in 0..730 {
        suite.add_day();
    }

    suite
        .query_rewards(
            1,
            Uint128::new(10_000u128),
            alice.to_string(),
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
            1,
            Uint128::new(10_000u128),
            None,
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();

                println!(">>>>> add a week and claim");
            },
        )
        .query_campaigns(Some(CampaignFilter::CampaignId(1)), None, None, {
            |result| {
                println!("{:?}", result);
                assert_eq!(
                    result.unwrap().campaigns[0].claimed,
                    coin(10_000u128, "uom")
                );
            }
        })
        .query_rewards(
            1,
            Uint128::new(10_000u128),
            alice.to_string(),
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    suite
        .claim(
            alice,
            1,
            Uint128::new(10_000u128),
            None,
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
            ],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_rewards(
            1,
            Uint128::new(35_000u128),
            alice.to_string(),
            vec![
                "8799448ea6334a9b96f60f63ef2e568be364c340fb1a189262d6d7955bce300b".to_string(),
                "34424a2e4bdc8c8e9c3fb3e4743fbc0abba484737ac49f195100d7b8133cf5be".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
            1,
            Uint128::new(20_000u128),
            alice.to_string(),
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
            1,
            Uint128::new(10_000u128),
            alice.to_string(),
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
            CampaignAction::EndCampaign { campaign_id: 1 },
            &[],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_rewards(
            1,
            Uint128::new(10_000u128),
            alice.to_string(),
            vec![
                "0fc46dd4b310f23d1020155ba0af2ec432fc7c8d2054dead064b1770ce2a1aee".to_string(),
                "4d30e2a708ec3a01d5fd01118a9fbb22d4f487e0ca11410c24313dfe738d1263".to_string(),
                "af892079af91afa431d8ddadfbc73904876513ed6eb5bcb967e615c178900ccd".to_string(),
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
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .manage_campaign(
            dan,
            CampaignAction::CreateCampaign {
                params: CampaignParams {
                    owner: None,
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
                    merkle_root: "3bbbd2c479fc54a483b3417a25417d2b71dc11a60b32d014ccfaccc8d878ce60"
                        .to_string(),
                },
            },
            &coins(50_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .query_campaigns(None, None, None, {
            |result| {
                assert_eq!(result.unwrap().campaigns.len(), 2);
            }
        });

    //end campaign
    suite
        .manage_campaign(
            bob,
            CampaignAction::EndCampaign { campaign_id: 1 },
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
            CampaignAction::EndCampaign { campaign_id: 2 },
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
            CampaignAction::EndCampaign { campaign_id: 1 },
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
            CampaignAction::EndCampaign { campaign_id: 1 },
            &[],
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        )
        .manage_campaign(
            alice, //alice can end the campaign since it's the owner of the contract
            CampaignAction::EndCampaign { campaign_id: 2 },
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
                    merkle_root: "3bbbd2c479fc54a483b3417a25417d2b71dc11a60b32d014ccfaccc8d878ce60"
                        .to_string(),
                },
            },
            &coins(100_000, "uom"),
            |result: Result<AppResponse, anyhow::Error>| {
                result.unwrap();
            },
        );
    }

    suite
        .query_campaigns(
            Some(CampaignFilter::Owner(alice.to_string())),
            None,
            None,
            {
                |result| {
                    let response = result.unwrap();

                    assert!(response.campaigns.is_empty());
                }
            },
        )
        .query_campaigns(Some(CampaignFilter::CampaignId(1)), None, None, {
            |result| {
                let response = result.unwrap();

                assert_eq!(response.campaigns.len(), 1);
                assert_eq!(response.campaigns[0].name, "Test Airdrop 0");
            }
        })
        .query_campaigns(Some(CampaignFilter::CampaignId(80)), None, None, {
            |result| {
                let response = result.unwrap();

                assert_eq!(response.campaigns.len(), 1);
                assert_eq!(response.campaigns[0].name, "Test Airdrop 79");
            }
        })
        .query_campaigns(Some(CampaignFilter::Owner(dan.to_string())), None, None, {
            |result| {
                let response = result.unwrap();

                assert_eq!(response.campaigns.len(), 50);
                assert_eq!(response.campaigns.last().unwrap().id, 51);
                assert_eq!(response.campaigns.first().unwrap().id, 100);
            }
        })
        .query_campaigns(None, Some(20), None, {
            |result| {
                let response = result.unwrap();

                assert_eq!(response.campaigns.len(), 10);
                assert_eq!(response.campaigns.first().unwrap().id, 100);
                assert_eq!(response.campaigns.last().unwrap().id, 91);
            }
        })
        .query_campaigns(None, Some(20u64), Some(30u8), {
            |result| {
                let response = result.unwrap();

                assert_eq!(response.campaigns.len(), 30);
                assert_eq!(response.campaigns.first().unwrap().id, 100);
                assert_eq!(response.campaigns.last().unwrap().id, 71);
            }
        });
}

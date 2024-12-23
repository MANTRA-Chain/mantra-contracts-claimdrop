use crate::hashes::{ALICE_PROOFS_X, MERKLE_ROOT_X};
use crate::suite::TestingSuite;
use claimdrop_contract::msg::{CampaignAction, CampaignParams, DistributionType, RewardsResponse};
use cosmwasm_std::{coin, coins, Decimal, Uint128};
use cw_multi_test::AppResponse;

mod hashes;
mod suite;

#[test]
fn can_claim_dust_after_vesting_ends() {
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
                        cliff_duration: None,
                    }],
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(90).seconds(),
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

    // This will make it 60 days, so the vesting will fully end, while the campaign is about to end
    // in 30 days.
    suite.add_day();

    // executing the claiming here, will result on the compute_claimable_amount::new_claims being empty,
    // as the claim_amount will be zero, while the rounding_error_compensation_amount will be 1.
    suite
        .query_rewards(Uint128::new(17u128), alice, ALICE_PROOFS_X, |result| {
            assert_eq!(
                result.unwrap(),
                RewardsResponse {
                    claimed: coins(16u128, "uom"),
                    pending: coins(17u128 - 16u128, "uom"),
                    available_to_claim: coins(17u128 - 16u128, "uom"),
                }
            );
        })
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
fn can_claim_dust_after_vesting_ends_2() {
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
                    distribution_type: vec![
                        DistributionType::LumpSum {
                            percentage: Decimal::percent(25),
                            start_time: current_time.seconds(),
                        },
                        DistributionType::LinearVesting {
                            percentage: Decimal::percent(75),
                            start_time: current_time.seconds(),
                            end_time: current_time.plus_days(60).seconds(),
                            cliff_duration: None,
                        },
                    ],
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(90).seconds(),
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

    for _ in 0..30 {
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
                (alice.to_string(), coin(10u128, "uom"))
            );
        });

    // This will make it 60 days, so the vesting will fully end, while the campaign is about to end
    // in 30 days.
    for _ in 0..30 {
        suite.add_day();
    }

    // executing the claiming here, will result on the compute_claimable_amount::new_claims being empty,
    // as the claim_amount will be zero, while the rounding_error_compensation_amount will be 1.
    suite
        .query_rewards(Uint128::new(17u128), alice, ALICE_PROOFS_X, |result| {
            assert_eq!(
                result.unwrap(),
                RewardsResponse {
                    claimed: coins(10u128, "uom"),
                    pending: coins(17u128 - 10u128, "uom"),
                    available_to_claim: coins(17u128 - 10u128, "uom"),
                }
            );
        })
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

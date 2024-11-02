use cosmwasm_std::{coin, coins, Addr, Decimal, Uint128};
use cw_multi_test::AppResponse;

use claimdrop_contract::msg::{CampaignAction, CampaignParams, DistributionType, RewardsResponse};

use crate::suite::TestingSuite;

mod hashes;
mod suite;

/*
This test is built to confirm a potential issue with the contract when using a denom with a exponent of 18.
The numbers used in this test are based on a small real-world scenario where the contract was not returning the correct rewards.
The contract will need to handle at least 999_000_000 * 10^18 to be able to handle the rewards in the real-world scenario.
Subsequent pools have investment in the 10's of millions of dollars, so the contract will need to handle these.
We should build to expect the contract to handle at least 999_000_000 * 10^18 preferably more.
 */

#[test]
fn bug_large_numbers() {
    let alice = Addr::unchecked("mantra13qtg0gys4lfxccjeqed3vrdgmp7g5kzcmf7kjm");
    let denom = "factory/mantra1ady9vl53r6ct6kxklhgxvtnscmszryl8nnzule/ausdy";

    let amount: u128 = 48138819536000000000000;
    let alice_amount: u128 = 2040555238000000000000;
    let merkle_root = "dc55a9806e62d0ae7f076e86f90c043efcd4d190202e6754f31118ace04c8ea7";
    let alice_proofs: &[&str] = &[
        "63cf88f63bd892d8f7244f1f1407ad7d02337bad1ee763486db7ca312e9d993f",
        "9365d584a75c90eb2d660759d5297cee5a1cafcae6f9fb5d94f9f68e41f0c917",
        "f7f4f5e06bd4d45579c2486d40ee450b21f220916575dd4408d89749a2abacca",
        "eb446cdba0f59a006d9326d951007464ea325bde0a3bd67359ff0abb47a01c93",
        "c1d990418390cb2b54f2383f44f189b3285613bc733fdf761992bade9b6532c5",
        "9e53a994f997c4da931d0d02c4bbf7e6652045bfb02c44b411bedf226ce3bbf7",
        "ff19c410f3b2d812b01544ed7cb72da34e0e174c519d033620991ce750b14993",
        "7de8d351e051243b44ab450e716613d41c80faaf4eb227034894c89e1b9f3a41",
        "43206510a1a9306756c31b7976d5546e182079f651b8389c9839733a5a93483e",
    ];
    let mut suite = TestingSuite::default_with_balances(vec![coin(amount, denom)]);

    let owner = &suite.senders[0].clone();

    let current_time = &suite.get_time();

    suite.instantiate_claimdrop_contract(None).manage_campaign(
        owner,
        CampaignAction::CreateCampaign {
            params: Box::new(CampaignParams {
                owner: None,
                name: "Test Airdrop I".to_string(),
                description: "This is an airdrop with no cliff".to_string(),
                reward_asset: coin(amount, denom),
                distribution_type: vec![DistributionType::LumpSum {
                    percentage: Decimal::percent(100),
                    start_time: current_time.seconds(),
                    end_time: current_time.plus_days(7).seconds(),
                }],
                cliff_duration: None,
                start_time: current_time.seconds(),
                end_time: current_time.plus_days(14).seconds(),
                merkle_root: merkle_root.to_string(),
            }),
        },
        &coins(amount, denom),
        |result: Result<AppResponse, anyhow::Error>| {
            result.unwrap();
        },
    );

    suite.query_rewards(Uint128::new(alice_amount), &alice, alice_proofs, |result| {
        assert_eq!(
            result.unwrap(),
            RewardsResponse {
                claimed: coins(0, denom),
                pending: coins(10_000u128 - 2_500u128, denom),
                available_to_claim: vec![],
            }
        );
    });
}

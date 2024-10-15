use airdrop_manager::helpers::compute_campaign_id;

#[test]
fn test_compute_campaign_id() {
    // assumes campaign_name, campaign_description, start_time, owner, and salt are all correctly set.
    // previous validation is done before compute_campaign_id is called to when creating a campaign

    let campaign_name = "Airdrop Campaign";
    let campaign_description = "This is a description for the campaign";
    let start_time = "1727924265";
    let owner = "mantra17kwgn5u7we5pmtf5890zf47p6l4jpsljl426sk";
    let salt = "zkzv117igbvuwqk12a68kx2zj823v7rg";

    let campaign_id_result =
        compute_campaign_id(campaign_name, campaign_description, start_time, owner, salt);

    assert!(campaign_id_result.is_ok());
    assert_eq!(
        campaign_id_result.unwrap(),
        "421690fe878ce73fb69e5e964c91aec1dcfea2ec3451f9bc9b5bb80505c73d2f"
    );
}

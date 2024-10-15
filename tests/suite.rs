use cosmwasm_std::{coin, Addr, Coin, Empty, StdResult, Timestamp, Uint128};
use cw_multi_test::{
    App, AppBuilder, AppResponse, BankKeeper, Contract, ContractWrapper, Executor, MockApiBech32,
    WasmKeeper,
};

use airdrop_manager::msg::{
    CampaignAction, CampaignResponse, ExecuteMsg, InstantiateMsg, QueryMsg, RewardsResponse,
};

type MantraApp = App<BankKeeper, MockApiBech32>;

pub fn airdrop_manager_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        airdrop_manager::contract::execute,
        airdrop_manager::contract::instantiate,
        airdrop_manager::contract::query,
    )
    .with_migrate(airdrop_manager::contract::migrate);

    Box::new(contract)
}

pub struct TestingSuite {
    app: MantraApp,
    pub senders: Vec<Addr>,
    pub airdrop_manager_addr: Addr,
}

// helpers
impl TestingSuite {
    #[track_caller]
    pub(crate) fn admin(&mut self) -> Addr {
        self.senders.first().unwrap().clone()
    }

    #[track_caller]
    pub(crate) fn get_time(&mut self) -> Timestamp {
        self.app.block_info().time
    }

    #[track_caller]
    pub(crate) fn add_day(&mut self) -> &mut Self {
        let mut block_info = self.app.block_info();
        block_info.time = block_info.time.plus_days(1);
        self.app.set_block(block_info);

        self
    }

    #[track_caller]
    pub(crate) fn add_week(&mut self) -> &mut Self {
        let mut block_info = self.app.block_info();
        block_info.time = block_info.time.plus_days(7);
        self.app.set_block(block_info);

        self
    }
}

// instantiate
impl TestingSuite {
    #[track_caller]
    pub fn default_with_balances(initial_balances: Vec<Coin>) -> Self {
        let mut senders = vec![];
        let mut balances = vec![];

        let sender0 = "mantra1c758pr6v2zpgdl2rg2enmjedfglxjkac8m7syw";
        let sender1 = "mantra1jg390tyu84e86ntmzhakcst8gmxnelycwsatsq";
        let sender2 = "mantra1eujd63rrvtc02mt08qp7wfnuzhscgs3laxdgkx";
        let sender3 = "mantra1vyd2mkkrff99kawaqge94puq099ghfkyncntmd";
        let sender4 = "mantra1fawdagn5sfq8c2rylzreepme0yy6h8sdaa603t";

        senders.push(Addr::unchecked(sender0));
        senders.push(Addr::unchecked(sender1));
        senders.push(Addr::unchecked(sender2));
        senders.push(Addr::unchecked(sender3));
        senders.push(Addr::unchecked(sender4));

        balances.push((Addr::unchecked(sender0), initial_balances.clone()));
        balances.push((Addr::unchecked(sender1), initial_balances.clone()));
        balances.push((Addr::unchecked(sender2), initial_balances.clone()));
        balances.push((Addr::unchecked(sender3), initial_balances.clone()));
        balances.push((Addr::unchecked(sender4), initial_balances.clone()));

        let app = AppBuilder::new()
            .with_wasm(WasmKeeper::default())
            .with_wasm(WasmKeeper::default())
            .with_bank(BankKeeper::new())
            .with_api(MockApiBech32::new("mantra"))
            .build(|router, _api, storage| {
                balances.into_iter().for_each(|(account, amount)| {
                    router.bank.init_balance(storage, &account, amount).unwrap()
                });
            });

        TestingSuite {
            app,
            senders,
            airdrop_manager_addr: Addr::unchecked(""),
        }
    }

    #[track_caller]
    pub fn instantiate_airdrop_manager(&mut self, owner: Option<String>) -> &mut Self {
        let msg = InstantiateMsg { owner };

        let airdrop_manager_code_id = self.app.store_code(airdrop_manager_contract());
        let admin = self.admin();

        self.airdrop_manager_addr = self
            .app
            .instantiate_contract(
                airdrop_manager_code_id,
                admin.clone(),
                &msg,
                &[],
                "mantra-airdrop-manager",
                Some(admin.into_string()),
            )
            .unwrap();

        self
    }
}

pub trait ResultHandler {
    fn handle_result(&self, result: Result<AppResponse, anyhow::Error>);
}

impl<F> ResultHandler for F
where
    F: Fn(Result<AppResponse, anyhow::Error>),
{
    fn handle_result(&self, result: Result<AppResponse, anyhow::Error>) {
        self(result);
    }
}

// execute msg
impl TestingSuite {
    fn execute_contract(
        &mut self,
        sender: &Addr,
        msg: ExecuteMsg,
        funds: &[Coin],
        result: impl ResultHandler,
    ) -> &mut Self {
        result.handle_result(self.app.execute_contract(
            sender.clone(),
            self.airdrop_manager_addr.clone(),
            &msg,
            funds,
        ));

        self
    }

    #[track_caller]
    pub(crate) fn manage_campaign(
        &mut self,
        sender: &Addr,
        action: CampaignAction,
        funds: &[Coin],
        result: impl ResultHandler,
    ) -> &mut Self {
        self.execute_contract(sender, ExecuteMsg::ManageCampaign { action }, funds, result)
    }

    #[track_caller]
    pub(crate) fn claim(
        &mut self,
        sender: &Addr,
        total_claimable_amount: Uint128,
        receiver: Option<String>,
        proof: Vec<String>,
        result: impl ResultHandler,
    ) -> &mut Self {
        self.execute_contract(
            sender,
            ExecuteMsg::Claim {
                total_claimable_amount,
                receiver,
                proof,
            },
            &[],
            result,
        )
    }

    #[track_caller]
    pub(crate) fn update_ownership(
        &mut self,
        sender: &Addr,
        action: cw_ownable::Action,
        result: impl ResultHandler,
    ) -> &mut Self {
        self.execute_contract(sender, ExecuteMsg::UpdateOwnership(action), &[], result)
    }
}

// queries
impl TestingSuite {
    fn query_contract<T>(&mut self, msg: QueryMsg, result: impl Fn(StdResult<T>)) -> &mut Self
    where
        T: serde::de::DeserializeOwned,
    {
        let response: StdResult<T> = self
            .app
            .wrap()
            .query_wasm_smart(&self.airdrop_manager_addr, &msg);

        result(response);

        self
    }

    #[track_caller]
    pub(crate) fn query_campaigns(
        &mut self,
        result: impl Fn(StdResult<CampaignResponse>),
    ) -> &mut Self {
        self.query_contract(QueryMsg::Campaign {}, result)
    }

    #[track_caller]
    pub(crate) fn query_rewards(
        &mut self,
        total_claimable_amount: Uint128,
        receiver: String,
        proof: Vec<String>,
        result: impl Fn(StdResult<RewardsResponse>),
    ) -> &mut Self {
        self.query_contract(
            QueryMsg::Rewards {
                total_claimable_amount,
                receiver,
                proof,
            },
            result,
        )
    }

    #[track_caller]
    pub(crate) fn _query_ownership(
        &mut self,
        result: impl Fn(StdResult<cw_ownable::Ownership<String>>),
    ) -> &mut Self {
        self.query_contract(QueryMsg::Ownership {}, result)
    }

    #[track_caller]
    pub(crate) fn query_balance(
        &mut self,
        denom: &str,
        address: &Addr,
        result: impl Fn(Uint128),
    ) -> &mut Self {
        let balance_response = self.app.wrap().query_balance(address, denom);
        result(balance_response.unwrap_or(coin(0, denom)).amount);
        self
    }
}

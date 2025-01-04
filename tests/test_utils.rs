#[cfg(test)]

pub mod test_utils {

    use cosmwasm_schema::{
        cw_serde,
        QueryResponses,
    };
    use cosmwasm_std::Response;
    use cosmwasm_std::StdError;
    use cosmwasm_std::{
        coins,
        Addr,
    };
    use cosmwasm_std::{
        DepsMut,
        Env,
        MessageInfo,
    };
    use cw_multi_test::AppResponse;
    use cw_multi_test::{
        next_block,
        App,
        ContractWrapper,
        Executor,
    };
    use cw_orch::anyhow::Error;
    use cw_orch::prelude::Empty;
    use cw_random::client::{
        JobResult,
        ReceiveRandomnessMsg,
        RequestedJob,
    };
    use cw_random::config::Config as RandomConfig;
    use cw_random::contract::{
        execute as random_execute,
        instantiate as random_instantiate,
        query as random_query,
        reply as random_reply,
        sudo as random_sudo,
    };
    use cw_random::state::AddWhitelistedAddress;
    use roll_a_dice::config::Config;
    use roll_a_dice::contract::{
        execute,
        instantiate,
        query,
        reply,
    };

    use roll_a_dice::msg::UserGamesQueryMsg;
    use roll_a_dice::responses::QueryPlayResponse;
    use roll_a_dice::responses::QueryUserGamesResponse;
    use roll_a_dice::state::{
        ExactNumberGame,
        GameRequest,
        GameType,
    };

    use cw_random::msg::{
        ExecuteMsg as RandomExecuteMsg,
        GenerateMsg,
        InstantiateMsg as RandomInstantiateMsg,
    };
    use roll_a_dice::msg::{
        ExecuteMsg,
        FetchRandomCWConfigMsg,
        InstantiateMsg,
        QueryMsg,
    };

    #[cw_serde]

    pub struct MockInstantiateMsg {}

    #[cw_serde]

    pub enum MockExecuteMsg {
        DefaultExecute(),
        ExecuteRandomness(ReceiveRandomnessMsg,),
    }

    #[cw_serde]

    pub enum MockFailingExecuteMsg {
        DefaultExecute(),
    }

    #[cw_serde]
    #[derive(cw_orch::QueryFns, QueryResponses,)]

    pub enum MockQueryMsg {
        #[returns(Empty)]
        Config {},
    }

    pub trait ExecutableMsg {
        fn execute(
            self,
            deps: DepsMut,
            env: Env,
            info: MessageInfo,
        ) -> Result<Response, StdError,>;
    }

    pub fn def_app(
        addr_list: Vec<String,>,
        amount: u128,
        denom: String,
    ) -> App {

        let app = App::new(|router, _, storage| {
            for addr in addr_list.iter() {

                router
                    .bank
                    .init_balance(storage, &Addr::unchecked(addr,), coins(amount, denom.to_string(),),)
                    .unwrap();
            }
        },);

        app
    }

    pub const GAS_TO_TOKEN_RATIO: u64 = 2;

    pub const GAS_PRICE_PER_JOB: u64 = 2;

    pub const DENOM: &str = "ujuno";

    pub const MAX_GAS_PER_BLOCK: u64 = 100_000;

    pub const MAX_JOB_PER_REQUEST: u16 = 10;

    pub const MAX_NUMBER_FOR_JOB: u16 = 10;

    pub const MAX_RECIPIENTS: u16 = 10;

    pub fn get_random_cw_default_config(operator: &Addr,) -> RandomConfig {

        RandomConfig {
            gas_to_token_ratio: GAS_TO_TOKEN_RATIO,
            gas_price_per_job: GAS_PRICE_PER_JOB,
            denom_accepted: DENOM.to_string(),
            max_gas_per_block: MAX_GAS_PER_BLOCK,
            max_job_per_request: MAX_JOB_PER_REQUEST,
            max_number_for_job: MAX_NUMBER_FOR_JOB,
            max_recipients: MAX_RECIPIENTS,
            operator: Some(operator.to_owned(),),
        }
    }

    pub const FEE_PERCENTAGE: u64 = 5;

    pub const MIN_BET_AMOUNT: u128 = 1_000_000;

    pub const DISABLED: bool = false;

    pub const GAS_LIMIT: u64 = 5000;

    pub fn get_roll_dice_cw_default_config(
        random_cw_address: &Addr,
        operator: &Addr,
    ) -> Config {

        Config {
            accepted_denom: DENOM.to_string(),
            random_cw_address: random_cw_address.clone(),
            fee_percentage: FEE_PERCENTAGE,
            min_bet: MIN_BET_AMOUNT,
            operator: Some(operator.clone(),),
            disabled: DISABLED,
            gas_limit: GAS_LIMIT,
        }
    }

    pub fn test_roll_dice_contract(app: &mut App,) -> u64 {

        let contract = ContractWrapper::new(execute, instantiate, query,).with_reply(reply,);

        let boxed_contract = Box::new(contract,);

        app.store_code(boxed_contract,)
    }

    pub fn test_roll_dice_instantiator(
        app: &mut App,
        code_id: u64,
        sender_address: &Addr,
        config: &Config,
        amount: u128,
        denom: String,
    ) -> Addr {

        let inst_msg = InstantiateMsg {
            config: config.clone(),
        };

        let roll_dice_contract_address = app
            .instantiate_contract(
                code_id,
                sender_address.clone(),
                &inst_msg,
                &coins(amount.into(), denom.clone(),),
                "test",
                Some(sender_address.to_string(),),
            )
            .unwrap();

        roll_dice_contract_address
    }

    pub fn test_cw_random_contract(app: &mut App,) -> u64 {

        let contract = ContractWrapper::new(random_execute, random_instantiate, random_query,)
            .with_reply(random_reply,)
            .with_sudo(random_sudo,);

        let boxed_contract = Box::new(contract,);

        app.store_code(boxed_contract,)
    }

    pub fn test_cw_random_instantiator(
        app: &mut App,
        code_id: u64,
        sender_address: &Addr,
        config: &RandomConfig,
    ) -> Addr {

        let inst_msg = RandomInstantiateMsg {
            config: config.to_owned(),
            starting_seed: Some("starting_seed1".to_string(),),
        };

        let rand_contract_address = app
            .instantiate_contract(
                code_id,
                sender_address.clone(),
                &inst_msg,
                &[],
                "test",
                Some(sender_address.to_string(),),
            )
            .unwrap();

        rand_contract_address
    }

    pub fn add_addresses_to_whitelist(
        app: &mut App,
        contract_address: &Addr,
        addresses: &Vec<Addr,>,
        owner_address: &Addr,
    ) {

        // add recipients to whitelisted addresses
        let add_addresses_msg = RandomExecuteMsg::AddWhitelistedAddressMsg(AddWhitelistedAddress {
            addresses: addresses.clone(),
        },);

        let _exec_response = app
            .execute_contract(owner_address.clone(), contract_address.clone(), &add_addresses_msg, &[],)
            .unwrap();
    }

    pub fn get_play_request_exact_number_execute_msg(chosen_number: u8,) -> ExecuteMsg {

        ExecuteMsg::PlayRequest(roll_a_dice::msg::PlayRequestMsg {
            game_type: GameType::ExactNumber(ExactNumberGame {
                chosen_number: chosen_number,
            },),
        },)
    }

    pub fn get_set_config_execute_msg(config: Config,) -> ExecuteMsg {

        ExecuteMsg::SetConfig(config,)
    }

    pub fn get_withdraw_execute_msg(
        denom: String,
        amount: u128,
    ) -> ExecuteMsg {

        ExecuteMsg::Withdraw(roll_a_dice::msg::WithdrawMsg {
            token: roll_a_dice::token::Token::Denom(denom,),
            amount: amount.into(),
        },)
    }

    pub fn get_play_request_high_low_execute_msg(dice_number: u8,) -> ExecuteMsg {

        ExecuteMsg::PlayRequest(roll_a_dice::msg::PlayRequestMsg {
            game_type: GameType::HighLow(roll_a_dice::state::HighLowGame {
                dice_number: dice_number,
            },),
        },)
    }

    pub fn advance_height_and_trigger_randomness_generation(
        app: &mut App,
        random_contract_address: &Addr,
        owner_address: &Addr,
    ) {

        app.update_block(next_block,);

        let generate_msg = RandomExecuteMsg::Generate(GenerateMsg { height_id: None, },);

        let _exec_response = app
            .execute_contract(
                owner_address.clone(),
                random_contract_address.clone(),
                &generate_msg,
                &[],
            )
            .unwrap();
    }

    pub fn send_randomness(
        app: &mut App,
        random_contract_address: &Addr,
        roll_dice_cw_addr: &Addr,
        game_id: u64,
        number_of_dice: u16,
    ) -> Result<AppResponse, Error,> {

        let msg = ExecuteMsg::ReceiveRandomness(ReceiveRandomnessMsg {
            id: game_id,
            results: vec![JobResult::U8(vec![4; number_of_dice as usize],)],
        },);

        let exec_response =
            app.execute_contract(random_contract_address.clone(), roll_dice_cw_addr.clone(), &msg, &[],);

        exec_response
    }

    pub fn send_empty_randomness(
        app: &mut App,
        random_contract_address: &Addr,
        roll_dice_cw_addr: &Addr,
        game_id: u64,
    ) {

        let empty_msg = ExecuteMsg::ReceiveRandomness(ReceiveRandomnessMsg {
            id: game_id,
            results: vec![],
        },);

        let _exec_response = app
            .execute_contract(
                random_contract_address.clone(),
                roll_dice_cw_addr.clone(),
                &empty_msg,
                &[],
            )
            .unwrap();
    }

    pub fn send_wrong_type_randomness(
        app: &mut App,
        random_contract_address: &Addr,
        roll_dice_cw_addr: &Addr,
        game_id: u64,
        number_of_dice: u16,
    ) {

        let wrong_type_msg = ExecuteMsg::ReceiveRandomness(ReceiveRandomnessMsg {
            id: game_id,
            results: vec![JobResult::U16(vec![4; number_of_dice as usize],)],
        },);

        let _exec_response = app
            .execute_contract(
                random_contract_address.clone(),
                roll_dice_cw_addr.clone(),
                &wrong_type_msg,
                &[],
            )
            .unwrap();
    }

    pub fn send_win_high_low_game(
        app: &mut App,
        random_contract_address: &Addr,
        roll_dice_cw_addr: &Addr,
        game_id: u64,
        number_of_dice: u16,
    ) {

        let win_msg = ExecuteMsg::ReceiveRandomness(ReceiveRandomnessMsg {
            id: game_id,
            results: vec![JobResult::U8(vec![4; number_of_dice as usize],)],
        },);

        let _exec_response = app
            .execute_contract(
                random_contract_address.clone(),
                roll_dice_cw_addr.clone(),
                &win_msg,
                &[],
            )
            .unwrap();
    }

    pub fn send_lose_high_low_game(
        app: &mut App,
        random_contract_address: &Addr,
        roll_dice_cw_addr: &Addr,
        game_id: u64,
        number_of_dice: u16,
    ) {

        let lose_msg = ExecuteMsg::ReceiveRandomness(ReceiveRandomnessMsg {
            id: game_id,
            results: vec![JobResult::U8(vec![2; number_of_dice as usize],)],
        },);

        let _exec_response = app
            .execute_contract(
                random_contract_address.clone(),
                roll_dice_cw_addr.clone(),
                &lose_msg,
                &[],
            )
            .unwrap();
    }

    pub fn send_win_exact_number_game(
        app: &mut App,
        random_contract_address: &Addr,
        roll_dice_cw_addr: &Addr,
        game_id: u64,
        chosen_number: u8,
    ) {

        let dice_number1 = chosen_number / 2;

        let dice_number2 = chosen_number - dice_number1;

        let win_msg = ExecuteMsg::ReceiveRandomness(ReceiveRandomnessMsg {
            id: game_id,
            results: vec![JobResult::U8(vec![dice_number1, dice_number2],)],
        },);

        let _exec_response = app
            .execute_contract(
                random_contract_address.clone(),
                roll_dice_cw_addr.clone(),
                &win_msg,
                &[],
            )
            .unwrap();
    }

    pub fn send_invalid_dice_number(
        app: &mut App,
        random_contract_address: &Addr,
        roll_dice_cw_addr: &Addr,
        game_id: u64,
        number_of_dice: u16,
    ) {

        let invalid_msg = ExecuteMsg::ReceiveRandomness(ReceiveRandomnessMsg {
            id: game_id,
            results: vec![JobResult::U8(vec![7; number_of_dice as usize],)],
        },);

        let _exec_response = app
            .execute_contract(
                random_contract_address.clone(),
                roll_dice_cw_addr.clone(),
                &invalid_msg,
                &[],
            )
            .unwrap();
    }

    pub fn send_lose_exact_number_game(
        app: &mut App,
        random_contract_address: &Addr,
        roll_dice_cw_addr: &Addr,
        game_id: u64,
        chosen_number: u8,
    ) {

        let dice_number1 = chosen_number / 2;

        let mut dice_number2 = chosen_number - dice_number1;

        if dice_number2 == 1 {

            dice_number2 = 6;
        } else {

            dice_number2 = dice_number2 - 1;
        }

        let lose_msg = ExecuteMsg::ReceiveRandomness(ReceiveRandomnessMsg {
            id: game_id,
            results: vec![JobResult::U8(vec![dice_number1, dice_number2],)],
        },);

        let _exec_response = app
            .execute_contract(
                random_contract_address.clone(),
                roll_dice_cw_addr.clone(),
                &lose_msg,
                &[],
            )
            .unwrap();
    }

    pub fn fetch_random_cw_config(
        app: &mut App,
        contract_address: &Addr,
        owner_address: &Addr,
    ) {

        let fetch_config_msg = ExecuteMsg::FetchRandomCWConfig(FetchRandomCWConfigMsg {},);

        let _exec_response = app
            .execute_contract(owner_address.clone(), contract_address.clone(), &fetch_config_msg, &[],)
            .unwrap();
    }

    pub fn query_roll_dice_game_request(
        app: &mut App,
        contract_address: &Addr,
        game_id: u64,
    ) -> GameRequest {

        let query_msg = QueryMsg::QueryGame { id: game_id, };

        let resp: QueryPlayResponse = app
            .wrap()
            .query_wasm_smart(contract_address.clone(), &query_msg,)
            .unwrap();

        resp.0
    }

    pub fn query_user_games<A: Into<u8,>,>(
        app: &mut App,
        contract_address: &Addr,
        user_address: &Addr,
        starting_index: Option<usize,>,
        limit: A,
    ) -> Result<QueryUserGamesResponse, StdError,> {

        let query_msg = QueryMsg::UserGamesQuery(UserGamesQueryMsg {
            user_address: user_address.clone(),
            cursor: starting_index,
            limit: limit.into(),
        },);

        let resp: Result<QueryUserGamesResponse, StdError,> =
            app.wrap().query_wasm_smart(contract_address.clone(), &query_msg,);

        resp
    }

    pub fn query_roll_dice_config(
        app: &mut App,
        contract_address: &Addr,
    ) -> Config {

        let query_msg = QueryMsg::Config {};

        let resp: roll_a_dice::responses::ConfigResponse = app
            .wrap()
            .query_wasm_smart(contract_address.clone(), &query_msg,)
            .unwrap();

        resp.0
    }

    // get an array having a request for each kind of RequestedJob
    pub fn get_request_job_array(n: u16,) -> Vec<RequestedJob,> {

        let job_u8 = RequestedJob::U8 {
            min: None,
            max: None,
            n: n,
        };

        let job_u16 = RequestedJob::U16 {
            min: None,
            max: None,
            n: n,
        };

        let job_u32 = RequestedJob::U32 {
            min: None,
            max: None,
            n: n,
        };

        let job_u64 = RequestedJob::U64 {
            min: None,
            max: None,
            n: n,
        };

        let job_i8 = RequestedJob::I8 {
            min: None,
            max: None,
            n: n,
        };

        let job_i16 = RequestedJob::I16 {
            min: None,
            max: None,
            n: n,
        };

        let job_i32 = RequestedJob::I32 {
            min: None,
            max: None,
            n: n,
        };

        let job_i64 = RequestedJob::I64 {
            min: None,
            max: None,
            n: n,
        };

        let job_u128 = RequestedJob::U128 {
            min: None,
            max: None,
            n: n,
        };

        let job_i128 = RequestedJob::I128 {
            min: None,
            max: None,
            n: n,
        };

        vec![
            job_u8, job_u16, job_u32, job_u64, job_i8, job_i16, job_i32, job_i64, job_u128, job_i128,
        ]
    }
}

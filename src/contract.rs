use crate::error::ContractError;
use crate::execute::play_request::exec_play_request;
use crate::execute::receive_randomness::exec_handle_received_randomness;
use crate::execute::set_config::exec_set_config;
use crate::execute::update_random_cw_cfg::exec_fetch_random_cw_cfg;
use crate::execute::withdraw::exec_withdraw;
use crate::math::sub_u128;
use crate::msg::{
    ExecuteMsg,
    InstantiateMsg,
    MigrateMsg,
    QueryMsg,
};
use crate::query::config::query_config;
use crate::query::play_request::query_game;
use crate::query::user_games::query_user_games;
use crate::state::{
    ExecuteContext,
    QueryContext,
    ID_GAME_REQUEST_MAP,
    NOT_RESOLVED_BET_AMOUNT,
};
use cosmwasm_std::{
    entry_point,
    to_json_binary,
    Env,
    Reply,
};
use cosmwasm_std::{
    Binary,
    Deps,
    DepsMut,
    MessageInfo,
    Response,
};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "crates.io:roll-a-dice";

const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError,> {

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION,)?;

    let mut ctx = ExecuteContext { deps, env, info, };

    ctx.instantiate(msg,)
}

#[entry_point]

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError,> {

    let ctx = ExecuteContext { deps, env, info, };

    match msg {
        ExecuteMsg::SetConfig(config,) => exec_set_config(ctx, config,),
        ExecuteMsg::PlayRequest(play_req_msg,) => exec_play_request(ctx, play_req_msg,),
        ExecuteMsg::ReceiveRandomness(received_randomness_msg,) => {
            exec_handle_received_randomness(ctx, received_randomness_msg,)
        },
        ExecuteMsg::FetchRandomCWConfig(_,) => exec_fetch_random_cw_cfg(ctx,),
        ExecuteMsg::Withdraw(withdraw_msg,) => exec_withdraw(ctx, withdraw_msg,),
    }
}

#[entry_point]

pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> Result<Binary, ContractError,> {

    let ctx = QueryContext { deps, env, };

    let result = match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(ctx,)?,),
        QueryMsg::QueryGame { id, } => to_json_binary(&query_game(ctx, id,)?,),
        QueryMsg::UserGamesQuery(user_games_query_msg,) => {
            to_json_binary(&query_user_games(ctx, user_games_query_msg,)?,)
        },
    }?;

    Ok(result,)
}

#[entry_point]

pub fn migrate(
    deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response, ContractError,> {

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION,)?;

    Ok(Response::default(),)
}

#[entry_point]

pub fn reply(
    deps: DepsMut,
    env: Env,
    msg: Reply,
) -> Result<Response, ContractError,> {

    let mut resp = Response::new();

    let request_id = msg.id;

    if msg.result.is_err() {

        let mut game_request = ID_GAME_REQUEST_MAP.load(deps.storage, request_id.to_string(),)?;

        let transfer_submsg =
            game_request.refund(env.block.time.seconds(), Some(msg.result.unwrap_err().to_string(),),)?;

        resp = resp.add_submessage(transfer_submsg,);

        let not_resolved_bet_amount = NOT_RESOLVED_BET_AMOUNT.load(deps.storage,)?;

        NOT_RESOLVED_BET_AMOUNT.save(
            deps.storage,
            &(sub_u128(not_resolved_bet_amount, game_request.bet_amount,)?),
        )?;

        ID_GAME_REQUEST_MAP.save(deps.storage, request_id.to_string(), &game_request,)?;
    }

    Ok(resp,)
}

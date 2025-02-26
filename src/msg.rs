use cosmwasm_schema::{
    cw_serde,
    QueryResponses,
};
use cosmwasm_std::Addr;
use cw_random::client::ReceiveRandomnessMsg;

#[allow(unused_imports)]
use crate::{
    config::Config,
    config::ConfigMsg,
    responses::ConfigResponse,
};
use crate::{
    responses::{
        QueryPlayResponse,
        QueryUserGamesResponse,
    },
    state::GameType,
    token::Token,
};

#[cw_serde]

pub struct InstantiateMsg {
    pub config: ConfigMsg,
}

#[cw_serde]

pub struct PlayRequestMsg {
    pub game_type: GameType,
}

#[cw_serde]

pub struct WithdrawMsg {
    pub token: Token,
    pub amount: String,
}

#[cw_serde]
#[derive(cw_orch::ExecuteFns,)]

pub enum ExecuteMsg {
    SetConfig(ConfigMsg,),
    PlayRequest(PlayRequestMsg,),
    ReceiveRandomness(ReceiveRandomnessMsg,),
    FetchRandomCWConfig(FetchRandomCWConfigMsg,),
    Withdraw(WithdrawMsg,),
}

#[cw_serde]

pub struct FetchRandomCWConfigMsg {}

#[cw_serde]

pub struct UserGamesQueryMsg {
    pub user_address: Addr,
    pub cursor: Option<usize,>,
    pub limit: u8,
}

#[cw_serde]
#[derive(cw_orch::QueryFns, QueryResponses,)]

pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(QueryPlayResponse)]
    QueryGame { id: String, },
    #[returns(QueryUserGamesResponse)]
    UserGamesQuery(UserGamesQueryMsg,),
}

#[cw_serde]

pub struct MigrateMsg {}

use cosmwasm_schema::cw_serde;

use crate::config::Config;
use crate::state::GameRequest;

#[cw_serde]

pub struct ConfigResponse(pub Config,);

#[cw_serde]

pub struct QueryPlayResponse(pub GameRequest,);

#[cw_serde]

pub struct QueryUserGamesResponse {
    pub games: Vec<GameRequest,>,
    pub next_cursor: Option<u64,>,
}

use crate::error::ContractError;
use crate::msg::UserGamesQueryMsg;
use crate::responses::QueryUserGamesResponse;
use crate::state::{
    GameRequest,
    QueryContext,
    ID_GAME_REQUEST_MAP,
    USER_ADDRESS_ID_GAME_REQUEST_MAP,
};

pub const MAX_REQUEST_LIMIT: u8 = 30;

/// Return x number of NameRecords

pub fn query_user_games(
    ctx: QueryContext,
    msg: UserGamesQueryMsg,
) -> Result<Option<QueryUserGamesResponse,>, ContractError,> {

    let UserGamesQueryMsg {
        limit,
        cursor,
        user_address,
    } = msg;

    if limit > MAX_REQUEST_LIMIT {

        return Err(ContractError::TooManyRecords {
            limit: MAX_REQUEST_LIMIT,
        },);
    }

    // check if the user_address has any games
    let user_games_id = USER_ADDRESS_ID_GAME_REQUEST_MAP.may_load(ctx.deps.storage, user_address.clone(),)?;

    if user_games_id.is_none() {

        return Err(ContractError::NotFound {
            reason: format!("User {} has not played any games", user_address.clone()),
        },);
    }

    let games_id = user_games_id.unwrap();

    let starting_id;

    if let Some(id,) = cursor {

        if games_id.len() <= id {

            return Err(ContractError::NotFound {
                reason: format!("Game {} for  not found", id.clone()),
            },);
        }

        starting_id = id;
    } else {

        starting_id = 0;
    }

    let exclusive_max_bound = usize::min(starting_id + limit as usize, games_id.len(),);

    let mut user_games: Vec<GameRequest,> = Vec::with_capacity(limit as usize,);

    let next_cursor_id: Option<u64,>;

    if exclusive_max_bound >= games_id.len() {

        next_cursor_id = None;
    } else {

        next_cursor_id = Some(exclusive_max_bound as u64,);
    }

    for i in starting_id..exclusive_max_bound {

        let game_id = games_id.get(i,).unwrap();

        let game = ID_GAME_REQUEST_MAP.may_load(ctx.deps.storage, game_id.to_string(),)?;

        if game.is_none() {

            // this should never happen
            return Err(ContractError::NotFound {
                reason: format!("Game {} for  not found", game_id.clone()),
            },);
        }

        user_games.push(game.unwrap(),);
    }

    Ok(Some(QueryUserGamesResponse {
        games: user_games,
        next_cursor: next_cursor_id,
    },),)
}

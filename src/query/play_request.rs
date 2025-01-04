use crate::{
    error::ContractError,
    responses::QueryPlayResponse,
    state::{
        QueryContext,
        ID_GAME_REQUEST_MAP,
    },
};

pub fn query_game(
    ctx: QueryContext,
    id: u64,
) -> Result<Option<QueryPlayResponse,>, ContractError,> {

    let QueryContext { deps, .. } = ctx;

    if let Some(game_request,) = ID_GAME_REQUEST_MAP.may_load(deps.storage, id.to_string(),)? {

        return Ok(Some(QueryPlayResponse(game_request,),),);
    }

    Ok(None,)
}

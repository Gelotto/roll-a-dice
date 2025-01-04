use crate::{
    config::get_current_config,
    error::ContractError,
    responses::ConfigResponse,
    state::QueryContext,
};

pub fn query_config(ctx: QueryContext,) -> Result<ConfigResponse, ContractError,> {

    let QueryContext { deps, .. } = ctx;

    let saved_config = get_current_config(deps.storage,);

    Ok(ConfigResponse(saved_config,),)
}

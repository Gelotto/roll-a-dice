use crate::{
    config::{
        get_operator,
        get_random_cw_address,
    },
    error::ContractError,
    state::{
        ExecuteContext,
        RANDOM_CONFIG,
    },
};
use cosmwasm_std::{
    attr,
    Response,
};

use cw_random::client::{
    ConfigResponse as RandomCwConfigResponse,
    QueryRandomCWCFG,
};

pub fn exec_fetch_random_cw_cfg(ctx: ExecuteContext,) -> Result<Response, ContractError,> {

    let ExecuteContext { deps, .. } = ctx;

    let saved_operator = get_operator(deps.storage,)?;

    if saved_operator != ctx.info.sender {

        return Err(ContractError::NotAuthorized {
            reason: "not authorized".to_string(),
        },);
    }

    let random_contract_addr = get_random_cw_address(deps.storage,)?;

    let random_cw_config: RandomCwConfigResponse = deps
        .querier
        .query_wasm_smart(random_contract_addr, &QueryRandomCWCFG::Config {},)?;

    RANDOM_CONFIG.save(deps.storage, &Some(random_cw_config.0,),)?;

    Ok(Response::new().add_attributes(vec![attr("action", "update_random_cw_cfg",)],),)
}

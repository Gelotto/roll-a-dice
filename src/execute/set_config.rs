use crate::{
    config::{
        get_operator,
        Config,
    },
    error::ContractError,
    state::ExecuteContext,
};
use cosmwasm_std::{
    attr,
    Response,
};

pub fn exec_set_config(
    ctx: ExecuteContext,
    config: Config,
) -> Result<Response, ContractError,> {

    let ExecuteContext { mut deps, .. } = ctx;

    let saved_operator = get_operator(deps.storage,)?;

    if saved_operator != ctx.info.sender {

        return Err(ContractError::NotAuthorized {
            reason: "not authorized".to_string(),
        },);
    }

    config.save(&mut deps, saved_operator,)?;

    Ok(Response::new().add_attributes(vec![attr("action", "set_config",)],),)
}

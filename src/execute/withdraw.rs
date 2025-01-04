use crate::error::ContractError;
use crate::{
    config::{
        get_accepted_denom,
        get_operator,
    },
    math::sub_u128,
    msg::WithdrawMsg,
    state::{
        ExecuteContext,
        NOT_RESOLVED_BET_AMOUNT,
    },
    token::Token,
};
use cosmwasm_std::{
    attr,
    Response,
    Uint128,
};

pub fn exec_withdraw(
    ctx: ExecuteContext,
    withdraw_msg: WithdrawMsg,
) -> Result<Response, ContractError,> {

    let saved_operator = get_operator(ctx.deps.storage,)?;

    if saved_operator != ctx.info.sender {

        return Err(ContractError::NotAuthorized {
            reason: "not authorized".to_string(),
        },);
    }

    let mut resp = Response::new();

    let token_requested = withdraw_msg.token;

    let amount_requested = withdraw_msg.amount;

    let type_of_token = match &token_requested {
        Token::Denom(denom,) => denom.to_string(),
        Token::Address(address,) => address.to_string(),
    };

    // query the contract balance

    // check contract balance
    let contract_balance = token_requested.query_balance(ctx.deps.querier, &ctx.env.contract.address,)?;

    if contract_balance == Uint128::zero() {

        return Err(ContractError::InsufficientFunds {
            denom_requested: type_of_token,
            requested: amount_requested.into(),
            available: contract_balance.into(),
        },);
    }

    let withdrawable_amount = match &token_requested {
        Token::Denom(denom,) => {

            let denom_accepted = get_accepted_denom(ctx.deps.storage,)?;

            if *denom == denom_accepted {

                let not_resolved_bet_amount = NOT_RESOLVED_BET_AMOUNT.load(ctx.deps.storage,)?;

                let usable_balance = sub_u128(contract_balance, not_resolved_bet_amount,)?;

                Uint128::min(amount_requested.into(), usable_balance,)
            } else {

                Uint128::min(amount_requested.into(), contract_balance,)
            }
        },
        Token::Address(_,) => Uint128::min(amount_requested.into(), contract_balance,),
    };

    resp = resp.add_submessage(token_requested.transfer(&ctx.info.sender, withdrawable_amount,)?,);

    resp = resp.add_attributes(vec![
        attr("action", "withdraw",),
        attr("token", type_of_token,),
        attr("amount", withdrawable_amount,),
    ],);

    Ok(resp,)
}

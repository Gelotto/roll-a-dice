use cw_random::client::{
    JobResult,
    ReceiveRandomnessMsg,
};

use crate::config::get_random_cw_address;
use crate::error::ContractError;
use crate::{
    math::sub_u128,
    state::{
        ExecuteContext,
        GameRequest,
        GameStatus,
        GameType,
        ID_GAME_REQUEST_MAP,
        NOT_RESOLVED_BET_AMOUNT,
    },
    token::TokenAmount,
};
use cosmwasm_std::{
    attr,
    Response,
};

pub fn exec_handle_received_randomness(
    ctx: ExecuteContext,
    received_randomness_msg: ReceiveRandomnessMsg,
) -> Result<Response, ContractError,> {

    // this function can be called only by the random contract

    // check if the sender is the random contract
    let random_cw_address = get_random_cw_address(ctx.deps.storage,)?;

    if ctx.info.sender != random_cw_address {

        return Err(ContractError::NotAuthorized {
            reason: "Only random cw address can call this".to_string(),
        },);
    }

    let mut resp = Response::new();

    let game_id = received_randomness_msg.id;

    let mut game_request: GameRequest = ID_GAME_REQUEST_MAP.load(ctx.deps.storage, game_id.to_string(),)?;

    let results = received_randomness_msg.results;

    if game_request.status != GameStatus::Requested {

        return Err(ContractError::InvalidGameStatus {
            reason: "Game status is not in Requested State".to_string(),
        },);
    }

    let denom_token = TokenAmount {
        token: crate::token::Token::Denom(game_request.bet_currency.clone(),),
        amount: game_request.bet_amount,
    };

    // check contract balance
    let contract_balance = denom_token
        .token
        .query_balance(ctx.deps.querier, &ctx.env.contract.address,)?;

    let not_resolved_bet_amount = NOT_RESOLVED_BET_AMOUNT.load(ctx.deps.storage,)?;

    let usable_balance = sub_u128(contract_balance, not_resolved_bet_amount,)?;

    let mut error_msg: Option<String,> = None;

    if results.len() != 1 {

        game_request.status = GameStatus::Refunded;

        error_msg = Some("Invalid Randomness Results".to_string(),);
    } else if let JobResult::U8(dice_results,) = &results[0] {

        let check_results = match &game_request.game_type {
            GameType::HighLow(high_low_game,) => high_low_game.check_win(dice_results,),
            GameType::ExactNumber(exact_number_game,) => exact_number_game.check_win(dice_results,),
        };

        if let Err(err,) = check_results {

            game_request.status = GameStatus::Refunded;

            error_msg = Some(err.to_string(),);
        } else {

            game_request.status = if check_results.unwrap() {

                GameStatus::Won
            } else {

                GameStatus::Lost
            };

            game_request.updated_at = ctx.env.block.time.seconds();

            game_request.dice_results = Some(dice_results.to_owned(),);

            if game_request.status == GameStatus::Won {

                let potential_winning_amount = game_request.potential_winning_amount;

                if usable_balance < potential_winning_amount {

                    game_request.status = GameStatus::Refunded;

                    error_msg = Some("Not enough balance -> refunding".to_string(),);
                }
            }
        }
    } else {

        game_request.status = GameStatus::Refunded;

        error_msg = Some("Invalid Randomness Results".to_string(),);
    }

    resp = match game_request.status {
        GameStatus::Won => {

            let transfer_submsg = denom_token
                .token
                .transfer(&game_request.user_address, game_request.potential_winning_amount,)?;

            resp.add_attribute("action", "won",).add_submessage(transfer_submsg,)
        },
        GameStatus::Lost => resp.add_attribute("action", "lost",),
        GameStatus::Refunded => {

            let transfer_submsg = game_request.refund(ctx.env.block.time.seconds(), error_msg,)?;

            resp.add_attribute("action", "refund",).add_submessage(transfer_submsg,)
        },
        _ => resp.add_attribute("action", "error",),
    };

    resp = resp.add_attributes(vec![
        attr("game_id", game_id.to_string(),),
        attr("user_address", game_request.user_address.to_string(),),
    ],);

    NOT_RESOLVED_BET_AMOUNT.save(
        ctx.deps.storage,
        &(sub_u128(not_resolved_bet_amount, game_request.bet_amount,)?),
    )?;

    ID_GAME_REQUEST_MAP.save(ctx.deps.storage, game_id.to_string(), &game_request,)?;

    Ok(resp,)
}

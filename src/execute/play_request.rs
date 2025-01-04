use crate::config::{
    get_accepted_denom,
    get_disabled,
    get_fee_percentage,
    get_gas_limit,
    get_random_cw_address,
};
use crate::error::ContractError;
use crate::token::TokenAmount;
use crate::{
    config::get_min_bet,
    math::{
        add_u128,
        mul_ratio_u128,
        sub_u128,
    },
    msg::PlayRequestMsg,
    state::{
        ExecuteContext,
        GameRequest,
        GameStatus,
        GameType,
        CURRENT_ID,
        ID_GAME_REQUEST_MAP,
        NOT_RESOLVED_BET_AMOUNT,
        RANDOM_CONFIG,
        USER_ADDRESS_ID_GAME_REQUEST_MAP,
    },
};
use cosmwasm_std::{
    attr,
    coins,
    Response,
    Uint128,
};
use cw_random::client::RequestMsg as RandomRequestMsg;
use cw_random::{
    client::{
        request_randomness_submsg,
        RequestedJob,
        PRNG,
    },
    config::Config as RandomConfig,
};

pub fn exec_play_request(
    ctx: ExecuteContext,
    play_req_msg: PlayRequestMsg,
) -> Result<Response, ContractError,> {

    // check if the contract is disabled
    if get_disabled(ctx.deps.storage,)? {

        return Err(ContractError::DisabledError {},);
    }

    let random_config: RandomConfig = match RANDOM_CONFIG.may_load(ctx.deps.storage,)? {
        Some(config,) => {
            if let Some(config,) = config {

                config
            } else {

                return Err(ContractError::RandomConfigNotSet {},);
            }
        },
        None => return Err(ContractError::RandomConfigNotSet {},),
    };

    let random_cw_address = get_random_cw_address(ctx.deps.storage,)?;

    //check the type of PlayRequestMsg

    let user_address = ctx.info.sender.clone();

    let game_id = CURRENT_ID.load(ctx.deps.storage,)?;

    let (winning_percentage, number_of_requests,) = match &play_req_msg.game_type {
        GameType::HighLow(high_low_game,) => {

            high_low_game.validate()?;

            (
                high_low_game.get_winning_percentage(),
                high_low_game.get_number_of_requests(),
            )
        },
        GameType::ExactNumber(exact_number_game,) => {

            exact_number_game.validate()?;

            (
                exact_number_game.get_winning_percentage(),
                exact_number_game.get_number_of_requests(),
            )
        },
    };

    let denom_accepted = get_accepted_denom(ctx.deps.storage,)?;

    let dice_requested_job = RequestedJob::U8 {
        min: Some(1,),
        max: Some(6,),
        n: number_of_requests as u16,
    };

    let gas_limit = get_gas_limit(ctx.deps.storage,)?;

    let request_msg = RandomRequestMsg {
        height: None,
        recipients: None,
        prng: Some(PRNG::ChaCha20,),
        jobs: vec![dice_requested_job],
        gas_limit: gas_limit,
        response_id: game_id,
    };

    let cw_random_request_token_needed =
        request_msg.calculate_price(random_config.gas_to_token_ratio, random_config.gas_price_per_job,);

    let min_bet = get_min_bet(ctx.deps.storage,)?;

    // get the maximum between the minimum bet and the denom amount needed

    let minimum_bet_amount_needed = Uint128::max(min_bet.into(), cw_random_request_token_needed,);

    let minimum_token_amount_needed = TokenAmount {
        token: crate::token::Token::Denom(denom_accepted.clone(),),
        amount: minimum_bet_amount_needed,
    };

    // check if the user has sent denom_accepted tokens

    if ctx.info.funds.first().map_or(true, |c| {

        c.denom != denom_accepted || c.amount < minimum_token_amount_needed.amount
    },)
    {

        return Err(ContractError::InsufficientFunds {
            denom_requested: denom_accepted,
            requested: minimum_token_amount_needed.amount,
            available: ctx.info.funds.first().map_or(Uint128::zero(), |c| c.amount,),
        },);
    }

    let sent_amount = ctx.info.funds.first().unwrap().amount;

    let bet_amount = sub_u128(sent_amount, cw_random_request_token_needed,)?;

    let fee_percentage = get_fee_percentage(ctx.deps.storage,)?;

    let mut potential_winning_amount = mul_ratio_u128(bet_amount, winning_percentage, 100u128,)?;

    let fee_retained = mul_ratio_u128(potential_winning_amount, fee_percentage, 100u128,)?;

    potential_winning_amount = sub_u128(potential_winning_amount, fee_retained,)?;

    let game_request = GameRequest {
        id: game_id,
        user_address: user_address.clone(),
        status: GameStatus::Requested,
        created_at: ctx.env.block.time.seconds(),
        updated_at: ctx.env.block.time.seconds(),
        bet_amount: bet_amount,
        bet_currency: denom_accepted.clone(),
        game_type: play_req_msg.game_type,
        potential_winning_amount: potential_winning_amount,
        dice_results: None,
        error_message: None,
    };

    ID_GAME_REQUEST_MAP.save(ctx.deps.storage, game_id.to_string(), &game_request,)?;

    let mut user_game_requests = USER_ADDRESS_ID_GAME_REQUEST_MAP
        .load(ctx.deps.storage, user_address.clone(),)
        .unwrap_or_default();

    user_game_requests.push(game_id,);

    USER_ADDRESS_ID_GAME_REQUEST_MAP.save(ctx.deps.storage, user_address.clone(), &user_game_requests,)?;

    CURRENT_ID.save(ctx.deps.storage, &(game_id + 1),)?;

    let not_resolved_bet_amount = NOT_RESOLVED_BET_AMOUNT.load(ctx.deps.storage,)?;

    NOT_RESOLVED_BET_AMOUNT.save(ctx.deps.storage, &(add_u128(not_resolved_bet_amount, bet_amount,)?),)?;

    let mut resp = Response::new().add_attributes(vec![
        attr("action", "play_request",),
        attr("game_id", game_id.to_string(),),
        attr("potential_winning_amount", potential_winning_amount,),
    ],);

    // send the request to the random contract
    let random_cw_submsg = request_randomness_submsg(
        request_msg,
        random_cw_address,
        coins(cw_random_request_token_needed.into(), denom_accepted.clone(),),
        game_id,
    )?;

    resp = resp.add_submessage(random_cw_submsg,);

    Ok(resp,)
}

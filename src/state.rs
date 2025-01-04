use crate::{
    error::ContractError,
    msg::InstantiateMsg,
    token::TokenAmount,
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    Addr,
    Deps,
    DepsMut,
    Env,
    MessageInfo,
    Response,
    SubMsg,
    Uint128,
};
use cw_random::config::Config as RandomConfig;
use cw_storage_plus::{
    Item,
    Map,
};

pub const ID_GAME_REQUEST_MAP: Map<String, GameRequest,> = Map::new("id_game_request",);

pub const CURRENT_ID: Item<u64,> = Item::new("current_id",);

pub const USER_ADDRESS_ID_GAME_REQUEST_MAP: Map<Addr, Vec<u64,>,> = Map::new("user_address_id_game_request",);

pub const RANDOM_CONFIG: Item<Option<RandomConfig,>,> = Item::new("random_config",);

pub const NOT_RESOLVED_BET_AMOUNT: Item<Uint128,> = Item::new("not_resolved_bet_amount",);

// constant vector for double dice probability eg 2: 1/36 3: 2/36 4: 3/36 5: 4/36 6: 5/36 7: 6/36 8: 5/36 9: 4/36 10: 3/36 11: 2/36 12: 1/36
pub const DOUBLE_DICE_PROBABILITY: [u64; 11] = [1, 2, 3, 4, 5, 6, 5, 4, 3, 2, 1,];

pub const DOUBLE_DICE_WINNING_MULTIPLIER_PERCENTAGE: [u64; 11] =
    [300, 270, 240, 200, 180, 150, 180, 200, 240, 270, 300,];

#[cw_serde]

pub enum GameType {
    ExactNumber(ExactNumberGame,), // roll 2 dices, if the sum is equal to the number, user wins
    HighLow(HighLowGame,), // roll an odd number of dices, if the number of dices with value higher than 3 is higher than the number of dices with value lower than 3, user wins
}

#[cw_serde]

pub struct ExactNumberGame {
    pub chosen_number: u8,
}

impl ExactNumberGame {
    pub fn validate(&self,) -> Result<(), ContractError,> {

        if self.chosen_number < 2 || self.chosen_number > 12 {

            return Err(ContractError::InvalidDiceNumber {
                reason: "Invalid dice number".to_string(),
            },);
        }

        Ok((),)
    }

    pub fn get_winning_percentage(&self,) -> u64 {

        if self.chosen_number < 2 || self.chosen_number > 12 {

            return 0u64;
        }

        let winning_percentage = DOUBLE_DICE_WINNING_MULTIPLIER_PERCENTAGE[(self.chosen_number - 2) as usize];

        winning_percentage
    }

    pub fn get_number_of_requests(&self,) -> u64 {

        2u64
    }

    pub fn check_win(
        &self,
        results: &Vec<u8,>,
    ) -> Result<bool, ContractError,> {

        if results.len() != self.get_number_of_requests() as usize {

            return Err(ContractError::InvalidDiceResults {
                reason: "Invalid dice result".to_string(),
            },);
        }

        // check if the results are valid

        for result in results {

            if *result < 1 || *result > 6 {

                return Err(ContractError::InvalidDiceResults {
                    reason: "Invalid dice result".to_string(),
                },);
            }
        }

        Ok(results[0] + results[1] == self.chosen_number,)
    }
}

#[cw_serde]

pub struct HighLowGame {
    pub dice_number: u8,
}

impl HighLowGame {
    pub fn validate(&self,) -> Result<(), ContractError,> {

        if self.dice_number < 1 || self.dice_number % 2 == 0 {

            return Err(ContractError::InvalidDiceNumber {
                reason: "Invalid dice number".to_string(),
            },);
        }

        Ok((),)
    }

    pub fn get_winning_percentage(&self,) -> u64 {

        let winning_percentage = 180u64;

        winning_percentage
    }

    pub fn get_number_of_requests(&self,) -> u64 {

        self.dice_number as u64
    }

    pub fn check_win(
        &self,
        results: &Vec<u8,>,
    ) -> Result<bool, ContractError,> {

        let mut higher_than_3 = 0u64;

        let mut lower_than_3 = 0u64;

        if results.len() != self.get_number_of_requests() as usize {

            return Err(ContractError::InvalidDiceResults {
                reason: "The number of the dice requested is different from the ones received".to_string(),
            },);
        }

        for result in results {

            if *result < 1 || *result > 6 {

                return Err(ContractError::InvalidDiceResults {
                    reason: "Invalid dice result".to_string(),
                },);
            }

            if *result > 3 {

                higher_than_3 += 1;
            } else {

                lower_than_3 += 1;
            }
        }

        Ok(higher_than_3 > lower_than_3,)
    }
}

#[cw_serde]

pub struct GameRequest {
    pub id: u64,
    pub user_address: Addr,
    pub status: GameStatus,
    pub game_type: GameType,
    pub created_at: u64,
    pub updated_at: u64,
    pub bet_amount: Uint128,
    pub bet_currency: String,
    pub potential_winning_amount: Uint128,
    pub dice_results: Option<Vec<u8,>,>,
    pub error_message: Option<String,>,
}

#[cw_serde]

pub enum GameStatus {
    Requested,
    Won,
    Lost,
    Error,
    Refunded,
}

impl GameRequest {
    pub fn refund(
        &mut self,
        actual_time: u64,
        error_msg: Option<String,>,
    ) -> Result<SubMsg, ContractError,> {

        self.status = GameStatus::Refunded;

        self.updated_at = actual_time;

        self.error_message = error_msg;

        self.dice_results = None;

        let denom_token = TokenAmount {
            token: crate::token::Token::Denom(self.bet_currency.clone(),),
            amount: self.bet_amount,
        };

        let transfer_submsg = denom_token.token.transfer(&self.user_address, self.bet_amount,)?;

        Ok(transfer_submsg,)
    }
}

pub struct ExecuteContext<'a,> {
    pub deps: DepsMut<'a,>,
    pub env: Env,
    pub info: MessageInfo,
}

pub struct QueryContext<'a,> {
    pub deps: Deps<'a,>,
    pub env: Env,
}

impl ExecuteContext<'_,> {
    /// Top-level initialization of contract state

    pub fn instantiate(
        &mut self,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError,> {

        let InstantiateMsg { config, } = msg;

        config.save(&mut self.deps, self.info.sender.clone(),)?;

        let current_id = 0u64;

        CURRENT_ID.save(self.deps.storage, &current_id,)?;

        let not_resolved_bet_amount = Uint128::zero();

        NOT_RESOLVED_BET_AMOUNT.save(self.deps.storage, &not_resolved_bet_amount,)?;

        Ok(Response::new().add_attribute("action", "instantiate",),)
    }
}

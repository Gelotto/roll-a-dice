use cosmwasm_std::{
    StdError,
    Uint128,
};
use thiserror::Error;

#[derive(Debug, Error, PartialEq,)]

pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError,),

    #[error("NotAuthorized: {reason:?}")]
    NotAuthorized { reason: String, },

    #[error("ValidationError: {reason:?}")]
    ValidationError { reason: String, },

    #[error("DisabledError: Contract is not enabled")]
    DisabledError {},

    #[error("RandomConfigNotSet: Random config is not set")]
    RandomConfigNotSet {},

    #[error("InsufficientFunds: {denom_requested:?} {requested:?} > {available:?}")]
    InsufficientFunds {
        denom_requested: String,
        requested: Uint128,
        available: Uint128,
    },
    #[error("InvalidDiceResults: {reason:?}")]
    InvalidDiceResults { reason: String, },

    #[error("InvalidGameStatus: {reason:?}")]
    InvalidGameStatus { reason: String, },

    #[error("TooManyRecords: Limit {limit:?} exceeded")]
    TooManyRecords { limit: u8, },

    #[error("NotFound: {reason:?}")]
    NotFound { reason: String, },

    #[error("InvalidDiceNumber: {reason:?}")]
    InvalidDiceNumber { reason: String, },
}

impl From<ContractError,> for StdError {
    fn from(err: ContractError,) -> Self {

        StdError::generic_err(err.to_string(),)
    }
}

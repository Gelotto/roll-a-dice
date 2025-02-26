use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    Addr,
    DepsMut,
    Storage,
    Uint128,
    Uint64,
};
use cw_storage_plus::Item;

use crate::error::ContractError;

pub const FEE_PERCENTAGE: Item<Uint64,> = Item::new("fee_percentage",);

pub const RANDOM_CW_ADDRESS: Item<Addr,> = Item::new("random_cw_address",);

pub const ACCEPTED_DENOM: Item<String,> = Item::new("accepted_denom",);

pub const OPERATOR: Item<Addr,> = Item::new("operator",);

pub const MAX_BET: Item<Uint128,> = Item::new("max_bet",);

pub const DISABLED: Item<bool,> = Item::new("disabled",);

pub const MIN_BET: Item<Uint128,> = Item::new("min_bet",);

// GAS_LIMIT is used to set the gas limit for the random request ( it's the gas that the contract will use to handle the response, if it exceeds the gas limit, the receive randomness will fail)
pub const GAS_LIMIT: Item<Uint64,> = Item::new("gas_limit",);

#[cw_serde]

pub struct Config {
    pub fee_percentage: Uint64,
    pub random_cw_address: Addr,
    pub accepted_denom: String,
    pub operator: Option<Addr,>,
    pub disabled: bool,
    pub min_bet: Uint128,
    pub gas_limit: Uint64,
    // pub max_bet: u128,
}

// ConfigMsg which is the same as Config but having String instead of Uint64 and Uint128
#[cw_serde]

pub struct ConfigMsg {
    pub fee_percentage: String,
    pub random_cw_address: Addr,
    pub accepted_denom: String,
    pub operator: Option<Addr,>,
    pub disabled: bool,
    pub min_bet: String,
    pub gas_limit: String,
    // pub max_bet: String,
}

impl ConfigMsg {
    pub fn to_config(&self,) -> Result<Config, ContractError,> {

        Ok(Config {
            fee_percentage: self.fee_percentage.parse::<u64>().unwrap().into(),
            random_cw_address: self.random_cw_address.clone(),
            accepted_denom: self.accepted_denom.clone(),
            operator: self.operator.clone(),
            disabled: self.disabled,
            min_bet: self.min_bet.parse()?,
            gas_limit: self.gas_limit.parse::<u64>().unwrap().into(),
            // max_bet: self.max_bet.parse()?,
        },)
    }
}

impl Config {
    pub fn save(
        &self,
        deps_mut: &mut DepsMut,
        sender: Addr,
    ) -> Result<(), ContractError,> {

        FEE_PERCENTAGE.save(deps_mut.storage, &self.fee_percentage,)?;

        RANDOM_CW_ADDRESS.save(deps_mut.storage, &self.random_cw_address,)?;

        ACCEPTED_DENOM.save(deps_mut.storage, &self.accepted_denom,)?;

        let operator_to_save = self.operator.clone().unwrap_or(sender,);

        OPERATOR.save(deps_mut.storage, &operator_to_save,)?;

        DISABLED.save(deps_mut.storage, &self.disabled,)?;

        MIN_BET.save(deps_mut.storage, &self.min_bet,)?;

        GAS_LIMIT.save(deps_mut.storage, &self.gas_limit,)?;

        // MAX_BET.save(storage, &self.max_bet)?;
        Ok((),)
    }

    pub fn load(deps_mut: &mut DepsMut,) -> Result<Config, ContractError,> {

        let fee_percentage = FEE_PERCENTAGE.load(deps_mut.storage,)?;

        let random_cw_address = RANDOM_CW_ADDRESS.load(deps_mut.storage,)?;

        let accepted_denom = ACCEPTED_DENOM.load(deps_mut.storage,)?;

        let operator = OPERATOR.load(deps_mut.storage,)?;

        let disabled = DISABLED.load(deps_mut.storage,)?;

        let min_bet = MIN_BET.load(deps_mut.storage,)?;

        let gas_limit = GAS_LIMIT.load(deps_mut.storage,)?;

        // let max_bet = MAX_BET.load(storage)?;
        Ok(Config {
            fee_percentage,
            random_cw_address,
            accepted_denom,
            operator: Some(operator,),
            disabled,
            min_bet,
            gas_limit,
            // max_bet,
        },)
    }
}

pub fn get_current_config(storage: &dyn Storage,) -> Config {

    Config {
        fee_percentage: FEE_PERCENTAGE.load(storage,).unwrap(),
        random_cw_address: RANDOM_CW_ADDRESS.load(storage,).unwrap(),
        accepted_denom: ACCEPTED_DENOM.load(storage,).unwrap(),
        operator: Some(OPERATOR.load(storage,).unwrap(),),
        disabled: DISABLED.load(storage,).unwrap(),
        min_bet: MIN_BET.load(storage,).unwrap(),
        gas_limit: GAS_LIMIT.load(storage,).unwrap(),
    }
}

pub fn get_disabled(storage: &dyn Storage,) -> Result<bool, ContractError,> {

    Ok(DISABLED.load(storage,)?,)
}

pub fn get_fee_percentage(storage: &dyn Storage,) -> Result<u64, ContractError,> {

    Ok(FEE_PERCENTAGE.load(storage,)?.into(),)
}

pub fn get_random_cw_address(storage: &dyn Storage,) -> Result<Addr, ContractError,> {

    Ok(RANDOM_CW_ADDRESS.load(storage,)?,)
}

pub fn get_accepted_denom(storage: &dyn Storage,) -> Result<String, ContractError,> {

    Ok(ACCEPTED_DENOM.load(storage,)?,)
}

pub fn get_operator(storage: &dyn Storage,) -> Result<Addr, ContractError,> {

    Ok(OPERATOR.load(storage,)?,)
}

pub fn get_min_bet(storage: &dyn Storage,) -> Result<u128, ContractError,> {

    Ok(MIN_BET.load(storage,)?.into(),)
}

pub fn get_gas_limit(storage: &dyn Storage,) -> Result<u64, ContractError,> {

    Ok(GAS_LIMIT.load(storage,)?.into(),)
}

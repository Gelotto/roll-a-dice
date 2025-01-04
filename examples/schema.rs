use cosmwasm_schema::write_api;
use roll_a_dice::msg::{
    ExecuteMsg,
    InstantiateMsg,
    QueryMsg,
};

fn main() {

    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}

#[cfg(test)]

mod test_withdraw {

    use crate::test_utils::test_utils::*;
    use cosmwasm_std::{
        coins,
        Addr,
        Uint128,
    };
    use cw_multi_test::Executor;

    use roll_a_dice::error::ContractError;

    //TODO: test withdraw not authorized
    //TODO: test withdraw with game to resolve

    #[test]

    pub fn test_withdraw_not_authorized() {

        let not_owner_str = "not_owner".to_string();

        let owner_str = "owner".to_string();

        let amount = 100_000_000u128;

        let denom = "ujuno".to_string();

        let init_addr_vec = vec![not_owner_str.clone(), owner_str.clone()];

        let mut app = def_app(init_addr_vec, amount, denom.clone(),);

        let not_owner_address = Addr::unchecked(not_owner_str.clone(),);

        let owner_address = Addr::unchecked(owner_str.clone(),);

        let random_cw_config = get_random_cw_default_config(&owner_address,);

        let rand_code_id = test_cw_random_contract(&mut app,);

        let rand_contract_address =
            test_cw_random_instantiator(&mut app, rand_code_id, &owner_address, &random_cw_config,);

        let roll_a_dice_config = get_roll_dice_cw_default_config(&rand_contract_address, &owner_address,);

        let roll_dice_code_id = test_roll_dice_contract(&mut app,);

        let roll_dice_contract_address = test_roll_dice_instantiator(
            &mut app,
            roll_dice_code_id,
            &owner_address,
            &roll_a_dice_config,
            10_000_000,
            denom.clone(),
        );

        add_addresses_to_whitelist(
            &mut app,
            &rand_contract_address,
            &vec![roll_dice_contract_address.clone()],
            &owner_address,
        );

        //fetch randomcw config

        fetch_random_cw_config(&mut app, &roll_dice_contract_address, &owner_address,);

        let withdraw_exec_msg = get_withdraw_execute_msg(roll_a_dice_config.accepted_denom, 100_000,);

        let resp = app
            .execute_contract(
                not_owner_address.clone(),
                roll_dice_contract_address.clone(),
                &withdraw_exec_msg,
                &[],
            )
            .unwrap_err();

        print!("{:?}", resp.to_string());

        assert_eq!(
            ContractError::NotAuthorized {
                reason: "not authorized".to_string(),
            },
            resp.downcast().unwrap(),
        );
    }

    #[test]

    pub fn test_withdraw_with_game_to_resolve() {

        let not_owner_str = "not_owner".to_string();

        let owner_str = "owner".to_string();

        let amount = 100_000_000u128;

        let roll_a_dice_init_amount = 10_000_000u128;

        let denom = "ujuno".to_string();

        let init_addr_vec = vec![not_owner_str.clone(), owner_str.clone()];

        let mut app = def_app(init_addr_vec, amount, denom.clone(),);

        let not_owner_address = Addr::unchecked(not_owner_str.clone(),);

        let owner_address = Addr::unchecked(owner_str.clone(),);

        let random_cw_config = get_random_cw_default_config(&owner_address,);

        let rand_code_id = test_cw_random_contract(&mut app,);

        let rand_contract_address =
            test_cw_random_instantiator(&mut app, rand_code_id, &owner_address, &random_cw_config,);

        let roll_a_dice_config = get_roll_dice_cw_default_config(&rand_contract_address, &owner_address,);

        let roll_dice_code_id = test_roll_dice_contract(&mut app,);

        let roll_dice_contract_address = test_roll_dice_instantiator(
            &mut app,
            roll_dice_code_id,
            &owner_address,
            &roll_a_dice_config,
            roll_a_dice_init_amount,
            denom.clone(),
        );

        add_addresses_to_whitelist(
            &mut app,
            &rand_contract_address,
            &vec![roll_dice_contract_address.clone()],
            &owner_address,
        );

        //fetch randomcw config

        fetch_random_cw_config(&mut app, &roll_dice_contract_address, &owner_address,);

        let chosen_number = 6;

        let exact_number_request = get_play_request_exact_number_execute_msg(chosen_number,);

        //let exact_number_request = get_play_request_high_low_execute_msg(9);
        let bet_amount = 1_000_000u128;

        let resp = app
            .execute_contract(
                not_owner_address.clone(),
                roll_dice_contract_address.clone(),
                &exact_number_request,
                &coins(bet_amount.into(), denom.clone(),),
            )
            .unwrap();

        let cust_attr = resp.custom_attrs(1 as usize,);

        //get request_id from cust_attr
        let game_id = cust_attr[1].value.clone().parse::<u64>().unwrap();

        let cw_balance_pre_withdraw = app
            .wrap()
            .query_balance(roll_dice_contract_address.clone(), denom.clone(),)
            .unwrap();

        let withdraw_exec_msg = get_withdraw_execute_msg(
            roll_a_dice_config.accepted_denom.clone(),
            bet_amount + roll_a_dice_init_amount,
        );

        let owner_pre_withdraw_balance = app.wrap().query_balance(owner_address.clone(), denom.clone(),).unwrap();

        let _resp = app
            .execute_contract(
                owner_address.clone(),
                roll_dice_contract_address.clone(),
                &withdraw_exec_msg,
                &[],
            )
            .unwrap();

        let balance = app.wrap().query_balance(owner_address.clone(), denom.clone(),).unwrap();

        print!("{:?}", balance);

        assert_eq!(
            balance.amount,
            owner_pre_withdraw_balance.amount + Uint128::from(roll_a_dice_init_amount)
        );

        let cw_balance = app
            .wrap()
            .query_balance(roll_dice_contract_address.clone(), denom.clone(),)
            .unwrap();

        assert_eq!(
            cw_balance.amount,
            cw_balance_pre_withdraw.amount - Uint128::from(roll_a_dice_init_amount)
        );

        // send lost to unlock the other amount
        send_lose_exact_number_game(
            &mut app,
            &rand_contract_address,
            &roll_dice_contract_address,
            game_id,
            chosen_number,
        );

        let withdraw_exec_msg =
            get_withdraw_execute_msg(roll_a_dice_config.accepted_denom, bet_amount + roll_a_dice_init_amount,);

        let owner_pre_withdraw_balance = app.wrap().query_balance(owner_address.clone(), denom.clone(),).unwrap();

        let _resp = app
            .execute_contract(
                owner_address.clone(),
                roll_dice_contract_address.clone(),
                &withdraw_exec_msg,
                &[],
            )
            .unwrap();

        let balance = app.wrap().query_balance(owner_address.clone(), denom.clone(),).unwrap();

        print!("{:?}", balance);

        assert_eq!(
            balance.amount,
            owner_pre_withdraw_balance.amount + Uint128::from(cw_balance.amount)
        );

        let cw_balance = app
            .wrap()
            .query_balance(roll_dice_contract_address.clone(), denom.clone(),)
            .unwrap();

        assert_eq!(cw_balance.amount, Uint128::from(0u128));
    }
}

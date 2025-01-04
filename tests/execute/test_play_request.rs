#[cfg(test)]

mod test_request {

    use crate::test_utils::test_utils::*;
    use cosmwasm_std::coins;
    use cosmwasm_std::{
        Addr,
        Uint128,
    };
    use cw_multi_test::Executor;

    use roll_a_dice::error::ContractError;

    use roll_a_dice::state::{
        GameStatus,
        GameType,
    };

    #[test]

    fn test_exec_request_ok() {

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

        //create a request

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

        print!("{:?}", cust_attr);

        //get request_id from cust_attr
        let req_id = cust_attr[1].value.clone();

        print!("{:?}", req_id);

        advance_height_and_trigger_randomness_generation(&mut app, &rand_contract_address, &owner_address,);

        let game_req =
            query_roll_dice_game_request(&mut app, &roll_dice_contract_address, req_id.parse::<u64>().unwrap(),);

        print!("{:?}", game_req);

        assert!(game_req.error_message.is_none());

        assert!(game_req.dice_results.is_some());

        assert!(game_req.status != GameStatus::Requested);

        assert!(game_req.status != GameStatus::Error);

        assert!(game_req.status != GameStatus::Refunded);

        let game_type = game_req.game_type;

        let number_dice_requested = match game_type {
            GameType::ExactNumber(exact_number_game,) => exact_number_game.get_number_of_requests(),
            GameType::HighLow(high_low_game,) => high_low_game.get_number_of_requests(),
        };

        assert_eq!(game_req.dice_results.unwrap().len(), number_dice_requested as usize);

        let high_low_request = get_play_request_high_low_execute_msg(9,);

        let bet_amount = 1_000_000u128;

        let resp = app
            .execute_contract(
                not_owner_address.clone(),
                roll_dice_contract_address.clone(),
                &high_low_request,
                &coins(bet_amount.into(), denom.clone(),),
            )
            .unwrap();

        let cust_attr = resp.custom_attrs(1 as usize,);

        print!("{:?}", cust_attr);

        //get request_id from cust_attr
        let req_id = cust_attr[1].value.clone();

        print!("{:?}", req_id);

        advance_height_and_trigger_randomness_generation(&mut app, &rand_contract_address, &owner_address,);

        let game_req =
            query_roll_dice_game_request(&mut app, &roll_dice_contract_address, req_id.parse::<u64>().unwrap(),);

        print!("{:?}", game_req);

        assert!(game_req.error_message.is_none());

        assert!(game_req.dice_results.is_some());

        assert!(game_req.status != GameStatus::Requested);

        assert!(game_req.status != GameStatus::Error);

        assert!(game_req.status != GameStatus::Refunded);

        let game_type = game_req.game_type;

        let number_dice_requested = match game_type {
            GameType::ExactNumber(exact_number_game,) => exact_number_game.get_number_of_requests(),
            GameType::HighLow(high_low_game,) => high_low_game.get_number_of_requests(),
        };

        assert_eq!(game_req.dice_results.unwrap().len(), number_dice_requested as usize);

        let balance = app
            .wrap()
            .query_balance(roll_dice_contract_address, denom.clone(),)
            .unwrap();

        print!("{:?}", balance);
    }

    // Test all request errors
    #[test]

    pub fn test_disabled_contract() {

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

        let mut roll_a_dice_config = get_roll_dice_cw_default_config(&rand_contract_address, &owner_address,);

        roll_a_dice_config.disabled = true;

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
            .unwrap_err();

        assert_eq!(ContractError::DisabledError {}, resp.downcast().unwrap());
    }

    // Test all request errors
    #[test]

    pub fn test_random_cw_cfg_not_fetched() {

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

        //   fetch_random_cw_config(&mut app, &roll_dice_contract_address, &owner_address,);

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
            .unwrap_err();

        assert_eq!(ContractError::RandomConfigNotSet {}, resp.downcast().unwrap());
    }

    #[test]

    pub fn test_not_valid_high_low_game() {

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

        //   let chosen_number = 6;
        //   let exact_number_request = get_play_request_exact_number_execute_msg(chosen_number,);
        let not_valid_number_of_dices = 0;

        let exact_number_request = get_play_request_high_low_execute_msg(not_valid_number_of_dices,);

        let bet_amount = 1_000_000u128;

        let resp = app
            .execute_contract(
                not_owner_address.clone(),
                roll_dice_contract_address.clone(),
                &exact_number_request,
                &coins(bet_amount.into(), denom.clone(),),
            )
            .unwrap_err();

        assert_eq!(
            ContractError::InvalidDiceNumber {
                reason: "Invalid dice number".to_string()
            },
            resp.downcast().unwrap()
        );

        let not_valid_number_of_dices = 2;

        let exact_number_request = get_play_request_high_low_execute_msg(not_valid_number_of_dices,);

        let bet_amount = 1_000_000u128;

        let resp = app
            .execute_contract(
                not_owner_address.clone(),
                roll_dice_contract_address.clone(),
                &exact_number_request,
                &coins(bet_amount.into(), denom.clone(),),
            )
            .unwrap_err();

        assert_eq!(
            ContractError::InvalidDiceNumber {
                reason: "Invalid dice number".to_string()
            },
            resp.downcast().unwrap()
        );
    }

    #[test]

    pub fn test_not_valid_exact_number_game() {

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

        let chosen_number = 1;

        let exact_number_request = get_play_request_exact_number_execute_msg(chosen_number,);

        let bet_amount = 1_000_000u128;

        let resp = app
            .execute_contract(
                not_owner_address.clone(),
                roll_dice_contract_address.clone(),
                &exact_number_request,
                &coins(bet_amount.into(), denom.clone(),),
            )
            .unwrap_err();

        assert_eq!(
            ContractError::InvalidDiceNumber {
                reason: "Invalid dice number".to_string()
            },
            resp.downcast().unwrap()
        );

        let chosen_number = 13;

        let exact_number_request = get_play_request_exact_number_execute_msg(chosen_number,);

        let bet_amount = 1_000_000u128;

        let resp = app
            .execute_contract(
                not_owner_address.clone(),
                roll_dice_contract_address.clone(),
                &exact_number_request,
                &coins(bet_amount.into(), denom.clone(),),
            )
            .unwrap_err();

        assert_eq!(
            ContractError::InvalidDiceNumber {
                reason: "Invalid dice number".to_string()
            },
            resp.downcast().unwrap()
        );
    }

    #[test]

    pub fn test_not_at_least_minimum_bet_amount() {

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

        let chosen_number = 2;

        let exact_number_request = get_play_request_exact_number_execute_msg(chosen_number,);

        let bet_amount = roll_a_dice_config.min_bet - 1;

        let resp = app
            .execute_contract(
                not_owner_address.clone(),
                roll_dice_contract_address.clone(),
                &exact_number_request,
                &coins(bet_amount.into(), denom.clone(),),
            )
            .unwrap_err();

        assert_eq!(
            ContractError::InsufficientFunds {
                denom_requested: denom.clone(),
                requested: Uint128::from(roll_a_dice_config.min_bet),
                available: Uint128::from(bet_amount),
            },
            resp.downcast().unwrap()
        );
    }

    #[test]

    pub fn test_error_random_cw_request_refund() {

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

        //   add_addresses_to_whitelist(&mut app, &rand_contract_address, &vec![roll_dice_contract_address.clone(),], &owner_address,);

        //fetch randomcw config

        fetch_random_cw_config(&mut app, &roll_dice_contract_address, &owner_address,);

        let chosen_number = 2;

        let exact_number_request = get_play_request_exact_number_execute_msg(chosen_number,);

        let bet_amount = roll_a_dice_config.min_bet;

        let resp = app
            .execute_contract(
                not_owner_address.clone(),
                roll_dice_contract_address.clone(),
                &exact_number_request,
                &coins(bet_amount.into(), denom.clone(),),
            )
            .unwrap();

        let cust_attr = resp.custom_attrs(1 as usize,);

        print!("{:?}", cust_attr);

        //get request_id from cust_attr
        let req_id = cust_attr[1].value.clone();

        print!("{:?}", req_id);

        //   advance_height_and_trigger_randomness_generation(&mut app, &rand_contract_address, &owner_address,);

        let game_req =
            query_roll_dice_game_request(&mut app, &roll_dice_contract_address, req_id.parse::<u64>().unwrap(),);

        print!("{:?}", game_req);

        assert!(game_req.error_message.is_some());

        assert!(game_req.dice_results.is_none());

        assert!(game_req.status == GameStatus::Refunded);

        let balance = app.wrap().query_balance(not_owner_address, denom.clone(),).unwrap();

        print!("{:?}", balance);

        assert_eq!(balance.amount, Uint128::from(amount - bet_amount) + game_req.bet_amount);
    }

    #[test]

    pub fn test_not_enough_coin_to_pay_the_win_refund() {

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
            500_000,
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

        let chosen_number = 2;

        let exact_number_request = get_play_request_exact_number_execute_msg(chosen_number,);

        let bet_amount = roll_a_dice_config.min_bet;

        let resp = app
            .execute_contract(
                not_owner_address.clone(),
                roll_dice_contract_address.clone(),
                &exact_number_request,
                &coins(bet_amount.into(), denom.clone(),),
            )
            .unwrap();

        let cust_attr = resp.custom_attrs(1 as usize,);

        print!("{:?}", cust_attr);

        //get request_id from cust_attr
        let req_id = cust_attr[1].value.clone();

        print!("{:?}", req_id);

        // send manual win to the roll_dice_cw contract

        send_win_exact_number_game(
            &mut app,
            &rand_contract_address,
            &roll_dice_contract_address,
            req_id.parse::<u64>().unwrap(),
            chosen_number,
        );

        //   advance_height_and_trigger_randomness_generation(&mut app, &rand_contract_address, &owner_address,);

        let game_req =
            query_roll_dice_game_request(&mut app, &roll_dice_contract_address, req_id.parse::<u64>().unwrap(),);

        print!("{:?}", game_req);

        assert!(game_req.error_message.is_some());

        assert_eq!(game_req.error_message.unwrap(), "Not enough balance -> refunding");

        assert!(game_req.dice_results.is_none());

        assert!(game_req.status == GameStatus::Refunded);

        let balance = app.wrap().query_balance(not_owner_address, denom.clone(),).unwrap();

        print!("{:?}", balance);

        assert_eq!(balance.amount, Uint128::from(amount - bet_amount) + game_req.bet_amount);

        let cw_balance = app
            .wrap()
            .query_balance(roll_dice_contract_address, denom.clone(),)
            .unwrap();

        assert_eq!(cw_balance.amount, Uint128::from(500_000u128));
    }

    //TODO tests: wrong_type_results -> refund
    #[test]

    pub fn test_wrong_job_result_type() {

        let not_owner_str = "not_owner".to_string();

        let owner_str = "owner".to_string();

        let amount = 100_000_000u128;

        let roll_dice_init_amount = 10_000_000u128;

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
            roll_dice_init_amount,
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

        let chosen_number = 2;

        let exact_number_request = get_play_request_exact_number_execute_msg(chosen_number,);

        let bet_amount = roll_a_dice_config.min_bet;

        let resp = app
            .execute_contract(
                not_owner_address.clone(),
                roll_dice_contract_address.clone(),
                &exact_number_request,
                &coins(bet_amount.into(), denom.clone(),),
            )
            .unwrap();

        let cust_attr = resp.custom_attrs(1 as usize,);

        print!("{:?}", cust_attr);

        //get request_id from cust_attr
        let req_id = cust_attr[1].value.clone();

        print!("{:?}", req_id);

        // send wrong type results to the roll_dice_cw contract

        send_wrong_type_randomness(
            &mut app,
            &rand_contract_address,
            &roll_dice_contract_address,
            req_id.parse::<u64>().unwrap(),
            2,
        );

        //   advance_height_and_trigger_randomness_generation(&mut app, &rand_contract_address, &owner_address,);

        let game_req =
            query_roll_dice_game_request(&mut app, &roll_dice_contract_address, req_id.parse::<u64>().unwrap(),);

        print!("{:?}", game_req);

        assert!(game_req.error_message.is_some());

        assert_eq!(game_req.error_message.unwrap(), "Invalid Randomness Results");

        assert!(game_req.dice_results.is_none());

        assert!(game_req.status == GameStatus::Refunded);

        let balance = app.wrap().query_balance(not_owner_address, denom.clone(),).unwrap();

        print!("{:?}", balance);

        assert_eq!(balance.amount, Uint128::from(amount - bet_amount) + game_req.bet_amount);

        let cw_balance = app
            .wrap()
            .query_balance(roll_dice_contract_address, denom.clone(),)
            .unwrap();

        assert_eq!(cw_balance.amount, Uint128::from(roll_dice_init_amount));
    }

    //TODO test: empty_results -> refund

    #[test]

    pub fn test_empty_results_refund() {

        let not_owner_str = "not_owner".to_string();

        let owner_str = "owner".to_string();

        let amount = 100_000_000u128;

        let roll_dice_init_amount = 10_000_000u128;

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
            roll_dice_init_amount,
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

        let chosen_number = 2;

        let exact_number_request = get_play_request_exact_number_execute_msg(chosen_number,);

        let bet_amount = roll_a_dice_config.min_bet;

        let resp = app
            .execute_contract(
                not_owner_address.clone(),
                roll_dice_contract_address.clone(),
                &exact_number_request,
                &coins(bet_amount.into(), denom.clone(),),
            )
            .unwrap();

        let cust_attr = resp.custom_attrs(1 as usize,);

        print!("{:?}", cust_attr);

        //get request_id from cust_attr
        let req_id = cust_attr[1].value.clone();

        print!("{:?}", req_id);

        // send empty results to the roll_dice_cw contract

        send_empty_randomness(
            &mut app,
            &rand_contract_address,
            &roll_dice_contract_address,
            req_id.parse::<u64>().unwrap(),
        );

        //   advance_height_and_trigger_randomness_generation(&mut app, &rand_contract_address, &owner_address,);

        let game_req =
            query_roll_dice_game_request(&mut app, &roll_dice_contract_address, req_id.parse::<u64>().unwrap(),);

        print!("{:?}", game_req);

        assert!(game_req.error_message.is_some());

        assert_eq!(game_req.error_message.unwrap(), "Invalid Randomness Results");

        assert!(game_req.dice_results.is_none());

        assert!(game_req.status == GameStatus::Refunded);

        let balance = app.wrap().query_balance(not_owner_address, denom.clone(),).unwrap();

        print!("{:?}", balance);

        assert_eq!(balance.amount, Uint128::from(amount - bet_amount) + game_req.bet_amount);

        let cw_balance = app
            .wrap()
            .query_balance(roll_dice_contract_address, denom.clone(),)
            .unwrap();

        assert_eq!(cw_balance.amount, Uint128::from(roll_dice_init_amount));
    }

    //TODO tests: invalid dice number results -> refund
    #[test]

    pub fn test_invalid_dice_number_results_refund() {

        let not_owner_str = "not_owner".to_string();

        let owner_str = "owner".to_string();

        let amount = 100_000_000u128;

        let roll_dice_init_amount = 10_000_000u128;

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
            roll_dice_init_amount,
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

        let chosen_number = 2;

        let exact_number_request = get_play_request_exact_number_execute_msg(chosen_number,);

        let bet_amount = roll_a_dice_config.min_bet;

        let resp = app
            .execute_contract(
                not_owner_address.clone(),
                roll_dice_contract_address.clone(),
                &exact_number_request,
                &coins(bet_amount.into(), denom.clone(),),
            )
            .unwrap();

        let cust_attr = resp.custom_attrs(1 as usize,);

        print!("{:?}", cust_attr);

        //get request_id from cust_attr
        let req_id = cust_attr[1].value.clone();

        print!("{:?}", req_id);

        // send invalid results to the roll_dice_cw contract
        send_invalid_dice_number(
            &mut app,
            &rand_contract_address,
            &roll_dice_contract_address,
            req_id.parse::<u64>().unwrap(),
            2,
        );

        //   advance_height_and_trigger_randomness_generation(&mut app, &rand_contract_address, &owner_address,);

        let game_req =
            query_roll_dice_game_request(&mut app, &roll_dice_contract_address, req_id.parse::<u64>().unwrap(),);

        print!("{:?}", game_req);

        assert!(game_req.error_message.is_some());

        assert_eq!(
            game_req.error_message.unwrap(),
            ContractError::InvalidDiceResults {
                reason: "Invalid dice result".to_string()
            }
            .to_string()
        );

        assert!(game_req.dice_results.is_none());

        assert!(game_req.status == GameStatus::Refunded);

        let balance = app
            .wrap()
            .query_balance(not_owner_address.clone(), denom.clone(),)
            .unwrap();

        print!("{:?}", balance);

        assert_eq!(balance.amount, Uint128::from(amount - bet_amount) + game_req.bet_amount);

        let cw_balance = app
            .wrap()
            .query_balance(roll_dice_contract_address.clone(), denom.clone(),)
            .unwrap();

        assert_eq!(cw_balance.amount, Uint128::from(roll_dice_init_amount));

        let number_of_dices = 9;

        let high_low_request = get_play_request_high_low_execute_msg(number_of_dices,);

        let resp = app
            .execute_contract(
                not_owner_address.clone(),
                roll_dice_contract_address.clone(),
                &high_low_request,
                &coins(bet_amount.into(), denom.clone(),),
            )
            .unwrap();

        let cust_attr = resp.custom_attrs(1 as usize,);

        print!("{:?}", cust_attr);

        //get request_id from cust_attr
        let new_req_id = cust_attr[1].value.clone();

        assert_eq!(
            req_id.parse::<u64>().unwrap() + 1u64,
            new_req_id.parse::<u64>().unwrap()
        );

        print!("{:?}", new_req_id);

        send_invalid_dice_number(
            &mut app,
            &rand_contract_address,
            &roll_dice_contract_address,
            new_req_id.parse::<u64>().unwrap(),
            number_of_dices.into(),
        );

        let game_req = query_roll_dice_game_request(
            &mut app,
            &roll_dice_contract_address,
            new_req_id.parse::<u64>().unwrap(),
        );

        print!("{:?}", game_req);

        assert!(game_req.error_message.is_some());

        assert_eq!(
            game_req.error_message.unwrap(),
            ContractError::InvalidDiceResults {
                reason: "Invalid dice result".to_string()
            }
            .to_string()
        );

        assert!(game_req.dice_results.is_none());

        assert!(game_req.status == GameStatus::Refunded);

        let new_balance = app
            .wrap()
            .query_balance(not_owner_address.clone(), denom.clone(),)
            .unwrap();

        print!("{:?}", new_balance);

        assert_eq!(
            new_balance.amount,
            balance.amount - Uint128::from(bet_amount) + game_req.bet_amount
        );

        let cw_balance = app
            .wrap()
            .query_balance(roll_dice_contract_address.clone(), denom.clone(),)
            .unwrap();

        assert_eq!(cw_balance.amount, Uint128::from(roll_dice_init_amount));
    }

    //TODO: high_low win -> transfer
    #[test]

    pub fn test_high_low_win() {

        let not_owner_str = "not_owner".to_string();

        let owner_str = "owner".to_string();

        let amount = 100_000_000u128;

        let roll_dice_init_amount = 10_000_000u128;

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
            roll_dice_init_amount,
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

        let number_of_dices = 9;

        let high_low_request = get_play_request_high_low_execute_msg(number_of_dices,);

        let bet_amount = roll_a_dice_config.min_bet;

        let resp = app
            .execute_contract(
                not_owner_address.clone(),
                roll_dice_contract_address.clone(),
                &high_low_request,
                &coins(bet_amount.into(), denom.clone(),),
            )
            .unwrap();

        let cust_attr = resp.custom_attrs(1 as usize,);

        print!("{:?}", cust_attr);

        //get request_id from cust_attr
        let req_id = cust_attr[1].value.clone();

        print!("{:?}", req_id);

        // send manual win to the roll_dice_cw contract
        send_win_high_low_game(
            &mut app,
            &rand_contract_address,
            &roll_dice_contract_address,
            req_id.parse::<u64>().unwrap(),
            number_of_dices.into(),
        );

        //   advance_height_and_trigger_randomness_generation(&mut app, &rand_contract_address, &owner_address,);

        let game_req =
            query_roll_dice_game_request(&mut app, &roll_dice_contract_address, req_id.parse::<u64>().unwrap(),);

        print!("{:?}", game_req);

        assert!(game_req.error_message.is_none());

        assert!(game_req.dice_results.is_some());

        assert!(game_req.status == GameStatus::Won);

        let balance = app
            .wrap()
            .query_balance(not_owner_address.clone(), denom.clone(),)
            .unwrap();

        print!("{:?}", balance);

        assert_eq!(
            balance.amount,
            Uint128::from(amount - bet_amount) + game_req.potential_winning_amount
        );

        let cw_balance = app
            .wrap()
            .query_balance(roll_dice_contract_address.clone(), denom.clone(),)
            .unwrap();

        assert_eq!(
            cw_balance.amount,
            Uint128::from(roll_dice_init_amount) - game_req.potential_winning_amount + game_req.bet_amount
        );
    }

    //TODO: high_low lose
    #[test]

    pub fn test_high_low_lose() {

        let not_owner_str = "not_owner".to_string();

        let owner_str = "owner".to_string();

        let amount = 100_000_000u128;

        let roll_dice_init_amount = 10_000_000u128;

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
            roll_dice_init_amount,
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

        let number_of_dices = 9;

        let high_low_request = get_play_request_high_low_execute_msg(number_of_dices,);

        let bet_amount = roll_a_dice_config.min_bet;

        let resp = app
            .execute_contract(
                not_owner_address.clone(),
                roll_dice_contract_address.clone(),
                &high_low_request,
                &coins(bet_amount.into(), denom.clone(),),
            )
            .unwrap();

        let cust_attr = resp.custom_attrs(1 as usize,);

        print!("{:?}", cust_attr);

        //get request_id from cust_attr
        let req_id = cust_attr[1].value.clone();

        print!("{:?}", req_id);

        // send manual win to the roll_dice_cw contract
        send_lose_high_low_game(
            &mut app,
            &rand_contract_address,
            &roll_dice_contract_address,
            req_id.parse::<u64>().unwrap(),
            number_of_dices.into(),
        );

        //   advance_height_and_trigger_randomness_generation(&mut app, &rand_contract_address, &owner_address,);

        let game_req =
            query_roll_dice_game_request(&mut app, &roll_dice_contract_address, req_id.parse::<u64>().unwrap(),);

        print!("{:?}", game_req);

        assert!(game_req.error_message.is_none());

        assert!(game_req.dice_results.is_some());

        assert!(game_req.status == GameStatus::Lost);

        let balance = app
            .wrap()
            .query_balance(not_owner_address.clone(), denom.clone(),)
            .unwrap();

        print!("{:?}", balance);

        assert_eq!(balance.amount, Uint128::from(amount - bet_amount));

        let cw_balance = app
            .wrap()
            .query_balance(roll_dice_contract_address.clone(), denom.clone(),)
            .unwrap();

        assert_eq!(
            cw_balance.amount,
            Uint128::from(roll_dice_init_amount) + game_req.bet_amount
        );
    }

    //TODO: exact_number win -> transfer
    #[test]

    pub fn test_exact_number_win() {

        let not_owner_str = "not_owner".to_string();

        let owner_str = "owner".to_string();

        let amount = 100_000_000u128;

        let roll_dice_init_amount = 10_000_000u128;

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
            roll_dice_init_amount,
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

        let chosen_number = 2;

        let exact_number_request = get_play_request_exact_number_execute_msg(chosen_number,);

        let bet_amount = roll_a_dice_config.min_bet;

        let resp = app
            .execute_contract(
                not_owner_address.clone(),
                roll_dice_contract_address.clone(),
                &exact_number_request,
                &coins(bet_amount.into(), denom.clone(),),
            )
            .unwrap();

        let cust_attr = resp.custom_attrs(1 as usize,);

        print!("{:?}", cust_attr);

        //get request_id from cust_attr
        let req_id = cust_attr[1].value.clone();

        print!("{:?}", req_id);

        // send manual win to the roll_dice_cw contract
        send_win_exact_number_game(
            &mut app,
            &rand_contract_address,
            &roll_dice_contract_address,
            req_id.parse::<u64>().unwrap(),
            chosen_number,
        );

        //   advance_height_and_trigger_randomness_generation(&mut app, &rand_contract_address, &owner_address,);

        let game_req =
            query_roll_dice_game_request(&mut app, &roll_dice_contract_address, req_id.parse::<u64>().unwrap(),);

        print!("{:?}", game_req);

        assert!(game_req.error_message.is_none());

        assert!(game_req.dice_results.is_some());

        assert!(game_req.status == GameStatus::Won);

        let balance = app
            .wrap()
            .query_balance(not_owner_address.clone(), denom.clone(),)
            .unwrap();

        print!("{:?}", balance);

        assert_eq!(
            balance.amount,
            Uint128::from(amount - bet_amount) + game_req.potential_winning_amount
        );

        let cw_balance = app
            .wrap()
            .query_balance(roll_dice_contract_address.clone(), denom.clone(),)
            .unwrap();

        assert_eq!(
            cw_balance.amount,
            Uint128::from(roll_dice_init_amount) - game_req.potential_winning_amount + game_req.bet_amount
        );
    }

    //TODO: exact_number lose
    #[test]

    pub fn test_exact_number_lose() {

        let not_owner_str = "not_owner".to_string();

        let owner_str = "owner".to_string();

        let amount = 100_000_000u128;

        let roll_dice_init_amount = 10_000_000u128;

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
            roll_dice_init_amount,
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

        let chosen_number = 2;

        let high_low_request = get_play_request_exact_number_execute_msg(chosen_number,);

        let bet_amount = roll_a_dice_config.min_bet;

        let resp = app
            .execute_contract(
                not_owner_address.clone(),
                roll_dice_contract_address.clone(),
                &high_low_request,
                &coins(bet_amount.into(), denom.clone(),),
            )
            .unwrap();

        let cust_attr = resp.custom_attrs(1 as usize,);

        print!("{:?}", cust_attr);

        //get request_id from cust_attr
        let req_id = cust_attr[1].value.clone();

        print!("{:?}", req_id);

        // send manual win to the roll_dice_cw contract
        send_lose_exact_number_game(
            &mut app,
            &rand_contract_address,
            &roll_dice_contract_address,
            req_id.parse::<u64>().unwrap(),
            chosen_number.into(),
        );

        //   advance_height_and_trigger_randomness_generation(&mut app, &rand_contract_address, &owner_address,);

        let game_req =
            query_roll_dice_game_request(&mut app, &roll_dice_contract_address, req_id.parse::<u64>().unwrap(),);

        print!("{:?}", game_req);

        assert!(game_req.error_message.is_none());

        assert!(game_req.dice_results.is_some());

        assert!(game_req.status == GameStatus::Lost);

        let balance = app
            .wrap()
            .query_balance(not_owner_address.clone(), denom.clone(),)
            .unwrap();

        print!("{:?}", balance);

        assert_eq!(balance.amount, Uint128::from(amount - bet_amount));

        let cw_balance = app
            .wrap()
            .query_balance(roll_dice_contract_address.clone(), denom.clone(),)
            .unwrap();

        assert_eq!(
            cw_balance.amount,
            Uint128::from(roll_dice_init_amount) + game_req.bet_amount
        );
    }

    //Not authorized randomness
    #[test]

    pub fn test_not_authorized_randomness() {

        let not_owner_str = "not_owner".to_string();

        let owner_str = "owner".to_string();

        let amount = 100_000_000u128;

        let roll_dice_init_amount = 10_000_000u128;

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
            roll_dice_init_amount,
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

        let chosen_number = 2;

        let high_low_request = get_play_request_exact_number_execute_msg(chosen_number,);

        let bet_amount = roll_a_dice_config.min_bet;

        let resp = app
            .execute_contract(
                not_owner_address.clone(),
                roll_dice_contract_address.clone(),
                &high_low_request,
                &coins(bet_amount.into(), denom.clone(),),
            )
            .unwrap();

        let cust_attr = resp.custom_attrs(1 as usize,);

        print!("{:?}", cust_attr);

        //get request_id from cust_attr
        let req_id = cust_attr[1].value.clone();

        print!("{:?}", req_id);

        // send manual win to the roll_dice_cw contract
        let result = send_randomness(
            &mut app,
            &owner_address,
            &roll_dice_contract_address,
            req_id.parse::<u64>().unwrap(),
            2,
        );

        //   advance_height_and_trigger_randomness_generation(&mut app, &rand_contract_address, &owner_address,);

        assert_eq!(
            ContractError::NotAuthorized {
                reason: "Only random cw address can call this".to_string()
            },
            result.unwrap_err().downcast().unwrap(),
        );

        let game_req =
            query_roll_dice_game_request(&mut app, &roll_dice_contract_address, req_id.parse::<u64>().unwrap(),);

        print!("{:?}", game_req);

        assert!(game_req.error_message.is_none());

        assert!(game_req.dice_results.is_none());

        assert!(game_req.status == GameStatus::Requested);

        let balance = app
            .wrap()
            .query_balance(not_owner_address.clone(), denom.clone(),)
            .unwrap();

        print!("{:?}", balance);

        assert_eq!(balance.amount, Uint128::from(amount - bet_amount));

        let cw_balance = app
            .wrap()
            .query_balance(roll_dice_contract_address.clone(), denom.clone(),)
            .unwrap();

        assert_eq!(
            cw_balance.amount,
            Uint128::from(roll_dice_init_amount) + game_req.bet_amount
        );
    }

    //TODO: GameRequest not in requested status
    #[test]

    pub fn test_resend_randomness_to_a_completed_game() {

        let not_owner_str = "not_owner".to_string();

        let owner_str = "owner".to_string();

        let amount = 100_000_000u128;

        let roll_dice_init_amount = 10_000_000u128;

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
            roll_dice_init_amount,
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

        let chosen_number = 2;

        let high_low_request = get_play_request_exact_number_execute_msg(chosen_number,);

        let bet_amount = roll_a_dice_config.min_bet;

        let resp = app
            .execute_contract(
                not_owner_address.clone(),
                roll_dice_contract_address.clone(),
                &high_low_request,
                &coins(bet_amount.into(), denom.clone(),),
            )
            .unwrap();

        let cust_attr = resp.custom_attrs(1 as usize,);

        print!("{:?}", cust_attr);

        //get request_id from cust_attr
        let req_id = cust_attr[1].value.clone();

        print!("{:?}", req_id);

        send_lose_exact_number_game(
            &mut app,
            &rand_contract_address,
            &roll_dice_contract_address,
            req_id.parse::<u64>().unwrap(),
            2,
        );

        // send manual win to the roll_dice_cw contract
        let result = send_randomness(
            &mut app,
            &rand_contract_address,
            &roll_dice_contract_address,
            req_id.parse::<u64>().unwrap(),
            2,
        );

        //   advance_height_and_trigger_randomness_generation(&mut app, &rand_contract_address, &owner_address,);

        assert_eq!(
            ContractError::InvalidGameStatus {
                reason: "Game status is not in Requested State".to_string()
            },
            result.unwrap_err().downcast().unwrap(),
        );

        let game_req =
            query_roll_dice_game_request(&mut app, &roll_dice_contract_address, req_id.parse::<u64>().unwrap(),);

        print!("{:?}", game_req);

        assert!(game_req.error_message.is_none());

        assert!(game_req.dice_results.is_some());

        assert!(game_req.status == GameStatus::Lost);

        let balance = app
            .wrap()
            .query_balance(not_owner_address.clone(), denom.clone(),)
            .unwrap();

        print!("{:?}", balance);

        assert_eq!(balance.amount, Uint128::from(amount - bet_amount));

        let cw_balance = app
            .wrap()
            .query_balance(roll_dice_contract_address.clone(), denom.clone(),)
            .unwrap();

        assert_eq!(
            cw_balance.amount,
            Uint128::from(roll_dice_init_amount) + game_req.bet_amount
        );
    }

    //TODO: Check for NOT resolved bet amount

    #[test]

    pub fn test_not_resolved_bet_amount_correctly_working() {

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

        let contract_init_amount = 500_000u128;

        let rand_contract_address =
            test_cw_random_instantiator(&mut app, rand_code_id, &owner_address, &random_cw_config,);

        let roll_a_dice_config = get_roll_dice_cw_default_config(&rand_contract_address, &owner_address,);

        let roll_dice_code_id = test_roll_dice_contract(&mut app,);

        let roll_dice_contract_address = test_roll_dice_instantiator(
            &mut app,
            roll_dice_code_id,
            &owner_address,
            &roll_a_dice_config,
            contract_init_amount,
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

        let chosen_number = 2;

        let exact_number_request = get_play_request_exact_number_execute_msg(chosen_number,);

        let bet_amount_1 = roll_a_dice_config.min_bet;

        let resp = app
            .execute_contract(
                not_owner_address.clone(),
                roll_dice_contract_address.clone(),
                &exact_number_request,
                &coins(bet_amount_1.into(), denom.clone(),),
            )
            .unwrap();

        let cust_attr = resp.custom_attrs(1 as usize,);

        print!("{:?}", cust_attr);

        //get request_id from cust_attr
        let req_id_1 = cust_attr[1].value.clone().parse::<u64>().unwrap();

        let bet_amount_2 = roll_a_dice_config.min_bet * 3u128;

        let resp = app
            .execute_contract(
                not_owner_address.clone(),
                roll_dice_contract_address.clone(),
                &exact_number_request,
                &coins(bet_amount_2.into(), denom.clone(),),
            )
            .unwrap();

        let cust_attr = resp.custom_attrs(1 as usize,);

        print!("{:?}", cust_attr);

        //get request_id from cust_attr
        let req_id_2 = cust_attr[1].value.clone().parse::<u64>().unwrap();

        let bet_amount_3 = roll_a_dice_config.min_bet;

        let resp = app
            .execute_contract(
                not_owner_address.clone(),
                roll_dice_contract_address.clone(),
                &exact_number_request,
                &coins(bet_amount_3.into(), denom.clone(),),
            )
            .unwrap();

        let cust_attr = resp.custom_attrs(1 as usize,);

        print!("{:?}", cust_attr);

        //get request_id from cust_attr
        let req_id_3 = cust_attr[1].value.clone().parse::<u64>().unwrap();

        // send manual win to the roll_dice_cw contract
        let cw_balance = app
            .wrap()
            .query_balance(roll_dice_contract_address.clone(), denom.clone(),)
            .unwrap();

        send_win_exact_number_game(
            &mut app,
            &rand_contract_address,
            &roll_dice_contract_address,
            req_id_1,
            chosen_number,
        );

        let game_req = query_roll_dice_game_request(&mut app, &roll_dice_contract_address, req_id_1,);

        print!("{:?}", game_req);

        assert!(game_req.error_message.is_some());

        assert_eq!(game_req.error_message.unwrap(), "Not enough balance -> refunding");

        assert!(game_req.dice_results.is_none());

        assert!(game_req.status == GameStatus::Refunded);

        let balance = app
            .wrap()
            .query_balance(not_owner_address.clone(), denom.clone(),)
            .unwrap();

        print!("{:?}", balance);

        assert_eq!(
            balance.amount,
            Uint128::from(amount - bet_amount_1 - bet_amount_2 - bet_amount_3) + game_req.bet_amount
        );

        let new_cw_balance = app
            .wrap()
            .query_balance(roll_dice_contract_address.clone(), denom.clone(),)
            .unwrap();

        assert_eq!(new_cw_balance.amount, cw_balance.amount - game_req.bet_amount);

        let not_owner_balance_after_req_1 = balance.amount;

        // send lose to 2nd game

        send_lose_exact_number_game(
            &mut app,
            &rand_contract_address,
            &roll_dice_contract_address,
            req_id_2,
            chosen_number,
        );

        let game_req = query_roll_dice_game_request(&mut app, &roll_dice_contract_address, req_id_2,);

        print!("{:?}", game_req);

        assert!(game_req.error_message.is_none());

        assert!(game_req.dice_results.is_some());

        assert!(game_req.status == GameStatus::Lost);

        let balance = app
            .wrap()
            .query_balance(not_owner_address.clone(), denom.clone(),)
            .unwrap();

        print!("{:?}", balance);

        assert_eq!(balance.amount, not_owner_balance_after_req_1);

        let cw_balance = app
            .wrap()
            .query_balance(roll_dice_contract_address.clone(), denom.clone(),)
            .unwrap();

        assert_eq!(cw_balance.amount, new_cw_balance.amount);

        let not_owner_balance_after_req_2 = balance.amount;

        let cw_balance_last_amount = cw_balance.amount;

        // send win to 3rd game

        send_win_exact_number_game(
            &mut app,
            &rand_contract_address,
            &roll_dice_contract_address,
            req_id_3,
            chosen_number,
        );

        let game_req = query_roll_dice_game_request(&mut app, &roll_dice_contract_address, req_id_3,);

        print!("{:?}", game_req);

        assert!(game_req.error_message.is_none());

        assert!(game_req.dice_results.is_some());

        assert!(game_req.status == GameStatus::Won);

        let balance = app
            .wrap()
            .query_balance(not_owner_address.clone(), denom.clone(),)
            .unwrap();

        print!("{:?}", balance);

        assert_eq!(
            balance.amount,
            not_owner_balance_after_req_2 + game_req.potential_winning_amount
        );

        let cw_balance = app
            .wrap()
            .query_balance(roll_dice_contract_address.clone(), denom.clone(),)
            .unwrap();

        assert_eq!(
            cw_balance.amount,
            cw_balance_last_amount - game_req.potential_winning_amount
        );
    }
}

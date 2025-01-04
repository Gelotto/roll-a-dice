#[cfg(test)]

mod test_user_games {

    use crate::test_utils::test_utils::*;
    use cosmwasm_std::{
        coins,
        Addr,
    };
    use cw_multi_test::Executor;

    use roll_a_dice::error::ContractError;

    //TODO: test withdraw not authorized
    //TODO: test withdraw with game to resolve

    #[test]

    pub fn test_user_games() {

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

        let number_of_games = 15;

        let mut req_id_vec: Vec<u64,> = Vec::with_capacity(number_of_games,);

        for _ in 0..number_of_games {

            let resp = app
                .execute_contract(
                    not_owner_address.clone(),
                    roll_dice_contract_address.clone(),
                    &exact_number_request,
                    &coins(bet_amount.into(), denom.clone(),),
                )
                .unwrap();

            let cust_attr = resp.custom_attrs(1 as usize,);

            let game_id = cust_attr[1].value.clone().parse::<u64>().unwrap();

            req_id_vec.push(game_id,);
        }

        let limit = 5;

        let query_user_games_response =
            query_user_games(&mut app, &roll_dice_contract_address, &not_owner_address, None, limit,).unwrap();

        for i in 0..5 {

            let game = query_user_games_response.games.get(i,).unwrap();

            assert_eq!(game.id, *req_id_vec.get(i).unwrap());
        }

        assert_eq!(query_user_games_response.games.len(), limit as usize);

        assert_eq!(query_user_games_response.next_cursor, Some(limit as u64));

        let new_limit = 15;

        let query_user_games_response = query_user_games(
            &mut app,
            &roll_dice_contract_address,
            &not_owner_address,
            Some(limit as usize,),
            new_limit,
        )
        .unwrap();

        for i in 5..15 {

            let game = query_user_games_response.games.get(i - 5,).unwrap();

            assert_eq!(game.id, *req_id_vec.get(i).unwrap());
        }

        assert_eq!(query_user_games_response.games.len(), 10);

        assert_eq!(query_user_games_response.next_cursor, None);
    }

    #[test]

    fn test_empty_user_games() {

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

        let limit = 5;

        let query_user_games_response =
            query_user_games(&mut app, &roll_dice_contract_address, &not_owner_address, None, limit,);

        println!("resp: {:?}", query_user_games_response);

        assert_eq!(query_user_games_response.is_err(), true);

        // Extract the error message from StdError
        let err_msg = query_user_games_response.err().unwrap().to_string();

        assert_eq!(
            "Generic error: Querier contract error: ".to_string()
                + &ContractError::NotFound {
                    reason: format!("User {} has not played any games", not_owner_address.clone()),
                }
                .to_string(),
            err_msg,
        );
    }
}

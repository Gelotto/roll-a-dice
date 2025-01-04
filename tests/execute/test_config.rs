#[cfg(test)]

mod test_config {

    use crate::test_utils::test_utils::*;
    use cosmwasm_std::Addr;
    use cw_multi_test::Executor;

    use roll_a_dice::error::ContractError;

    #[test]

    pub fn test_set_config() {

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

        let mut set_cfg_msg = roll_a_dice_config.clone();

        set_cfg_msg.operator = Some(not_owner_address.clone(),);

        let set_cfg_exec_msg = get_set_config_execute_msg(set_cfg_msg.clone(),);

        let resp = app
            .execute_contract(
                not_owner_address.clone(),
                roll_dice_contract_address.clone(),
                &set_cfg_exec_msg,
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

        let _resp = app
            .execute_contract(
                owner_address.clone(),
                roll_dice_contract_address.clone(),
                &set_cfg_exec_msg,
                &[],
            )
            .unwrap();

        let cfg_queried = query_roll_dice_config(&mut app, &roll_dice_contract_address,);

        assert_eq!(cfg_queried, set_cfg_msg);
    }
}

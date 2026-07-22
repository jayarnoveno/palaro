#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_happy_path_register_and_checkin() {
        let env = Env::default();
        let contract_id = env.register(PalaroPassContract, ());
        let client = PalaroPassContractClient::new(&env, &contract_id);

        let captain = Address::generate(&env);
        env.mock_all_auths();

        // Register team for Basketball (sport_type = 1)
        let reg_res = client.try_register_team(&captain, &101, &1, &50);
        assert!(reg_res.is_ok());

        let status = client.get_team_status(&101).unwrap();
        assert_eq!(status.captain, captain);
        assert_eq!(status.fee_paid, 50);
        assert_eq!(status.is_checked_in, false);

        // Check in team
        let checkin_res = client.try_check_in_team(&101);
        assert!(checkin_res.is_ok());

        let updated_status = client.get_team_status(&101).unwrap();
        assert_eq!(updated_status.is_checked_in, true);
    }

    #[test]
    fn test_zero_fee_fails() {
        let env = Env::default();
        let contract_id = env.register(PalaroPassContract, ());
        let client = PalaroPassContractClient::new(&env, &contract_id);

        let captain = Address::generate(&env);
        env.mock_all_auths();

        let res = client.try_register_team(&captain, &102, &1, &0);
        assert_eq!(res, Err(Ok(Error::InvalidFee)));
    }

    #[test]
    fn test_duplicate_team_registration_fails() {
        let env = Env::default();
        let contract_id = env.register(PalaroPassContract, ());
        let client = PalaroPassContractClient::new(&env, &contract_id);

        let captain = Address::generate(&env);
        env.mock_all_auths();

        client.register_team(&captain, &101, &1, &50);
        let second_res = client.try_register_team(&captain, &101, &1, &50);
        assert_eq!(second_res, Err(Ok(Error::AlreadyRegistered)));
    }

    #[test]
    fn test_state_persistence_across_reads() {
        let env = Env::default();
        let contract_id = env.register(PalaroPassContract, ());
        let client = PalaroPassContractClient::new(&env, &contract_id);

        let captain = Address::generate(&env);
        env.mock_all_auths();

        client.register_team(&captain, &201, &2, &100);
        let status = client.get_team_status(&201).expect("Team should exist");
        assert_eq!(status.sport_type, 2);
        assert_eq!(status.fee_paid, 100);
    }

    #[test]
    fn test_checkin_nonexistent_team_fails() {
        let env = Env::default();
        let contract_id = env.register(PalaroPassContract, ());
        let client = PalaroPassContractClient::new(&env, &contract_id);

        env.mock_all_auths();
        let res = client.try_check_in_team(&999);
        assert_eq!(res, Err(Ok(Error::TeamNotFound)));
    }
}

## link
stellar contract deploy --source-account alice --network testnet
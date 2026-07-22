#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

const REG_POOL: Symbol = symbol_short!("POOL");

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyRegistered = 1,
    TeamNotFound = 2,
    InvalidFee = 3,
    AlreadyCheckedIn = 4,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TeamRecord {
    pub captain: Address,
    pub sport_type: u32, // 1: Basketball, 2: Volleyball, 3: Badminton, 4: Table Tennis
    pub fee_paid: u32,
    pub is_checked_in: bool,
}

#[contract]
pub struct PalaroPassContract;

#[contractimpl]
impl PalaroPassContract {
    /// Registers a sports team, records fee payment, and updates registration storage
    pub fn register_team(env: Env, captain: Address, team_id: u32, sport_type: u32, fee_amount: u32) -> Result<(), Error> {
        captain.require_auth();

        if fee_amount == 0 {
            return Err(Error::InvalidFee);
        }

        if env.storage().persistent().has(&team_id) {
            return Err(Error::AlreadyRegistered);
        }

        let record = TeamRecord {
            captain: captain.clone(),
            sport_type,
            fee_paid: fee_amount,
            is_checked_in: false,
        };

        // Update total fee pool accumulated
        let current_pool: u32 = env.storage().instance().get(&REG_POOL).unwrap_or(0);
        env.storage().instance().set(&REG_POOL, &(current_pool + fee_amount));

        env.storage().persistent().set(&team_id, &record);
        env.storage().persistent().extend_ttl(&team_id, 50, 100);
        env.storage().instance().extend_ttl(50, 100);

        Ok(())
    }

    /// Check in registered team on matchday
    pub fn check_in_team(env: Env, team_id: u32) -> Result<(), Error> {
        let mut record: TeamRecord = env
            .storage()
            .persistent()
            .get(&team_id)
            .ok_or(Error::TeamNotFound)?;

        if record.is_checked_in {
            return Err(Error::AlreadyCheckedIn);
        }

        record.captain.require_auth();
        record.is_checked_in = true;

        env.storage().persistent().set(&team_id, &record);
        env.storage().persistent().extend_ttl(&team_id, 50, 100);

        Ok(())
    }

    /// Reads athlete team status from persistent storage
    pub fn get_team_status(env: Env, team_id: u32) -> Option<TeamRecord> {
        env.storage().persistent().get(&team_id)
    }
}
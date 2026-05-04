#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, BytesN, Env
};

// Fuel log structure
#[derive(Clone)]
#[contracttype]
pub struct FuelLog {
    pub rider: Address,
    pub hash: BytesN<32>,
    pub verified: bool,
}

// Storage key (rider + hash)
#[derive(Clone)]
#[contracttype]
pub struct LogKey(pub Address, pub BytesN<32>);

#[contract]
pub struct BiyaheroLedger;

#[contractimpl]
impl BiyaheroLedger {

    // Register a fuel log (called by rider)
    pub fn register_fuel_log(
        env: Env,
        rider: Address,
        hash: BytesN<32>,
    ) {
        rider.require_auth();

        let key = LogKey(rider.clone(), hash.clone());

        // Prevent duplicate entries
        if env.storage().instance().has(&key) {
            panic!("Duplicate log");
        }

        let log = FuelLog {
            rider: rider.clone(),
            hash,
            verified: false,
        };

        env.storage().instance().set(&key, &log);
    }

    // Verify fuel log (called by company)
    pub fn verify_log(
        env: Env,
        rider: Address,
        hash: BytesN<32>,
    ) {
        let key = LogKey(rider.clone(), hash.clone());

        let mut log: FuelLog = env.storage().instance()
            .get(&key)
            .expect("Log not found");

        log.verified = true;

        env.storage().instance().set(&key, &log);

        // Emit verification event
        env.events().publish(
            (symbol_short!("verified"), rider),
            hash
        );
    }

    // Distribute rebate (XLM simulated)
    pub fn distribute_rebate(
        env: Env,
        rider: Address,
        hash: BytesN<32>,
        amount: i128,
    ) {
        let key = LogKey(rider.clone(), hash.clone());

        let log: FuelLog = env.storage().instance()
            .get(&key)
            .expect("Log not found");

        if !log.verified {
            panic!("Not verified");
        }

        // Require rider auth (simplified payout control)
        rider.require_auth();

        env.events().publish(
            (symbol_short!("rebate"), rider),
            amount
        );
    }

    // Getter
    pub fn get_log(
        env: Env,
        rider: Address,
        hash: BytesN<32>,
    ) -> FuelLog {
        let key = LogKey(rider, hash);

        env.storage().instance()
            .get(&key)
            .expect("Log not found")
    }
}
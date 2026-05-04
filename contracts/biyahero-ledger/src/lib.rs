#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, BytesN, Env, Symbol
};

#[derive(Clone)]
#[contracttype]
pub struct FuelLog {
    pub rider: Address,
    pub price: i128,
    pub liters: i128,
    pub odometer: i128,
    pub timestamp: u64,
    pub hash: BytesN<32>,
    pub verified: bool,
}

// Storage key
#[derive(Clone)]
#[contracttype]
pub struct LogKey(pub Address, pub BytesN<32>);

#[contract]
pub struct BiyaheroLedger;

#[contractimpl]
impl BiyaheroLedger {

    // 1. Register fuel log
    pub fn register_fuel_log(
        env: Env,
        rider: Address,
        price: i128,
        liters: i128,
        odometer: i128,
        hash: BytesN<32>,
    ) {
        rider.require_auth();

        let key = LogKey(rider.clone(), hash.clone());

        if env.storage().instance().has(&key) {
            panic!("Duplicate fuel log");
        }

        let log = FuelLog {
            rider: rider.clone(),
            price,
            liters,
            odometer,
            timestamp: env.ledger().timestamp(),
            hash,
            verified: false,
        };

        env.storage().instance().set(&key, &log);
    }

    // 2. Verify log (platform side)
    pub fn verify_log(env: Env, rider: Address, hash: BytesN<32>) {
        let key = LogKey(rider.clone(), hash.clone());

        let mut log: FuelLog = env
            .storage()
            .instance()
            .get(&key)
            .expect("Log not found");

        log.verified = true;

        env.storage().instance().set(&key, &log);

        env.events().publish(
            (symbol_short!("verified"), rider),
            hash
        );
    }

    // 3. Distribute rebate
    pub fn distribute_rebate(
        env: Env,
        rider: Address,
        hash: BytesN<32>,
        amount: i128,
    ) {
        let key = LogKey(rider.clone(), hash.clone());

        let log: FuelLog = env
            .storage()
            .instance()
            .get(&key)
            .expect("Log not found");

        if !log.verified {
            panic!("Log not verified");
        }

        // Simplified transfer (native XLM)
        rider.require_auth();

        env.events().publish(
            (symbol_short!("rebate_paid"), rider),
            amount
        );
    }

    // Helper: get log
    pub fn get_log(env: Env, rider: Address, hash: BytesN<32>) -> FuelLog {
        let key = LogKey(rider, hash);

        env.storage()
            .instance()
            .get(&key)
            .expect("Log not found")
    }
}
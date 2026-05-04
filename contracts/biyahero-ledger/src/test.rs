#![cfg(test)]

use super::*;
use soroban_sdk::{Env, BytesN};

fn create_hash(env: &Env) -> BytesN<32> {
    BytesN::from_array(env, &[1; 32])
}

#![cfg(test)]

use super::*;
use soroban_sdk::{Env, BytesN, Address};

fn hash(env: &Env) -> BytesN<32> {
    BytesN::from_array(env, &[1; 32])
}

// Test 1: Happy path
#[test]
fn test_full_flow_success() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BiyaheroLedger);
    let client = BiyaheroLedgerClient::new(&env, &contract_id);

    let rider = Address::random(&env);
    let h = hash(&env);

    client.register_fuel_log(&rider, &h);
    client.verify_log(&rider, &h);
    client.distribute_rebate(&rider, &h, &50);

    assert!(true);
}

// Test 2: Edge case (duplicate)
#[test]
fn test_duplicate_log() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BiyaheroLedger);
    let client = BiyaheroLedgerClient::new(&env, &contract_id);

    let rider = Address::random(&env);
    let h = hash(&env);

    client.register_fuel_log(&rider, &h);

    let result = std::panic::catch_unwind(|| {
        client.register_fuel_log(&rider, &h);
    });

    assert!(result.is_err());
}

// Test 3: State verification
#[test]
fn test_log_state() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BiyaheroLedger);
    let client = BiyaheroLedgerClient::new(&env, &contract_id);

    let rider = Address::random(&env);
    let h = hash(&env);

    client.register_fuel_log(&rider, &h);

    let log = client.get_log(&rider, &h);

    assert_eq!(log.verified, false);
}

// Test 4: Verify updates state
#[test]
fn test_verify_updates() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BiyaheroLedger);
    let client = BiyaheroLedgerClient::new(&env, &contract_id);

    let rider = Address::random(&env);
    let h = hash(&env);

    client.register_fuel_log(&rider, &h);
    client.verify_log(&rider, &h);

    let log = client.get_log(&rider, &h);

    assert_eq!(log.verified, true);
}

// Test 5: Rebate fails if not verified
#[test]
fn test_rebate_fail_unverified() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BiyaheroLedger);
    let client = BiyaheroLedgerClient::new(&env, &contract_id);

    let rider = Address::random(&env);
    let h = hash(&env);

    client.register_fuel_log(&rider, &h);

    let result = std::panic::catch_unwind(|| {
        client.distribute_rebate(&rider, &h, &50);
    });

    assert!(result.is_err());
}
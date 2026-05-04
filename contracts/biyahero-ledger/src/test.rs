#![cfg(test)]

use super::*;
use soroban_sdk::{Env, BytesN};

fn create_hash(env: &Env) -> BytesN<32> {
    BytesN::from_array(env, &[1; 32])
}

#[test]
fn test_register_log() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BiyaheroLedger);
    let client = BiyaheroLedgerClient::new(&env, &contract_id);

    let rider = Address::random(&env);
    let hash = create_hash(&env);

    client.register_fuel_log(
        &rider,
        &100,
        &5,
        &1000,
        &hash
    );

    let log = client.get_log(&rider, &hash);

    assert_eq!(log.price, 100);
    assert_eq!(log.verified, false);
}

#[test]
fn test_verify_log() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BiyaheroLedger);
    let client = BiyaheroLedgerClient::new(&env, &contract_id);

    let rider = Address::random(&env);
    let hash = create_hash(&env);

    client.register_fuel_log(&rider, &100, &5, &1000, &hash);
    client.verify_log(&rider, &hash);

    let log = client.get_log(&rider, &hash);

    assert_eq!(log.verified, true);
}

#[test]
fn test_rebate_requires_verification() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BiyaheroLedger);
    let client = BiyaheroLedgerClient::new(&env, &contract_id);

    let rider = Address::random(&env);
    let hash = create_hash(&env);

    client.register_fuel_log(&rider, &100, &5, &1000, &hash);

    // This should panic because not verified
    let result = std::panic::catch_unwind(|| {
        client.distribute_rebate(&rider, &hash, &50);
    });

    assert!(result.is_err());
}

#[test]
fn test_rebate_success() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BiyaheroLedger);
    let client = BiyaheroLedgerClient::new(&env, &contract_id);

    let rider = Address::random(&env);
    let hash = create_hash(&env);

    client.register_fuel_log(&rider, &100, &5, &1000, &hash);
    client.verify_log(&rider, &hash);

    client.distribute_rebate(&rider, &hash, &50);

    // If no panic → success
    assert!(true);
}
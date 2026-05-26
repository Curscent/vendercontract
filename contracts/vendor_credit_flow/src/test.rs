#![cfg(test)]

use soroban_sdk::{Env, Address};
use super::*;

#[test]
fn test_happy_path() {
    let env = Env::default();
    let contract_id = env.register_contract(None, VendorCredit);
    let client = VendorCreditClient::new(&env, &contract_id);

    let vendor = Address::generate(&env);
    let supplier = Address::generate(&env);

    client.request_loan(&vendor, &supplier, &1000);
    client.fund(&vendor);
    client.repay(&vendor, &1000);

    let loan = client.get_loan(&vendor);
    assert_eq!(loan.active, false);
}

#[test]
fn test_edge_case_no_loan() {
    let env = Env::default();
    let contract_id = env.register_contract(None, VendorCredit);
    let client = VendorCreditClient::new(&env, &contract_id);

    let vendor = Address::generate(&env);

    // should panic (no loan exists)
    let result = std::panic::catch_unwind(|| {
        client.get_loan(&vendor);
    });

    assert!(result.is_err());
}

#[test]
fn test_state_after_request() {
    let env = Env::default();
    let contract_id = env.register_contract(None, VendorCredit);
    let client = VendorCreditClient::new(&env, &contract_id);

    let vendor = Address::generate(&env);
    let supplier = Address::generate(&env);

    client.request_loan(&vendor, &supplier, &500);

    let loan = client.get_loan(&vendor);
    assert_eq!(loan.amount, 500);
}

#[test]
fn test_partial_repayment() {
    let env = Env::default();
    let contract_id = env.register_contract(None, VendorCredit);
    let client = VendorCreditClient::new(&env, &contract_id);

    let vendor = Address::generate(&env);
    let supplier = Address::generate(&env);

    client.request_loan(&vendor, &supplier, &1000);
    client.repay(&vendor, &400);

    let loan = client.get_loan(&vendor);
    assert_eq!(loan.repaid, 400);
}

#[test]
fn test_full_repayment_closes() {
    let env = Env::default();
    let contract_id = env.register_contract(None, VendorCredit);
    let client = VendorCreditClient::new(&env, &contract_id);

    let vendor = Address::generate(&env);
    let supplier = Address::generate(&env);

    client.request_loan(&vendor, &supplier, &1000);
    client.repay(&vendor, &1000);

    let loan = client.get_loan(&vendor);
    assert_eq!(loan.active, false);
}
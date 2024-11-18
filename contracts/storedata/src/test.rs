#![cfg(test)]

use super::{StoredataContract, StoredataContractClient};
use soroban_sdk::{testutils::Logs, BytesN, Env};

extern crate std;

#[test]
fn test_increment() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StoredataContract);
    let client = StoredataContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);

    std::println!("{}", env.logs().all().join("\n"));
}

#[test]
fn test_hash_storage_and_retrieval() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StoredataContract);
    let client = StoredataContractClient::new(&env, &contract_id);

    // Create and store multiple hashes
    let hash1 = BytesN::from_array(&env, &[1; 32]);
    let id1 = client.store_hash(&hash1);

    let hash2 = BytesN::from_array(&env, &[2; 32]);
    let id2 = client.store_hash(&hash2);

    let hash3 = BytesN::from_array(&env, &[3; 32]);
    let id3 = client.store_hash(&hash3);

    // Retrieve and verify hashes
    assert_eq!(client.get_hash(&id1), Some(hash1));
    assert_eq!(client.get_hash(&id2), Some(hash2));
    assert_eq!(client.get_hash(&id3), Some(hash3));

    // Verify that the counter has increased correctly
    assert_eq!(client.get_counter(), 3);

    std::println!("Multiple hashes stored and retrieved successfully");
}

#[test]
fn test_increment_and_multiple_hashes() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StoredataContract);
    let client = StoredataContractClient::new(&env, &contract_id);

    // Increment and store hashes
    assert_eq!(client.increment(), 1);

    let hash1 = BytesN::from_array(&env, &[1; 32]);
    let id1 = client.store_hash(&hash1);
    assert_eq!(id1, 2);

    assert_eq!(client.increment(), 3);

    let hash2 = BytesN::from_array(&env, &[2; 32]);
    let id2 = client.store_hash(&hash2);
    assert_eq!(id2, 4);

    // Retrieve and verify hashes
    assert_eq!(client.get_hash(&id1), Some(hash1));
    assert_eq!(client.get_hash(&id2), Some(hash2));

    // Verify final counter value
    assert_eq!(client.get_counter(), 4);

    std::println!("Increment and multiple hash operations successful");
    std::println!("{}", env.logs().all().join("\n"));
}
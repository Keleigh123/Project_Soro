#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, BytesN, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");
const HASH_PREFIX: Symbol = symbol_short!("HASH_");

#[contract]
pub struct StoredataContract;

#[contractimpl]
impl StoredataContract {
    /// Storedata increments an internal counter, and returns the value.
    pub fn increment(env: &Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&COUNTER, &count);
        env.storage().instance().extend_ttl(50, 100);
        count
    }

    /// Store a hash value with the current counter as its ID
    pub fn store_hash(env: &Env, hash: BytesN<32>) -> u32 {
        let count = Self::increment(env);
        let key = (HASH_PREFIX, count);
        env.storage().instance().set(&key, &hash);
        env.storage().instance().extend_ttl(50, 100);
        count  // Return the ID (counter value) used for this hash
    }

    /// Retrieve a stored hash value by its ID (counter value)
    pub fn get_hash(env: &Env, id: u32) -> Option<BytesN<32>> {
        let key = (HASH_PREFIX, id);
        env.storage().instance().get(&key)
    }

    /// Get the current counter value
    pub fn get_counter(env: &Env) -> u32 {
        env.storage().instance().get(&COUNTER).unwrap_or(0)
    }
}

mod test;
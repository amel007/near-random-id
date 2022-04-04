//! This contract implements simple counter backed by storage on blockchain.
//!
//! The contract provides methods to [increment] / [decrement] counter and
//! [get it's current value][get_num] or [reset].
//!
//! [increment]: struct.Counter.html#method.increment
//! [decrement]: struct.Counter.html#method.decrement
//! [get_num]: struct.Counter.html#method.get_num
//! [reset]: struct.Counter.html#method.reset

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use near_sdk::collections::UnorderedMap;

near_sdk::setup_alloc!();

// add the following attributes to prepare your code for serialization and invocation on the blockchain
// More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Counter {
    // See more data types at https://doc.rust-lang.org/book/ch03-02-data-types.html
    max_supply_token: u64,
    token_count: u64,
    token_matrix: UnorderedMap<u64, u64>
}

#[near_bindgen]
impl Counter {

    #[init]
    pub fn new(max_supply_token: u64) -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self {
            max_supply_token,
            token_count: 0,
            token_matrix: UnorderedMap::new(b"t2o".to_vec())
        }
    }

    pub fn get_random_token(&mut self) -> u64 {

        let max_index = self.max_supply_token - self.token_count;

        if max_index == 0 {
            env::panic(b"no ids left")
        }

        use rand::prelude::*;
        use rand_chacha::ChaCha8Rng;
        use rand_seeder::{Seeder};
        let mut rng: ChaCha8Rng = Seeder::from(env::random_seed()).make_rng();

        let random_value: u64 = rng.gen();
        let rand_index = random_value % max_index;

        let current_id = self.token_matrix.get(&rand_index);

        let rand_id: u64;
        if current_id.is_some() && (current_id.unwrap() > 0) {
            rand_id = current_id.unwrap();
        } else {
            rand_id = rand_index;
        }

        let prev_index = max_index - 1;

        let prev_id = self.token_matrix.get(&prev_index);

        if prev_id.is_some() && (prev_id.unwrap() > 0) {
            self.token_matrix.insert(&rand_index, &prev_id.unwrap());
        } else {
            self.token_matrix.insert(&rand_index, &prev_index);
        }

        self.token_count += 1;

        let log_message = format!("Token id: {}", rand_id);
        env::log(log_message.as_bytes());

        return rand_id;
    }

}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-counter-tutorial -- --nocapture
 * Note: 'rust-counter-tutorial' comes from cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 231,
            block_timestamp: 54654,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 23, 2, 4, 8],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn mint_all_token() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        // instantiate a contract variable with the counter at zero
        let int_for_test: u64 = 10;

        let mut contract = Counter::new(int_for_test);
        let mut n = 1;

        for _ in 0..int_for_test {
            println!("Value after random: {}", contract.get_random_token());
            n = n + 1;
        }

        assert_eq!(1, 1);
    }
}
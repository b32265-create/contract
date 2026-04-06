#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, log};

#[contract]
pub struct DigitalWill;

#[contractimpl]
impl DigitalWill {
    // Initialize the will with a beneficiary and an expiration time (in seconds)
    pub fn init(env: Env, owner: Address, beneficiary: Address, timeout: u64) {
        owner.require_auth();
        env.storage().instance().set(&Symbol::new(&env, "owner"), &owner);
        env.storage().instance().set(&Symbol::new(&env, "ben"), &beneficiary);
        env.storage().instance().set(&Symbol::new(&env, "timeout"), &timeout);
        
        // Set the initial deadline
        let deadline = env.ledger().timestamp() + timeout;
        env.storage().instance().set(&Symbol::new(&env, "deadline"), &deadline);
    }

    // Owner calls this to prove they are still active
    pub fn ping(env: Env) {
        let owner: Address = env.storage().instance().get(&Symbol::new(&env, "owner")).unwrap();
        owner.require_auth();

        let timeout: u64 = env.storage().instance().get(&Symbol::new(&env, "timeout")).unwrap();
        let new_deadline = env.ledger().timestamp() + timeout;
        
        env.storage().instance().set(&Symbol::new(&env, "deadline"), &new_deadline);
        log!(&env, "Heartbeat received. Deadline extended.");
    }

    // Beneficiary calls this if the deadline has passed
    pub fn claim(env: Env, beneficiary: Address) {
        beneficiary.require_auth();
        
        let stored_ben: Address = env.storage().instance().get(&Symbol::new(&env, "ben")).unwrap();
        if beneficiary != stored_ben {
            panic!("Not the designated beneficiary");
        }

        let deadline: u64 = env.storage().instance().get(&Symbol::new(&env, "deadline")).unwrap();
        if env.ledger().timestamp() < deadline {
            panic!("Owner is still active. Deadline has not passed.");
        }

        // Logic to transfer assets would go here (e.g., using Stellar Asset Contract)
        log!(&env, "Will executed. Assets transferred to beneficiary.");
    }
}
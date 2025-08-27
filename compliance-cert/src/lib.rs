#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short, Address};

#[contract]
pub struct ComplianceCertContract;

#[contractimpl]
impl ComplianceCertContract {
    // Exemple de fonction d’enregistrement
    pub fn issue_cert(env: Env, owner: Address, cert_id: Symbol) {
        // on log pour debug
        env.events().publish(
    (symbol_short!("issue"), cert_id.clone()), 
    symbol_short!("issued"),
);

    env.storage().instance().set(&cert_id, &owner);


    }

    // Exemple de fonction de validation
    pub fn validate_cert(env: Env, cert_id: Symbol) -> bool {
        env.storage().instance().has(&cert_id)
    }
}

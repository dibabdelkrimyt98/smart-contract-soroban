#![no_std]

mod errors;
mod models;
mod storage;
mod events;

use soroban_sdk::{contract, contractimpl, Env, Address, String};
use errors::ComplianceError;
use models::Certificate;
use storage::{store_certificate, get_certificate};
use events::{emit_issued, emit_validated};

#[contract]
pub struct ComplianceContract;

#[contractimpl]
impl ComplianceContract {
    pub fn issue_certificate(env: Env, id: u32, owner: Address, details: String) -> Result<(), ComplianceError> {
        let cert = Certificate::new(id, owner.clone(), details);
        store_certificate(&env, &cert)?;
        emit_issued(&env, &cert);
        Ok(())
    }

    pub fn validate_certificate(env: Env, id: u32) -> Result<bool, ComplianceError> {
        let mut cert = get_certificate(&env, id)?;
        cert.valid = true;
        store_certificate(&env, &cert)?;
        emit_validated(&env, &cert);
        Ok(true)
    }
}

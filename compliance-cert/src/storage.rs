use soroban_sdk::{Env, Symbol};
use crate::{models::Certificate, errors::ComplianceError};

const CERT_KEY: &str = "certificates";

pub fn store_certificate(env: &Env, cert: &Certificate) -> Result<(), ComplianceError> {
    let key = (Symbol::new(env, CERT_KEY), cert.id);
    env.storage().persistent().set(&key, cert);
    Ok(())
}

pub fn get_certificate(env: &Env, id: u32) -> Result<Certificate, ComplianceError> {
    let key = (Symbol::new(env, CERT_KEY), id);
    env.storage()
        .persistent()
        .get::<_, Certificate>(&key)
        .ok_or(ComplianceError::NotFound)
}

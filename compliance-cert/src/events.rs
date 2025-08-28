use soroban_sdk::{Env, Symbol};
use crate::models::Certificate;

pub fn emit_issued(env: &Env, cert: &Certificate) {
    let topics = (Symbol::new(env, "issued"), cert.owner.clone());
    env.events().publish(topics, cert.id);
}

pub fn emit_validated(env: &Env, cert: &Certificate) {
    let topics = (Symbol::new(env, "validated"), cert.owner.clone());
    env.events().publish(topics, cert.id);
}

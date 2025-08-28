use soroban_sdk::{Address, String};

#[derive(Clone)]
pub struct Certificate {
    pub id: u32,
    pub owner: Address,
    pub details: String,
    pub valid: bool,
}

impl Certificate {
    pub fn new(id: u32, owner: Address, details: String) -> Self {
        Self {
            id,
            owner,
            details,
            valid: false,
        }
    }
}

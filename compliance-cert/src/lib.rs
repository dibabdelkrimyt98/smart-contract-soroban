#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Env, Symbol, Map, Address,
};

// Structure stockée on-chain pour un certificat
#[derive(Clone)]
#[contracttype]
pub struct Certificate {
    pub owner: Address,   // propriétaire (Address)
    pub details: Symbol,  // description / meta (Symbol)
    pub valid: bool,      // statut (true = actif)
}

// Déclare le contrat (génère les types client après build)
#[contract]
pub struct CertificateContract;

#[contractimpl]
impl CertificateContract {
    // CREATE : enregistre un nouveau certificat
    pub fn create_certificate(env: Env, id: Symbol, owner: Address, details: Symbol) {
        let mut certs: Map<Symbol, Certificate> =
            env.storage()
                .persistent()
                .get(&symbol_short!("certs"))
                .unwrap_or(Map::new(&env));

        if certs.contains_key(id.clone()) {
            panic!("cert_exists");
        }

        let cert = Certificate {
            owner: owner.clone(),
            details: details.clone(),
            valid: true,
        };

        certs.set(id.clone(), cert);
        env.storage().persistent().set(&symbol_short!("certs"), &certs);

        // Émettre un événement pour audit
        env.events().publish((symbol_short!("cert"), symbol_short!("created")), id);
    }

    // READ : récupérer un certificat (panique si introuvable)
    pub fn read_certificate(env: Env, id: Symbol) -> Certificate {
        let certs: Map<Symbol, Certificate> =
            env.storage()
                .persistent()
                .get(&symbol_short!("certs"))
                .unwrap_or(Map::new(&env));

        certs.get(id).unwrap_or_else(|| panic!("cert_not_found"))
    }

    // UPDATE : mettre à jour détails et statut
    pub fn update_certificate(env: Env, id: Symbol, new_details: Symbol, new_valid: bool) {
        let mut certs: Map<Symbol, Certificate> =
            env.storage()
                .persistent()
                .get(&symbol_short!("certs"))
                .unwrap_or(Map::new(&env));

        let mut cert = certs.get(id.clone()).unwrap_or_else(|| panic!("cert_not_found"));

        cert.details = new_details;
        cert.valid = new_valid;

        certs.set(id.clone(), cert);
        env.storage().persistent().set(&symbol_short!("certs"), &certs);

        env.events().publish((symbol_short!("cert"), symbol_short!("updated")), id);
    }

    // DELETE (revoke) : révoquer un certificat (on peut aussi supprimer physiquement si désiré)
    pub fn revoke_certificate(env: Env, id: Symbol) {
        let mut certs: Map<Symbol, Certificate> =
            env.storage()
                .persistent()
                .get(&symbol_short!("certs"))
                .unwrap_or(Map::new(&env));

        if !certs.contains_key(id.clone()) {
            panic!("cert_not_found");
        }

        // Option A : révoquer en marquant valid = false (conseillé)
        let mut cert = certs.get(id.clone()).unwrap();
        cert.valid = false;
        certs.set(id.clone(), cert);

        // Si tu veux supprimer totalement : certs.remove(id.clone());

        env.storage().persistent().set(&symbol_short!("certs"), &certs);
        env.events().publish((symbol_short!("cert"), symbol_short!("revoked")), id);
    }

    // OPTIONAL : vérifier si actif (helper)
    pub fn is_valid(env: Env, id: Symbol) -> bool {
        let certs: Map<Symbol, Certificate> =
            env.storage()
                .persistent()
                .get(&symbol_short!("certs"))
                .unwrap_or(Map::new(&env));

        match certs.get(id) {
            Some(c) => c.valid,
            None => false,
        }
    }
}

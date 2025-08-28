use soroban_sdk::{Env, Symbol, symbol_short};

/// Génère une clé Symbol courte avec un préfixe.
/// ⚠️ Limite : max 9 caractères pour `symbol_short!`.
pub fn build_cert_key(_env: &Env, prefix: &str, id: &str) -> Symbol {
    // Exemple simple : "cert1", "cert2"...
    // ATTENTION : ne dépasse pas 9 caractères !
    match prefix {
        "cert" => symbol_short!("cert"),
        "user" => symbol_short!("user"),
        _ => symbol_short!("unkn"), // fallback
    }
}

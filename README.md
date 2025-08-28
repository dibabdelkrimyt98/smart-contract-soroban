# Compliance AI Certifications - Soroban Smart Contract

## 📌 Description du projet
Ce projet implémente un **Smart Contract** sur **Soroban** (la plateforme de contrats intelligents de Stellar) afin de gérer la **certification de conformité IA (Intelligence Artificielle)**.  
L’objectif est de fournir un mécanisme sécurisé et transparent pour :

- **Émettre** des certifications de conformité pour des modèles IA ou des entités.
- **Valider** l’authenticité d’une certification.
- (Futur) **Gérer l’expiration** et le renouvellement des certifications.

Le projet inclut également une **API backend** permettant d’interagir avec le contrat intelligent, et de préparer l’intégration avec des applications externes.

---

## 🏗️ Architecture générale

Le projet est organisé en deux principales couches :

1. **Smart Contract (Soroban)**
   - Écrit en **Rust**.
   - Compilé en **WASM** (`wasm32-unknown-unknown`).
   - Déployé et testé sur :
     - **Local Sandbox Soroban** (via Docker).
     - **Stellar Testnet** (alternative en cas de problème avec sandbox).

2. **API d’intégration**
   - Fournit des endpoints REST pour faciliter l’interaction avec le contrat :
     - Émission d’une certification.
     - Validation d’une certification.
     - (Futur) Gestion d’expiration.
   - Implémentée en Node.js / Express (ou Rust backend selon choix futur).

---

## ⚙️ Prérequis

Avant de commencer, assurez-vous d’avoir installé :

- [Rust](https://www.rust-lang.org/) (version >= 1.75)  
- `cargo` (inclus avec Rust)  
- **Cible WebAssembly** :
  ```bash
  rustup target add wasm32-unknown-unknown

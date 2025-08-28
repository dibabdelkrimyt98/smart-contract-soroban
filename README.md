# Compliance Certification Platform – Soroban Smart Contract, API & Frontend

This project implements a **compliance certification management system** built on the **Stellar Soroban smart contract platform**.  
It provides a complete stack including:
- **Smart Contract (Rust, Soroban)** for certification issuance & validation.
- **Backend API (Python, Flask)** to interact with the smart contract.
- **Frontend (React.js)** for users to manage compliance certificates via a simple interface.

---

## 📂 Project Structure

/smart-contract-soroban
|-- /contracts
| |-- /compliance-cert
| | |-- /src
| | | |-- lib.rs <-- Smart contract code (Rust)
| | |-- Cargo.toml <-- Rust dependencies for the contract
| | |-- XDR <-- Auto-generated XDR files
| | |-- .gitkeep
| |-- .gitkeep
|
|-- /api
| |-- app.py <-- Python (Flask) API server
| |-- requirements.txt <-- Python dependencies
|
|-- /frontend
| |-- /src
| | |-- App.js <-- Main React component
| | |-- index.js <-- React entry point
| | |-- styles.css <-- CSS styling
| |-- /public
| | |-- index.html <-- HTML file
| |-- package.json <-- Node.js dependencies
|
|-- .gitignore
|-- README.md


---

## 🚀 Features

- **Smart Contract (Soroban, Rust)**
  - Issue compliance certificates (linked to an account).
  - Validate certificates on-chain.
  - Store metadata (e.g., certificate ID, issuer, validity).
  - Extensible for future features (e.g., expiration, revocation).

- **Backend API (Flask)**
  - Exposes RESTful endpoints.
  - Provides interaction with the smart contract (via Soroban RPC).
  - Decouples blockchain logic from frontend.

- **Frontend (React)**
  - User-friendly web interface.
  - Certificate management (issue, validate).
  - Connects to backend API.

---

## 🛠️ Setup Instructions

### 1. Prerequisites
Make sure you have the following installed:
- [Rust & Cargo](https://www.rust-lang.org/)
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup)
- [Docker](https://www.docker.com/) (for local sandbox)
- [Python 3.9+](https://www.python.org/downloads/)
- [Node.js & npm](https://nodejs.org/)

---

### 2. Smart Contract Setup

#### a) Compile the Contract
```bash
cd contracts/compliance-cert
cargo build --target wasm32-unknown-unknown --release

Run Soroban Sandbox (via Docker)
docker run --rm -it \
  -p 8000:8000 \
  stellar/quickstart:soroban-dev \
  --standalone \
  --enable-soroban-rpc

Deploy Contract to Sandbox/Testnet
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/compliance_cert.wasm \
  --source default \
  --rpc-url http://localhost:8000 \
  --network-passphrase "Standalone Network ; February 2017"



3. Backend API Setup
  a) Install Dependencies
      cd api
      python3 -m venv venv
      source venv/bin/activate   # On Windows: venv\Scripts\activate
      pip install -r requirements.txt

  b) Run the Flask Server
      python app.py


4. Frontend Setup
  a) Install Dependencies
      cd frontend
      npm install

  b) Run React App
      npm start

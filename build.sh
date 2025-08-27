#!/bin/bash
set -e

# Nettoyer les anciens builds
cargo clean

# Compiler en WASM optimisé
cargo build --target wasm32-unknown-unknown --release

# Copier le .wasm dans un dossier dédié (facultatif)
mkdir -p artifacts
cp target/wasm32-unknown-unknown/release/*.wasm artifacts/

echo "✅ Build terminé. Le fichier wasm est dans artifacts/"

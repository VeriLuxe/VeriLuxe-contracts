#!/usr/bin/env node

// Script para generar un nuevo keypair de Stellar/Soroban
// Requiere: npm install @stellar/stellar-sdk

const { Keypair } = require('@stellar/stellar-sdk');

function generateKeypair() {
    // Generar un nuevo keypair aleatorio
    const keypair = Keypair.random();
    
    console.log('=== NUEVO KEYPAIR GENERADO ===');
    console.log('Public Key (Address):', keypair.publicKey());
    console.log('Secret Key (Hex):', keypair.rawSecretKey().toString('hex'));
    console.log('Secret Key (Stellar):', keypair.secret());
    console.log('');
    console.log('IMPORTANTE:');
    console.log('- Usa el "Secret Key (Hex)" para ADMIN_SECRET_KEY en tu .env');
    console.log('- Usa el "Public Key (Address)" para inicializar el contrato');
    console.log('- NUNCA compartas el secret key - guárdalo de forma segura');
    console.log('');
    console.log('Para usar en testnet, financia la cuenta en:');
    console.log('https://laboratory.stellar.org/#account-creator?network=test');
}

generateKeypair();
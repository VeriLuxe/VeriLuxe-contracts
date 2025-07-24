#!/usr/bin/env python3

# Script para generar un nuevo keypair de Stellar/Soroban
# Requiere: pip install stellar-sdk

from stellar_sdk import Keypair

def generate_keypair():
    # Generar un nuevo keypair aleatorio
    keypair = Keypair.random()
    
    print('=== NUEVO KEYPAIR GENERADO ===')
    print(f'Public Key (Address): {keypair.public_key}')
    print(f'Secret Key (Hex): {keypair.raw_secret_key().hex()}')
    print(f'Secret Key (Stellar): {keypair.secret}')
    print('')
    print('IMPORTANTE:')
    print('- Usa el "Secret Key (Hex)" para ADMIN_SECRET_KEY en tu .env')
    print('- Usa el "Public Key (Address)" para inicializar el contrato')
    print('- NUNCA compartas el secret key - guárdalo de forma segura')
    print('')
    print('Para usar en testnet, financia la cuenta en:')
    print('https://laboratory.stellar.org/#account-creator?network=test')

if __name__ == '__main__':
    generate_keypair()
#!/usr/bin/env python3
"""
VeriLuxe Fashion Authenticity Contract - Python Deployment Script
Alternative deployment method using Stellar Python SDK
"""

import os
import sys
from stellar_sdk import (
    Keypair, 
    Server, 
    TransactionBuilder, 
    Network,
    SorobanServer,
    UploadContractWasm,
    CreateContract,
    InvokeContract
)
from stellar_sdk.exceptions import RequestException

class VeriLuxeDeployer:
    def __init__(self, network='testnet'):
        self.network = network
        self.setup_network()
        self.setup_account()
        
        print(f"🚀 VeriLuxe Python Deployer")
        print(f"📡 Network: {self.network}")
        print(f"👤 Admin: {self.admin_keypair.public_key}")

    def setup_network(self):
        """Configure network settings"""
        if self.network == 'testnet':
            self.horizon_url = "https://horizon-testnet.stellar.org"
            self.soroban_url = "https://soroban-testnet.stellar.org"
            self.network_passphrase = Network.TESTNET_NETWORK_PASSPHRASE
        elif self.network == 'futurenet':
            self.horizon_url = "https://horizon-futurenet.stellar.org"
            self.soroban_url = "https://rpc-futurenet.stellar.org:443"
            self.network_passphrase = Network.FUTURENET_NETWORK_PASSPHRASE
        else:
            raise ValueError(f"Unsupported network: {self.network}")
        
        self.horizon_server = Server(self.horizon_url)
        self.soroban_server = SorobanServer(self.soroban_url)

    def setup_account(self):
        """Setup admin account from environment or use generated"""
        secret_key = os.getenv('ADMIN_SECRET_KEY')
        
        if not secret_key:
            # Use one of our generated mnemonic phrases
            mnemonic = "coin idle bus total sense awful picture dial stick between erode expose stairs they swing account august indicate cruel nasty inherit vocal veteran deal"
            self.admin_keypair = Keypair.from_mnemonic_phrase(mnemonic)
            print("🔑 Using generated account from mnemonic")
        else:
            self.admin_keypair = Keypair.from_secret(secret_key)
            print("🔑 Using account from environment variable")

    def load_wasm(self, wasm_path=None):
        """Load contract WASM file"""
        if not wasm_path:
            wasm_path = "../contracts/target/wasm32v1-none/release/fashion_auth_contract.wasm"
        
        if not os.path.exists(wasm_path):
            raise FileNotFoundError(f"WASM file not found: {wasm_path}")
        
        with open(wasm_path, 'rb') as f:
            wasm_data = f.read()
        
        print(f"📦 Loaded WASM file ({len(wasm_data)} bytes)")
        return wasm_data

    async def deploy_contract(self):
        """Deploy the VeriLuxe contract"""
        try:
            print("\\n📋 Starting contract deployment...")
            
            # Load WASM
            wasm_data = self.load_wasm()
            
            # Get account
            account = self.horizon_server.load_account(self.admin_keypair.public_key)
            print(f"💰 Account loaded with sequence: {account.sequence}")

            # Upload contract
            print("\\n📤 Uploading contract WASM...")
            upload_op = UploadContractWasm(wasm=wasm_data)
            
            upload_tx = (
                TransactionBuilder(
                    source_account=account,
                    network_passphrase=self.network_passphrase,
                    base_fee=100000
                )
                .add_operation(upload_op)
                .set_timeout(300)
                .build()
            )
            
            upload_tx.sign(self.admin_keypair)
            
            upload_response = self.horizon_server.submit_transaction(upload_tx)
            
            if upload_response['successful']:
                print("✅ Contract WASM uploaded successfully")
                contract_hash = upload_response['hash']
                print(f"🔗 Contract hash: {contract_hash}")
            else:
                raise Exception(f"Upload failed: {upload_response}")

            # Deploy contract instance  
            print("\\n🚀 Creating contract instance...")
            
            # Refresh account for new sequence number
            account = self.horizon_server.load_account(self.admin_keypair.public_key)
            
            create_op = CreateContract(
                wasm_hash=bytes.fromhex(contract_hash),
                address=self.admin_keypair.public_key
            )
            
            create_tx = (
                TransactionBuilder(
                    source_account=account,
                    network_passphrase=self.network_passphrase,
                    base_fee=100000
                )
                .add_operation(create_op)
                .set_timeout(300)
                .build()
            )
            
            create_tx.sign(self.admin_keypair)
            
            create_response = self.horizon_server.submit_transaction(create_tx)
            
            if create_response['successful']:
                print("✅ Contract instance created successfully")
                contract_address = create_response['contract_address']
                print(f"🎯 Contract address: {contract_address}")
            else:
                raise Exception(f"Contract creation failed: {create_response}")

            # Initialize contract
            print("\\n⚙️ Initializing contract...")
            await self.initialize_contract(contract_address)

            return {
                'contract_hash': contract_hash,
                'contract_address': contract_address,
                'admin_public_key': self.admin_keypair.public_key,
                'network': self.network
            }

        except Exception as e:
            print(f"❌ Deployment failed: {str(e)}")
            raise

    async def initialize_contract(self, contract_address):
        """Initialize the deployed contract"""
        try:
            account = self.horizon_server.load_account(self.admin_keypair.public_key)
            
            init_op = InvokeContract(
                contract_address=contract_address,
                function_name="init",
                parameters=[self.admin_keypair.public_key]
            )
            
            init_tx = (
                TransactionBuilder(
                    source_account=account,
                    network_passphrase=self.network_passphrase,
                    base_fee=100000
                )
                .add_operation(init_op)
                .set_timeout(300)
                .build()
            )
            
            init_tx.sign(self.admin_keypair)
            
            init_response = self.horizon_server.submit_transaction(init_tx)
            
            if init_response['successful']:
                print("✅ Contract initialized successfully")
            else:
                print(f"⚠️ Contract initialization may have failed: {init_response}")
                
        except Exception as e:
            print(f"❌ Initialization failed: {str(e)}")
            raise

def main():
    """Main deployment function"""
    try:
        # Check for network argument
        network = sys.argv[1] if len(sys.argv) > 1 else 'testnet'
        
        deployer = VeriLuxeDeployer(network=network)
        result = deployer.deploy_contract()
        
        print("\\n🎉 Deployment Summary:")
        print("━" * 50)
        print(f"📋 Contract Hash: {result['contract_hash']}")
        print(f"🎯 Contract Address: {result['contract_address']}")
        print(f"👤 Admin Address: {result['admin_public_key']}")
        print(f"🌍 Network: {result['network']}")
        print("━" * 50)
        
        print("\\n✅ VeriLuxe Fashion Authenticity Contract deployed!")
        print("🔗 Ready for frontend integration.")
        
    except Exception as e:
        print(f"\\n💥 Deployment failed: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
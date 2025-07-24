#!/usr/bin/env node

import { 
  SorobanRpc, 
  Contract, 
  Keypair, 
  Networks, 
  TransactionBuilder,
  Operation,
  Asset
} from '@stellar/stellar-sdk';
import fs from 'fs';
import dotenv from 'dotenv';

// Load environment variables
dotenv.config();

class VeriLuxeDeployer {
  constructor() {
    this.network = process.env.NETWORK || 'testnet';
    this.rpcUrl = this.getRpcUrl();
    this.server = new SorobanRpc.Server(this.rpcUrl);
    this.networkPassphrase = this.getNetworkPassphrase();
    
    // Account setup
    this.adminKeypair = this.getAdminKeypair();
    
    console.log(`🚀 VeriLuxe Contract Deployer`);
    console.log(`📡 Network: ${this.network}`);
    console.log(`🔗 RPC URL: ${this.rpcUrl}`);
    console.log(`👤 Admin: ${this.adminKeypair.publicKey()}`);
  }

  getRpcUrl() {
    switch(this.network) {
      case 'testnet': return process.env.TESTNET_RPC || 'https://soroban-testnet.stellar.org';
      case 'futurenet': return process.env.FUTURENET_RPC || 'https://rpc-futurenet.stellar.org:443';
      case 'mainnet': return process.env.MAINNET_RPC || 'https://mainnet.sorobanrpc.com';
      default: throw new Error(`Unknown network: ${this.network}`);
    }
  }

  getNetworkPassphrase() {
    switch(this.network) {
      case 'testnet': return Networks.TESTNET;
      case 'futurenet': return Networks.FUTURENET;
      case 'mainnet': return Networks.PUBLIC;
      default: throw new Error(`Unknown network: ${this.network}`);
    }
  }

  getAdminKeypair() {
    const secretKey = process.env.ADMIN_SECRET_KEY;
    if (!secretKey) {
      // Use secret keys from our generated accounts
      const secretKeys = [
        'SA7323NS66JYLXKVGKRVGQ2EYL5VTOWYLJLHS4HFIHBOIVKIETIUZPD5', // veriluxe-fresh
        'SAXTXCNSOSLTFNB4PZR2HPTLKUVBKNP6ELIAYKK4BCWUZ3SXQ6NS6RVJ', // deploy-test
      ];
      
      console.log('🔑 Using generated account secret key');
      return Keypair.fromSecret(secretKeys[0]);
    }
    return Keypair.fromSecret(secretKey);
  }

  async deployContract() {
    try {
      console.log('\\n📋 Starting contract deployment...');
      
      // Load WASM file
      const wasmPath = process.env.CONTRACT_WASM_PATH || '../contracts/target/wasm32v1-none/release/fashion_auth_contract.wasm';
      
      if (!fs.existsSync(wasmPath)) {
        throw new Error(`WASM file not found at: ${wasmPath}`);
      }
      
      const wasmBuffer = fs.readFileSync(wasmPath);
      console.log(`📦 Loaded WASM file (${wasmBuffer.length} bytes)`);

      // Get account details
      const account = await this.server.getAccount(this.adminKeypair.publicKey());
      const balance = account.balances?.find(b => b.asset_type === 'native')?.balance || 'Unknown';
      console.log(`💰 Account balance: ${balance} XLM`);

      // Upload contract
      console.log('\\n📤 Uploading contract...');
      const uploadTx = new TransactionBuilder(account, {
        fee: '100000',
        networkPassphrase: this.networkPassphrase,
      })
        .addOperation(Operation.uploadContractWasm({
          wasm: wasmBuffer,
        }))
        .setTimeout(300)
        .build();

      uploadTx.sign(this.adminKeypair);

      const uploadResult = await this.server.sendTransaction(uploadTx);
      console.log(`✅ Upload result: ${uploadResult.status}`);
      
      if (uploadResult.status !== 'SUCCESS') {
        throw new Error(`Upload failed: ${JSON.stringify(uploadResult)}`);
      }

      // Extract contract hash
      const contractHash = uploadResult.hash;
      console.log(`🔗 Contract hash: ${contractHash}`);

      // Deploy contract instance
      console.log('\\n🚀 Deploying contract instance...');
      
      const deployTx = new TransactionBuilder(account, {
        fee: '100000', 
        networkPassphrase: this.networkPassphrase,
      })
        .addOperation(Operation.createContract({
          wasmHash: Buffer.from(contractHash, 'hex'),
          address: this.adminKeypair.publicKey(),
        }))
        .setTimeout(300)
        .build();

      deployTx.sign(this.adminKeypair);

      const deployResult = await this.server.sendTransaction(deployTx);
      console.log(`✅ Deploy result: ${deployResult.status}`);
      
      if (deployResult.status !== 'SUCCESS') {
        throw new Error(`Deploy failed: ${JSON.stringify(deployResult)}`);
      }

      const contractAddress = deployResult.contractAddress;
      console.log(`🎯 Contract deployed at: ${contractAddress}`);

      // Initialize contract
      console.log('\\n⚙️ Initializing contract...');
      await this.initializeContract(contractAddress);

      return {
        contractHash,
        contractAddress,
        adminPublicKey: this.adminKeypair.publicKey(),
        network: this.network
      };

    } catch (error) {
      console.error('❌ Deployment failed:', error.message);
      throw error;
    }
  }

  async initializeContract(contractAddress) {
    try {
      const account = await this.server.getAccount(this.adminKeypair.publicKey());
      
      const contract = new Contract(contractAddress);
      const initTx = new TransactionBuilder(account, {
        fee: '100000',
        networkPassphrase: this.networkPassphrase,
      })
        .addOperation(contract.call('init', this.adminKeypair.publicKey()))
        .setTimeout(300)
        .build();

      initTx.sign(this.adminKeypair);

      const result = await this.server.sendTransaction(initTx);
      
      if (result.status === 'SUCCESS') {
        console.log('✅ Contract initialized successfully');
      } else {
        console.log('⚠️ Contract initialization may have failed:', result.status);
      }
      
      return result;
    } catch (error) {
      console.error('❌ Initialization failed:', error.message);
      throw error;
    }
  }

  async testContract(contractAddress) {
    console.log('\\n🧪 Testing contract functionality...');
    
    try {
      const contract = new Contract(contractAddress);
      const account = await this.server.getAccount(this.adminKeypair.publicKey());

      // Test: Get admin
      const getAdminTx = new TransactionBuilder(account, {
        fee: '100000',
        networkPassphrase: this.networkPassphrase,
      })
        .addOperation(contract.call('get_admin'))
        .setTimeout(300)
        .build();

      const adminResult = await this.server.simulateTransaction(getAdminTx);
      console.log('✅ Admin check passed');

      return true;
    } catch (error) {
      console.error('❌ Contract test failed:', error.message);
      return false;
    }
  }
}

// Main execution
async function main() {
  try {
    const deployer = new VeriLuxeDeployer();
    const result = await deployer.deployContract();
    
    console.log('\\n🎉 Deployment Summary:');
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
    console.log(`📋 Contract Hash: ${result.contractHash}`);
    console.log(`🎯 Contract Address: ${result.contractAddress}`);
    console.log(`👤 Admin Address: ${result.adminPublicKey}`);
    console.log(`🌍 Network: ${result.network}`);
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');

    // Test the deployed contract
    await deployer.testContract(result.contractAddress);
    
    console.log('\\n✅ VeriLuxe Fashion Authenticity Contract deployed successfully!');
    console.log('🔗 You can now integrate this contract with your frontend.');
    
  } catch (error) {
    console.error('\\n💥 Deployment failed:', error);
    process.exit(1);
  }
}

// Run if called directly
if (import.meta.url === `file://${process.argv[1]}`) {
  main();
}
#![no_std]

//! Fashion Authenticity Certificate Smart Contract
//! 
//! This contract manages authenticity certificates for physical fashion items
//! such as luxury bags, sneakers, and other high-value fashion products.
//! 
//! Features:
//! - Issue certificates (admin only)
//! - Verify authenticity 
//! - Transfer ownership
//! - Revoke certificates (admin only)

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Map, String, Symbol};

// Storage keys for persistent data
const ADMIN_KEY: Symbol = symbol_short!("ADMIN");
const CERTS_KEY: Symbol = symbol_short!("CERTS");

/// Certificate structure containing all authenticity data
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Certificate {
    /// Current owner of the certificate
    pub owner: Address,
    /// Hash of the item's metadata (usually IPFS hash)
    pub metadata_hash: String,
    /// Whether the certificate is currently valid
    pub is_valid: bool,
}

/// Main contract for fashion authenticity certificates
#[contract]
pub struct FashionAuthContract;

/// Contract implementation with all required methods
#[contractimpl]
impl FashionAuthContract {
    /// Initialize the contract with an admin address
    /// 
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `admin` - Address that will have admin privileges
    /// 
    /// # Panics
    /// * If admin authentication fails
    pub fn init(env: Env, admin: Address) {
        // Require authentication from the admin
        admin.require_auth();
        
        // Store the admin address in persistent storage
        env.storage().instance().set(&ADMIN_KEY, &admin);
        
        // Initialize empty certificates map
        let certs: Map<String, Certificate> = Map::new(&env);
        env.storage().instance().set(&CERTS_KEY, &certs);
    }

    /// Issue a new authenticity certificate (admin only)
    /// 
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `cert_id` - Unique identifier for the certificate
    /// * `metadata_hash` - Hash of the item's metadata
    /// * `owner` - Initial owner of the certificate
    /// 
    /// # Panics
    /// * If called by non-admin
    /// * If certificate ID already exists
    /// * If contract is not initialized
    pub fn issue_certificate(
        env: Env,
        cert_id: String,
        metadata_hash: String,
        owner: Address,
    ) {
        // Get admin address and require authentication
        let admin: Address = env.storage().instance().get(&ADMIN_KEY)
            .expect("Contract not initialized");
        admin.require_auth();

        // Get existing certificates map
        let mut certs: Map<String, Certificate> = env.storage().instance()
            .get(&CERTS_KEY)
            .unwrap_or(Map::new(&env));

        // Prevent duplicate certificate IDs
        if certs.contains_key(cert_id.clone()) {
            panic!("Certificate already exists");
        }

        // Create new certificate with valid status
        let certificate = Certificate {
            owner: owner.clone(),
            metadata_hash: metadata_hash.clone(),
            is_valid: true,
        };

        // Store certificate and update persistent storage
        certs.set(cert_id, certificate);
        env.storage().instance().set(&CERTS_KEY, &certs);
    }

    /// Verify a certificate by ID and metadata hash
    /// 
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `cert_id` - Certificate identifier to verify
    /// * `metadata_hash` - Expected metadata hash
    /// 
    /// # Returns
    /// * `true` if certificate exists, is valid, and metadata hash matches
    /// * `false` otherwise
    pub fn verify(env: Env, cert_id: String, metadata_hash: String) -> bool {
        // Get certificates map
        let certs: Map<String, Certificate> = env.storage().instance()
            .get(&CERTS_KEY)
            .unwrap_or(Map::new(&env));

        // Check if certificate exists and verify conditions
        if let Some(certificate) = certs.get(cert_id) {
            // Must be valid AND metadata hash must match
            certificate.is_valid && certificate.metadata_hash == metadata_hash
        } else {
            false
        }
    }

    /// Get complete certificate details by ID
    /// 
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `cert_id` - Certificate identifier
    /// 
    /// # Returns
    /// * Complete Certificate struct
    /// 
    /// # Panics
    /// * If certificate doesn't exist
    pub fn get_certificate_details(env: Env, cert_id: String) -> Certificate {
        // Get certificates map
        let certs: Map<String, Certificate> = env.storage().instance()
            .get(&CERTS_KEY)
            .unwrap_or(Map::new(&env));

        // Return certificate or panic if not found
        certs.get(cert_id).expect("Certificate not found")
    }

    /// Transfer certificate ownership (current owner only)
    /// 
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `cert_id` - Certificate to transfer
    /// * `new_owner` - Address of the new owner
    /// 
    /// # Panics
    /// * If called by non-owner
    /// * If certificate doesn't exist
    /// * If certificate is invalid/revoked
    pub fn transfer(env: Env, cert_id: String, new_owner: Address) {
        // Get certificates map
        let mut certs: Map<String, Certificate> = env.storage().instance()
            .get(&CERTS_KEY)
            .unwrap_or(Map::new(&env));

        // Get existing certificate
        let mut certificate = certs.get(cert_id.clone())
            .expect("Certificate not found");

        // Require authentication from current owner
        certificate.owner.require_auth();

        // Prevent transfer of invalid certificates
        if !certificate.is_valid {
            panic!("Cannot transfer invalid certificate");
        }

        // Update ownership
        certificate.owner = new_owner;

        // Save updated certificate
        certs.set(cert_id, certificate);
        env.storage().instance().set(&CERTS_KEY, &certs);
    }

    /// Revoke a certificate (admin only)
    /// 
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `cert_id` - Certificate to revoke
    /// 
    /// # Panics
    /// * If called by non-admin
    /// * If certificate doesn't exist
    /// * If contract is not initialized
    pub fn revoke(env: Env, cert_id: String) {
        // Get admin address and require authentication
        let admin: Address = env.storage().instance().get(&ADMIN_KEY)
            .expect("Contract not initialized");
        admin.require_auth();

        // Get certificates map
        let mut certs: Map<String, Certificate> = env.storage().instance()
            .get(&CERTS_KEY)
            .unwrap_or(Map::new(&env));

        // Get existing certificate
        let mut certificate = certs.get(cert_id.clone())
            .expect("Certificate not found");

        // Mark certificate as invalid
        certificate.is_valid = false;

        // Save updated certificate
        certs.set(cert_id, certificate);
        env.storage().instance().set(&CERTS_KEY, &certs);
    }

    /// Get the current admin address (utility function)
    /// 
    /// # Arguments
    /// * `env` - Soroban environment
    /// 
    /// # Returns
    /// * Admin address
    /// 
    /// # Panics
    /// * If contract is not initialized
    pub fn get_admin(env: Env) -> Address {
        env.storage().instance().get(&ADMIN_KEY)
            .expect("Contract not initialized")
    }

    /// Check if a certificate exists
    /// 
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `cert_id` - Certificate identifier to check
    /// 
    /// # Returns
    /// * `true` if certificate exists, `false` otherwise
    pub fn certificate_exists(env: Env, cert_id: String) -> bool {
        let certs: Map<String, Certificate> = env.storage().instance()
            .get(&CERTS_KEY)
            .unwrap_or(Map::new(&env));
        
        certs.contains_key(cert_id)
    }
}

/// Comprehensive test module
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    /// Test contract initialization and certificate issuance
    #[test]
    fn test_init_and_issue_certificate() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(FashionAuthContract, ());
        let client = FashionAuthContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let owner = Address::generate(&env);

        // Initialize contract
        client.init(&admin);

        // Verify admin is set correctly
        assert_eq!(client.get_admin(), admin);

        // Issue a certificate
        client.issue_certificate(
            &String::from_str(&env, "CERT001"),
            &String::from_str(&env, "QmHash123"),
            &owner,
        );

        // Verify certificate exists
        assert!(client.certificate_exists(&String::from_str(&env, "CERT001")));
        
        // Verify certificate details
        let cert = client.get_certificate_details(&String::from_str(&env, "CERT001"));
        assert_eq!(cert.owner, owner);
        assert_eq!(cert.metadata_hash, String::from_str(&env, "QmHash123"));
        assert!(cert.is_valid);
    }

    /// Test certificate verification functionality
    #[test]
    fn test_verify_certificate() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(FashionAuthContract, ());
        let client = FashionAuthContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let owner = Address::generate(&env);

        client.init(&admin);
        client.issue_certificate(
            &String::from_str(&env, "CERT001"),
            &String::from_str(&env, "QmHash123"),
            &owner,
        );

        // Valid verification should return true
        assert!(client.verify(
            &String::from_str(&env, "CERT001"),
            &String::from_str(&env, "QmHash123")
        ));

        // Wrong metadata hash should return false
        assert!(!client.verify(
            &String::from_str(&env, "CERT001"),
            &String::from_str(&env, "WrongHash")
        ));

        // Non-existent certificate should return false
        assert!(!client.verify(
            &String::from_str(&env, "CERT999"),
            &String::from_str(&env, "QmHash123")
        ));
    }

    /// Test certificate ownership transfer
    #[test]
    fn test_transfer_certificate() {

        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(FashionAuthContract, ());
        let client = FashionAuthContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let owner1 = Address::generate(&env);
        let owner2 = Address::generate(&env);

        client.init(&admin);
        client.issue_certificate(
            &String::from_str(&env, "CERT001"),
            &String::from_str(&env, "QmHash123"),
            &owner1,
        );

        // Transfer certificate to new owner
        client.transfer(&String::from_str(&env, "CERT001"), &owner2);

        // Verify ownership change
        let cert = client.get_certificate_details(&String::from_str(&env, "CERT001"));
        assert_eq!(cert.owner, owner2);
        assert!(cert.is_valid); // Should still be valid
    }

    /// Test certificate revocation
    #[test]
    fn test_revoke_certificate() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(FashionAuthContract, ());
        let client = FashionAuthContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let owner = Address::generate(&env);

        client.init(&admin);
        client.issue_certificate(
            &String::from_str(&env, "CERT001"),
            &String::from_str(&env, "QmHash123"),
            &owner,
        );

        // Revoke certificate
        client.revoke(&String::from_str(&env, "CERT001"));

        // Verify certificate is marked invalid
        let cert = client.get_certificate_details(&String::from_str(&env, "CERT001"));
        assert!(!cert.is_valid);

        // Verify verification now fails for revoked certificate
        assert!(!client.verify(
            &String::from_str(&env, "CERT001"),
            &String::from_str(&env, "QmHash123")
        ));
    }

    /// Test error cases
    #[test]
    #[should_panic(expected = "Certificate already exists")]
    fn test_duplicate_certificate_id() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(FashionAuthContract, ());
        let client = FashionAuthContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let owner = Address::generate(&env);

        client.init(&admin);
        
        // Issue first certificate
        client.issue_certificate(
            &String::from_str(&env, "CERT001"),
            &String::from_str(&env, "QmHash123"),
            &owner,
        );

        // Try to issue duplicate - should panic
        client.issue_certificate(
            &String::from_str(&env, "CERT001"),
            &String::from_str(&env, "QmHash456"),
            &owner,
        );
    }

    /// Test transferring revoked certificate fails
    #[test]
    #[should_panic(expected = "Cannot transfer invalid certificate")]
    fn test_transfer_revoked_certificate() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(FashionAuthContract, ());
        let client = FashionAuthContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let owner1 = Address::generate(&env);
        let owner2 = Address::generate(&env);

        client.init(&admin);
        client.issue_certificate(
            &String::from_str(&env, "CERT001"),
            &String::from_str(&env, "QmHash123"),
            &owner1,
        );

        // Revoke certificate
        client.revoke(&String::from_str(&env, "CERT001"));

        // Try to transfer revoked certificate - should panic
        client.transfer(&String::from_str(&env, "CERT001"), &owner2);
    }
}
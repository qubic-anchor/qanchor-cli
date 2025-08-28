//! Cryptographic functions for Qubic protocol
//! 
//! Implements K12 hashing and key derivation as used by Qubic

use crate::error::{QubicRpcError, Result};
use sha3::Shake256;
use sha3::digest::{Update, ExtendableOutput, XofReader};
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer};
use base64::Engine;

/// Qubic uses K12 hash (which is based on Keccak/SHA-3)
/// For now, we use SHAKE256 as a close approximation
pub struct K12Hasher;

impl K12Hasher {
    /// Compute K12 hash of input data
    pub fn hash(input: &[u8]) -> [u8; 32] {
        let mut hasher = Shake256::default();
        hasher.update(input);
        let mut output = [0u8; 32];
        hasher.finalize_xof().read(&mut output);
        output
    }

    /// Double K12 hash (as used in Qubic key derivation)
    pub fn double_hash(input: &[u8]) -> [u8; 32] {
        let first_hash = Self::hash(input);
        Self::hash(&first_hash)
    }
}

/// Qubic key pair management
pub struct QubicKeyPair {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
}

impl QubicKeyPair {
    /// Create a new keypair from a 55-character seed (Qubic format)
    pub fn from_seed(seed: &str) -> Result<Self> {
        if seed.len() != 55 {
            return Err(QubicRpcError::crypto_error("Seed must be exactly 55 characters"));
        }

        // Convert seed to bytes and apply double K12 hash
        let seed_bytes = seed.as_bytes();
        let private_key_bytes = K12Hasher::double_hash(seed_bytes);

        // Create Ed25519 keypair
        let signing_key = SigningKey::from_bytes(&private_key_bytes);
        let verifying_key = VerifyingKey::from(&signing_key);

        Ok(Self { signing_key, verifying_key })
    }

    /// Create keypair from raw private key bytes
    pub fn from_private_key(private_key: &[u8; 32]) -> Result<Self> {
        let signing_key = SigningKey::from_bytes(private_key);
        let verifying_key = VerifyingKey::from(&signing_key);

        Ok(Self { signing_key, verifying_key })
    }

    /// Get the public key bytes
    pub fn public_key(&self) -> [u8; 32] {
        self.verifying_key.to_bytes()
    }

    /// Get the private key bytes
    pub fn private_key(&self) -> [u8; 32] {
        self.signing_key.to_bytes()
    }

    /// Sign data with this keypair
    pub fn sign(&self, data: &[u8]) -> [u8; 64] {
        let signature = self.signing_key.sign(data);
        signature.to_bytes()
    }

    /// Verify a signature
    pub fn verify(&self, data: &[u8], signature: &[u8; 64]) -> bool {
        let sig = Signature::from_bytes(signature);
        self.verifying_key.verify_strict(data, &sig).is_ok()
    }
}

/// Qubic address utilities
pub struct QubicAddress;

impl QubicAddress {
    /// Convert public key to Qubic address format
    pub fn from_public_key(public_key: &[u8; 32]) -> String {
        // Qubic addresses are typically the public key encoded in a specific format
        // For now, we'll use base64 encoding as a placeholder
        base64::engine::general_purpose::STANDARD.encode(public_key)
    }

    /// Parse Qubic address back to public key
    pub fn to_public_key(address: &str) -> Result<[u8; 32]> {
        let decoded = base64::engine::general_purpose::STANDARD.decode(address)?;
        if decoded.len() != 32 {
            return Err(QubicRpcError::crypto_error("Invalid address length"));
        }
        
        let mut public_key = [0u8; 32];
        public_key.copy_from_slice(&decoded);
        Ok(public_key)
    }

    /// Validate if a string is a valid Qubic address
    pub fn is_valid(address: &str) -> bool {
        Self::to_public_key(address).is_ok()
    }
}

/// Transaction signing utilities
pub struct TransactionSigner;

impl TransactionSigner {
    /// Sign a transaction with the given keypair
    pub fn sign_transaction(
        transaction_bytes: &[u8],
        keypair: &QubicKeyPair,
    ) -> [u8; 64] {
        keypair.sign(transaction_bytes)
    }

    /// Verify a transaction signature
    pub fn verify_transaction(
        transaction_bytes: &[u8],
        signature: &[u8; 64],
        public_key: &[u8; 32],
    ) -> Result<bool> {
        let verifying_key = VerifyingKey::from_bytes(public_key)
            .map_err(|e| QubicRpcError::crypto_error(format!("Invalid public key: {}", e)))?;
        
        let signature = Signature::from_bytes(signature);

        Ok(verifying_key.verify_strict(transaction_bytes, &signature).is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k12_hash() {
        let input = b"test data";
        let hash1 = K12Hasher::hash(input);
        let hash2 = K12Hasher::hash(input);
        
        // Same input should produce same hash
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 32);
    }

    #[test]
    fn test_keypair_creation() {
        let seed = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ123";
        assert_eq!(seed.len(), 55);
        
        let keypair = QubicKeyPair::from_seed(seed).unwrap();
        let public_key = keypair.public_key();
        assert_eq!(public_key.len(), 32);
    }

    #[test]
    fn test_signing_and_verification() {
        let seed = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ123";
        let keypair = QubicKeyPair::from_seed(seed).unwrap();
        
        let data = b"test transaction data";
        let signature = keypair.sign(data);
        
        assert!(keypair.verify(data, &signature));
        assert!(!keypair.verify(b"different data", &signature));
    }

    #[test]
    fn test_address_conversion() {
        let public_key = [1u8; 32];
        let address = QubicAddress::from_public_key(&public_key);
        let recovered = QubicAddress::to_public_key(&address).unwrap();
        
        assert_eq!(public_key, recovered);
        assert!(QubicAddress::is_valid(&address));
    }
}

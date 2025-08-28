//! Wallet functionality for Qubic transactions
//! 
//! Provides high-level interfaces for creating and signing transactions

use crate::{
    crypto::{QubicKeyPair, TransactionSigner},
    types::{Transaction, SignedTransaction},
    error::{QubicRpcError, Result},
};

/// Qubic wallet for managing keypairs and signing transactions
pub struct QubicWallet {
    keypair: QubicKeyPair,
}

impl QubicWallet {
    /// Create a new wallet from a 55-character seed
    pub fn from_seed(seed: &str) -> Result<Self> {
        let keypair = QubicKeyPair::from_seed(seed)?;
        Ok(Self { keypair })
    }

    /// Create wallet from private key bytes
    pub fn from_private_key(private_key: &[u8; 32]) -> Result<Self> {
        let keypair = QubicKeyPair::from_private_key(private_key)?;
        Ok(Self { keypair })
    }

    /// Get the public key (address) of this wallet
    pub fn public_key(&self) -> [u8; 32] {
        self.keypair.public_key()
    }

    /// Get the address string representation
    pub fn address(&self) -> String {
        crate::crypto::QubicAddress::from_public_key(&self.public_key())
    }

    /// Create and sign a transfer transaction
    pub fn create_transfer(
        &self,
        to: &[u8; 32],
        amount: u64,
        tick: u64,
    ) -> Result<SignedTransaction> {
        let transaction = Transaction::new_transfer(
            self.public_key(),
            *to,
            amount,
            tick,
        );

        self.sign_transaction(transaction)
    }

    /// Create and sign a smart contract transaction
    pub fn create_smart_contract_transaction(
        &self,
        contract: &[u8; 32],
        amount: u64,
        tick: u64,
        input_type: u16,
        input_data: Vec<u8>,
    ) -> Result<SignedTransaction> {
        let transaction = Transaction::new_smart_contract(
            self.public_key(),
            *contract,
            amount,
            tick,
            input_type,
            input_data,
        );

        self.sign_transaction(transaction)
    }

    /// Sign an existing transaction
    pub fn sign_transaction(&self, transaction: Transaction) -> Result<SignedTransaction> {
        let transaction_bytes = transaction.to_bytes();
        let signature = TransactionSigner::sign_transaction(&transaction_bytes, &self.keypair);

        Ok(SignedTransaction {
            transaction,
            signature,
        })
    }

    /// Verify a transaction signature
    pub fn verify_transaction(&self, signed_tx: &SignedTransaction) -> Result<bool> {
        let transaction_bytes = signed_tx.transaction.to_bytes();
        TransactionSigner::verify_transaction(
            &transaction_bytes,
            &signed_tx.signature,
            &signed_tx.transaction.source_public_key,
        )
    }
}

/// Transaction builder for complex transaction construction
pub struct TransactionBuilder {
    source: Option<[u8; 32]>,
    destination: Option<[u8; 32]>,
    amount: u64,
    tick: Option<u64>,
    input_type: u16,
    input_data: Vec<u8>,
}

impl TransactionBuilder {
    /// Create a new transaction builder
    pub fn new() -> Self {
        Self {
            source: None,
            destination: None,
            amount: 0,
            tick: None,
            input_type: 0,
            input_data: Vec::new(),
        }
    }

    /// Set the source address
    pub fn source(mut self, source: [u8; 32]) -> Self {
        self.source = Some(source);
        self
    }

    /// Set the destination address
    pub fn destination(mut self, destination: [u8; 32]) -> Self {
        self.destination = Some(destination);
        self
    }

    /// Set the transfer amount
    pub fn amount(mut self, amount: u64) -> Self {
        self.amount = amount;
        self
    }

    /// Set the transaction tick
    pub fn tick(mut self, tick: u64) -> Self {
        self.tick = Some(tick);
        self
    }

    /// Set smart contract input type
    pub fn input_type(mut self, input_type: u16) -> Self {
        self.input_type = input_type;
        self
    }

    /// Set smart contract input data
    pub fn input_data(mut self, input_data: Vec<u8>) -> Self {
        self.input_data = input_data;
        self
    }

    /// Build the transaction
    pub fn build(self) -> Result<Transaction> {
        let source = self.source.ok_or_else(|| {
            QubicRpcError::transaction_error("Source address is required")
        })?;

        let destination = self.destination.ok_or_else(|| {
            QubicRpcError::transaction_error("Destination address is required")
        })?;

        let tick = self.tick.ok_or_else(|| {
            QubicRpcError::transaction_error("Tick is required")
        })?;

        if self.input_type == 0 && !self.input_data.is_empty() {
            return Err(QubicRpcError::transaction_error(
                "Transfer transactions cannot have input data"
            ));
        }

        if self.input_type > 0 {
            Ok(Transaction::new_smart_contract(
                source,
                destination,
                self.amount,
                tick,
                self.input_type,
                self.input_data,
            ))
        } else {
            Ok(Transaction::new_transfer(
                source,
                destination,
                self.amount,
                tick,
            ))
        }
    }
}

impl Default for TransactionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_creation() {
        let seed = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ123";
        let wallet = QubicWallet::from_seed(seed).unwrap();
        
        let public_key = wallet.public_key();
        assert_eq!(public_key.len(), 32);
        
        let address = wallet.address();
        assert!(!address.is_empty());
    }

    #[test]
    fn test_transfer_transaction() {
        let seed = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ123";
        let wallet = QubicWallet::from_seed(seed).unwrap();
        
        let to = [1u8; 32];
        let amount = 1000;
        let tick = 12345;
        
        let signed_tx = wallet.create_transfer(&to, amount, tick).unwrap();
        
        assert_eq!(signed_tx.transaction.source_public_key, wallet.public_key());
        assert_eq!(signed_tx.transaction.destination_public_key, to);
        assert_eq!(signed_tx.transaction.amount, amount);
        assert_eq!(signed_tx.transaction.tick, tick);
        assert_eq!(signed_tx.transaction.input_type, 0);
        assert_eq!(signed_tx.signature.len(), 64);
        
        // Verify the signature
        assert!(wallet.verify_transaction(&signed_tx).unwrap());
    }

    #[test]
    fn test_smart_contract_transaction() {
        let seed = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ123";
        let wallet = QubicWallet::from_seed(seed).unwrap();
        
        let contract = [2u8; 32];
        let amount = 500;
        let tick = 54321;
        let input_type = 1;
        let input_data = vec![1, 2, 3, 4, 5];
        
        let signed_tx = wallet.create_smart_contract_transaction(
            &contract,
            amount,
            tick,
            input_type,
            input_data.clone(),
        ).unwrap();
        
        assert_eq!(signed_tx.transaction.source_public_key, wallet.public_key());
        assert_eq!(signed_tx.transaction.destination_public_key, contract);
        assert_eq!(signed_tx.transaction.amount, amount);
        assert_eq!(signed_tx.transaction.tick, tick);
        assert_eq!(signed_tx.transaction.input_type, input_type);
        assert_eq!(signed_tx.transaction.input_data, input_data);
        assert_eq!(signed_tx.signature.len(), 64);
        
        // Verify the signature
        assert!(wallet.verify_transaction(&signed_tx).unwrap());
    }

    #[test]
    fn test_transaction_builder() {
        let source = [1u8; 32];
        let destination = [2u8; 32];
        let amount = 1000;
        let tick = 12345;
        
        let transaction = TransactionBuilder::new()
            .source(source)
            .destination(destination)
            .amount(amount)
            .tick(tick)
            .build()
            .unwrap();
        
        assert_eq!(transaction.source_public_key, source);
        assert_eq!(transaction.destination_public_key, destination);
        assert_eq!(transaction.amount, amount);
        assert_eq!(transaction.tick, tick);
        assert_eq!(transaction.input_type, 0);
        assert!(transaction.input_data.is_empty());
    }

    #[test]
    fn test_transaction_builder_smart_contract() {
        let source = [1u8; 32];
        let contract = [2u8; 32];
        let amount = 500;
        let tick = 54321;
        let input_type = 1;
        let input_data = vec![1, 2, 3];
        
        let transaction = TransactionBuilder::new()
            .source(source)
            .destination(contract)
            .amount(amount)
            .tick(tick)
            .input_type(input_type)
            .input_data(input_data.clone())
            .build()
            .unwrap();
        
        assert_eq!(transaction.source_public_key, source);
        assert_eq!(transaction.destination_public_key, contract);
        assert_eq!(transaction.amount, amount);
        assert_eq!(transaction.tick, tick);
        assert_eq!(transaction.input_type, input_type);
        assert_eq!(transaction.input_data, input_data);
    }

    #[test]
    fn test_transaction_builder_validation() {
        // Missing source should fail
        let result = TransactionBuilder::new()
            .destination([2u8; 32])
            .amount(1000)
            .tick(12345)
            .build();
        
        assert!(result.is_err());

        // Transfer with input data should fail
        let result = TransactionBuilder::new()
            .source([1u8; 32])
            .destination([2u8; 32])
            .amount(1000)
            .tick(12345)
            .input_type(0)  // Transfer
            .input_data(vec![1, 2, 3])  // Should not have data
            .build();
        
        assert!(result.is_err());
    }
}

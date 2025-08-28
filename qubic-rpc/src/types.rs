//! Qubic RPC API types and structures

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Tick information from the last processed tick
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickInfo {
    #[serde(rename = "tickNumber")]
    pub tick_number: u64,
    pub epoch: u64,
}

/// Skipped tick range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkippedTick {
    #[serde(rename = "startTick")]
    pub start_tick: u64,
    #[serde(rename = "endTick")]
    pub end_tick: u64,
}

/// Processed tick interval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickInterval {
    #[serde(rename = "initialProcessedTick")]
    pub initial_processed_tick: u64,
    #[serde(rename = "lastProcessedTick")]
    pub last_processed_tick: u64,
}

/// Processed tick intervals per epoch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochInterval {
    pub epoch: u64,
    pub intervals: Vec<TickInterval>,
}

/// Network status response from `/v1/status` (actual Qubic mainnet format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatus {
    /// Last processed tick information
    #[serde(rename = "lastProcessedTick")]
    pub last_processed_tick: TickInfo,
    /// Last processed ticks per epoch (epoch -> tick mapping)
    #[serde(rename = "lastProcessedTicksPerEpoch")]
    pub last_processed_ticks_per_epoch: std::collections::HashMap<String, u64>,
    /// Skipped tick ranges
    #[serde(rename = "skippedTicks")]
    pub skipped_ticks: Vec<SkippedTick>,
    /// Processed tick intervals per epoch
    #[serde(rename = "processedTickIntervalsPerEpoch")]
    pub processed_tick_intervals_per_epoch: Vec<EpochInterval>,
    /// Empty ticks per epoch
    #[serde(rename = "emptyTicksPerEpoch")]
    pub empty_ticks_per_epoch: std::collections::HashMap<String, u64>,
}

/// Legacy network status for backwards compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyNetworkStatus {
    /// Current tick/block number
    pub tick: u64,
    /// Epoch information
    pub epoch: u64,
    /// Number of entities (accounts)
    pub number_of_entities: u64,
    /// Total supply
    pub total_supply: u64,
    /// Last update timestamp
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub last_update: Option<DateTime<Utc>>,
}

/// Smart contract query request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartContractQueryRequest {
    /// Contract index/ID
    #[serde(rename = "contractIndex")]
    pub contract_index: u32,
    /// Input type identifier
    #[serde(rename = "inputType")]
    pub input_type: u32,
    /// Size of input data
    #[serde(rename = "inputSize")]
    pub input_size: u32,
    /// Base64 encoded request data
    #[serde(rename = "requestData")]
    pub request_data: String,
}

/// Smart contract query response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartContractQueryResponse {
    /// Base64 encoded response data
    #[serde(rename = "responseData")]
    pub response_data: String,
}

/// Transaction structure for Qubic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Source public key (32 bytes)
    pub source_public_key: [u8; 32],
    /// Destination public key (32 bytes)
    pub destination_public_key: [u8; 32],
    /// Amount to transfer
    pub amount: u64,
    /// Current tick
    pub tick: u64,
    /// Input type (0 for transfers, > 0 for smart contracts)
    pub input_type: u16,
    /// Input size
    pub input_size: u16,
    /// Additional data for smart contracts
    pub input_data: Vec<u8>,
}

/// Signed transaction ready for broadcast
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedTransaction {
    /// The base transaction
    #[serde(flatten)]
    pub transaction: Transaction,
    /// Signature (64 bytes) - serialized as base64
    #[serde(with = "serde_bytes")]
    pub signature: [u8; 64],
}

// Helper module for serializing byte arrays
mod serde_bytes {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use base64::Engine;

    pub fn serialize<S>(bytes: &[u8; 64], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
        encoded.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 64], D::Error>
    where
        D: Deserializer<'de>,
    {
        let encoded = String::deserialize(deserializer)?;
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&encoded)
            .map_err(serde::de::Error::custom)?;
        
        if decoded.len() != 64 {
            return Err(serde::de::Error::custom("Invalid signature length"));
        }
        
        let mut signature = [0u8; 64];
        signature.copy_from_slice(&decoded);
        Ok(signature)
    }
}

/// Transaction broadcast request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastTransactionRequest {
    /// Base64 encoded signed transaction
    #[serde(rename = "encodedTransaction")]
    pub encoded_transaction: String,
}

/// Transaction broadcast response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastTransactionResponse {
    /// Transaction ID/hash
    #[serde(rename = "txId")]
    pub tx_id: String,
    /// Broadcast status
    pub status: String,
    /// Additional message
    pub message: Option<String>,
}

/// Entity (account) information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityInfo {
    /// Public key/address
    pub public_key: [u8; 32],
    /// Current balance
    pub balance: u64,
    /// Current tick
    pub tick: u64,
    /// Latest incoming transfer tick
    pub latest_incoming_transfer_tick: u64,
    /// Latest outgoing transfer tick
    pub latest_outgoing_transfer_tick: u64,
}

/// Block information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockInfo {
    /// Block tick/number
    pub tick: u64,
    /// Epoch
    pub epoch: u64,
    /// Number of transactions
    pub number_of_transactions: u32,
    /// Transactions in this block
    pub transactions: Vec<SignedTransaction>,
    /// Timestamp
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

/// Computor information (for Qubic consensus)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputorInfo {
    /// Computor index (0-675)
    pub index: u16,
    /// Public key
    pub public_key: [u8; 32],
    /// Is online/active
    pub is_online: bool,
    /// Score/reliability
    pub score: u32,
}

/// Network quorum information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuorumInfo {
    /// Current quorum size (should be >= 451 for Qubic)
    pub quorum_size: u16,
    /// Total computors
    pub total_computors: u16,
    /// Online computors
    pub online_computors: u16,
    /// Current epoch
    pub epoch: u64,
}

/// Smart contract info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartContractInfo {
    /// Contract index
    pub index: u32,
    /// Contract code size
    pub code_size: u32,
    /// Contract state size
    pub state_size: u32,
    /// Creator public key
    pub creator: [u8; 32],
    /// Creation tick
    pub creation_tick: u64,
}

impl Transaction {
    /// Create a new transfer transaction
    pub fn new_transfer(
        source: [u8; 32],
        destination: [u8; 32],
        amount: u64,
        tick: u64,
    ) -> Self {
        Self {
            source_public_key: source,
            destination_public_key: destination,
            amount,
            tick,
            input_type: 0, // 0 for transfers
            input_size: 0,
            input_data: Vec::new(),
        }
    }

    /// Create a new smart contract transaction
    pub fn new_smart_contract(
        source: [u8; 32],
        contract: [u8; 32],
        amount: u64,
        tick: u64,
        input_type: u16,
        input_data: Vec<u8>,
    ) -> Self {
        Self {
            source_public_key: source,
            destination_public_key: contract,
            amount,
            tick,
            input_type,
            input_size: input_data.len() as u16,
            input_data,
        }
    }

    /// Get the transaction data for signing
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.source_public_key);
        bytes.extend_from_slice(&self.destination_public_key);
        bytes.extend_from_slice(&self.amount.to_le_bytes());
        bytes.extend_from_slice(&self.tick.to_le_bytes());
        bytes.extend_from_slice(&self.input_type.to_le_bytes());
        bytes.extend_from_slice(&self.input_size.to_le_bytes());
        bytes.extend_from_slice(&self.input_data);
        bytes
    }
}

// ================================
// Qubic RPC 2.0 API Types
// ================================

/// V2 API Query filters for advanced searching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFilters {
    /// Filter by input type
    #[serde(rename = "inputType", skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
    /// Filter by transaction type  
    #[serde(rename = "transactionType", skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<String>,
    /// Filter by execution status
    #[serde(rename = "executionStatus", skip_serializing_if = "Option::is_none")]
    pub execution_status: Option<String>,
}

/// V2 API Query ranges for numeric/string filtering
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueryRanges {
    /// Amount range filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<RangeFilter>,
    /// Tick number range filter  
    #[serde(rename = "tickNumber", skip_serializing_if = "Option::is_none")]
    pub tick_number: Option<RangeFilter>,
    /// Timestamp range filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<RangeFilter>,
}

/// Range filter for numeric values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeFilter {
    /// Greater than
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt: Option<String>,
    /// Greater than or equal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gte: Option<String>,
    /// Less than
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<String>,
    /// Less than or equal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lte: Option<String>,
}

/// V2 API Pagination parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    /// Number of results per page (max 1024)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
    /// Offset for pagination (max 10000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            size: Some(100),
            offset: Some(0),
        }
    }
}

/// V2 API Request for getTransactionsForIdentity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsForIdentityRequest {
    /// Identity address to query
    pub identity: String,
    /// Optional filters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<QueryFilters>,
    /// Optional ranges
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranges: Option<QueryRanges>,
    /// Pagination parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}

/// V2 API Enhanced transaction response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionV2 {
    /// Transaction hash/ID
    #[serde(rename = "txId")]
    pub tx_id: String,
    /// Source identity address
    #[serde(rename = "sourceId")]
    pub source_id: String,
    /// Destination identity address  
    #[serde(rename = "destId")]
    pub dest_id: String,
    /// Transaction amount
    pub amount: String,
    /// Tick number when transaction was processed
    #[serde(rename = "tickNumber")]
    pub tick_number: u64,
    /// Input type for smart contract calls
    #[serde(rename = "inputType")]
    pub input_type: u16,
    /// Input size
    #[serde(rename = "inputSize")]
    pub input_size: u16,
    /// Input data (base64 encoded)
    #[serde(rename = "inputHex")]
    pub input_hex: Option<String>,
    /// Signature (base64 encoded)
    #[serde(rename = "signatureHex")]
    pub signature_hex: String,
    /// Transaction timestamp
    pub timestamp: Option<DateTime<Utc>>,
    /// Execution status
    #[serde(rename = "executionStatus", skip_serializing_if = "Option::is_none")]
    pub execution_status: Option<String>,
    /// Money flew status
    #[serde(rename = "moneyFlew", skip_serializing_if = "Option::is_none")]
    pub money_flew: Option<bool>,
}

/// V2 API Transactions response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsV2Response {
    /// List of transactions
    pub transactions: Vec<TransactionV2>,
    /// Total count of matching transactions
    #[serde(rename = "totalCount")]
    pub total_count: u64,
    /// Current page offset
    pub offset: u32,
    /// Page size used
    pub size: u32,
}

/// V2 API Enhanced tick data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickDataV2 {
    /// Tick number
    #[serde(rename = "tickNumber")]
    pub tick_number: u64,
    /// Epoch number
    pub epoch: u64,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Number of transactions in this tick
    #[serde(rename = "transactionCount")]
    pub transaction_count: u32,
    /// Block hash/signature
    #[serde(rename = "signature")]
    pub signature: String,
    /// Computor states
    #[serde(rename = "computorStates", skip_serializing_if = "Option::is_none")]
    pub computor_states: Option<HashMap<String, String>>,
}

/// V2 API Request for tick data queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickDataRequest {
    /// Optional filters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<QueryFilters>,
    /// Optional ranges (tick number, timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranges: Option<QueryRanges>,
    /// Pagination parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}

/// V2 API Tick data response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickDataV2Response {
    /// List of tick data
    #[serde(rename = "tickData")]
    pub tick_data: Vec<TickDataV2>,
    /// Total count of matching ticks
    #[serde(rename = "totalCount")]
    pub total_count: u64,
    /// Current page offset
    pub offset: u32,
    /// Page size used
    pub size: u32,
}

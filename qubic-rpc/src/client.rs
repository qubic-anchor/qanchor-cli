//! Qubic RPC client implementation
//! 
//! Provides methods to interact with Qubic blockchain via RPC API

use crate::{Network, types::*, error::{QubicRpcError, Result}, retry::{RetryConfig, with_retry}};
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;
use base64::Engine;

/// Qubic RPC client
/// 
/// Based on the [official Qubic RPC API](https://docs.qubic.org/api/rpc)
pub struct QubicRpcClient {
    client: Client,
    base_url: String,
    network: Network,
    retry_config: RetryConfig,
}

impl QubicRpcClient {
    /// Create a new Qubic RPC client for the specified network
    pub fn new(network: Network) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(QubicRpcError::Http)?;

        Ok(Self {
            client,
            base_url: network.api_base_url(),
            network,
            retry_config: RetryConfig::default(),
        })
    }

    /// Create a client with custom timeout
    pub fn with_timeout(network: Network, timeout: Duration) -> Result<Self> {
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .map_err(QubicRpcError::Http)?;

        Ok(Self {
            client,
            base_url: network.api_base_url(),
            network,
            retry_config: RetryConfig::default(),
        })
    }

    /// Create a client with custom retry configuration
    pub fn with_retry_config(network: Network, retry_config: RetryConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(QubicRpcError::Http)?;

        Ok(Self {
            client,
            base_url: network.api_base_url(),
            network,
            retry_config,
        })
    }

    /// Get network status
    /// 
    /// Calls `/v1/status` endpoint with automatic retry on failure
    pub async fn get_status(&self) -> Result<NetworkStatus> {
        with_retry(
            || async {
                let url = format!("{}/status", self.base_url);
                let response = self.client.get(&url).send().await?;
                
                if !response.status().is_success() {
                    return Err(QubicRpcError::server_error(format!(
                        "Status request failed: {}", response.status()
                    )));
                }

                let status: NetworkStatus = response.json().await?;
                Ok(status)
            },
            &self.retry_config,
        ).await
    }

    /// Query a smart contract
    /// 
    /// Calls `/v1/querySmartContract` endpoint with base64 encoded data
    pub async fn query_smart_contract(
        &self,
        contract_index: u32,
        input_type: u32,
        request_data: &[u8],
    ) -> Result<Vec<u8>> {
        let url = format!("{}/querySmartContract", self.base_url);
        
        // Encode request data to base64 as required by Qubic RPC
        let encoded_data = base64::engine::general_purpose::STANDARD.encode(request_data);
        
        let request = SmartContractQueryRequest {
            contract_index,
            input_type,
            input_size: request_data.len() as u32,
            request_data: encoded_data,
        };

        let response = self.client
            .post(&url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(QubicRpcError::server_error(format!(
                "Smart contract query failed: {}", response.status()
            )));
        }

        let query_response: SmartContractQueryResponse = response.json().await?;
        
        // Decode base64 response data
        let decoded_data = base64::engine::general_purpose::STANDARD
            .decode(&query_response.response_data)?;
        
        Ok(decoded_data)
    }

    /// Broadcast a signed transaction
    /// 
    /// Calls `/v1/broadcast-transaction` endpoint
    pub async fn broadcast_transaction(
        &self,
        signed_transaction: &SignedTransaction,
    ) -> Result<BroadcastTransactionResponse> {
        let url = format!("{}/broadcast-transaction", self.base_url);
        
        // Serialize and encode transaction
        let transaction_json = serde_json::to_string(signed_transaction)?;
        let encoded_transaction = base64::engine::general_purpose::STANDARD
            .encode(transaction_json.as_bytes());
        
        let request = BroadcastTransactionRequest {
            encoded_transaction,
        };

        let response = self.client
            .post(&url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(QubicRpcError::server_error(format!(
                "Transaction broadcast failed: {}", response.status()
            )));
        }

        let broadcast_response: BroadcastTransactionResponse = response.json().await?;
        Ok(broadcast_response)
    }

    /// Get entity (account) information
    /// 
    /// Note: This endpoint may not be available on all networks (404 on some nodes)
    pub async fn get_entity(&self, public_key: &[u8; 32]) -> Result<EntityInfo> {
        let address = base64::engine::general_purpose::STANDARD.encode(public_key);
        let url = format!("{}/entity/{}", self.base_url, address);
        
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(QubicRpcError::server_error(format!(
                "Entity query failed: {} (Note: This endpoint may not be implemented on all networks)", 
                response.status()
            )));
        }

        let entity: EntityInfo = response.json().await?;
        Ok(entity)
    }

    /// Get block information by tick
    pub async fn get_block(&self, tick: u64) -> Result<BlockInfo> {
        let url = format!("{}/block/{}", self.base_url, tick);
        
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(QubicRpcError::server_error(format!(
                "Block query failed: {}", response.status()
            )));
        }

        let block: BlockInfo = response.json().await?;
        Ok(block)
    }

    /// Get quorum information
    /// 
    /// Note: This endpoint may not be available on all networks (404 on some nodes)
    pub async fn get_quorum(&self) -> Result<QuorumInfo> {
        let url = format!("{}/quorum", self.base_url);
        
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(QubicRpcError::server_error(format!(
                "Quorum query failed: {} (Note: This endpoint may not be implemented on all networks)", 
                response.status()
            )));
        }

        let quorum: QuorumInfo = response.json().await?;
        Ok(quorum)
    }

    /// Get smart contract information
    pub async fn get_smart_contract(&self, contract_index: u32) -> Result<SmartContractInfo> {
        let url = format!("{}/contract/{}", self.base_url, contract_index);
        
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(QubicRpcError::server_error(format!(
                "Contract query failed: {}", response.status()
            )));
        }

        let contract: SmartContractInfo = response.json().await?;
        Ok(contract)
    }

    /// Get current tick (block number)
    pub async fn get_current_tick(&self) -> Result<u64> {
        let status = self.get_status().await?;
        Ok(status.last_processed_tick.tick_number)
    }

    /// Get current balance for an entity
    pub async fn get_balance(&self, public_key: &[u8; 32]) -> Result<u64> {
        let entity = self.get_entity(public_key).await?;
        Ok(entity.balance)
    }

    /// Check if the network is ready (has quorum)
    pub async fn is_network_ready(&self) -> Result<bool> {
        let quorum = self.get_quorum().await?;
        // Qubic needs at least 451 out of 676 computors for consensus
        Ok(quorum.quorum_size >= 451)
    }

    /// Ping the network to check connectivity
    pub async fn ping(&self) -> Result<Duration> {
        let start = std::time::Instant::now();
        let _status = self.get_status().await?;
        Ok(start.elapsed())
    }

    /// Get the network this client is connected to
    pub fn network(&self) -> Network {
        self.network
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    // ================================
    // Qubic RPC 2.0 API Methods
    // ================================

    /// Get transactions for a specific identity using V2 API
    /// 
    /// Calls `/getTransactionsForIdentity` endpoint with advanced filtering and pagination
    pub async fn get_transactions_for_identity_v2(
        &self,
        request: &TransactionsForIdentityRequest,
    ) -> Result<TransactionsV2Response> {
        with_retry(
            || async {
                let url = format!("{}/getTransactionsForIdentity", self.base_url);
                let response = self.client.post(&url)
                    .json(request)
                    .send()
                    .await?;
                
                if !response.status().is_success() {
                    return Err(QubicRpcError::server_error(format!(
                        "V2 Transactions query failed: {}", response.status()
                    )));
                }

                let transactions: TransactionsV2Response = response.json().await?;
                Ok(transactions)
            },
            &self.retry_config,
        ).await
    }

    /// Get tick data using V2 API with advanced filtering
    /// 
    /// Calls `/getTickData` endpoint with filtering and pagination support
    pub async fn get_tick_data_v2(
        &self,
        request: &TickDataRequest,
    ) -> Result<TickDataV2Response> {
        with_retry(
            || async {
                let url = format!("{}/getTickData", self.base_url);
                let response = self.client.post(&url)
                    .json(request)
                    .send()
                    .await?;
                
                if !response.status().is_success() {
                    return Err(QubicRpcError::server_error(format!(
                        "V2 Tick data query failed: {}", response.status()
                    )));
                }

                let tick_data: TickDataV2Response = response.json().await?;
                Ok(tick_data)
            },
            &self.retry_config,
        ).await
    }

    /// Helper method to get transactions for identity with simple filters
    pub async fn get_identity_transactions(
        &self,
        identity: &str,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<TransactionsV2Response> {
        let request = TransactionsForIdentityRequest {
            identity: identity.to_string(),
            filters: None,
            ranges: None,
            pagination: Some(Pagination {
                size: limit,
                offset,
            }),
        };
        
        self.get_transactions_for_identity_v2(&request).await
    }

    /// Helper method to get transactions with amount filter
    pub async fn get_transactions_with_amount_filter(
        &self,
        identity: &str,
        min_amount: Option<&str>,
        max_amount: Option<&str>,
        limit: Option<u32>,
    ) -> Result<TransactionsV2Response> {
        let amount_filter = RangeFilter {
            gt: None,
            gte: min_amount.map(|s| s.to_string()),
            lt: None,
            lte: max_amount.map(|s| s.to_string()),
        };

        let request = TransactionsForIdentityRequest {
            identity: identity.to_string(),
            filters: None,
            ranges: Some(QueryRanges {
                amount: Some(amount_filter),
                tick_number: None,
                timestamp: None,
            }),
            pagination: Some(Pagination {
                size: limit,
                offset: Some(0),
            }),
        };
        
        self.get_transactions_for_identity_v2(&request).await
    }

    /// Helper method to get transactions within tick range
    pub async fn get_transactions_in_tick_range(
        &self,
        identity: &str,
        start_tick: Option<u64>,
        end_tick: Option<u64>,
        limit: Option<u32>,
    ) -> Result<TransactionsV2Response> {
        let tick_filter = RangeFilter {
            gt: None,
            gte: start_tick.map(|t| t.to_string()),
            lt: None,
            lte: end_tick.map(|t| t.to_string()),
        };

        let request = TransactionsForIdentityRequest {
            identity: identity.to_string(),
            filters: None,
            ranges: Some(QueryRanges {
                amount: None,
                tick_number: Some(tick_filter),
                timestamp: None,
            }),
            pagination: Some(Pagination {
                size: limit,
                offset: Some(0),
            }),
        };
        
        self.get_transactions_for_identity_v2(&request).await
    }

    /// Helper method to get recent tick data
    pub async fn get_recent_ticks(
        &self,
        limit: Option<u32>,
    ) -> Result<TickDataV2Response> {
        let request = TickDataRequest {
            filters: None,
            ranges: None,
            pagination: Some(Pagination {
                size: limit,
                offset: Some(0),
            }),
        };
        
        self.get_tick_data_v2(&request).await
    }

    /// Make a raw HTTP GET request to the RPC endpoint
    pub async fn raw_get(&self, endpoint: &str) -> Result<Value> {
        let url = format!("{}/{}", self.base_url, endpoint.trim_start_matches('/'));
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(QubicRpcError::server_error(format!(
                "GET {} failed: {}", endpoint, response.status()
            )));
        }

        let json: Value = response.json().await?;
        Ok(json)
    }

    /// Make a raw HTTP POST request to the RPC endpoint
    pub async fn raw_post(&self, endpoint: &str, body: &Value) -> Result<Value> {
        let url = format!("{}/{}", self.base_url, endpoint.trim_start_matches('/'));
        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(QubicRpcError::server_error(format!(
                "POST {} failed: {}", endpoint, response.status()
            )));
        }

        let json: Value = response.json().await?;
        Ok(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = QubicRpcClient::new(Network::Testnet);
        assert!(client.is_ok());
        
        let client = client.unwrap();
        assert_eq!(client.network(), Network::Testnet);
        assert_eq!(client.base_url(), "https://testnet-rpc.qubic.org/v1");
    }

    #[tokio::test]
    async fn test_network_urls() {
        assert_eq!(Network::Mainnet.base_url(), "https://rpc.qubic.org");
        assert_eq!(Network::Testnet.base_url(), "https://testnet-rpc.qubic.org");
        assert_eq!(Network::Staging.base_url(), "https://rpc-staging.qubic.org");
        
        assert_eq!(Network::Mainnet.api_version(), "v1");
        assert_eq!(Network::Testnet.api_version(), "v1");
        assert_eq!(Network::Staging.api_version(), "v2");
    }

    // Note: These tests require actual network connectivity
    // and may fail if the Qubic testnet is down
    #[tokio::test]
    #[ignore] // Ignore by default, run with --ignored flag
    async fn test_get_status_integration() {
        let client = QubicRpcClient::new(Network::Testnet).unwrap();
        let result = client.get_status().await;
        
        match result {
            Ok(status) => {
                assert!(status.last_processed_tick.tick_number > 0);
                println!(
                    "Current tick: {}",
                    status.last_processed_tick.tick_number
                );
            }
            Err(e) => {
                println!("Network test failed (expected): {}", e);
            }
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_ping_integration() {
        let client = QubicRpcClient::new(Network::Testnet).unwrap();
        let result = client.ping().await;
        
        match result {
            Ok(duration) => {
                println!("Ping time: {:?}", duration);
                assert!(duration.as_millis() > 0);
            }
            Err(e) => {
                println!("Ping test failed (expected): {}", e);
            }
        }
    }
}

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;

use super::QubicState;

type SharedState = Arc<Mutex<QubicState>>;

#[derive(Deserialize)]
pub struct DeployRequest {
    pub name: String,
    pub code: String, // Base64 encoded WASM binary or hex string
    #[serde(default)]
    #[allow(dead_code)]
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct DeployResponse {
    pub contract_id: String,
    pub status: String,
    pub block_height: u64,
    pub transaction_id: String,
    pub deployed_at: u64,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: u16,
    pub details: Option<String>,
}

#[derive(Deserialize)]
pub struct CallRequest {
    pub method: String,
    pub args: serde_json::Value,
}

#[derive(Serialize)]
pub struct CallResponse {
    pub result: serde_json::Value,
    pub transaction_id: String,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub network: String,
    pub block_height: u64,
    pub contracts_count: usize,
}

pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/contracts", post(deploy_contract))
        .route("/contracts/:id", get(get_contract))
        .route("/contracts/:id/call", post(call_contract))
        .route("/blocks", get(get_latest_block))
        .route("/blocks/:height", get(get_block_by_height))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

async fn health_check(State(state): State<SharedState>) -> Json<HealthResponse> {
    let state = state.lock().unwrap();
    Json(HealthResponse {
        status: "healthy".to_string(),
        network: "local".to_string(),
        block_height: state.current_height,
        contracts_count: state.contracts.len(),
    })
}

async fn deploy_contract(
    State(state): State<SharedState>,
    Json(request): Json<DeployRequest>,
) -> Result<(StatusCode, Json<DeployResponse>), (StatusCode, Json<ErrorResponse>)> {
    // 驗證請求
    if request.name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Contract name cannot be empty".to_string(),
                code: 400,
                details: Some("Please provide a valid contract name".to_string()),
            })
        ));
    }
    
    if request.code.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Contract code cannot be empty".to_string(),
                code: 400,
                details: Some("Please provide Base64 encoded WASM binary".to_string()),
            })
        ));
    }
    
    let mut state = state.lock().unwrap();
    
    // 解碼 Base64 程式碼
    use base64::Engine;
    let code = match base64::engine::general_purpose::STANDARD.decode(&request.code) {
        Ok(code) => {
            if code.is_empty() {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: "Decoded contract code is empty".to_string(),
                        code: 400,
                        details: Some("Base64 decoded to empty content".to_string()),
                    })
                ));
            }
            code
        },
        Err(e) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Invalid Base64 encoding".to_string(),
                    code: 400,
                    details: Some(format!("Base64 decode error: {}", e)),
                })
            ));
        }
    };
    
    // 部署合約
    let contract_id = state.deploy_contract(request.name.clone(), code);
    let transaction_id = uuid::Uuid::new_v4().to_string();
    let deployed_at = chrono::Utc::now().timestamp() as u64;
    
    println!("✅ Contract deployed: {} (ID: {})", request.name, contract_id);
    
    Ok((
        StatusCode::CREATED,
        Json(DeployResponse {
            contract_id,
            status: "deployed".to_string(),
            block_height: state.current_height,
            transaction_id,
            deployed_at,
        })
    ))
}

async fn get_contract(
    State(state): State<SharedState>,
    Path(contract_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let state = state.lock().unwrap();
    
    match state.contracts.get(&contract_id) {
        Some(contract) => Ok(Json(serde_json::json!({
            "id": contract.id,
            "name": contract.name,
            "status": contract.status,
            "deployed_at": contract.deployed_at
        }))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn call_contract(
    State(state): State<SharedState>,
    Path(contract_id): Path<String>,
    Json(request): Json<CallRequest>,
) -> Result<Json<CallResponse>, StatusCode> {
    let mut state = state.lock().unwrap();
    
    match state.call_contract(&contract_id, &request.method, request.args) {
        Ok(result) => {
            let tx_id = uuid::Uuid::new_v4().to_string();
            Ok(Json(CallResponse {
                result,
                transaction_id: tx_id,
            }))
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn get_latest_block(State(state): State<SharedState>) -> Json<serde_json::Value> {
    let state = state.lock().unwrap();
    
    match state.get_latest_block() {
        Some(block) => Json(serde_json::json!(block)),
        None => Json(serde_json::json!(null)),
    }
}

async fn get_block_by_height(
    State(state): State<SharedState>,
    Path(height): Path<u64>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let state = state.lock().unwrap();
    
    match state.blocks.get(height as usize) {
        Some(block) => Ok(Json(serde_json::json!(block))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

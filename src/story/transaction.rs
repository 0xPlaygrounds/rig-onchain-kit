use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::STORY_API_URL;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTransactionResponse {
    pub data: TransactionData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionData {
    pub id: String,

    #[serde(rename = "blockNumber")]
    pub block_number: String,

    #[serde(rename = "blockTimestamp")]
    pub block_timestamp: String,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "actionType")]
    pub action_type: Option<String>,

    pub initiator: Option<String>,

    #[serde(rename = "ipId")]
    pub ip_id: Option<String>,

    #[serde(rename = "resourceId")]
    pub resource_id: Option<String>,

    #[serde(rename = "resourceType")]
    pub resource_type: Option<String>,

    pub tx_hash: Option<String>,

    #[serde(rename = "logIndex")]
    pub log_index: Option<String>,

    #[serde(rename = "transactionIndex")]
    pub transaction_index: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StoryConfig {
    pub api_key: String,
    pub chain: String,
}

impl StoryConfig {
    pub fn new(api_key: String, chain: String) -> Self {
        StoryConfig { api_key, chain }
    }
}

/// Retrieve a Transaction
///
/// # Arguments
///
/// * `config` - API Config
/// * `trx_id` - Transaction ID
///
/// # Returns
///
/// GetTransactionResponse
pub async fn get_a_transaction(
    config: &StoryConfig,
    trx_id: &str,
) -> Result<GetTransactionResponse> {
    let url = format!("{}/transactions/{}", STORY_API_URL, trx_id);

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("X-Api-Key", &config.api_key)
        .header("X-Chain", config.chain.clone())
        .header("accept", "application/json")
        .send()
        .await?
        .json::<GetTransactionResponse>()
        .await?;

    Ok(response)
}
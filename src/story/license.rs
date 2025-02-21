use super::{transaction::StoryConfig, STORY_API_URL};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLicenseTokenResponse {
    pub data: TokenData,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenData {
    pub id: String,
    pub licensor_ip_id: String,
    pub license_template: String,
    pub license_terms_id: String,
    pub transferable: String,
    pub owner: String,
    pub burnt_at: String,
    pub block_number: String,
    pub block_time: String,
}

/// Retrieve a LicenseToken
///
/// # Arguments
///
/// * `config` - API Config
/// * `license_token_id` - License Token ID
///
/// # Returns
///
/// GetLicenseTokenResponse
pub async fn get_license_token(
    config: &StoryConfig,
    license_token_id: &str,
) -> Result<GetLicenseTokenResponse> {
    let url = format!("{}/licenses/tokens/{}", STORY_API_URL, license_token_id);

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("X-Api-Key", &config.api_key)
        .header("X-Chain", config.chain.clone())
        .header("accept", "application/json")
        .send()
        .await?
        .json::<GetLicenseTokenResponse>()
        .await?;

    Ok(response)
}
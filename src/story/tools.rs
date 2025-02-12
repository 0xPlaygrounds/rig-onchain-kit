use anyhow::Result;
use rig_tool_macro::tool;
use super::{license::{get_license_token, GetLicenseTokenResponse}, transaction::{get_a_transaction, GetTransactionResponse, StoryConfig}};

#[tool(description = "Retrieve a Transaction on Story protocol")]
pub async fn get_a_transaction_on_story(
    config: StoryConfig,
    trx_id: String,
) -> Result<GetTransactionResponse> {
    get_a_transaction(&config, &trx_id).await
}

#[tool(description = "Retrieve a LicenseToken on Story protocol")]
pub async fn get_license_token_on_story(
    config: StoryConfig,
    license_token_id: String,
) -> Result<GetLicenseTokenResponse> {
    get_license_token(&config, &license_token_id).await
}

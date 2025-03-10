#[cfg(feature = "solana")]
use {
    anyhow::Result, rig::completion::Message,
    rig_onchain_kit::reasoning_loop::ReasoningLoop,
    rig_onchain_kit::signer::solana::LocalSolanaSigner,
    rig_onchain_kit::signer::SignerContext,
    rig_onchain_kit::solana::agent::create_solana_agent,
    rig_onchain_kit::solana::util::env, std::sync::Arc,
};

#[cfg(feature = "solana")]
#[tokio::main]
async fn main() -> Result<()> {
    let signer = LocalSolanaSigner::new(env("SOLANA_PRIVATE_KEY"));

    SignerContext::with_signer(Arc::new(signer), async {
        let trader_agent = Arc::new(create_solana_agent().await?);
        let wrapped_agent = ReasoningLoop::new(trader_agent).with_stdout(true);

        wrapped_agent
            .stream(vec![Message {
                role: "user".to_string(),
                content: "what is the liquidity in the pool for my largest holding?"
                    .to_string(),
            }], None)
            .await?;

        Ok(())
    })
    .await?;

    Ok(())
}

#[cfg(not(feature = "solana"))]
fn main() {
    println!("enable the 'solana' feature to run this example.");
}

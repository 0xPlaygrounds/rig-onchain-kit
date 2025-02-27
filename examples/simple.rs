#[cfg(feature = "solana")]
use {
    anyhow::Result,
    rig_onchain_kit::signer::solana::LocalSolanaSigner,
    rig_onchain_kit::signer::SignerContext,
    rig_onchain_kit::solana::util::env,
    rig::streaming::{stream_to_stdout, StreamingPrompt},
    std::sync::Arc,
};

#[cfg(feature = "solana")]
#[tokio::main]
async fn main() -> Result<()> {
    use rig_onchain_kit::solana::tools::GetPortfolio;

    let signer = LocalSolanaSigner::new(env("SOLANA_PRIVATE_KEY"));
    SignerContext::with_signer(Arc::new(signer), async {
        let agent = rig::providers::anthropic::Client::from_env()
            .agent(rig::providers::anthropic::CLAUDE_3_5_SONNET)
            .preamble("you are a portfolio checker, if you do wanna call a tool, outline the reasoning why that tool")
            .max_tokens(1024)
            .tool(GetPortfolio)
            .build();

        let mut stream = agent
            .stream_prompt("whats the portfolio looking like?")
            .await?; 

        stream_to_stdout(agent, &mut stream).await?;

        Ok(())

    }).await?;

    Ok(())
}

#[cfg(not(feature = "solana"))]
fn main() {
    println!("enable the solana feature to run this example");
}

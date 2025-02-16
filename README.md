# Rig Onchain Kit

A [rig](https://github.com/0xPlaygrounds/rig) companion crate for building AI-powered applications that interact natively
with blockchain networks built in partnership with [listen](https://github.com/piotrostr/listen). Combines LLM capabilities with secure blockchain
operations to create intelligent agents capable of executing complex on-chain
interactions across Solana and EVM networks.

## Features

- **Dual-chain Support** - Native support for Solana and EVM ecosystems
- **Secure by Design** - Thread-local signer isolation and Privy-based
  authentication
- **Real-time Streaming** - SSE-enabled HTTP service for concurrent user
  sessions
- **Extensible Tool System** - Combine prebuilt DeFi operations with custom
  logic
- **Wallet Agnostic** - Supports local key management and Privy-embedded
  wallets

## Quick Start

```bash
# Add to your project
cargo add rig-onchain-kit --features full

# For custom tools
cargo add rig-tool-macro
```

Basic usage:

```rust
use rig_onchain_kit::agent::create_solana_agent;
use rig_onchain_kit::signer::SignerContext;
use rig_onchain_kit::signer::solana::LocalSolanaSigner;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let private_key = std::env::var("SOLANA_PRIVATE_KEY")?;
    let signer = LocalSolanaSigner::new(private_key);

    SignerContext::with_signer(Arc::new(signer), async {
        let agent = create_solana_agent();
        let response = agent.prompt("what is my public key?")?);
        println!("{}", response);
    });

    Ok(())
}
```

## Documentation

Read the [full documentation](https://0xplaygrounds.github.io/rig-onchain-kit/) to learn more

## License

[MIT](https://github.com/0xPlaygrounds/rig-onchain-kit/blob/main/LICENSE)

---

build with <3 by [listen](https://github.com/piotrostr/listen)

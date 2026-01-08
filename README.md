# Rakk

<div align="center">
  <img src="./assets/rakkg.png" alt="Rakk" width="100%">
  <br/>
  <br/>
  <a href="https://x.com/rakk_research">
  <img src="https://img.shields.io/badge/X-rakk__research-000000?style=for-the-badge&logo=x&logoColor=white" alt="X (Twitter)">
</a>
  <a href="https://github.com/rakk-res">
    <img src="https://img.shields.io/badge/Maintained%20by-Rakk%20Res-000000?style=for-the-badge&logo=github&logoColor=white" alt="Maintainer">
  </a>
  <img src="https://img.shields.io/badge/Rust-1.78%2B-orange?style=for-the-badge&logo=rust" alt="Rust Version">
  <img src="https://img.shields.io/badge/License-MIT-blue?style=for-the-badge" alt="License">
  
  <br/>
  <br/>
  <p><strong>High-density Execution Infrastructure for Algorithmic Agents.</strong></p>
  <p>Rakk provides a modular, rack-mounted middleware architecture where trading strategies are inserted as "Blades" and execution venues are managed via "Slots".</p>
</div>

---

## Architecture

Rakk mimics a physical server rack. The **Frame** manages power and data flow, routing signals from **Blades** (Intents) to the appropriate **Slots** (Adapters).

```mermaid
graph TD
    classDef client fill:#eceff1,stroke:#455a64,stroke-width:2px;
    classDef core fill:#e3f2fd,stroke:#1565c0,stroke-width:2px;
    classDef slot fill:#f3e5f5,stroke:#4a148c,stroke-width:2px;
    classDef net fill:#fff3e0,stroke:#e65100,stroke-width:2px;

    Agent[Trading Algo]:::client
    CLI[Ops Console]:::client

    subgraph "Rakk Frame (Core)"
        Bus[Data Bus]:::core
        Cooling[Cooling / Risk Control]:::core
        Router[Slot Router]:::core
    end

    subgraph "Expansion Slots (Adapters)"
        Slot1[Slot 1: Solana]:::slot
        Slot2[Slot 2: Polymarket]:::slot
        Slot3[Slot 3: Kalshi]:::slot
    end

    subgraph "Network Layer"
        SolNet((Solana Mainnet)):::net
        PolyNet((Polygon PoS)):::net
        KalNet((Kalshi Exchange)):::net
    end

    %% Safe Quotes for Labels
    Agent -->|"1. Mount Blade (Intent)"| Bus
    CLI -->|"1. Admin Command"| Bus
    
    Bus -->|"2. Process Signal"| Cooling
    Cooling -->|"3. Signal Clean"| Router
    
    Router -->|"4. Dispatch"| Slot1
    Router -->|"4. Dispatch"| Slot2
    Router -->|"4. Dispatch"| Slot3
    
    Slot1 -->|"5. RPC / Jito"| SolNet
    Slot2 -->|"5. CTF Order"| PolyNet
    Slot3 -->|"5. FIX API"| KalNet
```

## Core Components

### 1. Frame (The Chassis)
The central nervous system that provides power (liquidity access) and cooling (risk management) to all connected components.
* **Bus**: High-speed, lock-free messaging channel using `crossbeam`.
* **Cooling**: Throttles execution if temperature (volatility/slippage) rises above critical levels.

### 2. Blades (The Logic)
Standardized containers for trade intents. Agents simply "slide" a Blade into the Frame to initiate execution. A Blade contains asset, side, size, and strategy parameters.

### 3. Slots (The Interface)
Hot-swappable modules for external connectivity using generic Traits.
* **Solana Slot**: Handles transaction signing and Jito bundle formation.
* **Polymarket Slot**: Abstracts CTF mechanics and CLOB interaction.

## Installation

```bash
# Clone the repository
git clone https://github.com/rakk-res/rakk.git
cd rakk

# Mount the frame (Build)
cargo build --release
```

## Configuration

Configuration is managed via `Rakk.toml` or environment variables.

```toml
[frame]
id = "ap-northeast-rack-01"
max_blades = 64

[cooling]
max_slippage_bps = 50
emergency_stop = false

[slots.solana]
rpc_url = "https://api.mainnet-beta.solana.com"
```

## Usage Example

### Mounting a Blade (SDK)

```rust
use rakk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Initialize the Frame
    let frame = Frame::boot(Config::from_env()?);

    // 2. Define a Blade (Trade Intent)
    let blade = Blade::new()
        .target(Slot::Solana)
        .asset("SOL/USDC")
        .size(10.0)
        .side(Side::Buy);

    // 3. Mount into Frame
    let tx_hash = frame.mount(blade).await?;
    
    println!("Blade Executed. Hash: {}", tx_hash);
    Ok(())
}
```


## License

Distributed under the MIT License. See [LICENSE](./LICENSE) for more information.

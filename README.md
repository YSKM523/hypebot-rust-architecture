<p align="center">
  <img src="./assets/banner.png" alt="hypebot-rs banner" width="100%" />
</p>

<p align="center">
  <img src="https://img.shields.io/badge/source-private-7c2d12?style=for-the-badge" alt="Private source badge" />
  <img src="https://img.shields.io/badge/status-active%20build-14532d?style=for-the-badge" alt="Status badge" />
  <img src="https://img.shields.io/badge/lang-rust-e43717?style=for-the-badge" alt="Rust badge" />
  <img src="https://img.shields.io/badge/runtime-tokio-0ea5e9?style=for-the-badge" alt="Tokio badge" />
  <img src="https://img.shields.io/badge/exchange-hyperliquid-1d4ed8?style=for-the-badge" alt="Hyperliquid badge" />
</p>

<p align="center">
  <a href="https://github.com/YSKM523/hypebot-rust-architecture/issues/new">Request walkthrough</a> ·
  <a href="./CHANGELOG.md">Changelog</a> ·
  <a href="./src/lib.rs">Public Rust crate</a> ·
  <a href="https://github.com/YSKM523">@YSKM523</a>
</p>

# hypebot-rust-architecture | Rust Hyperliquid Trading Bot

A public architecture repo for a private Rust-based Hyperliquid trading system built around typed pipelines, per-symbol runners, serialized order execution, persistent state, and long-running websocket resilience.

## At A Glance

| Item | Value |
|------|-------|
| Language | Rust |
| Async runtime | Tokio |
| Exchange target | Hyperliquid |
| Repo role | Public architecture repo, private implementation |
| Core pitch | Reliability-first trading infrastructure, not a toy bot script |

> The implementation repository is private. This showcase exists to share what is being built, the engineering direction, and progress — without exposing the full source.

## Why Rust Matters Here

Rust is part of the product quality, not just the stack label:

- **Long-running reliability**: better fit for a service that stays alive through reconnects, stale feeds, order events, and runtime recovery
- **Typed system boundaries**: commands, events, and state transitions are easier to reason about when the architecture is strongly typed
- **Cleaner async structure**: `Tokio`, channels, and per-symbol runners map naturally onto trading-system concurrency
- **Execution discipline**: ownership and explicit state handling help keep order flow and recovery logic coherent under stress
- **Infrastructure credibility**: it reads like systems engineering, not like a disposable trading script

## Why This Exists

Most trading bot repos show signal logic first and treat everything else as an afterthought — brittle runtimes, weak execution discipline, poor recoverability. `hypebot-rs` is built from the opposite direction: **reliability first, then strategy.**

The focus is on the parts that separate a toy bot from a serious one:

- **Websocket resilience** — lifecycle management that survives disconnects, stale feeds, and reconnect storms
- **Serialized execution** — single-path order flow to eliminate exchange-side race conditions and nonce collisions
- **Per-symbol isolation** — independent task groups so one market never contaminates another
- **Persistent state** — local bot state restored across restarts so strategy context isn't lost
- **Safe iteration** — dry-run mode and Discord notifications for runtime observability

## Public Repo Surface

This public repo is intentionally small and architecture-led:

- [src/lib.rs](./src/lib.rs): a lightweight public Rust crate that shows the style of the system
- [assets/architecture.png](./assets/architecture.png): visual overview of transport, processing, execution, and infra layers
- [CHANGELOG.md](./CHANGELOG.md): public-facing progress log
- [README.md](./README.md): product positioning, Rust rationale, and architectural framing

Public illustrative Rust example:

- [Cargo.toml](./Cargo.toml)
- [src/lib.rs](./src/lib.rs)

```rust
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum MarketEvent {
    CandleClosed { symbol: String, close: f64 },
}

#[derive(Debug)]
pub enum OrderCommand {
    EnterLong { symbol: String, price: f64 },
}

pub struct SymbolRunner {
    symbol: String,
}

impl SymbolRunner {
    pub async fn run(self, mut market_rx: mpsc::Receiver<MarketEvent>, order_tx: mpsc::Sender<OrderCommand>) {
        while let Some(event) = market_rx.recv().await {
            match event {
                MarketEvent::CandleClosed { close, .. } if close > 0.0 => {
                    let _ = order_tx
                        .send(OrderCommand::EnterLong {
                            symbol: self.symbol.clone(),
                            price: close,
                        })
                        .await;
                }
                _ => {}
            }
        }
    }
}
```

The private implementation is more complete than this snippet, but the architecture style is the same: typed flows, async runners, and explicit execution boundaries.

## Architecture

![hypebot-rs architecture overview](assets/architecture.png)

The system is split into four distinct layers:

| Layer | Components | Responsibility |
|-------|-----------|----------------|
| **Transport** | `HlWsClient`, `HlRestClient` | Websocket subscriptions, heartbeat, reconnect flow, HTTP order interface |
| **Processing** | `MarketFeed`, `SymbolRunner`, `Strategy` | Typed event pipeline, per-symbol lifecycle, signal generation |
| **Execution** | `OrderExecutor`, `PositionTracker` | Serialized exchange calls, order state tracking |
| **Infrastructure** | `State`, `Watchdog`, `DiscordNotifier` | Persistent context, runtime health monitoring, Discord alerts + dry-run |

## Strategy

Current strategy centers on a **breakout-retest approach** with layered filters:

1. Structure break detection
2. Retest confirmation windows
3. ADX trend strength gating
4. ATR-based buffers and stop logic
5. Bollinger Band width filtering
6. Volume ratio checks
7. Time filters and cooldown handling

Not "buy when X crosses Y" — this encodes market structure, volatility context, and execution discipline into the strategy layer.

## Roadmap

**Runtime** — improve connection health visibility, startup/recovery reporting, edge-case handling around disconnects and state restoration

**Strategy** — refine breakout-retest across volatility regimes, add additional strategy modules, improve parameter documentation

**Execution** — deepen reporting around order states (resting, filled, canceled, failed), improve stop placement and recovery after abnormal responses

## Links

- Showcase: [YSKM523/hypebot-rust-architecture](https://github.com/YSKM523/hypebot-rust-architecture)
- Private source: `YSKM523/hypebot-rs`
- Changelog: [CHANGELOG.md](./CHANGELOG.md)
- Contact: [open an issue](https://github.com/YSKM523/hypebot-rust-architecture/issues/new)

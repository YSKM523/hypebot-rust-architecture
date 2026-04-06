//! Public illustrative Rust sample for the `hypebot-rs` showcase.
//!
//! This is not the private production source code.
//! It exists to show the architectural style of the system:
//! typed events, async runners, serialized execution, and explicit boundaries.

use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum MarketEvent {
    CandleClosed {
        symbol: String,
        close: f64,
        volume: f64,
    },
    FeedDisconnected {
        reason: String,
    },
}

#[derive(Debug)]
pub enum Signal {
    EnterLong { symbol: String, price: f64 },
    Wait,
}

#[derive(Debug)]
pub enum OrderCommand {
    EnterLong { symbol: String, price: f64 },
    ReduceRisk { symbol: String },
}

pub struct BreakoutStrategy;

impl BreakoutStrategy {
    pub fn on_event(&self, event: &MarketEvent) -> Signal {
        match event {
            MarketEvent::CandleClosed {
                symbol,
                close,
                volume,
            } if *close > 0.0 && *volume > 0.0 => Signal::EnterLong {
                symbol: symbol.clone(),
                price: *close,
            },
            _ => Signal::Wait,
        }
    }
}

pub struct SymbolRunner {
    symbol: String,
    strategy: BreakoutStrategy,
}

impl SymbolRunner {
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            strategy: BreakoutStrategy,
        }
    }

    pub async fn run(
        self,
        mut market_rx: mpsc::Receiver<MarketEvent>,
        order_tx: mpsc::Sender<OrderCommand>,
    ) {
        while let Some(event) = market_rx.recv().await {
            match self.strategy.on_event(&event) {
                Signal::EnterLong { symbol, price } => {
                    let _ = order_tx
                        .send(OrderCommand::EnterLong { symbol, price })
                        .await;
                }
                Signal::Wait => {}
            }
        }
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }
}

pub struct OrderExecutor;

impl OrderExecutor {
    pub async fn run(mut rx: mpsc::Receiver<OrderCommand>) {
        while let Some(command) = rx.recv().await {
            match command {
                OrderCommand::EnterLong { symbol, price } => {
                    println!("serialized execution: enter long on {symbol} at {price}");
                }
                OrderCommand::ReduceRisk { symbol } => {
                    println!("serialized execution: reduce risk on {symbol}");
                }
            }
        }
    }
}

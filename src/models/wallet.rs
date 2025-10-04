use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Chain {
    Solana,
    Ethereum,
    Base,
    BSC,
}

impl std::fmt::Display for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Chain::Solana => write!(f, "sol"),
            Chain::Ethereum => write!(f, "eth"),
            Chain::Base => write!(f, "base"),
            Chain::BSC => write!(f, "bsc"),
        }
    }
}

impl std::str::FromStr for Chain {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sol" | "solana" => Ok(Chain::Solana),
            "eth" | "ethereum" => Ok(Chain::Ethereum),
            "base" => Ok(Chain::Base),
            "bsc" | "bnb" => Ok(Chain::BSC),
            _ => Err(format!("Unknown chain: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub address: String,
    pub chain: Chain,
    pub tags: Vec<String>,
    pub realized_profit: f64,
    pub unrealized_profit: f64,
    pub total_profit: f64,
    pub win_rate: f64,
    pub trade_count: u32,
    pub last_active: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletDetail {
    pub address: String,
    pub chain: Chain,
    pub pnl_7d: PnlData,
    pub pnl_30d: PnlData,
    pub holdings: Vec<Holding>,
    pub recent_trades: Vec<Trade>,
    pub stats: WalletStats,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PnlData {
    pub realized_profit: f64,
    pub unrealized_profit: f64,
    pub total_profit: f64,
    pub roi: f64,  // Return on Investment (%)
    pub win_rate: f64,  // 胜率 (%)
    pub max_drawdown: f64,  // 最大回撤 (%)
    pub trade_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Holding {
    pub token_address: String,
    pub token_symbol: String,
    pub amount: f64,
    pub value_usd: f64,
    pub cost_basis_usd: f64,
    pub unrealized_pnl: f64,
    pub weight: f64,  // 占总仓位的百分比
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub signature: String,
    pub timestamp: DateTime<Utc>,
    pub action: TradeAction,
    pub token_address: String,
    pub token_symbol: String,
    pub amount: f64,
    pub price_usd: f64,
    pub value_usd: f64,
    pub pnl: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeAction {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletStats {
    pub total_trades: u32,
    pub winning_trades: u32,
    pub losing_trades: u32,
    pub avg_hold_time_hours: f64,
    pub largest_win: f64,
    pub largest_loss: f64,
    pub sharpe_ratio: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredWallet {
    pub wallet: WalletDetail,
    pub score: f64,  // 综合评分 0-100
    pub score_breakdown: ScoreBreakdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreBreakdown {
    pub roi_score: f64,
    pub win_rate_score: f64,
    pub consistency_score: f64,
    pub diversification_score: f64,
    pub activity_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Period {
    Day1,
    Day7,
    Day30,
    Day90,
}

impl std::fmt::Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Period::Day1 => write!(f, "1d"),
            Period::Day7 => write!(f, "7d"),
            Period::Day30 => write!(f, "30d"),
            Period::Day90 => write!(f, "90d"),
        }
    }
}

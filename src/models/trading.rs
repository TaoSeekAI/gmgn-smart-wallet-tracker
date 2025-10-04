use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingSignal {
    pub wallet_address: String,
    pub action: SignalAction,
    pub token_address: String,
    pub token_symbol: String,
    pub confidence: f64,  // 置信度 0-1
    pub reason: String,
    pub timestamp: DateTime<Utc>,
    pub security_passed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalAction {
    Buy,
    Sell,
    Hold,
}

impl std::fmt::Display for SignalAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignalAction::Buy => write!(f, "买入"),
            SignalAction::Sell => write!(f, "卖出"),
            SignalAction::Hold => write!(f, "持有"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub signal: TradingSignal,
    pub risk_assessment: String,
    pub suggested_position_size: f64,  // 建议仓位大小 (%)
    pub stop_loss: Option<f64>,
    pub take_profit: Option<f64>,
}

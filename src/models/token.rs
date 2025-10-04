use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub address: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub total_supply: f64,
    pub price_usd: f64,
    pub market_cap_usd: f64,
    pub liquidity_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCheck {
    pub token_address: String,
    pub is_safe: bool,
    pub risk_score: u8,  // 0-100, 越低越安全
    pub lp_burned: bool,
    pub is_honeypot: bool,
    pub can_mint: bool,
    pub ownership_renounced: bool,
    pub liquidity_usd: f64,
    pub holder_count: u32,
    pub top_10_holder_percent: f64,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskReport {
    pub token_address: String,
    pub security_check: SecurityCheck,
    pub risk_level: RiskLevel,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiskLevel {
    Safe,
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiskLevel::Safe => write!(f, "安全"),
            RiskLevel::Low => write!(f, "低风险"),
            RiskLevel::Medium => write!(f, "中风险"),
            RiskLevel::High => write!(f, "高风险"),
            RiskLevel::Critical => write!(f, "极高风险"),
        }
    }
}

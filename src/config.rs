use serde::{Deserialize, Serialize};
use crate::error::{Result, TrackerError};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub gmgn: GmgnConfig,
    pub filters: FilterConfig,
    pub risk: RiskConfig,
    pub cache: CacheConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GmgnConfig {
    #[serde(default = "default_base_url")]
    pub base_url: String,
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
    #[serde(default = "default_rate_limit")]
    pub rate_limit_per_minute: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterConfig {
    #[serde(default = "default_min_roi_7d")]
    pub min_roi_7d: f64,
    #[serde(default = "default_min_roi_30d")]
    pub min_roi_30d: f64,
    #[serde(default = "default_min_win_rate")]
    pub min_win_rate: f64,
    #[serde(default = "default_max_drawdown")]
    pub max_drawdown: f64,
    #[serde(default = "default_min_trade_count")]
    pub min_trade_count: u32,
    #[serde(default = "default_require_diversification")]
    pub require_diversification: bool,
    #[serde(default = "default_min_diversification_score")]
    pub min_diversification_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskConfig {
    #[serde(default = "default_require_lp_burned")]
    pub require_lp_burned: bool,
    #[serde(default = "default_block_honeypot")]
    pub block_honeypot: bool,
    #[serde(default = "default_block_mintable")]
    pub block_mintable: bool,
    #[serde(default = "default_min_liquidity")]
    pub min_liquidity_usd: f64,
    #[serde(default = "default_max_risk_score")]
    pub max_risk_score: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    #[serde(default = "default_ttl")]
    pub ttl_seconds: u64,
    #[serde(default = "default_max_size")]
    pub max_size_mb: u64,
    #[serde(default = "default_cache_dir")]
    pub cache_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    #[serde(default = "default_log_level")]
    pub level: String,
    #[serde(default = "default_log_file")]
    pub file: Option<PathBuf>,
}

// Default values
fn default_base_url() -> String {
    "https://gmgn.ai".to_string()
}

fn default_timeout() -> u64 {
    30
}

fn default_rate_limit() -> u32 {
    60
}

fn default_min_roi_7d() -> f64 {
    50.0
}

fn default_min_roi_30d() -> f64 {
    100.0
}

fn default_min_win_rate() -> f64 {
    60.0
}

fn default_max_drawdown() -> f64 {
    30.0
}

fn default_min_trade_count() -> u32 {
    10
}

fn default_require_diversification() -> bool {
    true
}

fn default_min_diversification_score() -> f64 {
    0.6
}

fn default_require_lp_burned() -> bool {
    true
}

fn default_block_honeypot() -> bool {
    true
}

fn default_block_mintable() -> bool {
    true
}

fn default_min_liquidity() -> f64 {
    10000.0
}

fn default_max_risk_score() -> u8 {
    50
}

fn default_ttl() -> u64 {
    300
}

fn default_max_size() -> u64 {
    100
}

fn default_cache_dir() -> PathBuf {
    PathBuf::from(".cache")
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_log_file() -> Option<PathBuf> {
    Some(PathBuf::from("gmgn-tracker.log"))
}

impl Default for Config {
    fn default() -> Self {
        Self {
            gmgn: GmgnConfig {
                base_url: default_base_url(),
                timeout_seconds: default_timeout(),
                rate_limit_per_minute: default_rate_limit(),
            },
            filters: FilterConfig {
                min_roi_7d: default_min_roi_7d(),
                min_roi_30d: default_min_roi_30d(),
                min_win_rate: default_min_win_rate(),
                max_drawdown: default_max_drawdown(),
                min_trade_count: default_min_trade_count(),
                require_diversification: default_require_diversification(),
                min_diversification_score: default_min_diversification_score(),
            },
            risk: RiskConfig {
                require_lp_burned: default_require_lp_burned(),
                block_honeypot: default_block_honeypot(),
                block_mintable: default_block_mintable(),
                min_liquidity_usd: default_min_liquidity(),
                max_risk_score: default_max_risk_score(),
            },
            cache: CacheConfig {
                ttl_seconds: default_ttl(),
                max_size_mb: default_max_size(),
                cache_dir: default_cache_dir(),
            },
            logging: LoggingConfig {
                level: default_log_level(),
                file: default_log_file(),
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        // 尝试从多个位置加载配置文件
        let config_paths = vec![
            "config.yaml",
            "config.yml",
            "gmgn-tracker.yaml",
            ".gmgn-tracker.yaml",
        ];

        for path in config_paths {
            if std::path::Path::new(path).exists() {
                return Self::load_from_file(path);
            }
        }

        // 如果没有找到配置文件，使用默认配置
        Ok(Self::default())
    }

    pub fn load_from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| TrackerError::ConfigError(format!("Failed to read config file: {}", e)))?;

        serde_yaml::from_str(&content)
            .map_err(|e| TrackerError::ConfigError(format!("Failed to parse config: {}", e)))
    }

    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let content = serde_yaml::to_string(self)
            .map_err(|e| TrackerError::ConfigError(format!("Failed to serialize config: {}", e)))?;

        std::fs::write(path, content)
            .map_err(|e| TrackerError::ConfigError(format!("Failed to write config file: {}", e)))?;

        Ok(())
    }
}

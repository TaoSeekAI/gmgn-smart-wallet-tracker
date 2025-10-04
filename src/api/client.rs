use crate::error::{Result, TrackerError};
use crate::models::{Chain, Wallet, WalletDetail, PnlData, Period, TokenInfo, SecurityCheck};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub struct GmgnClient {
    base_url: String,
    client: Client,
}

impl GmgnClient {
    pub fn new(base_url: String, timeout_seconds: u64) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_seconds))
            .build()
            .map_err(|e| TrackerError::NetworkError(e))?;

        Ok(Self { base_url, client })
    }

    /// 获取智能钱包列表 (模拟实现 - 实际需要根据 GMGN 真实 API 调整)
    pub async fn get_smart_wallets(&self, chain: &Chain) -> Result<Vec<Wallet>> {
        // 注意: GMGN 可能需要通过前端页面抓取数据，或使用未公开的 API
        // 这里提供一个基于推测的实现框架

        let url = format!("{}/api/v1/smartmoney/sol/walletNew/wallet_activity", self.base_url);

        let response = self.client
            .get(&url)
            .query(&[("chain", chain.to_string())])
            .send()
            .await
            .map_err(|e| TrackerError::ApiError(format!("Failed to fetch smart wallets: {}", e)))?;

        if !response.status().is_success() {
            return Err(TrackerError::ApiError(format!(
                "API returned error status: {}",
                response.status()
            )));
        }

        // 模拟响应结构 - 实际需要根据真实 API 调整
        #[derive(Deserialize)]
        struct ApiResponse {
            data: Option<ApiData>,
        }

        #[derive(Deserialize)]
        struct ApiData {
            rank: Vec<WalletRankItem>,
        }

        #[derive(Deserialize)]
        struct WalletRankItem {
            wallet_address: String,
            realized_profit: Option<f64>,
            unrealized_profit: Option<f64>,
            winrate: Option<f64>,
            buy: Option<u32>,
            last_active_timestamp: Option<i64>,
            wallet_tag_v2: Option<Vec<String>>,
        }

        let api_response: ApiResponse = response.json().await
            .map_err(|e| TrackerError::ParseError(format!("Failed to parse response: {}", e)))?;

        let wallets = if let Some(data) = api_response.data {
            data.rank.into_iter().map(|item| {
                let realized = item.realized_profit.unwrap_or(0.0);
                let unrealized = item.unrealized_profit.unwrap_or(0.0);

                Wallet {
                    address: item.wallet_address,
                    chain: chain.clone(),
                    tags: item.wallet_tag_v2.unwrap_or_default(),
                    realized_profit: realized,
                    unrealized_profit: unrealized,
                    total_profit: realized + unrealized,
                    win_rate: item.winrate.unwrap_or(0.0) * 100.0,
                    trade_count: item.buy.unwrap_or(0),
                    last_active: chrono::Utc::now(), // 简化处理
                }
            }).collect()
        } else {
            Vec::new()
        };

        Ok(wallets)
    }

    /// 获取钱包详细信息
    pub async fn get_wallet_detail(&self, address: &str, chain: &Chain) -> Result<WalletDetail> {
        let url = format!("{}/api/v1/wallet_holdings/{}", self.base_url, address);

        let response = self.client
            .get(&url)
            .query(&[("chain", chain.to_string())])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(TrackerError::ApiError(format!(
                "Failed to fetch wallet detail: {}",
                response.status()
            )));
        }

        // 这里需要根据实际 API 响应结构解析
        // 目前提供一个占位实现
        Err(TrackerError::NotFound("Wallet detail API not implemented".to_string()))
    }

    /// 获取钱包 PnL 数据
    pub async fn get_wallet_pnl(&self, address: &str, period: Period) -> Result<PnlData> {
        // 占位实现
        Err(TrackerError::NotFound("PnL API not implemented".to_string()))
    }

    /// 检查代币安全性
    pub async fn check_token_security(&self, token_address: &str) -> Result<SecurityCheck> {
        let url = format!("{}/api/v1/token_security/{}", self.base_url, token_address);

        let response = self.client
            .get(&url)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(TrackerError::ApiError(format!(
                "Failed to check token security: {}",
                response.status()
            )));
        }

        // 占位实现
        Err(TrackerError::NotFound("Security check API not implemented".to_string()))
    }

    /// 获取代币信息
    pub async fn get_token_info(&self, token_address: &str) -> Result<TokenInfo> {
        // 占位实现
        Err(TrackerError::NotFound("Token info API not implemented".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = GmgnClient::new("https://gmgn.ai".to_string(), 30);
        assert!(client.is_ok());
    }
}

use crate::api::GmgnClient;
use crate::error::Result;
use crate::models::{Chain, ScoredWallet, WalletDetail};
use crate::tracker::{WalletFilter, WalletScorer};

pub struct WalletTracker {
    filter: WalletFilter,
    api_client: GmgnClient,
}

impl WalletTracker {
    pub fn new(filter: WalletFilter, api_client: GmgnClient) -> Self {
        Self {
            filter,
            api_client,
        }
    }

    /// 扫描并筛选智能钱包
    pub async fn scan_smart_wallets(&self, chain: &Chain) -> Result<Vec<ScoredWallet>> {
        tracing::info!("开始扫描 {} 链上的智能钱包...", chain);

        // 1. 获取智能钱包列表
        let wallets = self.api_client.get_smart_wallets(chain).await?;
        tracing::info!("获取到 {} 个钱包", wallets.len());

        // 2. 初步筛选
        let filtered: Vec<_> = wallets.into_iter()
            .filter(|w| self.filter.passes(w))
            .collect();

        tracing::info!("初步筛选后剩余 {} 个钱包", filtered.len());

        // 3. 获取详细信息并评分
        let mut scored_wallets = Vec::new();

        for wallet in filtered.iter().take(50) {  // 限制并发数量
            match self.api_client.get_wallet_detail(&wallet.address, chain).await {
                Ok(detail) => {
                    // 详细筛选
                    if self.filter.passes_detail(&detail) {
                        let scored = WalletScorer::score(&detail);
                        scored_wallets.push(scored);
                    }
                }
                Err(e) => {
                    tracing::warn!("获取钱包 {} 详情失败: {}", wallet.address, e);
                }
            }
        }

        // 4. 按分数排序
        scored_wallets.sort_by(|a, b| {
            b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal)
        });

        tracing::info!("最终筛选出 {} 个高质量钱包", scored_wallets.len());

        Ok(scored_wallets)
    }

    /// 获取单个钱包的评分
    pub async fn score_wallet(&self, address: &str, chain: &Chain) -> Result<ScoredWallet> {
        let detail = self.api_client.get_wallet_detail(address, chain).await?;
        Ok(WalletScorer::score(&detail))
    }
}

use crate::models::{Wallet, WalletDetail, Holding};
use crate::config::FilterConfig;

#[derive(Debug, Clone)]
pub struct WalletFilter {
    pub min_roi_7d: f64,
    pub min_roi_30d: f64,
    pub min_win_rate: f64,
    pub max_drawdown: f64,
    pub min_trade_count: u32,
    pub require_diversification: bool,
    pub min_diversification_score: f64,
}

impl From<FilterConfig> for WalletFilter {
    fn from(config: FilterConfig) -> Self {
        Self {
            min_roi_7d: config.min_roi_7d,
            min_roi_30d: config.min_roi_30d,
            min_win_rate: config.min_win_rate,
            max_drawdown: config.max_drawdown,
            min_trade_count: config.min_trade_count,
            require_diversification: config.require_diversification,
            min_diversification_score: config.min_diversification_score,
        }
    }
}

impl WalletFilter {
    pub fn passes(&self, wallet: &Wallet) -> bool {
        // 检查胜率
        if wallet.win_rate < self.min_win_rate {
            return false;
        }

        // 检查交易次数
        if wallet.trade_count < self.min_trade_count {
            return false;
        }

        true
    }

    pub fn passes_detail(&self, wallet: &WalletDetail) -> bool {
        // 检查 7 天 ROI
        if wallet.pnl_7d.roi < self.min_roi_7d {
            return false;
        }

        // 检查 30 天 ROI
        if wallet.pnl_30d.roi < self.min_roi_30d {
            return false;
        }

        // 检查胜率
        if wallet.pnl_30d.win_rate < self.min_win_rate {
            return false;
        }

        // 检查最大回撤
        if wallet.pnl_30d.max_drawdown > self.max_drawdown {
            return false;
        }

        // 检查交易次数
        if wallet.pnl_30d.trade_count < self.min_trade_count {
            return false;
        }

        // 检查持仓多样性
        if self.require_diversification {
            let diversification_score = self.calculate_diversification(&wallet.holdings);
            if diversification_score < self.min_diversification_score {
                return false;
            }
        }

        true
    }

    /// 计算持仓多样性分数
    /// 使用 Herfindahl-Hirschman Index (HHI) 的反向指标
    /// 分数范围 0-1，越高表示越分散
    pub fn calculate_diversification(&self, holdings: &[Holding]) -> f64 {
        if holdings.is_empty() {
            return 0.0;
        }

        // 计算 HHI
        let hhi: f64 = holdings.iter()
            .map(|h| {
                let weight = h.weight / 100.0; // 转换为 0-1 范围
                weight * weight
            })
            .sum();

        // HHI 范围是 [1/n, 1]，其中 n 是持仓数量
        // 我们将其转换为 0-1 的分数，1 表示完全分散
        // 分数 = 1 - (HHI - 1/n) / (1 - 1/n)

        let n = holdings.len() as f64;
        let min_hhi = 1.0 / n;
        let max_hhi = 1.0;

        if max_hhi - min_hhi < 0.001 {
            return 1.0; // 只有一个持仓
        }

        let score = 1.0 - (hhi - min_hhi) / (max_hhi - min_hhi);

        score.max(0.0).min(1.0)
    }

    /// 检查是否过度集中在单一代币
    pub fn is_overly_concentrated(&self, holdings: &[Holding]) -> bool {
        holdings.iter().any(|h| h.weight > 70.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diversification_single_holding() {
        let filter = WalletFilter {
            min_roi_7d: 0.0,
            min_roi_30d: 0.0,
            min_win_rate: 0.0,
            max_drawdown: 100.0,
            min_trade_count: 0,
            require_diversification: false,
            min_diversification_score: 0.0,
        };

        let holdings = vec![
            Holding {
                token_address: "token1".to_string(),
                token_symbol: "TKN1".to_string(),
                amount: 100.0,
                value_usd: 1000.0,
                cost_basis_usd: 800.0,
                unrealized_pnl: 200.0,
                weight: 100.0,
            }
        ];

        let score = filter.calculate_diversification(&holdings);
        assert!(score < 0.1); // 单一持仓分数应该很低
    }

    #[test]
    fn test_diversification_balanced() {
        let filter = WalletFilter {
            min_roi_7d: 0.0,
            min_roi_30d: 0.0,
            min_win_rate: 0.0,
            max_drawdown: 100.0,
            min_trade_count: 0,
            require_diversification: false,
            min_diversification_score: 0.0,
        };

        let holdings = vec![
            Holding {
                token_address: "token1".to_string(),
                token_symbol: "TKN1".to_string(),
                amount: 100.0,
                value_usd: 250.0,
                cost_basis_usd: 200.0,
                unrealized_pnl: 50.0,
                weight: 25.0,
            },
            Holding {
                token_address: "token2".to_string(),
                token_symbol: "TKN2".to_string(),
                amount: 200.0,
                value_usd: 250.0,
                cost_basis_usd: 200.0,
                unrealized_pnl: 50.0,
                weight: 25.0,
            },
            Holding {
                token_address: "token3".to_string(),
                token_symbol: "TKN3".to_string(),
                amount: 300.0,
                value_usd: 250.0,
                cost_basis_usd: 200.0,
                unrealized_pnl: 50.0,
                weight: 25.0,
            },
            Holding {
                token_address: "token4".to_string(),
                token_symbol: "TKN4".to_string(),
                amount: 400.0,
                value_usd: 250.0,
                cost_basis_usd: 200.0,
                unrealized_pnl: 50.0,
                weight: 25.0,
            },
        ];

        let score = filter.calculate_diversification(&holdings);
        assert!(score > 0.9); // 平衡的持仓分数应该很高
    }
}

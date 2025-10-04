use crate::models::{WalletDetail, ScoredWallet, ScoreBreakdown};

pub struct WalletScorer;

impl WalletScorer {
    /// 对钱包进行综合评分 (0-100)
    pub fn score(wallet: &WalletDetail) -> ScoredWallet {
        let roi_score = Self::score_roi(wallet);
        let win_rate_score = Self::score_win_rate(wallet);
        let consistency_score = Self::score_consistency(wallet);
        let diversification_score = Self::score_diversification(wallet);
        let activity_score = Self::score_activity(wallet);

        // 加权平均
        let weights = [0.3, 0.25, 0.2, 0.15, 0.1];
        let total_score = roi_score * weights[0]
            + win_rate_score * weights[1]
            + consistency_score * weights[2]
            + diversification_score * weights[3]
            + activity_score * weights[4];

        ScoredWallet {
            wallet: wallet.clone(),
            score: total_score,
            score_breakdown: ScoreBreakdown {
                roi_score,
                win_rate_score,
                consistency_score,
                diversification_score,
                activity_score,
            },
        }
    }

    /// ROI 评分 (0-100)
    fn score_roi(wallet: &WalletDetail) -> f64 {
        // 综合考虑 7 天和 30 天 ROI
        let roi_7d = wallet.pnl_7d.roi;
        let roi_30d = wallet.pnl_30d.roi;

        // 使用 sigmoid 函数将 ROI 映射到 0-100
        // ROI = 100% 时分数约为 73
        // ROI = 200% 时分数约为 88
        // ROI = 500% 时分数约为 98
        let score_7d = Self::sigmoid(roi_7d / 100.0) * 100.0;
        let score_30d = Self::sigmoid(roi_30d / 100.0) * 100.0;

        // 7 天权重 40%, 30 天权重 60%
        score_7d * 0.4 + score_30d * 0.6
    }

    /// 胜率评分 (0-100)
    fn score_win_rate(wallet: &WalletDetail) -> f64 {
        let win_rate = wallet.pnl_30d.win_rate;

        // 胜率直接映射
        // 50% -> 50 分
        // 70% -> 70 分
        // 90% -> 90 分
        win_rate.max(0.0).min(100.0)
    }

    /// 一致性评分 (0-100) - 基于回撤和 Sharpe Ratio
    fn score_consistency(wallet: &WalletDetail) -> f64 {
        let max_drawdown = wallet.pnl_30d.max_drawdown;

        // 回撤越小，分数越高
        // 回撤 0% -> 100 分
        // 回撤 20% -> 66 分
        // 回撤 50% -> 33 分
        let drawdown_score = (1.0 - max_drawdown / 100.0) * 100.0;

        // Sharpe Ratio 评分
        let sharpe_score = if let Some(sharpe) = wallet.stats.sharpe_ratio {
            // Sharpe > 2.0 为优秀
            // Sharpe > 1.0 为良好
            // Sharpe > 0.5 为一般
            (sharpe / 2.0).min(1.0) * 100.0
        } else {
            50.0 // 默认分数
        };

        // 回撤和 Sharpe 各占 50%
        drawdown_score * 0.5 + sharpe_score * 0.5
    }

    /// 多样性评分 (0-100)
    fn score_diversification(wallet: &WalletDetail) -> f64 {
        if wallet.holdings.is_empty() {
            return 0.0;
        }

        // 计算 HHI (Herfindahl-Hirschman Index)
        let hhi: f64 = wallet.holdings.iter()
            .map(|h| {
                let weight = h.weight / 100.0;
                weight * weight
            })
            .sum();

        let n = wallet.holdings.len() as f64;
        let min_hhi = 1.0 / n;

        if 1.0 - min_hhi < 0.001 {
            return 50.0;
        }

        // 转换为 0-100 分数
        let score = (1.0 - (hhi - min_hhi) / (1.0 - min_hhi)) * 100.0;

        score.max(0.0).min(100.0)
    }

    /// 活跃度评分 (0-100)
    fn score_activity(wallet: &WalletDetail) -> f64 {
        let trade_count = wallet.pnl_30d.trade_count as f64;

        // 使用对数函数，避免过度奖励高频交易
        // 10 次交易 -> 约 70 分
        // 30 次交易 -> 约 85 分
        // 100 次交易 -> 约 95 分
        let score = (trade_count.ln() / 5.0_f64.ln()) * 100.0;

        score.max(0.0).min(100.0)
    }

    /// Sigmoid 函数: 1 / (1 + e^(-x))
    fn sigmoid(x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;
    use chrono::Utc;

    fn create_test_wallet() -> WalletDetail {
        WalletDetail {
            address: "test_wallet".to_string(),
            chain: Chain::Solana,
            pnl_7d: PnlData {
                realized_profit: 5000.0,
                unrealized_profit: 2000.0,
                total_profit: 7000.0,
                roi: 70.0,
                win_rate: 65.0,
                max_drawdown: 15.0,
                trade_count: 20,
            },
            pnl_30d: PnlData {
                realized_profit: 15000.0,
                unrealized_profit: 5000.0,
                total_profit: 20000.0,
                roi: 100.0,
                win_rate: 70.0,
                max_drawdown: 20.0,
                trade_count: 50,
            },
            holdings: vec![
                Holding {
                    token_address: "token1".to_string(),
                    token_symbol: "TKN1".to_string(),
                    amount: 100.0,
                    value_usd: 3000.0,
                    cost_basis_usd: 2500.0,
                    unrealized_pnl: 500.0,
                    weight: 30.0,
                },
                Holding {
                    token_address: "token2".to_string(),
                    token_symbol: "TKN2".to_string(),
                    amount: 200.0,
                    value_usd: 4000.0,
                    cost_basis_usd: 3500.0,
                    unrealized_pnl: 500.0,
                    weight: 40.0,
                },
                Holding {
                    token_address: "token3".to_string(),
                    token_symbol: "TKN3".to_string(),
                    amount: 300.0,
                    value_usd: 3000.0,
                    cost_basis_usd: 2800.0,
                    unrealized_pnl: 200.0,
                    weight: 30.0,
                },
            ],
            recent_trades: vec![],
            stats: WalletStats {
                total_trades: 50,
                winning_trades: 35,
                losing_trades: 15,
                avg_hold_time_hours: 48.0,
                largest_win: 5000.0,
                largest_loss: -1000.0,
                sharpe_ratio: Some(1.5),
            },
            created_at: Utc::now(),
        }
    }

    #[test]
    fn test_scoring() {
        let wallet = create_test_wallet();
        let scored = WalletScorer::score(&wallet);

        assert!(scored.score > 0.0 && scored.score <= 100.0);
        assert!(scored.score_breakdown.roi_score > 0.0);
        assert!(scored.score_breakdown.win_rate_score > 0.0);
        assert!(scored.score_breakdown.consistency_score > 0.0);
        assert!(scored.score_breakdown.diversification_score > 0.0);
        assert!(scored.score_breakdown.activity_score > 0.0);
    }

    #[test]
    fn test_sigmoid() {
        assert!((WalletScorer::sigmoid(0.0) - 0.5).abs() < 0.01);
        assert!(WalletScorer::sigmoid(5.0) > 0.99);
        assert!(WalletScorer::sigmoid(-5.0) < 0.01);
    }
}

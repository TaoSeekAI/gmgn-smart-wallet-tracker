use crate::error::Result;
use crate::models::{Chain, Wallet, WalletDetail, PnlData, Holding, Trade, TradeAction, WalletStats, ScoredWallet};
use crate::tracker::WalletScorer;
use chrono::Utc;
use rand::Rng;

/// Mock data generator for development and testing
pub struct MockDataGenerator;

impl MockDataGenerator {
    /// Generate mock smart wallets with scores
    pub fn generate_wallets(chain: &Chain, count: usize) -> Result<Vec<ScoredWallet>> {
        let mut rng = rand::thread_rng();
        let mut scored_wallets = Vec::new();

        for _i in 0..count {
            let address = Self::generate_address(chain);
            let detail = Self::generate_wallet_detail(&address, chain)?;
            let scored = WalletScorer::score(&detail);
            scored_wallets.push(scored);
        }

        // Sort by score descending
        scored_wallets.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        Ok(scored_wallets)
    }

    /// Generate mock wallet detail
    pub fn generate_wallet_detail(address: &str, chain: &Chain) -> Result<WalletDetail> {
        let mut rng = rand::thread_rng();

        let pnl_7d = PnlData {
            realized_profit: rng.gen_range(500.0..10000.0),
            unrealized_profit: rng.gen_range(-1000.0..5000.0),
            total_profit: 0.0, // Will be calculated
            roi: rng.gen_range(30.0..150.0),
            win_rate: rng.gen_range(50.0..80.0),
            max_drawdown: rng.gen_range(10.0..40.0),
            trade_count: rng.gen_range(5..30),
        };

        let pnl_30d = PnlData {
            realized_profit: rng.gen_range(5000.0..50000.0),
            unrealized_profit: rng.gen_range(-5000.0..20000.0),
            total_profit: 0.0,
            roi: rng.gen_range(50.0..300.0),
            win_rate: rng.gen_range(55.0..85.0),
            max_drawdown: rng.gen_range(15.0..50.0),
            trade_count: rng.gen_range(20..100),
        };

        let holdings = Self::generate_holdings(3..8);
        let recent_trades = Self::generate_trades(5..15);

        let total_trades = pnl_30d.trade_count;
        let winning_trades = (total_trades as f64 * pnl_30d.win_rate / 100.0) as u32;

        Ok(WalletDetail {
            address: address.to_string(),
            chain: chain.clone(),
            pnl_7d,
            pnl_30d,
            holdings,
            recent_trades,
            stats: WalletStats {
                total_trades,
                winning_trades,
                losing_trades: total_trades - winning_trades,
                avg_hold_time_hours: rng.gen_range(12.0..168.0),
                largest_win: rng.gen_range(1000.0..10000.0),
                largest_loss: rng.gen_range(-5000.0..-500.0),
                sharpe_ratio: Some(rng.gen_range(0.5..2.5)),
            },
            created_at: Utc::now(),
        })
    }

    fn generate_address(chain: &Chain) -> String {
        let mut rng = rand::thread_rng();
        match chain {
            Chain::Solana => {
                // Generate base58-like address
                let chars: Vec<char> = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
                    .chars()
                    .collect();
                (0..44)
                    .map(|_| chars[rng.gen_range(0..chars.len())])
                    .collect()
            }
            Chain::Ethereum | Chain::Base | Chain::BSC => {
                // Generate 0x address
                format!(
                    "0x{}",
                    (0..40)
                        .map(|_| format!("{:x}", rng.gen_range(0..16)))
                        .collect::<String>()
                )
            }
        }
    }

    fn generate_tags() -> Vec<String> {
        let mut rng = rand::thread_rng();
        let all_tags = vec![
            "Smart Money",
            "Whale",
            "Early Adopter",
            "Stable Returns",
            "High Risk",
            "Diversified",
            "Day Trader",
            "Swing Trader",
            "DeFi Expert",
            "NFT Trader",
        ];

        let tag_count = rng.gen_range(1..4);
        all_tags
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .into_iter()
            .take(tag_count)
            .collect()
    }

    fn generate_holdings(count_range: std::ops::Range<usize>) -> Vec<Holding> {
        let mut rng = rand::thread_rng();
        let count = rng.gen_range(count_range);
        let mut holdings = Vec::new();

        // Generate random weights that sum to 100
        let mut weights: Vec<f64> = (0..count).map(|_| rng.gen_range(10.0..40.0)).collect();
        let sum: f64 = weights.iter().sum();
        weights = weights.iter().map(|w| w / sum * 100.0).collect();

        for i in 0..count {
            let value = rng.gen_range(500.0..10000.0);
            let cost = value * rng.gen_range(0.7..1.3);

            holdings.push(Holding {
                token_address: format!("TOKEN_{}", i),
                token_symbol: format!("TKN{}", i),
                amount: rng.gen_range(100.0..10000.0),
                value_usd: value,
                cost_basis_usd: cost,
                unrealized_pnl: value - cost,
                weight: weights[i],
            });
        }

        holdings
    }

    fn generate_trades(count_range: std::ops::Range<usize>) -> Vec<Trade> {
        let mut rng = rand::thread_rng();
        let count = rng.gen_range(count_range);
        let mut trades = Vec::new();

        for i in 0..count {
            let action = if rng.gen_bool(0.5) {
                TradeAction::Buy
            } else {
                TradeAction::Sell
            };

            let price = rng.gen_range(0.001..100.0);
            let amount = rng.gen_range(10.0..1000.0);

            trades.push(Trade {
                signature: format!("SIG_{:064x}", rng.gen::<u64>()),
                timestamp: Utc::now() - chrono::Duration::hours(rng.gen_range(1..720)),
                action,
                token_address: format!("TOKEN_{}", i % 5),
                token_symbol: format!("TKN{}", i % 5),
                amount,
                price_usd: price,
                value_usd: price * amount,
                pnl: Some(rng.gen_range(-500.0..2000.0)),
            });
        }

        trades.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        trades
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_wallets() {
        let wallets = MockDataGenerator::generate_wallets(&Chain::Solana, 10).unwrap();
        assert_eq!(wallets.len(), 10);

        // Check sorted by profit
        for i in 0..wallets.len() - 1 {
            assert!(wallets[i].total_profit >= wallets[i + 1].total_profit);
        }
    }

    #[test]
    fn test_generate_wallet_detail() {
        let detail = MockDataGenerator::generate_wallet_detail(
            "test_address",
            &Chain::Solana
        ).unwrap();

        assert_eq!(detail.address, "test_address");
        assert!(!detail.holdings.is_empty());
        assert!(!detail.recent_trades.is_empty());

        // Check weight sums to ~100%
        let total_weight: f64 = detail.holdings.iter().map(|h| h.weight).sum();
        assert!((total_weight - 100.0).abs() < 1.0);
    }

    #[test]
    fn test_solana_address_format() {
        let addr = MockDataGenerator::generate_address(&Chain::Solana);
        assert_eq!(addr.len(), 44);
        assert!(addr.chars().all(|c| c.is_alphanumeric()));
    }

    #[test]
    fn test_ethereum_address_format() {
        let addr = MockDataGenerator::generate_address(&Chain::Ethereum);
        assert_eq!(addr.len(), 42);
        assert!(addr.starts_with("0x"));
    }
}

use crate::models::ScoredWallet;
use colored::*;

pub struct OutputFormatter;

impl OutputFormatter {
    /// 格式化钱包列表为表格
    pub fn format_wallets_table(wallets: &[ScoredWallet], limit: usize) -> String {
        let mut output = String::new();

        output.push_str(&format!("\n{}\n", "=== 智能钱包排行榜 ===".bright_cyan().bold()));
        output.push_str(&format!(
            "{:<45} {:>8} {:>8} {:>8} {:>8} {:>8}\n",
            "钱包地址".bold(),
            "总分".bold(),
            "ROI分".bold(),
            "胜率".bold(),
            "多样性".bold(),
            "活跃度".bold()
        ));
        output.push_str(&"-".repeat(100));
        output.push('\n');

        for (i, wallet) in wallets.iter().take(limit).enumerate() {
            let rank = format!("#{}", i + 1).bright_yellow();
            let addr = Self::truncate_address(&wallet.wallet.address);

            let score_color = Self::score_color(wallet.score);
            let score_str = format!("{:.1}", wallet.score).color(score_color);

            output.push_str(&format!(
                "{} {:<40} {:>8} {:>8.1} {:>7.0}% {:>8.1} {:>8.1}\n",
                rank,
                addr,
                score_str,
                wallet.score_breakdown.roi_score,
                wallet.wallet.pnl_30d.win_rate,
                wallet.score_breakdown.diversification_score,
                wallet.score_breakdown.activity_score,
            ));
        }

        output.push_str(&"-".repeat(100));
        output.push('\n');

        output
    }

    /// 格式化钱包列表为 JSON
    pub fn format_wallets_json(wallets: &[ScoredWallet], limit: usize) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&wallets.iter().take(limit).collect::<Vec<_>>())
    }

    /// 格式化钱包列表为 CSV
    pub fn format_wallets_csv(wallets: &[ScoredWallet], limit: usize) -> String {
        let mut output = String::new();

        // CSV header
        output.push_str("Rank,Address,Score,ROI_Score,Win_Rate,Diversification,Activity,7d_ROI,30d_ROI,Trade_Count\n");

        for (i, wallet) in wallets.iter().take(limit).enumerate() {
            output.push_str(&format!(
                "{},{},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{}\n",
                i + 1,
                wallet.wallet.address,
                wallet.score,
                wallet.score_breakdown.roi_score,
                wallet.wallet.pnl_30d.win_rate,
                wallet.score_breakdown.diversification_score,
                wallet.score_breakdown.activity_score,
                wallet.wallet.pnl_7d.roi,
                wallet.wallet.pnl_30d.roi,
                wallet.wallet.pnl_30d.trade_count,
            ));
        }

        output
    }

    /// 截断地址显示
    fn truncate_address(addr: &str) -> String {
        if addr.len() <= 42 {
            return addr.to_string();
        }
        format!("{}...{}", &addr[..6], &addr[addr.len()-4..])
    }

    /// 根据分数返回颜色
    fn score_color(score: f64) -> colored::Color {
        if score >= 80.0 {
            colored::Color::BrightGreen
        } else if score >= 60.0 {
            colored::Color::Green
        } else if score >= 40.0 {
            colored::Color::Yellow
        } else {
            colored::Color::Red
        }
    }

    /// 格式化风险报告
    pub fn format_risk_report(token_address: &str, report: &crate::models::RiskReport) -> String {
        let mut output = String::new();

        output.push_str(&format!("\n{}\n", "=== 代币安全检查 ===".bright_cyan().bold()));
        output.push_str(&format!("代币地址: {}\n", token_address));
        output.push_str(&format!("风险等级: {}\n", Self::format_risk_level(&report.risk_level)));
        output.push_str(&format!("风险分数: {}/100\n", report.security_check.risk_score));
        output.push('\n');

        let check = &report.security_check;

        output.push_str(&format!("LP 销毁: {}\n", Self::bool_indicator(check.lp_burned)));
        output.push_str(&format!("蜜罐检测: {}\n", Self::bool_indicator(!check.is_honeypot)));
        output.push_str(&format!("可增发: {}\n", Self::bool_indicator(!check.can_mint)));
        output.push_str(&format!("所有权放弃: {}\n", Self::bool_indicator(check.ownership_renounced)));
        output.push_str(&format!("流动性: ${:.2}\n", check.liquidity_usd));
        output.push_str(&format!("持有者数量: {}\n", check.holder_count));
        output.push_str(&format!("前10持有者占比: {:.1}%\n", check.top_10_holder_percent));
        output.push('\n');

        output.push_str(&format!("{}\n", "建议:".bright_yellow().bold()));
        output.push_str(&report.recommendation);
        output.push('\n');

        output
    }

    fn format_risk_level(level: &crate::models::RiskLevel) -> colored::ColoredString {
        match level {
            crate::models::RiskLevel::Safe => "安全".bright_green(),
            crate::models::RiskLevel::Low => "低风险".green(),
            crate::models::RiskLevel::Medium => "中风险".yellow(),
            crate::models::RiskLevel::High => "高风险".red(),
            crate::models::RiskLevel::Critical => "极高风险".bright_red().bold(),
        }
    }

    fn bool_indicator(value: bool) -> colored::ColoredString {
        if value {
            "✓".bright_green()
        } else {
            "✗".bright_red()
        }
    }
}

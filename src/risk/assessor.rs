use crate::api::GmgnClient;
use crate::config::RiskConfig;
use crate::error::Result;
use crate::models::{RiskLevel, RiskReport, SecurityCheck};

pub struct RiskAssessor {
    api_client: GmgnClient,
    config: RiskConfig,
}

impl RiskAssessor {
    pub fn new(api_client: GmgnClient, config: RiskConfig) -> Self {
        Self { api_client, config }
    }

    /// 评估代币风险
    pub async fn assess_token(&self, token_address: &str) -> Result<RiskReport> {
        tracing::info!("开始评估代币风险: {}", token_address);

        let security_check = self.api_client.check_token_security(token_address).await?;

        let risk_score = self.calculate_risk_score(&security_check);
        let risk_level = self.determine_risk_level(risk_score);
        let recommendation = self.generate_recommendation(&security_check, &risk_level);

        Ok(RiskReport {
            token_address: token_address.to_string(),
            security_check,
            risk_level,
            recommendation,
        })
    }

    /// 计算风险分数 (0-100, 越低越安全)
    pub fn calculate_risk_score(&self, check: &SecurityCheck) -> u8 {
        let mut score = 0;

        // LP 未销毁 +20 分
        if !check.lp_burned {
            score += 20;
        }

        // 蜜罐合约 +40 分
        if check.is_honeypot {
            score += 40;
        }

        // 可增发 +25 分
        if check.can_mint {
            score += 25;
        }

        // 所有权未放弃 +10 分
        if !check.ownership_renounced {
            score += 10;
        }

        // 流动性不足 +15 分
        if check.liquidity_usd < self.config.min_liquidity_usd {
            score += 15;
        }

        // 持有者集中度高 +10 分
        if check.top_10_holder_percent > 50.0 {
            score += 10;
        } else if check.top_10_holder_percent > 30.0 {
            score += 5;
        }

        score.min(100)
    }

    /// 确定风险等级
    pub fn determine_risk_level(&self, risk_score: u8) -> RiskLevel {
        match risk_score {
            0..=10 => RiskLevel::Safe,
            11..=30 => RiskLevel::Low,
            31..=50 => RiskLevel::Medium,
            51..=70 => RiskLevel::High,
            _ => RiskLevel::Critical,
        }
    }

    /// 生成建议
    fn generate_recommendation(&self, check: &SecurityCheck, risk_level: &RiskLevel) -> String {
        match risk_level {
            RiskLevel::Safe => {
                "代币安全检查通过，可以考虑交易。".to_string()
            }
            RiskLevel::Low => {
                let mut msg = "代币风险较低，但需注意以下问题:\n".to_string();
                self.append_warnings(&mut msg, check);
                msg
            }
            RiskLevel::Medium => {
                let mut msg = "代币存在中等风险，请谨慎交易:\n".to_string();
                self.append_warnings(&mut msg, check);
                msg.push_str("\n建议: 小仓位试水，设置止损。");
                msg
            }
            RiskLevel::High => {
                let mut msg = "代币风险较高，不建议新手交易:\n".to_string();
                self.append_warnings(&mut msg, check);
                msg.push_str("\n警告: 可能损失全部本金。");
                msg
            }
            RiskLevel::Critical => {
                let mut msg = "代币风险极高，强烈不建议交易:\n".to_string();
                self.append_warnings(&mut msg, check);
                msg.push_str("\n严重警告: 极有可能是骗局或蜜罐合约!");
                msg
            }
        }
    }

    /// 添加警告信息
    fn append_warnings(&self, msg: &mut String, check: &SecurityCheck) {
        if !check.lp_burned {
            msg.push_str("  - LP 未销毁，开发者可以撤池\n");
        }
        if check.is_honeypot {
            msg.push_str("  - 检测到蜜罐特征，可能无法卖出\n");
        }
        if check.can_mint {
            msg.push_str("  - 代币可增发，有通胀风险\n");
        }
        if !check.ownership_renounced {
            msg.push_str("  - 合约所有权未放弃\n");
        }
        if check.liquidity_usd < self.config.min_liquidity_usd {
            msg.push_str(&format!("  - 流动性不足 (${:.2})\n", check.liquidity_usd));
        }
        if check.top_10_holder_percent > 50.0 {
            msg.push_str(&format!("  - 前 10 持有者占比过高 ({:.1}%)\n", check.top_10_holder_percent));
        }

        for warning in &check.warnings {
            msg.push_str(&format!("  - {}\n", warning));
        }
    }

    /// 检查代币是否通过风险筛选
    pub fn passes_risk_check(&self, check: &SecurityCheck) -> bool {
        // 检查蜜罐
        if self.config.block_honeypot && check.is_honeypot {
            return false;
        }

        // 检查可增发
        if self.config.block_mintable && check.can_mint {
            return false;
        }

        // 检查 LP 销毁
        if self.config.require_lp_burned && !check.lp_burned {
            return false;
        }

        // 检查流动性
        if check.liquidity_usd < self.config.min_liquidity_usd {
            return false;
        }

        // 检查风险分数
        if check.risk_score > self.config.max_risk_score {
            return false;
        }

        true
    }
}

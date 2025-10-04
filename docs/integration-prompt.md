# GMGN 智能钱包追踪器 - 集成 Prompt

## 概述

本文档提供了一个完整的 Prompt,用于在其他项目中复刻和集成 GMGN 智能钱包追踪功能。

---

## 完整集成 Prompt

```
你是一个经验丰富的 Rust 开发者。我需要你帮我在现有项目中集成 GMGN 智能钱包追踪功能。

### 背景信息

我有一个完整的 GMGN 智能钱包追踪器项目,项目架构如下:

1. **核心模块**
   - API 客户端: 封装 GMGN.ai 的 API 调用
   - 钱包追踪器: 实现钱包扫描和筛选
   - 评分系统: 综合评分算法(ROI、胜率、一致性、多样性、活跃度)
   - 风险评估: 代币安全检查(LP销毁、蜜罐、增发等)
   - 数据模型: 钱包、代币、交易等数据结构

2. **评分算法**
   ```rust
   总分 = ROI分×30% + 胜率分×25% + 一致性分×20% + 多样性分×15% + 活跃度分×10%
   ```

   - ROI评分: 基于7天和30天收益率的sigmoid函数映射
   - 胜率评分: 直接使用胜率百分比
   - 一致性评分: 基于最大回撤和Sharpe比率
   - 多样性评分: 使用HHI指数计算持仓分散程度
   - 活跃度评分: 基于交易次数的对数函数

3. **风险评估标准**
   ```rust
   风险分数计算:
   - LP未销毁: +20分
   - 蜜罐合约: +40分
   - 可增发: +25分
   - 所有权未放弃: +10分
   - 流动性不足: +15分
   - 持有者集中度高: +5-10分

   风险等级:
   - 0-10: 安全
   - 11-30: 低风险
   - 31-50: 中风险
   - 51-70: 高风险
   - 71+: 极高风险
   ```

4. **数据结构**
   ```rust
   // 钱包基本信息
   struct Wallet {
       address: String,
       chain: Chain,
       tags: Vec<String>,
       realized_profit: f64,
       unrealized_profit: f64,
       total_profit: f64,
       win_rate: f64,
       trade_count: u32,
   }

   // 钱包详细信息
   struct WalletDetail {
       address: String,
       chain: Chain,
       pnl_7d: PnlData,
       pnl_30d: PnlData,
       holdings: Vec<Holding>,
       recent_trades: Vec<Trade>,
       stats: WalletStats,
   }

   // 评分后的钱包
   struct ScoredWallet {
       wallet: WalletDetail,
       score: f64,
       score_breakdown: ScoreBreakdown,
   }
   ```

5. **API 端点** (基于 GMGN.ai)
   ```
   智能钱包活动: /api/v1/smartmoney/sol/walletNew/wallet_activity
   钱包持仓: /api/v1/wallet_holdings/{address}
   代币安全: /api/v1/token_security/{token_address}
   ```

6. **配置选项**
   ```yaml
   filters:
     min_roi_7d: 50.0
     min_roi_30d: 100.0
     min_win_rate: 60.0
     max_drawdown: 30.0
     min_trade_count: 10
     require_diversification: true
     min_diversification_score: 0.6

   risk:
     require_lp_burned: true
     block_honeypot: true
     block_mintable: true
     min_liquidity_usd: 10000.0
     max_risk_score: 50
   ```

### 集成任务

请帮我在当前项目中实现以下功能:

1. **基础集成** (必须)
   - [ ] 创建 GMGN API 客户端模块
   - [ ] 实现钱包数据获取功能
   - [ ] 实现评分算法
   - [ ] 实现基本的筛选功能

2. **进阶功能** (可选)
   - [ ] 添加代币安全检查
   - [ ] 实现缓存机制
   - [ ] 添加配置文件支持
   - [ ] 集成到现有 CLI/Web 界面

3. **代码要求**
   - 使用 Rust 实现
   - 遵循异步编程(tokio)
   - 良好的错误处理(thiserror + anyhow)
   - 包含单元测试
   - 代码注释清晰

### 实现步骤

请按以下步骤实现:

**Step 1: 创建基础模块结构**
```
src/
├── gmgn/
│   ├── mod.rs
│   ├── api.rs          # API 客户端
│   ├── models.rs       # 数据模型
│   ├── scorer.rs       # 评分算法
│   ├── filter.rs       # 筛选逻辑
│   └── risk.rs         # 风险评估
```

**Step 2: 实现 API 客户端**
- 封装 HTTP 请求
- 处理 API 响应
- 实现速率限制
- 添加重试机制

**Step 3: 实现核心算法**
- 钱包评分算法
- 多样性计算(HHI指数)
- 风险评估逻辑

**Step 4: 集成到现有项目**
- 添加新的命令/接口
- 更新配置文件
- 编写测试用例
- 更新文档

### 关键算法实现

1. **多样性评分 (HHI指数)**
   ```rust
   fn calculate_diversification(holdings: &[Holding]) -> f64 {
       let hhi: f64 = holdings.iter()
           .map(|h| {
               let weight = h.weight / 100.0;
               weight * weight
           })
           .sum();

       let n = holdings.len() as f64;
       let min_hhi = 1.0 / n;
       let score = 1.0 - (hhi - min_hhi) / (1.0 - min_hhi);
       score.max(0.0).min(1.0)
   }
   ```

2. **ROI 评分 (Sigmoid函数)**
   ```rust
   fn score_roi(pnl_7d: f64, pnl_30d: f64) -> f64 {
       let sigmoid = |x: f64| 1.0 / (1.0 + (-x).exp());
       let score_7d = sigmoid(pnl_7d / 100.0) * 100.0;
       let score_30d = sigmoid(pnl_30d / 100.0) * 100.0;
       score_7d * 0.4 + score_30d * 0.6
   }
   ```

3. **风险分数计算**
   ```rust
   fn calculate_risk_score(check: &SecurityCheck) -> u8 {
       let mut score = 0;
       if !check.lp_burned { score += 20; }
       if check.is_honeypot { score += 40; }
       if check.can_mint { score += 25; }
       if !check.ownership_renounced { score += 10; }
       if check.liquidity_usd < MIN_LIQUIDITY { score += 15; }
       if check.top_10_holder_percent > 50.0 { score += 10; }
       score.min(100)
   }
   ```

### 测试要求

请编写以下测试:

1. **单元测试**
   - 评分算法正确性
   - 多样性计算准确性
   - 风险评估逻辑
   - 筛选条件应用

2. **集成测试**
   - API 客户端调用
   - 完整的钱包扫描流程
   - 配置文件加载

### 输出要求

请提供:

1. **实现代码**
   - 所有模块的完整实现
   - 清晰的注释和文档字符串

2. **测试代码**
   - 单元测试覆盖核心逻辑
   - 示例使用代码

3. **使用文档**
   - 快速开始指南
   - API 使用示例
   - 配置说明

4. **集成指南**
   - 如何添加到现有项目
   - 依赖项说明
   - 注意事项

### 参考资源

- GMGN API 文档: https://docs.gmgn.ai
- 原项目仓库: https://github.com/yourusername/gmgn-tracker
- Rust 异步编程: https://tokio.rs

### 额外考虑

1. **性能优化**
   - 使用并发请求
   - 实现本地缓存
   - 批量处理

2. **错误处理**
   - 优雅处理 API 错误
   - 网络超时重试
   - 详细的错误信息

3. **可扩展性**
   - 支持多链(Solana, ETH, BSC等)
   - 插件化筛选策略
   - 自定义评分权重

### 示例使用场景

```rust
// 示例 1: 扫描智能钱包
use gmgn::{GmgnClient, WalletFilter, WalletScorer};

async fn scan_smart_wallets() -> Result<()> {
    let client = GmgnClient::new("https://gmgn.ai", 30)?;

    let filter = WalletFilter {
        min_roi_7d: 50.0,
        min_win_rate: 60.0,
        // ...
    };

    let wallets = client.get_smart_wallets(Chain::Solana).await?;
    let filtered: Vec<_> = wallets.into_iter()
        .filter(|w| filter.passes(w))
        .collect();

    let mut scored: Vec<_> = filtered.iter()
        .map(|w| WalletScorer::score(w))
        .collect();

    scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    for wallet in scored.iter().take(10) {
        println!("{}: {:.1}", wallet.wallet.address, wallet.score);
    }

    Ok(())
}

// 示例 2: 检查代币安全
use gmgn::RiskAssessor;

async fn check_token_safety(token_address: &str) -> Result<()> {
    let client = GmgnClient::new("https://gmgn.ai", 30)?;
    let assessor = RiskAssessor::new(client);

    let report = assessor.assess_token(token_address).await?;

    println!("风险等级: {}", report.risk_level);
    println!("风险分数: {}", report.security_check.risk_score);
    println!("建议: {}", report.recommendation);

    Ok(())
}
```

请开始实现!
```

---

## Prompt 使用说明

### 如何使用此 Prompt

1. **复制 Prompt**
   - 将上述完整 Prompt 复制到你的 AI 助手(如 Claude、ChatGPT 等)

2. **提供上下文**
   - 告诉 AI 你的项目类型(CLI、Web、库等)
   - 说明你想要哪些功能(基础/进阶)
   - 指定使用的 Rust 版本和依赖

3. **逐步实现**
   - AI 会按照步骤提供代码
   - 每一步都会有详细说明
   - 包含测试和文档

4. **自定义调整**
   - 根据需要调整评分权重
   - 修改筛选条件
   - 扩展支持的链

### 示例对话

**你:**
```
[粘贴上述 Prompt]

我想在我的 Rust CLI 项目中集成这个功能。
我的项目已经有 tokio 和 reqwest 依赖。
请先帮我实现基础集成部分。
```

**AI:**
```
好的!我会帮你实现基础集成。让我们从创建模块结构开始...

[AI 会提供详细的实现代码、说明和测试]
```

---

## 快速集成检查清单

在另一个项目中集成时,请确保:

- [ ] Rust 版本 >= 1.70
- [ ] 添加必要的依赖项
  - [ ] tokio (异步运行时)
  - [ ] reqwest (HTTP 客户端)
  - [ ] serde (序列化)
  - [ ] thiserror/anyhow (错误处理)
- [ ] 创建模块目录结构
- [ ] 实现核心算法
  - [ ] 评分系统
  - [ ] 筛选逻辑
  - [ ] 风险评估
- [ ] 编写测试用例
- [ ] 更新项目文档
- [ ] 配置 CI/CD

---

## 常见问题

### Q: 如何处理 GMGN API 的变化?

A:
1. 使用抽象层封装 API 调用
2. 定期更新 API 端点配置
3. 添加版本兼容性检查

### Q: 如何优化性能?

A:
1. 使用并发请求 (tokio::spawn)
2. 实现本地缓存 (sled 或 redis)
3. 批量处理钱包数据

### Q: 如何扩展到其他链?

A:
1. 添加 Chain 枚举变体
2. 实现链特定的 API 适配器
3. 更新评分算法以适应不同链的特性

---

## 联系支持

如果在集成过程中遇到问题:

1. 查看原项目文档: `docs/architecture.md`
2. 提交 Issue: https://github.com/yourusername/gmgn-tracker/issues
3. 参考示例代码: `examples/`

祝集成顺利! 🚀

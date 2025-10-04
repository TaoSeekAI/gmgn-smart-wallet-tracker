# GMGN 聪明钱包追踪器 - 核心架构设计

## 项目概述

这是一个基于 Rust 开发的命令行工具,用于自动化追踪和筛选 GMGN.ai 平台上的"聪明钱包"(Smart Money Wallets),并提供跟单建议。

## 核心设计理念(高中生版解读)

想象你在玩一个投资游戏,有些玩家特别厉害,总能赚钱。这个工具就像一个"侦探助手",帮你:
1. 找出那些最厉害的玩家(聪明钱包)
2. 看他们买了什么(持仓分析)
3. 跟着他们做(跟单建议)
4. 避开坏东西(安全检查)

## 系统架构

```
┌─────────────────────────────────────────────────────────────┐
│                     CLI 命令行界面                           │
│  (用户通过命令与工具交互)                                     │
└──────────────────┬──────────────────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────────────────┐
│                 核心业务逻辑层                                │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │ 钱包追踪 │  │ 数据筛选 │  │ 跟单建议 │  │ 风险评估 │   │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │
└──────────────────┬──────────────────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────────────────┐
│                  数据访问层                                   │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ GMGN API     │  │ 本地缓存     │  │ 配置管理     │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└──────────────────────────────────────────────────────────────┘
```

## 核心模块详解

### 1. GMGN API 客户端模块 (`src/api/`)

**职责**: 与 GMGN.ai 平台通信

**主要功能**:
- 获取智能钱包排行榜
- 查询钱包详细信息
- 获取交易历史
- 检查代币安全性

**关键接口**:
```rust
// 基于已知的 GMGN API 端点
pub struct GmgnClient {
    base_url: String,
    http_client: reqwest::Client,
}

impl GmgnClient {
    // 获取 Solana 智能钱包排行
    pub async fn get_smart_wallets(&self, chain: Chain) -> Result<Vec<Wallet>>;

    // 获取钱包详情
    pub async fn get_wallet_detail(&self, address: &str) -> Result<WalletDetail>;

    // 获取钱包 PnL 数据
    pub async fn get_wallet_pnl(&self, address: &str, period: Period) -> Result<PnlData>;

    // 检查代币安全性
    pub async fn check_token_security(&self, token_address: &str) -> Result<SecurityCheck>;
}
```

### 2. 钱包追踪引擎 (`src/tracker/`)

**职责**: 实现智能钱包的发现和追踪逻辑

**核心策略**:
```rust
pub struct WalletFilter {
    // 7天收益率阈值 (例如: > 50%)
    min_roi_7d: f64,

    // 30天收益率阈值
    min_roi_30d: f64,

    // 最小胜率 (例如: > 60%)
    min_win_rate: f64,

    // 最大回撤限制 (例如: < 30%)
    max_drawdown: f64,

    // 最小交易次数 (避免样本太小)
    min_trade_count: u32,

    // 持仓多样性检查
    require_diversification: bool,
}

pub struct WalletTracker {
    filters: WalletFilter,
    api_client: GmgnClient,
}

impl WalletTracker {
    // 扫描并筛选智能钱包
    pub async fn scan_smart_wallets(&self) -> Result<Vec<ScoredWallet>>;

    // 对钱包进行评分
    fn score_wallet(&self, wallet: &WalletDetail) -> WalletScore;

    // 检查持仓多样性
    fn check_diversification(&self, holdings: &[Holding]) -> bool;
}
```

### 3. 跟单策略模块 (`src/strategy/`)

**职责**: 分析钱包动作并生成跟单建议

**主要组件**:
```rust
pub struct TradingSignal {
    wallet_address: String,
    action: Action,  // Buy / Sell
    token_address: String,
    token_symbol: String,
    confidence: f64,  // 置信度 0-1
    reason: String,
    timestamp: DateTime<Utc>,
}

pub struct StrategyEngine {
    // 监控的钱包列表
    watched_wallets: Vec<String>,

    // 信号生成器
    signal_generator: SignalGenerator,
}

impl StrategyEngine {
    // 监控钱包动作
    pub async fn monitor_wallets(&self) -> Result<Vec<TradingSignal>>;

    // 验证代币安全性
    async fn validate_token_security(&self, token: &str) -> Result<bool>;

    // 生成跟单建议
    pub fn generate_recommendations(&self, signals: &[TradingSignal]) -> Vec<Recommendation>;
}
```

### 4. 风险评估模块 (`src/risk/`)

**职责**: 评估代币和交易的风险

**安全检查清单**:
```rust
pub struct SecurityCheck {
    // LP 是否销毁
    lp_burned: bool,

    // 是否蜜罐合约
    is_honeypot: bool,

    // 是否可增发
    can_mint: bool,

    // 合约所有权是否放弃
    ownership_renounced: bool,

    // 流动性大小
    liquidity_usd: f64,

    // 持有者分布
    holder_distribution: HolderStats,
}

pub struct RiskAssessor {
    api_client: GmgnClient,
}

impl RiskAssessor {
    // 执行完整的安全检查
    pub async fn assess_token(&self, token_address: &str) -> Result<RiskReport>;

    // 计算风险分数 (0-100, 越低越安全)
    pub fn calculate_risk_score(&self, check: &SecurityCheck) -> u8;
}
```

### 5. 数据持久化模块 (`src/storage/`)

**职责**: 缓存数据,减少 API 调用

```rust
pub struct Cache {
    // 使用 sled 作为嵌入式数据库
    db: sled::Db,
    ttl: Duration,
}

impl Cache {
    // 缓存钱包数据
    pub fn cache_wallet(&self, address: &str, data: &WalletDetail) -> Result<()>;

    // 读取缓存
    pub fn get_wallet(&self, address: &str) -> Result<Option<WalletDetail>>;

    // 清理过期缓存
    pub fn cleanup_expired(&self) -> Result<()>;
}
```

### 6. CLI 接口模块 (`src/cli/`)

**职责**: 提供用户友好的命令行界面

**命令设计**:
```bash
# 扫描智能钱包
gmgn-tracker scan --chain solana --min-roi 50 --min-winrate 60

# 查看钱包详情
gmgn-tracker wallet <address> --details

# 监控钱包动作
gmgn-tracker monitor --wallets wallet1,wallet2 --interval 60

# 检查代币安全
gmgn-tracker check-token <token_address>

# 导出监控列表
gmgn-tracker export --format json --output wallets.json
```

## 数据流程

### 钱包发现流程
```
1. 用户执行 scan 命令
   ↓
2. 调用 GMGN API 获取智能钱包榜单
   ↓
3. 应用筛选条件 (ROI, 胜率, 回撤等)
   ↓
4. 分析持仓多样性
   ↓
5. 计算综合评分
   ↓
6. 排序并展示结果
   ↓
7. (可选) 保存到监控列表
```

### 跟单监控流程
```
1. 启动 monitor 命令
   ↓
2. 轮询监控钱包的最新交易
   ↓
3. 检测到新交易 → 生成信号
   ↓
4. 执行安全检查 (LP/蜜罐/增发)
   ↓
5. 如果通过检查 → 输出跟单建议
   ↓
6. 记录到日志
   ↓
7. (可选) 发送通知
```

## 关键技术选型

| 组件 | 技术选择 | 理由 |
|------|---------|------|
| HTTP 客户端 | reqwest | 异步,功能完善 |
| 序列化 | serde + serde_json | Rust 标准选择 |
| CLI 框架 | clap | 强大的命令行参数解析 |
| 异步运行时 | tokio | 生态最成熟 |
| 数据库 | sled | 嵌入式,无需外部依赖 |
| 配置管理 | config | 支持多种格式 |
| 日志 | tracing | 结构化日志 |
| 错误处理 | thiserror + anyhow | 优雅的错误处理 |

## 配置文件设计

```yaml
# config.yaml
gmgn:
  base_url: "https://gmgn.ai"
  timeout_seconds: 30

filters:
  min_roi_7d: 50.0      # 7天最小收益率 (%)
  min_roi_30d: 100.0    # 30天最小收益率 (%)
  min_win_rate: 60.0    # 最小胜率 (%)
  max_drawdown: 30.0    # 最大回撤 (%)
  min_trade_count: 10   # 最小交易次数
  require_diversification: true

risk:
  require_lp_burned: true
  block_honeypot: true
  block_mintable: true
  min_liquidity_usd: 10000

cache:
  ttl_seconds: 300      # 缓存有效期 5分钟
  max_size_mb: 100

logging:
  level: "info"
  file: "gmgn-tracker.log"
```

## 错误处理策略

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TrackerError {
    #[error("API 请求失败: {0}")]
    ApiError(String),

    #[error("数据解析错误: {0}")]
    ParseError(String),

    #[error("缓存操作失败: {0}")]
    CacheError(String),

    #[error("配置错误: {0}")]
    ConfigError(String),

    #[error("网络错误: {0}")]
    NetworkError(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, TrackerError>;
```

## 性能优化考虑

1. **并发请求**: 使用 tokio 并发获取多个钱包数据
2. **缓存策略**: 本地缓存减少 API 调用
3. **增量更新**: 只获取变化的数据
4. **连接池**: 复用 HTTP 连接
5. **批量处理**: 批量查询多个钱包

## 安全性考虑

1. **API 限流**: 实现请求限流,避免被封禁
2. **数据验证**: 严格验证 API 返回数据
3. **敏感信息**: 不在日志中记录敏感信息
4. **错误处理**: 优雅处理网络错误和超时

## 可扩展性设计

1. **插件化筛选器**: 支持自定义筛选策略
2. **多链支持**: 设计抽象层支持 ETH/BSC 等其他链
3. **通知系统**: 预留接口支持 Telegram/Discord 通知
4. **导出格式**: 支持多种数据导出格式

## 测试策略

1. **单元测试**: 核心业务逻辑
2. **集成测试**: API 客户端 (使用 mock)
3. **端到端测试**: 完整的命令行流程
4. **性能测试**: 并发处理能力

## 项目里程碑

### Phase 1: MVP (最小可用产品)
- [x] 项目架构设计
- [ ] GMGN API 客户端实现
- [ ] 基础钱包筛选功能
- [ ] CLI 基础命令

### Phase 2: 核心功能
- [ ] 完整的筛选策略
- [ ] 安全检查模块
- [ ] 数据缓存
- [ ] 配置文件支持

### Phase 3: 高级特性
- [ ] 实时监控模式
- [ ] 跟单建议生成
- [ ] 多链支持
- [ ] 导出和报告

### Phase 4: 生产就绪
- [ ] 完整测试覆盖
- [ ] Docker 部署
- [ ] CI/CD 流水线
- [ ] 完整文档

## 下一步行动

1. 初始化 Rust 项目
2. 实现 GMGN API 客户端
3. 编写第一个 CLI 命令
4. 添加单元测试

# GMGN智能钱包追踪器 / GMGN Smart Wallet Tracker

> 一个基于 Rust 的命令行工具,用于追踪和分析 GMGN.ai 平台上的聪明钱包(Smart Money Wallets)。

[English](#english) | [中文](#中文)

---

## 中文

### 项目简介

这是一个自动化 CLI 工具,帮助您:
- 🔍 发现并筛选 GMGN.ai 上的高收益钱包
- 📊 分析钱包的历史表现和策略
- 🛡️ 评估代币的安全风险
- 📈 提供基于数据的跟单建议

### 核心特性

#### 1. 智能钱包扫描
- 支持 Solana、Ethereum、Base、BSC 等多链
- 多维度筛选:ROI、胜率、回撤、交易次数
- 持仓多样性分析
- 综合评分系统(0-100分)

#### 2. 钱包评分算法
系统使用加权评分模型,综合考虑:
- **ROI评分** (30%): 7天和30天收益率
- **胜率评分** (25%): 历史交易成功率
- **一致性评分** (20%): 基于最大回撤和Sharpe比率
- **多样性评分** (15%): 持仓分散程度(HHI指数)
- **活跃度评分** (10%): 交易频率

#### 3. 风险评估
自动检查代币安全性:
- LP 是否销毁
- 蜜罐合约检测
- 代币可增发检测
- 合约所有权检查
- 流动性充足性
- 持有者集中度分析

### 快速开始

#### 重要说明

**API 访问限制**: GMGN.ai 的数据爬取 API 需要 IP 白名单,因此本工具提供了两种模式:

1. **模拟数据模式** (推荐用于测试): 使用 `--mock` 标志生成模拟数据,无需 API 访问权限
2. **实时 API 模式**: 需要在 GMGN.ai 有交易历史并申请 IP 白名单

详细的 API 访问问题解决方案,请参阅 [TROUBLESHOOTING.md](./TROUBLESHOOTING.md)

#### 安装

1. 克隆仓库:
```bash
git clone https://github.com/TaoSeekAI/gmgn-smart-wallet-tracker.git
cd gmgn-smart-wallet-tracker
```

2. 构建项目:
```bash
cargo build --release
```

3. 将可执行文件添加到 PATH:
```bash
export PATH=$PATH:$(pwd)/target/release
```

#### 基本使用

1. **扫描智能钱包**
```bash
# 使用模拟数据模式(无需 API 访问权限)
gmgn-tracker scan --chain solana --mock

# 扫描 Solana 链上的智能钱包(需要 API 访问权限)
gmgn-tracker scan --chain solana

# 自定义筛选条件
gmgn-tracker scan \
  --chain solana \
  --min-roi-7d 50 \
  --min-roi-30d 100 \
  --min-win-rate 60 \
  --max-drawdown 30 \
  --min-trades 10 \
  --limit 20 \
  --mock
```

2. **查看钱包详情**
```bash
gmgn-tracker wallet <钱包地址> --chain solana --holdings --trades
```

3. **检查代币安全性**
```bash
gmgn-tracker check-token <代币地址> --chain solana
```

4. **生成配置文件**
```bash
gmgn-tracker config --output config.yaml
```

5. **使用配置文件**
```bash
gmgn-tracker scan --config config.yaml
```

### 配置文件示例

```yaml
gmgn:
  base_url: "https://gmgn.ai"
  timeout_seconds: 30
  rate_limit_per_minute: 60

filters:
  min_roi_7d: 50.0        # 7天最小ROI (%)
  min_roi_30d: 100.0      # 30天最小ROI (%)
  min_win_rate: 60.0      # 最小胜率 (%)
  max_drawdown: 30.0      # 最大回撤 (%)
  min_trade_count: 10     # 最小交易次数
  require_diversification: true
  min_diversification_score: 0.6

risk:
  require_lp_burned: true
  block_honeypot: true
  block_mintable: true
  min_liquidity_usd: 10000.0
  max_risk_score: 50

cache:
  ttl_seconds: 300
  max_size_mb: 100
  cache_dir: ".cache"

logging:
  level: "info"
  file: "gmgn-tracker.log"
```

### 输出格式

支持三种输出格式:
- `table` (默认): 彩色表格显示
- `json`: JSON格式,便于程序处理
- `csv`: CSV格式,便于Excel分析

```bash
gmgn-tracker scan --format json > results.json
gmgn-tracker scan --format csv > results.csv
```

### 项目架构

详细架构说明请查看 [docs/architecture.md](docs/architecture.md)

```
├── src/
│   ├── api/           # GMGN API客户端
│   ├── cli/           # 命令行接口
│   ├── config.rs      # 配置管理
│   ├── error.rs       # 错误类型
│   ├── models/        # 数据模型
│   ├── risk/          # 风险评估
│   ├── tracker/       # 钱包追踪
│   └── main.rs        # 主入口
├── docs/              # 文档
├── tests/             # 测试
└── Cargo.toml         # 项目配置
```

### 技术栈

- **Rust 2021**: 高性能、内存安全
- **Tokio**: 异步运行时
- **Reqwest**: HTTP客户端
- **Clap**: CLI框架
- **Serde**: 序列化/反序列化
- **Tracing**: 结构化日志

### 开发路线图

- [x] 核心钱包筛选功能
- [x] 风险评估模块
- [x] CLI 基础命令
- [x] 配置文件支持
- [ ] 实时监控模式
- [ ] Telegram 通知集成
- [ ] Web UI 界面
- [ ] 回测功能

### Docker 部署

```bash
# 构建镜像
docker-compose build

# 运行容器
docker-compose up -d

# 查看日志
docker-compose logs -f
```

### 贡献指南

欢迎提交 Issues 和 Pull Requests!

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

### 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

### 免责声明

⚠️ **风险提示**:
- 本工具仅供学习和研究使用
- 不构成任何投资建议
- 加密货币交易有风险,请谨慎决策
- 跟随任何钱包交易均可能导致资金损失

---

## English

### Overview

A Rust-based CLI tool for tracking and analyzing smart money wallets on GMGN.ai platform.

### Features

- 🔍 Discover and filter high-performing wallets
- 📊 Analyze wallet historical performance
- 🛡️ Assess token security risks
- 📈 Provide data-driven copy-trading suggestions

### Quick Start

#### Important Notice

**API Access Limitation**: GMGN.ai's data crawling APIs require IP whitelist. This tool provides two modes:

1. **Mock Data Mode** (Recommended for testing): Use `--mock` flag to generate mock data without API access
2. **Live API Mode**: Requires transaction history on GMGN.ai and IP whitelist application

For detailed API access troubleshooting, see [TROUBLESHOOTING.md](./TROUBLESHOOTING.md)

```bash
# Build the project
cargo build --release

# Scan smart wallets with mock data (no API access needed)
./target/release/gmgn-tracker scan --chain solana --mock

# Scan smart wallets with real API (requires whitelist)
./target/release/gmgn-tracker scan --chain solana

# Check token security
./target/release/gmgn-tracker check-token <TOKEN_ADDRESS>

# View wallet details
./target/release/gmgn-tracker wallet <WALLET_ADDRESS> --chain solana
```

### Scoring Algorithm

The system uses a weighted scoring model:
- **ROI Score** (30%): 7-day and 30-day returns
- **Win Rate Score** (25%): Historical success rate
- **Consistency Score** (20%): Based on max drawdown and Sharpe ratio
- **Diversification Score** (15%): Portfolio concentration (HHI index)
- **Activity Score** (10%): Trading frequency

### Configuration

Generate a default config file:
```bash
gmgn-tracker config --output config.yaml
```

### Documentation

See [docs/architecture.md](docs/architecture.md) for detailed architecture documentation.

### Tech Stack

- **Rust 2021**: High performance, memory safety
- **Tokio**: Async runtime
- **Reqwest**: HTTP client
- **Clap**: CLI framework
- **Serde**: Serialization
- **Tracing**: Structured logging

### License

MIT License

### Disclaimer

⚠️ **Risk Warning**:
- This tool is for educational and research purposes only
- Not financial advice
- Cryptocurrency trading involves risks
- You may lose all your capital

---

## 联系方式 / Contact

- GitHub Issues: [Report a bug](https://github.com/yourusername/gmgn-tracker/issues)
- Email: your.email@example.com

---

Made with ❤️ by GMGN Tracker Team

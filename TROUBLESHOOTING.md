# 故障排查指南

## 常见问题

### ❌ 错误: API returned error status: 403 Forbidden

#### 问题原因

GMGN.ai 的数据爬取 API 有以下限制:
1. **需要 IP 白名单** - 不是公开 API
2. **需要交易历史** - 必须在 GMGN 上有交易记录
3. **有速率限制** - 每秒最多 2 个请求

#### 解决方案

##### 方案 1: 申请 IP 白名单 (推荐用于真实使用)

1. **前置条件**:
   - 在 GMGN.ai 上有交易历史
   - 有 GMGN 邀请码

2. **申请步骤**:
   - 访问 https://docs.gmgn.ai/index/cooperation-api-access-data-api
   - 提供以下信息:
     - 你的 GMGN 交易地址
     - 你的邀请码
     - 需要加入白名单的 IP 地址

3. **配置工具**:
   ```bash
   # 获取你的公网 IP
   curl https://api.ipify.org

   # 申请通过后，正常使用即可
   gmgn-tracker scan --chain solana --limit 10
   ```

##### 方案 2: 使用 GMGN 交易 API (免费，无需认证)

GMGN 的交易 API 不需要认证，可以直接使用:

```rust
// 示例: 使用交易 API
// URL: https://gmgn.ai/defi/router/v1/sol/tx/get_swap_route
```

**注意**: 交易 API 主要用于执行交易，不能获取完整的钱包排行榜数据。

##### 方案 3: 使用第三方数据源 (开发和学习)

对于学习和开发目的，可以使用:

1. **Apify GMGN Scraper** (付费)
   - https://apify.com/muhammetakkurtt/gmgn-wallet-stat-scraper

2. **BitQuery** (付费)
   - https://docs.bitquery.io/docs/examples/Solana/solana-gmgn-api/

3. **模拟数据** (本项目演示)
   ```bash
   # 使用 --mock 标志生成模拟数据
   gmgn-tracker scan --chain solana --limit 10 --mock
   ```

##### 方案 4: Web 抓取 (仅供学习，请遵守 GMGN 的 ToS)

可以通过浏览器自动化工具抓取 GMGN 网页:

```bash
# 使用 Puppeteer, Selenium 等
# 访问: https://gmgn.ai/discover?chain=sol
```

**警告**: Web 抓取可能违反服务条款，仅供学习参考。

---

## 其他常见错误

### ❌ 错误: Network timeout

#### 原因
- 网络连接问题
- GMGN API 响应慢

#### 解决方案
```bash
# 增加超时时间
gmgn-tracker scan --timeout 60

# 或修改配置文件
# config.yaml
gmgn:
  timeout_seconds: 60
```

### ❌ 错误: Rate limit exceeded

#### 原因
- 请求过于频繁 (> 2次/秒)

#### 解决方案
```bash
# 减少并发请求
gmgn-tracker scan --concurrency 1 --delay 500
```

### ❌ 错误: Invalid chain

#### 原因
- 链名称错误

#### 解决方案
```bash
# 支持的链: solana, ethereum, base, bsc
gmgn-tracker scan --chain solana  # ✅ 正确
gmgn-tracker scan --chain sol     # ✅ 也可以
gmgn-tracker scan --chain polygon # ❌ 不支持
```

---

## API 端点参考

### 公开端点 (无需认证)

#### 1. 交易路由查询
```
GET https://gmgn.ai/defi/router/v1/sol/tx/get_swap_route
参数:
  - token_in_address
  - token_out_address
  - in_amount
  - from_address
  - slippage
```

#### 2. 交易状态查询
```
GET https://gmgn.ai/defi/router/v1/sol/tx/get_transaction_status
参数:
  - hash
  - last_valid_height
```

### 受限端点 (需要白名单)

#### 1. 智能钱包活动
```
GET https://gmgn.ai/api/v1/smartmoney/sol/walletNew/wallet_activity
限制:
  - 需要 IP 白名单
  - 限速: 2 req/s
```

#### 2. 钱包持仓
```
GET https://gmgn.ai/api/v1/wallet_holdings/{address}
限制:
  - 需要 IP 白名单
  - 限速: 2 req/s
```

---

## 开发和测试建议

### 使用模拟模式

对于开发和测试，建议使用模拟模式:

```bash
# 生成模拟的智能钱包数据
gmgn-tracker scan --chain solana --limit 10 --mock

# 模拟数据包含:
# - 随机生成的钱包地址
# - 合理的 ROI、胜率等指标
# - 用于测试评分算法
```

### 单元测试

```bash
# 运行测试 (不需要真实 API)
cargo test

# 运行集成测试 (需要真实 API 或 mock)
cargo test --features integration
```

### 使用 Postman 测试 API

1. 导入 GMGN API 集合
2. 测试各个端点
3. 检查响应格式
4. 更新代码中的数据结构

---

## 获取帮助

### GMGN 官方资源
- 文档: https://docs.gmgn.ai
- Telegram: @gmgn_sol (Solana)
- Discord: 查看官网

### 本项目资源
- GitHub Issues: https://github.com/TaoSeekAI/gmgn-smart-wallet-tracker/issues
- 文档: `/docs/` 目录
- 示例: `/examples/` 目录

---

## 配置检查清单

在使用前，请确认:

- [ ] 已阅读 GMGN API 文档
- [ ] 了解 API 限制 (白名单/速率)
- [ ] 配置文件正确 (`config.yaml`)
- [ ] 网络连接正常
- [ ] 如果使用真实 API，已申请白名单
- [ ] 如果是开发/学习，使用 `--mock` 模式

---

## 快速诊断命令

```bash
# 1. 检查网络连接
ping gmgn.ai

# 2. 测试 API 可访问性
curl -I https://gmgn.ai

# 3. 检查配置
gmgn-tracker config --validate

# 4. 查看详细日志
gmgn-tracker scan --chain solana --log-level debug

# 5. 使用模拟模式测试
gmgn-tracker scan --chain solana --mock
```

---

**最后更新**: 2025-10-04
**适用版本**: v0.1.0+

mod api;
mod cli;
mod config;
mod error;
mod models;
mod risk;
mod tracker;

use clap::Parser;
use cli::{Cli, OutputFormatter};
use cli::commands::Commands;
use config::Config;
use error::Result;
use std::str::FromStr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // 解析命令行参数
    let cli = Cli::parse();

    // 初始化日志
    init_logging(&cli.log_level);

    // 加载配置
    let config = if let Some(config_path) = &cli.config {
        Config::load_from_file(config_path)?
    } else {
        Config::load()?
    };

    // 执行命令
    match &cli.command {
        Commands::Scan {
            chain,
            min_roi_7d,
            min_roi_30d,
            min_win_rate,
            max_drawdown,
            min_trades,
            limit,
            format,
            mock,
        } => {
            handle_scan(
                &config,
                chain,
                *min_roi_7d,
                *min_roi_30d,
                *min_win_rate,
                *max_drawdown,
                *min_trades,
                *limit,
                format,
                *mock,
            )
            .await?;
        }
        Commands::Wallet {
            address,
            chain,
            holdings,
            trades,
        } => {
            handle_wallet(&config, address, chain, *holdings, *trades).await?;
        }
        Commands::Monitor {
            wallets,
            chain,
            interval,
            security_check,
        } => {
            handle_monitor(&config, wallets, chain, *interval, *security_check).await?;
        }
        Commands::CheckToken {
            token_address,
            chain,
        } => {
            handle_check_token(&config, token_address, chain).await?;
        }
        Commands::Export { output, format } => {
            handle_export(&config, output, format).await?;
        }
        Commands::Config { output } => {
            handle_generate_config(output)?;
        }
    }

    Ok(())
}

fn init_logging(level: &str) {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(level));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}

async fn handle_scan(
    config: &Config,
    chain_str: &str,
    min_roi_7d: Option<f64>,
    min_roi_30d: Option<f64>,
    min_win_rate: Option<f64>,
    max_drawdown: Option<f64>,
    min_trades: Option<u32>,
    limit: usize,
    format: &str,
    mock: bool,
) -> Result<()> {
    use models::Chain;
    use tracker::{WalletFilter, WalletTracker};

    let chain = Chain::from_str(chain_str)
        .map_err(|e| error::TrackerError::ConfigError(e))?;

    // 构建过滤器
    let mut filter_config = config.filters.clone();
    if let Some(roi) = min_roi_7d {
        filter_config.min_roi_7d = roi;
    }
    if let Some(roi) = min_roi_30d {
        filter_config.min_roi_30d = roi;
    }
    if let Some(wr) = min_win_rate {
        filter_config.min_win_rate = wr;
    }
    if let Some(dd) = max_drawdown {
        filter_config.max_drawdown = dd;
    }
    if let Some(tc) = min_trades {
        filter_config.min_trade_count = tc;
    }

    let filter = WalletFilter::from(filter_config);

    // 获取钱包列表
    let wallets = if mock {
        println!("🎭 使用模拟数据模式...");
        api::MockDataGenerator::generate_wallets(&chain, 50)?
    } else {
        // 创建 API 客户端
        let client = api::GmgnClient::new(
            config.gmgn.base_url.clone(),
            config.gmgn.timeout_seconds,
        )?;

        // 创建追踪器
        let tracker = WalletTracker::new(filter, client);

        println!("🔍 正在扫描 {} 链上的智能钱包...", chain);

        // 扫描钱包
        tracker.scan_smart_wallets(&chain).await?
    };

    if wallets.is_empty() {
        println!("❌ 未找到符合条件的钱包");
        return Ok(());
    }

    // 输出结果
    match format.to_lowercase().as_str() {
        "json" => {
            let json = OutputFormatter::format_wallets_json(&wallets, limit)
                .map_err(|e| error::TrackerError::SerializationError(e))?;
            println!("{}", json);
        }
        "csv" => {
            let csv = OutputFormatter::format_wallets_csv(&wallets, limit);
            println!("{}", csv);
        }
        _ => {
            let table = OutputFormatter::format_wallets_table(&wallets, limit);
            println!("{}", table);
        }
    }

    Ok(())
}

async fn handle_wallet(
    config: &Config,
    address: &str,
    chain_str: &str,
    _show_holdings: bool,
    _show_trades: bool,
) -> Result<()> {
    use models::Chain;

    let chain = Chain::from_str(chain_str)
        .map_err(|e| error::TrackerError::ConfigError(e))?;

    let client = api::GmgnClient::new(
        config.gmgn.base_url.clone(),
        config.gmgn.timeout_seconds,
    )?;

    println!("📊 正在获取钱包详情: {}", address);

    let detail = client.get_wallet_detail(address, &chain).await?;

    // 计算评分
    let scored = tracker::WalletScorer::score(&detail);

    println!("\n钱包地址: {}", detail.address);
    println!("综合评分: {:.1}/100", scored.score);
    println!("\n7 天数据:");
    println!("  ROI: {:.2}%", detail.pnl_7d.roi);
    println!("  胜率: {:.2}%", detail.pnl_7d.win_rate);
    println!("  交易次数: {}", detail.pnl_7d.trade_count);
    println!("\n30 天数据:");
    println!("  ROI: {:.2}%", detail.pnl_30d.roi);
    println!("  胜率: {:.2}%", detail.pnl_30d.win_rate);
    println!("  最大回撤: {:.2}%", detail.pnl_30d.max_drawdown);
    println!("  交易次数: {}", detail.pnl_30d.trade_count);

    Ok(())
}

async fn handle_monitor(
    _config: &Config,
    _wallets: &str,
    _chain: &str,
    _interval: u64,
    _security_check: bool,
) -> Result<()> {
    println!("⏳ 监控功能开发中...");
    Ok(())
}

async fn handle_check_token(
    config: &Config,
    token_address: &str,
    _chain: &str,
) -> Result<()> {
    let client = api::GmgnClient::new(
        config.gmgn.base_url.clone(),
        config.gmgn.timeout_seconds,
    )?;

    let assessor = risk::RiskAssessor::new(client, config.risk.clone());

    println!("🔒 正在检查代币安全性: {}", token_address);

    let report = assessor.assess_token(token_address).await?;

    let output = OutputFormatter::format_risk_report(token_address, &report);
    println!("{}", output);

    Ok(())
}

async fn handle_export(
    _config: &Config,
    _output: &str,
    _format: &str,
) -> Result<()> {
    println!("💾 导出功能开发中...");
    Ok(())
}

fn handle_generate_config(output: &str) -> Result<()> {
    let config = Config::default();
    config.save_to_file(output)?;
    println!("✅ 配置文件已生成: {}", output);
    Ok(())
}

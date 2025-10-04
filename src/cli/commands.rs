use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "gmgn-tracker")]
#[command(author = "GMGN Tracker Team")]
#[command(version = "0.1.0")]
#[command(about = "Smart money wallet tracker for GMGN.ai", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// 配置文件路径
    #[arg(short, long, global = true)]
    pub config: Option<String>,

    /// 日志级别 (trace, debug, info, warn, error)
    #[arg(short = 'l', long, global = true, default_value = "info")]
    pub log_level: String,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 扫描并筛选智能钱包
    Scan {
        /// 区块链 (solana, ethereum, base, bsc)
        #[arg(short, long, default_value = "solana")]
        chain: String,

        /// 最小 7 天 ROI (%)
        #[arg(long)]
        min_roi_7d: Option<f64>,

        /// 最小 30 天 ROI (%)
        #[arg(long)]
        min_roi_30d: Option<f64>,

        /// 最小胜率 (%)
        #[arg(long)]
        min_win_rate: Option<f64>,

        /// 最大回撤 (%)
        #[arg(long)]
        max_drawdown: Option<f64>,

        /// 最小交易次数
        #[arg(long)]
        min_trades: Option<u32>,

        /// 输出结果数量
        #[arg(short = 'n', long, default_value = "10")]
        limit: usize,

        /// 输出格式 (table, json, csv)
        #[arg(short = 'f', long, default_value = "table")]
        format: String,
    },

    /// 查看钱包详细信息
    Wallet {
        /// 钱包地址
        address: String,

        /// 区块链
        #[arg(short, long, default_value = "solana")]
        chain: String,

        /// 是否显示持仓详情
        #[arg(long)]
        holdings: bool,

        /// 是否显示交易历史
        #[arg(long)]
        trades: bool,
    },

    /// 监控钱包动作
    Monitor {
        /// 要监控的钱包地址 (逗号分隔)
        #[arg(short, long)]
        wallets: String,

        /// 区块链
        #[arg(short, long, default_value = "solana")]
        chain: String,

        /// 检查间隔 (秒)
        #[arg(short, long, default_value = "60")]
        interval: u64,

        /// 是否启用安全检查
        #[arg(long, default_value = "true")]
        security_check: bool,
    },

    /// 检查代币安全性
    CheckToken {
        /// 代币地址
        token_address: String,

        /// 区块链
        #[arg(short, long, default_value = "solana")]
        chain: String,
    },

    /// 导出监控列表
    Export {
        /// 输出文件路径
        #[arg(short, long, default_value = "wallets.json")]
        output: String,

        /// 输出格式 (json, csv)
        #[arg(short, long, default_value = "json")]
        format: String,
    },

    /// 生成默认配置文件
    Config {
        /// 输出路径
        #[arg(short, long, default_value = "config.yaml")]
        output: String,
    },
}

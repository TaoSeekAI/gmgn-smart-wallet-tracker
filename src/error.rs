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

    #[error("序列化错误: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("数据库错误: {0}")]
    DatabaseError(#[from] sled::Error),

    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("钱包地址无效: {0}")]
    InvalidWalletAddress(String),

    #[error("代币地址无效: {0}")]
    InvalidTokenAddress(String),

    #[error("未找到数据: {0}")]
    NotFound(String),

    #[error("限流错误: API 请求过于频繁")]
    RateLimitExceeded,

    #[error("未知错误: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, TrackerError>;

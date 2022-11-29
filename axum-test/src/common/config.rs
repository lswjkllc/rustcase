//! 配置文件

use serde::Deserialize;

/// 公共配置
#[derive(Deserialize)]
pub struct CommonConfig {
    pub debug: bool,
}

/// Web 配置
#[derive(Deserialize)]
pub struct WebConfig {
    pub addr: String,
}

/// Log 配置
#[derive(Deserialize)]
pub struct LogConfig {
    pub path: String,
    pub level: String,
}

/// 应用配置
#[derive(Deserialize)]
pub struct Config {
    pub common: CommonConfig,
    pub web: WebConfig,
    pub log: LogConfig,
}

impl Config {
    /// 从环境变量中初始化配置
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
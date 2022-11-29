use super::config;

/// 应用共享
#[derive(Clone)]
pub struct Container {
    pub config: config::Config,
}
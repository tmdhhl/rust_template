pub mod configuration;
pub mod entity;
pub mod error;
pub mod middleware;
pub mod route;
pub mod startup;
pub mod telemetry;
pub mod util;

/// 支持的平台枚举
#[derive(Debug)]
pub enum Platform {
    /// 拼多多
    Pdd,
    /// 未知平台
    Unknown,
}

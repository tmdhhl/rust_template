use std::sync::Arc;

use async_trait::async_trait;
use axum::{
    Json,
    extract::{Query, State},
};
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::{
    Platform,
    error::{AppError, AppResult, TranslateError, TranslateResult},
    route::{AppState, translate::pdd::Pdd},
};

mod pdd;

/// 转链服务接口
#[async_trait]
pub trait Translate: Send + Sync {
    /// 搜索商品信息
    async fn search(&self, url: &str) -> TranslateResult<GoodInfo>;

    /// 生成短链接
    async fn gen_short_url(&self, url: &str) -> anyhow::Result<String>;
}

#[derive(Debug, Serialize, Default)]
pub struct GoodInfo {
    /// 活动佣金比例，千分比（特定活动期间的佣金比例）
    pub activity_promotion_rate: i64,
    /// 比价行为预判定佣金，需要用户备案
    pub predict_promotion_rate: i64,
    /// 佣金比例，千分比
    pub promotion_rate: i64,
    /// 优惠券面额，单位为分
    pub coupon_discount: i64,
    /// 优惠券后价格
    pub coupon_discount_price: i64,
    /// 原价
    pub origin_price: i64,
    /// 转链后短链
    pub short_url: String,
}

/// 转链请求参数
#[derive(Debug, Deserialize)]
pub struct TranslateLinkParams {
    url: String,
}

pub async fn translate_link(
    Query(query): Query<TranslateLinkParams>,
    State(state): State<AppState>,
) -> AppResult<Json<GoodInfo>> {
    let url = query.url.as_str();

    // 获取适合的转链器
    let translator = get_translator(url, &state)?;

    // 使用转链器搜索商品信息
    let mut good_info = translator
        .search(url)
        .await
        .map_err(|e| AppError::Unknown(format!("搜索商品失败: {}", e)))?;

    // 生成短链接
    good_info.short_url = translator.gen_short_url(url).await.map_err(|e| {
        warn!("生成短链接失败: {}", e);
        AppError::Unknown(e.to_string())
    })?;

    Ok(Json(good_info))
}

// 根据URL获取适合的转链器
fn get_translator(url: &str, state: &AppState) -> AppResult<Arc<dyn Translate>> {
    let platform = identify_platform(url).ok_or_else(|| {
        AppError::Translate(TranslateError::UnsupportedPlatform(
            "平台暂不支持".to_string(),
        ))
    })?;

    match platform {
        Platform::Pdd => {
            let settings = state.inner.lock().unwrap().app_settings.pdd.clone();
            Ok(Arc::new(Pdd::new(settings)))
        }
        // 后续可以添加其他平台支持
        Platform::Unknown => {
            warn!("未知平台");
            Err(AppError::Translate(TranslateError::UnsupportedPlatform(
                "未知平台".to_string(),
            )))
        }
    }
}

/// 识别链接所属平台
pub fn identify_platform(url: &str) -> Option<Platform> {
    if url.contains("pinduoduo.com") || url.contains("yangkeduo.com") || url.contains("pdd.com") {
        Some(Platform::Pdd)
    } else {
        None
    }
}

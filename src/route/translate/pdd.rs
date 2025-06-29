use std::collections::HashMap;

use anyhow::anyhow;
use async_trait::async_trait;
use axum::http::StatusCode;
use chrono::Utc;
use reqwest::Client;
use secrecy::ExposeSecret;
use serde::Deserialize;
use tracing::info;

use crate::{
    configuration::application::PddSettings,
    error::{TranslateError, TranslateResult},
    route::translate::{GoodInfo, Translate},
    util::generate_signature,
};

#[derive(Debug)]
pub struct Pdd {
    client: reqwest::Client,
    domain: String,
    client_id: String,
    client_secret: String,
    pid: String,
    api_good_search: String,
    api_gen_short_url: String,
}

impl Pdd {
    pub fn new(settings: PddSettings) -> Self {
        Self {
            client: Client::builder().build().unwrap(),
            domain: settings.domain,
            client_id: settings.client_id.expose_secret().to_string(),
            client_secret: settings.client_secret.expose_secret().to_string(),
            pid: settings.pid.expose_secret().to_string(),
            api_good_search: settings.api_good_search,
            api_gen_short_url: settings.api_gen_short_url,
        }
    }
}

/// 拼多多商品搜索响应
#[derive(Debug, Deserialize)]
pub struct PddGoodsSearchResponse {
    goods_search_response: GoodsSearchResponse,
}

/// 商品搜索响应内容
#[derive(Debug, Deserialize)]
pub struct GoodsSearchResponse {
    goods_list: Vec<GoodsItem>,
}

/// 商品项
#[derive(Debug, Deserialize)]
pub struct GoodsItem {
    promotion_rate: i64,
    predict_promotion_rate: i64,
    coupon_discount: i64,
    min_group_price: i64,
    // 其他字段可以根据需要添加
}

/// 拼多多短链生成响应
#[derive(Debug, Deserialize)]
pub struct PddGoodsZsUnitGenerateResponse {
    goods_zs_unit_generate_response: GoodsZsUnitGenerateResponse,
}

/// 短链生成响应内容
#[derive(Debug, Deserialize)]
pub struct GoodsZsUnitGenerateResponse {
    short_url: String,
}

impl From<&GoodsItem> for GoodInfo {
    fn from(item: &GoodsItem) -> Self {
        // 计算优惠券后价格 = 原价 - 优惠券面额
        let coupon_discount_price = item.min_group_price - item.coupon_discount;

        GoodInfo {
            promotion_rate: item.promotion_rate,
            predict_promotion_rate: item.predict_promotion_rate,
            coupon_discount: item.coupon_discount,
            coupon_discount_price,
            origin_price: item.min_group_price,
            // 其他字段设置为默认值
            activity_promotion_rate: 0,
            short_url: String::new(),
        }
    }
}
impl Pdd {
    async fn make_request<T: for<'de> serde::Deserialize<'de>>(
        &self,
        api_type: &str,
        params: HashMap<&str, &str>,
    ) -> TranslateResult<T> {
        let timestamp = Utc::now().timestamp().to_string();
        // 构建基础参数
        let mut body = HashMap::new();
        body.insert("client_id", self.client_id.as_str());
        body.insert("client_secret", self.client_secret.as_str());
        body.insert("type", api_type);
        body.insert("timestamp", &timestamp);
        body.insert("data_type", "JSON");

        // 添加额外参数
        for (key, value) in params {
            body.insert(key, value);
        }

        // 生成签名
        let sign = generate_signature(body.clone(), &self.client_secret);
        body.insert("sign", sign.as_str());

        // 发送请求
        let res = self
            .client
            .get(self.domain.as_str())
            .query(&body)
            .send()
            .await
            .map_err(|e| TranslateError::Request(e.to_string()))?;

        if res.status() != StatusCode::OK {
            return Err(TranslateError::Internal(format!(
                "Status code: {}",
                res.status()
            )));
        }

        // 解析响应
        let response = res
            .json::<T>()
            .await
            .map_err(|e| TranslateError::Internal(format!("解析响应失败: {}", e)))?;

        Ok(response)
    }
}

#[async_trait]
impl Translate for Pdd {
    async fn search(&self, url: &str) -> TranslateResult<GoodInfo> {
        let mut params = HashMap::new();
        params.insert("keyword", url);
        params.insert("pid", self.pid.as_str());

        let response: PddGoodsSearchResponse = self
            .make_request(self.api_good_search.as_str(), params)
            .await?;

        // 获取第一个商品
        let good = response
            .goods_search_response
            .goods_list
            .first()
            .ok_or_else(|| TranslateError::Internal("未找到商品".to_string()))?;

        Ok(good.into())
    }

    async fn gen_short_url(&self, url: &str) -> anyhow::Result<String> {
        info!("生成短链接: {}", url);

        let mut params = HashMap::new();
        params.insert("source_url", url);
        params.insert("pid", self.pid.as_str());

        // 使用通用请求方法
        let response: PddGoodsZsUnitGenerateResponse =
            match self.make_request(&self.api_gen_short_url, params).await {
                Ok(resp) => resp,
                Err(e) => return Err(anyhow!(e.to_string())),
            };

        Ok(response.goods_zs_unit_generate_response.short_url)
    }
}

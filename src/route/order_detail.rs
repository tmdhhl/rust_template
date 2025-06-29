use async_trait::async_trait;

use crate::error::TranslateResult;

#[async_trait]
pub trait OrderDetailRetriever: Send + Sync {
    /// 搜索商品信息
    async fn search(&self, url: &str) -> TranslateResult<OrderDetail>;
}

pub struct OrderDetailResponse {
    auth_duo_id: i64,      // 多多客工具id
    batch_no: String,      // 结算批次号
    fail_reason: String,   // 订单审核失败/惩罚原因
    order_status: i8, // 	订单状态：0-已支付；1-已成团；2-确认收货；3-审核成功；4-审核失败（不可提现）；5-已经结算；10-已处罚
    promotion_amount: i64, // 佣金
    promotion_rate: i64, // 佣金比例 千分比
}


pub struct OrderDetail {
    promotion_amount: i64,
    order_status: i8,

}
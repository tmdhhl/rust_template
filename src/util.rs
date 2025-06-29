use std::collections::HashMap;

/// 计算 API 签名
///
/// # Arguments
///
/// * `params` - 参数映射
/// * `client_secret` - 客户端密钥
///
/// # Returns
///
/// 计算得到的签名字符串
pub fn generate_signature(params: HashMap<&str, &str>, client_secret: &str) -> String {
    let mut values: Vec<String> = Vec::new();

    for (k, v) in params {
        values.push(format!("{}{}", k, v));
    }

    values.sort();
    let mut result = String::new();
    result.push_str(client_secret);
    result.push_str(values.join("").as_str());
    result.push_str(client_secret);

    // 计算 MD5 并转换为大写
    let hash = md5::compute(result);
    format!("{:X}", hash).to_uppercase()
}

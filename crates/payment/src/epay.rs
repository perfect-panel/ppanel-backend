use std::collections::BTreeMap;

use crate::error::PaymentError;
use crate::types::Cents;

#[derive(Debug, Clone)]
pub struct Config {
    pub pid: String,
    pub url: String,
    pub key: String,
    pub pay_type: String,
}

pub struct Order {
    pub name: String,
    pub order_no: String,
    pub amount: Cents,
    pub sign_type: String,
    pub notify_url: String,
    pub return_url: String,
}

pub struct Provider {
    pub pid: String,
    pub url: String,
    pub key: String,
    pub pay_type: String,
}

impl Provider {
    pub fn new(config: Config) -> Self {
        Provider {
            pid: config.pid,
            url: config.url,
            key: config.key,
            pay_type: config.pay_type,
        }
    }

    fn params_map(&self, order: &Order) -> BTreeMap<&str, String> {
        let mut m = BTreeMap::new();
        m.insert("pid", self.pid.clone());
        m.insert("type", self.pay_type.clone());
        m.insert("out_trade_no", order.order_no.clone());
        m.insert("money", order.amount.to_yuan_string());
        m.insert("name", order.name.clone());
        m.insert("notify_url", order.notify_url.clone());
        m.insert("return_url", order.return_url.clone());
        m
    }

    fn create_sign(&self, params: &BTreeMap<&str, String>) -> String {
        let query: String = params
            .iter()
            .filter(|(k, v)| !v.is_empty() && **k != "sign" && **k != "sign_type")
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("&");
        let text = format!("{}{}", query, self.key);
        format!("{:x}", md5::compute(text))
    }

    pub fn create_pay_url(&self, order: &Order) -> Result<String, PaymentError> {
        let params = self.params_map(order);
        let sign = self.create_sign(&params);

        let mut base_url = url::Url::parse(&self.url)
            .map_err(|_| PaymentError::Config("invalid EPay URL".into()))?;
        base_url = base_url
            .join("/submit.php")
            .map_err(|_| PaymentError::Config("invalid EPay path".into()))?;

        {
            let mut pairs = base_url.query_pairs_mut();
            for (k, v) in &params {
                pairs.append_pair(k, v);
            }
            pairs.append_pair("sign", &sign);
            pairs.append_pair("sign_type", "MD5");
        }

        Ok(base_url.to_string())
    }

    pub fn verify_sign(&self, params: &std::collections::HashMap<String, String>) -> bool {
        let mut sorted = BTreeMap::new();
        for (k, v) in params {
            sorted.insert(k.as_str(), v.clone());
        }
        let expected = params.get("sign").cloned().unwrap_or_default();
        self.create_sign(&sorted) == expected
    }

    pub async fn query_order_status(&self, order_no: &str) -> Result<bool, PaymentError> {
        let query_url = format!(
            "{}/api.php?act=order&pid={}&out_trade_no={}",
            self.url, self.pid, order_no
        );
        let resp = reqwest::get(&query_url).await?;
        let body: serde_json::Value = resp.json().await?;
        let status = body.get("status").and_then(|v| v.as_i64()).unwrap_or(0);
        Ok(status == 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epay_sign() {
        let provider = Provider::new(Config {
            pid: "1654".into(),
            url: "http://127.0.0.1".into(),
            key: "LbTabbB580zWyhXhyyww7wwvy5u8k0wl".into(),
            pay_type: "alipay".into(),
        });

        let order = Order {
            name: "product".into(),
            order_no: "202412152115078262977262254".into(),
            amount: Cents(1000),
            sign_type: "MD5".into(),
            notify_url: "".into(),
            return_url: "".into(),
        };

        let url = provider.create_pay_url(&order).unwrap();
        assert!(url.contains("sign="));
        assert!(url.contains("sign_type=MD5"));

        // Verify sign from callback params (matches Go test data)
        let params = std::collections::HashMap::from([
            ("pid".into(), "1654".into()),
            ("trade_no".into(), "2024121521150860990".into()),
            ("out_trade_no".into(), "202412152115078262977262254".into()),
            ("type".into(), "alipay".into()),
            ("name".into(), "product".into()),
            ("money".into(), "10".into()),
            ("trade_status".into(), "TRADE_SUCCESS".into()),
            ("sign".into(), "d3181f18ebdf9821f0ab6ee93faa82d1".into()),
            ("sign_type".into(), "MD5".into()),
        ]);
        assert!(provider.verify_sign(&params));
    }
}

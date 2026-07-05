use alipay_sdk_rust::{
    biz::{BizContenter, TradePrecreateBiz, TradeQueryBiz},
    pay::{Payer, PayClient},
};

use crate::error::PaymentError;
use crate::types::Cents;

#[derive(Debug, Clone)]
pub struct Config {
    pub app_id: String,
    pub private_key: String,
    pub public_key: String,
    pub invoice_name: String,
    pub notify_url: String,
    pub sandbox: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderStatus {
    Success,
    Pending,
    Closed,
    Finished,
    Error(String),
}

pub struct Notification {
    pub order_no: String,
    pub amount: Cents,
    pub status: OrderStatus,
}

pub struct Provider {
    client: Box<dyn Payer>,
    config: Config,
}

impl Provider {
    pub fn new(config: Config) -> Result<Self, PaymentError> {
        let api_url = if config.sandbox {
            "https://openapi-sandbox.dl.alipaydev.com/gateway.do"
        } else {
            "https://openapi.dl.alipaydev.com/gateway.do"
        };

        let app_id = config.app_id.clone();
        let private_key = config.private_key.clone();
        let public_key = config.public_key.clone();

        let client: Box<dyn Payer> = Box::new(
            PayClient::builder()
                .api_url(api_url)
                .app_id(&app_id)
                .private_key(&private_key)
                .public_key(&public_key)
                .sign_type_rsa2()
                .charset_utf8()
                .format_json()
                .version_1_0()
                .build()
                .map_err(|e| PaymentError::Config(format!("Alipay config error: {e}")))?,
        );

        Ok(Provider { client, config })
    }

    pub fn pre_create_trade(&self, order_no: &str, amount: Cents) -> Result<String, PaymentError> {
        let mut biz = TradePrecreateBiz::new();
        biz.set_out_trade_no(order_no.to_string().into());
        biz.set_total_amount(amount.to_yuan_string().into());
        biz.set_subject(self.config.invoice_name.clone().into());
        biz.set("notify_url", self.config.notify_url.clone().into());

        let resp = self
            .client
            .trade_precreate(&biz)
            .map_err(|e| PaymentError::Alipay(e.to_string()))?;

        if resp.response.code.as_deref() != Some("10000") {
            return Err(PaymentError::Alipay(
                resp.response
                    .sub_msg
                    .unwrap_or_else(|| "unknown alipay error".into()),
            ));
        }

        resp.response
            .qr_code
            .ok_or_else(|| PaymentError::Alipay("QR code not returned".into()))
    }

    pub fn query_trade(&self, order_no: &str) -> Result<OrderStatus, PaymentError> {
        let mut biz = TradeQueryBiz::new();
        biz.set_out_trade_no(order_no.to_string().into());

        let resp = self
            .client
            .trade_query(&biz)
            .map_err(|e| PaymentError::Alipay(e.to_string()))?;

        match resp.response.trade_status.as_deref() {
            Some("TRADE_SUCCESS") | Some("TRADE_FINISHED") => Ok(OrderStatus::Success),
            Some("WAIT_BUYER_PAY") => Ok(OrderStatus::Pending),
            Some("TRADE_CLOSED") => Ok(OrderStatus::Closed),
            Some(s) => Ok(OrderStatus::Error(s.into())),
            None => Ok(OrderStatus::Error("no trade status".into())),
        }
    }

    pub fn decode_notification(&self, body: &[u8]) -> Result<Notification, PaymentError> {
        let verified = self
            .client
            .async_verify_sign(body)
            .map_err(|e| PaymentError::Alipay(format!("notification verify failed: {e}")))?;

        if !verified {
            return Err(PaymentError::Alipay(
                "notification sign verification failed".into(),
            ));
        }

        let parsed: serde_json::Value = serde_json::from_slice(body)
            .map_err(|e| PaymentError::Alipay(format!("invalid notify body: {e}")))?;

        let response = parsed
            .as_object()
            .and_then(|m| m.values().next())
            .and_then(|v| v.as_object())
            .ok_or_else(|| PaymentError::Alipay("cannot parse notification".into()))?;

        let trade_status = response
            .get("trade_status")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let out_trade_no = response
            .get("out_trade_no")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let total_amount = response
            .get("total_amount")
            .and_then(|v| v.as_str())
            .unwrap_or("0");

        let status = match trade_status {
            "TRADE_SUCCESS" => OrderStatus::Success,
            "WAIT_BUYER_PAY" => OrderStatus::Pending,
            "TRADE_CLOSED" => OrderStatus::Closed,
            "TRADE_FINISHED" => OrderStatus::Finished,
            s => OrderStatus::Error(s.into()),
        };

        let amount = Cents::from_yuan(total_amount)
            .map_err(|e| PaymentError::Alipay(format!("Invalid amount: {e}")))?;

        Ok(Notification {
            order_no: out_trade_no.into(),
            amount,
            status,
        })
    }
}

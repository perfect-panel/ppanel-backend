use thiserror::Error;

#[derive(Error, Debug)]
pub enum PaymentError {
    #[error("Stripe error: {0}")]
    Stripe(#[from] stripe::StripeError),

    #[error("Stripe webhook error: {0}")]
    StripeWebhook(String),

    #[error("Alipay error: {0}")]
    Alipay(String),

    #[error("EPay error: {0}")]
    EPay(String),

    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Invalid configuration: {0}")]
    Config(String),
}

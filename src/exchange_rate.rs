//! Exchange rate conversion via apilayer.com.
//! Port of `server/pkg/exchangeRate/exchangeRate.go`.

use serde::Deserialize;

const API_BASE: &str = "https://api.apilayer.com";

#[derive(Debug, Deserialize)]
struct ConvertResponse {
    success: bool,
    result: Option<f64>,
}

/// Convert `amount` from currency `from` to currency `to`.
///
/// - `access_key` — API key from apilayer.com (configured in `Config.Currency.AccessKey`)
///
/// Returns the converted amount, or an error if the request fails or the API
/// reports failure.
pub async fn convert(
    from: &str,
    to: &str,
    amount: f64,
    access_key: &str,
) -> anyhow::Result<f64> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()?;

    let resp: ConvertResponse = client
        .get(format!("{API_BASE}/currency_data/convert"))
        .header("apikey", access_key)
        .query(&[
            ("from", from),
            ("to", to),
            ("amount", &amount.to_string()),
        ])
        .send()
        .await?
        .json()
        .await?;

    if !resp.success {
        anyhow::bail!("exchange rate API returned failure for {from}→{to}");
    }
    resp.result
        .ok_or_else(|| anyhow::anyhow!("exchange rate API returned no result"))
}

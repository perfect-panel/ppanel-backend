use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Div, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Cents(pub i64);

impl Cents {
    pub fn from_yuan(s: &str) -> Result<Self, std::num::ParseFloatError> {
        let yuan: f64 = s.parse()?;
        Ok(Cents((yuan * 100.0).round() as i64))
    }

    pub fn to_yuan_f64(&self) -> f64 {
        self.0 as f64 / 100.0
    }

    pub fn to_yuan_string(&self) -> String {
        format!("{:.2}", self.to_yuan_f64())
    }
}

impl fmt::Display for Cents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Mul<i64> for Cents {
    type Output = Cents;
    fn mul(self, rhs: i64) -> Cents {
        Cents(self.0 * rhs)
    }
}

impl Div<i64> for Cents {
    type Output = Cents;
    fn div(self, rhs: i64) -> Cents {
        Cents(self.0 / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cents_from_yuan() {
        let c = Cents::from_yuan("10.00").unwrap();
        assert_eq!(c.0, 1000);
        assert_eq!(c.to_yuan_string(), "10.00");
    }

    #[test]
    fn test_cents_from_yuan_rounding() {
        let c = Cents::from_yuan("9.99").unwrap();
        assert_eq!(c.0, 999);
        assert_eq!(c.to_yuan_string(), "9.99");
    }

    #[test]
    fn test_cents_display() {
        let c = Cents(100);
        assert_eq!(c.to_string(), "100");
        assert_eq!(c.to_yuan_f64(), 1.0);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub order_no: String,
    pub amount: Cents,
    pub currency: String,
    pub payment: String,
    pub subscribe: String,
    pub name: String,
    pub notify_url: String,
    pub return_url: String,
}

#[derive(Debug, Clone)]
pub struct User {
    pub user_id: i64,
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub event_type: String,
    pub order_no: String,
    pub trade_no: String,
    pub user_id: i64,
    pub amount: Cents,
    pub method: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PaymentSheet {
    pub client_secret: String,
    pub ephemeral_key: String,
    pub customer: String,
    pub publishable_key: String,
    pub trade_no: String,
}

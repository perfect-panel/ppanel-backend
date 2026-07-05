use std::collections::HashMap;
use std::str::FromStr;

use stripe::{
    Client as StripeClient, CreateCustomer, CreateEphemeralKey, CreatePaymentIntent,
    CreateWebhookEndpoint, Customer, CustomerSearchParams, EphemeralKey, EventFilter,
    EventObject, Expandable, PaymentIntent, PaymentIntentId, PaymentIntentStatus,
    PaymentMethod, PaymentMethodId, Webhook, WebhookEndpoint,
};

use crate::error::PaymentError;
use crate::types::{Cents, Notification, Order, PaymentSheet, User};

pub const API_VERSION: &str = "2024-04-10";

#[derive(Debug, Clone)]
pub struct Config {
    pub public_key: String,
    pub secret_key: String,
    pub webhook_secret: String,
}

pub struct Provider {
    client: StripeClient,
    config: Config,
}

impl Provider {
    pub fn new(config: Config) -> Self {
        let client = StripeClient::new(&config.secret_key);
        Provider { client, config }
    }

    pub async fn create_payment_sheet(
        &self,
        order: &Order,
        user: &User,
    ) -> Result<PaymentSheet, PaymentError> {
        let customer = self.find_or_create_customer(user).await?;

        let mut ek_params = CreateEphemeralKey::new();
        ek_params.customer = Some(customer.id.clone());
        let ek = EphemeralKey::create(&self.client, ek_params).await?;

        let mut metadata = HashMap::new();
        metadata.insert("order_no".to_string(), order.order_no.clone());
        metadata.insert("user_id".to_string(), user.user_id.to_string());
        metadata.insert("subscribe".to_string(), order.subscribe.clone());

        let currency = stripe::Currency::from_str(&order.currency)
            .map_err(|e| PaymentError::Config(format!("invalid currency: {e}")))?;

        let mut pi_params = CreatePaymentIntent::new(order.amount.0, currency);
        pi_params.customer = Some(customer.id.clone());
        pi_params.payment_method_types = Some(vec![order.payment.clone()]);
        pi_params.metadata = Some(metadata);

        let pi = PaymentIntent::create(&self.client, pi_params).await?;

        Ok(PaymentSheet {
            client_secret: pi.client_secret.unwrap_or_default(),
            ephemeral_key: ek.secret.unwrap_or_default(),
            customer: customer.id.to_string(),
            publishable_key: self.config.public_key.clone(),
            trade_no: pi.id.to_string(),
        })
    }

    pub async fn find_or_create_customer(&self, user: &User) -> Result<Customer, PaymentError> {
        if let Some(customer) = self.search_customer(user).await? {
            return Ok(customer);
        }
        self.create_customer(user).await
    }

    pub async fn search_customer(&self, user: &User) -> Result<Option<Customer>, PaymentError> {
        let query = if !user.email.is_empty() {
            format!("email:'{}'", user.email)
        } else {
            format!("metadata['user_id']:'{}'", user.user_id)
        };

        let mut params = CustomerSearchParams::new();
        params.query = query;

        let result = Customer::search(&self.client, params).await?;
        Ok(result.data.into_iter().next())
    }

    pub async fn create_customer(&self, user: &User) -> Result<Customer, PaymentError> {
        let mut params = CreateCustomer::new();
        if !user.email.is_empty() {
            params.email = Some(&user.email);
        }

        let mut metadata = HashMap::new();
        metadata.insert("user_id".to_string(), user.user_id.to_string());
        params.metadata = Some(metadata);

        Ok(Customer::create(&self.client, params).await?)
    }

    pub async fn query_order_status(&self, trade_no: &str) -> Result<bool, PaymentError> {
        let id = PaymentIntentId::from_str(trade_no)
            .map_err(|_| PaymentError::Config("invalid PaymentIntent ID".into()))?;
        let intent = PaymentIntent::retrieve(&self.client, &id, &[]).await?;
        Ok(intent.status == PaymentIntentStatus::Succeeded)
    }

    pub fn parse_notify(
        &self,
        payload: &[u8],
        signature: &str,
    ) -> Result<Notification, PaymentError> {
        let payload_str =
            std::str::from_utf8(payload).map_err(|e| PaymentError::StripeWebhook(e.to_string()))?;

        let event = Webhook::construct_event(payload_str, signature, &self.config.webhook_secret)
            .map_err(|e| PaymentError::StripeWebhook(e.to_string()))?;

        let pi = match event.data.object {
            EventObject::PaymentIntent(pi) => pi,
            _ => {
                return Err(PaymentError::StripeWebhook(
                    "unexpected event object type".into(),
                ))
            }
        };

        let order_no = pi.metadata.get("order_no").cloned().unwrap_or_default();
        let user_id: i64 = pi
            .metadata
            .get("user_id")
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);

        let method = match pi.payment_method {
            Some(Expandable::Object(ref pm)) => Some(pm.type_.to_string()),
            _ => None,
        };

        Ok(Notification {
            event_type: event.type_.to_string(),
            order_no,
            trade_no: pi.id.to_string(),
            user_id,
            amount: Cents(pi.amount),
            method,
        })
    }

    pub async fn retrieve_payment_method(
        &self,
        id: &str,
    ) -> Result<PaymentMethod, PaymentError> {
        let pm_id = PaymentMethodId::from_str(id)
            .map_err(|_| PaymentError::Config("invalid PaymentMethod ID".into()))?;
        Ok(PaymentMethod::retrieve(&self.client, &pm_id, &[]).await?)
    }

    pub async fn create_webhook_endpoint(
        &self,
        url: &str,
    ) -> Result<WebhookEndpoint, PaymentError> {
        let params = CreateWebhookEndpoint::new(
            vec![EventFilter::PaymentIntentSucceeded, EventFilter::PaymentIntentPaymentFailed],
            url,
        );
        Ok(WebhookEndpoint::create(&self.client, params).await?)
    }
}

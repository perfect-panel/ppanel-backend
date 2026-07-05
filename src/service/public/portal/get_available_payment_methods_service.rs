//! `GetAvailablePaymentMethods` — list payment methods visible to users.
//!
//! Port of the portal variant: returns only enabled payment methods,
//! stripping internal config/keys before sending to the client.

use std::sync::Arc;

use anyhow::anyhow;

use crate::model::dto::payment::{GetAvailablePaymentMethodsResponse, PaymentMethod};
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct GetAvailablePaymentMethodsService {
    repos: Arc<Repositories>,
}

impl GetAvailablePaymentMethodsService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn get(&self) -> Result<GetAvailablePaymentMethodsResponse, anyhow::Error> {
        let methods = self
            .repos
            .payment
            .find_available_methods()
            .await
            .map_err(|_| anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR)))?;

        let list = methods
            .into_iter()
            .map(|p| PaymentMethod {
                id: p.id,
                name: p.name,
                platform: p.platform,
                description: p.description.unwrap_or_default(),
                icon: p.icon,
                fee_mode: p.fee_mode as u32,
                fee_percent: p.fee_percent,
                fee_amount: p.fee_amount,
                sort: p.sort,
            })
            .collect();

        Ok(GetAvailablePaymentMethodsResponse { list })
    }
}

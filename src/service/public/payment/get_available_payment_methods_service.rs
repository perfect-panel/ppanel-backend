//! List enabled payment methods (public).

use std::sync::Arc;

use anyhow::anyhow;

use crate::model::entity::payment::Payment;
use crate::repository::Repositories;

pub struct GetAvailablePaymentMethodsService {
    pub repos: Arc<Repositories>,
}

impl GetAvailablePaymentMethodsService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn get_methods(&self) -> Result<Vec<Payment>, anyhow::Error> {
        self.repos
            .payment
            .find_available_methods()
            .await
            .map_err(|e| anyhow!("find available payment methods: {e}"))
    }
}

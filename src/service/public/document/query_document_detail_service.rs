//! Get document detail by id.

use std::sync::Arc;

use anyhow::anyhow;

use crate::model::entity::document::Document;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct QueryDocumentDetailService {
    pub repos: Arc<Repositories>,
}

impl QueryDocumentDetailService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn query_detail(&self, id: i64) -> Result<Document, anyhow::Error> {
        self.repos
            .document
            .query_detail(id)
            .await
            .map_err(|e| anyhow!("query document detail: {e}"))?
            .ok_or_else(|| anyhow!(CodeError::new_err_code(error_code::ERROR)))
    }
}

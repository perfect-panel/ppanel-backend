//! List published documents.

use std::sync::Arc;

use anyhow::anyhow;

use crate::model::entity::document::Document;
use crate::repository::Repositories;

pub struct QueryDocumentListService {
    pub repos: Arc<Repositories>,
}

impl QueryDocumentListService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn query_list(
        &self,
        page: i64,
        size: i64,
        tag: Option<&str>,
    ) -> Result<(i64, Vec<Document>), anyhow::Error> {
        self.repos
            .document
            .query_list(page, size, tag, None)
            .await
            .map_err(|e| anyhow!("query document list: {e}"))
    }
}

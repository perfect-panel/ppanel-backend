//! List enabled announcements.

use std::sync::Arc;

use anyhow::anyhow;

use crate::model::entity::announcement::Announcement;
use crate::repository::Repositories;

pub struct QueryAnnouncementService {
    pub repos: Arc<Repositories>,
}

impl QueryAnnouncementService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    /// Return all shown (enabled) announcements, newest first.
    pub async fn query_list(&self, page: i64, size: i64) -> Result<(i64, Vec<Announcement>), anyhow::Error> {
        self.repos
            .announcement
            .get_list_by_page(page, size, Some(true), None, None, None)
            .await
            .map_err(|e| anyhow!("query announcements: {e}"))
    }
}

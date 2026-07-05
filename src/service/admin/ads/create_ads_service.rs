use chrono::Utc;

use crate::model::dto::{Ads, CreateAdsRequest};
use crate::model::entity::ads::Ads as AdsEntity;
use crate::repository::ads::AdsRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn create_ads(
    repo: &dyn AdsRepo,
    req: CreateAdsRequest,
) -> Result<Ads, anyhow::Error> {
    let now = Utc::now().timestamp_millis();
    let entity = AdsEntity {
        id: 0,
        title: req.title,
        type_: req.type_,
        content: req.content,
        description: req.description,
        target_url: req.target_url,
        start_time: req.start_time,
        end_time: req.end_time,
        status: req.status,
        created_at: now,
        updated_at: now,
    };
    let result = repo
        .insert(&entity)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_INSERT_ERROR,
            &e.to_string(),
        )))?;

    Ok(Ads {
        id: result.id as i32,
        title: result.title,
        type_: result.type_,
        content: result.content,
        description: result.description,
        target_url: result.target_url,
        start_time: result.start_time,
        end_time: result.end_time,
        status: result.status,
        created_at: result.created_at,
        updated_at: result.updated_at,
    })
}

use chrono::Utc;

use crate::model::entity::ads::Ads as AdsEntity;
use crate::model::dto::UpdateAdsRequest;
use crate::repository::ads::AdsRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn update_ads(
    repo: &dyn AdsRepo,
    req: UpdateAdsRequest,
) -> Result<(), anyhow::Error> {
    let now = Utc::now().timestamp_millis();
    let entity = AdsEntity {
        id: req.id,
        title: req.title,
        type_: req.type_,
        content: req.content,
        description: req.description,
        target_url: req.target_url,
        start_time: req.start_time,
        end_time: req.end_time,
        status: req.status,
        created_at: 0,
        updated_at: now,
    };

    repo.update(&entity)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_UPDATE_ERROR,
            &e.to_string(),
        )))?;

    Ok(())
}

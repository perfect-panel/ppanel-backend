use crate::model::dto::Ads;
use crate::model::entity::ads::Ads as AdsEntity;
use crate::repository::ads::AdsRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn get_ads_detail(
    repo: &dyn AdsRepo,
    id: i64,
) -> Result<Ads, anyhow::Error> {
    let entity: AdsEntity = repo
        .find_one(id)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            &e.to_string(),
        )))?;

    Ok(Ads {
        id: entity.id as i32,
        title: entity.title,
        type_: entity.type_,
        content: entity.content,
        description: entity.description,
        target_url: entity.target_url,
        start_time: entity.start_time,
        end_time: entity.end_time,
        status: entity.status,
        created_at: entity.created_at,
        updated_at: entity.updated_at,
    })
}

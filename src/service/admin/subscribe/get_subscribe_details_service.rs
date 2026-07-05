use crate::model::dto::subscribe::{GetSubscribeDetailsRequest, Subscribe as SubscribeDto};
use crate::model::entity::subscribe::Subscribe;
use crate::repository::subscribe::SubscribeRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn get_subscribe_details(
    repo: &dyn SubscribeRepo,
    req: GetSubscribeDetailsRequest,
) -> Result<SubscribeDto, anyhow::Error> {
    let sub = repo
        .find_one(req.id)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            &e.to_string(),
        )))?;
    Ok(entity_to_dto(sub))
}

/// Public re-export so sibling service modules can reuse the entity→DTO mapping
/// (e.g. `get_subscribe_list_service`).
pub fn entity_to_dto_pub(s: Subscribe) -> SubscribeDto {
    entity_to_dto(s)
}

fn entity_to_dto(s: Subscribe) -> SubscribeDto {
    let discount: Vec<crate::model::dto::subscribe::SubscribeDiscount> =
        serde_json::from_str(&s.discount).unwrap_or_default();
    let nodes: crate::model::dto::misc::StringInt64Slice =
        serde_json::from_str(&s.nodes).unwrap_or_default();
    let node_tags: Vec<String> = if s.node_tags.is_empty() {
        Vec::new()
    } else {
        s.node_tags.split(',').map(|t| t.trim().to_string()).collect()
    };
    SubscribeDto {
        id: s.id,
        name: s.name,
        language: Some(s.language),
        description: s.description,
        unit_price: s.unit_price,
        unit_time: s.unit_time,
        discount,
        replacement: s.replacement,
        inventory: s.inventory,
        traffic: s.traffic,
        speed_limit: s.speed_limit,
        device_limit: s.device_limit,
        quota: s.quota,
        nodes,
        node_tags,
        show: s.show,
        sell: s.sell,
        sort: s.sort,
        deduction_ratio: s.deduction_ratio,
        allow_deduction: s.allow_deduction,
        reset_cycle: s.reset_cycle,
        renewal_reset: s.renewal_reset,
        show_original_price: s.show_original_price,
        created_at: s.created_at,
        updated_at: s.updated_at,
    }
}

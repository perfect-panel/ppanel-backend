use chrono::Utc;

use crate::model::dto::subscribe::UpdateSubscribeRequest;
use crate::model::entity::subscribe::Subscribe;
use crate::repository::subscribe::SubscribeRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn update_subscribe(
    repo: &dyn SubscribeRepo,
    req: UpdateSubscribeRequest,
) -> Result<Subscribe, anyhow::Error> {
    let existing = repo
        .find_one(req.id)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            &e.to_string(),
        )))?;
    let discount = serde_json::to_string(&req.discount)
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::INVALID_PARAMS,
            &format!("encode discount: {e}"),
        )))?;
    let nodes = serde_json::to_string(&req.nodes)
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::INVALID_PARAMS,
            &format!("encode nodes: {e}"),
        )))?;
    let node_tags = req.node_tags.join(",");
    let updated = Subscribe {
        id: existing.id,
        name: req.name,
        language: req.language.unwrap_or(existing.language),
        description: req.description.or(existing.description),
        unit_price: req.unit_price,
        unit_time: req.unit_time,
        discount,
        replacement: req.replacement,
        inventory: req.inventory,
        traffic: req.traffic,
        speed_limit: req.speed_limit,
        device_limit: req.device_limit,
        quota: req.quota,
        nodes,
        node_tags,
        show: req.show.unwrap_or(existing.show),
        sell: req.sell.unwrap_or(existing.sell),
        sort: req.sort,
        deduction_ratio: req.deduction_ratio,
        allow_deduction: req.allow_deduction.unwrap_or(existing.allow_deduction),
        reset_cycle: req.reset_cycle,
        renewal_reset: req.renewal_reset.unwrap_or(existing.renewal_reset),
        show_original_price: req.show_original_price,
        created_at: existing.created_at,
        updated_at: Utc::now().timestamp_millis(),
    };
    repo.update(&updated)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_UPDATE_ERROR,
            &e.to_string(),
        )))
}

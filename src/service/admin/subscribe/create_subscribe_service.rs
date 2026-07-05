use chrono::Utc;

use crate::model::dto::subscribe::CreateSubscribeRequest;
use crate::model::entity::subscribe::Subscribe;
use crate::repository::subscribe::SubscribeRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn create_subscribe(
    repo: &dyn SubscribeRepo,
    req: CreateSubscribeRequest,
) -> Result<Subscribe, anyhow::Error> {
    let now = Utc::now().timestamp_millis();
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
    let entity = Subscribe {
        id: 0,
        name: req.name,
        language: req.language.unwrap_or_default(),
        description: req.description,
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
        show: req.show.unwrap_or(false),
        sell: req.sell.unwrap_or(false),
        sort: 0,
        deduction_ratio: req.deduction_ratio,
        allow_deduction: req.allow_deduction.unwrap_or(true),
        reset_cycle: req.reset_cycle,
        renewal_reset: req.renewal_reset.unwrap_or(false),
        show_original_price: req.show_original_price,
        created_at: now,
        updated_at: now,
    };
    repo.insert(&entity)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_INSERT_ERROR,
            &e.to_string(),
        )))
}

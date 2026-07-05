use crate::model::dto::subscribe::SubscribeSortRequest;
use crate::model::entity::subscribe::Subscribe;
use crate::repository::subscribe::SubscribeRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn subscribe_sort(
    repo: &dyn SubscribeRepo,
    req: SubscribeSortRequest,
) -> Result<(), anyhow::Error> {
    let now = chrono::Utc::now().timestamp_millis();
    let items: Vec<Subscribe> = req
        .sort
        .into_iter()
        .map(|s| Subscribe {
            id: s.id,
            sort: s.sort,
            updated_at: now,
            ..placeholder_subscribe()
        })
        .collect();
    repo.update_sort(&items)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_UPDATE_ERROR,
            &e.to_string(),
        )))
}

fn placeholder_subscribe() -> Subscribe {
    Subscribe {
        id: 0,
        name: String::new(),
        language: String::new(),
        description: None,
        unit_price: 0,
        unit_time: String::new(),
        discount: "[]".to_string(),
        replacement: 0,
        inventory: 0,
        traffic: 0,
        speed_limit: 0,
        device_limit: 0,
        quota: 0,
        nodes: "[]".to_string(),
        node_tags: String::new(),
        show: false,
        sell: false,
        sort: 0,
        deduction_ratio: 0,
        allow_deduction: true,
        reset_cycle: 0,
        renewal_reset: false,
        show_original_price: false,
        created_at: 0,
        updated_at: 0,
    }
}

use std::sync::Arc;

use asynq::serve_mux::ServeMux;

use crate::config::Config;
use crate::repository::Repositories;

pub mod email;
pub mod order;
pub mod sms;
pub mod subscription;
pub mod task;
pub mod traffic;

pub fn register_all(repos: Arc<Repositories>, config: Arc<Config>) -> ServeMux {
    let mut mux = ServeMux::new();

    // ── Email ─────────────────────────────────────────────────────────────────
    let email_repos = Arc::clone(&repos);
    let email_config = Arc::clone(&config);
    mux.handle_async_func(crate::queue::types::FORTHWITH_SEND_EMAIL, move |task| {
        email::send_email(task, Arc::clone(&email_repos), Arc::clone(&email_config))
    });
    let batch_repos = Arc::clone(&repos);
    let batch_config = Arc::clone(&config);
    mux.handle_async_func(crate::queue::types::SCHEDULED_BATCH_SEND_EMAIL, move |task| {
        email::batch_email(task, Arc::clone(&batch_repos), Arc::clone(&batch_config))
    });

    // ── SMS ───────────────────────────────────────────────────────────────────
    let sms_repos = Arc::clone(&repos);
    let sms_config = Arc::clone(&config);
    mux.handle_async_func(crate::queue::types::FORTHWITH_SEND_SMS, move |task| {
        sms::send_sms(task, Arc::clone(&sms_repos), Arc::clone(&sms_config))
    });

    // ── Order ─────────────────────────────────────────────────────────────────
    let activate_repos = Arc::clone(&repos);
    let activate_config = Arc::clone(&config);
    mux.handle_async_func(crate::queue::types::FORTHWITH_ACTIVATE_ORDER, move |task| {
        order::activate_order(task, Arc::clone(&activate_repos), Arc::clone(&activate_config))
    });
    let close_repos = Arc::clone(&repos);
    mux.handle_async_func(crate::queue::types::DEFER_CLOSE_ORDER, move |task| {
        order::defer_close_order(task, Arc::clone(&close_repos))
    });

    // ── Traffic ───────────────────────────────────────────────────────────────
    mux.handle_func(
        crate::queue::types::FORTHWITH_TRAFFIC_STATISTICS,
        traffic::stub_traffic_statistics,
    );
    mux.handle_func(
        crate::queue::types::SCHEDULER_TOTAL_SERVER_DATA,
        traffic::stub_server_data,
    );
    mux.handle_func(
        crate::queue::types::SCHEDULER_RESET_TRAFFIC,
        traffic::stub_reset_traffic,
    );
    mux.handle_func(
        crate::queue::types::SCHEDULER_TRAFFIC_STAT,
        traffic::stub_traffic_stat,
    );

    // ── Subscription ──────────────────────────────────────────────────────────
    let sub_repos = Arc::clone(&repos);
    let sub_config = Arc::clone(&config);
    mux.handle_async_func(
        crate::queue::types::SCHEDULER_CHECK_SUBSCRIPTION,
        move |task| {
            subscription::check_subscription(
                task,
                Arc::clone(&sub_repos),
                Arc::clone(&sub_config),
            )
        },
    );

    // ── Quota task + Rate task ─────────────────────────────────────────────────
    let quota_repos = Arc::clone(&repos);
    let quota_config = Arc::clone(&config);
    mux.handle_async_func(crate::queue::types::FORTHWITH_QUOTA_TASK, move |t| {
        task::quota_task(t, Arc::clone(&quota_repos), Arc::clone(&quota_config))
    });

    let rate_repos = Arc::clone(&repos);
    let rate_config = Arc::clone(&config);
    mux.handle_async_func(crate::queue::types::FORTHWITH_RATE_TASK, move |t| {
        task::rate_task(t, Arc::clone(&rate_repos), Arc::clone(&rate_config))
    });

    mux
}

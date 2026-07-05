// Routes — all HTTP routes registered with middleware applied per group.
//
// Middleware mapping (mirrors Go):
//   /v1/auth/*        — device_middleware only (public auth routes)
//   /v1/auth/oauth/*  — no middleware
//   /v1/admin/*       — auth_middleware
//   /v1/public/*      — auth_middleware + device_middleware
//   /v1/common/*      — device_middleware
//   /v1/server/*      — server_middleware (TODO)
//   notify / subscribe / telegram — no user auth
use axum::{
    Router,
    middleware,
    routing::{delete, get, post, put},
};
use tower_http::trace::TraceLayer;

use crate::handler::AppState;
use crate::middleware::auth_middleware::auth_middleware;
use crate::middleware::device_middleware::device_middleware;
use crate::middleware::logger_middleware::{RequestLog, RequestSpan};

use crate::handler::admin::ads::*;
use crate::handler::admin::announcement::*;
use crate::handler::admin::application::*;
use crate::handler::admin::auth_method::*;
use crate::handler::admin::console::*;
use crate::handler::admin::coupon::*;
use crate::handler::admin::document::*;
use crate::handler::admin::log::*;
use crate::handler::admin::marketing::*;
use crate::handler::admin::order::*;
use crate::handler::admin::payment::*;
use crate::handler::admin::plugin::*;
use crate::handler::admin::server::*;
use crate::handler::admin::subscribe::*;
use crate::handler::admin::system::*;
use crate::handler::admin::ticket::*;
use crate::handler::admin::tool::*;
use crate::handler::admin::user::*;

use crate::handler::public::announcement::*;
use crate::handler::public::document::*;
use crate::handler::public::subscribe::*;
use crate::handler::public::ticket::*;
use crate::handler::public::order::*;
use crate::handler::public::payment::*;
use crate::handler::public::portal::{
    purchase_checkout,
    query_purchase_order,
    get_available_payment_methods as portal_get_available_payment_methods,
    pre_purchase_order,
    purchase as portal_purchase,
    get_subscription,
};
use crate::handler::public::user::{
    bind_o_auth_callback, bind_o_auth, bind_telegram, commission_withdraw,
    get_device_list, get_login_log, get_o_auth_methods, get_subscribe_log,
    pre_unsubscribe, query_user_affiliate, query_user_affiliate_list,
    query_user_balance_log, query_user_commission_log, query_user_info,
    query_user_subscribe, query_withdrawal_log,
    reset_user_subscribe_token as public_reset_user_subscribe_token,
    unbind_device, unbind_o_auth, unbind_telegram, unsubscribe,
    update_bind_email, update_bind_mobile, update_user_notify,
    update_user_password, update_user_rules, update_user_subscribe_note,
    verify_email,
};

use crate::handler::auth::*;
use crate::handler::auth::oauth::*;

use crate::handler::common::*;
use crate::handler::server::*;
use crate::handler::subscribe::*;
use crate::handler::telegram::*;

pub fn register_routes(state: AppState) -> Router<()> {
    // ── OAuth (no middleware) ────────────────────────────────────────────
    let oauth_routes = Router::new()
        .route("/v1/auth/oauth/callback/apple", post(apple_login_callback))
        .route("/v1/auth/oauth/login", post(o_auth_login))
        .route("/v1/auth/oauth/login/token", post(o_auth_login_get_token));

    // ── Auth routes (device_middleware only — public) ────────────────────
    let auth_routes = Router::new()
        .route("/v1/auth/check", get(check_user))
        .route("/v1/auth/check/telephone", get(check_user_telephone))
        .route("/v1/auth/login", post(user_login))
        .route("/v1/auth/login/device", post(device_login))
        .route("/v1/auth/login/telephone", post(telephone_login))
        .route("/v1/auth/register", post(user_register))
        .route("/v1/auth/register/telephone", post(telephone_user_register))
        .route("/v1/auth/reset", post(reset_password))
        .route("/v1/auth/reset/telephone", post(telephone_reset_password))
        .layer(middleware::from_fn(device_middleware));

    // ── Admin routes (auth_middleware) ───────────────────────────────────
    let admin_routes = Router::new()
        // Ads
        .route("/v1/admin/ads", post(create_ads).put(update_ads).delete(delete_ads))
        .route("/v1/admin/ads/detail", get(get_ads_detail))
        .route("/v1/admin/ads/list", get(get_ads_list))
        // Announcement
        .route("/v1/admin/announcement", post(create_announcement).put(update_announcement).delete(delete_announcement))
        .route("/v1/admin/announcement/detail", get(get_announcement))
        .route("/v1/admin/announcement/list", get(get_announcement_list))
        // Application
        .route("/v1/admin/application", post(create_subscribe_application))
        .route("/v1/admin/application/preview", get(preview_subscribe_template))
        .route("/v1/admin/application/subscribe_application", put(update_subscribe_application).delete(delete_subscribe_application))
        .route("/v1/admin/application/subscribe_application_list", get(get_subscribe_application_list))
        // Auth-method
        .route("/v1/admin/auth-method/config", get(get_auth_method_config).put(update_auth_method_config))
        .route("/v1/admin/auth-method/email_platform", get(get_email_platform))
        .route("/v1/admin/auth-method/list", get(get_auth_method_list))
        .route("/v1/admin/auth-method/sms_platform", get(get_sms_platform))
        .route("/v1/admin/auth-method/test_email_send", post(test_email_send))
        .route("/v1/admin/auth-method/test_sms_send", post(test_sms_send))
        // Console
        .route("/v1/admin/console/revenue", get(query_revenue_statistics))
        .route("/v1/admin/console/server", get(query_server_total_data))
        .route("/v1/admin/console/ticket", get(query_ticket_wait_reply))
        .route("/v1/admin/console/user", get(query_user_statistics))
        // Coupon
        .route("/v1/admin/coupon", post(create_coupon).put(update_coupon).delete(delete_coupon))
        .route("/v1/admin/coupon/batch", delete(batch_delete_coupon))
        .route("/v1/admin/coupon/list", get(get_coupon_list))
        // Document
        .route("/v1/admin/document", post(create_document).put(update_document).delete(delete_document))
        .route("/v1/admin/document/batch", delete(batch_delete_document))
        .route("/v1/admin/document/detail", get(get_document_detail))
        .route("/v1/admin/document/list", get(get_document_list))
        // Log
        .route("/v1/admin/log/balance/list", get(filter_balance_log))
        .route("/v1/admin/log/commission/list", get(filter_commission_log))
        .route("/v1/admin/log/email/list", get(filter_email_log))
        .route("/v1/admin/log/gift/list", get(filter_gift_log))
        .route("/v1/admin/log/login/list", get(filter_login_log))
        .route("/v1/admin/log/message/list", get(get_message_log_list))
        .route("/v1/admin/log/mobile/list", get(filter_mobile_log))
        .route("/v1/admin/log/register/list", get(filter_register_log))
        .route("/v1/admin/log/server/traffic/list", get(filter_server_traffic_log))
        .route("/v1/admin/log/setting", get(get_log_setting).post(update_log_setting))
        .route("/v1/admin/log/subscribe/list", get(filter_subscribe_log))
        .route("/v1/admin/log/subscribe/reset/list", get(filter_reset_subscribe_log))
        .route("/v1/admin/log/subscribe/traffic/list", get(filter_user_subscribe_traffic_log))
        .route("/v1/admin/log/traffic/details", get(filter_traffic_log_details))
        // Marketing
        .route("/v1/admin/marketing/email/batch/list", get(get_batch_send_email_task_list))
        .route("/v1/admin/marketing/email/batch/pre-send-count", post(get_pre_send_email_count))
        .route("/v1/admin/marketing/email/batch/send", post(create_batch_send_email_task))
        .route("/v1/admin/marketing/email/batch/status", post(get_batch_send_email_task_status))
        .route("/v1/admin/marketing/email/batch/stop", post(stop_batch_send_email_task))
        .route("/v1/admin/marketing/quota/create", post(create_quota_task))
        .route("/v1/admin/marketing/quota/list", get(query_quota_task_list))
        .route("/v1/admin/marketing/quota/pre-count", post(query_quota_task_pre_count))
        .route("/v1/admin/marketing/quota/status", post(query_quota_task_status))
        // Order
        .route("/v1/admin/order", post(create_order))
        .route("/v1/admin/order/list", get(get_order_list))
        .route("/v1/admin/order/status", put(update_order_status))
        // Payment
        .route("/v1/admin/payment", post(create_payment_method).put(update_payment_method).delete(delete_payment_method))
        .route("/v1/admin/payment/list", get(get_payment_method_list))
        .route("/v1/admin/payment/platform", get(get_payment_platform))
        // Server
        .route("/v1/admin/server/create", post(create_server))
        .route("/v1/admin/server/delete", post(delete_server))
        .route("/v1/admin/server/list", get(filter_server_list))
        .route("/v1/admin/server/node/create", post(create_node))
        .route("/v1/admin/server/node/delete", post(delete_node))
        .route("/v1/admin/server/node/list", get(filter_node_list))
        .route("/v1/admin/server/node/sort", post(reset_sort_with_node))
        .route("/v1/admin/server/node/status/toggle", post(toggle_node_status))
        .route("/v1/admin/server/node/tags", get(query_node_tag))
        .route("/v1/admin/server/node_config", get(get_server_node_config))
        .route("/v1/admin/server/node_config/update", post(update_server_node_config))
        .route("/v1/admin/server/node/update", post(update_node))
        .route("/v1/admin/server/protocols", get(get_server_protocols))
        .route("/v1/admin/server/server/sort", post(reset_sort_with_server))
        .route("/v1/admin/server/update", post(update_server))
        // Subscribe
        .route("/v1/admin/subscribe", post(create_subscribe).put(update_subscribe).delete(delete_subscribe))
        .route("/v1/admin/subscribe/batch", delete(batch_delete_subscribe))
        .route("/v1/admin/subscribe/details", get(get_subscribe_details))
        .route("/v1/admin/subscribe/group", post(create_subscribe_group).put(update_subscribe_group).delete(delete_subscribe_group))
        .route("/v1/admin/subscribe/group/batch", delete(batch_delete_subscribe_group))
        .route("/v1/admin/subscribe/group/list", get(get_subscribe_group_list))
        .route("/v1/admin/subscribe/list", get(get_subscribe_list))
        .route("/v1/admin/subscribe/reset_all_token", post(reset_all_subscribe_token))
        .route("/v1/admin/subscribe/sort", post(subscribe_sort))
        // System
        .route("/v1/admin/system/currency_config", get(get_currency_config).put(update_currency_config))
        .route("/v1/admin/system/get_node_multiplier", get(get_node_multiplier))
        .route("/v1/admin/system/invite_config", get(get_invite_config).put(update_invite_config))
        .route("/v1/admin/system/module", get(get_module_config))
        .route("/v1/admin/system/node_config", get(get_node_config).put(update_node_config))
        .route("/v1/admin/system/node_multiplier/preview", get(pre_view_node_multiplier))
        .route("/v1/admin/system/privacy", get(get_privacy_policy_config).put(update_privacy_policy_config))
        .route("/v1/admin/system/register_config", get(get_register_config).put(update_register_config))
        .route("/v1/admin/system/set_node_multiplier", post(set_node_multiplier))
        .route("/v1/admin/system/setting_telegram_bot", post(setting_telegram_bot_handler))
        .route("/v1/admin/system/site_config", get(get_site_config).put(update_site_config))
        .route("/v1/admin/system/subscribe_config", get(get_subscribe_config).put(update_subscribe_config))
        .route("/v1/admin/system/tos_config", get(get_tos_config).put(update_tos_config))
        .route("/v1/admin/system/verify_code_config", get(get_verify_code_config).put(update_verify_code_config))
        .route("/v1/admin/system/verify_config", get(get_verify_config).put(update_verify_config))
        // Ticket
        .route("/v1/admin/ticket", put(update_ticket_status))
        .route("/v1/admin/ticket/detail", get(get_ticket))
        .route("/v1/admin/ticket/follow", post(create_ticket_follow))
        .route("/v1/admin/ticket/list", get(get_ticket_list))
        // Tool
        .route("/v1/admin/tool/ip/location", get(query_ip_location))
        .route("/v1/admin/tool/log", get(get_system_log))
        .route("/v1/admin/tool/restart", get(restart_system))
        .route("/v1/admin/tool/version", get(get_version))
        // User
        .route("/v1/admin/user", post(create_user).delete(delete_user))
        .route("/v1/admin/user/auth_method", get(get_user_auth_method).post(create_user_auth_method).put(update_user_auth_method).delete(delete_user_auth_method))
        .route("/v1/admin/user/basic", put(update_user_basic_info))
        .route("/v1/admin/user/batch", delete(batch_delete_user))
        .route("/v1/admin/user/current", get(current_user))
        .route("/v1/admin/user/detail", get(get_user_detail))
        .route("/v1/admin/user/device", put(update_user_device).delete(delete_user_device))
        .route("/v1/admin/user/device/kick_offline", put(kick_offline_by_user_device))
        .route("/v1/admin/user/list", get(get_user_list))
        .route("/v1/admin/user/login/logs", get(get_user_login_logs))
        .route("/v1/admin/user/notify", put(update_user_notify_setting))
        .route("/v1/admin/user/subscribe", get(get_user_subscribe).post(create_user_subscribe).put(update_user_subscribe).delete(delete_user_subscribe))
        .route("/v1/admin/user/subscribe/detail", get(get_user_subscribe_by_id))
        .route("/v1/admin/user/subscribe/device", get(get_user_subscribe_devices))
        .route("/v1/admin/user/subscribe/logs", get(get_user_subscribe_logs))
        .route("/v1/admin/user/subscribe/reset/logs", get(get_user_subscribe_reset_traffic_logs))
        .route("/v1/admin/user/subscribe/reset/token", post(reset_user_subscribe_token))
        .route("/v1/admin/user/subscribe/reset/traffic", post(reset_user_subscribe_traffic))
        .route("/v1/admin/user/subscribe/toggle", post(toggle_user_subscribe_status))
        .route("/v1/admin/user/subscribe/traffic_logs", get(get_user_subscribe_traffic_logs))
        // Plugin
        .route("/v1/admin/plugin/list", get(list))
        .route("/v1/admin/plugin/detail", get(detail))
        .route("/v1/admin/plugin/reload", post(reload_handler))
        .route("/v1/admin/plugin/enable", post(enable_handler))
        .route("/v1/admin/plugin/disable", post(disable_handler))
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    // ── Public routes (auth_middleware + device_middleware) ──────────────
    let public_routes = Router::new()
        .route("/v1/public/announcement/list", get(query_announcement))
        .route("/v1/public/document/detail", get(query_document_detail))
        .route("/v1/public/document/list", get(query_document_list))
        .route("/v1/public/order/close", post(close_order))
        .route("/v1/public/order/detail", get(query_order_detail))
        .route("/v1/public/order/list", get(query_order_list))
        .route("/v1/public/order/pre", post(pre_create_order))
        .route("/v1/public/order/purchase", post(purchase))
        .route("/v1/public/order/recharge", post(recharge))
        .route("/v1/public/order/renewal", post(renewal))
        .route("/v1/public/order/reset", post(reset_traffic))
        .route("/v1/public/payment/methods", get(get_available_payment_methods))
        .route("/v1/public/portal/order/checkout", post(purchase_checkout))
        .route("/v1/public/portal/order/status", get(query_purchase_order))
        .route("/v1/public/portal/payment-method", get(portal_get_available_payment_methods))
        .route("/v1/public/portal/pre", post(pre_purchase_order))
        .route("/v1/public/portal/purchase", post(portal_purchase))
        .route("/v1/public/portal/subscribe", get(get_subscription))
        .route("/v1/public/subscribe/list", get(query_subscribe_list))
        .route("/v1/public/subscribe/node/list", get(query_user_subscribe_node_list))
        .route("/v1/public/subscribe/group/list", get(query_subscribe_group_list))
        .route("/v1/public/ticket", put(update_user_ticket_status).post(create_user_ticket))
        .route("/v1/public/ticket/detail", get(get_user_ticket_details))
        .route("/v1/public/ticket/follow", post(create_user_ticket_follow))
        .route("/v1/public/ticket/list", get(get_user_ticket_list))
        .route("/v1/public/user/affiliate/count", get(query_user_affiliate))
        .route("/v1/public/user/affiliate/list", get(query_user_affiliate_list))
        .route("/v1/public/user/balance_log", get(query_user_balance_log))
        .route("/v1/public/user/bind_email", put(update_bind_email))
        .route("/v1/public/user/bind_mobile", put(update_bind_mobile))
        .route("/v1/public/user/bind_oauth", post(bind_o_auth))
        .route("/v1/public/user/bind_oauth/callback", post(bind_o_auth_callback))
        .route("/v1/public/user/bind_telegram", get(bind_telegram))
        .route("/v1/public/user/commission_log", get(query_user_commission_log))
        .route("/v1/public/user/commission_withdraw", post(commission_withdraw))
        .route("/v1/public/user/devices", get(get_device_list))
        .route("/v1/public/user/info", get(query_user_info))
        .route("/v1/public/user/login_log", get(get_login_log))
        .route("/v1/public/user/notify", put(update_user_notify))
        .route("/v1/public/user/oauth_methods", get(get_o_auth_methods))
        .route("/v1/public/user/password", put(update_user_password))
        .route("/v1/public/user/rules", put(update_user_rules))
        .route("/v1/public/user/subscribe", get(query_user_subscribe))
        .route("/v1/public/user/subscribe_log", get(get_subscribe_log))
        .route("/v1/public/user/subscribe_note", put(update_user_subscribe_note))
        .route("/v1/public/user/subscribe_token", put(public_reset_user_subscribe_token))
        .route("/v1/public/user/unbind_device", put(unbind_device))
        .route("/v1/public/user/unbind_oauth", post(unbind_o_auth))
        .route("/v1/public/user/unbind_telegram", post(unbind_telegram))
        .route("/v1/public/user/unsubscribe", post(unsubscribe))
        .route("/v1/public/user/unsubscribe/pre", post(pre_unsubscribe))
        .route("/v1/public/user/verify_email", post(verify_email))
        .route("/v1/public/user/withdrawal_log", get(query_withdrawal_log))
        // device_middleware first, then auth_middleware (layers apply bottom-up in axum)
        .layer(middleware::from_fn(device_middleware))
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    // ── Common routes (device_middleware only) ───────────────────────────
    let common_routes = Router::new()
        .route("/v1/common/ads", get(get_ads))
        .route("/v1/common/check_verification_code", post(check_verification_code))
        .route("/v1/common/client", get(get_client))
        .route("/v1/common/heartbeat", get(heartbeat))
        .route("/v1/common/send_code", post(send_email_code))
        .route("/v1/common/send_sms_code", post(send_sms_code))
        .route("/v1/common/site/config", get(get_global_config))
        .route("/v1/common/site/privacy", get(get_privacy_policy))
        .route("/v1/common/site/stat", get(get_stat))
        .route("/v1/common/site/tos", get(get_tos))
        .layer(middleware::from_fn(device_middleware));

    // ── Server / notify / subscribe / telegram (no user auth) ────────────
    let other_routes = Router::new()
        .route("/v1/server/config", get(get_server_config))
        .route("/v1/server/online", post(push_online_users))
        .route("/v1/server/push", post(server_push_user_traffic))
        .route("/v1/server/status", post(server_push_status))
        .route("/v1/server/user", get(get_server_user_list))
        .route("/v2/server/{server_id}", get(query_server_protocol_config))
        .route("/v1/subscribe/config", get(subscribe_handler))
        .route("/", get(pan_domain_subscribe_handler))
        .route("/v1/telegram", post(telegram_handler));

    Router::new()
        .merge(oauth_routes)
        .merge(auth_routes)
        .merge(admin_routes)
        .merge(public_routes)
        .merge(common_routes)
        .merge(other_routes)
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(RequestSpan)
                .on_response(RequestLog),
        )
}

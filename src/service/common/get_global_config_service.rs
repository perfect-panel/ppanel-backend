use crate::config::Config;
use crate::model::dto::auth::{AuthConfig, DeviceAuthticateConfig, EmailAuthticateConfig, MobileAuthenticateConfig};
use crate::model::dto::common::GetGlobalConfigResponse;
use crate::model::dto::subscribe::SubscribeConfig;
use crate::model::dto::system::{Currency, InviteConfig, PubilcRegisterConfig, PubilcVerifyCodeConfig, SiteConfig, VeifyConfig};
use crate::repository::Repositories;
use anyhow::anyhow;
use result::code_error::CodeError;
use result::error_code;

pub async fn get_global_config(
    repos: &Repositories,
    config: &Config,
) -> anyhow::Result<GetGlobalConfigResponse> {
    let auth_methods = repos
        .auth
        .find_all_enabled()
        .await
        .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;

    let oauth_methods: Vec<String> = auth_methods.iter().map(|m| m.method.clone()).collect();

    let web_ad = repos
        .system
        .find_one_by_category_key("site", "WebAD")
        .await
        .map(|s| s.value == "true")
        .unwrap_or(false);

    let site = SiteConfig {
        host: config.site.host.clone(),
        site_name: config.site.site_name.clone(),
        site_desc: config.site.site_desc.clone(),
        site_logo: config.site.site_logo.clone(),
        keywords: config.site.keywords.clone(),
        custom_html: config.site.custom_html.clone(),
        custom_data: config.site.custom_data.clone(),
    };

    let verify = VeifyConfig {
        turnstile_site_key: config.verify.turnstile_site_key.clone(),
        enable_login_verify: config.verify.login_verify,
        enable_register_verify: config.verify.register_verify,
        enable_reset_password_verify: config.verify.reset_password_verify,
    };

    let auth = AuthConfig {
        mobile: MobileAuthenticateConfig {
            enable: config.mobile.enable,
            enable_whitelist: config.mobile.enable_whitelist,
            whitelist: config.mobile.whitelist.clone(),
        },
        email: EmailAuthticateConfig {
            enable: config.email.enable,
            enable_verify: config.email.enable_verify,
            enable_domain_suffix: config.email.enable_domain_suffix,
            domain_suffix_list: config.email.domain_suffix_list.clone(),
        },
        device: DeviceAuthticateConfig {
            enable: config.device.enable,
            show_ads: config.device.show_ads,
            enable_security: config.device.enable_security,
            only_real_device: config.device.only_real_device,
        },
        register: PubilcRegisterConfig {
            stop_register: config.register.stop_register,
            enable_ip_register_limit: config.register.enable_ip_register_limit,
            ip_register_limit: config.register.ip_register_limit,
            ip_register_limit_duration: config.register.ip_register_limit_duration,
        },
    };

    let invite = InviteConfig {
        forced_invite: config.invite.forced_invite,
        referral_percentage: config.invite.referral_percentage,
        only_first_purchase: config.invite.only_first_purchase,
    };

    let currency = Currency {
        currency_unit: config.currency.unit.clone(),
        currency_symbol: config.currency.symbol.clone(),
    };

    let subscribe = SubscribeConfig {
        single_model: config.subscribe.single_model,
        subscribe_path: config.subscribe.subscribe_path.clone(),
        subscribe_domain: config.subscribe.subscribe_domain.clone(),
        pan_domain: config.subscribe.pan_domain,
        user_agent_limit: config.subscribe.user_agent_limit,
        user_agent_list: config.subscribe.user_agent_list.clone(),
        show_tutorial: config.subscribe.show_tutorial,
    };

    let verify_code = PubilcVerifyCodeConfig {
        verify_code_interval: config.verify_code.interval,
    };

    Ok(GetGlobalConfigResponse {
        site,
        verify,
        auth,
        invite,
        currency,
        subscribe,
        verify_code,
        oauth_methods,
        web_ad,
    })
}

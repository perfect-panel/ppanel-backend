use crate::config::Config;
use crate::model::dto::log::LogSetting;

pub async fn get_log_setting(config: &Config) -> anyhow::Result<LogSetting> {
    Ok(LogSetting {
        auto_clear: Some(config.log.auto_clear),
        clear_days: config.log.clear_days,
    })
}

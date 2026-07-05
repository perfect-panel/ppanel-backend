use crate::config::SmsConfig;
use crate::platform::Platform;
use crate::providers::{abosend::AbosendSender, alibabacloud::AlibabaCloudSender, smsbao::SmsbaoSender, twilio::TwilioSender};
use crate::sender::Sender;

pub fn create_sender(platform: Platform, config: SmsConfig) -> Box<dyn Sender> {
    match platform {
        Platform::AlibabaCloud => Box::new(AlibabaCloudSender::new(config)),
        Platform::Smsbao => Box::new(SmsbaoSender::new(config)),
        Platform::Abosend => Box::new(AbosendSender::new(config)),
        Platform::Twilio => Box::new(TwilioSender::new(config)),
    }
}

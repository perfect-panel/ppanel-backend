use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use serde::Deserialize;

use crate::sender::EmailError;

#[derive(Debug, Clone, Deserialize)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub pass: String,
    pub from: String,
    pub reply_to: Option<String>,
    pub ssl: bool,
    #[serde(default)]
    pub site_name: String,
}

pub struct SmtpClient {
    config: SmtpConfig,
    mailer: AsyncSmtpTransport<Tokio1Executor>,
}

impl SmtpClient {
    pub fn new(config: SmtpConfig) -> Self {
        let creds = Credentials::new(config.user.clone(), config.pass.clone());

        let tls_params = TlsParameters::new(config.host.clone())
            .expect("failed to build TLS parameters");

        let tls = if config.ssl {
            Tls::Wrapper(tls_params)
        } else {
            Tls::Required(tls_params)
        };

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.host)
            .expect("failed to build SMTP relay")
            .port(config.port)
            .credentials(creds)
            .tls(tls)
            .build();

        SmtpClient { config, mailer }
    }
}

#[async_trait::async_trait]
impl crate::sender::Sender for SmtpClient {
    async fn send(&self, to: &[String], subject: &str, body: &str) -> Result<(), EmailError> {
        let site_name = if self.config.site_name.is_empty() {
            self.config.from.clone()
        } else {
            self.config.site_name.clone()
        };

        let from_header = format!("{} <{}>", site_name, self.config.from);

        let from_addr: lettre::message::Mailbox = from_header
            .parse()
            .map_err(|e: lettre::address::AddressError| {
                EmailError::MessageBuild(e.to_string())
            })?;

        let mut builder = Message::builder().from(from_addr);

        if let Some(ref reply_to) = self.config.reply_to {
            let reply_addr: lettre::message::Mailbox = reply_to
                .parse()
                .map_err(|e: lettre::address::AddressError| {
                    EmailError::MessageBuild(e.to_string())
                })?;
            builder = builder.reply_to(reply_addr);
        }

        for addr in to {
            let to_addr: lettre::message::Mailbox = addr
                .parse()
                .map_err(|e: lettre::address::AddressError| {
                    EmailError::MessageBuild(e.to_string())
                })?;
            builder = builder.to(to_addr);
        }

        let message = builder
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(body.to_string())
            .map_err(|e| EmailError::MessageBuild(e.to_string()))?;

        self.mailer.send(message).await?;
        Ok(())
    }
}

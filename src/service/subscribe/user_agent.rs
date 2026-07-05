//! User-agent detection for subscribe endpoint.
//!
//! Port of `server/internal/logic/subscribe/userAgentLogic.go`.

use crate::model::entity::client::SubscribeApplication;

/// Detect which [`SubscribeApplication`] matches the given `ua` string.
///
/// Matching is case-insensitive. Stash is checked first because it may embed
/// "quantumult" in its UA string, and must not be misidentified.
pub fn detect_client<'a>(ua: &str, clients: &'a [SubscribeApplication]) -> Option<&'a SubscribeApplication> {
    let ua_lower = ua.to_lowercase();

    // Stash special-case: must be matched before Quantumult.
    if ua_lower.contains("stash") {
        if let Some(c) = clients.iter().find(|c| c.user_agent.to_lowercase().contains("stash")) {
            return Some(c);
        }
    }

    // General: first client whose user_agent substring appears in UA.
    clients.iter().find(|c| {
        let needle = c.user_agent.to_lowercase();
        !needle.is_empty() && ua_lower.contains(needle.as_str())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_client(name: &str, ua: &str) -> SubscribeApplication {
        SubscribeApplication {
            id: 0,
            name: name.into(),
            icon: None,
            description: None,
            scheme: String::new(),
            user_agent: ua.into(),
            is_default: false,
            subscribe_template: None,
            output_format: "yaml".into(),
            download_link: String::new(),
            created_at: 0,
            updated_at: 0,
        }
    }

    #[test]
    fn test_detect_clash() {
        let clients = vec![
            make_client("Clash", "clash"),
            make_client("Quantumult X", "quantumult"),
        ];
        let result = detect_client("ClashforAndroid/2.5.12 okhttp/3.12.1", &clients);
        assert_eq!(result.map(|c| c.name.as_str()), Some("Clash"));
    }

    #[test]
    fn test_stash_before_quantumult() {
        let clients = vec![
            make_client("Quantumult X", "quantumult"),
            make_client("Stash", "stash"),
        ];
        let result = detect_client("Stash/2.4.5 like QuantumultX", &clients);
        assert_eq!(result.map(|c| c.name.as_str()), Some("Stash"));
    }

    #[test]
    fn test_no_match() {
        let clients = vec![make_client("Clash", "clash")];
        assert!(detect_client("curl/7.88.1", &clients).is_none());
    }
}

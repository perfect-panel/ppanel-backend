use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct RequestMeta {
    pub if_none_match: String,
}

#[derive(Debug, Clone, Default)]
pub struct ResponseMeta {
    pub headers: HashMap<String, String>,
}

impl ResponseMeta {
    pub fn new() -> Self {
        Self { headers: HashMap::new() }
    }

    pub fn set_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }
}

pub fn generate_etag(data: &[u8]) -> String {
    let digest = md5::compute(data);
    format!("{:x}", digest)
}

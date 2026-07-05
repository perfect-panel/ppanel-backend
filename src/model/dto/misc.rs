//! Uncategorized DTO types (shared/generic).
//! Auto-generated from the monolithic `dto.rs`.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;


// ─── Custom Types ───────────────────────────────────────────────────────────

/// A `Vec<i64>` that serializes as `["1","2","3"]` and accepts both strings
/// and numbers when deserializing.
#[derive(Debug, Clone, Default)]
pub struct StringInt64Slice(pub Vec<i64>);

impl Serialize for StringInt64Slice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for item in &self.0 {
            seq.serialize_element(&item.to_string())?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for StringInt64Slice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let raw: Vec<Value> = Vec::deserialize(deserializer)?;
        let mut values = Vec::with_capacity(raw.len());

        for item in raw {
            match item {
                Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        values.push(i);
                    } else {
                        return Err(D::Error::custom("number out of i64 range"));
                    }
                }
                Value::String(s) => {
                    let trimmed = s.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    let parsed = trimmed
                        .parse::<i64>()
                        .map_err(|_| D::Error::custom(format!("invalid integer string: {}", s)))?;
                    values.push(parsed);
                }
                Value::Null => continue,
                _ => return Err(D::Error::custom("expected number or string")),
            }
        }

        Ok(StringInt64Slice(values))
    }
}

impl StringInt64Slice {
    pub fn int64s(&self) -> &[i64] {
        &self.0
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimePeriod {
    pub start_time: String,
    pub end_time: String,
    pub multiplier: f32,
}

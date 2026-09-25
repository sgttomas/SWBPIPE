//! Pure protocol handler for the standalone scientific authority executable.
use crate::binary64::{
    canonical_json_binary64_v1_text, parse_with_byte_limit, BINARY64_PROFILE_V1,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub const PROTOCOL_VERSION: &str = "1.0.0";
pub const MAX_REQUEST_BYTES: usize = 64 * 1024 * 1024;
pub const MAX_BATCH_ITEMS: usize = 4096;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    protocol_version: String,
    profile: String,
    items: Vec<RequestItem>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RequestItem {
    id: String,
    json_text: String,
}
#[derive(Serialize)]
struct Response<'a> {
    protocol_version: &'a str,
    profile: &'a str,
    items: Vec<ResponseItem>,
}
#[derive(Serialize)]
struct ResponseItem {
    id: String,
    canonical_json: String,
}

/// Whole-batch success or rejection. The envelope itself is strictly parsed;
/// duplicate/unknown fields cannot be overwritten or silently ignored.
pub fn canonicalize_binary64_v1_request(input: &str) -> Result<String, String> {
    let envelope = parse_with_byte_limit(input, MAX_REQUEST_BYTES)?;
    let request: Request = serde_json::from_value(envelope.into_value())
        .map_err(|e| format!("BINARY64-CLI-REQUEST: {e}"))?;
    if request.protocol_version != PROTOCOL_VERSION {
        return Err("BINARY64-CLI-PROTOCOL-MISMATCH".into());
    }
    if request.profile != BINARY64_PROFILE_V1 {
        return Err("BINARY64-CLI-PROFILE-MISMATCH".into());
    }
    if request.items.len() > MAX_BATCH_ITEMS {
        return Err("BINARY64-CLI-BATCH-LIMIT".into());
    }
    let mut ids = HashSet::new();
    let mut items = Vec::with_capacity(request.items.len());
    for item in request.items {
        if item.id.is_empty() || !ids.insert(item.id.clone()) {
            return Err("BINARY64-CLI-ITEM-ID-INVALID".into());
        }
        let canonical_json = canonical_json_binary64_v1_text(&item.json_text)
            .map_err(|e| format!("BINARY64-CLI-ITEM-INVALID {}: {e}", item.id))?;
        items.push(ResponseItem {
            id: item.id,
            canonical_json,
        });
    }
    serde_json::to_string(&Response {
        protocol_version: PROTOCOL_VERSION,
        profile: BINARY64_PROFILE_V1,
        items,
    })
    .map_err(|e| format!("BINARY64-CLI-RESPONSE: {e}"))
}

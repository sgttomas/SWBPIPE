#![cfg(feature = "checked-cli")]

use open_pipe_stress_canonical_json::{canonical_json_checked_v1_text, CHECKED_PROFILE_V1};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io::{self, Read};

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

fn run() -> Result<(), String> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).map_err(|e| format!("CHECKED-CLI-STDIN: {e}"))?;
    let request: Request = serde_json::from_str(&input).map_err(|e| format!("CHECKED-CLI-REQUEST: {e}"))?;
    if request.protocol_version != "1.0.0" { return Err("CHECKED-CLI-PROTOCOL-MISMATCH".into()); }
    if request.profile != CHECKED_PROFILE_V1 { return Err("CHECKED-CLI-PROFILE-MISMATCH".into()); }
    let mut ids = HashSet::new();
    let mut items = Vec::with_capacity(request.items.len());
    for item in request.items {
        if item.id.is_empty() || !ids.insert(item.id.clone()) { return Err("CHECKED-CLI-ITEM-ID-INVALID".into()); }
        let canonical_json = canonical_json_checked_v1_text(&item.json_text)
            .map_err(|e| format!("CHECKED-CLI-ITEM-INVALID {}: {e}", item.id))?;
        items.push(ResponseItem { id: item.id, canonical_json });
    }
    let response = Response { protocol_version: "1.0.0", profile: CHECKED_PROFILE_V1, items };
    print!("{}", serde_json::to_string(&response).map_err(|e| e.to_string())?);
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(2);
    }
}

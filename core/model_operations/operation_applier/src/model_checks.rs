//! On-demand model checks for the desktop (DEL-04-06 Design
//! `on-demand-model-checks.md`): the solve's own checks before stiffness
//! formation (`open_pipe_stress_product_physics::model_settings::model_checks`),
//! run on the session model as the desktop's solve request carries it
//! (`{"model": …, "materials": []}`). Nothing here writes to the model, adds a
//! rule or solves.
//!
//! The report is bounded whatever the model (Codex review r1 of #1665): at most
//! [`MODEL_CHECKS_MAX_RESPONSE_BYTES`] of serialized findings, in the checks'
//! order, each text cut at [`MODEL_CHECKS_MAX_TEXT_BYTES`] on a character
//! boundary and each finding's references at [`MODEL_CHECKS_MAX_REFS`];
//! `diagnostics_not_shown` counts the findings left out. A refusal's text is cut
//! the same way. The native command runs it under its interactive lease; the
//! browser engine also refuses a model text above
//! [`MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES`] before parsing it.
use open_pipe_stress_product_physics::model_settings::model_checks::preview_model_checks_value;
use serde_json::{json, Map, Value};

pub const MODEL_CHECKS_DOCUMENT_KIND: &str = "openpipestress.desktop.model_checks";
/// The serialized findings a report carries at most.
pub const MODEL_CHECKS_MAX_RESPONSE_BYTES: usize = 1024 * 1024;
/// The longest text of a finding or a refusal, in UTF-8 bytes.
pub const MODEL_CHECKS_MAX_TEXT_BYTES: usize = 2048;
/// The references a finding names at most.
pub const MODEL_CHECKS_MAX_REFS: usize = 16;
/// The longest model text the browser engine checks (4 MiB); a longer one is
/// refused `MODEL-CHECKS-INPUT-TOO-LARGE` before it is parsed.
pub const MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES: usize = 4 * 1024 * 1024;
pub const MODEL_CHECKS_INPUT_TOO_LARGE: &str = "MODEL-CHECKS-INPUT-TOO-LARGE";

/// [`model_checks_owned`] on a copy of `model`.
pub fn model_checks(model: &Value) -> Value {
    model_checks_owned(model.clone())
}

/// `{document_kind, checked: true, diagnostics, diagnostics_not_shown}` with the
/// solve's diagnostics as it publishes them (bounded as the module says), or
/// `{document_kind, checked: false, refusal}` with the solve's own refusal text
/// when the request cannot be read. The model is consumed.
pub fn model_checks_owned(model: Value) -> Value {
    let mut request = Map::new();
    request.insert("model".to_string(), model);
    request.insert("materials".to_string(), Value::Array(Vec::new()));
    match preview_model_checks_value(Value::Object(request)) {
        Ok(diagnostics) => {
            let mut shown = Vec::new();
            let mut bytes = 0usize;
            let mut not_shown = 0usize;
            for diagnostic in diagnostics {
                if not_shown > 0 {
                    not_shown += 1;
                    continue;
                }
                let Ok(Value::Object(fields)) = serde_json::to_value(diagnostic) else {
                    not_shown += 1;
                    continue;
                };
                let bounded = bounded_finding(fields);
                let length = serde_json::to_string(&bounded).map(|text| text.len() + 1).unwrap_or(usize::MAX);
                if bytes.saturating_add(length) > MODEL_CHECKS_MAX_RESPONSE_BYTES {
                    not_shown += 1;
                    continue;
                }
                bytes += length;
                shown.push(bounded);
            }
            json!({
                "document_kind": MODEL_CHECKS_DOCUMENT_KIND,
                "checked": true,
                "diagnostics": shown,
                "diagnostics_not_shown": not_shown,
            })
        }
        Err(refusal) => json!({
            "document_kind": MODEL_CHECKS_DOCUMENT_KIND,
            "checked": false,
            "refusal": cut(refusal).0,
        }),
    }
}

/// The browser engine's entry: the model text, refused unparsed above
/// [`MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES`]; `Err` carries a text that is not JSON.
pub fn model_checks_text(model_json: &str) -> Result<Value, serde_json::Error> {
    if model_json.len() > MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES {
        return Ok(json!({
            "document_kind": MODEL_CHECKS_DOCUMENT_KIND,
            "checked": false,
            "refusal_code": MODEL_CHECKS_INPUT_TOO_LARGE,
            "refusal": format!(
                "{MODEL_CHECKS_INPUT_TOO_LARGE}: the browser preview checks a model text of at most {MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES} bytes; this one is {} bytes. The desktop app checks larger models under its memory pool.",
                model_json.len()
            ),
        }));
    }
    serde_json::from_str::<Value>(model_json).map(model_checks_owned)
}

fn bounded_finding(mut fields: Map<String, Value>) -> Value {
    let mut truncated = false;
    for key in ["id", "code", "severity", "message", "source"] {
        if let Some(Value::String(text)) = fields.remove(key) {
            let (text, was_cut) = cut(text);
            truncated |= was_cut;
            fields.insert(key.to_string(), Value::String(text));
        }
    }
    if let Some(Value::Array(refs)) = fields.remove("affected_refs") {
        let total = refs.len();
        let kept: Vec<Value> = refs
            .into_iter()
            .take(MODEL_CHECKS_MAX_REFS)
            .map(|reference| match reference {
                Value::String(text) => {
                    let (text, was_cut) = cut(text);
                    truncated |= was_cut;
                    Value::String(text)
                }
                other => other,
            })
            .collect();
        truncated |= total > kept.len();
        fields.insert("affected_refs".to_string(), Value::Array(kept));
    }
    if truncated {
        fields.insert("truncated".to_string(), Value::Bool(true));
    }
    Value::Object(fields)
}

/// `text` cut at [`MODEL_CHECKS_MAX_TEXT_BYTES`] on a character boundary, and whether it was cut.
fn cut(mut text: String) -> (String, bool) {
    if text.len() <= MODEL_CHECKS_MAX_TEXT_BYTES {
        return (text, false);
    }
    let mut end = MODEL_CHECKS_MAX_TEXT_BYTES;
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    text.truncate(end);
    (text, true)
}

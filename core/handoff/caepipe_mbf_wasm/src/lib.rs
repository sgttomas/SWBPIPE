//! The desktop webview's MBF writer and importer (DEL-17-04 Design `desktop-mbf-export-import.md` §2, §3.1–§3.3,
//! slice 4a): a JSON-text wrapper of the engine crate `open_pipe_stress_caepipe_mbf`, compiled to wasm with
//! `--features wasm` by `apps/desktop/scripts/build-wasm-engine.mjs` and loaded by `src/services/mbfEngine.ts`.
//!
//! [`render_mbf_json`] runs the SI writer over the session's canonical model text with the export dialog's
//! settings and returns the review: the status, the MBF text and its SHA-256, the writer's `to_dict()` bytes, and
//! the identity file's bytes (the writer sidecar for that SHA-256) and their SHA-256. The webview shows the review
//! and discards the text: the bytes written are always the native writer's output for the hash-checked text
//! (`apps/desktop/src-tauri/src/mbf_export.rs`), which maps the same settings text by the same engine function
//! ([`export_settings`]) and refuses a save whose digests differ from the reviewed ones. [`import_mbf_json`] runs the importer for the
//! later import slice.
//!
//! The model text is read as the engine reads JSON (`parse_sidecar_json`: strict UTF-8 and CPython's `json.loads`,
//! with its nesting bound), so the webview and the native command parse the same text to the same values.
//!
//! The crate does no file I/O. Standard claim fence applies (claims taxonomy per DEC-081): a written MBF means the
//! listed subset of the model was written; it is not evidence that two programs analyse the model alike.

#![forbid(unsafe_code)]

use open_pipe_stress_caepipe_mbf::{
    self as mbf, ImportError, ImportOptions, SidecarInput, Value, WriterResult, WriterSettings, MBF_SI_PROFILE_ID,
    MBF_VERSION,
};
use serde_json::json;
use sha2::{Digest, Sha256};

pub const RENDER_OUTCOME_KIND: &str = "swbpipe.desktop.mbf_render_outcome";
pub const IMPORT_OUTCOME_KIND: &str = "swbpipe.desktop.mbf_import_outcome";

/// The export settings are not `{title, referenceTemperature: {value, unit}, maxLayoutRows: null}`.
pub const SETTINGS_INVALID: &str = mbf::EXPORT_SETTINGS_INVALID;
/// The model text is not JSON the engine reads.
pub const MODEL_JSON_INVALID: &str = "MBF-EXPORT-MODEL-JSON-INVALID";
/// The import options are not `{sourceLabel, materialProvenance?}`.
pub const IMPORT_OPTIONS_INVALID: &str = "MBF-IMPORT-OPTIONS-INVALID";
/// The engine returned no result (an exception the Python twin raises, or an input it fails closed on).
pub const ENGINE_FAILED: &str = "MBF-ENGINE-FAILED";

/// A refusal before or instead of a writer or importer result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Failure {
    pub code: &'static str,
    pub message: String,
}

fn failure(code: &'static str, message: impl Into<String>) -> Failure {
    Failure { code, message: message.into() }
}

/// Lowercase hex SHA-256.
pub fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes).iter().map(|byte| format!("{byte:02x}")).collect()
}

/// Reads JSON text as the engine does (`parse_sidecar_json`), naming `what` in a refusal.
pub fn parse_json(bytes: &[u8], what: &str, code: &'static str) -> Result<Value, Failure> {
    mbf::parse_sidecar_json(bytes).map_err(|error| {
        let reason = match error {
            mbf::SidecarJsonError::Invalid(findings) => findings
                .into_iter()
                .map(|(_, reason)| reason)
                .collect::<Vec<_>>()
                .join("; "),
            mbf::SidecarJsonError::LoneSurrogate => "holds an unpaired surrogate escape".to_string(),
            mbf::SidecarJsonError::TooDeep => {
                format!("nests deeper than {} arrays and objects", mbf::MAX_SIDECAR_NESTING)
            }
        };
        failure(code, format!("The {what} {reason}."))
    })
}

/// The export dialog's settings text `{title, referenceTemperature: {value, unit}, maxLayoutRows: null}` as the
/// writer's arguments (Design §3.1), by the engine's one mapping (`export_settings_from_json`): the title as given, at
/// most 4 KiB of UTF-8 (the writer trims it), the reference temperature as `reference_temperature_c` for `degC` or
/// `reference_temperature_f` for `degF` (exact), and no row limit (`max_layout_rows=None`, Q-4). The native command
/// is given the same text and maps it by the same function.
pub fn export_settings(settings_json: &[u8]) -> Result<WriterSettings, Failure> {
    mbf::export_settings_from_json(settings_json).map_err(|error| failure(SETTINGS_INVALID, error.message))
}

/// The unit conversions the writer applied: each normalized unit, its exact definition and, where it has one, its
/// exact factor as `numerator/denominator` (the engine's normalization table).
fn unit_conversions(result: &WriterResult) -> Vec<serde_json::Value> {
    let factors = mbf::ports::writer_unit_factors();
    result
        .unit_normalizations
        .iter()
        .map(|(unit, definition)| {
            let factor = factors
                .iter()
                .find(|(table, key, _)| *table == "normalization" && key == unit)
                .and_then(|(_, _, factor)| factor.as_ref())
                .map(|(numerator, denominator)| format!("{numerator}/{denominator}"));
            json!({"unit": unit, "definition": definition, "factor": factor})
        })
        .collect()
}

/// The writer's result for the model text, and for a written result the MBF's SHA-256 and the identity file's bytes
/// and SHA-256.
#[derive(Debug)]
pub struct Rendered {
    pub result: WriterResult,
    pub written: Option<Written>,
}

/// A written result's file contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Written {
    pub mbf_sha256: String,
    /// The identity file (`<stem>.swbpipe-ids.json`): the writer sidecar for `mbf_sha256`, as its bytes.
    pub sidecar: Vec<u8>,
    pub sidecar_sha256: String,
}

fn engine_failure(error: ImportError) -> Failure {
    failure(ENGINE_FAILED, format!("The MBF engine returned no result ({error})."))
}

/// Runs the writer over `model_text` (read as the engine reads JSON) with `settings`.
pub fn render_export(model_text: &[u8], settings: &WriterSettings) -> Result<Rendered, Failure> {
    let model = parse_json(model_text, "model text", MODEL_JSON_INVALID)?;
    let result = mbf::render_si(&model, settings).map_err(engine_failure)?;
    mbf::value::dispose(model);
    let written = match (&result.status[..], &result.text) {
        ("written", Some(text)) => {
            let mbf_sha256 = sha256_hex(text.as_bytes());
            let sidecar = result.sidecar_bytes(&mbf_sha256).map_err(engine_failure)?;
            let sidecar_sha256 = sha256_hex(&sidecar);
            Some(Written { mbf_sha256, sidecar, sidecar_sha256 })
        }
        _ => None,
    };
    Ok(Rendered { result, written })
}

fn ascii(bytes: Vec<u8>) -> String {
    // The engine's canonical bytes are `ensure_ascii` JSON.
    String::from_utf8(bytes).unwrap_or_default()
}

fn failed(kind: &str, failure: Failure) -> String {
    json!({
        "document_kind": kind,
        "status": "failed",
        "failure": {"code": failure.code, "message": failure.message},
    })
    .to_string()
}

/// The export review for the webview: `{document_kind, status, profile_id, mbf_version, mbf_text, mbf_sha256,
/// sidecar_text, sidecar_sha256, report_json, unit_conversions, failure}`. `status` is the writer's (`written` or `refused`), or
/// `failed` with `failure` `{code, message}` when the settings or the model text are refused or the engine returns no
/// result. `report_json` is the writer's `to_dict()` bytes; the digests are lowercase hex SHA-256 of the MBF text
/// and of the identity file's bytes (`sidecar_text`).
pub fn render_mbf_outcome(model_json: &str, settings_json: &str) -> String {
    let settings = match export_settings(settings_json.as_bytes()) {
        Ok(settings) => settings,
        Err(error) => return failed(RENDER_OUTCOME_KIND, error),
    };
    let rendered = match render_export(model_json.as_bytes(), &settings) {
        Ok(rendered) => rendered,
        Err(error) => return failed(RENDER_OUTCOME_KIND, error),
    };
    let report = match rendered.result.to_dict_bytes() {
        Ok(report) => ascii(report),
        Err(error) => return failed(RENDER_OUTCOME_KIND, engine_failure(error)),
    };
    let written = rendered.written.as_ref();
    json!({
        "document_kind": RENDER_OUTCOME_KIND,
        "status": rendered.result.status,
        "profile_id": MBF_SI_PROFILE_ID,
        "mbf_version": MBF_VERSION,
        "mbf_text": written.and(rendered.result.text.as_deref()),
        "mbf_sha256": written.map(|item| item.mbf_sha256.as_str()),
        "sidecar_text": written.map(|item| String::from_utf8_lossy(&item.sidecar).into_owned()),
        "sidecar_sha256": written.map(|item| item.sidecar_sha256.as_str()),
        "report_json": report,
        "unit_conversions": unit_conversions(&rendered.result),
        "failure": null,
    })
    .to_string()
}

/// The importer for the webview (the later import slice): `{document_kind, status, model_json, report_json,
/// id_map_json, failure}`. `options_json` is `{sourceLabel, materialProvenance?}`; `sidecar` is the identity file's
/// bytes when one was read. `status` is the importer's, or `failed` with `failure`.
pub fn import_mbf_outcome(mbf_bytes: &[u8], sidecar: Option<Vec<u8>>, options_json: &str) -> String {
    let options = match parse_json(options_json.as_bytes(), "import options", IMPORT_OPTIONS_INVALID) {
        Ok(options) => options,
        Err(error) => return failed(IMPORT_OUTCOME_KIND, error),
    };
    let invalid = |detail: &str| failure(IMPORT_OPTIONS_INVALID, format!("The import options must be {{sourceLabel, materialProvenance?}}: {detail}."));
    let Some(dict) = options.as_dict() else { return failed(IMPORT_OUTCOME_KIND, invalid("not an object")) };
    if dict.keys().any(|key| key != "sourceLabel" && key != "materialProvenance") {
        return failed(IMPORT_OUTCOME_KIND, invalid("an unknown member"));
    }
    let Some(label) = options.get_or_null("sourceLabel").as_str() else {
        return failed(IMPORT_OUTCOME_KIND, invalid("sourceLabel is not a string"));
    };
    let mut import = ImportOptions::new(label.to_string());
    import.sidecar = sidecar.map(SidecarInput::Json);
    import.material_provenance = options.get("materialProvenance").filter(|value| !value.is_null()).cloned();
    let result = match mbf::import(mbf_bytes, &import) {
        Ok(result) => result,
        Err(error) => return failed(IMPORT_OUTCOME_KIND, engine_failure(error)),
    };
    let parts = (|| Ok::<_, ImportError>((result.model_bytes()?, result.report_bytes()?, result.id_map_bytes()?)))();
    match parts {
        Ok((model, report, id_map)) => json!({
            "document_kind": IMPORT_OUTCOME_KIND,
            "status": result.status,
            "model_json": model.map(ascii),
            "report_json": ascii(report),
            "id_map_json": ascii(id_map),
            "failure": null,
        })
        .to_string(),
        Err(error) => failed(IMPORT_OUTCOME_KIND, engine_failure(error)),
    }
}

/// [`render_mbf_outcome`], exported to the webview.
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub fn render_mbf_json(model_json: &str, settings_json: &str) -> String {
    render_mbf_outcome(model_json, settings_json)
}

/// [`import_mbf_outcome`], exported to the webview.
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub fn import_mbf_json(mbf_bytes: &[u8], sidecar: Option<Vec<u8>>, options_json: &str) -> String {
    import_mbf_outcome(mbf_bytes, sidecar, options_json)
}

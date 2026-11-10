//! The designated-source check (DEL-17-04 Design `desktop-mbf-export-import.md` §5.4): whether a model document the
//! user designated is the exact source a format-6 MBF pair was written from, and whether the bound import of that pair
//! carries the source's identity set for all nine A-3 classes.
//!
//! The check runs in phases, so each phase's working values are released before the next is formed (a lifetime
//! reduction; the peak is the largest phase, not their sum):
//!
//! 1. **The pair's declared derivation** ([`evaluate_designated_source`]): the identity file is read by the engine's
//!    strict reader, its format must be 6, and its `derivation` gives the source digest and the writer's settings.
//!    The parsed file is disposed before the next phase.
//! 2. **The designated source**: read by the same reader the export worker reads its model text with; its canonical
//!    SHA-256 (RFC 8785) must be the derivation's `source_model_sha256`. Only then is it rendered by the same writer
//!    ([`crate::render_si`]) with the derivation's settings, and the rendered MBF and identity-file bytes must be the
//!    pair's bytes exactly. Its identity inventory ([`crate::identity_inventory`]) is kept (fixed size); the parsed
//!    source and the rendering are disposed.
//! 3. **The import** ([`designation`], after the caller's import of the same pair): the import must be bound to a
//!    format-6 identity file declaring the same derivation, its identity inventory must equal the source's class by
//!    class (no duplicate or malformed entry on either side), and it must report no unrecoverable id and no
//!    unrecovered field or reference.
//!
//! A source that passes every phase is established **for the identity set** only: the same ids in each class. It says
//! nothing about whole-model equivalence, values, or what the exchange loses (the loss entries still apply). The check
//! keeps nothing: the source's bytes are the caller's, and only fixed-size facts leave this module.

use crate::identity_inventory::{compare_inventories, identity_inventory, IdentityInventory};
use crate::value::{self, Value};
use crate::writer::WriterSettings;
use crate::ImportResult;

/// The identity file is absent, so there is no derivation to check against.
pub const IDENTITY_FILE_ABSENT: &str = "identity_file_absent";
/// The identity file is not a format-6 file with a readable `derivation`.
pub const IDENTITY_FILE_NOT_FORMAT_6: &str = "identity_file_not_format_6";
/// The designated file is not a model document the engine reads (not strict JSON, or no canonical form).
pub const SOURCE_NOT_A_MODEL_DOCUMENT: &str = "source_not_a_model_document";
/// The designated document's canonical digest is not the derivation's.
pub const SOURCE_DIGEST_DIFFERS: &str = "source_digest_differs";
/// The writer refused the designated document with the derivation's settings, or returned no result.
pub const RERENDER_REFUSED: &str = "rerender_refused";
/// The re-rendered MBF is not the chosen MBF's bytes.
pub const RERENDER_MBF_DIFFERS: &str = "rerender_mbf_differs";
/// The re-rendered identity file is not the chosen identity file's bytes.
pub const RERENDER_IDENTITY_FILE_DIFFERS: &str = "rerender_identity_file_differs";
/// The import is not bound to a format-6 identity file declaring the same derivation.
pub const IMPORT_NOT_BOUND_FORMAT_6: &str = "import_not_bound_format_6";
/// An entry without a string id, or a class member that is not a list, on either side.
pub const IDENTITY_INVENTORY_MALFORMED: &str = "identity_inventory_malformed";
/// At least one class's identity set differs, or holds a duplicate (`differing_classes` names them).
pub const IDENTITY_SET_DIFFERS: &str = "identity_set_differs";
/// The import names original ids it does not preserve.
pub const IDENTITIES_NOT_PRESERVED: &str = "identities_not_preserved";
/// The import reports fields or references it did not recover.
pub const FIELDS_NOT_RECOVERED: &str = "fields_not_recovered";
/// The review holding the outcome is truncated (added by the review itself).
pub const REVIEW_TRUNCATED: &str = "review_truncated";

/// Every reason, in the order a review lists them.
pub const DESIGNATION_REASONS: [&str; 13] = [
    IDENTITY_FILE_ABSENT,
    IDENTITY_FILE_NOT_FORMAT_6,
    SOURCE_NOT_A_MODEL_DOCUMENT,
    SOURCE_DIGEST_DIFFERS,
    RERENDER_REFUSED,
    RERENDER_MBF_DIFFERS,
    RERENDER_IDENTITY_FILE_DIFFERS,
    IMPORT_NOT_BOUND_FORMAT_6,
    IDENTITY_INVENTORY_MALFORMED,
    IDENTITY_SET_DIFFERS,
    IDENTITIES_NOT_PRESERVED,
    FIELDS_NOT_RECOVERED,
    REVIEW_TRUNCATED,
];

/// What phases 1 and 2 found: fixed-size facts only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceEvaluation {
    /// The derivation's `source_model_sha256`, when phase 1 read one.
    pub derivation_source_sha256: Option<String>,
    /// The designated document's identity inventory, when it is a model document.
    pub inventory: Option<IdentityInventory>,
    /// The reasons found so far, in [`DESIGNATION_REASONS`] order.
    pub reasons: Vec<&'static str>,
}

/// The designated source as the review states it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReviewDesignation<'a> {
    /// The designated file's name.
    pub file_name: &'a str,
    /// The SHA-256 of the designated file's bytes, as given (lowercase hex).
    pub sha256: &'a str,
    pub bytes: u64,
    /// The reasons the identity set is not established, in [`DESIGNATION_REASONS`] order; empty when it is (before
    /// the review's own completeness, which the review adds).
    pub reasons: Vec<&'static str>,
    /// Per [`crate::INVENTORY_CLASSES`] entry: its identity set differs or holds a duplicate.
    pub differing: [bool; 9],
}

/// The writer's arguments a format-6 `derivation.export_settings` records, or None when they are not as the writer
/// records them.
fn settings_of(derivation: &Value) -> Option<(String, WriterSettings)> {
    let source = derivation.get("source_model_sha256")?.as_str()?;
    if source.len() != 64 || !source.bytes().all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f')) {
        return None;
    }
    let recorded = derivation.get("export_settings")?;
    if recorded.as_dict()?.len() != 4 {
        return None;
    }
    let title = recorded.get("title")?.as_str()?;
    let reference = recorded.get("reference_temperature")?;
    if reference.as_dict()?.len() != 2 {
        return None;
    }
    let temperature = match reference.get("value")? {
        Value::Float(value) if value.is_finite() => Value::Float(*value),
        _ => return None,
    };
    let mut settings = WriterSettings::new(title.to_string());
    match reference.get("unit")?.as_str()? {
        "degC" => settings.reference_temperature_c = temperature,
        "degF" => settings.reference_temperature_f = temperature,
        _ => return None,
    }
    settings.max_layout_rows = match recorded.get("max_layout_rows")? {
        Value::Null => None,
        rows => Some(value::small_int(rows)?),
    };
    settings.piping_code = match recorded.get("piping_code")? {
        Value::Null => Value::Null,
        Value::Str(code) => Value::Str(code.clone()),
        _ => return None,
    };
    Some((source.to_string(), settings))
}

/// Phase 1: the derivation the identity file declares, or why there is none.
fn declared(identity: Option<&[u8]>) -> Result<(String, WriterSettings), &'static str> {
    let Some(identity) = identity else { return Err(IDENTITY_FILE_ABSENT) };
    let Ok(sidecar) = crate::parse_sidecar_json(identity) else { return Err(IDENTITY_FILE_NOT_FORMAT_6) };
    let format_6 = value::small_int(sidecar.get_or_null("sidecar_format")) == Some(crate::sidecar::SIDECAR_FORMAT);
    let declared = if format_6 { sidecar.get("derivation").and_then(settings_of) } else { None };
    value::dispose(sidecar);
    declared.ok_or(IDENTITY_FILE_NOT_FORMAT_6)
}

fn sha256_hex(bytes: &[u8]) -> String {
    crate::sidecar::hex(&<sha2::Sha256 as sha2::Digest>::digest(bytes))
}

/// Phases 1 and 2 for the designated document `source` (its bytes as read) and the chosen pair (`mbf`, `identity`).
pub fn evaluate_designated_source(source: &[u8], mbf: &[u8], identity: Option<&[u8]>) -> SourceEvaluation {
    let mut reasons = Vec::new();
    let declared = declared(identity).map_err(|reason| reasons.push(reason)).ok();
    let derivation_source_sha256 = declared.as_ref().map(|(sha, _)| sha.clone());
    let Ok(document) = crate::parse_sidecar_json(source) else {
        reasons.push(SOURCE_NOT_A_MODEL_DOCUMENT);
        return SourceEvaluation { derivation_source_sha256, inventory: None, reasons };
    };
    let inventory = identity_inventory(&document);
    match crate::model_canonical_sha256(&document) {
        Err(_) => reasons.push(SOURCE_NOT_A_MODEL_DOCUMENT),
        Ok(canonical) => {
            if let Some((declared_sha, settings)) = &declared {
                if canonical != *declared_sha {
                    reasons.push(SOURCE_DIGEST_DIFFERS);
                } else {
                    let rendered = crate::render_si(&document, settings);
                    match rendered {
                        Ok(mut result) => match result.text.take().filter(|_| result.status == "written") {
                            Some(text) => {
                                let mbf_sha = sha256_hex(text.as_bytes());
                                let mbf_same = text.as_bytes() == mbf;
                                drop(text);
                                if !mbf_same {
                                    reasons.push(RERENDER_MBF_DIFFERS);
                                }
                                match result.sidecar_bytes(&mbf_sha) {
                                    Ok(sidecar) if Some(sidecar.as_slice()) == identity => {}
                                    Ok(_) => reasons.push(RERENDER_IDENTITY_FILE_DIFFERS),
                                    Err(_) => reasons.push(RERENDER_REFUSED),
                                }
                            }
                            None => reasons.push(RERENDER_REFUSED),
                        },
                        Err(_) => reasons.push(RERENDER_REFUSED),
                    }
                }
            }
        }
    }
    value::dispose(document);
    SourceEvaluation { derivation_source_sha256, inventory: Some(inventory), reasons }
}

/// Phase 3: the designation the review states for `evaluation` and the caller's import of the same pair. `result`'s
/// model must still be present (its inventory is read here).
pub fn designation<'a>(file_name: &'a str, sha256: &'a str, bytes: u64, evaluation: &SourceEvaluation, result: &ImportResult) -> ReviewDesignation<'a> {
    let mut reasons = evaluation.reasons.clone();
    let bound_6 = result.status == "imported"
        && result.derivation.as_ref().is_some_and(|derivation| Some(&derivation.source_model_sha256) == evaluation.derivation_source_sha256.as_ref());
    if !bound_6 && evaluation.derivation_source_sha256.is_some() {
        reasons.push(IMPORT_NOT_BOUND_FORMAT_6);
    }
    let mut differing = [false; 9];
    match (&evaluation.inventory, &result.model) {
        (Some(source), Some(model)) => {
            let comparison = compare_inventories(source, &identity_inventory(model));
            differing = comparison.differing;
            if comparison.malformed {
                reasons.push(IDENTITY_INVENTORY_MALFORMED);
            }
            if differing.iter().any(|differs| *differs) {
                reasons.push(IDENTITY_SET_DIFFERS);
            }
        }
        // No candidate model, or no source document: the identity set cannot be compared (a reason above says why).
        _ if reasons.is_empty() => reasons.push(IMPORT_NOT_BOUND_FORMAT_6),
        _ => {}
    }
    if result.identity_recovery.known_missing_ids > 0 {
        reasons.push(IDENTITIES_NOT_PRESERVED);
    }
    if result.identity_recovery.unrecovered_fields > 0 {
        reasons.push(FIELDS_NOT_RECOVERED);
    }
    reasons.sort_by_key(|reason| DESIGNATION_REASONS.iter().position(|known| known == reason));
    reasons.dedup();
    ReviewDesignation { file_name, sha256, bytes, reasons, differing }
}

//! The pressure-free Timoshenko report projection (DEL-05-03 Design
//! `pf-timoshenko.md` §4, "Source binding on PF").
//!
//! A report of a `preview-physics-timoshenko-1` result carries the identity's
//! limitation 0 as one entry after the technical-preview entry, and the
//! result's diagnostics (among them `PF_TIMOSHENKO_REALIZED_ARCS_EULER_BERNOULLI`
//! where the result has realized arcs) as the report's first Diagnostics
//! entries. The package's item 9 and the desktop's native render compare both,
//! text and references, with the delivered result. This identity only: a
//! preview-physics-2 report carries no such entry and takes neither check, so
//! its bytes and checks are unchanged. Equality checks only. Standard claim
//! fence applies (claims taxonomy per DEC-081).

use open_pipe_stress_result_export::semantic_contract::{
    preview_physics_timoshenko_contract, PREVIEW_PHYSICS_TIMOSHENKO_ID,
};
use serde_json::{json, Value};

pub const PF_TIMOSHENKO_SECTIONS_MISMATCH: &str = "REPORT-PACKAGE-PF-TIMOSHENKO-SECTIONS-MISMATCH";
/// The arc disclosure's diagnostic code (pf-timoshenko.md §4, "Named arc disclosure").
pub const REALIZED_ARCS_DIAGNOSTIC: &str = "PF_TIMOSHENKO_REALIZED_ARCS_EULER_BERNOULLI";
/// The id infix of the projection's limitation entries.
pub const LIMITATION_INFIX: &str = ":pf-timoshenko:";

/// Whether `id` is the pressure-free Timoshenko identity.
pub fn is_pf_timoshenko_identity(id: &str) -> bool {
    id == PREVIEW_PHYSICS_TIMOSHENKO_ID
}

/// Whether a limitation entry id is a projection entry (any run).
pub fn is_pf_timoshenko_limitation_id(id: &str) -> bool {
    id.starts_with("limitation:") && id.contains(LIMITATION_INFIX)
}

/// The one expected limitation entry for `run`: the identity's limitation 0,
/// as the source publishes it and as its table states it (the reader admitted
/// the complete identity; both are required here), without the `effect` and
/// `provenance` it shares with the technical-preview entry.
pub fn expected_limitation(source: &Value, run: &str) -> Result<Value, String> {
    let identity = source.pointer("/producer/semantic_contract_id").and_then(Value::as_str);
    if identity != Some(PREVIEW_PHYSICS_TIMOSHENKO_ID) {
        return Err(mismatch("the source is not a pressure-free Timoshenko result"));
    }
    let published = source.pointer("/formulation_basis/limitations/0").and_then(Value::as_str);
    let stated = preview_physics_timoshenko_contract()["supported_profile_limitations"][0].as_str();
    let statement = match (published, stated) {
        (Some(published), Some(stated)) if published == stated => published,
        _ => return Err(mismatch("the source's limitation 0 is not the identity's")),
    };
    Ok(json!({
        "limitation_id": format!("limitation:{run}{LIMITATION_INFIX}1"),
        "source": {"ref_type": "semantic_contract", "ref_id": PREVIEW_PHYSICS_TIMOSHENKO_ID},
        "affected_scope": {"ref_type": "analysis_run", "ref_id": run},
        "statement": statement,
    }))
}

/// The report's limitation entries for a Timoshenko source: the
/// technical-preview entry of `run` first, then exactly the expected entry,
/// sharing the technical-preview entry's effect and provenance and carrying no
/// other member. `Err` names the first difference. The package's item 9 and the
/// native render use it.
pub fn check_limitation_entries(source: &Value, run: &str, limitations: &[Value]) -> Result<(), String> {
    let expected = expected_limitation(source, run)?;
    let Some((preview, rest)) = limitations.split_first() else {
        return Err(mismatch("limitations: the technical-preview entry is missing"));
    };
    if preview["limitation_id"].as_str() != Some(format!("limitation:{run}:technical-preview").as_str()) {
        return Err(mismatch("limitations: the technical-preview entry is not first"));
    }
    let [item] = rest else {
        return Err(mismatch("limitations: the report does not carry exactly the identity's limitation 0 after the technical-preview entry"));
    };
    let equal = ["limitation_id", "source", "affected_scope", "statement"].iter().all(|field| item[*field] == expected[*field])
        && item["effect"] == preview["effect"]
        && item["provenance"] == preview["provenance"]
        && item.as_object().is_some_and(|object| object.len() == 6);
    if !equal {
        return Err(mismatch(format!("limitations: {}", expected["limitation_id"].as_str().unwrap_or_default())));
    }
    Ok(())
}

/// Any other identity carries no projection entry.
pub fn check_no_limitation_entries(limitations: &[Value]) -> Result<(), String> {
    match limitations.iter().filter_map(|item| item["limitation_id"].as_str()).find(|id| is_pf_timoshenko_limitation_id(id)) {
        Some(_) => Err(mismatch("limitations: the result is not a pressure-free Timoshenko result")),
        None => Ok(()),
    }
}

/// Item 9's Timoshenko branch: the report's Diagnostics are the source's
/// (`bindings::check_report_diagnostics`, so the arc diagnostic's text and
/// references are the result's) and its limitation entries are the identity's.
/// Any other identity carries no projection entry.
pub fn check_sections(source: &Value, report: &Value) -> Result<(), String> {
    let limitations = report.pointer("/report_sections/limitations").and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
    let identity = source.pointer("/producer/semantic_contract_id").and_then(Value::as_str);
    if !identity.is_some_and(is_pf_timoshenko_identity) {
        return check_no_limitation_entries(limitations);
    }
    crate::bindings::check_report_diagnostics(report, source).map_err(|detail| mismatch(format!("diagnostics: {detail}")))?;
    let run = source["run_id"].as_str().filter(|run| !run.is_empty()).ok_or_else(|| mismatch("the source names no run"))?;
    check_limitation_entries(source, run, limitations)
}

fn mismatch(what: impl std::fmt::Display) -> String {
    format!("{PF_TIMOSHENKO_SECTIONS_MISMATCH}: {what}")
}

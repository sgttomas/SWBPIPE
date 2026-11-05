//! Operating-case slice 1b (DEL-05-02 Work `operating-case.md` §1.2 and "As built
//! (1b)"; DEL-02-01 Work `model-document-0.9.0.md` §2.2): a declared operating
//! comparison (`result_state_subtraction` with `difference_basis =
//! "operating_minus_sustained"`) on preview-physics-2, its gates G-1
//! (`EXPANSION_BASIS_SUSTAINED_NOT_IN_OPERATING`) and G-2 (`EXPANSION_BASIS_MODULUS_MIXED`),
//! its rows' sign convention and its G-1 derivation evidence
//! (`contract_evidence.difference_basis_derivation`). The Python mirror is
//! `tests/verifiers/preview_difference_basis.py`.
//!
//! - **With the model.** For each declared subtraction the model admits (S1's interim
//!   `minuend_final` withholding removed first), G-1 is re-derived from the model's own
//!   primitive loads, after the member passes the same shape, count-domain, binding and
//!   recurrence checks as the model-free reading (so a malformed member is refused by its
//!   shape code before any comparison): each load's category, target, direction, dimension, and magnitude
//!   normalized by the shared units authority exactly as the solve's
//!   `normalize_model_units` does (value bits and canonical unit). The published
//!   evidence must equal that, record for record (`…_DIFFERENCE_BASIS_MODEL`), and the
//!   gate must be G-1's, else G-2's from the cases' modulus-basis keys. The reader has no
//!   condition resolver (decision M1): when either operand case carries a
//!   `condition_use`, whose generated primitives the solve's lowering adds, G-1 cannot be
//!   re-derived and the document is refused by name
//!   (`SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_UNVERIFIED`); it never falls back to the
//!   model-free reading.
//! - **Without the model** (option A, Agent 0's choice; enabled after its independent
//!   contract review). Any declared-subtraction claim (the evidence member, a gate reason
//!   of G-1 or G-2, or a row stating the declared sign convention) is read from the
//!   evidence: the evidence's closed shape, canonical order, counts (an integer token from 1 to
//!   2^53 - 1, [`count_of`]) and bindings are checked, and a case id recurring across
//!   records must carry byte-identical contents (`…_DIFFERENCE_BASIS_BINDING`); G-1
//!   is re-derived from its two multisets, G-2 from the published
//!   `combination_modulus_basis_record` labels, and the gate must be exactly the derived
//!   one. **Scope:** this checks the record's internal derivation and its binding to
//!   the document. It does not establish that the evidence is true to the model, which
//!   the reader does not have: a coherent forgery (evidence, gate and rows all altered
//!   together) cannot be authenticated without the model.
use crate::combination_graph as graph;
use open_pipe_stress_units::{canonical_unit, Dimension};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap, HashSet};

type Check = Result<(), String>;

/// The evidence member.
pub(crate) const MEMBER: &str = "difference_basis_derivation";
pub(crate) const SUSTAINED_NOT_IN_OPERATING: &str = "EXPANSION_BASIS_SUSTAINED_NOT_IN_OPERATING";
pub(crate) const MODULUS_MIXED: &str = "EXPANSION_BASIS_MODULUS_MIXED";
/// The producer's `difference_basis::SIGN`.
pub(crate) const SIGN: &str = "positive value follows explicit user result-state subtraction (minuend minus subtrahend) of matching source result sign conventions; operating minus sustained difference: a signed difference of two independently solved states, each using its own reported support state and carrying its own numerical standing under the static, history-free friction model; derived from case results; not an equilibrium state";
const TOKEN: &str = "operating_minus_sustained";
const SUBTRACTION_BASIS: &str = "explicit_user_result_state_subtraction";
const MODULUS_RECORD: &str = "combination_modulus_basis_record";
const UNVERIFIED: &str = "SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_UNVERIFIED";
const GATE_DERIVATION: &str = "SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_DERIVATION";
const EVIDENCE: &str = "SOURCE_PREVIEW_PHYSICS_DIFFERENCE_BASIS_EVIDENCE";
const BINDING: &str = "SOURCE_PREVIEW_PHYSICS_DIFFERENCE_BASIS_BINDING";
const ROW: &str = "SOURCE_PREVIEW_PHYSICS_DIFFERENCE_BASIS_ROW";
const MODEL: &str = "SOURCE_PREVIEW_PHYSICS_DIFFERENCE_BASIS_MODEL";

fn fail(code: &str) -> String {
    code.to_string()
}
fn require(ok: bool, code: &str) -> Check {
    if ok {
        Ok(())
    } else {
        Err(fail(code))
    }
}
fn items(v: &Value) -> &[Value] {
    v.as_array().map(Vec::as_slice).unwrap_or(&[])
}
fn exact_keys(value: &Value, keys: &[&str]) -> bool {
    value.as_object().is_some_and(|o| o.len() == keys.len() && keys.iter().all(|k| o.contains_key(*k)))
}
fn combination_of(row: &Value) -> Option<&str> {
    (row["basis_ref"]["ref_type"] == "combination").then(|| row["basis_ref"]["ref_id"].as_str()).flatten()
}
fn is_gate_code(code: Option<&str>) -> bool {
    matches!(code, Some(SUSTAINED_NOT_IN_OPERATING | MODULUS_MIXED))
}

/// One primitive's content key, in the canonical sort's order: category, target type,
/// target ids (node; pipe; support then DOF; an absent second id is empty), direction,
/// dimension, value bits (16 lowercase hexadecimal digits), unit; compared byte-wise.
type Key = (String, String, String, String, String, String, String, String);
type Multiset = BTreeMap<Key, u64>;

fn contained(minuend: &Multiset, subtrahend: &Multiset) -> bool {
    subtrahend.iter().all(|(key, count)| minuend.get(key).is_some_and(|m| m >= count))
}

fn target_key(target: &Value) -> Option<(String, String, String)> {
    let s = |k: &str| target[k].as_str().map(str::to_string);
    match target["type"].as_str()? {
        "node" if exact_keys(target, &["type", "node"]) => Some(("node".into(), s("node")?, String::new())),
        "element" if exact_keys(target, &["type", "pipe"]) => Some(("element".into(), s("pipe")?, String::new())),
        "support" if exact_keys(target, &["type", "support", "dof"]) => Some(("support".into(), s("support")?, s("dof")?)),
        _ => None,
    }
}

fn target_json(key: &Key) -> Value {
    match key.1.as_str() {
        "support" => json!({"type": "support", "support": key.2, "dof": key.3}),
        "element" => json!({"type": "element", "pipe": key.2}),
        _ => json!({"type": "node", "node": key.2}),
    }
}

fn bits_of(text: &str) -> Option<u64> {
    (text.len() == 16 && text.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))).then(|| u64::from_str_radix(text, 16).ok()).flatten()
}

/// The count domain: a JSON integer token from 1 to 2^53 - 1. The producer counts a case's
/// primitive loads (`usize`), and a published statement is a checked-JSON document
/// (`openpipestress_jcs_ijson_v1`: integers within +-(2^53 - 1), `MAX_SAFE_INTEGER`), so no
/// larger count can be published; a float token (`1.0`), a boolean, a string or a negative
/// number is not a count. The Python mirror states the same domain.
fn count_of(value: &Value) -> Option<u64> {
    value.as_u64().filter(|c| value.is_u64() && (1..=open_pipe_stress_canonical_json::MAX_SAFE_INTEGER as u64).contains(c))
}

/// One operand's published multiset: closed entries, strictly increasing in the
/// canonical order (so each content once), counts in the count domain, finite values.
fn read_multiset(entries: &Value) -> Result<Multiset, String> {
    let list = entries.as_array().ok_or_else(|| fail(EVIDENCE))?;
    let mut out = Multiset::new();
    let mut last: Option<Key> = None;
    for entry in list {
        require(exact_keys(entry, &["category", "target", "direction", "dimension", "magnitude", "count"]), EVIDENCE)?;
        let text = |k: &str| entry[k].as_str().map(str::to_string).ok_or_else(|| fail(EVIDENCE));
        let (kind, a, b) = target_key(&entry["target"]).ok_or_else(|| fail(EVIDENCE))?;
        let magnitude = &entry["magnitude"];
        require(exact_keys(magnitude, &["value_bits", "unit"]), EVIDENCE)?;
        let hex = magnitude["value_bits"].as_str().ok_or_else(|| fail(EVIDENCE))?;
        let bits = bits_of(hex).ok_or_else(|| fail(EVIDENCE))?;
        require(f64::from_bits(bits).is_finite(), EVIDENCE)?;
        let unit = magnitude["unit"].as_str().ok_or_else(|| fail(EVIDENCE))?.to_string();
        let count = count_of(&entry["count"]).ok_or_else(|| fail(EVIDENCE))?;
        let key: Key = (text("category")?, kind, a, b, text("direction")?, text("dimension")?, hex.to_string(), unit);
        require(last.as_ref().is_none_or(|l| *l < key), EVIDENCE)?;
        last = Some(key.clone());
        out.insert(key, count);
    }
    Ok(out)
}

fn multiset_json(multiset: &Multiset) -> Value {
    Value::Array(
        multiset
            .iter()
            .map(|(key, count)| {
                json!({"category": key.0, "target": target_json(key), "direction": key.4, "dimension": key.5,
                       "magnitude": {"value_bits": key.6, "unit": key.7}, "count": count})
            })
            .collect(),
    )
}

/// One evidence record, read.
struct Record<'a> {
    id: &'a str,
    minuend: &'a str,
    subtrahend: &'a str,
    minuend_contents: Multiset,
    subtrahend_contents: Multiset,
}

/// The member, read and bound: closed records in gate order, one per combination
/// id, each naming a gate and two distinct published load cases.
fn read(source: &Value) -> Result<Vec<Record<'_>>, String> {
    let evidence = &source["contract_evidence"];
    let Some(member) = evidence.get(MEMBER) else { return Ok(Vec::new()) };
    let list = member.as_array().filter(|l| !l.is_empty()).ok_or_else(|| fail(EVIDENCE))?;
    let gates = items(&evidence["combination_gates"]);
    let position: HashMap<&str, usize> = gates.iter().enumerate().filter_map(|(i, g)| g["combination_id"].as_str().map(|id| (id, i))).collect();
    let cases: HashSet<&str> = items(&evidence["preview_cases"]).iter().filter_map(|c| c["load_case_id"].as_str()).collect();
    let mut records = Vec::new();
    let mut last: Option<usize> = None;
    for record in list {
        require(exact_keys(record, &["combination_id", "difference_basis", "minuend", "subtrahend"]) && record["difference_basis"] == TOKEN, EVIDENCE)?;
        let id = record["combination_id"].as_str().filter(|s| !s.is_empty()).ok_or_else(|| fail(EVIDENCE))?;
        let at = *position.get(id).ok_or_else(|| fail(BINDING))?;
        require(last.is_none_or(|l| at > l), EVIDENCE)?;
        last = Some(at);
        let operand = |key: &str| -> Result<(&str, Multiset), String> {
            let side = &record[key];
            require(exact_keys(side, &["load_case_id", "primitive_contents"]), EVIDENCE)?;
            let case = side["load_case_id"].as_str().filter(|c| cases.contains(c)).ok_or_else(|| fail(BINDING))?;
            Ok((case, read_multiset(&side["primitive_contents"])?))
        };
        let (minuend, minuend_contents) = operand("minuend")?;
        let (subtrahend, subtrahend_contents) = operand("subtrahend")?;
        require(minuend != subtrahend, BINDING)?;
        records.push(Record { id, minuend, subtrahend, minuend_contents, subtrahend_contents });
    }
    // Review r1 P1: one producer resolution gives each case one multiset, so a case id that
    // recurs (in any record, as minuend or subtrahend) carries byte-identical contents: the
    // same JSON with members sorted (serde_json's map), as the Python mirror compares it.
    let mut bound: HashMap<&str, String> = HashMap::new();
    for record in list {
        for side in ["minuend", "subtrahend"] {
            let case = record[side]["load_case_id"].as_str().unwrap_or_default();
            let bytes = serde_json::to_string(&record[side]["primitive_contents"]).map_err(|_| fail(EVIDENCE))?;
            if bound.entry(case).or_insert_with(|| bytes.clone()) != &bytes {
                return Err(fail(BINDING));
            }
        }
    }
    Ok(records)
}

/// The gate a declared subtraction's verdict must be: G-1, then G-2.
fn derived_gate(g1_contained: bool, same_modulus: bool) -> Option<&'static str> {
    if !g1_contained {
        Some(SUSTAINED_NOT_IN_OPERATING)
    } else if !same_modulus {
        Some(MODULUS_MIXED)
    } else {
        None
    }
}

fn gate_of<'a>(gates: &'a [Value], id: &str) -> Option<&'a Value> {
    gates.iter().find(|g| g["combination_id"] == id)
}

fn require_gate(gate: Option<&Value>, derived: Option<&str>) -> Check {
    require(gate.is_some_and(|g| g["withheld"] == derived.is_some() && g["reason"].as_str() == derived), GATE_DERIVATION)
}

/// The rows' sign conventions: an admitted declared subtraction's rows that state one
/// state [`SIGN`] (and the subtraction basis); no other combination's row does. A
/// `minuend_final` subtraction's rows state the held sign and its held-operand rows their
/// clause, which `preview_held_status` checks.
fn rows_sign(rows: &[Value], declared: &HashSet<&str>, held: &HashSet<&str>) -> Check {
    for row in rows {
        let Some(id) = combination_of(row) else { continue };
        if row["kind"] == MODULUS_RECORD {
            continue;
        }
        let sign = row["metadata"]["sign_convention"].as_str();
        if held.contains(id) {
            require(sign != Some(SIGN), ROW)?;
        } else if declared.contains(id) {
            require(sign.is_none() || (sign == Some(SIGN) && row["metadata"]["basis"] == SUBTRACTION_BASIS), ROW)?;
        } else {
            require(sign != Some(SIGN), ROW)?;
        }
    }
    Ok(())
}

/// Entry from `preview_combination_gates::check` on a solved document.
pub(crate) fn check(source: &Value, model: Option<&Value>) -> Check {
    match model {
        Some(model) => model_check(source, model),
        None => free_check(source),
    }
}

fn free_check(source: &Value) -> Check {
    let evidence = &source["contract_evidence"];
    let gates = items(&evidence["combination_gates"]);
    let rows = items(&source["results"]);
    let claimed = evidence.get(MEMBER).is_some()
        || gates.iter().any(|g| is_gate_code(g["reason"].as_str()))
        || rows.iter().any(|r| combination_of(r).is_some() && r["metadata"]["sign_convention"] == SIGN);
    if !claimed {
        return Ok(());
    }
    let records = read(source)?;
    let declared: HashSet<&str> = records.iter().map(|r| r.id).collect();
    // Every declared claim has its record.
    for gate in gates {
        if is_gate_code(gate["reason"].as_str()) {
            require(gate["combination_id"].as_str().is_some_and(|id| declared.contains(id)), EVIDENCE)?;
        }
    }
    for row in rows {
        if let (Some(id), true) = (combination_of(row), row["metadata"]["sign_convention"] == SIGN) {
            require(declared.contains(id), EVIDENCE)?;
        }
    }
    let by_id: HashMap<&str, &Value> = rows.iter().filter_map(|r| r["id"].as_str().map(|id| (id, r))).collect();
    for record in &records {
        // Bindings the document states: the modulus records name exactly the two
        // operands, minuend first; every source row is one of theirs, minuend first;
        // the operand and derivation records, where present, state the same expression.
        let modulus: Vec<(&str, &str)> = rows
            .iter()
            .filter(|r| r["kind"] == MODULUS_RECORD && combination_of(r) == Some(record.id))
            .map(|r| (r["entity_ref"].as_str().unwrap_or_default(), r["metadata"]["basis"].as_str().unwrap_or_default()))
            .collect();
        require(modulus.len() == 2 && modulus[0].0 == record.minuend && modulus[1].0 == record.subtrahend, BINDING)?;
        for row in rows.iter().filter(|r| combination_of(r) == Some(record.id) && r["kind"] != MODULUS_RECORD) {
            // A source row is an operand case's, or (a recomputed magnitude's
            // components) this combination's own.
            let sources: Vec<&Value> = items(&row["source_result_refs"]).iter().map(|r| r.as_str().and_then(|r| by_id.get(r)).copied().ok_or_else(|| fail(BINDING))).collect::<Result<_, _>>()?;
            let owners: Vec<Option<&str>> = sources
                .iter()
                .filter(|s| combination_of(s) != Some(record.id))
                .map(|s| (s["basis_ref"]["ref_type"] == "load_case").then(|| s["basis_ref"]["ref_id"].as_str()).flatten())
                .collect();
            require(owners.iter().all(|o| matches!(o, Some(c) if *c == record.minuend || *c == record.subtrahend)), BINDING)?;
            if owners.len() == 2 {
                require(owners[0] == Some(record.minuend) && owners[1] == Some(record.subtrahend), BINDING)?;
            }
        }
        let expression = json!({"minuend_id": record.minuend, "subtrahend_id": record.subtrahend});
        for member in ["combination_operands", crate::preview_combination_gates::DERIVATION_MEMBER] {
            let list = match evidence.get(member) {
                Some(Value::Object(o)) => o.get("combinations").map(items).unwrap_or(&[]),
                Some(other) => items(other),
                None => &[],
            };
            for stated in list.iter().filter(|r| r["combination_id"] == record.id) {
                require(stated["basis"] == "result_state_subtraction" && stated["expression"] == expression, BINDING)?;
            }
        }
        let derived = derived_gate(contained(&record.minuend_contents, &record.subtrahend_contents), modulus[0].1 == modulus[1].1);
        // RT-A5 (§7.5): past G-1 and G-2 a `minuend_final` subtraction may be withheld by the
        // held gate its record states (`preview_held_status`).
        let derived = derived.or_else(|| crate::preview_held_status::stated_gate(source, record.id));
        require_gate(gate_of(gates, record.id), derived)?;
    }
    rows_sign(rows, &declared, &held_ids(source))
}

/// The combinations the held-status evidence records (`preview_held_status`).
fn held_ids(source: &Value) -> HashSet<&str> {
    items(source["contract_evidence"].get(crate::preview_held_status::MEMBER).unwrap_or(&Value::Null))
        .iter()
        .filter_map(|r| r["combination_id"].as_str())
        .collect()
}

/// The dimension a load's `dimension` names (the producer's `expected_load_dimension`).
fn load_dimension(text: &str) -> Option<Dimension> {
    Some(match text {
        "force" => Dimension::Force,
        "moment" => Dimension::Moment,
        "force_per_length" => Dimension::ForcePerLength,
        "pressure" => Dimension::Pressure,
        "temperature_change" | "temperature_interval" => Dimension::TemperatureInterval,
        "acceleration" => Dimension::Acceleration,
        "displacement" => Dimension::Displacement,
        "rotation" => Dimension::Rotation,
        _ => return None,
    })
}

/// A model load case's primitive loads as the solve's G-1 reads them, from the authored
/// model (no `condition_use`): the magnitude normalized as `normalize_model_units` does.
fn model_multiset(case: &Value) -> Result<Multiset, String> {
    let mut out = Multiset::new();
    for load in items(&case["primitive_loads"]) {
        let text = |k: &str| load[k].as_str().map(str::to_string).ok_or_else(|| fail(MODEL));
        let (kind, a, b) = target_key(&load["target"]).ok_or_else(|| fail(MODEL))?;
        let dimension = text("dimension")?;
        let magnitude = &load["magnitude"];
        let (value, unit) = match load_dimension(&dimension) {
            Some(d) => {
                let value = crate::physics_source::normalized_quantity(magnitude, d).map_err(|_| fail(MODEL))?;
                (value, canonical_unit(d).ok_or_else(|| fail(MODEL))?.definition().symbol.to_string())
            }
            None => (magnitude["value"].as_f64().ok_or_else(|| fail(MODEL))?, magnitude["unit"].as_str().ok_or_else(|| fail(MODEL))?.to_string()),
        };
        require(value.is_finite(), MODEL)?;
        let key: Key = (text("category")?, kind, a, b, text("direction")?, dimension, format!("{:016x}", value.to_bits()), unit);
        *out.entry(key).or_insert(0) += 1;
    }
    Ok(out)
}

/// The combinations the solve gates, in evaluation order. S1's interim withholding removed
/// a `minuend_final` combination and every combination naming one until RT-A5
/// (operating-case §7.5), which gates it at the solve: every combination is kept.
pub(crate) fn kept_combinations(model: &Value) -> (Vec<Value>, Vec<usize>) {
    let cases: HashSet<&str> = items(&model["load_cases"]).iter().filter_map(|c| c["id"].as_str()).collect();
    let kept: Vec<Value> = items(&model["combinations"]).to_vec();
    let order = graph::model_order(&kept, &cases);
    (kept, order)
}

/// Whether a model combination declares the operating comparison.
pub(crate) fn declared(combination: &Value) -> bool {
    combination["basis"] == "result_state_subtraction" && combination["difference_basis"] == TOKEN
}

/// A declared subtraction's gate from the model (G-1 from its primitives, G-2 from the
/// modulus keys), with its evidence record; refused when an operand uses conditions.
pub(crate) fn model_gate(model: &Value, combination: &Value) -> Result<(Option<&'static str>, Value), String> {
    let case = |key: &str| items(&model["load_cases"]).iter().find(|c| c["id"] == combination[key]).ok_or_else(|| fail(MODEL));
    let (minuend, subtrahend) = (case("minuend_id")?, case("subtrahend_id")?);
    if [minuend, subtrahend].iter().any(|c| c.get("condition_use").is_some_and(|u| !u.is_null())) {
        return Err(fail(UNVERIFIED));
    }
    let (m, s) = (model_multiset(minuend)?, model_multiset(subtrahend)?);
    let same_modulus = crate::preview_combination_gates::modulus_key(minuend)? == crate::preview_combination_gates::modulus_key(subtrahend)?;
    let record = json!({"combination_id": combination["id"], "difference_basis": TOKEN,
        "minuend": {"load_case_id": minuend["id"], "primitive_contents": multiset_json(&m)},
        "subtrahend": {"load_case_id": subtrahend["id"], "primitive_contents": multiset_json(&s)}});
    Ok((derived_gate(contained(&m, &s), same_modulus), record))
}

fn model_check(source: &Value, model: &Value) -> Check {
    // The member's closed shape, count domain, bindings and case recurrence, by the one
    // validator the model-free reading uses, before any comparison with the model's.
    read(source)?;
    let evidence = &source["contract_evidence"];
    let gates = items(&evidence["combination_gates"]);
    let (kept, order) = kept_combinations(model);
    let mut expected = Vec::new();
    let mut declared_ids: HashSet<&str> = HashSet::new();
    for &index in &order {
        let combination = &kept[index];
        if !declared(combination) {
            continue;
        }
        let (derived, record) = model_gate(model, combination)?;
        let id = combination["id"].as_str().unwrap_or_default();
        let derived = derived.or_else(|| crate::preview_held_status::minuend_final(combination).then(|| crate::preview_held_status::stated_gate(source, id)).flatten());
        require_gate(gate_of(gates, id), derived)?;
        expected.push(record);
        if let Some(gate) = gate_of(gates, id) {
            declared_ids.extend(gate["combination_id"].as_str());
        }
    }
    let stated = evidence.get(MEMBER);
    require(
        match stated {
            None => expected.is_empty(),
            Some(stated) => !expected.is_empty() && *stated == Value::Array(expected),
        },
        MODEL,
    )?;
    rows_sign(items(&source["results"]), &declared_ids, &held_ids(source))
}

#[cfg(test)]
#[path = "preview_difference_basis_tests.rs"]
mod corpus_tests;

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(bits: u64, count: u64) -> Value {
        json!({"category": "weight", "target": {"type": "node", "node": "n"}, "direction": "-Y", "dimension": "force",
               "magnitude": {"value_bits": format!("{bits:016x}"), "unit": "N"}, "count": count})
    }

    #[test]
    fn multisets_are_closed_ordered_and_counted() {
        let two = 2f64.to_bits();
        assert!(read_multiset(&json!([entry(two, 1), entry(two + 1, 2)])).is_ok());
        for bad in [
            json!([entry(two + 1, 1), entry(two, 1)]), // out of canonical order
            json!([entry(two, 1), entry(two, 1)]),     // a content twice
            json!([entry(two, 0)]),                    // a zero count
            json!([entry(f64::INFINITY.to_bits(), 1)]),
        ] {
            assert_eq!(read_multiset(&bad).unwrap_err(), EVIDENCE, "{bad}");
        }
        let mut float_count = entry(two, 1);
        float_count["count"] = json!(1.0);
        let tenth = 0.1f64.to_bits();
        let mut upper = entry(tenth, 1);
        upper["magnitude"]["value_bits"] = json!(format!("{tenth:016X}"));
        let mut extra = entry(two, 1);
        extra["id"] = json!("x");
        for bad in [float_count, upper, extra] {
            assert_eq!(read_multiset(&json!([bad])).unwrap_err(), EVIDENCE);
        }
        let one = read_multiset(&json!([entry(two, 1)])).unwrap();
        let twice = read_multiset(&json!([entry(two, 2)])).unwrap();
        assert!(contained(&twice, &one) && !contained(&one, &twice));
    }
}

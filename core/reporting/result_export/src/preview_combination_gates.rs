//! RT-A4 (DEL-05-02 Work `absolute-sum-srss-combinations.md` §4-§5, §10; review r1
//! P2; §10.3): preview-physics-2's combination gates and unsigned rows, re-derived from
//! the received document and, when it is supplied, from its model. The Python mirror is
//! `tests/verifiers/preview_combination_gates.py`.
//!
//! - **Model-free** (`check(source, None)`). The document's `combination_derivation`
//!   (§10.3) states the model conditions the gates read and, in gate order, every
//!   absolute sum's, SRSS's and nested combination's expression with those of the
//!   combinations it reaches. Its shape, order and closure are checked; from it κ and
//!   decision OQ-1 are re-derived, so each recorded combination's withheld-operand,
//!   nonlinear and κ gate is checked exactly (the modulus gate, which reads the load
//!   cases' modulus bases, is admitted as stated, as for flat combinations), and every
//!   unsigned row is A or R of the recorded factors and source rows, bit for bit.
//!   `LOAD_COMBINATION_OPERAND_WITHHELD` also needs its diagnostic naming earlier
//!   withheld combinations. A κ withholding (`CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND`,
//!   `CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED`) without its record is refused as
//!   unverifiable (`..._COMBINATION_GATE_UNVERIFIED`), as is a row citing an operand
//!   combination's row without its record; an unsigned row without its record is refused
//!   (`..._COMBINATION_UNSIGNED_UNVERIFIED`). The flat gates keep the model-free reading
//!   they had (the shared retained corpus's `t_gate_ok` states a nonlinear gate on a
//!   document with no nonlinear row and must pass).
//! - **With the model** (`check(source, Some(model))`). The published derivation must be
//!   exactly the model's (`..._COMBINATION_DERIVATION_EVIDENCE`). Every gate, in the
//!   model's evaluation order after S1's interim removal, is re-derived as the producer
//!   forms it (`preview_physics.rs` `gate_reason`): a withheld operand combination; then,
//!   for the term bases, nonlinear supports (except decision OQ-1), κ where a consuming
//!   constant-effort support applies, and the modulus bases of every load case reached.
//!   Every row of an absolute sum or SRSS states its basis and sign convention, and each
//!   row formed from operand rows is A or R of the model's factors, bit for bit.
//! - **Whole rows** (review r2 P2; both readings). Every row of an admitted absolute sum
//!   or SRSS (flat or nested), and of an admitted mechanics combination, subtraction or
//!   envelope with a combination operand, is re-derived from its operands' rows with the
//!   model's or the record's factors, as the producer forms it (`form`: nested pairing
//!   with a combination operand, flat pairing otherwise), and the published rows and
//!   withholdings must be exactly the derived ones (`..._NESTED_ROW`, `..._NESTED_WITHHELD`,
//!   so named for the nested case they were introduced for). Flat mechanics,
//!   subtractions and envelopes keep their pre-RT-A4 reading (signed values not re-derived).
use crate::combination_graph::{self as graph, Expression, Graph, Operand};
use open_pipe_stress_correct_norm::norm3;
use open_pipe_stress_load_case_algebra::AlgebraDimension as LoadDimension;
use open_pipe_stress_load_case_algebra::{mechanics_fixed_load_gate, unsigned_fixed_load_gate, FixedLoadCount, FixedLoadWithholding};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap, HashSet};

type Check = Result<(), String>;

const NONLINEAR: &str = "NONLINEAR_COMBINATION_REQUIRES_SOLVE";
const CONSTANT_EFFORT: &str = "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE";
const CONSTANT_EFFORT_UNSIGNED: &str = "CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND";
const CONSTANT_EFFORT_MIXED: &str = "CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED";
const MODULUS_MIXED: &str = "COMBINATION_MODULUS_BASIS_MIXED";
const STATE_CODE: &str = "nonlinear_support_active_set_state_code";
const SUPPORT_FORCE: &str = "support_reaction_force_magnitude_v2";
const SUPPORT_MOMENT: &str = "support_reaction_moment_magnitude_v2";
const MODULUS_RECORD: &str = "combination_modulus_basis_record";
/// §10.3: the evidence member a reader without the model re-derives from.
pub(crate) const DERIVATION_MEMBER: &str = "combination_derivation";
const DERIVATION_KEYS: [&str; 3] = ["nonlinear_supports", "consuming_constant_effort", "combinations"];
const TRANSLATIONS: [&str; 9] = ["UX", "Ux", "ux", "UY", "Uy", "uy", "UZ", "Uz", "uz"];
const ROTATIONS: [&str; 9] = ["RX", "Rx", "rx", "RY", "Ry", "ry", "RZ", "Rz", "rz"];

fn require(ok: bool, code: &str) -> Check {
    if ok {
        Ok(())
    } else {
        Err(format!("SOURCE_PREVIEW_PHYSICS_COMBINATION_{code}"))
    }
}
fn items(v: &Value) -> &[Value] {
    v.as_array().map(Vec::as_slice).unwrap_or(&[])
}
fn combination_of(row: &Value) -> Option<&str> {
    (row["basis_ref"]["ref_type"] == "combination").then(|| row["basis_ref"]["ref_id"].as_str()).flatten()
}

/// The unsigned basis (`absolute_sum` or `srss`) a row states by its metadata basis or
/// its sign convention, and whether it states both consistently.
fn unsigned_claim(row: &Value) -> Option<(&'static str, bool)> {
    let basis = row["metadata"]["basis"].as_str();
    let sign = row["metadata"]["sign_convention"].as_str();
    let by_basis = match basis {
        Some(graph::ABSOLUTE_SUM_BASIS) => Some("absolute_sum"),
        Some(graph::SRSS_BASIS) => Some("srss"),
        _ => None,
    };
    let by_sign = match sign {
        Some(graph::ABSOLUTE_SUM_SIGN) => Some("absolute_sum"),
        Some(graph::SRSS_SIGN) => Some("srss"),
        _ => None,
    };
    let claimed = by_basis.or(by_sign)?;
    Some((claimed, by_basis == by_sign))
}

/// A combination's terms, by operand kind, as `(operand, factor)`.
type Terms<'a> = Vec<(Operand<'a>, f64)>;

/// The direct (not recomputed-magnitude) rows of one unsigned combination: each names
/// one source row per term, the term's operand's own row of the same quantity, and is A
/// or R of the factors times those source values, bit for bit.
fn unsigned_rows(rows: &[Value], by_id: &HashMap<&str, &Value>, id: &str, basis: &str, terms: &Terms<'_>) -> Check {
    let srss = basis == "srss";
    let factors: Vec<f64> = terms.iter().map(|(_, f)| *f).collect();
    for row in rows.iter().filter(|r| combination_of(r) == Some(id)) {
        let kind = row["kind"].as_str().unwrap_or_default();
        if kind == MODULUS_RECORD {
            continue;
        }
        let (sign, basis_text) = if srss { (graph::SRSS_SIGN, graph::SRSS_BASIS) } else { (graph::ABSOLUTE_SUM_SIGN, graph::ABSOLUTE_SUM_BASIS) };
        require(row["metadata"]["basis"] == basis_text && row["metadata"]["sign_convention"] == sign, "UNSIGNED_ROW")?;
        if kind == "displacement_magnitude" || kind == SUPPORT_FORCE || kind == SUPPORT_MOMENT {
            continue; // The norm of the combined components (`combination_magnitudes`).
        }
        let refs = items(&row["source_result_refs"]);
        require(row["source_result_refs"].is_array() && refs.len() == terms.len(), "UNSIGNED_SOURCES")?;
        let mut values = Vec::new();
        for (reference, (operand, _)) in refs.iter().zip(terms) {
            let source = reference.as_str().and_then(|r| by_id.get(r)).ok_or_else(|| "SOURCE_PREVIEW_PHYSICS_COMBINATION_UNSIGNED_SOURCES".to_string())?;
            require(
                operand.owns(source)
                    && source["kind"] == row["kind"]
                    && source["entity_ref"] == row["entity_ref"]
                    && source["unit"] == row["unit"]
                    && source["metadata"]["component"] == row["metadata"]["component"]
                    && source["metadata"]["location"] == row["metadata"]["location"],
                "UNSIGNED_SOURCES",
            )?;
            values.push(source["value"].as_f64().ok_or_else(|| "SOURCE_PREVIEW_PHYSICS_COMBINATION_UNSIGNED_SOURCES".to_string())?);
        }
        let expected = graph::unsigned_value(srss, &factors, &values).ok_or_else(|| "SOURCE_PREVIEW_PHYSICS_COMBINATION_UNSIGNED_VALUE".to_string())?;
        require(row["value"].as_f64().is_some_and(|v| v.to_bits() == expected.to_bits()), "UNSIGNED_VALUE")?;
    }
    Ok(())
}

/// A model's terms of a term-basis combination, by operand kind.
fn model_terms(combination: &Value) -> Terms<'_> {
    items(&combination["terms"])
        .iter()
        .filter_map(|t| {
            let factor = t["factor"].as_f64()?;
            match (t["combination"].as_str(), t["load_case"].as_str()) {
                (Some(c), _) => Some((Operand::Combination(c), factor)),
                (None, Some(case)) => Some((Operand::Case(case), factor)),
                _ => None,
            }
        })
        .collect()
}

/// The gate a combination takes before its modulus gate (`gate_reason`): a withheld
/// operand combination; nonlinear supports (`mechanics` except decision OQ-1); κ where
/// a consuming constant-effort support applies. `None` leaves only the modulus gate.
fn gate_before_modulus(basis: &str, operand_withheld: bool, nonlinear: bool, oq1: bool, consuming: bool, own: &FixedLoadCount, operand_kappa: &[FixedLoadCount]) -> Option<&'static str> {
    if operand_withheld {
        return Some(graph::OPERAND_WITHHELD);
    }
    if !graph::term_basis(basis) {
        return None;
    }
    if basis == "mechanics" && nonlinear && !oq1 {
        return Some(NONLINEAR);
    }
    let fixed = consuming
        .then(|| if basis == "mechanics" { mechanics_fixed_load_gate(own, operand_kappa.iter()) } else { unsigned_fixed_load_gate(operand_kappa.iter()) })
        .flatten();
    fixed.map(|withholding| match withholding {
        FixedLoadWithholding::UnitFactorSum => CONSTANT_EFFORT,
        FixedLoadWithholding::OperandUndefined => CONSTANT_EFFORT_MIXED,
        FixedLoadWithholding::UnsignedOperand => CONSTANT_EFFORT_UNSIGNED,
    })
}

/// One `combination_derivation` record, its operands resolved.
struct Record<'a> {
    id: &'a str,
    basis: &'a str,
    operands: Vec<Operand<'a>>,
    terms: Terms<'a>,
    /// An envelope's mode.
    mode: Option<&'a str>,
}

/// The `combination_derivation` member, read and checked (§10.3).
struct Derivation<'a> {
    nonlinear: bool,
    consuming: bool,
    records: Vec<Record<'a>>,
}

fn exact_keys(value: &Value, keys: &[&str]) -> bool {
    value.as_object().is_some_and(|o| o.len() == keys.len() && keys.iter().all(|k| o.contains_key(*k)))
}
fn non_empty(value: &Value) -> Option<&str> {
    value.as_str().filter(|s| !s.is_empty())
}

/// Reads the member: its closed shape; each record's operands resolved against the
/// document's load cases (a term by kind; a subtraction's or envelope's id the load
/// case when one has it), a load case published and a combination recorded earlier;
/// the records in gate order, once each; every record an absolute sum, an SRSS or a
/// combination with a combination operand, or an operand of a later record; agreement
/// with the `combination_operands` records; and `nonlinear_supports` stated where the
/// document shows a nonlinear support. `None` when the member is absent.
fn derivation<'a>(source: &'a Value, gates: &'a [Value]) -> Result<Option<Derivation<'a>>, String> {
    let evidence = &source["contract_evidence"];
    let Some(member) = evidence.get(DERIVATION_MEMBER) else { return Ok(None) };
    let fail = || "SOURCE_PREVIEW_PHYSICS_COMBINATION_DERIVATION_EVIDENCE".to_string();
    let check = |ok: bool| if ok { Ok(()) } else { Err(fail()) };
    check(exact_keys(member, &DERIVATION_KEYS))?;
    let nonlinear = member["nonlinear_supports"].as_bool().ok_or_else(fail)?;
    let consuming = member["consuming_constant_effort"].as_bool().ok_or_else(fail)?;
    let list = member["combinations"].as_array().filter(|l| !l.is_empty()).ok_or_else(fail)?;
    let cases: HashSet<&str> = items(&evidence["preview_cases"]).iter().filter_map(|c| c["load_case_id"].as_str()).collect();
    let resolve = |id: &'a str| if cases.contains(id) { Operand::Case(id) } else { Operand::Combination(id) };
    let position: HashMap<&str, usize> = gates.iter().enumerate().filter_map(|(i, g)| g["combination_id"].as_str().map(|id| (id, i))).collect();
    let mut records: Vec<Record<'a>> = Vec::new();
    let mut last: Option<usize> = None;
    for record in list {
        check(exact_keys(record, &["combination_id", "basis", "expression"]))?;
        let id = non_empty(&record["combination_id"]).ok_or_else(fail)?;
        let basis = record["basis"].as_str().ok_or_else(fail)?;
        let at = *position.get(id).ok_or_else(fail)?;
        check(last.is_none_or(|l| at > l))?;
        last = Some(at);
        let expression = &record["expression"];
        let mode = expression["mode"].as_str();
        let (operands, terms) = match basis {
            "mechanics" | "absolute_sum" | "srss" => {
                check(exact_keys(expression, &["terms"]))?;
                let terms: Terms<'a> = items(&expression["terms"]).iter().map(graph::evidence_term).collect::<Option<_>>().ok_or_else(fail)?;
                check(!terms.is_empty())?;
                (terms.iter().map(|(o, _)| *o).collect(), terms)
            }
            "result_state_subtraction" => {
                check(exact_keys(expression, &["minuend_id", "subtrahend_id"]))?;
                let minuend = non_empty(&expression["minuend_id"]).ok_or_else(fail)?;
                let subtrahend = non_empty(&expression["subtrahend_id"]).ok_or_else(fail)?;
                (vec![resolve(minuend), resolve(subtrahend)], Vec::new())
            }
            "range_envelope" => {
                check(exact_keys(expression, &["operand_ids", "mode"]) && mode.and_then(open_pipe_stress_load_case_algebra::RangeMode::parse_token).is_some())?;
                let mut ids: Vec<&str> = items(&expression["operand_ids"]).iter().map(non_empty).collect::<Option<_>>().ok_or_else(fail)?;
                check(!ids.is_empty())?;
                ids.sort_unstable();
                (ids.into_iter().map(resolve).collect(), Vec::new())
            }
            _ => return Err(fail()),
        };
        // The producer refuses a repeated operand in every form
        // (LOAD_COMBINATION_DUPLICATE_TERM), so a record repeating one is not its
        // output (review r3).
        check(operands.iter().enumerate().all(|(i, o)| !operands[..i].contains(o)))?;
        for operand in &operands {
            check(match operand {
                Operand::Case(case) => cases.contains(case),
                Operand::Combination(c) => records.iter().any(|r| r.id == *c),
            })?;
        }
        records.push(Record { id, basis, operands, terms, mode });
    }
    for (i, record) in records.iter().enumerate() {
        let subject = matches!(record.basis, "absolute_sum" | "srss") || record.operands.iter().any(|o| matches!(o, Operand::Combination(_)));
        let operand = records[i + 1..].iter().any(|later| later.operands.contains(&Operand::Combination(record.id)));
        check(subject || operand)?;
    }
    let operand_records = items(&evidence["combination_operands"]);
    for (stated, record) in operand_records.iter().filter_map(|o| list.iter().find(|r| r["combination_id"] == o["combination_id"]).map(|r| (o, r))) {
        check(stated["basis"] == record["basis"] && same(&stated["expression"], &record["expression"]))?;
    }
    let shows_nonlinear = !operand_records.is_empty() || items(&source["results"]).iter().any(|r| r["kind"] == STATE_CODE);
    check(nonlinear || !shows_nonlinear)?;
    Ok(Some(Derivation { nonlinear, consuming, records }))
}

/// Whether `row` (of combination `id`) cites another combination's row.
fn cites_combination(row: &Value, by_id: &HashMap<&str, &Value>, id: &str) -> bool {
    items(&row["source_result_refs"]).iter().filter_map(Value::as_str).filter_map(|r| by_id.get(r)).any(|s| combination_of(s).is_some_and(|c| c != id))
}

/// The gates and unsigned rows of a solved preview-physics-2 document; `model`, when
/// supplied, is the document's model (module doc).
pub(crate) fn check(source: &Value, model: Option<&Value>) -> Check {
    if source["status"]["mechanics"] != "MECHANICS_SOLVED" {
        return Ok(());
    }
    let evidence = &source["contract_evidence"];
    let gates = items(&evidence["combination_gates"]);
    let rows = items(&source["results"]);
    let by_id: HashMap<&str, &Value> = rows.iter().filter_map(|r| r["id"].as_str().map(|id| (id, r))).collect();
    // Operating-case slice 1b: a declared subtraction's G-1, G-2, evidence and sign.
    crate::preview_difference_basis::check(source, model)?;
    // RT-A5 (operating-case §7.6): a `minuend_final` subtraction's held gates, rows and evidence.
    crate::preview_held_status::check(source, model)?;
    match model {
        Some(model) => model_check(source, model, gates, rows, &by_id),
        None => free_check(source, gates, rows, &by_id),
    }
}

fn free_check(source: &Value, gates: &[Value], rows: &[Value], by_id: &HashMap<&str, &Value>) -> Check {
    let derivation = derivation(source, gates)?;
    let records: HashMap<&str, &Record<'_>> = derivation.iter().flat_map(|d| d.records.iter().map(|r| (r.id, r))).collect();
    let diagnostics = items(&source["diagnostics"]);
    let mut withheld: HashSet<&str> = HashSet::new();
    for gate in gates {
        let id = gate["combination_id"].as_str().unwrap_or_default();
        let Some(reason) = gate["reason"].as_str() else { continue };
        match reason {
            graph::OPERAND_WITHHELD => {
                // Its diagnostic names this combination and the withheld operands.
                let named: Vec<&Value> = diagnostics
                    .iter()
                    .filter(|d| d["code"] == graph::OPERAND_WITHHELD && items(&d["affected_refs"]).first().is_some_and(|r| r == id))
                    .collect();
                require(named.len() == 1, "GATE_DERIVATION")?;
                let operands = &items(&named[0]["affected_refs"])[1..];
                require(!operands.is_empty() && operands.iter().all(|o| o.as_str().is_some_and(|o| withheld.contains(o))), "GATE_DERIVATION")?;
            }
            CONSTANT_EFFORT_UNSIGNED | CONSTANT_EFFORT_MIXED => require(records.contains_key(id), "GATE_UNVERIFIED")?,
            _ => {}
        }
        withheld.insert(id);
    }
    // §10.3: each recorded combination's gate, up to its modulus gate, from the records.
    if let Some(derivation) = &derivation {
        let mut index: Option<RowIndex<'_>> = None;
        let mut combination_graph = Graph::default();
        for record in &derivation.records {
            let factors: Vec<f64> = record.terms.iter().map(|(_, f)| *f).collect();
            let operand_kappa: Vec<FixedLoadCount> = record.operands.iter().map(|o| combination_graph.kappa_of(o)).collect();
            let oq1 = record.basis == "mechanics" && combination_graph.solved_state_plus_increments(&record.terms);
            let own = combination_graph.add(record.id, record.basis, &record.operands, &factors);
            let operand_withheld = record.operands.iter().any(|o| matches!(o, Operand::Combination(c) if withheld.contains(c)));
            let derived = gate_before_modulus(record.basis, operand_withheld, derivation.nonlinear, oq1, derivation.consuming, &own, &operand_kappa);
            let stated = gates.iter().find(|g| g["combination_id"] == record.id).and_then(|g| g["reason"].as_str());
            require(
                match derived {
                    Some(code) => stated == Some(code),
                    // A declared subtraction's G-1 or G-2 (`preview_difference_basis`).
                    None => stated.is_none() || stated == Some(MODULUS_MIXED)
                        || (record.basis == "result_state_subtraction"
                            && (matches!(stated, Some(crate::preview_difference_basis::SUSTAINED_NOT_IN_OPERATING | crate::preview_difference_basis::MODULUS_MIXED))
                                || stated.is_some_and(|s| crate::preview_held_status::GATES.contains(&s)))),
                },
                "GATE_DERIVATION",
            )?;
            if derived.is_none() && stated.is_none() {
                let unsigned = matches!(record.basis, "absolute_sum" | "srss");
                if unsigned {
                    unsigned_rows(rows, by_id, record.id, record.basis, &record.terms)?;
                }
                if unsigned || record.operands.iter().any(|o| matches!(o, Operand::Combination(_))) {
                    // Review r2 P2: an absolute sum or SRSS, and a mechanics, subtraction or
                    // envelope with a combination operand, whole, from its operands' rows and
                    // the record's factors.
                    let index = index.get_or_insert_with(|| RowIndex::new(rows));
                    derived_rows(index, source, record.id, record.basis, &record.operands, &factors, record.mode, derivation.nonlinear && oq1)?;
                }
            }
        }
    }
    // Every unsigned row and every row citing an operand combination's row has its record.
    for row in rows {
        let Some(id) = combination_of(row) else { continue };
        match unsigned_claim(row) {
            Some((claimed, consistent)) => {
                require(consistent, "UNSIGNED_ROW")?;
                let record = records.get(id).ok_or_else(|| "SOURCE_PREVIEW_PHYSICS_COMBINATION_UNSIGNED_UNVERIFIED".to_string())?;
                require(record.basis == claimed, "UNSIGNED_ROW")?;
            }
            None => require(records.contains_key(id) || !cites_combination(row, by_id, id), "GATE_UNVERIFIED")?,
        }
    }
    Ok(())
}

/// The `combination_derivation` the model implies (producer `combination_derivation`):
/// `None` when no kept combination is unsigned or has a combination operand.
fn model_derivation(kept: &[Value], order: &[usize], cases: &HashSet<&str>, nonlinear: bool, consuming: bool) -> Option<Value> {
    let combination_operands = |c: &Value| -> Vec<String> {
        graph::model_operands(c, cases).iter().filter_map(|o| match o {
            Operand::Combination(id) => Some(id.to_string()),
            Operand::Case(_) => None,
        }).collect()
    };
    let mut recorded: HashSet<String> = order
        .iter()
        .map(|&i| &kept[i])
        .filter(|c| matches!(c["basis"].as_str(), Some("absolute_sum" | "srss")) || !combination_operands(c).is_empty())
        .filter_map(|c| c["id"].as_str().map(str::to_string))
        .collect();
    if recorded.is_empty() {
        return None;
    }
    for &i in order.iter().rev() {
        if kept[i]["id"].as_str().is_some_and(|id| recorded.contains(id)) {
            recorded.extend(combination_operands(&kept[i]));
        }
    }
    let records: Vec<Value> = order
        .iter()
        .map(|&i| &kept[i])
        .filter(|c| c["id"].as_str().is_some_and(|id| recorded.contains(id)))
        .map(|c| serde_json::json!({"combination_id": c["id"], "basis": c["basis"], "expression": graph::evidence_expression(c)}))
        .collect();
    Some(serde_json::json!({"nonlinear_supports": nonlinear, "consuming_constant_effort": consuming, "combinations": records}))
}

/// JSON equality with numbers compared as binary64 values (a model's factor `1` and a
/// document's `1.0` are one factor).
fn same(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => x.as_f64().map(f64::to_bits) == y.as_f64().map(f64::to_bits),
        (Value::Array(x), Value::Array(y)) => x.len() == y.len() && x.iter().zip(y).all(|(x, y)| same(x, y)),
        (Value::Object(x), Value::Object(y)) => x.len() == y.len() && x.iter().all(|(k, v)| y.get(k).is_some_and(|w| same(v, w))),
        _ => a == b,
    }
}

/// A load case's modulus-basis key, as the producer's `modulus_basis_key` forms it.
pub(crate) fn modulus_key(case: &Value) -> Result<Option<String>, String> {
    let reference = case["modulus_basis_ref"].as_str();
    let temperature = case.get("modulus_basis_temperature").filter(|t| !t.is_null());
    Ok(match (reference, temperature) {
        (Some(r), None) => Some(format!("exact:{r}")),
        (None, Some(t)) => {
            let kelvin = crate::physics_source::normalized_quantity(t, open_pipe_stress_units::Dimension::Temperature)
                .map_err(|_| "SOURCE_PREVIEW_PHYSICS_COMBINATION_MODEL".to_string())?;
            Some(format!("temperature_kelvin_bits:{:016x}", kelvin.to_bits()))
        }
        (None, None) => None,
        (Some(_), Some(_)) => return Err("SOURCE_PREVIEW_PHYSICS_COMBINATION_MODEL".into()),
    })
}

/// Whether a support is a constant-effort support the solve consumes (product
/// `constant_effort_solve_dispositions`): no nonlinear member, hanger type (else
/// family) `constant_effort_support`, a finite positive constant load, every restraint
/// a frame DOF, exactly one of them translational, on a model node.
fn consuming_constant_effort(support: &Value, nodes: &HashSet<&str>) -> bool {
    if support.get("nonlinear").is_some_and(|n| !n.is_null()) {
        return false;
    }
    let hanger_type = support["hanger"]["hanger_type"].as_str().map(str::trim).filter(|t| !t.is_empty()).or_else(|| support["family"].as_str().map(str::trim));
    if hanger_type != Some("constant_effort_support") {
        return false;
    }
    if !support["hanger"]["constant_load"]["value"].as_f64().is_some_and(|v| v.is_finite() && v > 0.0) {
        return false;
    }
    let restraints: Vec<&str> = items(&support["restraints"]).iter().filter_map(Value::as_str).collect();
    restraints.len() == items(&support["restraints"]).len()
        && restraints.iter().all(|r| TRANSLATIONS.contains(r) || ROTATIONS.contains(r))
        && restraints.iter().filter(|r| TRANSLATIONS.contains(r)).count() == 1
        && support["node"].as_str().is_some_and(|n| nodes.contains(n))
}

fn model_check(source: &Value, model: &Value, gates: &[Value], rows: &[Value], by_id: &HashMap<&str, &Value>) -> Check {
    let stated_derivation = source["contract_evidence"].get(DERIVATION_MEMBER);
    // A document stating no combination at all (no gate and no combination row) makes no
    // gate claim to re-derive; a combination row without its gate is refused earlier
    // (`GATED_COMBINATION_ROW`).
    if gates.is_empty() && !rows.iter().any(|r| combination_of(r).is_some()) {
        return require(stated_derivation.is_none(), "DERIVATION_EVIDENCE");
    }
    let load_cases = items(&model["load_cases"]);
    let cases: HashSet<&str> = load_cases.iter().filter_map(|c| c["id"].as_str()).collect();
    // S1's interim withholding, which RT-A5 left empty: every combination is gated.
    let (kept, order) = crate::preview_difference_basis::kept_combinations(model);
    require(gates.len() == order.len(), "GATE_DERIVATION")?;

    let supports = items(&model["supports"]);
    let nodes: HashSet<&str> = items(&model["nodes"]).iter().filter_map(|n| n["id"].as_str()).collect();
    let nonlinear = supports.iter().any(|s| s.get("nonlinear").is_some_and(|n| !n.is_null())) || rows.iter().any(|r| r["kind"] == STATE_CODE);
    let consuming = supports.iter().any(|s| consuming_constant_effort(s, &nodes));
    let mut keys: HashMap<&str, Option<String>> = HashMap::new();
    for case in load_cases {
        if let Some(id) = case["id"].as_str() {
            keys.insert(id, modulus_key(case)?);
        }
    }
    // §10.3: the published derivation is exactly the model's.
    let expected = model_derivation(&kept, &order, &cases, nonlinear, consuming);
    require(
        match (stated_derivation, &expected) {
            (None, None) => true,
            (Some(stated), Some(expected)) => same(stated, expected),
            _ => false,
        },
        "DERIVATION_EVIDENCE",
    )?;

    let mut combination_graph = Graph::default();
    let mut withheld: HashSet<String> = HashSet::new();
    let mut index: Option<RowIndex<'_>> = None;
    for (gate, &at) in gates.iter().zip(&order) {
        let combination = &kept[at];
        let id = combination["id"].as_str().unwrap_or_default();
        let basis = combination["basis"].as_str().unwrap_or_default();
        let operands = graph::model_operands(combination, &cases);
        let terms = if graph::term_basis(basis) { model_terms(combination) } else { Vec::new() };
        let factors: Vec<f64> = terms.iter().map(|(_, f)| *f).collect();
        let operand_kappa: Vec<_> = operands.iter().map(|o| combination_graph.kappa_of(o)).collect();
        let oq1 = basis == "mechanics" && combination_graph.solved_state_plus_increments(&terms);
        // κ is read for every combination, withheld or not (product `Plan`).
        let own = combination_graph.add(id, basis, &operands, &factors);
        let operand_withheld = operands.iter().any(|o| matches!(o, Operand::Combination(c) if withheld.contains(*c)));
        let declared = crate::preview_difference_basis::declared(combination);
        let declared_gate = if declared { crate::preview_difference_basis::model_gate(model, combination)?.0 } else { None };
        // RT-A5 (§7.5): past G-1 and G-2, a `minuend_final` subtraction's held gate is the one
        // its record states; the reader cannot solve the held system, and
        // `preview_held_status` binds the record to the document and the model.
        let declared_gate = declared_gate.or_else(|| crate::preview_held_status::minuend_final(combination).then(|| crate::preview_held_status::stated_gate(source, id)).flatten());
        let derived: Option<&str> = declared_gate.or_else(|| gate_before_modulus(basis, operand_withheld, nonlinear, oq1, consuming, &own, &operand_kappa)).or_else(|| {
            if operand_withheld || !graph::term_basis(basis) {
                return None;
            }
            let reached: HashSet<&Option<String>> = combination_graph.reached(id).iter().filter_map(|case| keys.get(case)).collect();
            (reached.len() > 1).then_some(MODULUS_MIXED)
        });
        require(
            gate["combination_id"] == id && gate["withheld"] == derived.is_some() && gate["reason"].as_str() == derived,
            "GATE_DERIVATION",
        )?;
        if derived.is_some() {
            withheld.insert(id.to_string());
        } else {
            let unsigned = matches!(basis, "absolute_sum" | "srss");
            if unsigned {
                unsigned_rows(rows, by_id, id, basis, &terms)?;
            }
            if unsigned || operands.iter().any(|o| matches!(o, Operand::Combination(_))) {
                // Review r2 P2: whole, from its operands' rows and the model's factors.
                let index = index.get_or_insert_with(|| RowIndex::new(rows));
                derived_rows(index, source, id, basis, &operands, &factors, combination["mode"].as_str(), nonlinear && oq1)?;
            }
        }
    }
    // No row claims an unsigned basis its model combination does not have.
    for row in rows {
        if let (Some(id), Some((claimed, consistent))) = (combination_of(row), unsigned_claim(row)) {
            let basis = kept.iter().find(|c| c["id"] == id).and_then(|c| c["basis"].as_str());
            require(consistent && basis == Some(claimed), "UNSIGNED_ROW")?;
        }
    }
    Ok(())
}

// ------------------------------------------------------------------ whole rows (review r2 P2)
const NESTED_ROW: &str = "NESTED_ROW";
const NESTED_WITHHELD: &str = "NESTED_WITHHELD";
const SOURCE_MISSING: &str = "LOAD_COMBINATION_SOURCE_RESULT_MISSING";
const ROW_WITHHELD: &str = "LOAD_COMBINATION_ROW_WITHHELD";
const VECTOR_MISSING: &str = "LOAD_COMBINATION_VECTOR_COMPONENT_MISSING";
const VECTOR_INVALID: &str = "LOAD_COMBINATION_VECTOR_INVALID";
const SUPPORT_COMPONENTS: [&str; 6] = ["Fx", "Fy", "Fz", "Mx", "My", "Mz"];
const DISPLACEMENT_AXES: [(&str, &str); 3] = [("ux", "global_nodal_displacement_x"), ("uy", "global_nodal_displacement_y"), ("uz", "global_nodal_displacement_z")];
/// The producer's `LINEAR_STATE_KINDS` (`preview_physics.rs`): with
/// `displacement_magnitude`, the kinds a combination combines, except `_v2` kinds.
const LINEAR_STATE_KINDS: [&str; 23] = [
    "global_nodal_displacement_x",
    "global_nodal_displacement_y",
    "global_nodal_displacement_z",
    "global_nodal_rotation_x",
    "global_nodal_rotation_y",
    "global_nodal_rotation_z",
    "element_local_axial_force",
    "element_local_shear_force_y",
    "element_local_shear_force_z",
    "element_local_torsional_moment",
    "element_local_bending_moment_y",
    "element_local_bending_moment_z",
    "element_local_axial_normal_stress",
    "element_local_bending_normal_stress_y",
    "element_local_bending_normal_stress_z",
    "element_local_torsional_shear_stress",
    "pipe_section_pressure_hoop_stress",
    "pipe_section_pressure_longitudinal_stress",
    "support_reaction_component_v2",
    "constant_effort_support_applied_load",
    "element_global_end_force_v1",
    "element_global_end_moment_v1",
    "support_reaction_along_direction_v1",
];

fn stable_suffix(id: &str) -> String {
    id.replace(':', "-")
}
fn result_tail(id: &str) -> &str {
    id.strip_prefix("result:").unwrap_or(id)
}
fn identity(parts: &[&str]) -> String {
    parts.iter().map(|p| format!("{}:{p}", p.len())).collect()
}
fn kind(row: &Value) -> &str {
    row["kind"].as_str().unwrap_or_default()
}
/// The producer's `skew::case_named_id`: the `_v2` kinds and §7.1's along-direction kind
/// name their case in their id, so they are combined in their support's group, never by
/// base id.
fn case_named_id(kind: &str) -> bool {
    kind.ends_with("_v2") || kind == "support_reaction_along_direction_v1"
}
fn combined_kind(kind: &str) -> bool {
    !case_named_id(kind) && (LINEAR_STATE_KINDS.contains(&kind) || kind == "displacement_magnitude")
}
/// The producer's `algebra_dimension`.
fn dimension(row: &Value) -> Option<LoadDimension> {
    match row["unit"].as_str()? {
        "mm" | "m" => Some(LoadDimension::Displacement),
        "rad" => Some(LoadDimension::Rotation),
        "N" => Some(LoadDimension::Force),
        "N*m" => Some(LoadDimension::Moment),
        "MPa" | "Pa" => Some(LoadDimension::Pressure),
        _ => None,
    }
}
fn metadata(row: &Value) -> Option<&Value> {
    row.get("metadata").filter(|m| !m.is_null())
}
/// §4 (r1 F4): with a combination operand, rows pair by kind, unit, entity and every
/// metadata field except `basis` and `sign_convention`, the section-basis suffix within
/// `basis` still compared; a `displacement_magnitude` row by kind, unit and node
/// (producer `nested_source_identity_matches`).
fn nested_identity(reference: &Value, candidate: &Value) -> bool {
    const SECTION: &str = "; section_basis=";
    let section = |m: &Value| m["basis"].as_str().unwrap_or_default().split_once(SECTION).map(|(_, s)| s.to_string());
    reference["kind"] == candidate["kind"]
        && reference["unit"] == candidate["unit"]
        && reference["entity_ref"] == candidate["entity_ref"]
        && (reference["kind"] == "displacement_magnitude"
            || match (metadata(reference), metadata(candidate)) {
                (None, None) => true,
                (Some(a), Some(b)) => {
                    a["component"] == b["component"] && a["coordinate_system"] == b["coordinate_system"] && a["location"] == b["location"] && section(a) == section(b)
                }
                _ => false,
            })
}
/// A flat combination's pairing (producer `combination_source_identity_matches`): kind,
/// unit, entity and the whole metadata, `basis` and `sign_convention` included.
fn flat_identity(reference: &Value, candidate: &Value) -> bool {
    reference["kind"] == candidate["kind"]
        && reference["unit"] == candidate["unit"]
        && reference["entity_ref"] == candidate["entity_ref"]
        && metadata(reference) == metadata(candidate)
}
fn combination_row_id(id: &str, base: &str) -> String {
    format!("result:combination:{}:{}", stable_suffix(id), result_tail(base))
}

/// The document's rows as the producer reads them when it forms a combination: each
/// load case's rows by base id, and each combination's published rows, by base id
/// (ordinary rows and displacement magnitudes) and whole (for its support groups).
pub(crate) struct RowIndex<'a> {
    /// Base id → load case → row.
    pub(crate) cases: BTreeMap<String, HashMap<&'a str, &'a Value>>,
    case_rows: Vec<&'a Value>,
    /// Combination → base id → row.
    published: HashMap<&'a str, HashMap<String, &'a Value>>,
    /// Combination → its rows, in order (its modulus-basis records excluded).
    published_rows: HashMap<&'a str, Vec<&'a Value>>,
}

impl<'a> RowIndex<'a> {
    /// A load case's row id is its base id (the first case's, and a `_v2` row's) or
    /// `result:loadcase:<case>:<tail>`; a combination's is `result:combination:<id>:<tail>`
    /// (producer `qualified_*_result_id`). A base id is recovered as the unqualified
    /// id where a case row has it, and as `result:<tail>` otherwise.
    pub(crate) fn new(rows: &'a [Value]) -> Self {
        let mut tails: BTreeMap<String, (String, HashMap<&'a str, &'a Value>)> = BTreeMap::new();
        let mut case_rows = Vec::new();
        let mut published: HashMap<&'a str, HashMap<String, &'a Value>> = HashMap::new();
        let mut published_rows: HashMap<&'a str, Vec<&'a Value>> = HashMap::new();
        for row in rows {
            let Some(id) = row["id"].as_str() else { continue };
            let basis = &row["basis_ref"];
            let Some(owner) = basis["ref_id"].as_str() else { continue };
            match basis["ref_type"].as_str() {
                Some("load_case") => {
                    case_rows.push(row);
                    let qualified = format!("result:loadcase:{}:", stable_suffix(owner));
                    let (tail, base) = match id.strip_prefix(&qualified) {
                        Some(tail) => (tail.to_string(), None),
                        None => (result_tail(id).to_string(), Some(id.to_string())),
                    };
                    let entry = tails.entry(tail.clone()).or_insert_with(|| (format!("result:{tail}"), HashMap::new()));
                    if let Some(base) = base {
                        entry.0 = base;
                    }
                    entry.1.insert(owner, row);
                }
                Some("combination") if kind(row) != MODULUS_RECORD => {
                    published_rows.entry(owner).or_default().push(row);
                    if !kind(row).starts_with("support_reaction_") {
                        if let Some(tail) = id.strip_prefix(&format!("result:combination:{}:", stable_suffix(owner))) {
                            published.entry(owner).or_default().insert(tail.to_string(), row);
                        }
                    }
                }
                _ => {}
            }
        }
        // Keyed by base id, so the producer's sorted base-id order is kept.
        let mut cases = BTreeMap::new();
        let mut by_tail: HashMap<String, String> = HashMap::new();
        for (tail, (base, rows)) in tails {
            by_tail.insert(tail, base.clone());
            cases.insert(base, rows);
        }
        let published = published
            .into_iter()
            .map(|(owner, rows)| (owner, rows.into_iter().filter_map(|(tail, row)| by_tail.get(&tail).map(|base| (base.clone(), row))).collect()))
            .collect();
        RowIndex { cases, case_rows, published, published_rows }
    }

    fn lookup(&self, base: &str, operand: Operand<'_>) -> Option<&'a Value> {
        match operand {
            Operand::Case(case) => self.cases.get(base)?.get(case).copied(),
            Operand::Combination(id) => self.published.get(id)?.get(base).copied(),
        }
    }

    /// An operand's support row of `component` for `support` (producer `find`).
    fn support_row(&self, operand: Operand<'_>, support: &str, component: &str) -> Option<&'a Value> {
        let hit = |r: &&&'a Value| r["entity_ref"] == support && metadata(r).is_some_and(|m| m["component"] == component) && kind(r).starts_with("support_reaction_");
        match operand {
            Operand::Case(case) => self.case_rows.iter().find(|r| hit(r) && r["basis_ref"]["ref_id"] == case).copied(),
            Operand::Combination(id) => self.published_rows.get(id)?.iter().find(hit).copied(),
        }
    }
}

/// What a combination publishes, as the producer forms it.
#[derive(Default)]
struct Formed {
    rows: Vec<Value>,
    /// The rows withheld into its one `LOAD_COMBINATION_ROW_WITHHELD`.
    withheld: usize,
    /// The diagnostics naming a missing source row or vector component: id → code.
    named: HashMap<String, &'static str>,
}

#[allow(clippy::too_many_arguments)]
fn combination_row(reference: &Value, id: &str, row_id: String, value: f64, refs: Vec<String>, basis: &str, sign: &str) -> Value {
    let mut row = reference.clone();
    row["id"] = json!(row_id);
    row["value"] = json!(value);
    row["basis_ref"] = json!({"ref_type": "combination", "ref_id": id});
    row["source_result_refs"] = json!(refs);
    if let Some(metadata) = row.get_mut("metadata").filter(|m| m.is_object()) {
        metadata["basis"] = json!(basis);
        metadata["sign_convention"] = json!(sign);
    }
    row
}

/// The rows, withholdings and diagnostics of one admitted combination (producer
/// `append_combination_results`): rows pair by the nested rules with a combination
/// operand and by the flat rules otherwise; a magnitude states its derivation (the
/// producer's `derived`, true for every combination read here).
fn form(index: &RowIndex<'_>, id: &str, expression: &Expression<'_>, targets: &[Operand<'_>], sign: &str) -> Formed {
    let nested = targets.iter().any(|t| matches!(t, Operand::Combination(_)));
    let pairs = |reference: &Value, source: &Value| if nested { nested_identity(reference, source) } else { flat_identity(reference, source) };
    let mut formed = Formed::default();
    let basis = expression.metadata_basis();
    let range = expression.is_range();
    let row_id = |base: &str| combination_row_id(id, base);
    'bases: for base in index.cases.keys() {
        let missing = || format!("diagnostic:preview-physics:combination-source-missing:{}", identity(&[id, base]));
        let Some(reference) = index.lookup(base, targets[0]) else {
            if targets.iter().filter_map(|t| index.lookup(base, *t)).next().is_some_and(|r| combined_kind(kind(r))) {
                formed.named.insert(missing(), SOURCE_MISSING);
            }
            continue;
        };
        if !combined_kind(kind(reference)) {
            continue;
        }
        if kind(reference) == "displacement_magnitude" && !range {
            // The norm of the combined signed components (`append_combined_vector_magnitude`).
            let vector_diag = |code: &str| format!("diagnostic:combination:{}:{}:{}", stable_suffix(id), stable_suffix(base), stable_suffix(code));
            let mut vector = [0.0; 3];
            let mut refs = Vec::new();
            for (axis, (suffix, axis_kind)) in DISPLACEMENT_AXES.iter().enumerate() {
                let component = format!("{base}:{suffix}");
                let mut values = Vec::new();
                for target in targets {
                    let value = index
                        .lookup(&component, *target)
                        .filter(|r| r["entity_ref"] == reference["entity_ref"] && r["unit"] == reference["unit"] && r["kind"] == *axis_kind)
                        .and_then(|r| r["value"].as_f64())
                        .filter(|v| v.is_finite());
                    match value {
                        Some(value) => values.push(value),
                        None => {
                            formed.named.insert(vector_diag(VECTOR_MISSING), VECTOR_MISSING);
                            continue 'bases;
                        }
                    }
                }
                let Some(value) = expression.value(&values, LoadDimension::Displacement) else {
                    formed.named.insert(vector_diag(VECTOR_INVALID), VECTOR_INVALID);
                    continue 'bases;
                };
                vector[axis] = value;
                refs.push(row_id(&component));
            }
            let value = norm3(vector[0], vector[1], vector[2]);
            if expression.is_unsigned() && !value.is_finite() {
                formed.withheld += 1;
                continue;
            }
            let mut row = combination_row(reference, id, row_id(base), value, refs, basis, sign);
            row["metadata"] = graph::derived_magnitude_metadata(basis, sign);
            formed.rows.push(row);
            continue;
        }
        let Some(dimension) = dimension(reference) else {
            formed.withheld += 1;
            continue;
        };
        let mut values = Vec::new();
        let mut refs = Vec::new();
        for target in targets {
            match index.lookup(base, *target).filter(|s| pairs(reference, s)) {
                Some(source) => match source["value"].as_f64() {
                    Some(value) => {
                        values.push(value);
                        refs.push(source["id"].as_str().unwrap_or_default().to_string());
                    }
                    None => {
                        formed.named.insert(missing(), SOURCE_MISSING);
                        continue 'bases;
                    }
                },
                None => {
                    formed.named.insert(missing(), SOURCE_MISSING);
                    continue 'bases;
                }
            }
        }
        match expression.value(&values, dimension) {
            Some(value) => formed.rows.push(combination_row(reference, id, row_id(base), value, refs, basis, sign)),
            None => formed.withheld += 1,
        }
    }
    // Signed support actions by (support, component), each support's group whole or
    // not at all, then its magnitudes.
    let mut supports: Vec<&str> = Vec::new();
    for target in targets {
        let candidates: Vec<&Value> = match target {
            Operand::Case(case) => index.case_rows.iter().filter(|r| r["basis_ref"]["ref_id"] == *case).copied().collect(),
            Operand::Combination(c) => index.published_rows.get(c).cloned().unwrap_or_default(),
        };
        for row in candidates {
            if let Some(support) = row["entity_ref"].as_str() {
                if !supports.contains(&support) && index.support_row(*target, support, "Fx").is_some() {
                    supports.push(support);
                }
            }
        }
    }
    for support in supports {
        let base = |component: &str| format!("support-action:{}:{support}:{component}", support.len());
        let all = |component: &str| -> Option<Vec<&Value>> { targets.iter().map(|t| index.support_row(*t, support, component)).collect() };
        let group = || -> Option<Vec<Value>> {
            let mut rows = Vec::new();
            let mut combined = [0.0; 6];
            for (slot, component) in SUPPORT_COMPONENTS.iter().enumerate() {
                let sources = all(component)?;
                let dimension = if slot < 3 { LoadDimension::Force } else { LoadDimension::Moment };
                let values: Vec<f64> = sources.iter().map(|s| s["value"].as_f64()).collect::<Option<_>>()?;
                let value = expression.value(&values, dimension)?;
                combined[slot] = value;
                let refs = sources.iter().map(|s| s["id"].as_str().unwrap_or_default().to_string()).collect();
                rows.push(combination_row(sources[0], id, row_id(&base(component)), value, refs, basis, sign));
            }
            // R-6/R-7 (§7.1): the along-direction value in its support's group (producer).
            let along: Vec<Option<&Value>> = targets.iter().map(|t| index.support_row(*t, support, "along_direction")).collect();
            if along.iter().any(Option::is_some) {
                let sources: Vec<&Value> = along.into_iter().collect::<Option<_>>()?;
                let values: Vec<f64> = sources.iter().map(|s| s["value"].as_f64()).collect::<Option<_>>()?;
                let value = expression.value(&values, LoadDimension::Force)?;
                let refs = sources.iter().map(|s| s["id"].as_str().unwrap_or_default().to_string()).collect();
                rows.push(combination_row(sources[0], id, row_id(&base("along_direction")), value, refs, basis, sign));
            }
            for (component, norm, dimension) in [
                ("force_magnitude", norm3(combined[0], combined[1], combined[2]), LoadDimension::Force),
                ("moment_magnitude", norm3(combined[3], combined[4], combined[5]), LoadDimension::Moment),
            ] {
                if !range && expression.is_unsigned() && !norm.is_finite() {
                    return None;
                }
                let sources: Vec<&Value> = if range { all(component)? } else { targets.iter().filter_map(|t| index.support_row(*t, support, component)).collect() };
                let first = *sources.first()?;
                let (value, refs) = if range {
                    let values: Vec<f64> = sources.iter().map(|s| s["value"].as_f64()).collect::<Option<_>>()?;
                    (expression.value(&values, dimension)?, sources.iter().map(|s| s["id"].as_str().unwrap_or_default().to_string()).collect())
                } else {
                    (norm, SUPPORT_COMPONENTS.iter().map(|c| row_id(&base(c))).collect())
                };
                rows.push(combination_row(first, id, row_id(&base(component)), value, refs, basis, sign));
            }
            Some(rows)
        };
        match group() {
            Some(rows) => formed.rows.extend(rows),
            None => formed.withheld += 1,
        }
    }
    // A range-selected magnitude whose source is a load case's carries no metadata.
    for row in formed.rows.iter_mut().filter(|r| r["kind"] == "displacement_magnitude" && metadata(r).is_none()) {
        row["metadata"] = graph::derived_magnitude_metadata(basis, sign);
    }
    formed
}

/// Review r2 P2: every row of an admitted absolute sum or SRSS, and of a mechanics
/// combination, subtraction or envelope with a combination operand, is re-derived from
/// its operands' rows (a load case's rows, or the operand combination's published rows)
/// with `factors` (the model's, or the derivation record's), as the producer forms it,
/// and compared bit for bit: each published row must be a derived row and each derived row published; every
/// row it does not publish must be named by the producer's withholding diagnostics, and
/// no other withholding stated. `solved_state_plus_increments` is decision OQ-1's
/// admission in a nonlinear model (its sign convention).
#[allow(clippy::too_many_arguments)]
fn derived_rows(index: &RowIndex<'_>, source: &Value, id: &str, basis: &str, operands: &[Operand<'_>], factors: &[f64], mode: Option<&str>, solved_state_plus_increments: bool) -> Check {
    let expression = Expression::new(basis, operands, factors, mode).ok_or_else(|| format!("SOURCE_PREVIEW_PHYSICS_COMBINATION_{NESTED_ROW}"))?;
    let sign = expression.sign_convention(solved_state_plus_increments);
    let formed = form(index, id, &expression, operands, &sign);
    let published = index.published_rows.get(id).map(Vec::as_slice).unwrap_or(&[]);
    let mut expected: HashMap<&str, &Value> = HashMap::new();
    for row in &formed.rows {
        require(expected.insert(row["id"].as_str().unwrap_or_default(), row).is_none(), NESTED_ROW)?;
    }
    let mut seen: HashSet<&str> = HashSet::new();
    for row in published {
        let row_id = row["id"].as_str().unwrap_or_default();
        require(seen.insert(row_id) && expected.get(row_id).is_some_and(|e| same(row, e)), NESTED_ROW)?;
    }
    require(seen.len() == expected.len(), NESTED_ROW)?;
    // The withholdings: exactly the derived ones.
    let diagnostics = items(&source["diagnostics"]);
    let names = |d: &&Value| items(&d["affected_refs"]).first().is_some_and(|r| r == id);
    // Each stated withholding binds its code to its id (review r3): a source-missing
    // diagnostic restated under a vector code, or the reverse, is refused.
    let mut stated: HashMap<&str, &str> = HashMap::new();
    for d in diagnostics.iter().filter(|d| matches!(d["code"].as_str(), Some(SOURCE_MISSING | VECTOR_MISSING | VECTOR_INVALID))).filter(names) {
        let (Some(diagnostic), Some(code)) = (d["id"].as_str(), d["code"].as_str()) else { continue };
        require(stated.insert(diagnostic, code).is_none(), NESTED_WITHHELD)?;
    }
    require(
        stated.len() == formed.named.len() && formed.named.iter().all(|(n, code)| stated.get(n.as_str()) == Some(code)),
        NESTED_WITHHELD,
    )?;
    let withheld: Vec<&Value> = diagnostics.iter().filter(|d| d["code"] == ROW_WITHHELD).filter(names).collect();
    let prefix = format!("combination {id} withholds {} combined row(s) that could not be formed: ", formed.withheld);
    require(
        if formed.withheld == 0 {
            withheld.is_empty()
        } else {
            withheld.len() == 1
                && withheld[0]["id"] == format!("diagnostic:preview-physics:combination-row-withheld:{}", identity(&[id]))
                && withheld[0]["message"].as_str().is_some_and(|m| m.starts_with(&prefix))
        },
        NESTED_WITHHELD,
    )
}

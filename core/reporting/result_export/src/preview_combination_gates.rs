//! RT-A4 (DEL-05-02 Work `absolute-sum-srss-combinations.md` §4-§5, §10; review r1
//! P2): preview-physics-2's combination gates and unsigned rows, re-derived from what
//! the received document allows and, when it is supplied, from its model. The Python
//! mirror is `tests/verifiers/preview_combination_gates.py`.
//!
//! - **Model-free** (`check(source, None)`). A gate stating
//!   `NONLINEAR_COMBINATION_REQUIRES_SOLVE` needs a published nonlinear support state
//!   row; `LOAD_COMBINATION_OPERAND_WITHHELD` needs its diagnostic naming at least one
//!   combination withheld earlier in the gate order.
//!   `CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND` and
//!   `CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED` turn on a combination's expression and
//!   on κ, which the document never states for a withheld combination, so they are
//!   refused as unverifiable (`..._COMBINATION_GATE_UNVERIFIED`). An absolute sum's or
//!   SRSS's rows are re-derived bit for bit from their source rows and the factors of
//!   the combination's `combination_operands` record, which a model with nonlinear
//!   supports publishes; with no record (a linear model) they cannot be verified
//!   without the model and are refused (`..._COMBINATION_UNSIGNED_UNVERIFIED`). The
//!   flat gates (`CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE`,
//!   `COMBINATION_MODULUS_BASIS_MIXED`) keep the model-free reading they had.
//! - **With the model** (`check(source, Some(model))`). Every gate, in the model's
//!   evaluation order after S1's interim removal, is re-derived as the producer forms
//!   it (`preview_physics.rs` `gate_reason`): a withheld operand combination; then, for
//!   the term bases, nonlinear supports (except decision OQ-1), κ where a consuming
//!   constant-effort support applies, and the modulus bases of every load case reached.
//!   Every row of an absolute sum or SRSS states its basis and sign convention, and each
//!   row formed from operand rows is A or R of the model's factors, bit for bit.
use crate::combination_graph::{self as graph, Graph, Operand};
use open_pipe_stress_load_case_algebra::{mechanics_fixed_load_gate, unsigned_fixed_load_gate, FixedLoadWithholding};
use serde_json::Value;
use std::collections::{HashMap, HashSet};

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
    match model {
        Some(model) => model_check(model, gates, rows, &by_id),
        None => free_check(source, gates, rows, &by_id),
    }
}

fn free_check(source: &Value, gates: &[Value], rows: &[Value], by_id: &HashMap<&str, &Value>) -> Check {
    let nonlinear = rows.iter().any(|r| r["kind"] == STATE_CODE);
    let diagnostics = items(&source["diagnostics"]);
    let mut withheld: HashSet<&str> = HashSet::new();
    for gate in gates {
        let id = gate["combination_id"].as_str().unwrap_or_default();
        let Some(reason) = gate["reason"].as_str() else { continue };
        match reason {
            NONLINEAR => require(nonlinear, "GATE_DERIVATION")?,
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
            CONSTANT_EFFORT_UNSIGNED | CONSTANT_EFFORT_MIXED => return Err("SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_UNVERIFIED".into()),
            _ => {}
        }
        withheld.insert(id);
    }
    // The records a nonlinear model publishes for its admitted combinations.
    let records: HashMap<&str, &Value> = items(&source["contract_evidence"]["combination_operands"])
        .iter()
        .filter_map(|r| r["combination_id"].as_str().map(|id| (id, r)))
        .collect();
    let mut checked: HashSet<&str> = HashSet::new();
    for row in rows {
        let (Some(id), Some((claimed, consistent))) = (combination_of(row), unsigned_claim(row)) else { continue };
        require(consistent, "UNSIGNED_ROW")?;
        if !checked.insert(id) {
            continue;
        }
        let record = records.get(id).ok_or_else(|| "SOURCE_PREVIEW_PHYSICS_COMBINATION_UNSIGNED_UNVERIFIED".to_string())?;
        require(record["basis"] == claimed, "UNSIGNED_ROW")?;
        let terms: Terms<'_> = items(&record["expression"]["terms"]).iter().map(graph::evidence_term).collect::<Option<_>>().ok_or_else(|| "SOURCE_PREVIEW_PHYSICS_COMBINATION_UNSIGNED_SOURCES".to_string())?;
        unsigned_rows(rows, by_id, id, claimed, &terms)?;
    }
    Ok(())
}

/// A load case's modulus-basis key, as the producer's `modulus_basis_key` forms it.
fn modulus_key(case: &Value) -> Result<Option<String>, String> {
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

fn model_check(model: &Value, gates: &[Value], rows: &[Value], by_id: &HashMap<&str, &Value>) -> Check {
    let combinations = items(&model["combinations"]);
    let load_cases = items(&model["load_cases"]);
    let cases: HashSet<&str> = load_cases.iter().filter_map(|c| c["id"].as_str()).collect();
    // S1's interim withholding removes a combination with a difference basis or a
    // `minuend_final` subtrahend, and every combination naming one, before the gates.
    let mut removed: HashSet<&str> = HashSet::new();
    for &index in &graph::model_order(combinations, &cases) {
        let c = &combinations[index];
        let interim = c.get("difference_basis").is_some() || c["subtrahend_support_state"] == "minuend_final";
        let names_removed = graph::model_operands(c, &cases).iter().any(|o| matches!(o, Operand::Combination(id) if removed.contains(id)));
        if interim || names_removed {
            removed.extend(c["id"].as_str());
        }
    }
    let kept: Vec<Value> = combinations.iter().filter(|c| !c["id"].as_str().is_some_and(|id| removed.contains(id))).cloned().collect();
    let order = graph::model_order(&kept, &cases);
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

    let mut combination_graph = Graph::default();
    let mut withheld: HashSet<String> = HashSet::new();
    for (gate, &index) in gates.iter().zip(&order) {
        let combination = &kept[index];
        let id = combination["id"].as_str().unwrap_or_default();
        let basis = combination["basis"].as_str().unwrap_or_default();
        let operands = graph::model_operands(combination, &cases);
        let terms = if graph::term_basis(basis) { model_terms(combination) } else { Vec::new() };
        let factors: Vec<f64> = terms.iter().map(|(_, f)| *f).collect();
        let operand_kappa: Vec<_> = operands.iter().map(|o| combination_graph.kappa_of(o)).collect();
        let oq1 = basis == "mechanics" && combination_graph.solved_state_plus_increments(&terms);
        // κ is read for every combination, withheld or not (product `Plan`).
        let own = combination_graph.add(id, basis, &operands, &factors);
        let derived: Option<&str> = if operands.iter().any(|o| matches!(o, Operand::Combination(c) if withheld.contains(*c))) {
            Some(graph::OPERAND_WITHHELD)
        } else if !graph::term_basis(basis) {
            None
        } else if basis == "mechanics" && nonlinear && !oq1 {
            Some(NONLINEAR)
        } else {
            let fixed = consuming
                .then(|| if basis == "mechanics" { mechanics_fixed_load_gate(&own, operand_kappa.iter()) } else { unsigned_fixed_load_gate(operand_kappa.iter()) })
                .flatten();
            match fixed {
                Some(FixedLoadWithholding::UnitFactorSum) => Some(CONSTANT_EFFORT),
                Some(FixedLoadWithholding::OperandUndefined) => Some(CONSTANT_EFFORT_MIXED),
                Some(FixedLoadWithholding::UnsignedOperand) => Some(CONSTANT_EFFORT_UNSIGNED),
                None => {
                    let reached: HashSet<&Option<String>> = combination_graph.reached(id).iter().filter_map(|case| keys.get(case)).collect();
                    (reached.len() > 1).then_some(MODULUS_MIXED)
                }
            }
        };
        require(
            gate["combination_id"] == id && gate["withheld"] == derived.is_some() && gate["reason"].as_str() == derived,
            "GATE_DERIVATION",
        )?;
        if derived.is_some() {
            withheld.insert(id.to_string());
        } else if matches!(basis, "absolute_sum" | "srss") {
            unsigned_rows(rows, by_id, id, basis, &terms)?;
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

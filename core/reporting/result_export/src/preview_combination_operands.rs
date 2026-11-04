//! Operating-case slice 1a (DEL-05-02 Design `pf-nonlinear-operand-standing.md` §3):
//! the preview-physics-2 `combination_operands` statement of a model with nonlinear
//! supports, and its two notices, re-derived from the received document. These
//! checks establish the statement's internal consistency, not any case's standing:
//! each operand's quality record is the document's own, compared bit for bit, never
//! re-established here. The Python mirror is `tests/verifiers/preview_combination_operands.py`.
//!
//! RT-A4 (DEL-05-02 `absolute-sum-srss-combinations.md` §4-§5): a record may also
//! state an absolute sum, an SRSS, or a mechanics combination admitted by decision
//! OQ-1 (one load case with factor exactly 1 plus combinations whose fixed-load
//! count is exactly 0, re-derived here from the earlier records); a term names a
//! load case or a combination; a combination operand is `{combination_id}` and
//! states its own standing in its own, earlier record.
use crate::combination_graph::{self as graph, Graph, Operand};
use crate::section_basis::bit_equal;
use serde_json::{json, Value};
use std::collections::HashSet;

type Check = Result<(), String>;

pub(crate) const MEMBER: &str = "combination_operands";
const STATES_DIFFER: &str = "COMBINATION_OPERAND_SUPPORT_STATES_DIFFER";
const NOT_PASSED: &str = "COMBINATION_OPERAND_CHECKS_NOT_PASSED";
const STATE_CODE: &str = "nonlinear_support_active_set_state_code";
const STANDING: &str = "derived_from_case_results";
const SOURCE: &str = "core/product_physics";
const RECORD_KEYS: [&str; 5] = ["combination_id", "basis", "expression", "operands", "standing"];
const OPERAND_KEYS: [&str; 4] = ["load_case_id", "modulus_basis_ref", "modulus_basis_temperature", "numerical_quality"];
const NAMED: usize = 8;
const LINEAR_BASIS: &str = "explicit_user_linear_combination";
const MODES: [&str; 4] = ["min", "max", "min_abs", "max_abs"];
const DISPLACEMENT: &str = "nonlinear_support_final_displacement";
const RELATIVE_DISPLACEMENT: &str = "nonlinear_support_final_relative_displacement";
const MODULUS_RECORD: &str = "combination_modulus_basis_record";
const SUPPORT_COMPONENT: &str = "support_reaction_component_v2";
const SUPPORT_COMPONENTS: [&str; 6] = ["Fx", "Fy", "Fz", "Mx", "My", "Mz"];
const DISPLACEMENT_COMPONENTS: [&str; 3] = ["global_nodal_displacement_x", "global_nodal_displacement_y", "global_nodal_displacement_z"];

fn require(ok: bool, code: &str) -> Check {
    if ok {
        Ok(())
    } else {
        Err(format!("SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_{code}"))
    }
}
fn keys(v: &Value, required: &[&str]) -> bool {
    v.as_object().is_some_and(|o| o.len() == required.len() && required.iter().all(|k| o.contains_key(*k)))
}
fn items(v: &Value) -> &[Value] {
    v.as_array().map(Vec::as_slice).unwrap_or(&[])
}
fn text(v: &Value) -> Option<&str> {
    v.as_str().filter(|s| !s.is_empty())
}
fn identity(id: &str) -> String {
    format!("{}:{id}", id.len())
}
fn case_of(row: &Value) -> Option<&str> {
    (row["basis_ref"]["ref_type"] == "load_case").then(|| row["basis_ref"]["ref_id"].as_str()).flatten()
}
fn combination_of(row: &Value) -> Option<&str> {
    (row["basis_ref"]["ref_type"] == "combination").then(|| row["basis_ref"]["ref_id"].as_str()).flatten()
}

/// The operands of an expression in the producer's evaluation order (terms; minuend
/// then subtrahend; a range envelope's ids sorted, byte order), with the terms'
/// factors. An id names the load case when one has it, a combination otherwise.
fn evaluation_order<'a>(basis: &str, expression: &'a Value, cases: &HashSet<&str>) -> Result<(Vec<Operand<'a>>, Vec<f64>), String> {
    let resolve = |id: &'a str| if cases.contains(id) { Operand::Case(id) } else { Operand::Combination(id) };
    match basis {
        "mechanics" | "absolute_sum" | "srss" => {
            require(keys(expression, &["terms"]), "SHAPE")?;
            let mut operands = Vec::new();
            let mut factors = Vec::new();
            for term in items(&expression["terms"]) {
                let (operand, factor) = graph::evidence_term(term).ok_or_else(|| "SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_SHAPE".to_string())?;
                require(!operands.contains(&operand), "SHAPE")?;
                operands.push(operand);
                factors.push(factor);
            }
            require(!operands.is_empty(), "SHAPE")?;
            Ok((operands, factors))
        }
        "result_state_subtraction" => {
            require(keys(expression, &["minuend_id", "subtrahend_id"]), "SHAPE")?;
            let (a, b) = (text(&expression["minuend_id"]), text(&expression["subtrahend_id"]));
            require(a.is_some() && b.is_some() && a != b, "SHAPE")?;
            Ok((vec![resolve(a.unwrap()), resolve(b.unwrap())], Vec::new()))
        }
        "range_envelope" => {
            require(keys(expression, &["operand_ids", "mode"]) && expression["mode"].as_str().is_some_and(|m| MODES.contains(&m)), "SHAPE")?;
            let mut ids = items(&expression["operand_ids"]).iter().map(text).collect::<Option<Vec<_>>>().ok_or_else(|| "SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_SHAPE".to_string())?;
            require(!ids.is_empty() && expression["operand_ids"].is_array(), "SHAPE")?;
            ids.sort_unstable();
            require(ids.windows(2).all(|w| w[0] != w[1]), "SHAPE")?;
            Ok((ids.into_iter().map(resolve).collect(), Vec::new()))
        }
        _ => Err("SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_SHAPE".into()),
    }
}

/// The row basis and, for the RT-A4 bases, the sign convention of a record's rows.
fn row_basis(basis: &str) -> (&'static str, Option<&'static str>) {
    match basis {
        "range_envelope" => ("explicit_user_range_envelope", None),
        "result_state_subtraction" => ("explicit_user_result_state_subtraction", None),
        "absolute_sum" => (graph::ABSOLUTE_SUM_BASIS, Some(graph::ABSOLUTE_SUM_SIGN)),
        "srss" => (graph::SRSS_BASIS, Some(graph::SRSS_SIGN)),
        _ => (LINEAR_BASIS, Some(graph::SOLVED_STATE_PLUS_INCREMENTS_SIGN)),
    }
}

/// §2.2: an operand's reported state of one support: the state code and, for a
/// sliding friction support, its slip sense, from the relative-displacement row when
/// the case publishes one and otherwise the zero-ground displacement row.
fn reported_state(code: Option<f64>, sense: Option<f64>) -> &'static str {
    match code {
        None => "unreported",
        Some(c) if c == 0.0 => "inactive",
        Some(c) if c == 1.0 => "active",
        Some(c) if c == 2.0 => "sticking",
        Some(c) if c == 3.0 => match sense {
            Some(v) if v > 0.0 => "sliding (positive slip)",
            Some(v) if v < 0.0 => "sliding (negative slip)",
            _ => "sliding (slip sense unavailable)",
        },
        Some(_) => "unknown",
    }
}
fn same_state(states: &[&str]) -> bool {
    states.iter().all(|s| *s == states[0] && *s != "unreported" && !s.contains("unavailable"))
}
fn notice(id: String, code: &str, severity: &str, message: String, refs: Vec<String>) -> Value {
    json!({"id": id, "code": code, "severity": severity, "message": message, "source": SOURCE, "affected_refs": refs})
}

/// §2.2: the support-state notice the producer forms for one record.
fn states_notice(id: &str, operands: &[&str], rows: &[Value]) -> Result<Option<Value>, String> {
    let mut states: Vec<(&str, &str, f64)> = Vec::new();
    for row in rows.iter().filter(|r| r["kind"] == STATE_CODE) {
        let (Some(case), Some(support)) = (case_of(row), text(&row["entity_ref"])) else {
            return Err("SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_STATE_CODE".into());
        };
        let code = row["value"].as_f64().filter(|c| [0.0, 1.0, 2.0, 3.0].contains(c));
        require(code.is_some(), "STATE_CODE")?;
        require(!states.iter().any(|(c, s, _)| *c == case && *s == support), "STATE_CODE")?;
        states.push((case, support, code.unwrap()));
    }
    let mut supports: Vec<&str> = Vec::new();
    for case in operands {
        for (_, support, _) in states.iter().filter(|(c, _, _)| c == case) {
            if !supports.contains(support) {
                supports.push(support);
            }
        }
    }
    let sense = |case: &str, support: &str| {
        let find = |kind: &str| rows.iter().find(|r| r["kind"] == kind && case_of(r) == Some(case) && r["entity_ref"] == support).and_then(|r| r["value"].as_f64());
        find(RELATIVE_DISPLACEMENT).or_else(|| find(DISPLACEMENT))
    };
    let mut differing: Vec<(&str, Vec<&str>)> = Vec::new();
    for support in supports {
        let reported: Vec<&str> = operands
            .iter()
            .map(|case| reported_state(states.iter().find(|(c, s, _)| c == case && *s == support).map(|t| t.2), sense(case, support)))
            .collect();
        if !same_state(&reported) {
            differing.push((support, reported));
        }
    }
    if differing.is_empty() {
        return Ok(None);
    }
    let mut named = differing
        .iter()
        .take(NAMED)
        .map(|(support, states)| {
            let per_case = operands.iter().zip(states).map(|(case, state)| format!("{case}: {state}")).collect::<Vec<_>>().join(", ");
            format!("{support} ({per_case})")
        })
        .collect::<Vec<_>>()
        .join("; ");
    if differing.len() > NAMED {
        named.push_str(&format!("; and {} more", differing.len() - NAMED));
    }
    Ok(Some(notice(
        format!("diagnostic:preview-physics:combination-operand-states:{}", identity(id)),
        STATES_DIFFER,
        "info",
        format!(
            "combination {id}: the reported final state of {} nonlinear support(s) differs between its operand cases: {named}. Each operand case is solved independently from the model's authored initial support states, under the static, history-free support model, and keeps its own reported support state; the combination's rows are derived from those case results and are not one solved state.",
            differing.len()
        ),
        std::iter::once(id.to_string()).chain(differing.iter().map(|(s, _)| s.to_string())).collect(),
    )))
}

/// §2.2: the not-passed notice the producer forms for one record.
fn standing_notice(id: &str, operands: &[Value]) -> Option<Value> {
    let failing: Vec<(&str, &str)> = operands
        .iter()
        .filter(|o| o.get("load_case_id").is_some())
        .map(|o| (o["load_case_id"].as_str().unwrap_or_default(), o["numerical_quality"]["solve_quality"].as_str().unwrap_or("not_assessed")))
        .filter(|(_, quality)| *quality != "checks_passed")
        .collect();
    if failing.is_empty() {
        return None;
    }
    let named = failing.iter().map(|(case, quality)| format!("{case} ({quality})")).collect::<Vec<_>>().join(", ");
    Some(notice(
        format!("diagnostic:preview-physics:combination-operand-standing:{}", identity(id)),
        NOT_PASSED,
        "warning",
        format!("combination {id} uses operand case(s) whose numerical integrity checks did not pass: {named}; its rows are still published, as derived inspection results without their own numerical enclosure; each operand's standing is stated in contract_evidence.combination_operands"),
        std::iter::once(id.to_string()).chain(failing.iter().map(|(case, _)| case.to_string())).collect(),
    ))
}

/// §3: presence, shape, standing, row binding and both notices. A blocked envelope
/// carries neither the member nor a notice.
pub(crate) fn check(source: &Value, solved: bool) -> Check {
    let member = source["contract_evidence"].get(MEMBER);
    let rows = items(&source["results"]);
    let diagnostics = items(&source["diagnostics"]);
    let mut expected: Vec<Value> = Vec::new();
    let nonlinear = rows.iter().any(|r| r["kind"] == STATE_CODE);
    let admitted: Vec<&str> = items(&source["contract_evidence"]["combination_gates"])
        .iter()
        .filter(|g| g["withheld"] == false)
        .filter_map(|g| g["combination_id"].as_str())
        .collect();
    require(member.is_some() == (solved && nonlinear && !admitted.is_empty()), "PRESENCE")?;
    let quality = items(&source["numerical_quality"]["cases"]);
    let cases: HashSet<&str> = quality.iter().filter(|q| q["basis_ref"]["ref_type"] == "load_case").filter_map(|q| q["basis_ref"]["ref_id"].as_str()).collect();
    let mut recorded = Graph::default();
    // In a solved nonlinear document a linear-combination row belongs to a recorded mechanics
    // combination, which decision OQ-1 admitted (checked on its record below).
    if solved && nonlinear {
        let mechanics: Vec<&str> = member
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter(|r| r["basis"] == "mechanics")
            .filter_map(|r| r["combination_id"].as_str())
            .collect();
        require(
            !rows.iter().any(|r| combination_of(r).is_some_and(|c| !mechanics.contains(&c)) && r["metadata"]["basis"] == LINEAR_BASIS),
            "MECHANICS_ADMITTED",
        )?;
    }
    if let Some(member) = member {
        let records = member.as_array().ok_or_else(|| "SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_SHAPE".to_string())?;
        require(records.len() == admitted.len(), "PRESENCE")?;
        for (record, gate_id) in records.iter().zip(&admitted) {
            require(keys(record, &RECORD_KEYS) && record["standing"] == STANDING, "SHAPE")?;
            let id = text(&record["combination_id"]).filter(|id| id == gate_id).ok_or_else(|| "SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_PRESENCE".to_string())?;
            let basis = record["basis"].as_str().unwrap_or_default();
            let (order, factors) = evaluation_order(basis, &record["expression"], &cases)?;
            // In a solved nonlinear document a mechanics combination is admitted only by
            // decision OQ-1 (absolute-sum-srss §5.2).
            if basis == "mechanics" {
                let terms: Vec<(Operand<'_>, f64)> = order.iter().copied().zip(factors.iter().copied()).collect();
                require(recorded.solved_state_plus_increments(&terms), "MECHANICS_ADMITTED")?;
            }
            let operands = items(&record["operands"]);
            require(record["operands"].is_array() && operands.len() == order.len(), "SHAPE")?;
            for (operand, target) in operands.iter().zip(&order) {
                let case = match target {
                    // A combination operand states its standing in its own, earlier record.
                    Operand::Combination(c) => {
                        require(bit_equal(operand, &json!({"combination_id": c})) && recorded.knows(c), "SHAPE")?;
                        continue;
                    }
                    Operand::Case(case) => *case,
                };
                require(keys(operand, &OPERAND_KEYS) && operand["load_case_id"] == case, "SHAPE")?;
                require(operand["modulus_basis_ref"].is_null() || text(&operand["modulus_basis_ref"]).is_some(), "SHAPE")?;
                let t = &operand["modulus_basis_temperature"];
                require(t.is_null() || (keys(t, &["value", "unit"]) && t["value"].as_f64().is_some() && text(&t["unit"]).is_some()), "SHAPE")?;
                let stated = quality.iter().find(|q| q["basis_ref"]["ref_type"] == "load_case" && q["basis_ref"]["ref_id"] == case);
                require(stated.is_some_and(|q| bit_equal(q, &operand["numerical_quality"])), "STANDING")?;
            }
            recorded.add(id, basis, &order, &factors);
            // Rows: references by row kind and basis.
            let (row_basis, sign) = row_basis(basis);
            let by_id = |reference: &Value| rows.iter().find(|r| r["id"] == *reference);
            for row in rows.iter().filter(|r| combination_of(r) == Some(id)) {
                let kind = row["kind"].as_str().unwrap_or_default();
                if kind == MODULUS_RECORD {
                    continue;
                }
                require(row["metadata"].is_null() || row["metadata"]["basis"] == row_basis, "ROWS")?;
                require(row["metadata"].is_null() || sign.is_none_or(|s| row["metadata"]["sign_convention"] == s), "ROWS")?;
                let refs = items(&row["source_result_refs"]);
                let recomputed: Option<Vec<(&str, Option<&str>)>> = match (basis, kind) {
                    ("range_envelope", _) => None,
                    (_, "displacement_magnitude") => Some(DISPLACEMENT_COMPONENTS.iter().map(|k| (*k, None)).collect()),
                    (_, "support_reaction_force_magnitude_v2" | "support_reaction_moment_magnitude_v2") => {
                        Some(SUPPORT_COMPONENTS.iter().map(|c| (SUPPORT_COMPONENT, Some(*c))).collect())
                    }
                    _ => None,
                };
                match recomputed {
                    // Recomputed magnitudes name this combination's component rows, in order.
                    Some(components) => {
                        require(refs.len() == components.len(), "ROWS")?;
                        for (reference, (component_kind, component)) in refs.iter().zip(components) {
                            let source = by_id(reference).ok_or_else(|| "SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_ROWS".to_string())?;
                            require(
                                combination_of(source) == Some(id)
                                    && source["kind"] == component_kind
                                    && source["entity_ref"] == row["entity_ref"]
                                    && component.is_none_or(|c| source["metadata"]["component"] == c),
                                "ROWS",
                            )?;
                        }
                    }
                    // Direct rows name one matching operand row per operand, in evaluation order.
                    None => {
                        require(refs.len() == order.len(), "ROWS")?;
                        for (reference, operand) in refs.iter().zip(&order) {
                            let source = by_id(reference).ok_or_else(|| "SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_ROWS".to_string())?;
                            require(
                                operand.owns(source)
                                    && source["kind"] == row["kind"]
                                    && source["entity_ref"] == row["entity_ref"]
                                    && source["unit"] == row["unit"]
                                    && source["metadata"]["component"] == row["metadata"]["component"]
                                    && source["metadata"]["location"] == row["metadata"]["location"],
                                "ROWS",
                            )?;
                        }
                    }
                }
            }
            let order_cases: Vec<&str> = order.iter().filter_map(|o| match o {
                Operand::Case(case) => Some(*case),
                Operand::Combination(_) => None,
            }).collect();
            expected.extend(states_notice(id, &order_cases, rows)?);
            expected.extend(standing_notice(id, operands));
        }
    }
    let stated: Vec<&Value> = diagnostics.iter().filter(|d| d["code"] == STATES_DIFFER || d["code"] == NOT_PASSED).collect();
    require(stated.len() == expected.len(), "NOTICE")?;
    for notice in &expected {
        require(stated.iter().filter(|d| bit_equal(d, notice)).count() == 1, "NOTICE")?;
    }
    Ok(())
}

/// §3, with a supplied model: when the model has a nonlinear support, the records
/// follow the model's evaluation order (absolute-sum-srss §2), each binds its id,
/// basis, expression and operand modulus selections to the model's, and every
/// subtraction or range envelope whose operand combinations are all recorded is
/// recorded (only a withheld operand can withhold one); with no nonlinear support
/// there is none.
pub(crate) fn model_checks(source: &Value, model: &Value) -> Check {
    if source["status"]["mechanics"] != "MECHANICS_SOLVED" {
        return Ok(());
    }
    let nonlinear = items(&model["supports"]).iter().any(|s| s.get("nonlinear").is_some_and(|n| !n.is_null()));
    let combinations: &[Value] = match model.get("combinations") {
        None | Some(Value::Null) => &[],
        Some(Value::Array(list)) => list.as_slice(),
        Some(_) => return Err("SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_MODEL".into()),
    };
    let records = items(source["contract_evidence"].get(MEMBER).unwrap_or(&Value::Null));
    if !nonlinear {
        return require(records.is_empty(), "MODEL");
    }
    let cases = items(&model["load_cases"]);
    let case_ids: HashSet<&str> = cases.iter().filter_map(|c| c["id"].as_str()).collect();
    let order = graph::model_order(combinations, &case_ids);
    let recorded: HashSet<&str> = records.iter().filter_map(|r| r["combination_id"].as_str()).collect();
    // The records, a subsequence of the evaluation order.
    let mut position = 0;
    for record in records {
        let at = order[position..].iter().position(|&i| record["combination_id"] == combinations[i]["id"]).ok_or_else(|| "SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_MODEL".to_string())?;
        let combination = &combinations[order[position + at]];
        position += at + 1;
        require(record["basis"] == combination["basis"] && bit_equal(&record["expression"], &graph::evidence_expression(combination)), "MODEL")?;
        for operand in items(&record["operands"]) {
            if operand.get("combination_id").is_some() {
                continue;
            }
            let case = cases.iter().find(|c| c["id"] == operand["load_case_id"]).ok_or_else(|| "SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_MODEL".to_string())?;
            let reference = case.get("modulus_basis_ref").cloned().unwrap_or(Value::Null);
            let temperature = match case.get("modulus_basis_temperature").filter(|t| !t.is_null()) {
                Some(t) => {
                    let dimension = open_pipe_stress_units::Dimension::Temperature;
                    let kelvin = crate::physics_source::normalized_quantity(t, dimension).map_err(|_| "SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_MODEL".to_string())?;
                    let unit = open_pipe_stress_units::canonical_unit(dimension).ok_or_else(|| "SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_MODEL".to_string())?;
                    json!({"value": kelvin, "unit": unit.definition().symbol})
                }
                None => Value::Null,
            };
            require(bit_equal(&operand["modulus_basis_ref"], &reference) && bit_equal(&operand["modulus_basis_temperature"], &temperature), "MODEL")?;
        }
    }
    for &index in &order {
        let combination = &combinations[index];
        if matches!(combination["basis"].as_str(), Some("result_state_subtraction" | "range_envelope"))
            && graph::model_operands(combination, &case_ids).iter().all(|o| matches!(o, Operand::Case(_)) || recorded.contains(o.id()))
        {
            require(combination["id"].as_str().is_some_and(|id| recorded.contains(id)), "MODEL")?;
        }
    }
    Ok(())
}

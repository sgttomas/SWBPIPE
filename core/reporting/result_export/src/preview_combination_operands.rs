//! Operating-case slice 1a (DEL-05-02 Design `pf-nonlinear-operand-standing.md` §3):
//! the preview-physics-2 `combination_operands` statement of a model with nonlinear
//! supports, and its two notices, re-derived from the received document. These
//! checks establish the statement's internal consistency, not any case's standing:
//! each operand's quality record is the document's own, compared bit for bit, never
//! re-established here. The Python mirror is `tests/verifiers/preview_combination_operands.py`.
use crate::section_basis::bit_equal;
use serde_json::{json, Value};

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

/// The operands of an expression in the producer's evaluation order: minuend then
/// subtrahend; a range envelope's ids sorted (byte order).
fn evaluation_order<'a>(basis: &str, expression: &'a Value) -> Result<Vec<&'a str>, String> {
    match basis {
        "result_state_subtraction" => {
            require(keys(expression, &["minuend_id", "subtrahend_id"]), "SHAPE")?;
            let (a, b) = (text(&expression["minuend_id"]), text(&expression["subtrahend_id"]));
            require(a.is_some() && b.is_some() && a != b, "SHAPE")?;
            Ok(vec![a.unwrap(), b.unwrap()])
        }
        "range_envelope" => {
            require(keys(expression, &["operand_ids", "mode"]) && text(&expression["mode"]).is_some(), "SHAPE")?;
            let mut ids = items(&expression["operand_ids"]).iter().map(text).collect::<Option<Vec<_>>>().ok_or_else(|| "SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_SHAPE".to_string())?;
            require(!ids.is_empty() && expression["operand_ids"].is_array(), "SHAPE")?;
            ids.sort_unstable();
            require(ids.windows(2).all(|w| w[0] != w[1]), "SHAPE")?;
            Ok(ids)
        }
        _ => Err("SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_SHAPE".into()),
    }
}

fn state_token(code: Option<f64>) -> &'static str {
    match code {
        Some(c) if c == 0.0 => "inactive",
        Some(c) if c == 1.0 => "active",
        Some(c) if c == 2.0 => "sticking",
        Some(c) if c == 3.0 => "sliding",
        Some(_) => "unknown",
        None => "unreported",
    }
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
    let mut differing: Vec<(&str, Vec<&str>)> = Vec::new();
    for support in supports {
        let codes: Vec<Option<f64>> = operands.iter().map(|case| states.iter().find(|(c, s, _)| c == case && *s == support).map(|t| t.2)).collect();
        if codes.iter().any(|c| c.is_none() || *c != codes[0]) {
            differing.push((support, codes.into_iter().map(state_token).collect()));
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
    // A combination whose rows carry the linear-combination basis is a mechanics
    // combination: it has no operand record (and the producer admits none in a
    // nonlinear model). Every other admitted combination has one.
    let admitted: Vec<&str> = items(&source["contract_evidence"]["combination_gates"])
        .iter()
        .filter(|g| g["withheld"] == false)
        .filter_map(|g| g["combination_id"].as_str())
        .filter(|id| !rows.iter().any(|r| combination_of(r) == Some(*id) && r["metadata"]["basis"] == LINEAR_BASIS))
        .collect();
    require(member.is_some() == (solved && nonlinear && !admitted.is_empty()), "PRESENCE")?;
    if let Some(member) = member {
        let records = member.as_array().ok_or_else(|| "SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_SHAPE".to_string())?;
        require(records.len() == admitted.len(), "PRESENCE")?;
        let quality = items(&source["numerical_quality"]["cases"]);
        for (record, gate_id) in records.iter().zip(&admitted) {
            require(keys(record, &RECORD_KEYS) && record["standing"] == STANDING, "SHAPE")?;
            let id = text(&record["combination_id"]).filter(|id| id == gate_id).ok_or_else(|| "SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_PRESENCE".to_string())?;
            let order = evaluation_order(record["basis"].as_str().unwrap_or_default(), &record["expression"])?;
            let operands = items(&record["operands"]);
            require(record["operands"].is_array() && operands.len() == order.len(), "SHAPE")?;
            for (operand, case) in operands.iter().zip(&order) {
                require(keys(operand, &OPERAND_KEYS) && operand["load_case_id"] == *case, "SHAPE")?;
                require(operand["modulus_basis_ref"].is_null() || text(&operand["modulus_basis_ref"]).is_some(), "SHAPE")?;
                let t = &operand["modulus_basis_temperature"];
                require(t.is_null() || (keys(t, &["value", "unit"]) && t["value"].as_f64().is_some() && text(&t["unit"]).is_some()), "SHAPE")?;
                let stated = quality.iter().find(|q| q["basis_ref"]["ref_type"] == "load_case" && q["basis_ref"]["ref_id"] == *case);
                require(stated.is_some_and(|q| bit_equal(q, &operand["numerical_quality"])), "STANDING")?;
            }
            // Rows: a row formed from case rows names one source row per operand, in order.
            for row in rows.iter().filter(|r| combination_of(r) == Some(id)) {
                let sources: Vec<Option<&str>> = items(&row["source_result_refs"])
                    .iter()
                    .map(|reference| rows.iter().find(|r| r["id"] == *reference).and_then(case_of))
                    .collect();
                if !sources.is_empty() && sources.iter().all(Option::is_some) {
                    require(sources.iter().map(|c| c.unwrap()).eq(order.iter().copied()), "ROWS")?;
                }
            }
            expected.extend(states_notice(id, &order, rows)?);
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

/// §3, with a supplied model: one record per subtraction or range envelope, in model
/// order, when the model has a nonlinear support, and none otherwise; each record's
/// id, basis, expression and operand modulus selections are the model's.
pub(crate) fn model_checks(source: &Value, model: &Value) -> Check {
    if source["status"]["mechanics"] != "MECHANICS_SOLVED" {
        return Ok(());
    }
    let nonlinear = items(&model["supports"]).iter().any(|s| s.get("nonlinear").is_some_and(|n| !n.is_null()));
    let combinations: Vec<&Value> = match model.get("combinations") {
        None | Some(Value::Null) => Vec::new(),
        Some(Value::Array(list)) => list.iter().filter(|c| c["basis"] != "mechanics").collect(),
        Some(_) => return Err("SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_MODEL".into()),
    };
    let records = items(source["contract_evidence"].get(MEMBER).unwrap_or(&Value::Null));
    let expected = if nonlinear { combinations.len() } else { 0 };
    require(records.len() == expected, "MODEL")?;
    let cases = items(&model["load_cases"]);
    for (record, combination) in records.iter().zip(&combinations) {
        require(record["combination_id"] == combination["id"] && record["basis"] == combination["basis"], "MODEL")?;
        let expression = match combination["basis"].as_str() {
            Some("result_state_subtraction") => json!({"minuend_id": combination["minuend_id"], "subtrahend_id": combination["subtrahend_id"]}),
            _ => json!({"operand_ids": combination["operand_ids"], "mode": combination["mode"]}),
        };
        require(bit_equal(&record["expression"], &expression), "MODEL")?;
        for operand in items(&record["operands"]) {
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
    Ok(())
}

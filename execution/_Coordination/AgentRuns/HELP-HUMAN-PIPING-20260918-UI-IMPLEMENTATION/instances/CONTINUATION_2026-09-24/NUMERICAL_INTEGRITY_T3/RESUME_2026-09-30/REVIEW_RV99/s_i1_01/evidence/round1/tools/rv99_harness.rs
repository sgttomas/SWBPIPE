//! RV99 scratch harness (not a candidate file). Reads RV99_CASES, runs the
//! candidate's interval mode and bounded runner, and the unchanged point path
//! at sample points (with bisection to the predicate's flip points), and writes
//! RV99_OUT. Invented values only.

use std::fs;
use std::panic::{catch_unwind, AssertUnwindSafe};

use open_pipe_stress_expression_evaluator::{
    enclosure_from_bound, evaluate, evaluate_interval, AnalysisStatus, BindingSource, Enclosure,
    EvaluationInput, EvaluationValue, IntervalBinding, IntervalValue, Quantity, Truth,
    VariableBinding,
};
use open_pipe_stress_rule_check_runner::{
    run_rule_checks, run_rule_checks_with_bounds, RuleCheckRunInput, SolverResultBinding,
    SolverResultBound, SuppliedValueBinding,
};
use open_pipe_stress_rule_pack_document::{decode_dimension, decode_expression};
use serde_json::{json, Value};

fn hexf(v: &Value) -> f64 {
    let t = v.as_str().expect("hex");
    f64::from_bits(u64::from_str_radix(t.trim_start_matches("0x"), 16).expect("bits"))
}

fn bits(x: f64) -> String {
    format!("0x{:016x}", x.to_bits())
}

fn dehex(v: &Value) -> Value {
    match v {
        Value::Object(m) => {
            let mut out = serde_json::Map::new();
            for (k, x) in m {
                let is_num = matches!(k.as_str(), "value" | "argument" | "result");
                if is_num && x.as_str().map_or(false, |s| s.starts_with("0x")) {
                    out.insert(k.clone(), Value::from(hexf(x)));
                } else {
                    out.insert(k.clone(), dehex(x));
                }
            }
            Value::Object(out)
        }
        Value::Array(a) => Value::Array(a.iter().map(dehex).collect()),
        other => other.clone(),
    }
}

fn okey(x: f64) -> i64 {
    let b = x.to_bits() as i64;
    if b >= 0 {
        b
    } else {
        -(b & 0x7fff_ffff_ffff_ffff)
    }
}

fn ofromkey(k: i64) -> f64 {
    if k >= 0 {
        f64::from_bits(k as u64)
    } else {
        f64::from_bits(((-k) as u64) | 0x8000_0000_0000_0000)
    }
}

fn truth(t: Truth) -> &'static str {
    match t {
        Truth::True => "T",
        Truth::False => "F",
        Truth::Indeterminate => "U",
    }
}

fn point_input(case: &Value, values: &[f64]) -> EvaluationInput {
    let expression = decode_expression(&dehex(&case["formula"])).expect("decodes");
    let inputs = case["inputs"].as_array().unwrap();
    let bindings = inputs
        .iter()
        .zip(values)
        .map(|(inp, v)| {
            let dim = decode_dimension(inp["dimension"].as_str().unwrap()).unwrap();
            VariableBinding::new(
                inp["id"].as_str().unwrap(),
                BindingSource::SolverResultField,
                Quantity::new(*v, dim, inp["unit_ref"].as_str().unwrap()).unwrap(),
            )
        })
        .collect();
    EvaluationInput {
        expression,
        bindings,
        required_variable_ids: Vec::new(),
        statuses: vec![AnalysisStatus::MechanicsSolved],
        declared_grammar_version: "1.0.0".to_string(),
    }
}

fn point_class(case: &Value, values: &[f64]) -> String {
    let input = point_input(case, values);
    match catch_unwind(AssertUnwindSafe(|| evaluate(&input))) {
        Err(_) => "panic".to_string(),
        Ok(r) if !r.findings.is_empty() => format!(
            "blocked:{}",
            r.findings
                .iter()
                .map(|f| format!("{:?}", f.code))
                .collect::<Vec<_>>()
                .join(",")
        ),
        Ok(r) => match r.value {
            Some(EvaluationValue::Boolean(b)) => format!("bool:{b}"),
            Some(EvaluationValue::Quantity(q)) => format!("q:{}", bits(q.value)),
            None => "none".to_string(),
        },
    }
}

fn bool_class(c: &str) -> &str {
    if c.starts_with("q:") {
        "q"
    } else {
        c
    }
}

fn run_eval(case: &Value) -> Value {
    let inputs = case["inputs"].as_array().unwrap();
    let qs: Vec<f64> = inputs.iter().map(|i| hexf(&i["value"])).collect();
    let mut input = point_input(case, &qs);
    let mut intervals = Vec::new();
    let mut encs = Vec::new();
    for inp in inputs {
        let (q, b) = (hexf(&inp["value"]), hexf(&inp["bound"]));
        if let Some(explicit) = inp.get("enclosure") {
            // Round 1: an explicit overlay (a bit pair, or null for none).
            let e = if explicit.is_null() {
                None
            } else {
                Some(Enclosure { lo: hexf(&explicit[0]), hi: hexf(&explicit[1]) })
            };
            encs.push(json!(e.map(|e| vec![bits(e.lo), bits(e.hi)])));
            intervals.push(IntervalBinding {
                variable_id: inp["id"].as_str().unwrap().to_string(),
                enclosure: e,
            });
        } else if !(b == 0.0) {
            let e = enclosure_from_bound(q, b);
            encs.push(json!(e.map(|e| vec![bits(e.lo), bits(e.hi)])));
            intervals.push(IntervalBinding {
                variable_id: inp["id"].as_str().unwrap().to_string(),
                enclosure: e,
            });
        } else {
            encs.push(json!([bits(q), bits(q)]));
        }
    }
    input.statuses = vec![AnalysisStatus::MechanicsSolved];
    let interval = match catch_unwind(AssertUnwindSafe(|| evaluate_interval(&input, &intervals))) {
        Err(_) => json!({"panic": true}),
        Ok(r) => {
            let value = match &r.value {
                None => Value::Null,
                Some(IntervalValue::Boolean(t)) => json!({"kind": "truth", "truth": truth(*t)}),
                Some(IntervalValue::Quantity(q)) => json!({
                    "kind": "quantity",
                    "enclosure": q.enclosure.map(|e| vec![bits(e.lo), bits(e.hi)]),
                    "dimension": format!("{:?}", q.dimension),
                    "unit_ref": q.unit_ref,
                }),
            };
            json!({
                "panic": false,
                "findings": r.findings.iter().map(|f| vec![format!("{:?}", f.code), f.subject_id.clone()]).collect::<Vec<_>>(),
                "value": value,
                "notes": r.notes.iter().map(|n| vec![n.code.as_str().to_string(), n.subject_id.clone()]).collect::<Vec<_>>(),
            })
        }
    };
    // Point path at the samples, plus bisection to flip points (one input).
    let mut samples: Vec<Vec<f64>> = case["samples"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t.as_array().unwrap().iter().map(hexf).collect())
        .collect();
    let mut classes: Vec<String> = samples.iter().map(|s| point_class(case, s)).collect();
    if case["bisect"].as_bool().unwrap_or(false) && inputs.len() == 1 {
        let mut pairs: Vec<(f64, String)> = samples
            .iter()
            .map(|s| s[0])
            .zip(classes.iter().cloned())
            .collect();
        pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let mut added = Vec::new();
        for w in pairs.windows(2) {
            let (a, ca) = (&w[0].0, bool_class(&w[0].1).to_string());
            let (b, cb) = (&w[1].0, bool_class(&w[1].1).to_string());
            if ca == cb || ca == "q" {
                continue;
            }
            let (mut ka, mut kb) = (okey(*a), okey(*b));
            let mut steps = 0;
            while kb - ka > 1 && steps < 80 {
                let km = ka + (kb - ka) / 2;
                let cm = point_class(case, &[ofromkey(km)]);
                if bool_class(&cm) == ca {
                    ka = km;
                } else {
                    kb = km;
                }
                steps += 1;
            }
            added.push(ofromkey(ka));
            added.push(ofromkey(kb));
        }
        for x in added {
            classes.push(point_class(case, &[x]));
            samples.push(vec![x]);
        }
    }
    json!({
        "id": case["id"],
        "input_enclosures": encs,
        "interval": interval,
        "samples": samples.iter().map(|s| s.iter().map(|x| bits(*x)).collect::<Vec<_>>()).collect::<Vec<_>>(),
        "points": classes,
    })
}

fn pack(case: &Value) -> Value {
    let dim = case["dimension"].as_str().unwrap();
    let decl = |id: &str, kind: &str, unit: &str| {
        json!({"input_id": id, "name": id, "source_kind": kind, "required_for": "rule_check",
            "provenance_required": true, "redistribution_status_required": true,
            "quantity_intent": {"dimension": dim, "unit_ref": unit, "unit_required": true,
                                "dimension_check_required": true}})
    };
    let mut check = json!({
        "check_id": "c1",
        "required_input_refs": [{"ref_id": "actual", "ref_type": "required_input"},
                                {"ref_id": "limit", "ref_type": "required_input"}],
        "formula_ref": {"ref_id": "f1", "ref_type": "formula"},
        "result_statuses": ["RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED"],
        "diagnostic_policy": {"missing_input": "RULE_INPUT_MISSING", "evaluator_error": "RULE_EVALUATOR_ERROR"}
    });
    if !case["slot"].is_null() {
        check["value_slot_refs"] = json!([{"ref_id": "ratio_limit", "ref_type": "value_slot"}]);
    }
    if let Some(r) = case["relation"].as_str() {
        check["acceptability_relation"] = json!(r);
    }
    json!({
        "grammar_version": "1.0.0",
        "metadata": {"rule_pack_id": "rv99_pack"},
        "required_inputs": [decl("actual", "solver_result", case["declared_unit"].as_str().unwrap()),
                            decl("limit", "user_supplied_rule_value", case["limit_unit"].as_str().unwrap())],
        "formula_declarations": [{"formula_id": "f1",
            "declaration_payload": {"expression_ast": dehex(&case["formula"])},
            "input_refs": [{"ref_id": "actual", "ref_type": "required_input"},
                           {"ref_id": "limit", "ref_type": "required_input"}]}],
        "value_slots": [{"slot_id": "ratio_limit", "slot_kind": "ratio_limit",
            "quantity_intent": {"dimension": "dimensionless", "unit_ref": "ratio",
                                "unit_required": true, "dimension_check_required": true}}],
        "check_definitions": [check]
    })
}

fn run_input<'a>(doc: &'a Value, case: &Value, actual: f64) -> RuleCheckRunInput<'a> {
    let mut supplied = vec![SuppliedValueBinding {
        ref_id: "limit".to_string(),
        value: hexf(&case["limit"]),
        unit: case["limit_unit"].as_str().unwrap().to_string(),
        dimension: case["dimension"].as_str().unwrap().to_string(),
    }];
    if !case["slot"].is_null() {
        supplied.push(SuppliedValueBinding {
            ref_id: "ratio_limit".to_string(),
            value: hexf(&case["slot"]),
            unit: "ratio".to_string(),
            dimension: "dimensionless".to_string(),
        });
    }
    RuleCheckRunInput {
        rule_pack_document: doc,
        solver_results: vec![SolverResultBinding {
            input_id: "actual".to_string(),
            result_id: "result:rv99:actual".to_string(),
            value: actual,
            unit: case["entered_unit"].as_str().unwrap().to_string(),
        }],
        refused_solver_results: Vec::new(),
        supplied_values: supplied,
        library_values: Vec::new(),
        current_statuses: vec![AnalysisStatus::MechanicsSolved],
    }
}

fn point_status(doc: &Value, case: &Value, actual: f64) -> String {
    let input = run_input(doc, case, actual);
    match catch_unwind(AssertUnwindSafe(|| run_rule_checks(&input))) {
        Err(_) => "panic".to_string(),
        Ok(r) => r.checks[0].status.as_str().to_string(),
    }
}

fn run_runner(case: &Value) -> Value {
    let doc = pack(case);
    let q = hexf(&case["actual"]);
    let b = hexf(&case["bound"]);
    let input = run_input(&doc, case, q);
    let bounds = vec![SolverResultBound {
        input_id: "actual".to_string(),
        absolute_bound: b,
    }];
    let bounded = match catch_unwind(AssertUnwindSafe(|| run_rule_checks_with_bounds(&input, &bounds))) {
        Err(_) => json!({"panic": true}),
        Ok(r) => {
            let c = &r.checks[0];
            json!({
                "panic": false,
                "status": c.status.as_str(),
                "aggregate": r.aggregate_status.as_str(),
                "diagnostic_codes": c.diagnostic_codes,
                "evaluator_findings": c.evaluator_findings.iter().map(|f| vec![f.code.clone(), f.severity.clone(), f.subject_id.clone(), f.message.clone()]).collect::<Vec<_>>(),
                "completeness_findings": c.completeness_findings.iter().map(|f| vec![f.code.clone(), f.severity.clone()]).collect::<Vec<_>>(),
                "computed_value": c.computed_value.is_some(),
                "limit_value": c.limit_value.as_ref().map(|l| bits(l.value)),
                "acceptability_relation": c.acceptability_relation,
                "notes": c.bound_inputs.iter().map(|bi| bi.note.clone()).collect::<Vec<_>>(),
            })
        }
    };
    // b = 0 must equal the no-bound run byte for byte (D2 §4.11.2).
    let zero_equal = catch_unwind(AssertUnwindSafe(|| {
        let a = serde_json::to_string(&run_rule_checks(&input)).unwrap();
        let z = serde_json::to_string(&run_rule_checks_with_bounds(
            &input,
            &[SolverResultBound { input_id: "actual".to_string(), absolute_bound: 0.0 }],
        ))
        .unwrap();
        a == z
    }))
    .map(|x| json!(x))
    .unwrap_or(json!("panic"));
    let mut samples: Vec<f64> = case["samples"].as_array().unwrap().iter().map(hexf).collect();
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut statuses: Vec<String> = samples.iter().map(|x| point_status(&doc, case, *x)).collect();
    let mut added = Vec::new();
    for i in 0..samples.len().saturating_sub(1) {
        if statuses[i] == statuses[i + 1] {
            continue;
        }
        let (mut ka, mut kb) = (okey(samples[i]), okey(samples[i + 1]));
        let sa = statuses[i].clone();
        let mut steps = 0;
        while kb - ka > 1 && steps < 80 {
            let km = ka + (kb - ka) / 2;
            if point_status(&doc, case, ofromkey(km)) == sa {
                ka = km;
            } else {
                kb = km;
            }
            steps += 1;
        }
        added.push(ofromkey(ka));
        added.push(ofromkey(kb));
    }
    for x in added {
        statuses.push(point_status(&doc, case, x));
        samples.push(x);
    }
    json!({
        "id": case["id"],
        "bounded": bounded,
        "zero_bound_equal": zero_equal,
        "samples": samples.iter().map(|x| bits(*x)).collect::<Vec<_>>(),
        "points": statuses,
    })
}

#[test]
fn rv99_harness() {
    let Ok(path) = std::env::var("RV99_CASES") else {
        return;
    };
    let out = std::env::var("RV99_OUT").expect("RV99_OUT");
    let cases: Value = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    std::panic::set_hook(Box::new(|_| {}));
    let eval: Vec<Value> = cases["eval"].as_array().unwrap().iter().map(run_eval).collect();
    let runner: Vec<Value> = cases["runner"].as_array().unwrap().iter().map(run_runner).collect();
    let _ = std::panic::take_hook();
    fs::write(out, serde_json::to_string(&json!({"eval": eval, "runner": runner})).unwrap()).unwrap();
}

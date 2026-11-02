//! The sizing arithmetic against the frozen independent references R-3, R-3b and
//! the arithmetic part of R-4/R-4b (`validation/references/hanger_sizing/`,
//! DEL-04-11 Design §12). The generator reads no product code. Outcomes, record
//! ids, failure counts and failed tests compare exactly; values compare at
//! 1e-9 relative with a 1e-12 absolute floor.

use open_pipe_stress_hanger_sizing::*;
use serde_json::Value;

const REFERENCES: &str = include_str!("../../../../validation/references/hanger_sizing/r3_arithmetic.json");

fn close(actual: f64, expected: f64) -> bool {
    (actual - expected).abs() <= 1e-9 * actual.abs().max(expected.abs()) + 1e-12
}

fn num(value: &Value) -> f64 {
    value.as_f64().unwrap_or_else(|| panic!("number expected, got {value}"))
}

fn nums(value: &Value) -> Vec<f64> {
    value.as_array().expect("array").iter().map(num).collect()
}

fn pair(value: &Value) -> (f64, f64) {
    let v = nums(value);
    assert_eq!(v.len(), 2);
    (v[0], v[1])
}

fn dof(value: &Value) -> Option<Dof> {
    match value.as_str()? {
        "UX" => Some(Dof::Ux),
        "UY" => Some(Dof::Uy),
        "UZ" => Some(Dof::Uz),
        other => panic!("unknown DOF {other}"),
    }
}

fn method(value: &Value) -> Method {
    Method {
        load_design: match value["load_design"].as_str().unwrap() {
            "hot" => LoadDesign::Hot,
            "cold" => LoadDesign::Cold,
            other => panic!("{other}"),
        },
        variation_reference: match value["variation_reference"].as_str().unwrap() {
            "operating_load" => VariationReference::OperatingLoad,
            "design_load" => VariationReference::DesignLoad,
            other => panic!("{other}"),
        },
        variation_limit: num(&value["variation_limit"]),
        travel_allowance_m: num(&value["travel_allowance_m"]),
        selection_rule: match value["selection_rule"].as_str().unwrap() {
            "smallest_capacity" => SelectionRule::SmallestCapacity,
            "least_variation" => SelectionRule::LeastVariation,
            other => panic!("{other}"),
        },
    }
}

fn record(value: &Value) -> Record {
    Record {
        hanger_id: value["hanger_id"].as_str().unwrap().to_string(),
        hanger_type: match value["hanger_type"].as_str().unwrap() {
            "variable_spring_hanger" => HangerType::VariableSpring,
            "constant_effort_support" => HangerType::ConstantEffort,
            other => panic!("{other}"),
        },
        stiffness_n_per_m: value["stiffness_n_per_m"].as_f64(),
        stiffness_dof: dof(&value["stiffness_dof"]),
        working_load_range_n: (!value["working_load_range_n"].is_null()).then(|| pair(&value["working_load_range_n"])),
        travel_range_m: value["travel_range_m"].as_f64(),
    }
}

fn positions(value: &Value) -> Positions {
    Positions { u_cold_m: num(&value["u_cold_m"]), u_operating_m: nums(&value["u_operating_m"]) }
}

fn candidate(value: &Value) -> Candidate {
    Candidate {
        id: value["id"].as_str().unwrap().to_string(),
        dof: dof(&value["dof"]).expect("candidate DOF"),
        family_policy: match value["family_policy"].as_str().unwrap() {
            "variable" => FamilyPolicy::Variable,
            "constant" => FamilyPolicy::Constant,
            "variable_else_constant" => FamilyPolicy::VariableElseConstant,
            other => panic!("{other}"),
        },
        w_n: num(&value["w_n"]),
        positions: positions(value),
        design_index: value["design_index"].as_u64().unwrap() as usize,
    }
}

fn assert_close(case: &str, what: &str, actual: f64, expected: &Value) {
    let expected = num(expected);
    assert!(close(actual, expected), "{case} {what}: {actual} against {expected}");
}

fn assert_all_close(case: &str, what: &str, actual: &[f64], expected: &Value) {
    let expected = nums(expected);
    assert_eq!(actual.len(), expected.len(), "{case} {what} length");
    for (a, e) in actual.iter().zip(&expected) {
        assert!(close(*a, *e), "{case} {what}: {a} against {e}");
    }
}

fn counts(failures: &Option<std::collections::BTreeMap<Test, usize>>) -> Option<Vec<(String, u64)>> {
    failures.as_ref().map(|map| map.iter().map(|(t, n)| (t.name().to_string(), *n as u64)).collect())
}

fn expected_counts(value: &Value) -> Option<Vec<(String, u64)>> {
    let object = value.as_object()?;
    let order = ["E1", "E2", "V0", "L", "T", "V"];
    Some(order.iter().filter_map(|t| object.get(*t).map(|n| (t.to_string(), n.as_u64().unwrap()))).collect())
}

fn check_case(case: &Value) {
    let id = case["id"].as_str().unwrap();
    let method = method(&case["method"]);
    let records: Vec<Record> = case["records"].as_array().unwrap().iter().map(record).collect();
    let inputs = case["candidates"].as_array().unwrap();
    let candidates: Vec<Candidate> = inputs.iter().map(candidate).collect();
    let expected = &case["expected"];

    if let Some(refused) = expected.get("refused") {
        let refusals = size(&candidates, &records, &method).expect_err(id);
        let code = refused["code"].as_str().unwrap();
        let names: Vec<&str> = refused["candidates"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
        let found = refusals.iter().find(|r| r.code == code).unwrap_or_else(|| panic!("{id}: no {code} in {refusals:?}"));
        assert_eq!(found.names, names, "{id} refused candidates");
        return;
    }

    let results = size(&candidates, &records, &method).unwrap_or_else(|r| panic!("{id}: refused {r:?}"));
    let rows = expected["candidates"].as_array().unwrap();
    assert_eq!(results.len(), rows.len(), "{id} candidate count");
    for ((result, row), (input, cand)) in results.iter().zip(rows).zip(inputs.iter().zip(&candidates)) {
        let who = format!("{id}/{}", result.id);
        assert_eq!(result.id, row["id"].as_str().unwrap());
        let outcome = match result.outcome {
            Outcome::Selected => "selected",
            Outcome::SelectedByFallback => "selected_by_fallback",
            Outcome::NoRecordFits => "no_record_fits",
        };
        assert_eq!(outcome, row["outcome"].as_str().unwrap(), "{who} outcome");
        let selected_id = result.selected.as_ref().map(|s| records[s.record_index()].hanger_id.as_str());
        assert_eq!(selected_id, row["hanger_id"].as_str(), "{who} hanger_id");
        let (span_min, span_max) = pair(&row["span_m"]);
        assert!(close(result.span.min_m, span_min) && close(result.span.max_m, span_max), "{who} span");
        assert_close(&who, "required travel", result.required_travel_m, &row["required_travel_m"]);
        let failures = &row["failure_counts"];
        assert_eq!(counts(&result.variable_failures), expected_counts(&failures["variable"]), "{who} variable counts");
        assert_eq!(counts(&result.constant_failures), expected_counts(&failures["constant"]), "{who} constant counts");
        match &result.selected {
            Some(Selected::Variable { loads, .. }) => {
                assert_close(&who, "F_cold", loads.f_cold_n, &row["f_cold_n"]);
                assert_close(&who, "F_C", loads.f_cold_condition_n, &row["f_cold_condition_n"]);
                assert_all_close(&who, "F_i", &loads.f_operating_n, &row["f_operating_n"]);
                assert_all_close(&who, "v_i", &loads.variation, &row["variation"]);
                let (lo, hi) = pair(&row["envelope_n"]);
                assert!(close(loads.envelope_n.0, lo) && close(loads.envelope_n.1, hi), "{who} envelope");
            }
            Some(Selected::Constant { load_n, .. }) => {
                assert_close(&who, "constant load", *load_n, &row["f_cold_n"]);
                assert!(row["envelope_n"].is_null() && row["variation"].is_null(), "{who} constant has no spring values");
            }
            None => assert!(row["f_cold_n"].is_null(), "{who} no fit has no loads"),
        }

        let expected_verification = &row["verification"];
        if input["verified"].is_null() {
            assert!(expected_verification.is_null(), "{who} verification without verified positions");
            continue;
        }
        let verification = verify(cand, result, &records, &method, &positions(&input["verified"]))
            .unwrap_or_else(|r| panic!("{who}: verification refused {r:?}"));
        let Some(verification) = verification else {
            assert!(expected_verification.is_null(), "{who} no-fit verification");
            continue;
        };
        let ev = expected_verification;
        let (vmin, vmax) = pair(&ev["span_m"]);
        assert!(close(verification.span.min_m, vmin) && close(verification.span.max_m, vmax), "{who} verified span");
        assert_close(&who, "verified required travel", verification.required_travel_m, &ev["required_travel_m"]);
        assert_eq!(verification.t_pass, ev["t_pass"].as_bool().unwrap(), "{who} T");
        let failed: Vec<&str> = verification.failed_tests.iter().map(|t| t.name()).collect();
        let expected_failed: Vec<&str> = ev["failed_tests"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
        assert_eq!(failed, expected_failed, "{who} failed tests");
        match &verification.variable {
            Some(v) => {
                assert_close(&who, "F_C^v", v.f_cold_condition_n, &ev["f_cold_condition_n"]);
                assert_all_close(&who, "F_i^v", &v.f_operating_n, &ev["f_operating_n"]);
                let variation: Vec<f64> = v.variation.iter().map(|x| x.expect("formed ratio")).collect();
                assert_all_close(&who, "v_i^v", &variation, &ev["variation"]);
                assert_close(&who, "departure", v.departure_n, &ev["departure_n"]);
            }
            None => assert!(ev["f_cold_condition_n"].is_null() && ev["variation"].is_null(), "{who} constant verification"),
        }
    }
    let standing = match verification_standing(&results) {
        VerificationStanding::Unconditional => "unconditional",
        VerificationStanding::Conditional => "conditional",
    };
    assert_eq!(standing, expected["verification_standing"].as_str().unwrap(), "{id} standing");
}

#[test]
fn every_frozen_case_is_reproduced() {
    let document: Value = serde_json::from_str(REFERENCES).expect("references parse");
    assert_eq!(document["schema"], "swbpipe.hanger_sizing_reference.r3/1");
    let cases = document["cases"].as_array().unwrap();
    assert!(cases.len() >= 25, "the reference set has {} cases", cases.len());
    for case in cases {
        check_case(case);
    }
}

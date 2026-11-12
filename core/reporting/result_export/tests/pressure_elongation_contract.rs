//! RT-L2 (DEL-04-07 Design `pressure-elongation.md` §5): the pressure-3 readers are
//! switch-aware, keyed on each case's `pressure_elongation` evidence. Included: the
//! straight Poisson group's coefficient is sign·2ν and a realized arc carries its signed
//! nonzero free strain. Not included: sign·1 and exactly zero. A case whose evidence and
//! coefficients (or arc strain) disagree is refused by name, as is a case whose regions,
//! or whose assembly, state different settings.
//!
//! The published formulation limitations carry exactly one of the three fixed
//! elongation statements, the included one whenever a region states `included`; any
//! other is refused by name (`SOURCE_PHYSICS_PRESSURE_ELONGATION_STATEMENT_MISMATCH`).
//!
//! The inputs are the committed straight and arc reader corpora with the member stamped
//! and, for "not included", the Poisson coefficients and arc strains re-formed by hand,
//! and the not-included statement published, as the producer publishes it without the
//! setting: the readers check bindings and declared semantics, never the assembled
//! values, so these hand-built statements exercise exactly the switch.
use open_pipe_stress_result_export::semantic_contract as s;
use serde_json::{json, Value};

fn corpus(text: &str) -> Vec<(String, Value)> {
    let corpus: Value = serde_json::from_str(text).unwrap();
    corpus["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|case| (case["label"].as_str().unwrap().to_string(), case["envelope"].clone()))
        .collect()
}
fn straight() -> Vec<(String, Value)> {
    corpus(include_str!("../../../../fixtures/results/pressure_v3_straight_reader_corpus.json"))
}
fn arcs() -> Vec<(String, Value)> {
    corpus(include_str!("../../../../fixtures/results/pressure_v3_arc_reader_corpus.json"))
        .into_iter()
        .filter(|(_, e)| arc_loads(e).next().is_some())
        .collect()
}
fn arc_loads(e: &Value) -> impl Iterator<Item = &Value> {
    e["contract_evidence"]["pressure"].as_array().unwrap().iter().flat_map(|region| {
        region["applied_loads"].as_array().unwrap().iter().filter(|load| load.get("arc_pressure_strain").is_some())
    })
}
fn token(included: bool) -> &'static str {
    if included { "included" } else { "not_included" }
}
const NOT_INCLUDED: &str = "Pressure elongation is not included in any load case: the closed-end free pressure strain is omitted; pressure thrust at unbalanced locations is applied.";
const INCLUDED: &str = "Pressure elongation is included only in the operating load cases the model settings name, as the closed-end free strain (1 \u{2212} 2\u{3bd})\u{b7}p\u{b7}A_i/(E\u{b7}A_s), with A_i on the pressure bore and A_s the nominal section area; it is not a thin-wall mean-diameter form.";
const NO_EFFECT: &str = "Pressure elongation is set, but none of the cases it names carries pressure; it has no effect.";
/// `e` with its elongation statement replaced by `statements` (none, one or several), in place.
fn with_statements(e: &Value, statements: &[&str]) -> Value {
    let mut e = e.clone();
    let list = e["formulation_basis"]["limitations"].as_array_mut().unwrap();
    let at = list.iter().position(|text| [NOT_INCLUDED, INCLUDED, NO_EFFECT].contains(&text.as_str().unwrap())).expect("an elongation statement");
    list.splice(at..=at, statements.iter().map(|text| json!(text)));
    e
}
fn poisson_terms(e: &mut Value) -> impl Iterator<Item = &mut Value> {
    e["contract_evidence"]["exact_cases"].as_array_mut().unwrap().iter_mut().flat_map(|case| {
        case["pressure_rhs_assembly"]["groups"].as_array_mut().unwrap().iter_mut().flat_map(|group| {
            group.get_mut("terms").and_then(Value::as_array_mut).into_iter().flatten().filter(|term| term["kind"] == "poisson_eigen")
        })
    })
}

/// The corpus statement as pressure-3 with every case's setting stated; for "not included"
/// the Poisson coefficients become sign·1 and every arc's free strain exactly zero.
fn stamped(envelope: &Value, included: bool) -> Value {
    let mut e = envelope.clone();
    e["producer"]["semantic_contract_id"] = json!(s::PRESSURE_ID);
    let mut pressurized = std::collections::HashSet::new();
    for region in e["contract_evidence"]["pressure"].as_array_mut().unwrap() {
        region["pressure_elongation"] = json!(token(included));
        pressurized.insert(region["load_case_id"].as_str().unwrap().to_string());
        if !included {
            for load in region["applied_loads"].as_array_mut().unwrap() {
                if load.get("arc_pressure_strain").is_some() {
                    load["arc_pressure_strain"] = json!(0.0);
                }
            }
        }
    }
    for case in e["contract_evidence"]["exact_cases"].as_array_mut().unwrap() {
        if pressurized.contains(case["load_case_id"].as_str().unwrap()) {
            case["pressure_rhs_assembly"]["pressure_elongation"] = json!(token(included));
        }
    }
    if !included {
        for term in poisson_terms(&mut e) {
            let c = term["coefficient"].as_f64().unwrap();
            term["coefficient"] = json!(if c.is_sign_negative() { -1.0 } else { 1.0 });
        }
    }
    with_statements(&e, &[if included { INCLUDED } else { NOT_INCLUDED }])
}

fn refusal(e: &Value) -> String {
    s::validate_pressure_evidence(e).unwrap_err()
}
/// The refusal ends with `code` (a load/reference-state statement wraps the physics
/// refusal in its own prefix, and names its region shape refusal as its own).
#[track_caller]
fn assert_refused(e: &Value, code: &str, what: std::fmt::Arguments) {
    let error = refusal(e);
    assert!(error.ends_with(code), "{what}: {error}");
}

#[test]
fn both_settings_are_admitted_on_straight_and_arc_corpora() {
    let mut poisson = 0;
    let mut arc = 0;
    for (label, envelope) in straight().into_iter().chain(arcs()) {
        for included in [true, false] {
            let e = stamped(&envelope, included);
            s::validate_pressure_evidence(&e).unwrap_or_else(|error| panic!("{label} {included}: {error}"));
            s::for_source(&e).unwrap_or_else(|error| panic!("{label} {included}: {error}"));
            poisson += poisson_terms(&mut e.clone()).count();
            arc += arc_loads(&e).count();
        }
    }
    // The switch is exercised: straight Poisson terms and realized arcs, in both settings.
    assert!(poisson > 0 && arc > 0, "{poisson} {arc}");
}

#[test]
fn a_straight_coefficient_of_the_other_setting_is_refused_by_name() {
    for (label, envelope) in straight() {
        // Included evidence with κ = 1, and not-included evidence with κ = 2ν.
        let mut included = stamped(&envelope, true);
        for term in poisson_terms(&mut included) {
            let c = term["coefficient"].as_f64().unwrap();
            term["coefficient"] = json!(if c.is_sign_negative() { -1.0 } else { 1.0 });
        }
        let mut excluded = stamped(&envelope, false);
        let original = stamped(&envelope, true);
        let mut originals = poisson_terms(&mut original.clone()).map(|t| t["coefficient"].clone()).collect::<Vec<_>>().into_iter();
        for term in poisson_terms(&mut excluded) {
            term["coefficient"] = originals.next().unwrap();
        }
        for (what, e) in [("included with kappa 1", included), ("not included with kappa 2nu", excluded)] {
            assert_refused(&e, "SOURCE_PHYSICS_PRESSURE_ELONGATION_COEFFICIENT_MISMATCH", format_args!("{label}: {what}"));
        }
        // A coefficient of neither setting keeps its own refusal.
        let mut neither = stamped(&envelope, false);
        poisson_terms(&mut neither).next().unwrap()["coefficient"] = json!(0.75);
        assert_refused(&neither, "SOURCE_PHYSICS_RHS_EIGEN_COEFFICIENT", format_args!("{label}"));
    }
}

#[test]
fn an_arc_strain_of_the_other_setting_is_refused_by_name() {
    for (label, envelope) in arcs() {
        // Included evidence with a zero strain; not-included evidence with the signed strain.
        let mut included = stamped(&envelope, true);
        let mut excluded = stamped(&envelope, false);
        for region in included["contract_evidence"]["pressure"].as_array_mut().unwrap() {
            for load in region["applied_loads"].as_array_mut().unwrap() {
                if load.get("arc_pressure_strain").is_some() {
                    load["arc_pressure_strain"] = json!(0.0);
                }
            }
        }
        let strains: Vec<Value> = arc_loads(&stamped(&envelope, true)).map(|l| l["arc_pressure_strain"].clone()).collect();
        let mut strains = strains.into_iter();
        for region in excluded["contract_evidence"]["pressure"].as_array_mut().unwrap() {
            for load in region["applied_loads"].as_array_mut().unwrap() {
                if load.get("arc_pressure_strain").is_some() {
                    load["arc_pressure_strain"] = strains.next().unwrap();
                }
            }
        }
        for (what, e) in [("included with zero strain", included), ("not included with the free strain", excluded)] {
            assert_refused(&e, "SOURCE_PHYSICS_ARC_PRESSURE_ELONGATION_MISMATCH", format_args!("{label}: {what}"));
        }
        // A wrong-signed strain on a not-included arc is no setting's: the applied-load refusal.
        let mut wrong = stamped(&envelope, false);
        let p_negative = |region: &Value| region["p_pa"].as_f64().unwrap() < 0.0;
        for region in wrong["contract_evidence"]["pressure"].as_array_mut().unwrap() {
            let sign = if p_negative(region) { 1.0 } else { -1.0 };
            for load in region["applied_loads"].as_array_mut().unwrap() {
                if load.get("arc_pressure_strain").is_some() {
                    load["arc_pressure_strain"] = json!(sign * 1e-6);
                }
            }
        }
        assert_refused(&wrong, "SOURCE_PHYSICS_ARC_APPLIED_LOAD", format_args!("{label}"));
    }
}

#[test]
fn the_setting_is_required_closed_and_agreed_within_a_case() {
    for (label, envelope) in straight().into_iter().chain(arcs()) {
        let base = stamped(&envelope, true);
        // A region without the member, or with an unknown value.
        let mut missing = base.clone();
        missing["contract_evidence"]["pressure"][0].as_object_mut().unwrap().remove("pressure_elongation");
        assert_refused(&missing, "REGION_SHAPE", format_args!("{label}"));
        let mut unknown = base.clone();
        unknown["contract_evidence"]["pressure"][0]["pressure_elongation"] = json!("on");
        assert_refused(&unknown, "SOURCE_PHYSICS_PRESSURE_ELONGATION_INVALID", format_args!("{label}"));
        // The assembly disagrees with its case's regions, or omits the member.
        let case = base["contract_evidence"]["pressure"][0]["load_case_id"].clone();
        let at = base["contract_evidence"]["exact_cases"].as_array().unwrap().iter().position(|c| c["load_case_id"] == case).unwrap();
        let mut assembly = base.clone();
        assembly["contract_evidence"]["exact_cases"][at]["pressure_rhs_assembly"]["pressure_elongation"] = json!("not_included");
        assert_refused(&assembly, "SOURCE_PHYSICS_PRESSURE_ELONGATION_CASE_MISMATCH", format_args!("{label}"));
        let mut absent = base.clone();
        absent["contract_evidence"]["exact_cases"][at]["pressure_rhs_assembly"].as_object_mut().unwrap().remove("pressure_elongation");
        assert_refused(&absent, "SOURCE_PHYSICS_RHS_SHAPE", format_args!("{label}"));
    }
    // Two regions of one case stating different settings (a case with two regions).
    let two = straight()
        .into_iter()
        .chain(arcs())
        .map(|(label, e)| (label, stamped(&e, true)))
        .find(|(_, e)| {
            let regions = e["contract_evidence"]["pressure"].as_array().unwrap();
            regions.iter().enumerate().any(|(i, r)| regions[i + 1..].iter().any(|q| q["load_case_id"] == r["load_case_id"]))
        });
    if let Some((label, mut e)) = two {
        let regions = e["contract_evidence"]["pressure"].as_array_mut().unwrap();
        let case = regions[0]["load_case_id"].clone();
        let second = regions.iter().skip(1).position(|r| r["load_case_id"] == case);
        if let Some(i) = second {
            regions[i + 1]["pressure_elongation"] = json!("not_included");
            assert_refused(&e, "SOURCE_PHYSICS_PRESSURE_ELONGATION_CASE_MISMATCH", format_args!("{label}"));
        }
    }
}

#[test]
fn a_pressure_2_statement_is_refused_by_name() {
    for (label, envelope) in straight() {
        let mut e = stamped(&envelope, true);
        e["producer"]["semantic_contract_id"] = json!(s::RETIRED_PRESSURE_2_ID);
        for refusal in [s::for_source(&e).unwrap_err(), s::for_source_metadata(&e).unwrap_err()] {
            assert_eq!(refusal, format!("{}: pressure-2 results predate the pressure elongation setting and are no longer read; re-solve the model", s::SOURCE_PRESSURE_2_RETIRED), "{label}");
        }
    }
}

/// RT-L2: the reader refuses a statement the regions contradict. A region stating
/// `included` calls for the included statement. Without one, any of the three can be
/// true of the document (a named pressurised case may have no region evidence in the
/// envelope), so each is admitted. No statement, or two, is refused.
#[test]
fn a_statement_the_regions_contradict_is_refused_by_name() {
    const CODE: &str = "SOURCE_PHYSICS_PRESSURE_ELONGATION_STATEMENT_MISMATCH";
    for (label, envelope) in straight().into_iter().chain(arcs()) {
        let included = stamped(&envelope, true);
        let excluded = stamped(&envelope, false);
        for (what, e) in [
            ("included regions, not-included statement", with_statements(&included, &[NOT_INCLUDED])),
            ("included regions, no-effect statement", with_statements(&included, &[NO_EFFECT])),
            ("no statement", with_statements(&included, &[])),
            ("two statements", with_statements(&included, &[INCLUDED, NO_EFFECT])),
            ("the same statement twice", with_statements(&excluded, &[NOT_INCLUDED, NOT_INCLUDED])),
        ] {
            assert_refused(&e, CODE, format_args!("{label}: {what}"));
        }
        for statement in [NOT_INCLUDED, NO_EFFECT, INCLUDED] {
            s::validate_pressure_evidence(&with_statements(&excluded, &[statement]))
                .unwrap_or_else(|error| panic!("{label}: not-included regions admit {statement:.40}: {error}"));
        }
    }
    // A load/reference-state envelope reads the same rule (the refusal carries its prefix).
    let load_state: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/product_preview/pressure/load_reference/pressure-sparse_interactive.raw.json"
    ))
    .unwrap();
    s::validate_pressure_evidence(&load_state).unwrap_or_else(|error| panic!("load/reference state: {error}"));
    assert_refused(&with_statements(&load_state, &[NOT_INCLUDED]), CODE, format_args!("load/reference state"));
    // The producer's real no-effect envelope (a connector document naming a case without
    // a pressure region) is admitted with its statement; two statements are refused.
    let connector: Value = serde_json::from_str(include_str!("../../../../fixtures/results/pressure_v3_connector_reader_corpus.json")).unwrap();
    let no_effect = connector["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|case| case["envelope"].clone())
        .find(|e| e["formulation_basis"]["limitations"].as_array().unwrap().iter().any(|text| text == NO_EFFECT))
        .expect("the connector corpus's no-effect envelope");
    s::validate_pressure_evidence(&no_effect).unwrap_or_else(|error| panic!("no-effect envelope: {error}"));
    assert_refused(&with_statements(&no_effect, &[NO_EFFECT, NOT_INCLUDED]), CODE, format_args!("no-effect envelope"));
}

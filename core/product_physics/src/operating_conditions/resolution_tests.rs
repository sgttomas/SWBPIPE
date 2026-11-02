//! S2 unit tests (DEL-02-01 Work `model-document-0.9.0.md` §1.5, §1.5.1, §1.8):
//! E3 against the independent reference table, E1 (the lowered typed model
//! equals the decoded independent twin), the plan's non-mutation, and the
//! multisets G-1 will read after the lowering (E7's precondition).
use super::*;
use crate::case_state::temperature::canonical_decimal::{canonical_decimal, kelvin_numerator, CanonicalDecimal};
use crate::case_state::temperature::exact_kelvin;
use crate::operating_conditions::{findings, lower, preview_operating_conditions, OPERATING_CONDITION_TEMPERATURE_RANGE_UNSUPPORTED};
use crate::document_members::MemberFindings;
use serde_json::{json, Value};

#[path = "../../tests/common/condition_twin.rs"]
mod twin;

fn f(bits: &Value) -> f64 {
    f64::from_bits(u64::from_str_radix(bits.as_str().unwrap(), 16).unwrap())
}

fn quantity(operand: &Value) -> Quantity {
    Quantity { value: f(&operand["bits"]), unit: operand["unit"].as_str().unwrap().into() }
}

/// (negative, digits, exponent) of a decimal text such as `-12e-3` or `0.0078`.
fn parse(text: &str) -> CanonicalDecimal {
    let (negative, text) = text.strip_prefix('-').map_or((false, text), |rest| (true, rest));
    let (mantissa, exponent) = text.split_once(['e', 'E']).unwrap_or((text, "0"));
    let (whole, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let mut digits: u64 = format!("{whole}{fraction}").parse().unwrap();
    let mut exponent = exponent.parse::<i32>().unwrap() - fraction.len() as i32;
    if digits == 0 {
        return CanonicalDecimal { negative: false, digits: 0, exponent: 0 };
    }
    while digits % 10 == 0 {
        digits /= 10;
        exponent += 1;
    }
    CanonicalDecimal { negative, digits, exponent }
}

fn rows() -> Vec<Value> {
    twin::references()["temperature_differences"].as_array().unwrap().clone()
}

/// The document-level code of one set holding (T, T₀) as a condition and the
/// installation temperature.
fn document_codes(temperature: Quantity, installation: Quantity) -> Vec<&'static str> {
    let mut set: ConditionSetInput = serde_json::from_value(json!({"id": "cs", "provenance": "p",
        "conditions": [{"key": "K", "temperature": {"value": 0.0, "unit": "K"}}],
        "installation_temperature": {"value": 0.0, "unit": "K"}})).unwrap();
    set.conditions.as_mut().unwrap()[0].temperature = Some(temperature);
    set.installation_temperature = Some(installation);
    let mut found = MemberFindings::default();
    findings(&[set], &[], &[], &mut found);
    found.into_vec().into_iter().map(|finding| finding.code).collect()
}

/// E3: every admitted row's ΔT has the reference bits; the operands' integer
/// numerators are the reference's (and order as `exact_kelvin` orders them); every
/// refused row is refused at document level with its code.
#[test]
fn e3_every_reference_row_gives_its_bits_or_its_refusal() {
    let (mut admitted, mut refused) = (0, 0);
    for row in rows() {
        let id = row["id"].as_str().unwrap();
        let (t, t0) = (quantity(&row["temperature"]), quantity(&row["installation"]));
        if let Some(code) = row.get("refused") {
            let codes = document_codes(t.clone(), t0.clone());
            assert!(codes.contains(&code.as_str().unwrap()), "{id}: {codes:?}");
            if code == OPERATING_CONDITION_TEMPERATURE_RANGE_UNSUPPORTED {
                assert_eq!(exact_temperature_difference(&t, &t0), Err(DifferenceError::Range), "{id}");
            }
            refused += 1;
            continue;
        }
        assert!(document_codes(t.clone(), t0.clone()).is_empty(), "{id}: admitted");
        let got = exact_temperature_difference(&t, &t0).unwrap();
        let want = f(&row["expected"]["bits"]);
        assert_eq!(got.to_bits(), want.to_bits(), "{id}: {got:e} against {want:e}");
        for (member, q) in [("temperature", &t), ("installation", &t0)] {
            let unit = unit_by_symbol(&q.unit, Dimension::Temperature).unwrap();
            let n = kelvin_numerator(canonical_decimal(q.value).unwrap(), unit).unwrap();
            assert_eq!(n.to_string(), row["numerators"][member].as_str().unwrap(), "{id}: {member}");
        }
        // The EX identity reads the same decimal: equality and order agree with ΔT.
        let order = exact_kelvin(&t).unwrap().cmp(&exact_kelvin(&t0).unwrap());
        assert_eq!(order, got.partial_cmp(&0.0).unwrap(), "{id}: exact_kelvin order");
        if got == 0.0 {
            assert_eq!(got.to_bits(), 0, "{id}: an exact zero is +0.0");
        }
        admitted += 1;
    }
    assert!(admitted >= 30 && refused >= 8, "{admitted} admitted, {refused} refused");
}

/// The mutants E3 names give other bits on the table's rows: the binary64 affine
/// route, conversion of each temperature before subtracting, the formatter's
/// decimal on the tie row, and a 128-bit intermediate (the width row's shifted
/// numerator exceeds `i128`).
#[test]
fn e3_the_named_mutants_are_killed_by_the_table() {
    let table: Vec<Value> = rows();
    let row = |id: &str| table.iter().find(|row| row["id"] == id).unwrap().clone();
    for id in ["celsius_typed_decimals", "fahrenheit_212_70"] {
        let row = row(id);
        let affine = row["binary64_affine_route"].as_f64().unwrap();
        assert_ne!(affine.to_bits(), f(&row["expected"]["bits"]).to_bits(), "{id}: the affine route differs");
        // Converting each operand to binary64 kelvin before subtracting.
        let kelvin = |q: Quantity| open_pipe_stress_units::convert(q.value, unit_by_symbol(&q.unit, Dimension::Temperature).unwrap(), open_pipe_stress_units::UnitId::Kelvin).unwrap();
        let converted = kelvin(quantity(&row["temperature"])) - kelvin(quantity(&row["installation"]));
        assert_ne!(converted.to_bits(), f(&row["expected"]["bits"]).to_bits(), "{id}: convert-then-subtract differs");
    }
    let tie = row("decimal_tie_regression");
    assert_eq!(tie["expected"]["bits"], "3f4aafbe76c8b435");
    assert_eq!(tie["formatter_route_bits"], "3f4aafbe76c8b43e");
    assert_eq!(canonical_decimal(f(&tie["temperature"]["bits"])).unwrap(), parse("0.007814407348632812"));
    let width = row("width_regression");
    assert_eq!(width["expected"]["bits"], "3af357c299a88ea7");
    let n: u128 = width["numerators"]["difference"].as_str().unwrap().parse().unwrap();
    assert_eq!(n, 9);
    // The shifted numerator 9*2^132 has 136 bits: a signed 128-bit intermediate overflows.
    assert_eq!((128 - n.leading_zeros()) + 132, 136);
    // The disclosure: a typed 7 degF interval keeps the unchanged interval conversion.
    let disclosure = row("fahrenheit_77_70");
    assert_eq!(disclosure["expected"]["bits"], "400f1c71c71c71c7");
    let typed = open_pipe_stress_units::convert(7.0, unit_by_symbol("degF", Dimension::TemperatureInterval).unwrap(),
        unit_by_symbol("K", Dimension::TemperatureInterval).unwrap()).unwrap();
    assert_eq!(format!("{:016x}", typed.to_bits()), disclosure["typed_interval"]["bits"].as_str().unwrap());
    assert_eq!(disclosure["typed_interval"]["bits"], "400f1c71c71c71c8");
}

/// The decimal stage against Python `repr` on the table's seeded samples, and the
/// tie rows where Rust's own formatter takes the other candidate.
#[test]
fn e3_the_canonical_decimal_matches_repr_and_lists_the_formatter_ties() {
    let table = twin::references();
    let samples = table["decimal_samples"].as_array().unwrap();
    assert!(samples.len() >= 400);
    for sample in samples {
        assert_eq!(canonical_decimal(f(&sample["bits"])), Ok(parse(sample["repr"].as_str().unwrap())), "{sample}");
    }
    for tie in table["decimal_ties"].as_array().unwrap() {
        let v = f(&tie["bits"]);
        let canonical = canonical_decimal(v).unwrap();
        assert_eq!(canonical, parse(tie["canonical"].as_str().unwrap()), "{tie}");
        assert_eq!(canonical, parse(tie["repr"].as_str().unwrap()), "{tie}");
        let rust = parse(&format!("{v:e}"));
        assert_eq!(rust, parse(tie["formatter"].as_str().unwrap()), "{tie}: Rust's formatter takes the other candidate");
        assert_ne!(rust, canonical, "{tie}");
    }
}

fn decode(document: &Value) -> PreviewModel {
    serde_json::from_value(document.clone()).expect("the document decodes")
}

fn lowered(document: &Value) -> (PreviewModel, Vec<Diagnostic>) {
    let mut model = decode(document);
    let mut diagnostics = Vec::new();
    lower(&mut model, &mut diagnostics);
    (model, diagnostics)
}

fn structure(model: &PreviewModel) -> String {
    format!("{model:#?}")
}

fn variants() -> Vec<(&'static str, Value)> {
    let base = twin::document(crate::MODEL_SCHEMA_VERSION);
    let mut wind_only = base.clone();
    wind_only["load_cases"] = json!([base["load_cases"][2].clone()]);
    wind_only["combinations"] = json!([]);
    let mut none_exposed = wind_only.clone();
    for set in none_exposed["condition_sets"].as_array_mut().unwrap() {
        set.as_object_mut().unwrap().remove("exposure");
    }
    let mut no_added_weight = base.clone();
    for set in no_added_weight["condition_sets"].as_array_mut().unwrap() {
        set.as_object_mut().unwrap().remove("added_weight");
    }
    // Equal operating and installation temperatures (the table's 20 degC row): +0.0 K.
    let mut equal = base.clone();
    equal["load_cases"][0]["condition_use"] = json!({"temperature": "HYDRO"});
    for set in equal["condition_sets"].as_array_mut().unwrap() {
        set["installation_temperature"] = json!({"value": 20.0, "unit": "degC"});
        set["conditions"] = json!([{"key": "HYDRO", "temperature": {"value": 20.0, "unit": "degC"}}]);
    }
    vec![("base", base), ("wind_only", wind_only), ("none_exposed", none_exposed), ("no_added_weight", no_added_weight), ("equal_temperatures", equal)]
}

/// E1: `lower(decode(D))` equals `decode(T(D))` as typed models, for each
/// variant; the twin comes from the independent builder, not from `lower`. The
/// mutant twins (generated records first, or in set order) do not.
#[test]
fn e1_the_lowered_model_is_the_decoded_twin() {
    for (name, document) in variants() {
        let (model, diagnostics) = lowered(&document);
        assert!(diagnostics.is_empty(), "{name}: {diagnostics:?}");
        assert_eq!(structure(&model), structure(&decode(&twin::twin(&document))), "{name}");
    }
    let document = twin::document(crate::MODEL_SCHEMA_VERSION);
    let (model, _) = lowered(&document);
    for order in [twin::Order::Prepended, twin::Order::SetOrder] {
        assert_ne!(structure(&model), structure(&decode(&twin::twin_with(&document, order))), "a mutant twin differs");
    }
    // The generated records, spelled out once.
    let ope = &model.load_cases[0].primitive_loads;
    let ids: Vec<&str> = ope.iter().map(|load| load.id.as_str()).collect();
    assert_eq!(ids, ["OPE:w1", "OPE:w2", "OPE:w3",
        "load:condition-temperature:3:OPE:2:P1", "load:condition-temperature:3:OPE:2:P2", "load:condition-temperature:3:OPE:2:P3",
        "load:condition-added-weight:3:OPE:2:P2", "load:condition-added-weight:3:OPE:2:P3"]);
    assert_eq!(ope[3].provenance.as_deref(), Some(r#"{"condition":"OPE1","condition_set":"cs:warm","method":"operating_condition_set_v1","quantity":"temperature"}"#));
    assert_eq!(ope[6].provenance.as_deref(), Some(r#"{"condition_set":"cs:hot","method":"operating_condition_set_v1","quantity":"added_weight"}"#));
    assert_eq!([ope[3].magnitude.value, ope[4].magnitude.value, ope[5].magnitude.value], [78.88888888888889, 130.0, 80.0]);
    assert!(ope[3..6].iter().all(|load| load.magnitude.unit == "K" && load.direction == "global_z" && load.dimension == "temperature_change"));
    assert_eq!(model.load_cases[2].equivalent_static.as_ref().unwrap().wind.as_ref().unwrap().exposed_pipe_refs, ["P2", "P3"]);
    assert!(model.condition_sets.is_empty() && model.pipe_segments.iter().all(|p| p.condition_set_ref.is_none())
        && model.load_cases.iter().all(|c| c.condition_use.is_none()));
}

/// Generated ids are length-prefixed, so a case and pipe whose ids contain the
/// separator still give distinct ids; JCS escapes a quote and a control in a key.
#[test]
fn generated_identity_is_injective_and_provenance_is_jcs() {
    assert_ne!(temperature_load_id("a:3:b", "c"), temperature_load_id("a", "b:1:c"));
    assert_eq!(temperature_load_id("a:3:b", "c"), "load:condition-temperature:5:a:3:b:1:c");
    assert_eq!(added_weight_load_id("é", "p"), "load:condition-added-weight:2:é:1:p");
    assert_eq!(temperature_provenance("s\"1", "k\n\u{1}/é"),
        "{\"condition\":\"k\\n\\u0001/é\",\"condition_set\":\"s\\\"1\",\"method\":\"operating_condition_set_v1\",\"quantity\":\"temperature\"}");
}

/// The plan reads the model and does not change it; applying it is the lowering's
/// only change besides removing the members (so a preview of the plan shows
/// exactly what the solve applies).
#[test]
fn the_plan_does_not_mutate_and_is_what_the_lowering_applies() {
    let document = twin::document(crate::MODEL_SCHEMA_VERSION);
    let model = decode(&document);
    let before = structure(&model);
    let planned = plan(&model).expect("the base resolves");
    assert_eq!(structure(&model), before, "planning is read-only");
    let mut applied = model.clone();
    planned.clone().apply(&mut applied);
    let (lowered_model, _) = lowered(&document);
    for (a, b) in applied.load_cases.iter().zip(&lowered_model.load_cases) {
        assert_eq!(format!("{:?}", a.primitive_loads), format!("{:?}", b.primitive_loads));
        assert_eq!(format!("{:?}", a.equivalent_static), format!("{:?}", b.equivalent_static));
    }
    assert_eq!(planned.cases.iter().map(|c| c.case_id.as_str()).collect::<Vec<_>>(), ["OPE", "SUS", "WIND"]);
}

/// A refusal anywhere leaves no generated record in any case: the refused
/// document's cases keep only their authored primitives.
#[test]
fn a_refusal_leaves_no_generated_record() {
    let mut document = twin::document(crate::MODEL_SCHEMA_VERSION);
    document["load_cases"][1]["condition_use"] = json!({"added_weight": true, "temperature": "HYDRO"});
    let (model, diagnostics) = lowered(&document);
    assert!(diagnostics.iter().any(|d| d.code == OPERATING_CONDITION_KEY_MISSING), "{diagnostics:?}");
    for case in &model.load_cases {
        assert!(case.primitive_loads.iter().all(|load| !load.id.starts_with("load:condition-")), "{}", case.id);
    }
    assert!(model.load_cases[2].equivalent_static.as_ref().unwrap().wind.as_ref().unwrap().exposed_pipe_refs.is_empty());
}

/// G-1's content key: category, target, direction, dimension, value bits, unit.
fn content(load: &PreviewPrimitiveLoad) -> String {
    format!("{}|{:?}|{}|{}|{:016x}|{}", load.category, load.target, load.direction, load.dimension, load.magnitude.value.to_bits(), load.magnitude.unit)
}

fn count(model: &PreviewModel, case: usize, key: &str) -> usize {
    model.load_cases[case].primitive_loads.iter().filter(|load| content(load) == key).count()
}

/// E7's precondition (G-1 itself is the solver lane's 1b, not on main): after the
/// lowering the generated thermal primitives are in the operating case only, and
/// the added weight typed by hand in OPE and drawn from the set in both OPE and
/// SUS counts twice in the minuend and once in the subtrahend.
#[test]
fn e7_the_lowered_multisets_g1_reads() {
    let (model, _) = lowered(&twin::document(crate::MODEL_SCHEMA_VERSION));
    let thermal = |case: usize| model.load_cases[case].primitive_loads.iter().filter(|l| l.category == "thermal").count();
    assert_eq!((thermal(0), thermal(1)), (3, 0));
    let weight = content(&model.load_cases[1].primitive_loads[0]);
    assert!(weight.starts_with("weight|Element { pipe: \"P2\" }|global_z|force_per_length|"), "{weight}");
    assert_eq!((count(&model, 0, &weight), count(&model, 1, &weight)), (2, 1));
}

/// The wind input's private presence (§1.3's conflict counts an authored `[]`):
/// each list absent, empty and non-empty; `null` and malformed values keep the
/// errors of the plain `Vec` members they replaced; clone and unit
/// normalization keep the presence; code-built values count a non-empty list.
#[test]
fn wind_list_presence_is_tracked_without_changing_the_wire_contract() {
    use crate::WindGenerationInput;
    /// The members as they were declared before presence tracking.
    #[derive(serde::Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    #[allow(dead_code)]
    struct Before {
        #[serde(default)]
        pressure: Option<Quantity>,
        #[serde(default)]
        shape_factor: Option<Quantity>,
        #[serde(default)]
        direction: Option<String>,
        #[serde(default)]
        exposed_pipe_refs: Vec<String>,
        #[serde(default)]
        exposed_spans: Vec<crate::WindExposedSpanInput>,
    }
    let span = json!({"pipe_ref": "P1", "start_fraction": {"value": 0.0, "unit": "1"}, "end_fraction": {"value": 0.5, "unit": "1"}});
    for (refs, spans) in [(None, None), (Some(json!([])), None), (None, Some(json!([]))), (Some(json!(["P1"])), Some(json!([span.clone()]))), (Some(json!([])), Some(json!([])))] {
        let mut value = json!({"pressure": {"value": 1.0, "unit": "Pa"}, "direction": "global_y"});
        if let Some(refs) = &refs { value["exposed_pipe_refs"] = refs.clone(); }
        if let Some(spans) = &spans { value["exposed_spans"] = spans.clone(); }
        let wind: WindGenerationInput = serde_json::from_value(value.clone()).unwrap();
        let before: Before = serde_json::from_value(value).unwrap();
        assert_eq!((wind.exposed_pipe_refs_authored, wind.exposed_spans_authored), (refs.is_some(), spans.is_some()));
        assert_eq!((wind.authors_exposed_pipe_refs(), wind.authors_exposed_spans()), (refs.is_some(), spans.is_some()));
        assert_eq!(wind.exposed_pipe_refs, before.exposed_pipe_refs);
        assert_eq!(wind.exposed_spans.len(), before.exposed_spans.len());
        let copy = wind.clone();
        assert_eq!((copy.exposed_pipe_refs_authored, copy.exposed_spans_authored), (refs.is_some(), spans.is_some()));
    }
    // null, malformed and unknown members: the same errors as before.
    for bad in [json!({"exposed_pipe_refs": null}), json!({"exposed_spans": null}), json!({"exposed_pipe_refs": "P1"}),
                json!({"exposed_spans": [{"pipe_ref": "P1", "extra": 1}]}), json!({"exposed_pipe_refs": [1]}), json!({"exposd_pipe_refs": []})] {
        let now = serde_json::from_value::<WindGenerationInput>(bad.clone()).unwrap_err().to_string();
        let then = serde_json::from_value::<Before>(bad.clone()).unwrap_err().to_string();
        assert_eq!(now, then, "{bad}");
    }
    // Built in code: a non-empty list counts as authored; an empty one with no flag does not.
    let built = WindGenerationInput { exposed_pipe_refs: vec!["P1".into()], ..Default::default() };
    assert!(built.authors_exposed_pipe_refs() && !built.authors_exposed_spans());
    assert!(!WindGenerationInput::default().authors_exposed_pipe_refs());
    // Through the model decode and unit normalization.
    let mut document = twin::document(crate::MODEL_SCHEMA_VERSION);
    document["load_cases"][2]["equivalent_static"]["wind"]["exposed_spans"] = json!([]);
    let mut model = decode(&document);
    let mut materials = model.materials.clone();
    let mut diagnostics = Vec::new();
    crate::normalize_model_units(&mut model, &mut materials, &mut diagnostics);
    let wind = model.load_cases[2].equivalent_static.as_ref().unwrap().wind.as_ref().unwrap();
    assert!(wind.exposed_spans_authored && !wind.exposed_pipe_refs_authored, "{diagnostics:?}");
}

/// An authored empty list conflicts with a wind-exposure use; the case is
/// refused by name and nothing is generated.
#[test]
fn an_authored_empty_exposure_list_conflicts_with_a_wind_exposure_use() {
    for member in ["exposed_pipe_refs", "exposed_spans"] {
        let mut document = twin::document(crate::MODEL_SCHEMA_VERSION);
        document["load_cases"][2]["equivalent_static"]["wind"][member] = json!([]);
        let (_, diagnostics) = lowered(&document);
        assert!(diagnostics.iter().any(|d| d.code == LOAD_CASE_CONDITION_WIND_EXPOSURE_CONFLICT && d.message.contains(&format!("equivalent_static.wind.{member}"))), "{member}: {diagnostics:?}");
    }
}

/// The preview takes the lowering's own judgement: with the plan resolved, its
/// records are exactly what `lower` appends to each case (after the authored
/// ones), in order, and its wind ids are the case's exposure; refused, it lists
/// no record and the lowering's diagnostics. The input value is not changed.
#[test]
fn the_preview_reports_exactly_what_the_lowering_applies() {
    let mut refused = twin::document(crate::MODEL_SCHEMA_VERSION);
    refused["pipe_segments"][1].as_object_mut().unwrap().remove("condition_set_ref");
    for (name, document) in variants().into_iter().chain([("refused", refused)]) {
        let before = document.clone();
        let preview = preview_operating_conditions(&document);
        assert_eq!(document, before, "{name}: the input is not changed");
        let decoded = decode(&document);
        let (model, diagnostics) = lowered(&document);
        assert_eq!(preview["diagnostics"], serde_json::to_value(&diagnostics).unwrap(), "{name}");
        let resolved = !crate::has_blocking(&diagnostics);
        assert_eq!(preview["resolved"], json!(resolved), "{name}");
        let cases = preview["cases"].as_array().unwrap();
        if !resolved {
            assert!(cases.is_empty(), "{name}");
            continue;
        }
        let using: Vec<usize> = decoded.load_cases.iter().enumerate().filter(|(_, c)| c.condition_use.is_some()).map(|(i, _)| i).collect();
        assert_eq!(cases.len(), using.len(), "{name}");
        for (case, &index) in cases.iter().zip(&using) {
            assert_eq!(case["case_id"], json!(decoded.load_cases[index].id));
            let authored = decoded.load_cases[index].primitive_loads.len();
            let appended: Vec<Value> = model.load_cases[index].primitive_loads[authored..].iter().map(|load| json!({
                "id": load.id, "category": load.category, "target": {"type": "element", "pipe": match &load.target { crate::LoadTargetInput::Element { pipe } => pipe.clone(), _ => unreachable!() }},
                "direction": load.direction, "magnitude": {"value": load.magnitude.value, "unit": load.magnitude.unit},
                "dimension": load.dimension, "provenance": load.provenance})).collect();
            assert_eq!(case["generated_primitive_loads"], json!(appended), "{name}");
            let wind = decoded.load_cases[index].condition_use.as_ref().unwrap().wind_exposure == Some(true);
            let exposed = model.load_cases[index].equivalent_static.as_ref().and_then(|e| e.wind.as_ref()).map(|w| w.exposed_pipe_refs.clone());
            assert_eq!(case["wind_exposed_pipe_refs"], if wind { json!(exposed.unwrap()) } else { Value::Null }, "{name}");
        }
    }
}

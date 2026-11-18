//! DEL-04-10 AC-001, for unit S2: each refusal of the table and its evaluation is named, with
//! its object and field, and no input is defaulted. The README's example request runs as stated.
use open_pipe_stress_response_spectrum::{
    common_damping_ratio, evaluate_request_json, QueryFrequency, RefusalCode, ScaleFactor, SpectrumRecord,
    SpectrumTable, STATEMENT,
};
use serde_json::{json, Value};

const README: &str = include_str!("../README.md");

fn g_record() -> Value {
    json!({"id": "SP", "abscissa": "period", "ordinate": "g_multiple", "units": {"abscissa": "s"},
        "g_value": {"value": 9.80665, "unit": "m/s^2"}, "damping_ratio": 0.05,
        "interpolation": {"abscissa": "linear", "ordinate": "log"},
        "points": [[0.03125, 0.35], [0.125, 1.1], [0.5, 0.8], [2.0, 0.25], [4.0, 0.1]]})
}

fn record(value: Value) -> SpectrumRecord {
    serde_json::from_value(value).expect("the record decodes")
}

fn refusals(value: Value) -> Vec<(RefusalCode, Option<String>)> {
    SpectrumTable::admit(&record(value)).expect_err("refused").into_iter().map(|r| {
        assert_eq!(r.object, "SP");
        (r.code, r.field)
    }).collect()
}

#[test]
fn each_record_rule_is_refused_by_name_with_its_field() {
    assert!(SpectrumTable::admit(&record(g_record())).is_ok());
    let invalid = |field: &str| vec![(RefusalCode::RecordInvalid, Some(field.to_string()))];
    type Mutation = Box<dyn Fn(&mut Value)>;
    type Expected = Vec<(RefusalCode, Option<String>)>;
    let cases: Vec<(Mutation, Expected)> = vec![
        (Box::new(|v| v["units"]["abscissa"] = "Hz".into()), invalid("units.abscissa")),
        (Box::new(|v| v["units"]["abscissa"] = "ms".into()), invalid("units.abscissa")),
        (Box::new(|v| v["units"]["ordinate"] = "m/s^2".into()), invalid("units.ordinate")),
        (Box::new(|v| { v.as_object_mut().unwrap().remove("g_value"); }), invalid("g_value")),
        (Box::new(|v| v["g_value"]["value"] = 0.0.into()), invalid("g_value")),
        (Box::new(|v| v["g_value"]["value"] = (-9.8).into()), invalid("g_value")),
        (Box::new(|v| v["g_value"]["unit"] = "Hz".into()), invalid("g_value")),
        (Box::new(|v| v["damping_ratio"] = 1.0.into()), invalid("damping_ratio")),
        (Box::new(|v| v["damping_ratio"] = (-0.01).into()), invalid("damping_ratio")),
        (Box::new(|v| v["points"] = json!([[1.0, 0.2]])), invalid("points")),
        (Box::new(|v| v["points"] = json!([[1.0, 0.2], [1.0, 0.3]])), invalid("points[1]")),
        (Box::new(|v| v["points"] = json!([[2.0, 0.2], [1.0, 0.3]])), invalid("points[1]")),
        (Box::new(|v| v["points"] = json!([[0.0, 0.2], [1.0, 0.3]])), invalid("points[0]")),
        (Box::new(|v| v["points"] = json!([[-1.0, 0.2], [1.0, 0.3]])), invalid("points[0]")),
        (Box::new(|v| v["points"] = json!([[1.0, 0.0], [2.0, 0.3]])), invalid("points[0]")),
        (Box::new(|v| v["points"] = json!([[1.0, 0.2], [2.0, -0.3]])), invalid("points[1]")),
        // A valid record with more damping than a spectrum solve admits.
        (Box::new(|v| v["damping_ratio"] = 0.21.into()), vec![(RefusalCode::DampingOutOfRange, Some("damping_ratio".into()))]),
    ];
    for (mutate, expected) in cases {
        let mut value = g_record();
        mutate(&mut value);
        assert_eq!(refusals(value.clone()), expected, "{value}");
    }
    // The bound is inclusive, and zero damping is admitted.
    for damping in [0.2, 0.0] {
        let mut value = g_record();
        value["damping_ratio"] = damping.into();
        assert!(SpectrumTable::admit(&record(value)).is_ok());
    }
    // An acceleration table in a unit the catalog lacks, and one with its unit missing.
    let mut acceleration = g_record();
    acceleration["ordinate"] = "acceleration".into();
    acceleration.as_object_mut().unwrap().remove("g_value");
    acceleration["units"]["ordinate"] = "mm/s^2".into();
    assert_eq!(refusals(acceleration.clone()), invalid("units.ordinate"));
    acceleration["units"].as_object_mut().unwrap().remove("ordinate");
    assert_eq!(refusals(acceleration), invalid("units.ordinate"));
    // Non-finite inputs, which JSON cannot carry, reach the typed record directly.
    for (field, mutate) in [
        ("damping_ratio", Box::new(|r: &mut SpectrumRecord| r.damping_ratio = f64::NAN) as Box<dyn Fn(&mut SpectrumRecord)>),
        ("points[2]", Box::new(|r| r.points[2][0] = f64::INFINITY)),
        ("points[2]", Box::new(|r| r.points[2][1] = f64::NAN)),
        ("g_value", Box::new(|r| r.g_value.as_mut().unwrap().value = f64::INFINITY)),
    ] {
        let mut typed = record(g_record());
        mutate(&mut typed);
        let found: Vec<_> = SpectrumTable::admit(&typed).unwrap_err().into_iter().map(|r| (r.code, r.field)).collect();
        assert_eq!(found, invalid(field), "{field}");
    }
    // Several faults are all reported, in field order.
    let mut many = g_record();
    many["id"] = " ".into();
    many["damping_ratio"] = 1.5.into();
    many["points"] = json!([[1.0, 0.2]]);
    let found: Vec<_> = SpectrumTable::admit(&record(many)).unwrap_err().into_iter().map(|r| (r.code.as_str(), r.object, r.field)).collect();
    assert_eq!(found, [
        ("SPECTRUM_RECORD_INVALID", "spectrum".to_string(), Some("id".to_string())),
        ("SPECTRUM_RECORD_INVALID", "spectrum".to_string(), Some("damping_ratio".to_string())),
        ("SPECTRUM_RECORD_INVALID", "spectrum".to_string(), Some("points".to_string())),
    ]);
}

#[test]
fn query_scale_and_nonfinite_refusals_are_named() {
    for (value, unit) in [(0.0, "Hz"), (-1.0, "Hz"), (f64::INFINITY, "Hz"), (f64::NAN, "Hz"), (1.0, "rad/s"), (1.0, "s")] {
        let refusal = QueryFrequency::new(value, unit).unwrap_err();
        assert_eq!((refusal.code, refusal.object.as_str(), refusal.field.as_deref()),
            (RefusalCode::QueryFrequencyInvalid, "query", Some("frequency")), "{value} {unit}");
    }
    for value in [0.0, -1.5, f64::INFINITY, f64::NAN] {
        assert_eq!(ScaleFactor::new(value).unwrap_err().code, RefusalCode::ScaleFactorInvalid, "{value}");
    }
    // Admitted ordinates whose scaled value leaves the binary64 range are refused, not published.
    let mut huge = g_record();
    huge["ordinate"] = "acceleration".into();
    huge.as_object_mut().unwrap().remove("g_value");
    huge["units"]["ordinate"] = "m/s^2".into();
    huge["points"] = json!([[1.0, 1.0e308], [2.0, 1.5e308]]);
    let table = SpectrumTable::admit(&record(huge)).unwrap();
    let f = QueryFrequency::new(0.75, "Hz").unwrap();
    assert!(table.evaluate(f, ScaleFactor::new(1.0).unwrap()).is_ok());
    let refusal = table.evaluate(f, ScaleFactor::new(2.0).unwrap()).unwrap_err();
    assert_eq!((refusal.code, refusal.object.as_str()), (RefusalCode::EvaluationAccuracyUnsupported, "SP"));
    // The same through g: a g multiple near the binary64 limit times g.
    let mut g_huge = g_record();
    g_huge["points"] = json!([[1.0, 1.0e308], [2.0, 1.5e308]]);
    let refusal = SpectrumTable::admit(&record(g_huge)).unwrap().evaluate(f, ScaleFactor::new(1.0).unwrap()).unwrap_err();
    assert_eq!(refusal.code, RefusalCode::EvaluationAccuracyUnsupported);
}

#[test]
fn damping_must_match_across_the_spectra_of_a_request() {
    let a = SpectrumTable::admit(&record(g_record())).unwrap();
    let mut other = g_record();
    other["id"] = "SP2".into();
    let b = SpectrumTable::admit(&record(other.clone())).unwrap();
    other["damping_ratio"] = 0.02.into();
    let c = SpectrumTable::admit(&record(other)).unwrap();
    assert_eq!(common_damping_ratio(&[]), Ok(None));
    assert_eq!(common_damping_ratio(&[&a, &b]), Ok(Some(0.05)));
    let refusal = common_damping_ratio(&[&a, &b, &c]).unwrap_err();
    assert_eq!((refusal.code.as_str(), refusal.object.as_str(), refusal.field.as_deref()),
        ("SPECTRUM_DAMPING_MISMATCH", "SP2", Some("damping_ratio")));
}

#[test]
fn nothing_is_defaulted_a_missing_unknown_or_null_member_does_not_decode() {
    let request = |spectrum: Value| json!({"spectrum": spectrum,
        "queries": [{"frequency": {"value": 2.0, "unit": "Hz"}, "scale_factor": 1.0}]}).to_string();
    assert!(evaluate_request_json(&request(g_record())).is_ok());
    for member in ["id", "abscissa", "ordinate", "units", "damping_ratio", "interpolation", "points"] {
        let mut value = g_record();
        value.as_object_mut().unwrap().remove(member);
        assert!(evaluate_request_json(&request(value)).is_err(), "missing {member}");
    }
    let mut value = g_record();
    value["interpolation"].as_object_mut().unwrap().remove("ordinate");
    assert!(evaluate_request_json(&request(value)).is_err(), "missing interpolation.ordinate");
    let mut value = g_record();
    value["interpolation"]["abscissa"] = "spline".into();
    assert!(evaluate_request_json(&request(value)).is_err(), "unknown rule");
    let mut value = g_record();
    value["extrapolate"] = true.into();
    assert!(evaluate_request_json(&request(value)).is_err(), "unknown member");
    let mut value = g_record();
    value["g_value"] = Value::Null;
    assert!(evaluate_request_json(&request(value)).is_err(), "null g_value");
    // A query without its scale factor or unit.
    for query in [json!({"frequency": {"value": 2.0, "unit": "Hz"}}), json!({"frequency": {"value": 2.0}, "scale_factor": 1.0})] {
        let text = json!({"spectrum": g_record(), "queries": [query]}).to_string();
        assert!(evaluate_request_json(&text).is_err(), "{query}");
    }
}

#[test]
fn metadata_matches_the_model_schema_without_rewriting_authored_text() {
    let request = |spectrum: Value| json!({"spectrum": spectrum,
        "queries": [{"frequency": {"value": 2.0, "unit": "Hz"}, "scale_factor": 1.0}]}).to_string();
    let baseline = evaluate_request_json(&request(g_record())).unwrap();
    let absent = record(g_record());
    assert_eq!(absent.tags, None);
    assert_eq!(absent.notes, None);

    let mut valid = g_record();
    let long_tag = "é".repeat(64);
    valid["tags"] = json!(["z tag", long_tag, "A"]);
    valid["notes"] = json!("  authored notes\n");
    let decoded = record(valid.clone());
    assert_eq!(decoded.tags.unwrap(), vec!["z tag".to_owned(), long_tag, "A".to_owned()]);
    assert_eq!(decoded.notes.as_deref(), Some("  authored notes\n"));
    assert_eq!(evaluate_request_json(&request(valid)).unwrap(), baseline);
    let mut empty_tags = g_record();
    empty_tags["tags"] = json!([]);
    assert_eq!(record(empty_tags).tags, Some(vec![]));

    for (member, value) in [
        ("notes", Value::Null),
        ("notes", json!(42)),
        ("notes", json!([])),
        ("notes", json!("")),
        ("notes", json!(" \t\n\u{2003}")),
        ("tags", Value::Null),
        ("tags", json!(42)),
        ("tags", json!("tag")),
        ("tags", json!([42])),
        ("tags", json!([null])),
        ("tags", json!([""])),
        ("tags", json!(["duplicate", "duplicate"])),
        ("tags", json!([" leading"])),
        ("tags", json!(["trailing\n"])),
        ("tags", json!(["\u{2003}tag"])),
        ("tags", json!(["é".repeat(65)])),
    ] {
        let mut spectrum = g_record();
        spectrum[member] = value.clone();
        assert!(evaluate_request_json(&request(spectrum)).is_err(), "{member}: {value}");
    }
}

#[test]
fn the_readme_example_runs_as_stated() {
    let start = README.find("```json\n").expect("the README has a JSON request") + "```json\n".len();
    let text = &README[start..start + README[start..].find("```").unwrap()];
    let document = evaluate_request_json(text).unwrap();
    assert_eq!(document["statement"], STATEMENT);
    assert_eq!(document["admitted"], true);
    let results = document["results"].as_array().unwrap();
    assert_eq!(results.len(), 4);
    assert_eq!(results[0]["location"], json!({"knot": 2}));
    assert_eq!(results[0]["pseudo_acceleration_m_per_s2"], 9.81);
    assert_eq!(results[1]["location"], json!({"segment": [1, 2]}));
    // spectrum_tables.json, T1 log-abscissa/log-ordinate at 3 Hz.
    let s = results[1]["pseudo_acceleration_m_per_s2"].as_f64().unwrap();
    assert!(((s - 5.858130291157739) / 5.858130291157739).abs() <= 1e-9, "{s}");
    assert_eq!(results[2]["refusal"]["code"], "SPECTRUM_FREQUENCY_OUTSIDE_RANGE");
    assert_eq!(results[3]["refusal"]["code"], "SPECTRUM_SCALE_FACTOR_INVALID");
    // A refused table returns its refusals and evaluates nothing.
    let mut refused: Value = serde_json::from_str(text).unwrap();
    refused["spectrum"]["damping_ratio"] = 0.3.into();
    let document = evaluate_request_json(&refused.to_string()).unwrap();
    assert_eq!(document["admitted"], false);
    assert_eq!(document["refusals"][0]["code"], "SPECTRUM_DAMPING_OUT_OF_RANGE");
    assert!(document.get("results").is_none());
}

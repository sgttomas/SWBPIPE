//! A pressure-free document's shear model selection, the Rust twin of
//! `test_a_pressure_free_shear_model_selection_travels_in_the_sidecar` and
//! `test_a_pressure_free_shear_model_the_solve_refuses_is_not_exported` in `tests/test_caepipe_mbf_round_trip.py`:
//! `model_settings.shear_model` is not in the MBF, so the sidecar's pressure_map carries the document's
//! timoshenko_annulus_energy_v1 under contract "none", and a bound import restores it. euler_bernoulli there is no
//! statement. A bound import keeps the sidecar's; a different shear_model argument with it is refused. An unbound
//! import of a file without pressure states none, whatever shear_model is given. A present null is refused like any
//! other value the solve refuses, and a sidecar older than format 6 states euler_bernoulli without a contract, as its
//! writer did. Invented data only.

mod common;

use open_pipe_stress_caepipe_mbf::{self as mbf, py_eq, ImportOptions, ImportResult, SidecarInput, Value, WriterResult, WriterSettings};

const EXPORT_MODEL: &str = include_str!("../../../../fixtures/caepipe_mbf/invented/desktop_export/export_model.json");
const INVENTED: &str = r#"{"*":{"origin":"project_invented","source_name":"invented pressure-free shear model test material"}}"#;
const TIMOSHENKO: &str = "timoshenko_annulus_energy_v1";

fn parse(text: &str) -> Value {
    mbf::parse_sidecar_json(text.as_bytes()).expect("invented JSON")
}

fn settings() -> WriterSettings {
    mbf::export_settings_from_json(br#"{"title":"Invented MBF export","referenceTemperature":{"value":20,"unit":"degC"},"maxLayoutRows":null}"#)
        .expect("valid export settings")
}

/// The pressure-free export fixture with `model_settings` set to `settings`, or without it.
fn model(settings: Option<&str>) -> Value {
    let mut model = parse(EXPORT_MODEL);
    assert!(model.get("pressure_contract").is_none(), "the fixture is pressure-free");
    let dict = model.as_dict_mut().expect("an object");
    dict.remove(&"model_settings".to_string());
    if let Some(settings) = settings {
        dict.insert("model_settings".into(), parse(settings));
    }
    model
}

fn render(model: &Value) -> WriterResult {
    mbf::render_si(model, &settings()).expect("a writer result")
}

fn written(model: &Value) -> (WriterResult, Vec<u8>, Value) {
    let result = render(model);
    assert_eq!(result.status, "written", "{:?}", result.diagnostics);
    let text = result.text.clone().expect("text").into_bytes();
    let sidecar = result.sidecar(&common::sha(&text)).expect("a sidecar");
    (result, text, sidecar)
}

fn import(data: &[u8], sidecar: Option<&Value>, shear_model: Option<&str>) -> ImportResult {
    let mut options = ImportOptions::new("invented.mbf");
    options.sidecar = sidecar.cloned().map(SidecarInput::Parsed);
    options.material_provenance = Some(parse(INVENTED));
    options.shear_model = shear_model.map(Value::str);
    mbf::import(data, &options).expect("an import result")
}

fn errors(result: &ImportResult) -> Vec<(String, String)> {
    result
        .diagnostics
        .iter()
        .filter(|item| item.get_or_null("severity").as_str() == Some("error"))
        .map(|item| {
            (item.get_or_null("code").as_str().unwrap_or_default().to_string(), item.get_or_null("location").as_str().unwrap_or_default().to_string())
        })
        .collect()
}

fn lost(result: &WriterResult) -> Vec<(String, String)> {
    result
        .loss_report
        .iter()
        .map(|item| (item.get_or_null("code").as_str().unwrap_or_default().to_string(), item.get_or_null("entity").as_str().unwrap_or_default().to_string()))
        .collect()
}

fn settings_of(result: &ImportResult) -> Option<Value> {
    result.model.as_ref().expect("a model").get("model_settings").cloned()
}

/// Equal as the Python twin compares them (`==`), absence included.
fn same(a: Option<&Value>, b: Option<&Value>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => py_eq(a, b),
        (None, None) => true,
        _ => false,
    }
}

/// `sidecar` as a deliberate editor leaves it: its self-digest, where it has one, recomputed.
fn redigested(mut sidecar: Value) -> Value {
    if sidecar.get("sidecar_sha256").is_some() {
        let digest = mbf::sidecar_self_digest(&sidecar).expect("a canonical sidecar");
        sidecar.as_dict_mut().expect("an object").insert("sidecar_sha256".into(), Value::Str(digest));
    }
    sidecar
}

/// The writer's format-6 `sidecar` as an earlier writer recorded it (`as_format` in the Python twin): format 4 without
/// `derivation` and each bend entry's `node`; format 3 also without `section_map`; format 2 also without the
/// self-digest. The self-digest of formats 3 and 4 is recomputed.
fn as_format(sidecar: &Value, number: i64) -> Value {
    let mut copy = sidecar.clone();
    let dict = copy.as_dict_mut().expect("an object");
    dict.remove(&"derivation".to_string());
    if let Some(elements) = dict.get_mut(&"element_map".to_string()).and_then(Value::as_dict_mut) {
        for entry in elements.values_mut() {
            if let Some(bend) = entry.as_dict_mut().and_then(|entry| entry.get_mut(&"bend".to_string())).and_then(Value::as_dict_mut) {
                bend.remove(&"node".to_string());
            }
        }
    }
    dict.insert("sidecar_format".into(), Value::int(number));
    if number < 4 {
        dict.remove(&"section_map".to_string());
    }
    if number == 2 {
        dict.remove(&"sidecar_sha256".to_string());
    }
    redigested(copy)
}

/// The fixture as a contract-only document: a pressure contract, an explicit zero allowance and no region in any case.
fn contract_only(mut model: Value, shear: &str) -> Value {
    let dict = model.as_dict_mut().expect("an object");
    dict.insert("pressure_contract".into(), parse(&format!(r#"{{"version":"3.0.0","mode":"exact_pressure_v3","shear_model":"{shear}"}}"#)));
    if let Some(Value::List(pipes)) = dict.get_mut(&"pipe_segments".to_string()) {
        for pipe in pipes {
            let section = pipe.as_dict_mut().and_then(|pipe| pipe.get_mut(&"section".to_string())).and_then(Value::as_dict_mut).expect("a section");
            section.insert("corrosion_allowance".into(), parse(r#"{"value":0.0,"unit":"m"}"#));
        }
    }
    if let Some(Value::List(cases)) = dict.get_mut(&"load_cases".to_string()) {
        for case in cases {
            case.as_dict_mut().expect("a case").insert("pressure_regions".into(), Value::List(Vec::new()));
        }
    }
    model
}

fn refusals(result: &WriterResult) -> Vec<(String, String)> {
    result
        .diagnostics
        .iter()
        .filter(|item| item.get_or_null("severity").as_str() == Some("error"))
        .map(|item| (item.get_or_null("code").as_str().unwrap_or_default().to_string(), item.get_or_null("message").as_str().unwrap_or_default().to_string()))
        .collect()
}

fn pressure_map(shear: &str) -> Value {
    parse(&format!(r#"{{"contract":"none","shear_model":"{shear}","regions":[]}}"#))
}

#[test]
fn a_pressure_free_timoshenko_selection_travels_in_the_sidecar_and_is_restored_bound() {
    let (writer, data, sidecar) = written(&model(Some(r#"{"shear_model":"timoshenko_annulus_energy_v1"}"#)));
    assert!(same(sidecar.get("pressure_map"), Some(&pressure_map(TIMOSHENKO))));
    let lost = lost(&writer);
    assert!(lost.contains(&("MBF-SI-SHEAR-MODEL-NOT-EXPORTED".into(), "model.model_settings.shear_model".into())), "{lost:?}");
    assert!(!lost.iter().any(|(code, _)| code == "MBF-SI-MODEL-SETTINGS-NOT-EXPORTED"), "{lost:?}");
    let bound = import(&data, Some(&sidecar), None);
    assert_eq!(bound.status, "imported", "{:?}", bound.diagnostics);
    assert!(same(settings_of(&bound).as_ref(), Some(&parse(r#"{"shear_model":"timoshenko_annulus_energy_v1"}"#))));
    assert!(bound.model.as_ref().unwrap().get("pressure_contract").is_none());
    // Written again from the restored document, the sidecar states the same selection.
    let (_, _, again) = written(bound.model.as_ref().unwrap());
    assert!(same(again.get("pressure_map"), sidecar.get("pressure_map")));
    // A bound import keeps the sidecar's: the same argument is admitted, a different one refused.
    assert!(same(settings_of(&import(&data, Some(&sidecar), Some(TIMOSHENKO))).as_ref(), settings_of(&bound).as_ref()));
    assert_eq!(errors(&import(&data, Some(&sidecar), Some("euler_bernoulli"))), vec![("MBF-IMPORT-SIDECAR-CONFLICT".to_string(), "shear_model".to_string())]);
}

/// The sidecar's one accepted loss, guarded (the Rust half of
/// `test_a_stated_shear_model_comes_back_bound_with_the_same_mechanics`): a document that states euler_bernoulli
/// explicitly comes back from a bound import stating none, which the solve reads as Euler-Bernoulli, and is never
/// given any authored default; one that states timoshenko_annulus_energy_v1 comes back stating it.
#[test]
fn euler_bernoulli_without_a_contract_is_no_statement() {
    for settings in [None, Some(r#"{"shear_model":"euler_bernoulli"}"#)] {
        let (_, data, sidecar) = written(&model(settings));
        assert!(same(sidecar.get("pressure_map"), Some(&pressure_map("euler_bernoulli"))), "{settings:?}");
        let bound = import(&data, Some(&sidecar), None);
        assert_eq!(bound.status, "imported", "{:?}", bound.diagnostics);
        assert!(settings_of(&bound).is_none(), "{settings:?}");
    }
}

#[test]
fn an_unbound_import_without_pressure_states_none() {
    let (_, data, _) = written(&model(Some(r#"{"shear_model":"timoshenko_annulus_energy_v1"}"#)));
    for shear in [None, Some("euler_bernoulli"), Some(TIMOSHENKO)] {
        assert!(settings_of(&import(&data, None, shear)).is_none(), "{shear:?}");
    }
    assert_eq!(errors(&import(&data, None, Some("timoshenko"))), vec![("MBF-IMPORT-SHEAR-MODEL-INVALID".to_string(), "shear_model".to_string())]);
}

#[test]
fn a_sidecar_region_without_a_contract_is_refused() {
    let (_, data, mut sidecar) = written(&model(Some(r#"{"shear_model":"timoshenko_annulus_energy_v1"}"#)));
    let state = parse(
        r#"{"contract":"none","shear_model":"timoshenko_annulus_energy_v1","regions":[{"id":"region:zz","members":["zz"],"terminals":[{"node_ref":"node:zz","closure_transfer":"transfers_to_wall","provenance":"invented"},{"node_ref":"node:zz","closure_transfer":"transfers_to_wall","provenance":"invented"}]}]}"#,
    );
    sidecar.as_dict_mut().unwrap().insert("pressure_map".into(), state);
    // The self-digest recomputed, so the import reaches the parser's rule rather than stopping at the digest.
    let refused = import(&data, Some(&redigested(sidecar)), None);
    assert_eq!(refused.status, "refused");
    let found = errors(&refused);
    assert!(found.iter().any(|(code, location)| code == "MBF-IMPORT-SIDECAR-INVALID" && location == "pressure_map"), "{found:?}");
    assert!(!found.iter().any(|(code, _)| code == "MBF-IMPORT-SIDECAR-DIGEST-MISMATCH"), "{found:?}");
}

#[test]
fn a_pressure_free_shear_model_the_solve_refuses_is_not_exported() {
    let refused = render(&model(Some(r#"{"shear_model":"cowper"}"#)));
    assert_eq!(refused.status, "refused");
    assert!(refused.diagnostics.iter().any(|item| item.get_or_null("code").as_str() == Some("MBF-SI-SHEAR-MODEL-UNSUPPORTED")));
    let both = written(&model(Some(r#"{"shear_model":"timoshenko_annulus_energy_v1","node_proximity_tolerance":{"value":5.0,"unit":"mm"}}"#))).0;
    let lost = lost(&both);
    for (code, entity) in [("MBF-SI-SHEAR-MODEL-NOT-EXPORTED", "model.model_settings.shear_model"), ("MBF-SI-MODEL-SETTINGS-NOT-EXPORTED", "model.model_settings")] {
        assert!(lost.contains(&(code.to_string(), entity.to_string())), "{lost:?}");
    }
}

/// A present null is a value the solve refuses (the model schema lists it), not an absent selection: the writer refuses
/// it by name, with or without a contract, rather than exporting a pair whose bound import would solve. The Python twin
/// is in `test_a_pressure_free_shear_model_the_solve_refuses_is_not_exported`; parity vectors cannot catch an error
/// both implementations share, so each has its own direct test.
#[test]
fn a_present_null_shear_model_is_refused_with_or_without_a_contract() {
    let free = refusals(&render(&model(Some(r#"{"shear_model":null}"#))));
    assert!(
        free.iter().any(|(code, message)| code == "MBF-SI-SHEAR-MODEL-UNSUPPORTED" && message.contains("model_settings.shear_model None is not one of")),
        "{free:?}"
    );
    let beside = refusals(&render(&contract_only(model(Some(r#"{"shear_model":null}"#)), "euler_bernoulli")));
    assert!(
        beside.iter().any(|(code, message)| code == "MBF-SI-SHEAR-MODEL-UNSUPPORTED" && message.contains("MODEL_SETTING_SHEAR_MODEL_WITH_PRESSURE_CONTRACT")),
        "{beside:?}"
    );
    // The same contract-only document without the setting is written: the refusal is the null's.
    assert_eq!(render(&contract_only(model(None), "euler_bernoulli")).status, "written");
}

/// Formats 2, 3 and 4 never recorded a pressure-free Timoshenko selection: without a contract their shear model is the
/// route's euler_bernoulli, and timoshenko_annulus_energy_v1 there is refused at pressure_map (format 2 has no
/// self-digest to protect it). Format 6 carries it. euler_bernoulli is admitted in every format.
#[test]
fn before_format_6_a_sidecar_without_a_contract_states_only_euler_bernoulli() {
    for (stated, refused) in [(TIMOSHENKO, true), ("euler_bernoulli", false)] {
        let (_, data, sidecar) = written(&model(Some(&format!(r#"{{"shear_model":"{stated}"}}"#))));
        for number in [2, 3, 4] {
            let found = errors(&import(&data, Some(&as_format(&sidecar, number)), None));
            let at_map = found.contains(&("MBF-IMPORT-SIDECAR-INVALID".to_string(), "pressure_map".to_string()));
            assert_eq!(at_map, refused, "{stated} in format {number}: {found:?}");
        }
        let current = import(&data, Some(&sidecar), None);
        assert_eq!(current.status, "imported", "{stated} in format 6: {:?}", current.diagnostics);
    }
}

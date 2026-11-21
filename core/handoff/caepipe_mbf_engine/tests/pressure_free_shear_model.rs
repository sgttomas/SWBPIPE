//! A pressure-free document's shear model selection, the Rust twin of
//! `test_a_pressure_free_shear_model_selection_travels_in_the_sidecar` and
//! `test_a_pressure_free_shear_model_the_solve_refuses_is_not_exported` in `tests/test_caepipe_mbf_round_trip.py`:
//! `model_settings.shear_model` is not in the MBF, so the sidecar's pressure_map carries the document's
//! timoshenko_annulus_energy_v1 under contract "none", and a bound import restores it. euler_bernoulli there is no
//! statement. A bound import keeps the sidecar's; a different shear_model argument with it is refused. An unbound
//! import of a file without pressure states none, whatever shear_model is given. Invented data only.

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
    let refused = import(&data, Some(&sidecar), None);
    assert_eq!(refused.status, "refused");
    assert!(errors(&refused).iter().any(|(code, location)| code == "MBF-IMPORT-SIDECAR-INVALID" && location == "pressure_map"), "{:?}", errors(&refused));
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

//! Sidecar format 6 (DEL-17-04 Design `caepipe-mbf-import.md` §15, Format 6), the Rust twin of the format-6 tests in
//! `tests/test_caepipe_mbf_si_writer.py`, `tests/test_caepipe_mbf_importer.py` and `tests/test_caepipe_mbf_round_trip.py`:
//! the writer records the digest of the canonical model document it read, every export setting that can change the
//! bytes, and the tangent node each bend component names; the strict parser refuses a malformed format-6 sidecar by
//! JSON path; a bound import restores the bend node only where it is one of that bend's tangent nodes; the
//! regeneration identity compares the bend node and sets the derivation aside; format 5 stays refused; format 4
//! imports as before; and the writer's loss report names every identity-set entity it does not carry, or the export
//! is refused. Invented data only.

mod common;

use open_pipe_stress_caepipe_mbf::{
    self as mbf, Dict, ImportOptions, ImportResult, Reference, Regenerated, SidecarInput, SidecarRegenerator, Value, WriterResult,
    WriterSettings,
};

const EXPORT_MODEL: &str = include_str!("../../../../fixtures/caepipe_mbf/invented/desktop_export/export_model.json");
/// test_caepipe_mbf_si_writer.pressurized_l() with its one region's pressure set to zero (invented).
const PRESSURE_L_ZERO: &str = r##"{"schema_version":"0.9.0","document_kind":"openpipestress.product_preview.model","analysis_status":{"mechanics":"invented","rule_check":"not_performed","professional_acceptance":"not_provided"},"project":{"id":"invented:mbf-si","units":{"length":"m","force":"N","temperature":"degC"}},"nodes":[{"id":"A","position":{"x":0.0,"y":0.0,"z":0.0}},{"id":"T1","position":{"x":3.75,"y":0.0,"z":0.0}},{"id":"T2","position":{"x":4.0,"y":0.0,"z":0.25}},{"id":"B","position":{"x":4.0,"y":0.0,"z":3.0}}],"pipe_segments":[{"id":"S1","from":"A","to":"T1","material":"mat:CS","section":{"outside_diameter":{"value":0.1683,"unit":"m"},"wall_thickness":{"value":0.00711,"unit":"m"},"corrosion_allowance":{"value":0.0,"unit":"m"}},"y_reference":{"x":0.0,"y":1.0,"z":0.0}},{"id":"BEND","from":"T1","to":"T2","material":"mat:CS","section":{"outside_diameter":{"value":0.1683,"unit":"m"},"wall_thickness":{"value":0.00711,"unit":"m"},"corrosion_allowance":{"value":0.0,"unit":"m"}},"y_reference":{"x":1.0,"y":0.0,"z":-1.0}},{"id":"S2","from":"T2","to":"B","material":"mat:CS","section":{"outside_diameter":{"value":0.1683,"unit":"m"},"wall_thickness":{"value":0.00711,"unit":"m"},"corrosion_allowance":{"value":0.0,"unit":"m"}},"y_reference":{"x":0.0,"y":1.0,"z":0.0}}],"materials":[{"id":"mat:CS","elastic_modulus":{"value":200000000000.0,"unit":"Pa"},"thermal_expansion_coefficient":{"value":1.2e-05,"unit":"1/K"},"constitutive_basis":"homogeneous_isotropic_E_nu_v1","poisson_ratio":{"value":0.25,"unit":"1"}}],"supports":[{"id":"SA","node":"A","restraints":["UX","UY","UZ","RX","RY","RZ"]},{"id":"SB","node":"B","restraints":["UX","UY","UZ","RX","RY","RZ"]}],"load_cases":[{"id":"case:p","primitive_loads":[],"pressure_regions":[{"id":"region:L","member_pipe_ids":["S1","BEND","S2"],"pressure_basis":"internal_differential_zero_external_v1","pressure":{"value":0.0,"unit":"Pa"},"provenance":"invented pressure for this test","terminals":[{"node_ref":"A","closure_transfer":"transfers_to_wall","provenance":"invented closure for this test"},{"node_ref":"B","closure_transfer":"transfers_to_wall","provenance":"invented closure for this test"}]}]},{"id":"case:th","primitive_loads":[{"id":"t:S1","category":"thermal","target":{"type":"element","pipe":"S1"},"direction":"global_x","magnitude":{"value":100.0,"unit":"degC"},"dimension":"temperature_interval"},{"id":"t:BEND","category":"thermal","target":{"type":"element","pipe":"BEND"},"direction":"global_x","magnitude":{"value":100.0,"unit":"degC"},"dimension":"temperature_interval"},{"id":"t:S2","category":"thermal","target":{"type":"element","pipe":"S2"},"direction":"global_x","magnitude":{"value":100.0,"unit":"degC"},"dimension":"temperature_interval"}],"pressure_regions":[]}],"combinations":[],"components":[{"id":"bend-1","kind":"bend","node":"T1","geometry":{"bend_pipe_ref":"BEND","bend_radius":{"value":250.0,"unit":"mm"},"bend_angle":{"value":1.5707963267948966,"unit":"rad"}},"mechanics_interface":{"solver_consumption":"curved_bend_macro_element"},"modifiers":{"flexibility_factor_user_value":{"value":1.0,"unit":"none"}}}],"pressure_contract":{"version":"3.0.0","mode":"exact_pressure_v3","shear_model":"euler_bernoulli"}}"##;
/// test_caepipe_mbf_round_trip.two_bend_u(): two 90 degree bends in one run, bend-1 at T1-T2 and bend-2 at T3-T4 (invented).
const TWO_BEND_U: &str = r##"{"schema_version":"0.9.0","document_kind":"openpipestress.product_preview.model","analysis_status":{"mechanics":"invented","rule_check":"not_performed","professional_acceptance":"not_provided"},"project":{"id":"invented:mbf-si","units":{"length":"m","force":"N","temperature":"degC"}},"nodes":[{"id":"A","position":{"x":0.0,"y":0.0,"z":0.0}},{"id":"T1","position":{"x":3.75,"y":0.0,"z":0.0}},{"id":"T2","position":{"x":4.0,"y":0.0,"z":0.25}},{"id":"T3","position":{"x":4.0,"y":0.0,"z":2.75}},{"id":"T4","position":{"x":3.75,"y":0.0,"z":3.0}},{"id":"B","position":{"x":0.0,"y":0.0,"z":3.0}}],"pipe_segments":[{"id":"S1","from":"A","to":"T1","material":"mat:CS","section":{"outside_diameter":{"value":0.1683,"unit":"m"},"wall_thickness":{"value":0.00711,"unit":"m"}},"y_reference":{"x":0.0,"y":1.0,"z":0.0}},{"id":"BEND","from":"T1","to":"T2","material":"mat:CS","section":{"outside_diameter":{"value":0.1683,"unit":"m"},"wall_thickness":{"value":0.00711,"unit":"m"}},"y_reference":{"x":1.0,"y":0.0,"z":-1.0}},{"id":"S2","from":"T2","to":"T3","material":"mat:CS","section":{"outside_diameter":{"value":0.1683,"unit":"m"},"wall_thickness":{"value":0.00711,"unit":"m"}},"y_reference":{"x":0.0,"y":1.0,"z":0.0}},{"id":"BEND2","from":"T3","to":"T4","material":"mat:CS","section":{"outside_diameter":{"value":0.1683,"unit":"m"},"wall_thickness":{"value":0.00711,"unit":"m"}},"y_reference":{"x":1.0,"y":0.0,"z":1.0}},{"id":"S3","from":"T4","to":"B","material":"mat:CS","section":{"outside_diameter":{"value":0.1683,"unit":"m"},"wall_thickness":{"value":0.00711,"unit":"m"}},"y_reference":{"x":0.0,"y":1.0,"z":0.0}}],"materials":[{"id":"mat:CS","elastic_modulus":{"value":200000000000.0,"unit":"Pa"},"shear_modulus":{"value":80000000000.0,"unit":"Pa"},"thermal_expansion_coefficient":{"value":1.2e-05,"unit":"1/K"}}],"supports":[{"id":"SA","node":"A","restraints":["UX","UY","UZ","RX","RY","RZ"]},{"id":"SB","node":"B","restraints":["UX","UY","UZ","RX","RY","RZ"]}],"load_cases":[{"id":"case:th","primitive_loads":[{"id":"t:S1","category":"thermal","target":{"type":"element","pipe":"S1"},"direction":"global_x","magnitude":{"value":100.0,"unit":"degC"},"dimension":"temperature_interval"},{"id":"t:BEND","category":"thermal","target":{"type":"element","pipe":"BEND"},"direction":"global_x","magnitude":{"value":100.0,"unit":"degC"},"dimension":"temperature_interval"},{"id":"t:S2","category":"thermal","target":{"type":"element","pipe":"S2"},"direction":"global_x","magnitude":{"value":100.0,"unit":"degC"},"dimension":"temperature_interval"},{"id":"t:BEND2","category":"thermal","target":{"type":"element","pipe":"BEND2"},"direction":"global_x","magnitude":{"value":100.0,"unit":"degC"},"dimension":"temperature_interval"},{"id":"t:S3","category":"thermal","target":{"type":"element","pipe":"S3"},"direction":"global_x","magnitude":{"value":100.0,"unit":"degC"},"dimension":"temperature_interval"}]}],"combinations":[],"components":[{"id":"bend-1","kind":"bend","node":"T1","geometry":{"bend_pipe_ref":"BEND","bend_radius":{"value":250.0,"unit":"mm"},"bend_angle":{"value":1.5707963267948966,"unit":"rad"}},"mechanics_interface":{"solver_consumption":"curved_bend_macro_element"},"modifiers":{"flexibility_factor_user_value":{"value":1.0,"unit":"none"}}},{"id":"bend-2","kind":"bend","node":"T3","geometry":{"bend_pipe_ref":"BEND2","bend_radius":{"value":250.0,"unit":"mm"}},"modifiers":{"flexibility_factor_user_value":{"value":1.0,"unit":"none"}},"mechanics_interface":{"solver_consumption":"curved_bend_macro_element"}}]}"##;
const INVENTED: &str = r#"{"*":{"origin":"project_invented","source_name":"invented format 6 test material"}}"#;

fn parse(text: &str) -> Value {
    mbf::parse_sidecar_json(text.as_bytes()).expect("invented JSON")
}

fn settings(json: &str) -> WriterSettings {
    mbf::export_settings_from_json(json.as_bytes()).expect("valid export settings")
}

fn celsius() -> WriterSettings {
    settings(r#"{"title":"Invented MBF export","referenceTemperature":{"value":20,"unit":"degC"},"maxLayoutRows":null}"#)
}

fn written(model: &Value, settings: &WriterSettings) -> (WriterResult, Vec<u8>, Value) {
    let result = mbf::render_si(model, settings).expect("a writer result");
    assert_eq!(result.status, "written", "{:?}", result.diagnostics);
    let text = result.text.clone().expect("text").into_bytes();
    let sidecar = result.sidecar(&common::sha(&text)).expect("a sidecar");
    (result, text, sidecar)
}

fn member<'v>(value: &'v mut Value, path: &[&str]) -> &'v mut Value {
    let mut here = value;
    for step in path {
        here = here.as_dict_mut().expect("an object").get_mut(&step.to_string()).expect("a member");
    }
    here
}

fn set(value: &mut Value, path: &[&str], key: &str, item: Value) {
    member(value, path).as_dict_mut().expect("an object").insert(key.to_string(), item);
}

fn remove(value: &mut Value, path: &[&str], key: &str) {
    member(value, path).as_dict_mut().expect("an object").remove(&key.to_string());
}

fn push(value: &mut Value, path: &[&str], item: Value) {
    let Value::List(items) = member(value, path) else { panic!("a list at {path:?}") };
    items.push(item);
}

/// `sidecar` as a deliberate editor leaves it: its self-digest recomputed.
fn redigested(mut sidecar: Value) -> Value {
    if sidecar.get("sidecar_sha256").is_some() {
        let digest = mbf::sidecar_self_digest(&sidecar).expect("a canonical sidecar");
        set(&mut sidecar, &[], "sidecar_sha256", Value::Str(digest));
    }
    sidecar
}

/// The writer's format-6 `sidecar` as format 4 recorded it: without `derivation` and each bend entry's `node`.
fn as_format_4(sidecar: &Value) -> Value {
    let mut copy = sidecar.clone();
    remove(&mut copy, &[], "derivation");
    for entry in member(&mut copy, &["element_map"]).as_dict_mut().expect("element_map").values_mut() {
        if let Some(bend) = entry.as_dict_mut().and_then(|entry| entry.get_mut(&"bend".to_string())).and_then(Value::as_dict_mut) {
            bend.remove(&"node".to_string());
        }
    }
    set(&mut copy, &[], "sidecar_format", Value::int(4));
    redigested(copy)
}

fn import_with(data: &[u8], sidecar: Option<Value>, rebind: bool, declaration: bool) -> ImportResult {
    let mut options = ImportOptions::new("invented.mbf");
    options.sidecar = sidecar.map(SidecarInput::Parsed);
    options.rebind_by_node_number = rebind;
    options.material_provenance = declaration.then(|| parse(INVENTED));
    mbf::import(data, &options).expect("an import result")
}

fn errors(result: &ImportResult) -> Vec<(String, String)> {
    let mut found: Vec<(String, String)> = result
        .diagnostics
        .iter()
        .filter(|item| item.get_or_null("severity").as_str() == Some("error"))
        .map(|item| {
            (item.get_or_null("code").as_str().unwrap_or_default().to_string(), item.get_or_null("location").as_str().unwrap_or_default().to_string())
        })
        .collect();
    found.sort();
    found.dedup();
    found
}

fn only(code: &str, location: &str) -> Vec<(String, String)> {
    vec![(code.to_string(), location.to_string())]
}

fn has_notice(result: &ImportResult, code: &str, location: &str) -> bool {
    result.loss_report.iter().any(|entry| entry.get_or_null("code").as_str() == Some(code) && entry.get_or_null("location").as_str() == Some(location))
}

fn component_node(result: &ImportResult, id: &str) -> Option<String> {
    let model = result.model.as_ref()?;
    let components = model.get_or_null("components").as_list()?;
    let component = components.iter().find(|item| item.get_or_null("id").as_str() == Some(id))?;
    component.get_or_null("node").as_str().map(String::from)
}

#[test]
fn format_6_records_the_source_model_digest_and_every_export_setting() {
    assert_eq!(mbf::SIDECAR_FORMAT, 6);
    assert_eq!(mbf::SIDECAR_FORMATS, [2, 3, 4, 6]);
    let model = parse(EXPORT_MODEL);
    let before = model.clone();
    let (_, text, sidecar) = written(&model, &celsius());
    assert!(mbf::py_eq(&model, &before), "reading the model does not change it");
    assert_eq!(mbf::value::small_int(sidecar.get_or_null("sidecar_format")), Some(6));
    let digest = mbf::model_canonical_sha256(&model).expect("a canonical model");
    let derivation = sidecar.get_or_null("derivation");
    assert_eq!(derivation.as_dict().map(|dict| dict.len()), Some(2));
    assert_eq!(derivation.get_or_null("source_model_sha256").as_str(), Some(digest.as_str()));
    let recorded = derivation.get_or_null("export_settings");
    assert_eq!(recorded.as_dict().map(|dict| dict.len()), Some(4));
    assert_eq!(recorded.get_or_null("title").as_str(), Some("Invented MBF export"));
    let reference = recorded.get_or_null("reference_temperature");
    assert_eq!((reference.get_or_null("value").as_f64(), reference.get_or_null("unit").as_str()), (Some(20.0), Some("degC")));
    assert!(recorded.get_or_null("max_layout_rows").is_null() && recorded.get_or_null("piping_code").is_null());
    // Key order is not content; a change of the document that the MBF does not carry still changes the digest.
    let mut reordered = Dict::new();
    for (key, value) in model.as_dict().expect("an object").iter().collect::<Vec<_>>().into_iter().rev() {
        reordered.insert(key.clone(), value.clone());
    }
    assert_eq!(mbf::model_canonical_sha256(&Value::Dict(reordered)).ok(), Some(digest.clone()));
    let mut renamed = model.clone();
    set(&mut renamed, &["project"], "name", Value::str("another invented name"));
    let (_, renamed_text, renamed_sidecar) = written(&renamed, &celsius());
    assert_eq!(renamed_text, text);
    let other = renamed_sidecar.get_or_null("derivation").get_or_null("source_model_sha256").as_str().map(String::from);
    assert_eq!(other, mbf::model_canonical_sha256(&renamed).ok());
    assert_ne!(other.as_deref(), Some(digest.as_str()));
    // Fahrenheit: the argument and its unit, not the Celsius value OPTEMP prints.
    let fahrenheit = settings(r#"{"title":"Invented MBF export","referenceTemperature":{"value":68,"unit":"degF"},"maxLayoutRows":null}"#);
    let (result, _, sidecar) = written(&model, &fahrenheit);
    assert!(result.text.as_deref().unwrap_or_default().contains("\r\nOPTEMP\r\n20\r\n"));
    let reference = sidecar.get_or_null("derivation").get_or_null("export_settings").get_or_null("reference_temperature").clone();
    assert_eq!((reference.get_or_null("value").as_f64(), reference.get_or_null("unit").as_str()), (Some(68.0), Some("degF")));
    // The row limit changes no byte and is recorded as given.
    let mut limited = celsius();
    limited.max_layout_rows = Some(7);
    let (_, limited_text, sidecar) = written(&model, &limited);
    assert_eq!(limited_text, text);
    let limit = sidecar.get_or_null("derivation").get_or_null("export_settings").get_or_null("max_layout_rows").clone();
    assert_eq!(mbf::value::small_int(&limit), Some(7));
    // A piping code changes the bytes and is recorded.
    let mut coded = celsius();
    coded.piping_code = Value::str("ASME");
    let (_, coded_text, sidecar) = written(&model, &coded);
    assert_ne!(coded_text, text);
    assert_eq!(sidecar.get_or_null("derivation").get_or_null("export_settings").get_or_null("piping_code").as_str(), Some("ASME"));
    // The printed title line is recorded, as the MBF prints it.
    let mut titled = celsius();
    titled.title = Value::str("  a,b  ");
    let (result, _, sidecar) = written(&model, &titled);
    assert!(result.text.as_deref().unwrap_or_default().starts_with("ab\r\n"));
    assert_eq!(sidecar.get_or_null("derivation").get_or_null("export_settings").get_or_null("title").as_str(), Some("ab"));
}

#[test]
fn a_row_limit_format_6_cannot_record_is_refused() {
    for limit in [-1, (1i64 << 53) + 1] {
        let mut limited = celsius();
        limited.max_layout_rows = Some(limit);
        let result = mbf::render_si(&parse(EXPORT_MODEL), &limited).expect("a writer result");
        assert_eq!(result.status, "refused", "{limit}");
        assert!(result.diagnostics.iter().any(|item| item.get_or_null("code").as_str() == Some("MBF-SI-INPUT-INVALID")
            && item.get_or_null("entity").as_str() == Some("max_layout_rows")), "{limit}");
    }
}

#[test]
fn a_model_document_without_a_canonical_form_is_refused_by_path() {
    let mut model = parse(EXPORT_MODEL);
    set(&mut model, &["project"], "revision", Value::int((1i64 << 53) + 1));
    let result = mbf::render_si(&model, &celsius()).expect("a writer result");
    assert_eq!(result.status, "refused");
    let refusal = result.diagnostics.iter().find(|item| item.get_or_null("severity").as_str() == Some("error")).expect("a refusal");
    assert_eq!(refusal.get_or_null("code").as_str(), Some("MBF-SI-INPUT-INVALID"));
    assert_eq!(refusal.get_or_null("entity").as_str(), Some("model"));
    let message = refusal.get_or_null("message").as_str().unwrap_or_default();
    assert!(message.contains("project.revision is an integer that is not exactly a binary64 number"), "{message}");
    assert!(message.contains("derivation.source_model_sha256"), "{message}");
}

struct Fixed(Value);

impl SidecarRegenerator for Fixed {
    fn regenerate(&self, _model: &Value, _mbf_sha256: &str, _derived: &[String], _reference: Reference) -> Result<Regenerated, mbf::ImportError> {
        Ok(Regenerated::Sidecar(self.0.clone()))
    }
}

#[test]
fn a_bend_node_outside_that_bends_tangent_nodes_is_refused_and_the_identity_compares_it() {
    let model = parse(TWO_BEND_U);
    let (_, data, sidecar) = written(&model, &WriterSettings::new("GUARD"));
    let base = import_with(&data, Some(sidecar.clone()), false, true);
    assert_eq!(base.status, "imported", "{:?}", base.diagnostics);
    let node_of = |pipe: &str| sidecar.get_or_null("element_map").get_or_null(pipe).get_or_null("bend").get_or_null("node").as_str().map(String::from);
    assert_eq!(node_of("BEND").as_deref(), Some("T1"));
    assert_eq!(node_of("BEND2").as_deref(), Some("T3"));
    assert_eq!(component_node(&base, "bend-1").as_deref(), Some("T1"));
    assert_eq!(component_node(&base, "bend-2").as_deref(), Some("T3"));
    // Another node of the model, a node absent from node_map, and the other bend's tangent nodes.
    for node in ["A", "node:absent", "T3", "T4"] {
        let mut changed = sidecar.clone();
        set(&mut changed, &["element_map", "BEND", "bend"], "node", Value::str(node));
        let result = import_with(&data, Some(redigested(changed)), false, true);
        assert_eq!(errors(&result), only("MBF-IMPORT-SIDECAR-CONFLICT", "element_map[BEND].bend.node"), "{node}");
        assert!(result.derivation.is_none() && result.model.is_none(), "{node}");
    }
    // The bend's own other tangent node is a consistent edit: imported as stated.
    let mut swapped = sidecar.clone();
    set(&mut swapped, &["element_map", "BEND", "bend"], "node", Value::str("T2"));
    let swapped = redigested(swapped);
    let result = import_with(&data, Some(swapped.clone()), false, true);
    assert_eq!(result.status, "imported", "{:?}", result.diagnostics);
    assert_eq!(component_node(&result, "bend-1").as_deref(), Some("T2"));
    // The regeneration identity alone, given the unedited import's regeneration, refuses the swap at bend.node: the
    // node is compared. Its derivation (title "REGENERATED") differs and is set aside.
    let derived: Vec<String> =
        base.id_map.get_or_null("y_reference_derived").as_list().expect("list").iter().filter_map(|item| item.as_str().map(String::from)).collect();
    let (regenerated, refusals) =
        mbf::regenerate_caepipe_sidecar(base.model.as_ref().expect("model"), &common::sha(&data), &derived, Reference::Celsius(20.0)).expect("regenerated");
    assert!(refusals.is_empty());
    let regenerated = regenerated.expect("a regenerated sidecar");
    assert_eq!(regenerated.get_or_null("derivation").get_or_null("export_settings").get_or_null("title").as_str(), Some("REGENERATED"));
    let fixed = Fixed(regenerated);
    for (given, expected) in [(sidecar.clone(), vec![]), (swapped, only("MBF-IMPORT-SIDECAR-NOT-REPRODUCED", "element_map[BEND].bend.node"))] {
        let mut options = ImportOptions::new("invented.mbf");
        options.sidecar = Some(SidecarInput::Parsed(given));
        options.material_provenance = Some(parse(INVENTED));
        options.regenerator = Some(&fixed as &dyn SidecarRegenerator);
        let result = mbf::import(&data, &options).expect("an import result");
        assert_eq!(errors(&result), expected);
    }
}

#[test]
fn a_malformed_format_6_sidecar_is_refused_by_path() {
    let (_, data, sidecar) = written(&parse(EXPORT_MODEL), &celsius());
    const SETTINGS: &[&str] = &["derivation", "export_settings"];
    const REFERENCE: &[&str] = &["derivation", "export_settings", "reference_temperature"];
    const BEND: &[&str] = &["element_map", "pipe:BEND", "bend"];
    let cases: Vec<(&str, Box<dyn Fn(&mut Value)>)> = vec![
        ("derivation", Box::new(|s: &mut Value| remove(s, &[], "derivation"))),
        ("derivation", Box::new(|s: &mut Value| set(s, &[], "derivation", Value::Null))),
        ("derivation.zz", Box::new(|s: &mut Value| set(s, &["derivation"], "zz", Value::int(1)))),
        ("derivation.source_model_sha256", Box::new(|s: &mut Value| set(s, &["derivation"], "source_model_sha256", Value::Null))),
        ("derivation.source_model_sha256", Box::new(|s: &mut Value| set(s, &["derivation"], "source_model_sha256", Value::str("A".repeat(64))))),
        ("derivation.source_model_sha256", Box::new(|s: &mut Value| set(s, &["derivation"], "source_model_sha256", Value::str("a".repeat(63))))),
        ("derivation.export_settings", Box::new(|s: &mut Value| remove(s, &["derivation"], "export_settings"))),
        ("derivation.export_settings.zz", Box::new(|s: &mut Value| set(s, SETTINGS, "zz", Value::Null))),
        ("derivation.export_settings.title", Box::new(|s: &mut Value| set(s, SETTINGS, "title", Value::Null))),
        ("derivation.export_settings.title", Box::new(|s: &mut Value| set(s, SETTINGS, "title", Value::str(" padded")))),
        ("derivation.export_settings.title", Box::new(|s: &mut Value| set(s, SETTINGS, "title", Value::str("A,B")))),
        ("derivation.export_settings.reference_temperature", Box::new(|s: &mut Value| set(s, SETTINGS, "reference_temperature", Value::Float(20.0)))),
        ("derivation.export_settings.reference_temperature.unit", Box::new(|s: &mut Value| set(s, REFERENCE, "unit", Value::str("K")))),
        ("derivation.export_settings.reference_temperature.unit", Box::new(|s: &mut Value| remove(s, REFERENCE, "unit"))),
        ("derivation.export_settings.reference_temperature.value", Box::new(|s: &mut Value| set(s, REFERENCE, "value", Value::str("20")))),
        ("derivation.export_settings.reference_temperature.value", Box::new(|s: &mut Value| set(s, REFERENCE, "value", Value::Null))),
        ("derivation.export_settings.max_layout_rows", Box::new(|s: &mut Value| remove(s, SETTINGS, "max_layout_rows"))),
        ("derivation.export_settings.max_layout_rows", Box::new(|s: &mut Value| set(s, SETTINGS, "max_layout_rows", Value::int(-1)))),
        ("derivation.export_settings.max_layout_rows", Box::new(|s: &mut Value| set(s, SETTINGS, "max_layout_rows", Value::Float(1.5)))),
        ("derivation.export_settings.max_layout_rows", Box::new(|s: &mut Value| set(s, SETTINGS, "max_layout_rows", Value::Bool(true)))),
        ("derivation.export_settings.piping_code", Box::new(|s: &mut Value| remove(s, SETTINGS, "piping_code"))),
        ("derivation.export_settings.piping_code", Box::new(|s: &mut Value| set(s, SETTINGS, "piping_code", Value::str("B31.3")))),
        ("derivation.export_settings.piping_code", Box::new(|s: &mut Value| set(s, SETTINGS, "piping_code", Value::int(7)))),
        ("element_map[pipe:BEND].bend.node", Box::new(|s: &mut Value| remove(s, BEND, "node"))),
        ("element_map[pipe:BEND].bend.node", Box::new(|s: &mut Value| set(s, BEND, "node", Value::Null))),
        ("element_map[pipe:BEND].bend.node", Box::new(|s: &mut Value| set(s, BEND, "node", Value::int(7)))),
        ("element_map[pipe:BEND].bend.node", Box::new(|s: &mut Value| set(s, BEND, "node", Value::str(" ")))),
        ("element_map[pipe:BEND].bend.zz", Box::new(|s: &mut Value| set(s, BEND, "zz", Value::str("node:T1")))),
    ];
    for (location, change) in &cases {
        for rebind in [false, true] {
            let mut changed = sidecar.clone();
            change(&mut changed);
            let result = import_with(&data, Some(redigested(changed)), rebind, rebind);
            assert_eq!(errors(&result), only("MBF-IMPORT-SIDECAR-INVALID", location), "{location} rebind={rebind}");
            assert!(result.derivation.is_none());
        }
    }
}

#[test]
fn format_5_the_held_route_1_format_stays_refused() {
    let (_, data, sidecar) = written(&parse(EXPORT_MODEL), &celsius());
    for drop_derivation in [false, true] {
        let mut changed = sidecar.clone();
        set(&mut changed, &[], "sidecar_format", Value::int(5));
        if drop_derivation {
            remove(&mut changed, &[], "derivation");
        }
        let result = import_with(&data, Some(changed), false, false);
        assert_eq!(result.status, "refused");
        assert!(result.derivation.is_none());
        let refusal = result.diagnostics.iter().find(|item| item.get_or_null("location").as_str() == Some("sidecar_format")).expect("a refusal");
        assert_eq!(refusal.get_or_null("code").as_str(), Some("MBF-IMPORT-SIDECAR-INVALID"));
        let message = refusal.get_or_null("message").as_str().unwrap_or_default();
        assert!(message.contains("must be 2 or 3 or 4 or 6, a format this importer reads"), "{message}");
    }
}

#[test]
fn format_4_imports_as_before_and_only_a_bound_format_6_import_exposes_its_derivation() {
    let model = parse(EXPORT_MODEL);
    let (_, data, sidecar) = written(&model, &celsius());
    let six = import_with(&data, Some(sidecar.clone()), false, false);
    assert_eq!(six.status, "imported", "{:?}", six.diagnostics);
    assert!(!has_notice(&six, "MBF-IMPORT-REFERENCE-UNRECOVERABLE", "component:bend-1.node"));
    assert!(has_notice(&six, "MBF-IMPORT-SOURCE-COMPLETENESS-UNESTABLISHED", "unexported members"));
    assert_eq!((six.identity_recovery.known_missing_ids, six.identity_recovery.unrecovered_fields), (0, 0));
    let expected = mbf::SidecarDerivation {
        source_model_sha256: mbf::model_canonical_sha256(&model).expect("a canonical model"),
        export_settings: mbf::SidecarExportSettings {
            title: "Invented MBF export".into(),
            reference_temperature: 20.0,
            reference_temperature_unit: "degC",
            max_layout_rows: None,
            piping_code: None,
        },
    };
    assert_eq!(six.derivation.as_ref(), Some(&expected));
    let four = import_with(&data, Some(as_format_4(&sidecar)), false, false);
    assert_eq!(four.status, "imported", "{:?}", four.diagnostics);
    assert!(has_notice(&four, "MBF-IMPORT-REFERENCE-UNRECOVERABLE", "component:bend-1.node"));
    assert!(has_notice(&four, "MBF-IMPORT-SOURCE-COMPLETENESS-UNESTABLISHED", "unexported members"));
    assert_eq!((four.identity_recovery.known_missing_ids, four.identity_recovery.unrecovered_fields), (0, 1));
    assert_eq!(component_node(&four, "component:bend-1").as_deref(), Some("node:T1"));
    assert_eq!(four.model_bytes().unwrap(), six.model_bytes().unwrap(), "the near node is this model's own choice");
    assert!(four.derivation.is_none());
    let mut elsewhere = sidecar.clone();
    set(&mut elsewhere, &[], "mbf_sha256", Value::str("0".repeat(64)));
    for result in [
        import_with(&data, None, false, true),
        import_with(&data, Some(redigested(elsewhere)), false, true),
        import_with(&data, Some(sidecar.clone()), true, true),
    ] {
        assert_eq!(result.status, "imported", "{:?}", result.diagnostics);
        assert!(result.derivation.is_none());
    }
}

#[test]
fn near_and_far_bend_choices_give_different_identity_files_and_each_is_restored() {
    let original = parse(EXPORT_MODEL);
    let mut texts = Vec::new();
    let mut digests = Vec::new();
    for endpoint in ["node:T1", "node:T2"] {
        let mut model = original.clone();
        let Value::List(components) = member(&mut model, &["components"]) else { panic!("components") };
        components[0].as_dict_mut().expect("component").insert("node".into(), Value::str(endpoint));
        let (_, data, sidecar) = written(&model, &celsius());
        let bend = sidecar.get_or_null("element_map").get_or_null("pipe:BEND").get_or_null("bend").clone();
        assert_eq!(bend.get_or_null("component").as_str(), Some("component:bend-1"));
        assert_eq!(bend.get_or_null("node").as_str(), Some(endpoint));
        let result = import_with(&data, Some(sidecar.clone()), false, false);
        assert_eq!(result.status, "imported", "{:?}", result.diagnostics);
        assert_eq!(component_node(&result, "component:bend-1").as_deref(), Some(endpoint));
        assert!(!has_notice(&result, "MBF-IMPORT-REFERENCE-UNRECOVERABLE", "component:bend-1.node"));
        assert_eq!(result.identity_recovery.unrecovered_fields, 0);
        let four = import_with(&data, Some(as_format_4(&sidecar)), false, false);
        assert_eq!(component_node(&four, "component:bend-1").as_deref(), Some("node:T1"), "format 4 keeps the near node");
        assert!(has_notice(&four, "MBF-IMPORT-REFERENCE-UNRECOVERABLE", "component:bend-1.node"));
        texts.push(data);
        digests.push(sidecar.get_or_null("sidecar_sha256").as_str().map(String::from));
    }
    assert_eq!(texts[0], texts[1], "the MBF bytes do not carry the endpoint choice");
    assert_ne!(digests[0], digests[1], "the format-6 identity file does");
}

/// Pin control (A-3's nine identity-set classes): an entity the export does not carry is named in the loss report and
/// absent from the identity file, or the export is refused by name. None is dropped silently; a silent drop fails
/// here and is not repaired by the test.
#[test]
fn the_loss_report_names_every_identity_set_entity_the_export_does_not_carry() {
    let base = parse(EXPORT_MODEL);
    let entity = |text: &str| parse(text);
    let mut cases: Vec<(&str, Value, &str, (&str, &str), bool)> = Vec::new();
    let mut model = base.clone();
    push(&mut model, &["nodes"], entity(r#"{"id":"node:FREE","position":{"x":9.0,"y":9.0,"z":0.0}}"#));
    cases.push(("nodes", model, "node:FREE", ("MBF-SI-NODE-NOT-EXPORTED", "node:node:FREE"), true));
    let mut model = base.clone();
    let mut pipe = base.get_or_null("pipe_segments").as_list().expect("pipes")[3].clone();
    set(&mut pipe, &[], "id", Value::str("pipe:LOOP"));
    set(&mut pipe, &[], "from", Value::str("node:C"));
    set(&mut pipe, &[], "to", Value::str("node:A"));
    push(&mut model, &["pipe_segments"], pipe);
    cases.push(("pipes", model, "pipe:LOOP", ("MBF-SI-TOPOLOGY-UNSUPPORTED", "pipe:pipe:S1"), false));
    let mut model = base.clone();
    push(&mut model, &["components"], entity(r#"{"id":"component:valve-1","kind":"rigid","node":"node:B","mechanics_interface":{"solver_consumption":"rigid_element"}}"#));
    cases.push(("components", model, "component:valve-1", ("MBF-SI-COMPONENT-UNSUPPORTED", "component:component:valve-1"), false));
    let mut model = base.clone();
    push(&mut model, &["supports"], entity(r#"{"id":"support:EMPTY","node":"node:B","restraints":[]}"#));
    cases.push(("supports", model, "support:EMPTY", ("MBF-SI-SUPPORT-EMPTY", "support:support:EMPTY"), true));
    let mut model = base.clone();
    push(&mut model, &["load_cases"], entity(r#"{"id":"load_case:empty","primitive_loads":[]}"#));
    cases.push(("load cases", model, "load_case:empty", ("MBF-SI-LOAD-CASE-EMPTY", "load_case:load_case:empty"), true));
    let mut model = base.clone();
    let Value::List(cases_list) = member(&mut model, &["load_cases"]) else { panic!("load cases") };
    let mut zero = cases_list[0].get_or_null("primitive_loads").as_list().expect("loads")[0].clone();
    set(&mut zero, &[], "id", Value::str("load:zero"));
    set(&mut zero, &[], "direction", Value::str("global_z"));
    set(&mut zero, &["magnitude"], "value", Value::Float(0.0));
    let Value::List(loads) = cases_list[0].as_dict_mut().expect("case").get_mut(&"primitive_loads".to_string()).expect("loads") else { panic!("loads") };
    loads.push(zero);
    cases.push(("primitive loads", model, "load:zero", ("MBF-SI-PREFLIGHT-REFUSED", "load_map"), false));
    let mut model = base.clone();
    let mut material = base.get_or_null("materials").as_list().expect("materials")[0].clone();
    set(&mut material, &[], "id", Value::str("material:unused"));
    push(&mut model, &["materials"], material);
    cases.push(("materials", model, "material:unused", ("MBF-SI-FIELD-NOT-EXPORTED", "model.materials"), true));
    cases.push(("pressure regions", parse(PRESSURE_L_ZERO), "region:L", ("MBF-SI-PRESSURE-REGION-ZERO", "load_case:case:p/region:region:L"), true));
    let mut model = base.clone();
    let mut section = base.get_or_null("sections").as_list().expect("sections")[0].clone();
    set(&mut section, &[], "id", Value::str("section:unused"));
    push(&mut model, &["sections"], section);
    cases.push(("sections", model, "section:unused", ("MBF-SI-FIELD-NOT-EXPORTED", "model.sections"), true));
    assert_eq!(cases.len(), 9);
    for (name, model, identity, (code, location), written_back) in cases {
        let result = mbf::render_si(&model, &WriterSettings::new("PIN")).expect("a writer result");
        let entries: Vec<&Value> = result
            .loss_report
            .iter()
            .filter(|item| item.get_or_null("code").as_str() == Some(code) && item.get_or_null("entity").as_str() == Some(location))
            .collect();
        assert!(!entries.is_empty(), "{name}: {:?}", result.loss_report);
        if !written_back {
            assert_eq!((result.status.as_str(), result.text.is_none()), ("refused", true), "{name}");
            continue;
        }
        assert_eq!(result.status, "written", "{name}: {:?}", result.diagnostics);
        assert!(entries.iter().all(|item| item.get_or_null("category").as_str() == Some("omitted")), "{name}");
        assert!(entries.iter().any(|item| {
            item.get_or_null("entity").as_str().unwrap_or_default().contains(identity) || item.get_or_null("reason").as_str().unwrap_or_default().contains(identity)
        }), "{name}");
        let text = result.text.clone().expect("text").into_bytes();
        let mut sidecar = result.sidecar(&common::sha(&text)).expect("a sidecar");
        remove(&mut sidecar, &[], "derivation");
        let carried = String::from_utf8(mbf::canonical_bytes(&sidecar).expect("bytes")).expect("ASCII");
        assert!(!carried.contains(&format!("\"{identity}\"")), "{name}");
    }
}

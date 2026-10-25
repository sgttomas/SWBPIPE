//! No silent combination row drops (Agent 0, 2026-10-23), at the one combination
//! reader. The rows are the actual solved case rows of the bundled demo with its
//! linear supports only (so its `mechanics` combination publishes), handed to
//! `append_combination_results` exactly as the solve hands them; each probe
//! removes or invalidates one source to reach a withholding branch that a valid
//! request does not otherwise reach once duplicate terms block before solving.
use super::*;
use crate::{
    qualified_combination_result_id, qualified_load_case_result_id,
    run_linear_static_preview_value_with_mode, PreviewSolverMode,
};
use std::collections::HashMap;

const COMBINATION: &str = "combination:C-OPER-ALT";
const FIRST: &str = "load:L-100";
const SECOND: &str = "load:L-200";

struct Fixture {
    model: PreviewModel,
    case_rows: Vec<ResultItem>,
    rows_by_base_id: HashMap<String, HashMap<String, ResultItem>>,
    published: Vec<ResultItem>,
}

fn fixture(mode: PreviewSolverMode) -> Fixture {
    let mut model: serde_json::Value =
        serde_json::from_str(include_str!("../../../fixtures/product_preview/invented_demo_model.json")).unwrap();
    model["supports"]
        .as_array_mut()
        .unwrap()
        .retain(|support| matches!(support["family"].as_str(), Some("anchor" | "guide")));
    let request = serde_json::json!({"model": model, "materials": []});
    let envelope = run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap();
    assert_eq!(envelope.status.mechanics, "MECHANICS_SOLVED");
    let of = |kind: &str| -> Vec<ResultItem> {
        envelope.results.iter().filter(|r| r.basis_ref.as_ref().is_some_and(|b| b.ref_type == kind)).cloned().collect()
    };
    let case_rows = of("load_case");
    let published = of("combination");
    assert!(!published.is_empty());
    // Staged as the solve stages them: a row's base id is its first-case id; the
    // second case's row carries it qualified (its `_v2` rows keep their own
    // case-specific ids, so they never pair across cases).
    let base_of: HashMap<String, String> = case_rows
        .iter()
        .filter(|r| r.basis_ref.as_ref().unwrap().ref_id == FIRST && !r.kind.ends_with("_v2"))
        .map(|r| (qualified_load_case_result_id(SECOND, &r.id), r.id.clone()))
        .collect();
    let mut rows_by_base_id: HashMap<String, HashMap<String, ResultItem>> = HashMap::new();
    for row in &case_rows {
        let case = row.basis_ref.as_ref().unwrap().ref_id.clone();
        let base = if case == SECOND && !row.kind.ends_with("_v2") { base_of[&row.id].clone() } else { row.id.clone() };
        rows_by_base_id.entry(base).or_default().insert(case, row.clone());
    }
    let model: PreviewModel = serde_json::from_value(request["model"].clone()).unwrap();
    Fixture { model, case_rows, rows_by_base_id, published }
}

fn rendered() -> Rendered {
    Rendered { preview_cases: Vec::new(), nonlinear: false, consuming_constant_effort: false }
}

/// The combination rows and the combination-reader diagnostics for `fixture`.
fn combine(fixture: &Fixture) -> (Vec<ResultItem>, Vec<Diagnostic>) {
    let mut results = fixture.case_rows.clone();
    let mut diagnostics = Vec::new();
    append_combination_results(&fixture.model, &rendered(), &fixture.rows_by_base_id, &mut results, &mut diagnostics);
    (results.split_off(fixture.case_rows.len()), diagnostics)
}

fn bytes(rows: &[ResultItem]) -> Vec<u8> {
    serde_json::to_vec(rows).unwrap()
}

fn withheld(diagnostics: &[Diagnostic]) -> Vec<&Diagnostic> {
    diagnostics.iter().filter(|d| d.code == "LOAD_COMBINATION_ROW_WITHHELD").collect()
}

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];

/// Control: the reader reproduces the published combination rows byte for byte
/// and adds no withheld or missing-source diagnostic.
#[test]
fn the_reader_reproduces_the_published_combination_rows() {
    for mode in MODES {
        let fixture = fixture(mode);
        let (rows, diagnostics) = combine(&fixture);
        assert_eq!(bytes(&rows), bytes(&fixture.published), "{mode:?}");
        let codes: Vec<&str> = diagnostics.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes, ["COMBINATION_STRESS_MAXIMUM_UNAVAILABLE", "COMBINATION_INTENSIFIED_STRESS_UNAVAILABLE"], "{mode:?}");
    }
}

/// A source row missing in the first (reference) operand is reported like one
/// missing in any other operand.
#[test]
fn a_row_missing_in_the_first_operand_is_reported() {
    for mode in MODES {
        let fixture = fixture(mode);
        let base = fixture.rows_by_base_id.iter().find(|(_, rows)| rows.get(FIRST).is_some_and(|r| r.kind == "element_local_axial_force")).map(|(id, _)| id.clone()).unwrap();
        for (removed, kept) in [(FIRST, SECOND), (SECOND, FIRST)] {
            let mut probe = Fixture { model: fixture.model.clone(), case_rows: fixture.case_rows.clone(), rows_by_base_id: fixture.rows_by_base_id.clone(), published: fixture.published.clone() };
            probe.rows_by_base_id.get_mut(&base).unwrap().remove(removed);
            assert!(probe.rows_by_base_id[&base].contains_key(kept));
            let (rows, diagnostics) = combine(&probe);
            let combined = qualified_combination_result_id(COMBINATION, &base);
            assert!(fixture.published.iter().any(|r| r.id == combined), "{mode:?}: the control publishes {combined}");
            assert!(!rows.iter().any(|r| r.id == combined), "{mode:?}/{removed}");
            let missing: Vec<&Diagnostic> = diagnostics.iter().filter(|d| d.code == "LOAD_COMBINATION_SOURCE_RESULT_MISSING").collect();
            assert_eq!(missing.len(), 1, "{mode:?}/{removed}: {diagnostics:?}");
            assert!(missing[0].message.contains(&base), "{mode:?}/{removed}: {}", missing[0].message);
            assert_eq!(missing[0].affected_refs, vec![COMBINATION.to_string()]);
        }
    }
}

/// An ordinary row the load-case algebra refuses is withheld by name.
#[test]
fn an_algebra_refused_row_is_withheld_by_name() {
    for mode in MODES {
        let mut fixture = fixture(mode);
        let base = fixture.rows_by_base_id.iter().find(|(_, rows)| rows.get(FIRST).is_some_and(|r| r.kind == "element_local_bending_moment_y")).map(|(id, _)| id.clone()).unwrap();
        fixture.rows_by_base_id.get_mut(&base).unwrap().get_mut(SECOND).unwrap().value = f64::NAN;
        let (rows, diagnostics) = combine(&fixture);
        assert_eq!(rows.len(), fixture.published.len() - 1, "{mode:?}");
        let named = withheld(&diagnostics);
        assert_eq!(named.len(), 1, "{mode:?}: {diagnostics:?}");
        assert_eq!(named[0].severity, "warning");
        assert_eq!(named[0].affected_refs, vec![COMBINATION.to_string()]);
        assert!(named[0].message.contains(COMBINATION) && named[0].message.contains(&base), "{}", named[0].message);
    }
}

/// A support group publishes whole or not at all, and a withheld group is named.
#[test]
fn a_support_group_is_never_published_partially() {
    for mode in MODES {
        let mut fixture = fixture(mode);
        let support = "support:S-100";
        let my = fixture
            .case_rows
            .iter_mut()
            .find(|r| {
                r.entity_ref == support
                    && r.basis_ref.as_ref().unwrap().ref_id == SECOND
                    && r.kind.starts_with("support_reaction_")
                    && r.metadata.as_ref().is_some_and(|m| m.component == "My")
            })
            .unwrap();
        my.value = f64::NAN;
        let (rows, diagnostics) = combine(&fixture);
        let support_rows = |rows: &[ResultItem]| rows.iter().filter(|r| r.kind.starts_with("support_reaction_") && r.entity_ref == support).count();
        assert_eq!(support_rows(&fixture.published), 8, "{mode:?}");
        assert_eq!(support_rows(&rows), 0, "{mode:?}: no partial support group");
        assert_eq!(rows.len(), fixture.published.len() - 8, "{mode:?}");
        let named = withheld(&diagnostics);
        assert_eq!(named.len(), 1, "{mode:?}: {diagnostics:?}");
        assert!(named[0].message.contains(support), "{}", named[0].message);
    }
}

/// The withheld list is bounded: eight rows are named, the rest counted.
#[test]
fn the_withheld_list_is_bounded() {
    for mode in MODES {
        let mut fixture = fixture(mode);
        let mut bases: Vec<String> = fixture.rows_by_base_id.iter().filter(|(_, rows)| rows.get(FIRST).is_some_and(|r| r.kind == "element_local_axial_force")).map(|(id, _)| id.clone()).collect();
        bases.sort();
        assert!(bases.len() > 8);
        for base in &bases {
            fixture.rows_by_base_id.get_mut(base).unwrap().get_mut(SECOND).unwrap().value = f64::NAN;
        }
        let (_, diagnostics) = combine(&fixture);
        let named = withheld(&diagnostics);
        assert_eq!(named.len(), 1, "{mode:?}");
        let listed = bases.iter().filter(|base| named[0].message.contains(base.as_str())).count();
        assert_eq!(listed, 8, "{}", named[0].message);
        assert!(named[0].message.contains(&format!("and {} more", bases.len() - 8)), "{}", named[0].message);
    }
}

/// Closed nested records (Agent 0, 2026-10-25): an undeclared key on a node, a
/// pipe segment, a section, a support, a component, a material or one of their
/// member records and quantities is named by the value entry's scan with its
/// record and path, and every typed reader refuses the same key. A request
/// carrying every family's records and declared members passes both.
mod nested_records {
    use crate::model_fields::unknown_field_refusal;
    use crate::LinearStaticPreviewRequest;
    use serde_json::{json, Value};

    fn q(value: f64, unit: &str) -> Value {
        json!({"value": value, "unit": unit})
    }

    /// One record of every closed family, each with its member records,
    /// quantities and declared (solve-ignored) members.
    fn request() -> Value {
        let annotations = |extra: Value| {
            let mut members = json!({"tags": ["t"], "notes": "n"});
            members.as_object_mut().unwrap().extend(extra.as_object().unwrap().clone());
            members
        };
        let mut node = json!({"id": "node:N", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": "p"});
        let mut pipe = json!({"id": "pipe:P", "from": "node:N", "to": "node:M", "material": "material:M", "section_ref": "section:S",
            "section": {"outside_diameter": q(0.1, "m"), "wall_thickness": q(0.01, "m"), "mill_tolerance": q(0.001, "m"),
                "corrosion_allowance": q(0.0, "m"), "material_density": q(7850.0, "kg/m^3"), "contents_density": q(1000.0, "kg/m^3"),
                "insulation_thickness": q(0.05, "m"), "insulation_density": q(100.0, "kg/m^3")},
            "y_reference": {"x": 0.0, "y": 0.0, "z": 1.0}, "provenance": "p"});
        let mut section = json!({"id": "section:S", "name": "s", "section_type": "pipe",
            "properties": {"outside_diameter": q(0.1, "m"), "wall_thickness": q(0.01, "m")}, "provenance": {"source": "any"}});
        let mut support = json!({"id": "support:S", "node": "node:N", "restraints": ["UX"], "family": "spring",
            "stiffness": {"dof": "UY", "value": q(1.0e6, "N/m")},
            "hanger": {"hanger_type": "variable_spring_hanger", "stiffness": {"dof": "UZ", "value": q(4.0e4, "N/m")},
                "installed_load": q(1.0, "N"), "cold_load": q(1.0, "N"), "hot_load": q(1.0, "N"), "constant_load": q(1.0, "N"),
                "travel_range": q(0.04, "m"), "movement_limit": q(0.05, "m"), "manufacturer_reference": "r", "source_reference": "r",
                "load_side_review_reference": "r", "mechanics_consumption": "c"},
            "nonlinear": {"behavior": "friction", "dof": "UZ", "initial_state": "sliding", "gap": q(0.0, "m"),
                "friction_coefficient": q(0.3, "none"), "normal_reaction": q(1.0, "N"),
                "normal_reaction_source": {"support_ref": "support:T", "dof": "UY"}},
            "provenance": "p"});
        let mut component = json!({"id": "component:C", "kind": "valve", "node": "node:N", "label": "c",
            "objective_connector": {"version": "1.0.0", "motion_basis": "symmetric_midpoint_small_rotation_v1"},
            "geometry": {"rigid_pipe_ref": "pipe:P", "rigid_body_length": q(0.7, "m"), "weight": q(420.0, "N"),
                "center_of_gravity": {"x": 0.1, "y": 0.0, "z": 0.0, "unit": "m"}},
            "modifiers": {"sif_user_value": q(1.1, "none"), "source_reference": "r"},
            "mechanics_interface": {"solver_consumption": "mechanics_geometry_only", "rule_check_consumption": "r"},
            "provenance": "p"});
        let material = |id: &str| json!({"id": id, "elastic_modulus": q(2.0e11, "Pa"), "poisson_ratio": q(0.3, "1"),
            "shear_modulus": q(7.7e10, "Pa"), "thermal_expansion_coefficient": q(1.2e-5, "1/degC"), "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
            "temperature_points": [{"id": "point:hot", "temperature": q(120.0, "degC"), "elastic_modulus": q(1.9e11, "Pa"),
                "poisson_ratio": q(0.3, "1"), "shear_modulus": q(7.3e10, "Pa"), "thermal_expansion_coefficient": q(1.3e-5, "1/degC"), "provenance": "p"}],
            "provenance": "p"});
        let mut model_material = material("material:M");
        for (record, extra) in [
            (&mut node, json!({"label": "n"})),
            (&mut pipe, json!({"label": "p"})),
            (&mut section, json!({"provenance_record": {"source": "s"}})),
            (&mut support, json!({"label": "s", "boundary_association": {"kind": "k"}, "properties": {"k": 1}, "provenance_record": {"source": "s"}})),
            (&mut component, json!({"completeness": [], "provenance_record": {"source": "s"}})),
            (&mut model_material, json!({"label": "m", "provenance_record": {"source": "s"}})),
        ] {
            record.as_object_mut().unwrap().extend(annotations(extra).as_object().unwrap().clone());
        }
        json!({"model": {"schema_version": "0.7.0", "document_kind": "model", "project": {"id": "project:P"},
            "analysis_status": {"mechanics": "m", "rule_check": "r", "professional_acceptance": "a"},
            "nodes": [node], "pipe_segments": [pipe], "sections": [section], "supports": [support], "components": [component],
            "materials": [model_material]},
            "materials": [material("material:R")]})
    }

    /// (pointer of the record, named record, path)
    const RECORDS: [(&str, &str, &str); 28] = [
        ("/model/nodes/0", "node `node:N`", "model.nodes[0]"),
        ("/model/nodes/0/position", "the position of node `node:N`", "model.nodes[0].position"),
        ("/model/pipe_segments/0", "pipe segment `pipe:P`", "model.pipe_segments[0]"),
        ("/model/pipe_segments/0/y_reference", "the y_reference of pipe segment `pipe:P`", "model.pipe_segments[0].y_reference"),
        ("/model/pipe_segments/0/section", "the section of pipe segment `pipe:P`", "model.pipe_segments[0].section"),
        ("/model/pipe_segments/0/section/corrosion_allowance", "the corrosion_allowance of the section of pipe segment `pipe:P`", "model.pipe_segments[0].section.corrosion_allowance"),
        ("/model/pipe_segments/0/section/insulation_density", "the insulation_density of the section of pipe segment `pipe:P`", "model.pipe_segments[0].section.insulation_density"),
        ("/model/sections/0", "section `section:S`", "model.sections[0]"),
        ("/model/sections/0/properties/wall_thickness", "the wall_thickness of the properties of section `section:S`", "model.sections[0].properties.wall_thickness"),
        ("/model/supports/0", "support `support:S`", "model.supports[0]"),
        ("/model/supports/0/stiffness", "the stiffness of support `support:S`", "model.supports[0].stiffness"),
        ("/model/supports/0/stiffness/value", "the value of the stiffness of support `support:S`", "model.supports[0].stiffness.value"),
        ("/model/supports/0/hanger", "the hanger of support `support:S`", "model.supports[0].hanger"),
        ("/model/supports/0/hanger/stiffness/value", "the value of the stiffness of the hanger of support `support:S`", "model.supports[0].hanger.stiffness.value"),
        ("/model/supports/0/hanger/movement_limit", "the movement_limit of the hanger of support `support:S`", "model.supports[0].hanger.movement_limit"),
        ("/model/supports/0/nonlinear", "the nonlinear of support `support:S`", "model.supports[0].nonlinear"),
        ("/model/supports/0/nonlinear/normal_reaction_source", "the normal_reaction_source of the nonlinear of support `support:S`", "model.supports[0].nonlinear.normal_reaction_source"),
        ("/model/supports/0/nonlinear/gap", "the gap of the nonlinear of support `support:S`", "model.supports[0].nonlinear.gap"),
        ("/model/components/0", "component `component:C`", "model.components[0]"),
        ("/model/components/0/objective_connector", "the objective_connector of component `component:C`", "model.components[0].objective_connector"),
        ("/model/components/0/geometry", "the geometry of component `component:C`", "model.components[0].geometry"),
        ("/model/components/0/geometry/center_of_gravity", "the center_of_gravity of the geometry of component `component:C`", "model.components[0].geometry.center_of_gravity"),
        ("/model/components/0/modifiers/sif_user_value", "the sif_user_value of the modifiers of component `component:C`", "model.components[0].modifiers.sif_user_value"),
        ("/model/components/0/mechanics_interface", "the mechanics_interface of component `component:C`", "model.components[0].mechanics_interface"),
        ("/model/materials/0", "material `material:M`", "model.materials[0]"),
        ("/model/materials/0/temperature_points/0", "temperature point `point:hot` of material `material:M`", "model.materials[0].temperature_points[0]"),
        ("/model/materials/0/temperature_points/0/temperature", "the temperature of temperature point `point:hot` of material `material:M`", "model.materials[0].temperature_points[0].temperature"),
        ("/materials/0/shear_modulus", "the shear_modulus of request material `material:R`", "materials[0].shear_modulus"),
    ];

    fn typed(request: &Value) -> Result<LinearStaticPreviewRequest, String> {
        serde_json::from_value(request.clone()).map_err(|error| error.to_string())
    }

    #[test]
    fn every_family_and_its_declared_members_pass_the_scan_and_the_typed_parse() {
        let request = request();
        assert_eq!(unknown_field_refusal(&request), None);
        let parsed = typed(&request).unwrap();
        assert_eq!(parsed.model.sections[0].properties.len(), 2);
        assert_eq!(parsed.model.pipe_segments[0].section.corrosion_allowance.as_ref().map(|q| q.value), Some(0.0));
        assert_eq!(parsed.materials[0].temperature_points.len(), 1);
    }

    #[test]
    fn an_unknown_key_on_a_nested_record_is_named_with_its_record_and_path() {
        for (pointer, record, path) in RECORDS {
            let mut request = request();
            request.pointer_mut(pointer).unwrap_or_else(|| panic!("{pointer}")).as_object_mut().unwrap().insert("mill_tolerence".into(), json!(1));
            let refusal = unknown_field_refusal(&request).unwrap_or_else(|| panic!("{pointer}: no refusal"));
            let named = format!("unknown field `mill_tolerence` in {record} ({path})");
            assert!(refusal.starts_with("UNKNOWN_MODEL_FIELD: request: ") && refusal.contains(&named), "{pointer}: expected `{named}` in {refusal}");
            assert_eq!(refusal.matches("unknown field").count(), 1, "{pointer}: {refusal}");
            let error = typed(&request).unwrap_err();
            assert!(error.contains("unknown field `mill_tolerence`"), "{pointer}: the typed parse refuses the same key: {error}");
        }
    }

    #[test]
    fn a_material_record_is_read_only_as_a_map() {
        // The closed record keeps the former flattened wire's map-only reading:
        // a positional (array) material record stays refused at both levels.
        for pointer in ["/model/materials/0", "/materials/0"] {
            let mut request = request();
            *request.pointer_mut(pointer).unwrap() = json!(["material:M", {"value": 2.0e11, "unit": "Pa"}]);
            let error = typed(&request).unwrap_err();
            assert!(error.contains("invalid type: sequence, expected struct MaterialRecordWire"), "{pointer}: {error}");
        }
    }

    #[test]
    fn a_nested_record_without_an_id_is_named_by_its_index() {
        let mut request = request();
        request["model"]["nodes"][0].as_object_mut().unwrap().remove("id");
        request["model"]["nodes"][0]["lable"] = json!("n");
        let refusal = unknown_field_refusal(&request).unwrap();
        assert!(refusal.contains("unknown field `lable` in node 0 (model.nodes[0])"), "{refusal}");
    }

    #[test]
    fn nested_findings_follow_the_closed_root_records_with_the_bounded_list() {
        let mut request = request();
        request["model"]["combination"] = json!([]);
        for index in 0..11 {
            request["model"]["pipe_segments"][0]["section"][format!("extra_{index:02}")] = json!(index);
        }
        let refusal = unknown_field_refusal(&request).unwrap();
        assert!(refusal.starts_with("UNKNOWN_MODEL_FIELD: request: unknown field `combination` in the model (model); "), "{refusal}");
        assert_eq!(refusal.matches("unknown field `extra_").count(), 7, "{refusal}");
        assert!(refusal.contains("; and 4 more."), "{refusal}");
    }
}

/// The material record reads exactly as main's flattened wire read it, from the
/// raw text and from a value, except that an unknown key is refused (Agent 0,
/// binding: no wire change). Main's definitions are kept here verbatim, renamed
/// (`25b7df82ae` lib.rs `MaterialRecordWire`, `MaterialInput`,
/// `MaterialTemperaturePointInput`). With `MATERIAL_WIRE_CORPUS=<a
/// projects/chirality-piping directory>`, `every_committed_material_record_reads_as_at_main`
/// compares every element of every `materials` array in its JSON files, sliced
/// from the raw text, and prints the counts.
mod material_wire {
    use crate::{Authored, ClosedMaterialRecord, ExpansionLawInput, Quantity};
    use serde::de::{DeserializeOwned, IgnoredAny};
    use serde::Deserialize;
    use serde_json::Value;

    #[derive(Deserialize)]
    #[serde(bound(deserialize = "L: Deserialize<'de> + Default"))]
    struct MaterialRecordWireAtMain<L> {
        #[serde(flatten)]
        material: MaterialInputAtMain,
        #[serde(default)]
        expansion_laws: Authored<L>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct MaterialInputAtMain {
        #[serde(default)]
        pub constitutive_basis: Option<String>,
        #[serde(default)]
        pub poisson_ratio: Option<Quantity>,
        pub id: String,
        pub elastic_modulus: Quantity,
        #[serde(default)]
        pub shear_modulus: Option<Quantity>,
        #[serde(default)]
        pub thermal_expansion_coefficient: Option<Quantity>,
        #[serde(default)]
        pub temperature_points: Vec<MaterialTemperaturePointInputAtMain>,
        #[serde(default)]
        pub provenance: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct MaterialTemperaturePointInputAtMain {
        #[serde(default)]
        pub poisson_ratio: Option<Quantity>,
        pub id: String,
        #[serde(default)]
        pub temperature: Option<Quantity>,
        #[serde(default)]
        pub elastic_modulus: Option<Quantity>,
        #[serde(default)]
        pub shear_modulus: Option<Quantity>,
        #[serde(default)]
        pub thermal_expansion_coefficient: Option<Quantity>,
        #[serde(default)]
        pub provenance: Option<String>,
    }

    /// A parse's outcome: the parsed value's `Debug` text, or the refusal.
    type Outcome = Result<String, String>;

    fn at_main<L: DeserializeOwned + Default + std::fmt::Debug>(parse: Result<MaterialRecordWireAtMain<L>, serde_json::Error>) -> Outcome {
        parse
            .map(|r| format!("{:?} {:?}", r.material, r.expansion_laws).replace("InputAtMain", "Input"))
            .map_err(|e| e.to_string())
    }

    fn at_head<L: DeserializeOwned + Default + std::fmt::Debug>(parse: Result<ClosedMaterialRecord<L>, serde_json::Error>) -> Outcome {
        parse.map(|r| format!("{:?} {:?}", r.material, r.expansion_laws)).map_err(|e| e.to_string())
    }

    /// Both levels' records (the model's laws typed, the request's ignored),
    /// read from the text and, where the text is a JSON value, from the value.
    fn outcomes(text: &str) -> Vec<(&'static str, Outcome, Outcome)> {
        let mut pairs = vec![
            ("model text", at_main(serde_json::from_str::<MaterialRecordWireAtMain<Vec<ExpansionLawInput>>>(text)),
             at_head(serde_json::from_str::<ClosedMaterialRecord<Vec<ExpansionLawInput>>>(text))),
            ("request text", at_main(serde_json::from_str::<MaterialRecordWireAtMain<IgnoredAny>>(text)),
             at_head(serde_json::from_str::<ClosedMaterialRecord<IgnoredAny>>(text))),
        ];
        if let Ok(value) = serde_json::from_str::<Value>(text) {
            pairs.push(("model value", at_main(serde_json::from_value::<MaterialRecordWireAtMain<Vec<ExpansionLawInput>>>(value.clone())),
                        at_head(serde_json::from_value::<ClosedMaterialRecord<Vec<ExpansionLawInput>>>(value.clone()))));
            pairs.push(("request value", at_main(serde_json::from_value::<MaterialRecordWireAtMain<IgnoredAny>>(value.clone())),
                        at_head(serde_json::from_value::<ClosedMaterialRecord<IgnoredAny>>(value))));
        }
        pairs
    }

    /// How a record's outcomes compare: the same value, both refused, or the
    /// intended change (main accepted, the head refuses an unknown key).
    #[derive(Debug, PartialEq)]
    enum Compared {
        Same,
        BothRefused,
        UnknownKeyRefused,
    }

    fn compare(text: &str) -> Compared {
        let mut verdicts = Vec::new();
        for (level, main, head) in outcomes(text) {
            verdicts.push(match (&main, &head) {
                (Ok(a), Ok(b)) if a == b => Compared::Same,
                (Err(_), Err(_)) => Compared::BothRefused,
                (Ok(_), Err(e)) if e.contains("unknown field `") => Compared::UnknownKeyRefused,
                _ => panic!("{level}: the material record reads differently from main:\n  text {text}\n  main {main:?}\n  head {head:?}"),
            });
        }
        // The text's verdict; a value has no duplicate key, so its levels may read the same where the text is refused.
        verdicts.remove(0)
    }

    const Q: &str = r#""elastic_modulus":{"value":200,"unit":"GPa"}"#;

    #[test]
    fn regression_cases_read_as_at_main() {
        use Compared::*;
        let cases: Vec<(String, Compared)> = vec![
            (format!(r#"{{"id":"m",{Q}}}"#), Same),
            (format!(r#"{{"id":"m",{Q},"label":"a","label":"b"}}"#), Same),
            (format!(r#"{{"id":"m",{Q},"notes":"a","notes":["b"]}}"#), Same),
            (format!(r#"{{"id":"m",{Q},"tags":["a"],"tags":["b"]}}"#), Same),
            (format!(r#"{{"id":"m",{Q},"provenance_record":{{"a":1}},"provenance_record":{{"b":2}}}}"#), Same),
            (format!(r#"{{"id":"m",{Q},"notes":1e400}}"#), BothRefused),
            (format!(r#"{{"id":"m",{Q},"label":[1e400]}}"#), BothRefused),
            (format!(r#"{{"id":"m","elastic_modulus":{{"value":1e400,"unit":"GPa"}}}}"#), BothRefused),
            (format!(r#"{{"id":"m",{Q},{}}}"#, Q.replace("200", "1")), BothRefused),
            (format!(r#"{{"id":"m","id":"n",{Q}}}"#), BothRefused),
            (format!(r#"{{"id":"m",{Q},"expansion_laws":null,"expansion_laws":null}}"#), BothRefused),
            (format!(r#"{{"notes":"n",{Q},"expansion_laws":null,"provenance":"p","id":"m"}}"#), Same),
            (format!(r#"{{"provenance":"p","temperature_points":[],"id":"m",{Q},"label":"l"}}"#), Same),
            (format!(r#"{{"id":"m",{Q},"shear_modulus":null,"provenance":null,"constitutive_basis":null}}"#), Same),
            (format!(r#"{{"id":"m",{Q},"temperature_points":null}}"#), BothRefused),
            (format!(r#"{{"id":"m",{Q},"expansion_laws":null}}"#), Same),
            (format!(r#"{{"id":"m",{Q},"temperature_points":[{{"id":"t","temperature":{{"value":20,"unit":"degC"}},"elastic_modulus":null}}]}}"#), Same),
            (format!(r#"{{"id":"m",{Q},"temperature_points":[[null,"t"]]}}"#), Same),
            (format!(r#"{{"id":"m","elastic_modulus":[200,"GPa"]}}"#), Same),
            (format!(r#"["m",{{"value":200,"unit":"GPa"}}]"#), BothRefused),
            ("null".to_string(), BothRefused),
            (format!(r#"{{"id":"m",{Q},"lable":"a"}}"#), UnknownKeyRefused),
            (format!(r#"{{"id":"m",{Q},"x":1,"x":2}}"#), UnknownKeyRefused),
            (format!(r#"{{"id":"m","elastic_modulus":{{"value":200,"unit":"GPa","basis":"x"}}}}"#), UnknownKeyRefused),
            (format!(r#"{{"id":"m",{Q},"temperature_points":[{{"id":"t","tempreature":{{"value":20,"unit":"degC"}}}}]}}"#), UnknownKeyRefused),
        ];
        for (text, expected) in cases {
            assert_eq!(compare(&text), expected, "{text}");
        }
        // Duplicate known keys keep their typed refusal, with a position from the text.
        let text = format!(r#"{{"id":"m",{Q},{}}}"#, Q.replace("200", "1"));
        let refusal = serde_json::from_str::<ClosedMaterialRecord<IgnoredAny>>(&text).map(drop).unwrap_err().to_string();
        assert!(refusal.contains("duplicate field `elastic_modulus`") && refusal.contains("line 1 column"), "{refusal}");
    }

    /// The text of every element of an array held under a `materials` key, at
    /// any depth of a JSON text (the raw slices, never a parsed value).
    fn material_slices(text: &str) -> Vec<&str> {
        struct Scan<'a> { b: &'a [u8], i: usize, found: Vec<(usize, usize)> }
        impl Scan<'_> {
            fn ws(&mut self) { while self.i < self.b.len() && self.b[self.i].is_ascii_whitespace() { self.i += 1; } }
            fn string(&mut self) -> (usize, usize) {
                let start = self.i + 1;
                self.i += 1;
                while self.b[self.i] != b'"' { self.i += if self.b[self.i] == b'\\' { 2 } else { 1 }; }
                self.i += 1;
                (start, self.i - 1)
            }
            fn value(&mut self, materials: bool) {
                self.ws();
                match self.b[self.i] {
                    b'{' => {
                        self.i += 1;
                        loop {
                            self.ws();
                            if self.b[self.i] == b'}' { self.i += 1; break; }
                            let (a, z) = self.string();
                            self.ws();
                            self.i += 1; // ':'
                            self.value(&self.b[a..z] == b"materials");
                            self.ws();
                            if self.b[self.i] == b',' { self.i += 1; }
                        }
                    }
                    b'[' => {
                        self.i += 1;
                        loop {
                            self.ws();
                            if self.b[self.i] == b']' { self.i += 1; break; }
                            let start = self.i;
                            self.value(false);
                            if materials && self.b[start] == b'{' { self.found.push((start, self.i)); }
                            self.ws();
                            if self.b[self.i] == b',' { self.i += 1; }
                        }
                    }
                    b'"' => { self.string(); }
                    _ => { while self.i < self.b.len() && !b",}] \t\r\n".contains(&self.b[self.i]) { self.i += 1; } }
                }
            }
        }
        let mut scan = Scan { b: text.as_bytes(), i: 0, found: Vec::new() };
        scan.value(false);
        scan.found.into_iter().map(|(a, z)| &text[a..z]).collect()
    }

    #[test]
    fn the_raw_slicer_finds_nested_material_arrays() {
        let text = r#"{"a":{"materials":[{"id":"x" , "s":"]}\""},1,[2]]},"materials":[ {"id":"y"} ],"b":["materials",{"materials":[]}]}"#;
        assert_eq!(material_slices(text), vec![r#"{"id":"x" , "s":"]}\""}"#, r#"{"id":"y"}"#]);
    }

    #[test]
    fn every_committed_material_record_reads_as_at_main() {
        let Some(root) = std::env::var_os("MATERIAL_WIRE_CORPUS") else { return };
        fn files(dir: &std::path::Path, found: &mut Vec<std::path::PathBuf>) {
            let mut entries: Vec<_> = std::fs::read_dir(dir).unwrap().map(|entry| entry.unwrap().path()).collect();
            entries.sort();
            for path in entries {
                let name = path.file_name().unwrap().to_string_lossy().to_string();
                if path.is_dir() && !["node_modules", "target", ".git"].contains(&name.as_str()) {
                    files(&path, found);
                } else if name.ends_with(".json") {
                    found.push(path);
                }
            }
        }
        let mut paths = Vec::new();
        files(std::path::Path::new(&root), &mut paths);
        let (mut records, mut with, mut same, mut refused, mut unknown) = (0, 0, 0, 0, 0);
        let mut distinct = std::collections::BTreeSet::new();
        for path in paths {
            let text = std::fs::read_to_string(&path).unwrap();
            if serde_json::from_str::<IgnoredAny>(&text).is_err() {
                continue;
            }
            let slices = material_slices(&text);
            with += usize::from(!slices.is_empty());
            for slice in slices {
                records += 1;
                distinct.insert(slice.to_string());
                match compare(slice) {
                    Compared::Same => same += 1,
                    Compared::BothRefused => refused += 1,
                    Compared::UnknownKeyRefused => unknown += 1,
                }
            }
        }
        eprintln!("MATERIAL_WIRE_CORPUS: {records} records ({} distinct texts) in {with} files: {same} read the same value, {refused} refused by both, {unknown} refused for an unknown key only at the head", distinct.len());
        assert!(records > 0);
    }
}

//! K-D5 §3.2 on the ordinary preview route: every public ordinary maximum row
//! publishes the global maximum enclosure, `value_upper_pa ==
//! global_upper_bound_pa`, the row value is the enclosure midpoint the Rust
//! reader requires, and the published upper contains the member's own
//! published normal-stress station samples. The models are invented UDL lines
//! (one with two bends), not library, catalog or code-rule data.
use open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator;
use open_pipe_stress_product_physics::{run_linear_static_preview_with_mode, LinearStaticPreviewRequest, PreviewSolverMode};
use serde_json::{json, Value};

const P: &str = "invented K-D5 maximum-enclosure control; not library, catalog or code-rule data";

fn node(id: &str, x: f64, y: f64, z: f64) -> Value {
    json!({"id":id,"position":{"x":x,"y":y,"z":z},"provenance":P})
}
fn pipe(id: &str, from: &str, to: &str, y: [f64; 3]) -> Value {
    json!({"id":id,"from":from,"to":to,"material":"mat","y_reference":{"x":y[0],"y":y[1],"z":y[2]},
        "section":{"outside_diameter":{"value":0.168,"unit":"m"},"wall_thickness":{"value":0.007,"unit":"m"}},"provenance":P})
}
fn anchor(id: &str, node: &str) -> Value {
    json!({"id":id,"node":node,"family":"anchor","restraints":["UX","UY","UZ","RX","RY","RZ"],"provenance":P})
}
fn udl(pipe: &str, direction: &str, value: f64) -> Value {
    json!({"id":format!("w:{pipe}:{direction}"),"category":"distributed_force","target":{"type":"element","pipe":pipe},"direction":direction,
        "magnitude":{"value":value,"unit":"N/m"},"dimension":"force_per_length","provenance":P})
}
fn request(id: &str, nodes: Vec<Value>, pipes: Vec<Value>, supports: Vec<Value>, loads: Vec<Value>) -> LinearStaticPreviewRequest {
    serde_json::from_value(json!({"model":{"schema_version":"0.1.0","document_kind":"openpipestress.product_preview.model",
        "analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},
        "project":{"id":id,"units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","temperature":"degC","stress":"Pa"}},
        "nodes":nodes,"pipe_segments":pipes,
        "materials":[{"id":"mat","elastic_modulus":{"value":200e9,"unit":"Pa"},"shear_modulus":{"value":80e9,"unit":"Pa"},"thermal_expansion_coefficient":{"value":1.2e-5,"unit":"1/degC"},"provenance":P}],
        "supports":supports,"components":[],
        "load_cases":[{"id":"case","label":id,"kind":"primitive_user_load","primitive_loads":loads,"provenance":P}],"combinations":[]},"materials":[]}))
    .unwrap()
}
/// An invented U line at UTM-like coordinates (two 90-degree bends) whose
/// middle-leg end maxima tie, so the witness interval upper lies below the
/// global upper bound in both modes.
const U_LINE: &str = concat!(
    r#"{"model":{"schema_version":"0.1.0","document_kind":"openpipestress.product_preview.model","analysis_status":{""#,
    r#"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional"#,
    r#"_acceptance":"not_provided"},"project":{"id":"kd5-max:u-line-utm-skew","units":{"length":"m","force":"N","angl"#,
    r#"e":"rad","pressure":"Pa","temperature":"degC","stress":"Pa"}},"nodes":[{"id":"N0","position":{"x":7300000.0,"y"#,
    r#"":4600000.0,"z":312.5},"provenance":"invented K-D5 maximum-enclosure control"},{"id":"N1","position":{"x":7300"#,
    r#"002.4,"y":4600001.08,"z":313.94},"provenance":"invented K-D5 maximum-enclosure control"},{"id":"N2","position""#,
    r#":{"x":7300002.526113987,"y":4600001.40578719,"z":314.01046963026033},"provenance":"invented K-D5 maximum-enclo"#,
    r#"sure control"},{"id":"N3","position":{"x":7300001.627470963,"y":4600004.20420613,"z":313.40939380002624},"prov"#,
    r#"enance":"invented K-D5 maximum-enclosure control"},{"id":"N4","position":{"x":7300001.350384949,"y":4600004.34"#,
    r#"855332,"z":313.2379434302865},"provenance":"invented K-D5 maximum-enclosure control"},{"id":"N5","position":{""#,
    r#"x":7299998.950384948,"y":4600003.26855332,"z":311.7979434302865},"provenance":"invented K-D5 maximum-enclosure"#,
    r#" control"}],"pipe_segments":[{"id":"P0","from":"N0","to":"N1","material":"mat","y_reference":{"x":-0.519876129"#,
    r#"9388959,"y":0.016504004125044353,"z":0.8540822134710433},"section":{"outside_diameter":{"value":0.168,"unit":""#,
    r#"m"},"wall_thickness":{"value":0.007,"unit":"m"}},"provenance":"invented K-D5 maximum-enclosure control"},{"id""#,
    r#":"P1","from":"N1","to":"N2","material":"mat","y_reference":{"x":0.8,"y":0.36,"z":0.48},"section":{"outside_dia"#,
    r#"meter":{"value":0.168,"unit":"m"},"wall_thickness":{"value":0.007,"unit":"m"}},"provenance":"invented K-D5 max"#,
    r#"imum-enclosure control"},{"id":"P2","from":"N2","to":"N3","material":"mat","y_reference":{"x":-0.5198761299388"#,
    r#"959,"y":0.016504004125044353,"z":0.8540822134710433},"section":{"outside_diameter":{"value":0.168,"unit":"m"},"#,
    r#""wall_thickness":{"value":0.007,"unit":"m"}},"provenance":"invented K-D5 maximum-enclosure control"},{"id":"P3"#,
    r#"","from":"N3","to":"N4","material":"mat","y_reference":{"x":-0.29954767486955425,"y":0.9328063131475047,"z":-0"#,
    r#".20035861007803799},"section":{"outside_diameter":{"value":0.168,"unit":"m"},"wall_thickness":{"value":0.007,""#,
    r#"unit":"m"}},"provenance":"invented K-D5 maximum-enclosure control"},{"id":"P4","from":"N4","to":"N5","material"#,
    r#"":"mat","y_reference":{"x":-0.5198761299388959,"y":0.016504004125044353,"z":0.8540822134710433},"section":{"ou"#,
    r#"tside_diameter":{"value":0.168,"unit":"m"},"wall_thickness":{"value":0.007,"unit":"m"}},"provenance":"invented"#,
    r#" K-D5 maximum-enclosure control"}],"materials":[{"id":"mat","elastic_modulus":{"value":200000000000.0,"unit":""#,
    r#"Pa"},"shear_modulus":{"value":80000000000.0,"unit":"Pa"},"thermal_expansion_coefficient":{"value":1.2e-05,"uni"#,
    r#"t":"1/degC"},"provenance":"invented K-D5 maximum-enclosure control"}],"supports":[{"id":"aN0","node":"N0","fam"#,
    r#"ily":"anchor","restraints":["UX","UY","UZ","RX","RY","RZ"],"provenance":"invented K-D5 maximum-enclosure contr"#,
    r#"ol"},{"id":"aN5","node":"N5","family":"anchor","restraints":["UX","UY","UZ","RX","RY","RZ"],"provenance":"inve"#,
    r#"nted K-D5 maximum-enclosure control"}],"components":[{"id":"component:P1","label":"Invented bend","kind":"bend"#,
    r#"","node":"N2","geometry":{"bend_pipe_ref":"P1","bend_radius":{"value":0.252,"unit":"m"},"bend_geometry_source_"#,
    r#"reference":"invented_user_entered_preview_geometry","bend_angle":{"value":1.5707963252128452,"unit":"rad"}},"m"#,
    r#"odifiers":{"flexibility_factor_user_value":{"value":1.0,"unit":"none"},"source_reference":"invented_user_enter"#,
    r#"ed_preview_no_code_table","sif_user_value":{"value":1.0,"unit":"none"}},"mechanics_interface":{"solver_consump"#,
    r#"tion":"curved_bend_macro_element","rule_check_consumption":"user_rule_pack_inputs_only"},"provenance":"invente"#,
    r#"d K-D5 maximum-enclosure control","completeness":[{"finding_id":"finding:P1:geometry","status":"complete","dia"#,
    r#"gnostic_code":"BEND_GEOMETRY_INCOMPLETE","missing_field_kinds":[]}]},{"id":"component:P3","label":"Invented be"#,
    r#"nd","kind":"bend","node":"N4","geometry":{"bend_pipe_ref":"P3","bend_radius":{"value":0.252,"unit":"m"},"bend_"#,
    r#"geometry_source_reference":"invented_user_entered_preview_geometry","bend_angle":{"value":1.5707963260603928,""#,
    r#"unit":"rad"}},"modifiers":{"flexibility_factor_user_value":{"value":1.0,"unit":"none"},"source_reference":"inv"#,
    r#"ented_user_entered_preview_no_code_table","sif_user_value":{"value":1.0,"unit":"none"}},"mechanics_interface":"#,
    r#"{"solver_consumption":"curved_bend_macro_element","rule_check_consumption":"user_rule_pack_inputs_only"},"prov"#,
    r#"enance":"invented K-D5 maximum-enclosure control","completeness":[{"finding_id":"finding:P3:geometry","status""#,
    r#":"complete","diagnostic_code":"BEND_GEOMETRY_INCOMPLETE","missing_field_kinds":[]}]}],"load_cases":[{"id":"cas"#,
    r#"e","label":"STD-U|t24|R0.252|3-3-3|90|w|utm-skew","kind":"primitive_user_load","primitive_loads":[{"id":"w:P0""#,
    r#","category":"distributed_force","target":{"type":"element","pipe":"P0"},"direction":"global_z","magnitude":{"v"#,
    r#"alue":-450.0,"unit":"N/m"},"dimension":"force_per_length","provenance":"invented K-D5 maximum-enclosure contro"#,
    r#"l"},{"id":"w:P1","category":"distributed_force","target":{"type":"element","pipe":"P1"},"direction":"global_z""#,
    r#","magnitude":{"value":-450.0,"unit":"N/m"},"dimension":"force_per_length","provenance":"invented K-D5 maximum-"#,
    r#"enclosure control"},{"id":"w:P2","category":"distributed_force","target":{"type":"element","pipe":"P2"},"direc"#,
    r#"tion":"global_z","magnitude":{"value":-450.0,"unit":"N/m"},"dimension":"force_per_length","provenance":"invent"#,
    r#"ed K-D5 maximum-enclosure control"},{"id":"w:P3","category":"distributed_force","target":{"type":"element","pi"#,
    r#"pe":"P3"},"direction":"global_z","magnitude":{"value":-450.0,"unit":"N/m"},"dimension":"force_per_length","pro"#,
    r#"venance":"invented K-D5 maximum-enclosure control"},{"id":"w:P4","category":"distributed_force","target":{"typ"#,
    r#"e":"element","pipe":"P4"},"direction":"global_z","magnitude":{"value":-450.0,"unit":"N/m"},"dimension":"force_"#,
    r#"per_length","provenance":"invented K-D5 maximum-enclosure control"}],"provenance":"invented K-D5 maximum-enclo"#,
    r#"sure control"}],"combinations":[]},"materials":[]}"#,
);
fn models() -> Vec<(&'static str, LinearStaticPreviewRequest)> {
    vec![
        ("U line with a middle-leg tie", serde_json::from_str(U_LINE).unwrap()),
        ("fixed-fixed skew UDL line", request("kd5-max:fixed-fixed",
            vec![node("N0", 0.0, 0.0, 0.0), node("N1", 2.5, 1.0, 0.5), node("N2", 6.0, 2.0, 1.0)],
            vec![pipe("P0", "N0", "N1", [0.0, 0.0, 1.0]), pipe("P1", "N1", "N2", [0.0, 0.0, 1.0])],
            vec![anchor("aN0", "N0"), anchor("aN2", "N2")],
            vec![udl("P0", "global_z", -450.0), udl("P1", "global_z", -450.0), udl("P1", "global_y", 120.0)])),
        ("L line with UDL legs", request("kd5-max:l-line",
            vec![node("N0", 0.0, 0.0, 0.0), node("N1", 4.0, 0.0, 0.0), node("N2", 4.0, 3.0, 0.0)],
            vec![pipe("P0", "N0", "N1", [0.0, 0.0, 1.0]), pipe("P1", "N1", "N2", [0.0, 0.0, 1.0])],
            vec![anchor("aN0", "N0"), anchor("aN2", "N2")],
            vec![udl("P0", "global_z", -900.0), udl("P1", "global_z", -300.0), udl("P1", "global_x", 250.0)])),
    ]
}

fn mpa_in_pa(acc: &mut ExactAccumulator, value: f64, sign: f64) {
    acc.add_product(sign * value.abs(), 1e6).unwrap();
}

#[test]
fn kd5_preview_maximum_publishes_global_enclosure_midpoint_and_covers_public_samples() {
    for (label, base) in models() {
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let envelope = run_linear_static_preview_with_mode(base.clone(), mode);
            let integrity: Vec<_> = envelope.diagnostics.iter().filter(|d| d.code.starts_with("NUMERICAL_INTEGRITY_")).map(|d| d.code.clone()).collect();
            assert_eq!(integrity, vec!["NUMERICAL_INTEGRITY_CHECKS_PASSED".to_string()], "{label} {mode:?}: precondition, a certified Passed case");
            let evidence = serde_json::to_value(&envelope.contract_evidence).unwrap();
            let extrema: Vec<Value> = evidence["preview_cases"].as_array().unwrap().iter()
                .flat_map(|case| case["pipe_stress_extrema"].as_array().cloned().unwrap_or_default()).collect();
            let rows: Vec<_> = envelope.results.iter().filter(|r| r.kind == "pipe_elastic_normal_stress_maximum_v2").collect();
            let straight = if label.starts_with("U line") { 3 } else { 2 };
            assert_eq!(extrema.len(), straight, "{label} {mode:?}: one maximum per straight member");
            assert_eq!(rows.len(), extrema.len(), "{label} {mode:?}");
            for x in &extrema {
                let pipe_id = x["pipe_id"].as_str().unwrap();
                let lower = x["value_lower_pa"].as_f64().unwrap();
                let upper = x["value_upper_pa"].as_f64().unwrap();
                let global = x["global_upper_bound_pa"].as_f64().unwrap();
                // §3.2: the public upper is the global maximum enclosure, never the
                // local witness interval or a midpoint below it.
                assert_eq!(upper.to_bits(), global.to_bits(), "{label} {mode:?} {pipe_id}: value_upper_pa must equal global_upper_bound_pa");
                assert!(lower.is_finite() && lower <= upper, "{label} {mode:?} {pipe_id}");
                let row = rows.iter().find(|r| r.entity_ref == pipe_id).unwrap();
                assert_eq!(row.value.to_bits(), (lower + 0.5 * (upper - lower)).to_bits(), "{label} {mode:?} {pipe_id}: the row is the global enclosure midpoint");
                // Necessary condition of the strict public-sample check: U >= |axial| + max(|My|,|Mz|),
                // with the published MPa values converted to Pa exactly.
                let component = |kind: &str, location: &str| envelope.results.iter().find(|r| r.entity_ref == pipe_id && r.kind == kind
                    && r.metadata.as_ref().is_some_and(|m| m.location == location)).map(|r| r.value);
                let mut samples = 0;
                for location in ["end_i", "quarter_1", "midspan", "quarter_3", "end_j"] {
                    let (Some(a), Some(y), Some(z)) = (component("element_local_axial_normal_stress", location),
                        component("element_local_bending_normal_stress_y", location), component("element_local_bending_normal_stress_z", location)) else { continue };
                    samples += 1;
                    let mut acc = ExactAccumulator::new();
                    acc.add(upper).unwrap();
                    mpa_in_pa(&mut acc, a, -1.0);
                    mpa_in_pa(&mut acc, y.abs().max(z.abs()), -1.0);
                    assert!(acc.signum() >= 0, "{label} {mode:?} {pipe_id} {location}: the public upper misses a published station sample");
                }
                assert!(samples >= 3, "{label} {mode:?} {pipe_id}: published station samples found: {samples}");
            }
        }
    }
}

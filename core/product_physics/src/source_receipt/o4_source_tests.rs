//! O4 (DEL-04-08 Design `o4-source-extension.md` §3, §5 and §6): the
//! retained-source route (`pressure-source-1`) on Timoshenko and dual-basis
//! documents. Every control here is an in-memory re-authoring of a committed
//! current request (n05, fields), never a committed corpus entry:
//! - a Timoshenko twin of n05;
//! - fields with a nonzero mill tolerance and corrosion allowance (one member
//!   whose two stress sections differ), Euler-Bernoulli and Timoshenko;
//! - a two-member split of fields whose first member's sections differ and
//!   whose second member's sections are bit-equal.
//!
//! The reader mutations (`reduced_check_nominal_parameters`,
//! `reduced_maximum_copied`, `shear_area_changed`, `kappa_changed`, the
//! one-basis tamper) are made in memory on those genuine publications and
//! resealed, so only the named statement is wrong; the committed J2 corpus is
//! not regenerated. `o4_source_controls_are_current` writes the same controls
//! to a test-only fixture for the independent Python verifier.
use super::*;
use open_pipe_stress_result_export::physics_source as ps;
use open_pipe_stress_result_export::reader_verdict as rv;
use open_pipe_stress_result_export::source_blocks::domain_hash;

const TIMOSHENKO: &str = "timoshenko_annulus_energy_v1";
const EXACT: &str = "retained_source_blocks_exact_v1";
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
/// The twelve committed current source requests (MJ6/MJ9's controls).
const FAMILIES: [(&str, &[&str]); 2] = [
    ("physics_source", &["n05", "n06", "n05_unicode", "n05_units", "fields", "mixed", "mixed_units"]),
    ("load_reference_source", &["n05", "n06", "eigen_motion", "fields", "mixed"]),
];
/// RECIPE_WORK plus the cached identity-digest read of one endpoint-maximum
/// call (`composite.rs`, `source_recovery.rs` `retained_identity_digest`).
const MAXIMUM_CALL: i128 = 4096 + 128;
const CONTROLS_PATH: &str = "fixtures/product_preview/pressure_source/o4/o4_source_controls.json";

fn project_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn fixture_request(family: &str, name: &str) -> Value {
    let path = project_root().join(format!("fixtures/product_preview/pressure_source/{family}/{name}.request.json"));
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}
fn with_shear_model(mut request: Value, shear_model: &str) -> Value {
    request["model"]["pressure_contract"]["shear_model"] = json!(shear_model);
    request
}
/// `request` with pipe `pipe`'s mill tolerance (metres; `None` leaves the key
/// absent) and corrosion allowance (metres) authored.
fn with_section(mut request: Value, pipe: usize, mill: Option<f64>, allowance: f64) -> Value {
    let section = &mut request["model"]["pipe_segments"][pipe]["section"];
    if let Some(mill) = mill {
        section["mill_tolerance"] = json!({"value": mill, "unit": "m"});
    }
    section["corrosion_allowance"] = json!({"value": allowance, "unit": "m"});
    request
}
/// The single straight member of `request` split at x = 1 into two members.
fn two_member(mut request: Value) -> Value {
    let model = &mut request["model"];
    assert_eq!(model["pipe_segments"].as_array().unwrap().len(), 1);
    let mut mid = model["nodes"][1].clone();
    mid["id"] = json!("independent-mid");
    mid["label"] = json!("Invented independent-mid");
    mid["position"] = json!({"x": 1.0, "y": 0, "z": 0});
    model["nodes"].as_array_mut().unwrap().insert(1, mid);
    let mut second = model["pipe_segments"][0].clone();
    model["pipe_segments"][0]["to"] = json!("independent-mid");
    second["id"] = json!("independent-member-2");
    second["label"] = json!("Invented independent-member-2");
    second["from"] = json!("independent-mid");
    model["pipe_segments"].as_array_mut().unwrap().push(second);
    request
}
fn fields() -> Value {
    fixture_request("physics_source", "fields")
}
/// The O4 source controls, by id.
fn controls() -> Vec<(&'static str, Value)> {
    vec![
        ("timoshenko-n05", with_shear_model(fixture_request("physics_source", "n05"), TIMOSHENKO)),
        ("dual-basis-fields", with_section(fields(), 0, Some(0.0005), 0.001)),
        ("timoshenko-dual-basis-fields", with_shear_model(with_section(fields(), 0, Some(0.0005), 0.001), TIMOSHENKO)),
        ("mixed-two-member", with_section(two_member(fields()), 0, Some(0.0005), 0.001)),
    ]
}
fn control(id: &str) -> Value {
    controls().into_iter().find(|(c, _)| *c == id).unwrap().1
}
fn run(request: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(crate::run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap()).unwrap()
}
fn invocation(request: &Value, mode: PreviewSolverMode) -> Value {
    json!({"request": request, "solver_mode": mode.as_str()})
}
fn verdict(envelope: &Value, request: &Value, mode: PreviewSolverMode) -> Value {
    rv::validate("pressure-source", envelope, Some(&request["model"]), Some(&invocation(request, mode)))
}
fn eligible() -> Value {
    json!({"ok": true, "eligible": true, "findings": []})
}
fn refused(envelope: &Value, request: &Value, mode: PreviewSolverMode) -> String {
    let outcome = verdict(envelope, request, mode);
    assert_eq!(outcome["ok"], json!(false), "{outcome}");
    outcome["error"]["code"].as_str().unwrap().to_owned()
}
/// The physical case commitments, the publication commitment and the
/// receipt, recomputed over the (mutated) statement.
fn reseal(source: &mut Value) {
    let evidence = source["contract_evidence"].clone();
    let cases = source["source_block_recovery"]["body"]["cases"].as_array().unwrap().len();
    for index in 0..cases {
        let exact = &evidence["exact_cases"][index];
        let pressure: Vec<_> = evidence["pressure"].as_array().unwrap().iter().filter(|p| p["load_case_id"] == exact["load_case_id"]).collect();
        let mut proof = json!({"exact_case": exact, "pressure": pressure});
        if let Some(states) = evidence.get("load_reference_states") {
            proof["load_reference_state"] = states.as_array().unwrap().iter().find(|s| s["load_case_id"] == exact["load_case_id"]).unwrap().clone();
        }
        source["source_block_recovery"]["body"]["cases"][index]["physical_evidence_sha256"] =
            json!(domain_hash("pressure_source_case_evidence_v1", &proof).unwrap());
    }
    let mut publication = source.clone();
    publication.as_object_mut().unwrap().remove("source_block_recovery");
    source["source_block_recovery"]["body"]["publication_sha256"] = json!(domain_hash("source_blocks_publication_v1", &publication).unwrap());
    source["source_block_recovery"]["receipt_sha256"] =
        json!(domain_hash("source_blocks_receipt_v1", &source["source_block_recovery"]["body"]).unwrap());
}
fn exact_case(envelope: &Value) -> &Value {
    &envelope["contract_evidence"]["exact_cases"][0]
}
fn selected_case(envelope: &Value) -> &Value {
    let cases = envelope["source_block_recovery"]["body"]["cases"].as_array().unwrap();
    let selected: Vec<_> = cases.iter().filter(|c| c["selected_method"] == EXACT).collect();
    assert_eq!(selected.len(), 1);
    selected[0]
}
fn selected_case_index(envelope: &Value) -> usize {
    envelope["source_block_recovery"]["body"]["cases"].as_array().unwrap().iter().position(|c| c["selected_method"] == EXACT).unwrap()
}
fn section<'a>(envelope: &'a Value, pipe: &str) -> &'a Value {
    exact_case(envelope)["pipe_sections"].as_array().unwrap().iter().find(|s| s["pipe_id"] == pipe).unwrap()
}
/// Whether a member's two stress sections differ (A, Z or J).
fn distinct(section: &Value) -> bool {
    ["As_m2", "Z_m3", "J_m4"].iter().any(|k| section["nominal_section"][k] != section["reduced_section"][k])
}
fn row<'a>(envelope: &'a Value, id: &str) -> &'a Value {
    envelope["results"].as_array().unwrap().iter().find(|r| r["id"] == id).unwrap_or_else(|| panic!("row {id}"))
}
/// A reduced stress row's nominal twin id (source stress rows take the
/// hyphen-style id segment, maxima the underscore style).
fn twin(id: &str) -> Option<String> {
    id.strip_suffix(":reduced-section")
        .map(|s| format!("{s}:nominal-section"))
        .or_else(|| id.strip_suffix(":reduced_section").map(|s| format!("{s}:nominal_section")))
}
fn is_reduced(id: &str) -> bool {
    id.ends_with(":reduced-section") || id.ends_with(":reduced_section")
}
fn f(v: &Value) -> f64 {
    v.as_f64().unwrap_or_else(|| panic!("number: {v}"))
}
fn bits(value: f64) -> String {
    format!("{:016x}", value.to_bits())
}
/// The dump's payload (`normalized_source_text`) or plan of record `index`.
fn dumped(records: &[Value], index: usize, key: &str) -> Value {
    serde_json::from_str::<Value>(records[index][key].as_str().unwrap()).unwrap()["payload"].clone()
}

/// O4 §3.2 and §6: documents with a nonzero mill tolerance and allowance
/// select pressure-source-1 in both modes and read eligible with their
/// captured invocation (Rust raw reader, G2's model checks and transport).
/// A member whose sections differ publishes reduced stress rows formed on its
/// reduced section (not its nominal rows); a bit-equal member's reduced rows
/// are its nominal rows bit for bit.
#[test]
fn o4_dual_basis_source_case_selects() {
    for id in ["dual-basis-fields", "timoshenko-dual-basis-fields", "mixed-two-member"] {
        let request = control(id);
        for mode in MODES {
            let at = format!("{id} {}", mode.as_str());
            let envelope = run(&request, mode);
            assert_eq!(envelope["producer"]["semantic_contract_id"], json!(ps::PRESSURE_CONTRACT_ID), "{at}: {}", envelope["diagnostics"]);
            assert_eq!(verdict(&envelope, &request, mode), eligible(), "{at}");
            assert_eq!(ps::validate_transport_metadata_in(&envelope, ps::SourceContract::PressureV3), Ok(()), "{at}");
            let mut differing = 0;
            for pipe in request["model"]["pipe_segments"].as_array().unwrap() {
                let pipe = pipe["id"].as_str().unwrap();
                let distinct = distinct(section(&envelope, pipe));
                differing += usize::from(distinct);
                let mut compared = 0;
                for reduced in envelope["results"].as_array().unwrap().iter().filter(|r| {
                    r["entity_ref"] == pipe
                        && is_reduced(r["id"].as_str().unwrap())
                        && matches!(
                            r["kind"].as_str(),
                            Some("element_local_axial_normal_stress" | "element_local_bending_normal_stress_y"
                                | "element_local_bending_normal_stress_z" | "element_local_torsional_shear_stress")
                        )
                }) {
                    let nominal = row(&envelope, &twin(reduced["id"].as_str().unwrap()).unwrap());
                    let (n, r) = (f(&nominal["value"]), f(&reduced["value"]));
                    if distinct {
                        assert!(n == 0.0 && r == 0.0 || n.to_bits() != r.to_bits(), "{at}: {} is formed on its own section", reduced["id"]);
                    } else {
                        assert_eq!(n.to_bits(), r.to_bits(), "{at}: {} is its twin", reduced["id"]);
                    }
                    compared += 1;
                }
                assert_eq!(compared, 20, "{at}: {pipe}: twenty reduced source stress rows");
            }
            assert_eq!(differing, 1, "{at}: one member's sections differ");
        }
    }
}

/// O4 §3.2 (stress checks): each reduced `section_stress_checks` entry binds
/// its nominal twin's functional and action bit for bit, with the reduced
/// section's parameters (A_r, Z_r, J_r, r_o), and its row value is the
/// existing recipe on those parameters. The private plan's reduced recipes
/// read the payload's `reduced_recovery_section`. Mutation
/// `reduced_check_nominal_parameters` (in memory, resealed): the reduced checks
/// carrying the nominal parameters are refused by name, and so is the full
/// mutant whose reduced rows are then the nominal values.
#[test]
fn o4_source_reduced_checks_bind_the_reduced_section() {
    let request = control("dual-basis-fields");
    let mode = PreviewSolverMode::SparseInteractive;
    let (envelope, records) = with_source_dump(|| run(&request, mode));
    let case = selected_case(&envelope);
    let pipe = request["model"]["pipe_segments"][0]["id"].as_str().unwrap();
    let s = section(&envelope, pipe);
    assert!(distinct(s));
    let checks = case["section_stress_checks"].as_array().unwrap();
    let expected = |b: &str| {
        json!({"area_m2": s[b]["As_m2"], "section_modulus_m3": s[b]["Z_m3"], "torsion_radius_m": s["ro_m"], "torsion_constant_m4": s[b]["J_m4"], "pa_per_mpa": 1e6})
    };
    let mut reduced_checks = 0;
    for check in checks.iter().filter(|c| is_reduced(c["result_id"].as_str().unwrap())) {
        let twin_id = twin(check["result_id"].as_str().unwrap()).unwrap();
        let nominal = checks.iter().find(|c| c["result_id"] == twin_id.as_str()).unwrap();
        for key in ["recipe_id", "pipe_id", "location", "component", "functional_index", "functional_id", "action"] {
            assert_eq!(check[key], nominal[key], "{key}");
        }
        assert_eq!(check["parameters"], expected("reduced_section"));
        assert_eq!(nominal["parameters"], expected("nominal_section"));
        let (action, p) = (f(&check["action"]["value"]), &check["parameters"]);
        let pa = match check["component"].as_str().unwrap() {
            "axial_normal_stress" => action / f(&p["area_m2"]),
            "torsional_shear_stress" => action * f(&p["torsion_radius_m"]) / f(&p["torsion_constant_m4"]),
            _ => action / f(&p["section_modulus_m3"]),
        };
        assert_eq!(f(&row(&envelope, check["result_id"].as_str().unwrap())["value"]).to_bits(), (pa / 1e6).to_bits());
        reduced_checks += 1;
    }
    assert_eq!(reduced_checks, 20);
    // The private plan: a reduced stress or maximum recipe reads
    // `reduced_recovery_section`, a nominal one does not.
    assert_eq!(records.len(), 1);
    let plan = dumped(&records, 0, "functional_plan_text");
    let mut reduced_recipes = 0;
    for recipe in plan["derived_recipes"].as_array().unwrap() {
        if !matches!(recipe["recipe_id"].as_str(), Some("retained_source_straight_stress_v1" | "retained_source_endpoint_normal_max_v1")) {
            continue;
        }
        let reduced = is_reduced(recipe["result_id"].as_str().unwrap());
        for parameter in recipe["parameters"].as_array().unwrap() {
            if let Some(path) = parameter["operand"]["source_path"].as_str() {
                assert_eq!(path.contains("/reduced_recovery_section/"), reduced, "{path}");
            }
        }
        reduced_recipes += usize::from(reduced);
    }
    assert_eq!(reduced_recipes, 21, "twenty reduced stress rows and the reduced maximum");
    // reduced_check_nominal_parameters, the checks alone.
    let mut parameters = envelope.clone();
    let mutated = selected_case_index(&parameters);
    let nominal_parameters: BTreeMap<String, Value> =
        checks.iter().map(|c| (c["result_id"].as_str().unwrap().to_owned(), c["parameters"].clone())).collect();
    for check in parameters["source_block_recovery"]["body"]["cases"][mutated]["section_stress_checks"].as_array_mut().unwrap() {
        if let Some(twin_id) = twin(check["result_id"].as_str().unwrap()) {
            check["parameters"] = nominal_parameters[&twin_id].clone();
        }
    }
    reseal(&mut parameters);
    assert_eq!(refused(&parameters, &request, mode), "PHYSICS_SOURCE_STRESS_SECTION_PARAMETERS");
    // The full mutant: the reduced rows then carry the nominal values.
    let mut full = parameters.clone();
    let values: BTreeMap<String, Value> =
        envelope["results"].as_array().unwrap().iter().map(|r| (r["id"].as_str().unwrap().to_owned(), r["value"].clone())).collect();
    for row in full["results"].as_array_mut().unwrap() {
        let id = row["id"].as_str().unwrap().to_owned();
        if checks.iter().any(|c| c["result_id"] == id.as_str()) {
            if let Some(twin_id) = twin(&id) {
                row["value"] = values[&twin_id].clone();
            }
        }
    }
    reseal(&mut full);
    assert_eq!(refused(&full, &request, mode), "SOURCE_PHYSICS_ROW_BASIS_SECTION");
}

/// O4 §3.2 (endpoint maximum) and §3.5: each basis's maximum is evaluated
/// from the same retained endpoint functionals on its own section, the
/// reduced call taking the reduced section explicitly: the reduced
/// extremum's A and Z are the reduced section's bits, its endpoint values are
/// the recipe on them, its value exceeds the nominal one and its headline is
/// its own. The maximum is called twice at each of the two sites for a member
/// whose sections differ and once for a bit-equal member. Mutation
/// `reduced_maximum_copied` (in memory, resealed): the nominal maximum copied
/// to the reduced basis is refused by name.
#[test]
fn o4_source_maximum_per_basis() {
    for id in ["dual-basis-fields", "mixed-two-member"] {
        let request = control(id);
        for mode in MODES {
            let at = format!("{id} {}", mode.as_str());
            composite::start_trace();
            let envelope = run(&request, mode);
            let trace = composite::take_trace();
            let mut expected_calls = 0;
            for pipe in request["model"]["pipe_segments"].as_array().unwrap() {
                let pipe = pipe["id"].as_str().unwrap();
                let s = section(&envelope, pipe);
                let extrema: Vec<_> =
                    exact_case(&envelope)["pipe_stress_extrema"].as_array().unwrap().iter().filter(|e| e["pipe_id"] == pipe).collect();
                assert_eq!(extrema.len(), 2, "{at}");
                let (n, r) = (extrema[0], extrema[1]);
                assert_eq!((n["section_basis"].as_str(), r["section_basis"].as_str()), (Some("nominal_section"), Some("reduced_section")), "{at}");
                for (e, b) in [(n, "nominal_section"), (r, "reduced_section")] {
                    assert_eq!(f(&e["area_m2"]).to_bits(), f(&s[b]["As_m2"]).to_bits(), "{at}: {b} A");
                    assert_eq!(f(&e["section_modulus_m3"]).to_bits(), f(&s[b]["Z_m3"]).to_bits(), "{at}: {b} Z");
                    for endpoint in e["endpoints"].as_array().unwrap() {
                        let a = endpoint["actions"].as_array().unwrap();
                        let z = f(&e["section_modulus_m3"]);
                        let value = (f(&a[0]["value"]).abs() / f(&e["area_m2"])) + norm3(f(&a[1]["value"]).abs() / z, f(&a[2]["value"]).abs() / z, 0.0);
                        assert_eq!(f(&endpoint["value_pa"]).to_bits(), value.to_bits(), "{at}: {b} endpoint recipe");
                    }
                    assert_eq!(f(&row(&envelope, e["result_id"].as_str().unwrap())["value"]).to_bits(), f(&e["value_pa"]).to_bits(), "{at}");
                }
                let endpoints = |e: &Value| e["endpoints"].as_array().unwrap().iter().map(|p| (p["functional_indices"].clone(), p["actions"].clone())).collect::<Vec<_>>();
                assert_eq!(endpoints(n), endpoints(r), "{at}: one set of endpoint functionals");
                if distinct(s) {
                    assert!(f(&r["value_pa"]) > f(&n["value_pa"]), "{at}: the smaller reduced section");
                    expected_calls += 2;
                } else {
                    let strip = |e: &Value| {
                        let mut e = e.clone();
                        e.as_object_mut().unwrap().remove("result_id");
                        e.as_object_mut().unwrap().remove("section_basis");
                        e
                    };
                    assert_eq!(strip(n), strip(r), "{at}: a bit-equal member's two maxima are one");
                    expected_calls += 1;
                }
            }
            // Per-basis headlines bind their own basis's maxima.
            for b in ["nominal_section", "reduced_section"] {
                let headline = &envelope["summary"][format!("max_open_formula_stress_{b}")];
                let max = envelope["results"].as_array().unwrap().iter()
                    .filter(|r| r["kind"] == "pipe_elastic_normal_stress_maximum_v2" && r["id"].as_str().unwrap().ends_with(b))
                    .map(|r| f(&r["value"]))
                    .fold(f64::NEG_INFINITY, f64::max);
                assert_eq!(f(&headline["value"]).to_bits(), max.to_bits(), "{at}: {b} headline");
                assert!(headline["result_ref"].as_str().unwrap().ends_with(b), "{at}");
            }
            // The calls at the two sites: the publication loop (after the
            // support norms) and the composite's expected-rows pass.
            let supports = selected_case(&envelope)["source"]["support_ids"].as_array().unwrap().len();
            let stage = |name: &str| trace.iter().position(|t| t["stage"] == name).unwrap();
            let entries = |range: std::ops::Range<usize>| trace[range].iter().filter(|t| t["stage"] == "derived_recipe_entry").count();
            assert_eq!(entries(0..stage("source_case_finalization_entry")), supports + expected_calls, "{at}: publication-loop calls");
            assert_eq!(entries(stage("current_source_binding_complete")..stage("source_case_finalization_complete")), expected_calls, "{at}: composite calls");
            // reduced_maximum_copied: the distinct member's nominal maximum,
            // value and evidence, published as its reduced maximum.
            let pipe = request["model"]["pipe_segments"][0]["id"].as_str().unwrap();
            let mut copied = envelope.clone();
            let extrema = copied["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"].as_array_mut().unwrap();
            let n = extrema.iter().position(|e| e["pipe_id"] == pipe && e["section_basis"] == "nominal_section").unwrap();
            let r = extrema.iter().position(|e| e["pipe_id"] == pipe && e["section_basis"] == "reduced_section").unwrap();
            let mut nominal = extrema[n].clone();
            nominal["section_basis"] = extrema[r]["section_basis"].clone();
            nominal["result_id"] = extrema[r]["result_id"].clone();
            let reduced_id = nominal["result_id"].as_str().unwrap().to_owned();
            let value = nominal["value_pa"].clone();
            extrema[r] = nominal;
            for row in copied["results"].as_array_mut().unwrap() {
                if row["id"] == reduced_id.as_str() {
                    row["value"] = value.clone();
                }
            }
            if copied["summary"]["max_open_formula_stress_reduced_section"]["result_ref"] == reduced_id.as_str() {
                copied["summary"]["max_open_formula_stress_reduced_section"]["value"] = value.clone();
            }
            reseal(&mut copied);
            assert_eq!(refused(&copied, &request, mode), "PHYSICS_SOURCE_MAXIMUM_SECTION", "{at}");
        }
    }
}

/// O4 §3.1, §3.2 and §5 (the test-only dump): the private payload names a
/// Timoshenko frame's shear-area leaf (`formation_inputs.shear_area_bits`,
/// the evidence's A_s bit for bit) and only there, and a member's
/// `reduced_recovery_section` (A_r, Z_r, J_r, r_o) exactly where its stress
/// sections differ. An Euler-Bernoulli, zero-allowance source carries
/// neither. Each dumped text hashes to the receipt's commitment.
#[test]
fn o4_source_payload_members_are_conditional() {
    let mut cases = controls();
    cases.push(("euler-bernoulli-n05", fixture_request("physics_source", "n05")));
    cases.push(("euler-bernoulli-fields", fields()));
    for (id, request) in cases {
        for mode in MODES {
            let at = format!("{id} {}", mode.as_str());
            let (envelope, records) = with_source_dump(|| run(&request, mode));
            assert_eq!(records.len(), 1, "{at}");
            let source = &selected_case(&envelope)["source"];
            for (text, hash_key) in [("normalized_source_text", "normalized_source_sha256"), ("functional_plan_text", "functional_plan_sha256")] {
                let digest = format!("{:x}", Sha256::digest(records[0][text].as_str().unwrap().as_bytes()));
                assert_eq!(records[0][hash_key], json!(digest), "{at}: {text}");
                assert_eq!(source[hash_key], json!(digest), "{at}: the receipt's {hash_key}");
            }
            let payload = dumped(&records, 0, "normalized_source_text");
            let frames = payload["frames"].as_array().unwrap();
            assert_eq!(frames.len(), request["model"]["pipe_segments"].as_array().unwrap().len(), "{at}");
            for frame in frames {
                let s = section(&envelope, frame["element_id"].as_str().unwrap());
                let theory = &s["beam_theory"];
                match theory["model"].as_str() {
                    Some(TIMOSHENKO) => assert_eq!(frame["formation_inputs"]["shear_area_bits"], json!(bits(f(&theory["shear_area_m2"]))), "{at}"),
                    _ => assert!(frame["formation_inputs"].get("shear_area_bits").is_none(), "{at}"),
                }
                if distinct(s) {
                    let r = &s["reduced_section"];
                    assert_eq!(
                        frame["reduced_recovery_section"],
                        json!({"area_bits": bits(f(&r["As_m2"])), "section_modulus_bits": bits(f(&r["Z_m3"])),
                            "torsion_constant_bits": bits(f(&r["J_m4"])), "torsion_radius_bits": bits(f(&s["ro_m"]))}),
                        "{at}"
                    );
                } else {
                    assert!(frame.get("reduced_recovery_section").is_none(), "{at}");
                }
                assert_eq!(frame["recovery_section"].as_object().unwrap().len(), 2, "{at}: the nominal object keeps its keys");
            }
        }
    }
}

/// O4 §3.2, §3.5 and §6 (MJ6/MJ9's twelve requests): every committed
/// Euler-Bernoulli, zero-allowance source control keeps its numbers, work and
/// bytes: in both modes the producer's exact serialization equals the
/// committed raw fixture, and its private payload carries no O4 member.
#[test]
fn o4_euler_bernoulli_sources_keep_their_numbers() {
    let mut covered = 0;
    for (family, names) in FAMILIES {
        for name in names {
            let request = fixture_request(family, name);
            for mode in MODES {
                let at = format!("{family}/{name} {}", mode.as_str());
                let (bytes, records) = with_source_dump(|| {
                    serde_json::to_string(&crate::run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap()).unwrap()
                });
                let committed = std::fs::read_to_string(project_root().join(format!(
                    "fixtures/product_preview/pressure_source/{family}/{name}-{}.raw.json",
                    mode.as_str()
                )))
                .unwrap();
                assert!(bytes == committed, "{at}: producer bytes (numbers, work and text) equal the committed raw");
                assert!(!records.is_empty(), "{at}");
                for index in 0..records.len() {
                    for frame in dumped(&records, index, "normalized_source_text")["frames"].as_array().unwrap() {
                        assert!(frame["formation_inputs"].get("shear_area_bits").is_none() && frame.get("reduced_recovery_section").is_none(), "{at}");
                    }
                }
                covered += 1;
            }
        }
    }
    assert_eq!(covered, 24);
}

/// One captured run: the envelope, its dump and the work trace.
struct Captured {
    request: Value,
    envelope: Value,
    records: Vec<Value>,
    trace: Vec<Value>,
}
fn capture(request: &Value, mode: PreviewSolverMode) -> Captured {
    composite::start_trace();
    let (envelope, records) = with_source_dump(|| run(request, mode));
    let trace = composite::take_trace();
    assert_eq!(envelope["producer"]["semantic_contract_id"], json!(ps::PRESSURE_CONTRACT_ID), "{}", envelope["diagnostics"]);
    assert_eq!(records.len(), 1);
    Captured { request: request.clone(), envelope, records, trace }
}
impl Captured {
    fn charged_at(&self, stage: &str) -> i128 {
        self.trace.iter().find(|t| t["stage"] == stage).unwrap_or_else(|| panic!("{stage}"))["charged"].as_u64().unwrap().into()
    }
    fn case_work(&self) -> i128 {
        selected_case(&self.envelope)["work"]["charged"].as_u64().unwrap().into()
    }
    fn payload(&self) -> Value {
        dumped(&self.records, 0, "normalized_source_text")
    }
    /// The serialized source identity's length (`source_recovery.rs` `Identity::finish`).
    fn identity(&self) -> i128 {
        self.payload()["source_identity"].as_str().unwrap().len() as i128
    }
    /// E: the checked canonical encoded request.
    fn encoded(&self) -> i128 {
        checked(&self.request).unwrap().len() as i128
    }
    /// The descriptor terms of `finalization_size`, from the payload's lowered
    /// operands (case id excluded: it is the twins' common one).
    fn descriptors(&self) -> i128 {
        let mut d = 0i128;
        for operand in self.payload()["lowered_recovery_operands"].as_array().unwrap() {
            for product in operand["offset_bits"].as_array().unwrap() {
                d += 48 * product.as_array().unwrap().len() as i128;
            }
            for term in operand["terms"].as_array().unwrap() {
                d += 48;
                for product in term["product_bits"].as_array().unwrap() {
                    d += 48 * product.as_array().unwrap().len() as i128;
                }
            }
        }
        d
    }
    /// The selected case's rows' identity and metadata lengths.
    fn row_lengths(&self) -> Vec<Vec<usize>> {
        let case = selected_case(&self.envelope)["basis_ref"]["ref_id"].clone();
        self.envelope["results"].as_array().unwrap().iter().filter(|r| r["basis_ref"]["ref_id"] == case).map(|r| {
            let mut lengths: Vec<usize> = ["id", "kind", "entity_ref", "unit"].iter().map(|k| r[*k].as_str().unwrap().len()).collect();
            if let Some(m) = r["metadata"].as_object() {
                lengths.extend(["component", "coordinate_system", "location", "basis", "sign_convention"].iter().map(|k| m[*k].as_str().unwrap().len()));
            }
            lengths
        }).collect()
    }
    /// The members whose stress sections differ.
    fn distinct_members(&self) -> i128 {
        exact_case(&self.envelope)["pipe_sections"].as_array().unwrap().iter().filter(|s| distinct(s)).count() as i128
    }
    fn body(&self) -> &Value {
        &self.envelope["source_block_recovery"]["body"]
    }
    fn publication(&self) -> i128 {
        self.body()["invocation_work"]["publication_charged"].as_u64().unwrap().into()
    }
    fn diagnostic_bytes(&self) -> i128 {
        let utf8 = |v: &Value| v.as_str().unwrap().len() as i128;
        self.envelope["diagnostics"].as_array().unwrap().iter().map(|d| {
            utf8(&d["id"]) + utf8(&d["code"]) + utf8(&d["message"]) + d["affected_refs"].as_array().into_iter().flatten().map(utf8).sum::<i128>()
        }).sum()
    }
    /// P, the publication reservation's other inputs, enumerated from the
    /// actual request and publication (`pressure_source_join/consequences.rs`
    /// `publication_other_inputs`).
    fn publication_other_inputs(&self) -> i128 {
        let count = |v: &Value| v.as_array().unwrap().len() as i128;
        let model = &self.request["model"];
        let rows = count(&self.envelope["results"]);
        1024 + 32 * rows + 32 * count(&self.envelope["diagnostics"]) + 12 * self.diagnostic_bytes()
            + count(&model["load_cases"]) * (32000 * count(&model["pipe_segments"]) + 64 * count(&model["nodes"]).pow(2) + 256 * rows)
    }
}
/// Decimal digits of a u64 bit pattern, as `serde_json` writes it.
fn digits(bits: u64) -> i128 {
    bits.to_string().len() as i128
}
/// The serialized identity delta predicted from the twins' authored and
/// declared operands, independently of the identity string: each present
/// mill tolerance's value bits (the allowance is not an identity operand),
/// and each declared local-matrix entry's bits.
fn predicted_identity_delta(a: &Captured, b: &Captured) -> i128 {
    let mut delta = 0;
    let (pa, pb) = (&a.request["model"]["pipe_segments"], &b.request["model"]["pipe_segments"]);
    for (x, y) in pa.as_array().unwrap().iter().zip(pb.as_array().unwrap()) {
        match (x["section"].get("mill_tolerance"), y["section"].get("mill_tolerance")) {
            (Some(x), Some(y)) => {
                assert_eq!((x["unit"].as_str(), y["unit"].as_str()), (Some("m"), Some("m")));
                delta += digits(f(&y["value"]).to_bits()) - digits(f(&x["value"]).to_bits());
            }
            (None, None) => {}
            _ => panic!("the twins author the same mill-tolerance presence"),
        }
    }
    let (fa, fb) = (a.payload()["frames"].clone(), b.payload()["frames"].clone());
    let parse = |v: &Value| u64::from_str_radix(v.as_str().unwrap(), 16).unwrap();
    for (x, y) in fa.as_array().unwrap().iter().zip(fb.as_array().unwrap()) {
        for (rx, ry) in x["local_matrix_bits"].as_array().unwrap().iter().zip(y["local_matrix_bits"].as_array().unwrap()) {
            for (ex, ey) in rx.as_array().unwrap().iter().zip(ry.as_array().unwrap()) {
                delta += digits(parse(ey)) - digits(parse(ex));
            }
        }
    }
    delta
}

/// O4 §3.5 (the D5/MJ9 law; no charging rule changes), as measured here: for
/// a document newly admitted under O4 against its Euler-Bernoulli,
/// zero-allowance twin, every term is derived from the actual inputs and the
/// actual charging sites, and the sum reconciles with the case, publication
/// and invocation counters exactly:
/// - ΔI, the serialized source identity's length delta, is predicted from the
///   operands and equals the actual. The identity changes only through a
///   present mill tolerance's value bits or a Timoshenko frame's local-matrix
///   bits; the corrosion allowance is not an identity operand, so an
///   allowance-only change has ΔI = 0.
/// - Method work moves by 5·ΔI wherever the identity changes, from the exact
///   Context's `source_preflight` identity charge at each call: 2·ΔI in the
///   live attempt (`Context::prepare`, `retained_charge`), 2·ΔI in the
///   captured replay (the same two) and 1·ΔI at `check_binding_with_budget`.
///   With unchanged declared matrices (the Euler-Bernoulli twins) that is the
///   whole method delta (asserted); a Timoshenko matrix also moves the metered
///   method work, which is measured, not predicted (its binding-check delta is
///   asserted to be ΔI where the descriptor terms are unchanged).
/// - The publication loop adds the first identity digest's 12·ΔI and, for
///   each extra reduced-basis endpoint-maximum call, RECIPE_WORK + 128; the
///   composite pass adds the same call again: 2·4,224 = 8,448 per member whose
///   stress sections differ, none where they are bit-equal (values that round
///   away included).
/// - Finalization: ΔF_k = 2·(12·ΔI + 12·ΔE + ΔD), including the encoded
///   request's charges (ΔE, its checked canonical byte delta) and the
///   descriptor terms ΔD; the published rows' lengths are unchanged.
/// - Publication: 24·ΔE + ΔP. No published row or diagnostic is added; P moves
///   only through existing diagnostics' text bytes, ΔP = 12·Δ(diagnostic
///   bytes), asserted exactly. ΔP = 0 for the Euler-Bernoulli twins
///   (asserted); a Timoshenko twin can have ΔP ≠ 0, because the ordinary
///   report of its changed matrix and the selected summary state other numbers.
///
/// Existing Euler-Bernoulli, zero-allowance documents are unchanged: their
/// publications, work counters included, stay byte-identical
/// (`o4_euler_bernoulli_sources_keep_their_numbers`).
#[test]
fn o4_source_work_deltas_follow_the_d5_law() {
    let zero_fields = with_section(fields(), 0, Some(0.0), 0.0);
    let zero_two = with_section(two_member(fields()), 0, Some(0.0), 0.0);
    let t = 0.01f64;
    assert_eq!(fields()["model"]["pipe_segments"][0]["section"]["wall_thickness"], json!({"unit": "m", "value": t}));
    let pairs: [(&str, Value, Value, bool, i128); 6] = [
        ("dual basis", zero_fields.clone(), with_section(fields(), 0, Some(0.0005), 0.001), false, 1),
        ("allowance that rounds away", zero_fields.clone(), with_section(fields(), 0, Some(0.0), 1e-20), false, 0),
        ("mill tolerance that rounds away", zero_fields.clone(), with_section(fields(), 0, Some(t * 2f64.powi(-80)), 0.0), false, 0),
        ("mixed two members", zero_two, with_section(two_member(fields()), 0, Some(0.0005), 0.001), false, 1),
        ("timoshenko", fixture_request("physics_source", "n05"), control("timoshenko-n05"), true, 0),
        ("timoshenko dual basis", zero_fields, with_shear_model(with_section(fields(), 0, Some(0.0005), 0.001), TIMOSHENKO), true, 1),
    ];
    for (label, twin, admitted, timoshenko, n) in pairs {
        for mode in MODES {
            let at = format!("{label} {}", mode.as_str());
            let (a, b) = (capture(&twin, mode), capture(&admitted, mode));
            let delta = |from: &str, to: &str| (b.charged_at(to) - b.charged_at(from)) - (a.charged_at(to) - a.charged_at(from));
            // Method work, live and replayed.
            let live = b.charged_at("derived_recipe_entry") - a.charged_at("derived_recipe_entry");
            let replay = delta("source_case_finalization_reserved", "captured_source_replay_complete");
            let binding = delta("captured_source_replay_complete", "current_source_binding_complete");
            // ΔI from the operands, equal to the actual serialized identity.
            let di = predicted_identity_delta(&a, &b);
            assert_eq!(di, b.identity() - a.identity(), "{at}: identity delta");
            let de = b.encoded() - a.encoded();
            let dd = b.descriptors() - a.descriptors();
            // The exact Context's `source_preflight` charges the serialized
            // identity's length at each of its calls: `Context::prepare` and
            // `retained_charge` in the live attempt and again in
            // `replay_against`, and once in `check_binding_with_budget`. With
            // unchanged declared matrices that is the whole method delta; a
            // Timoshenko matrix moves the metered method work besides
            // (measured), while the binding check reads no matrix value.
            if !timoshenko {
                assert_eq!((live, replay, binding), (2 * di, 2 * di, di), "{at}: unchanged declared matrices: the identity charges only");
            } else if dd == 0 {
                assert_eq!(binding, di, "{at}: the binding check's identity charge");
            }
            assert_eq!(a.row_lengths(), b.row_lengths(), "{at}: published rows and their metadata lengths are unchanged");
            assert_eq!((a.distinct_members(), b.distinct_members()), (0, n), "{at}: members whose sections differ");
            // Each site.
            assert_eq!(delta("derived_recipe_entry", "source_case_finalization_entry"), 12 * di + MAXIMUM_CALL * n, "{at}: publication loop");
            let df = 2 * (12 * di + 12 * de + dd);
            assert_eq!(delta("source_case_finalization_entry", "source_case_finalization_reserved"), df, "{at}: ΔF_k");
            assert_eq!(delta("current_source_binding_complete", "source_case_finalization_complete"), MAXIMUM_CALL * n, "{at}: composite pass");
            for c in [&a, &b] {
                assert_eq!(c.case_work(), c.charged_at("source_case_finalization_complete"), "{at}: the case counter");
            }
            // The reconciliation of the whole case counter.
            assert_eq!(b.case_work() - a.case_work(), live + replay + binding + 12 * di + df + 2 * MAXIMUM_CALL * n, "{at}: Δ case work");
            // Publication: 24E + P, P from its actual inputs. No published row
            // or diagnostic is added: P moves only by the existing diagnostics'
            // text where it states measured numbers (the selected summary's
            // work and bounds, the ordinary report of a changed matrix), which
            // P counts as before; that is zero for the Euler-Bernoulli twins.
            for c in [&a, &b] {
                assert_eq!(c.publication(), 24 * c.encoded() + c.publication_other_inputs(), "{at}: 24E + P");
            }
            let diagnostics = |c: &Captured| c.envelope["diagnostics"].as_array().unwrap().iter().map(|d| (d["id"].clone(), d["code"].clone())).collect::<Vec<_>>();
            assert_eq!(diagnostics(&a), diagnostics(&b), "{at}: no diagnostic added");
            assert_eq!(a.envelope["results"].as_array().unwrap().len(), b.envelope["results"].as_array().unwrap().len(), "{at}: no row added");
            let dp = b.publication_other_inputs() - a.publication_other_inputs();
            for (x, y) in a.envelope["diagnostics"].as_array().unwrap().iter().zip(b.envelope["diagnostics"].as_array().unwrap()) {
                let measured = x["code"] == "SOURCE_BLOCK_RECOVERY_SELECTED" || x["code"].as_str().unwrap().starts_with("NUMERICAL_INTEGRITY_");
                assert!(x == y || measured, "{at}: only measured-number text moves: {}", x["code"]);
            }
            assert_eq!(dp, 12 * (b.diagnostic_bytes() - a.diagnostic_bytes()), "{at}: ΔP is the diagnostics' text only");
            if !timoshenko {
                assert_eq!(dp, 0, "{at}: ΔP = 0");
            }
            assert_eq!(b.publication() - a.publication(), 24 * de + dp, "{at}: Δ publication");
            for c in [&a, &b] {
                let cases: i128 = c.body()["cases"].as_array().unwrap().iter()
                    .map(|k| i128::from(k["work"]["charged"].as_u64().unwrap() + k["work"]["reserved_unobserved_failure"].as_u64().unwrap()))
                    .sum();
                assert_eq!(i128::from(c.body()["invocation_work"]["charged"].as_u64().unwrap()), cases + c.publication(), "{at}");
                assert!(c.body()["invocation_work"]["charged"].as_u64().unwrap() <= 64_000_000, "{at}");
                assert_eq!(selected_case(&c.envelope)["work"]["limit"], json!(8_000_000), "{at}");
            }
            if label.contains("rounds away") {
                // Bit-equal sections keep the twin's payload but for the
                // invocation digest and identity, every row value and check.
                let strip = |c: &Captured| {
                    let mut p = c.payload();
                    p.as_object_mut().unwrap().remove("invocation_sha256");
                    p.as_object_mut().unwrap().remove("source_identity");
                    p
                };
                assert_eq!(strip(&a), strip(&b), "{at}: payload");
                let values = |c: &Captured| c.envelope["results"].as_array().unwrap().iter().map(|r| (r["id"].clone(), r["value"].clone())).collect::<Vec<_>>();
                assert_eq!(values(&a), values(&b), "{at}: values");
                assert_eq!(selected_case(&a.envelope)["section_stress_checks"], selected_case(&b.envelope)["section_stress_checks"], "{at}: checks");
            }
        }
    }
}

/// O4 §3.4 and §6 (`shear_area_changed`, `kappa_changed`): the reader binds
/// each section's beam theory to its rederivation. On the Timoshenko control
/// a one-ulp A_s or κ_E (resealed) is refused by the transport rederivation,
/// and a dropped Timoshenko leaf by the contract's selection; on the
/// Euler-Bernoulli n05 control a correct Timoshenko statement is refused by
/// the contract's selection.
#[test]
fn o4_source_shear_area_and_kappa_mutations_are_refused() {
    let request = control("timoshenko-n05");
    for mode in MODES {
        let envelope = run(&request, mode);
        assert_eq!(verdict(&envelope, &request, mode), eligible());
        for key in ["shear_area_m2", "shear_factor"] {
            let mut changed = envelope.clone();
            let theory = &mut changed["contract_evidence"]["exact_cases"][0]["pipe_sections"][0]["beam_theory"];
            theory[key] = json!(f(&theory[key]).next_up());
            reseal(&mut changed);
            assert_eq!(refused(&changed, &request, mode), "PHYSICS_SOURCE_SECTION_BASES", "{key}");
            assert_eq!(ps::validate_transport_metadata_in(&changed, ps::SourceContract::PressureV3), Err("PHYSICS_SOURCE_SECTION_BASES".into()), "{key}");
        }
        let mut dropped = envelope.clone();
        dropped["contract_evidence"]["exact_cases"][0]["pipe_sections"][0]["beam_theory"] = json!({"model": "euler_bernoulli"});
        reseal(&mut dropped);
        assert_eq!(ps::validate_transport_metadata_in(&dropped, ps::SourceContract::PressureV3), Ok(()));
        assert_eq!(refused(&dropped, &request, mode), "PHYSICS_SOURCE_ACTUAL_SHEAR_MODEL");
    }
    let request = fixture_request("physics_source", "n05");
    let mode = PreviewSolverMode::SparseInteractive;
    let mut stated = run(&request, mode);
    let exact = &stated["contract_evidence"]["exact_cases"][0];
    let nu = f(&exact["pipe_materials"][0]["nu"]);
    let s = &exact["pipe_sections"][0];
    let kappa = open_pipe_stress_frame_kernel::shear::annulus_energy_shear_factor(f(&s["nominal_section"]["ri_m"]), f(&s["ro_m"]), nu).unwrap();
    let area = kappa * f(&s["nominal_section"]["As_m2"]);
    stated["contract_evidence"]["exact_cases"][0]["pipe_sections"][0]["beam_theory"] = json!({"model": TIMOSHENKO, "shear_factor": kappa, "shear_area_m2": area});
    reseal(&mut stated);
    assert_eq!(ps::validate_transport_metadata_in(&stated, ps::SourceContract::PressureV3), Ok(()), "a consistent leaf passes the rederivation");
    assert_eq!(refused(&stated, &request, mode), "PHYSICS_SOURCE_ACTUAL_SHEAR_MODEL");
}

/// O4 §6 (MJ9's both-bases tamper, revised): on a member whose sections
/// differ each basis is judged on its own, so a one-ulp τ_v,max on one basis
/// alone (resealed) is refused by that basis's own shear check, the twin rule
/// not standing in; on the bit-equal member today's both-bases tamper keeps
/// its outcome, and a one-basis tamper there is refused by the coincident
/// twin rule.
#[test]
fn o4_source_one_basis_tamper_on_a_distinct_basis_member() {
    const SHEAR_MAX: &str = "element_local_transverse_shear_stress_maximum";
    let request = control("mixed-two-member");
    let mode = PreviewSolverMode::SparseInteractive;
    let envelope = run(&request, mode);
    let (distinct_pipe, equal_pipe) = ("independent-member", "independent-member-2");
    assert!(distinct(section(&envelope, distinct_pipe)) && !distinct(section(&envelope, equal_pipe)));
    let tamper = |pipe: &str, bases: &[&str]| {
        let mut doc = envelope.clone();
        let mut changed = 0;
        for row in doc["results"].as_array_mut().unwrap() {
            if row["entity_ref"] == pipe
                && row["kind"] == SHEAR_MAX
                && row["metadata"]["location"] == "end_i"
                && bases.iter().any(|b| row["metadata"]["basis"].as_str().unwrap().ends_with(&format!("; section_basis={b}")))
            {
                row["value"] = json!(f(&row["value"]).next_up());
                changed += 1;
            }
        }
        assert_eq!(changed, bases.len());
        reseal(&mut doc);
        doc
    };
    for basis in ["nominal_section", "reduced_section"] {
        assert_eq!(refused(&tamper(distinct_pipe, &[basis]), &request, mode), "SOURCE_PHYSICS_SHEAR_VALUE", "{basis} alone");
    }
    assert_eq!(refused(&tamper(equal_pipe, &["nominal_section", "reduced_section"]), &request, mode), "SOURCE_PHYSICS_SHEAR_VALUE");
    assert_eq!(refused(&tamper(equal_pipe, &["reduced_section"]), &request, mode), "SOURCE_PHYSICS_ROW_BASIS_SECTION");
}

/// The O4 controls as a test-only fixture for the independent Python
/// verifier (`tests/test_o4_source_verifier.py`): each control in sparse
/// mode, and the Euler-Bernoulli dual-basis control in dense mode too, with
/// its captured invocation. The committed file equals the producer's current
/// output; `SWBPIPE_O4_SOURCE_CONTROLS_WRITE=1` rewrites it.
#[test]
fn o4_source_controls_are_current() {
    let mut entries = Vec::new();
    for (id, request) in controls() {
        let mut modes = vec![PreviewSolverMode::SparseInteractive];
        if id == "dual-basis-fields" {
            modes.push(PreviewSolverMode::DenseScrutiny);
        }
        for mode in modes {
            let source = run(&request, mode);
            assert_eq!(verdict(&source, &request, mode), eligible(), "{id}");
            entries.push(json!({"id": id, "solver_mode": mode.as_str(), "invocation": invocation(&request, mode), "source": source}));
        }
    }
    let document = json!({"artifact": "o4-source-controls", "schema_version": "1.0.0",
        "provenance": "invented in-memory re-authorings of the committed pressure-source n05 and fields requests (O4 design section 6); test-only",
        "entries": entries});
    let path = project_root().join(CONTROLS_PATH);
    if std::env::var("SWBPIPE_O4_SOURCE_CONTROLS_WRITE").as_deref() == Ok("1") {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, format!("{}\n", serde_json::to_string(&document).unwrap())).unwrap();
    }
    let committed: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    assert!(committed == document, "{CONTROLS_PATH} is current (SWBPIPE_O4_SOURCE_CONTROLS_WRITE=1 rewrites it)");
}

/// O4 §5: the test-only declared-problem dump (`source.rs`
/// `with_source_dump`, whose doc comment states the format). Always: the
/// records of the Timoshenko dual-basis control hash to its receipt's
/// commitments. With `SWBPIPE_O4_SOURCE_DUMP_REQUEST` (a request JSON file)
/// and `SWBPIPE_O4_SOURCE_DUMP_OUT`, the records of that request are written
/// to the named file (`SWBPIPE_O4_SOURCE_DUMP_MODE` selects the mode).
#[test]
fn o4_source_payload_dump() {
    let write = |request: &Value, mode: PreviewSolverMode| -> Value {
        let (envelope, records) = with_source_dump(|| run(request, mode));
        for record in &records {
            for (text, hash_key) in [("normalized_source_text", "normalized_source_sha256"), ("functional_plan_text", "functional_plan_sha256")] {
                assert_eq!(record[hash_key], json!(format!("{:x}", Sha256::digest(record[text].as_str().unwrap().as_bytes()))));
            }
            let case = envelope["source_block_recovery"]["body"]["cases"].as_array().unwrap().iter()
                .find(|c| c["basis_ref"]["ref_id"] == record["case_id"]).unwrap();
            assert_eq!(case["source"]["normalized_source_sha256"], record["normalized_source_sha256"]);
            assert_eq!(case["source"]["functional_plan_sha256"], record["functional_plan_sha256"]);
        }
        json!({"artifact": "o4-source-declared-problem-dump", "schema_version": "1.0.0", "solver_mode": mode.as_str(),
            "publication_sha256": envelope["source_block_recovery"]["body"]["publication_sha256"], "records": records})
    };
    let dump = write(&control("timoshenko-dual-basis-fields"), PreviewSolverMode::SparseInteractive);
    assert_eq!(dump["records"].as_array().unwrap().len(), 1);
    if let (Some(request), Some(out)) = (std::env::var_os("SWBPIPE_O4_SOURCE_DUMP_REQUEST"), std::env::var_os("SWBPIPE_O4_SOURCE_DUMP_OUT")) {
        let request: Value = serde_json::from_str(&std::fs::read_to_string(request).unwrap()).unwrap();
        let mode = match std::env::var("SWBPIPE_O4_SOURCE_DUMP_MODE").as_deref() {
            Ok("dense_scrutiny") => PreviewSolverMode::DenseScrutiny,
            Ok("sparse_interactive") | Err(_) => PreviewSolverMode::SparseInteractive,
            Ok(other) => panic!("unknown SWBPIPE_O4_SOURCE_DUMP_MODE {other}"),
        };
        std::fs::write(out, format!("{}\n", serde_json::to_string(&write(&request, mode)).unwrap())).unwrap();
    }
}

/// R-A's gating test on the committed O4 source controls (DEL-04-08 Design
/// `source-work-nondyadic-inputs.md` §3.2): in both modes, every row equals the long
/// evaluation's bit for bit.
#[test]
fn o4_source_controls_negated_rows_equal_the_long_evaluation() {
    use crate::source_recovery::j7_rf_census::ra_gating::gate;
    let mut gated = 0;
    for (id, request) in controls() {
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let Some(gate) = gate(&request, mode) else { continue };
            let gate = gate.unwrap_or_else(|e| panic!("{id} {}: {e}", mode.as_str()));
            assert!(gate.mismatches.is_empty(), "{id} {}: rows {:?} differ", mode.as_str(), gate.mismatches);
            gated += 1;
        }
    }
    // Exactly the 8 control evaluations that reach a plan, so none can drop out of the
    // gate unnoticed (Opus review, round 2).
    assert_eq!(gated, 8, "O4 controls gated");
}

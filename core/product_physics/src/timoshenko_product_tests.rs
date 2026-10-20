//! T4-U8 (DEL-05-03 Design `section-bases-and-timoshenko.md` §3, §4 item 2,
//! §5, §8): the product's Timoshenko threading (which section, which nu, which
//! members, which binding). `timoshenko_shear_area_uses_the_stiffness_section`
//! compares with the frozen t4_u6 Timoshenko variant; the other checks here
//! are of the threading itself, and the t4_u8 freeze is consumed by
//! `tests/t4_u8_timoshenko_references.rs`.
use super::*;
use open_pipe_stress_frame_kernel::shear::{annulus_energy_shear_factor, shear_area};
use open_pipe_stress_frame_kernel::BeamTheory;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const N05: &str = include_str!("../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json");
const U6_ORACLE: &str = include_str!("../../../validation/references/t4_u6/oracle.json");
const U6_TIMOSHENKO_SELF_WEIGHT: &str =
    include_str!("../../../validation/references/t4_u6/requests/U6-BASES-001-TIMO-SELF-WEIGHT.request.json");
const EIGEN_MOTION: &str =
    include_str!("../../../fixtures/product_preview/pressure_source/load_reference_source/eigen_motion.request.json");
const I7_DOCUMENTS: &str = include_str!("../../../validation/references/t4_i7/u2_document_sketches.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const TIMOSHENKO: &str = "timoshenko_annulus_energy_v1";
const INVENTED: &str = "invented_t4_u8_product_threading_test_not_library_data";

fn with_shear_model(mut request: Value, shear_model: &str) -> Value {
    request["model"]["pressure_contract"]["shear_model"] = json!(shear_model);
    request
}
fn run(request: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap()).unwrap()
}
fn number(v: &Value) -> f64 {
    v.as_f64().unwrap_or_else(|| panic!("a number: {v}"))
}
/// The product's own preparation and build of a request's model (base E/nu).
fn built(request: &Value) -> (PreviewModel, BuiltModel) {
    let request: LinearStaticPreviewRequest = serde_json::from_value(request.clone()).unwrap();
    let mut model = request.model;
    let mut materials = if request.materials.is_empty() { model.materials.clone() } else { request.materials };
    let mut diagnostics = Vec::new();
    pressure_runtime::validate_profile(&model, &mut diagnostics);
    resolve_shared_sections(&mut model, &mut diagnostics);
    normalize_model_units(&mut model, &mut materials, &mut diagnostics);
    pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
    let built = build_model(&model, &materials, &mut diagnostics).expect("the model builds");
    assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
    model.materials = materials;
    (model, built)
}
/// kappa_E and A_s of an evidence section record, checked bit for bit
/// against FK's functions of its nominal leaves and `nu`.
fn assert_timoshenko_evidence(section: &Value, nu: f64, label: &str) -> (f64, f64) {
    let nominal = &section["nominal_section"];
    let kappa = annulus_energy_shear_factor(number(&nominal["ri_m"]), number(&section["ro_m"]), nu).unwrap();
    let shear = shear_area(kappa, number(&nominal["As_m2"])).unwrap();
    let theory = &section["beam_theory"];
    assert_eq!(theory["model"], json!(TIMOSHENKO), "{label}: {theory}");
    assert_eq!(theory.as_object().unwrap().len(), 3, "{label}: {theory}");
    assert_eq!(number(&theory["shear_factor"]).to_bits(), kappa.to_bits(), "{label}: kappa_E");
    assert_eq!(number(&theory["shear_area_m2"]).to_bits(), shear.to_bits(), "{label}: A_s");
    (kappa, shear)
}

/// The published rows of `case` with a frozen row's identity and `basis`.
fn published_rows<'a>(envelope: &'a Value, row: &Value, case: &str, basis: &str) -> Vec<&'a Value> {
    envelope["results"].as_array().unwrap().iter().filter(|r| {
        r["kind"] == row["kind"] && r["entity_ref"] == row["entity_ref"]
            && r["metadata"]["component"] == row["component"] && r["metadata"]["location"] == row["location"]
            && r["basis_ref"]["ref_id"] == json!(case)
            && r["metadata"]["basis"].as_str().and_then(|b| b.rsplit_once("; section_basis=")).map_or("", |(_, b)| b) == basis
    }).collect()
}

/// Design §3.1, §8 ("kappa or A_s formed on the reduced section"), against
/// the frozen t4_u6 Timoshenko variant (c_m 0.00125 m, c_a 0.002 m; its
/// SHA-bound self-weight request, read unmodified): kappa_E and A_s come from
/// the nominal (stiffness) annulus, never the reduced section. The built
/// member's leaf and the published `beam_theory` carry kappa_E bit-equal to
/// the frozen binary64 exhibit of Design §3.1's order and A_s at the frozen
/// exact value; A, I and J are the Euler-Bernoulli twin's nominal section;
/// and no row listed for `kappa_on_reduced_section` takes its wrong value.
#[test]
fn timoshenko_shear_area_uses_the_stiffness_section() {
    let oracle: Value = serde_json::from_str(U6_ORACLE).unwrap();
    let variant = &oracle["cases"]["U6-BASES-001"]["variants"][TIMOSHENKO];
    let binding = &variant["request_self_weight"];
    assert_eq!(format!("{:x}", Sha256::digest(U6_TIMOSHENKO_SELF_WEIGHT.as_bytes())), binding["sha256"].as_str().unwrap());
    // The frozen request, re-authored in memory to the current version (its bytes are bound above).
    let timoshenko = crate::b7b_admission_tests::reauthor_current_request(serde_json::from_str(U6_TIMOSHENKO_SELF_WEIGHT).unwrap());
    assert_eq!(timoshenko["model"]["pressure_contract"]["shear_model"], json!(TIMOSHENKO));
    let frozen = &variant["pipe_sections_evidence"]["beam_theory"];
    let exhibit = number(&variant["binary64_exhibits"]["shear_factor"]["value"]);
    let frozen_kappa = number(&frozen["shear_factor"]["value"]);
    let frozen_area = number(&frozen["shear_area_m2"]["value"]);
    assert!((exhibit - frozen_kappa).abs() <= 1e-9 * frozen_kappa);
    let euler_bernoulli = with_shear_model(timoshenko.clone(), "euler_bernoulli");
    let (_, t) = built(&timoshenko);
    let (_, eb) = built(&euler_bernoulli);
    assert!(!t.pipes.is_empty());
    for (tp, ep) in t.pipes.iter().zip(&eb.pipes) {
        let bases = t.exact_sections[&tp.element_id];
        let (nominal, reduced) = (bases.nominal(), bases.reduced());
        assert_ne!(nominal.wall_area_m2().to_bits(), reduced.wall_area_m2().to_bits(), "precondition: the bases differ");
        let (kappa, area) = bases.timoshenko().expect("a Timoshenko member's factor");
        assert_eq!(kappa.to_bits(), exhibit.to_bits(), "kappa_E in Design §3.1's binary64 order");
        assert!((area - frozen_area).abs() <= 1e-9 * frozen_area, "A_s {area:e} vs frozen {frozen_area:e}");
        assert_eq!(tp.section.beam_theory, BeamTheory::Timoshenko { shear_area: area });
        let frame = tp.frame_element().unwrap();
        assert_eq!(frame.section.beam_theory.shear_area().map(f64::to_bits), Some(area.to_bits()));
        // A, I and J are the nominal section's, identical to the Euler-Bernoulli twin's.
        assert_eq!(ep.section.beam_theory, BeamTheory::EulerBernoulli);
        for (x, y) in [(tp.section.area, ep.section.area), (tp.section.second_moment_y, ep.section.second_moment_y),
            (tp.section.torsion_constant, ep.section.torsion_constant), (tp.section.area, nominal.wall_area_m2())] {
            assert_eq!(x.to_bits(), y.to_bits());
        }
    }
    let cases: Vec<&str> = binding["load_cases"].as_array().unwrap().iter().map(|c| c.as_str().unwrap()).collect();
    for mode in MODES {
        let envelope = run(&timoshenko, mode);
        assert_eq!(envelope["status"]["mechanics"], json!("MECHANICS_SOLVED"), "{mode:?}: {}", envelope["diagnostics"]);
        for case in envelope["contract_evidence"]["exact_cases"].as_array().unwrap() {
            for section in case["pipe_sections"].as_array().unwrap() {
                let theory = &section["beam_theory"];
                assert_eq!(theory["model"], json!(TIMOSHENKO), "{mode:?}");
                assert_eq!(number(&theory["shear_factor"]).to_bits(), exhibit.to_bits(), "{mode:?}: kappa_E");
                let area = number(&theory["shear_area_m2"]);
                assert!((area - frozen_area).abs() <= 1e-9 * frozen_area, "{mode:?}: A_s {area:e}");
            }
        }
        let mut evaluated = 0;
        for discriminator in oracle["cases"]["U6-BASES-001"]["wrong_result_discriminators"].as_array().unwrap() {
            let load_case = discriminator["load_case"].as_str().unwrap();
            if discriminator["id"] != json!("kappa_on_reduced_section") || discriminator["variant"] != json!(TIMOSHENKO) || !cases.contains(&load_case) {
                continue;
            }
            let rows = variant["load_cases"][load_case]["rows"].as_array().unwrap();
            for listed in discriminator["listed_rows"].as_array().unwrap() {
                let row = rows.iter().find(|r| ["kind", "entity_ref", "component", "location", "section_basis"].iter().all(|k| r.get(*k) == listed.get(*k))).unwrap();
                let factor = match (row["reference_origin"]["reference_unit"].as_str().unwrap(), row["unit"].as_str().unwrap()) {
                    ("m", "mm") => 1000.0, ("Pa", "MPa") => 1e-6, (a, b) => { assert_eq!(a, b); 1.0 }
                };
                let expected = number(&oracle.pointer(row["reference_origin"]["pointer"].as_str().unwrap()).unwrap()["value"]) * factor;
                let wrong = number(&listed["wrong"]["value"]) * factor;
                let absolute = number(&row["criterion"]["absolute_tolerance"]);
                let tolerance = number(&row["criterion"]["relative_tolerance"]) * expected.abs() + absolute;
                let found = published_rows(&envelope, row, load_case, listed["section_basis"].as_str().unwrap_or(""));
                assert_eq!(found.len(), 1, "{mode:?} {load_case} {listed}");
                let observed = number(&found[0]["value"]);
                assert!((observed - expected).abs() <= tolerance, "{mode:?} {load_case} {}: {observed:e} vs frozen {expected:e}", row["kind"]);
                assert!((observed - wrong).abs() > absolute.max(1e-9 * observed.abs().max(wrong.abs())), "{mode:?} {load_case}: the reduced-section kappa value");
                evaluated += 1;
            }
        }
        assert!(evaluated > 0, "{mode:?}: kappa_on_reduced_section rows");
    }
}

/// Design §3.1 ("A missing nu or G basis blocks a Timoshenko solve by name; no
/// default factor exists"): a timoshenko_annulus_energy_v1 straight member
/// whose material carries G but no Poisson ratio is not built; the build blocks
/// with EXACT_PRESSURE_SHEAR_FACTOR_UNAVAILABLE naming the member and the
/// missing nu, while its Euler-Bernoulli twin builds from the same inputs. On
/// the public route the same document is refused earlier, by
/// EXACT_PRESSURE_POISSON_RATIO_REQUIRED, and nothing is solved.
#[test]
fn timoshenko_without_a_nu_basis_is_blocked_by_name() {
    let base: Value = serde_json::from_str(N05).unwrap();
    let timoshenko = with_shear_model(base.clone(), TIMOSHENKO);
    // The product's preparation (G derived from E/nu), then a material basis
    // without nu, as a direct build sees it.
    let build = |request: &Value| {
        let request: LinearStaticPreviewRequest = serde_json::from_value(request.clone()).unwrap();
        let mut model = request.model;
        let mut materials = if request.materials.is_empty() { model.materials.clone() } else { request.materials };
        let mut diagnostics = Vec::new();
        resolve_shared_sections(&mut model, &mut diagnostics);
        normalize_model_units(&mut model, &mut materials, &mut diagnostics);
        pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
        assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
        for material in &mut materials {
            assert!(material.shear_modulus.is_some(), "precondition: G is present");
            material.poisson_ratio = None;
        }
        let built = build_model(&model, &materials, &mut diagnostics);
        (built, diagnostics)
    };
    let (built, diagnostics) = build(&timoshenko);
    let pipes: Vec<String> = timoshenko["model"]["pipe_segments"].as_array().unwrap().iter()
        .map(|pipe| pipe["id"].as_str().unwrap().to_owned()).collect();
    assert!(!pipes.is_empty());
    for pipe in &pipes {
        let blocked = diagnostics.iter().find(|d| d.code == "EXACT_PRESSURE_SHEAR_FACTOR_UNAVAILABLE" && d.affected_refs == vec![pipe.clone()])
            .unwrap_or_else(|| panic!("{pipe}: blocked by name: {diagnostics:?}"));
        assert_eq!(blocked.severity, "blocking");
        assert!(blocked.message.contains("Poisson ratio") && blocked.message.contains("no default shear factor"), "{}", blocked.message);
    }
    assert!(built.map_or(true, |built| built.pipes.is_empty() && built.frame_elements.is_empty()), "no Timoshenko member is built without nu");
    let (built, diagnostics) = build(&base);
    assert!(!has_blocking(&diagnostics), "the Euler-Bernoulli twin needs no nu here: {diagnostics:?}");
    assert_eq!(built.expect("the Euler-Bernoulli twin builds").pipes.len(), pipes.len());

    let mut public = timoshenko.clone();
    for material in public["model"]["materials"].as_array_mut().unwrap() {
        material.as_object_mut().unwrap().remove("poisson_ratio");
    }
    for mode in MODES {
        let envelope = run(&public, mode);
        assert_ne!(envelope["status"]["mechanics"], json!("MECHANICS_SOLVED"), "{mode:?}");
        assert!(envelope["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == json!("EXACT_PRESSURE_POISSON_RATIO_REQUIRED")
            && d["severity"] == json!("blocking")), "{mode:?}: {}", envelope["diagnostics"]);
    }
}

/// Design §3.1, §8 ("Base-material nu instead of the resolved per-case nu"):
/// on a load-state document whose cases resolve different E/nu pairs, each
/// case's kappa_E and A_s are FK's functions of that case's own nu, and they
/// differ between the cases.
#[test]
fn timoshenko_load_state_uses_the_resolved_case_nu() {
    let mut request: Value = serde_json::from_str(EIGEN_MOTION).unwrap();
    let base_nu = number(&request["model"]["materials"][0]["poisson_ratio"]["value"]);
    let point_nu = 0.3;
    assert_ne!(base_nu, point_nu);
    // Case 0 selects an invented exact point (E 1.5e11 Pa, nu 0.3); a second
    // case, copied from it, selects the base pair explicitly.
    let mut second = request["model"]["load_cases"][0].clone();
    request["model"]["materials"][0]["temperature_points"] = json!([{
        "id": "point:u8-test", "temperature": {"value": 20, "unit": "degC"},
        "elastic_modulus": {"value": 1.5e11, "unit": "Pa"}, "poisson_ratio": {"value": point_nu, "unit": "1"},
        "provenance": INVENTED}]);
    request["model"]["load_cases"][0]["analysis_state"]["element_states"][0]["material_selection"] =
        json!({"kind": "exact_point", "material_ref": "material", "point_ref": "point:u8-test"});
    second["id"] = json!("case:second");
    second["primitive_loads"][0]["id"] = json!("torque:second");
    second["analysis_state"]["load_sources"][0]["source_ref"] = json!("torque:second");
    request["model"]["load_cases"].as_array_mut().unwrap().push(second);
    let request = with_shear_model(request, TIMOSHENKO);
    for mode in MODES {
        let envelope = run(&request, mode);
        assert_eq!(envelope["status"]["mechanics"], json!("MECHANICS_SOLVED"), "{mode:?}: {}", envelope["diagnostics"]);
        let cases = envelope["contract_evidence"]["exact_cases"].as_array().unwrap();
        let section = |id: &str| {
            let case = cases.iter().find(|c| c["load_case_id"] == json!(id)).unwrap_or_else(|| panic!("{mode:?}: {id}"));
            case["pipe_sections"][0].clone()
        };
        let (kappa_point, area_point) = assert_timoshenko_evidence(&section("case:join"), point_nu, "case:join");
        let (kappa_base, area_base) = assert_timoshenko_evidence(&section("case:second"), base_nu, "case:second");
        assert_ne!(kappa_point.to_bits(), kappa_base.to_bits(), "{mode:?}: each case its own nu");
        assert_ne!(area_point.to_bits(), area_base.to_bits(), "{mode:?}");
    }
}

/// The T4-I7 sketch re-authored to the current form (B7b), as a request.
fn sketch(id: &str) -> Value {
    let documents: Value = serde_json::from_str(I7_DOCUMENTS).unwrap();
    crate::b7b_admission_tests::reauthor_current_request(documents["sketches"][id]["0.3.0"]["document"].clone())
}
/// Every object of `value` holding `key`.
fn records_with<'a>(value: &'a Value, key: &str, out: &mut Vec<&'a Value>) {
    match value {
        Value::Object(map) => {
            if map.contains_key(key) {
                out.push(value);
            }
            map.values().for_each(|v| records_with(v, key, out));
        }
        Value::Array(items) => items.iter().for_each(|v| records_with(v, key, out)),
        _ => {}
    }
}

/// Design §3.6, §8 ("Shear deformation applied to a realized arc, or arc
/// evidence claiming Timoshenko"): on a mixed L-line under
/// timoshenko_annulus_energy_v1 every realized arc's element stiffness is the
/// Euler-Bernoulli document's, bit for bit, and its evidence names
/// euler_bernoulli; the straight members carry the Timoshenko leaf.
#[test]
fn timoshenko_realized_arcs_stay_euler_bernoulli() {
    let euler_bernoulli = sketch("U2-L-ANCH-PTW-K2");
    assert_eq!(euler_bernoulli["model"]["pressure_contract"]["shear_model"], json!("euler_bernoulli"));
    let timoshenko = with_shear_model(euler_bernoulli.clone(), TIMOSHENKO);
    let (_, t) = built(&timoshenko);
    let (_, eb) = built(&euler_bernoulli);
    assert!(!t.curved_bend_elements.is_empty() && !t.frame_elements.is_empty(), "precondition: a mixed L-line");
    assert_eq!(t.curved_bend_elements.len(), eb.curved_bend_elements.len());
    for (x, y) in t.curved_bend_elements.iter().zip(&eb.curved_bend_elements) {
        assert_eq!(x.pipe_id, y.pipe_id);
        let (kx, ky) = (x.macro_element.global_stiffness().unwrap(), y.macro_element.global_stiffness().unwrap());
        for (rx, ry) in kx.iter().zip(&ky) {
            assert_eq!(rx.map(f64::to_bits), ry.map(f64::to_bits), "{}: arc stiffness row", x.pipe_id);
        }
        assert_eq!(x.global_stiffness.map(|r| r.map(f64::to_bits)), y.global_stiffness.map(|r| r.map(f64::to_bits)));
        assert!(t.exact_sections[&x.pipe_id].timoshenko().is_none(), "{}: an arc has no shear leaf", x.pipe_id);
    }
    assert!(t.frame_elements.iter().all(|f| f.section.beam_theory.is_timoshenko()));
    assert!(eb.frame_elements.iter().all(|f| !f.section.beam_theory.is_timoshenko()));
    let arc_ids: Vec<&str> = t.curved_bend_elements.iter().map(|b| b.pipe_id.as_str()).collect();
    for mode in MODES {
        let envelope = run(&timoshenko, mode);
        assert_eq!(envelope["status"]["mechanics"], json!("MECHANICS_SOLVED"), "{mode:?}: {}", envelope["diagnostics"]);
        let mut sections = Vec::new();
        records_with(&envelope["contract_evidence"], "beam_theory", &mut sections);
        let mut arcs = 0;
        for section in &sections {
            let arc = section.get("member_kind") == Some(&json!("realized_arc"))
                || arc_ids.contains(&section["pipe_id"].as_str().unwrap_or(""));
            if arc {
                arcs += 1;
                assert_eq!(section["beam_theory"], json!({"model": "euler_bernoulli"}), "{mode:?}: {}", section["pipe_id"]);
            } else {
                assert_eq!(section["beam_theory"]["model"], json!(TIMOSHENKO), "{mode:?}: {}", section["pipe_id"]);
            }
        }
        assert!(arcs > 0, "{mode:?}: arc evidence is published");
    }
}

/// Design §2.5: a straight member's `pipe_sections` entry has exactly these keys; a realized
/// arc's has these and `member_kind`.
const PIPE_SECTION_KEYS: [&str; 11] = ["pipe_id", "geometry_basis", "outside_diameter_m", "ro_m", "wall_thickness_m",
    "mill_tolerance_m", "corrosion_allowance_m", "nominal_section", "reduced_section", "pressure_bore", "beam_theory"];
/// The I7 mixed L-line (straight S1, realized arc BEND, straight S2) and the pipe ids of the
/// realized arcs the product builds from it.
fn mixed_l_line(shear_model: &str) -> (Value, std::collections::BTreeSet<String>, usize) {
    let request = with_shear_model(sketch("U2-L-ANCH-PTW-K2"), shear_model);
    let (_, b) = built(&request);
    let arcs: std::collections::BTreeSet<String> = b.curved_bend_elements.iter().map(|bend| bend.pipe_id.clone()).collect();
    assert!(!arcs.is_empty() && arcs.len() < b.pipes.len(), "precondition: a mixed L-line");
    (request, arcs, b.pipes.len())
}
/// One exact case's `pipe_sections` entries by pipe id, each checked for the §2.5 key set: a
/// realized arc (and only a realized arc) carries `member_kind = "realized_arc"` and
/// Euler-Bernoulli beam theory.
fn checked_pipe_sections<'a>(case: &'a Value, arcs: &std::collections::BTreeSet<String>, members: usize, label: &str)
    -> BTreeMap<&'a str, &'a Value> {
    let sections = case["pipe_sections"].as_array().unwrap();
    assert_eq!(sections.len(), members, "{label}: one entry per member");
    let mut by_id = BTreeMap::new();
    for section in sections {
        let id = section["pipe_id"].as_str().unwrap();
        let mut keys: std::collections::BTreeSet<&str> = section.as_object().unwrap().keys().map(String::as_str).collect();
        if arcs.contains(id) {
            assert_eq!(section["member_kind"], json!("realized_arc"), "{label} {id}: the arc marker");
            assert_eq!(section["beam_theory"], json!({"model": "euler_bernoulli"}), "{label} {id}: an arc is Euler-Bernoulli");
            keys.remove("member_kind");
        } else {
            assert!(section.get("member_kind").is_none(), "{label} {id}: a straight member has no member_kind");
        }
        assert_eq!(keys, PIPE_SECTION_KEYS.into_iter().collect(), "{label} {id}: the section keys");
        assert!(by_id.insert(id, section).is_none(), "{label} {id}: one entry");
    }
    by_id
}

/// Design §2.5 (the arc marker): a Timoshenko document whose realized arc lies outside every
/// pressure region (explicitly empty regions, so no region geometry names any member) still
/// marks the arc's `pipe_sections` entry `member_kind = "realized_arc"` with Euler-Bernoulli beam
/// theory, and each straight member's entry has exactly the D-5 keys and its Timoshenko leaf.
#[test]
fn pipe_sections_mark_a_realized_arc_outside_every_pressure_region() {
    let (mut request, arcs, members) = mixed_l_line(TIMOSHENKO);
    for case in request["model"]["load_cases"].as_array_mut().unwrap() {
        assert!(case["pressure_regions"].as_array().is_some_and(|r| !r.is_empty()), "precondition: authored regions");
        case["pressure_regions"] = json!([]);
    }
    for mode in MODES {
        let envelope = run(&request, mode);
        assert_eq!(envelope["status"]["mechanics"], json!("MECHANICS_SOLVED"), "{mode:?}: {}", envelope["diagnostics"]);
        let evidence = &envelope["contract_evidence"];
        let mut geometry = Vec::new();
        records_with(&evidence["pressure"], "traversal_forward", &mut geometry);
        assert!(geometry.is_empty(), "{mode:?}: precondition: no region geometry");
        let cases = evidence["exact_cases"].as_array().unwrap();
        assert!(!cases.is_empty(), "{mode:?}: exact case evidence");
        for case in cases {
            let label = format!("{mode:?} {}", case["load_case_id"]);
            let sections = checked_pipe_sections(case, &arcs, members, &label);
            let mut straight = 0;
            for (id, section) in sections {
                if arcs.contains(id) {
                    continue;
                }
                let material = case["pipe_materials"].as_array().unwrap().iter().find(|m| m["pipe_id"] == id).unwrap();
                assert_timoshenko_evidence(section, number(&material["nu"]), &format!("{label} {id}"));
                straight += 1;
            }
            assert_eq!(straight + arcs.len(), members, "{label}: every straight member is Timoshenko");
        }
    }
}

/// Design §2.5: with the arc inside a pressure region, under either shear model, the region
/// geometry and `pipe_sections` mark the same members, exactly the realized arcs, and each region
/// geometry record's section part (without its traversal and tangent keys) is its member's
/// `pipe_sections` entry, marker included.
#[test]
fn pipe_sections_and_region_geometry_mark_the_same_realized_arcs() {
    for shear_model in ["euler_bernoulli", TIMOSHENKO] {
        let (request, arcs, members) = mixed_l_line(shear_model);
        for mode in MODES {
            let envelope = run(&request, mode);
            assert_eq!(envelope["status"]["mechanics"], json!("MECHANICS_SOLVED"), "{shear_model} {mode:?}: {}", envelope["diagnostics"]);
            let evidence = &envelope["contract_evidence"];
            let cases = evidence["exact_cases"].as_array().unwrap();
            assert!(!cases.is_empty(), "{shear_model} {mode:?}: exact case evidence");
            for case in cases {
                let label = format!("{shear_model} {mode:?} {}", case["load_case_id"]);
                let sections = checked_pipe_sections(case, &arcs, members, &label);
                let regions: Vec<&Value> = evidence["pressure"].as_array().unwrap().iter()
                    .filter(|region| region["load_case_id"] == case["load_case_id"]).collect();
                assert!(!regions.is_empty(), "{label}: a pressure region");
                let mut region_arcs = std::collections::BTreeSet::new();
                for record in regions.iter().flat_map(|region| region["geometry"].as_array().unwrap()) {
                    let id = record["pipe_id"].as_str().unwrap();
                    match record.get("member_kind") {
                        Some(kind) => {
                            assert_eq!(kind, &json!("realized_arc"), "{label} {id}: region marker");
                            region_arcs.insert(id.to_string());
                        }
                        None => assert!(!arcs.contains(id), "{label} {id}: an unmarked arc in the region geometry"),
                    }
                    let mut part = record.clone();
                    let object = part.as_object_mut().unwrap();
                    object.remove("traversal_forward").expect("a region geometry record");
                    object.remove("arc_end_tangents_global");
                    // T4-U4b (§2.5): an arc's region record also states its bend radius.
                    object.remove("bend_radius_m");
                    assert_eq!(&part, sections[id], "{label} {id}: region geometry section part is the pipe_sections entry");
                }
                assert_eq!(region_arcs, arcs, "{label}: the region marks exactly the realized arcs");
            }
        }
    }
}

/// Design §4 item 2, §8 ("Timoshenko leaf missing from a recovery or
/// primitive frame"), at the product level: a member with no load primitive
/// has nothing to bind its recovery frame to but its formation-source frame.
/// The product's certified member recovery binds it there by member identity:
/// the unmutated frame certifies; the same member's frame with its Timoshenko
/// leaf dropped (a stale Euler-Bernoulli recovery frame) fails closed, as does
/// a source that names no member identities.
#[test]
fn kd5_straight_frames_are_bound_to_the_formation_source() {
    let request = with_shear_model(serde_json::from_str(N05).unwrap(), TIMOSHENKO);
    let (_, built) = built(&request);
    let n = built.nodes.len() * DOF_PER_NODE;
    let force = LoadLedger::new().finish(n).unwrap();
    assert!(force.load_primitives().is_empty(), "precondition: no load primitive on any member");
    let source = certified_publication::source(&built, &force, &[]);
    assert_eq!(source.frame_members.len(), source.frames.len());
    let bound = MemberBoundInputs { w: vec![0.0; n], w2: vec![0.0; n], exponents: vec![i32::MIN; n], lambda_sigma: 0.0 };
    let u = vec![0.0; n];
    let section = built.sections[&built.pipes[0].element_id].clone();
    for pipe in &built.pipes {
        assert!(pipe.section.beam_theory.is_timoshenko(), "precondition: {} is Timoshenko", pipe.element_id);
        assert!(certified_publication::member(pipe, None, &force, &source, &u, &bound).is_ok(), "{}", pipe.element_id);
        assert!(certified_publication::straight_stress_spans(pipe, &force, &source, &u, &bound, &section, None).is_ok());
        let mut stale = pipe.clone();
        stale.section.beam_theory = BeamTheory::EulerBernoulli;
        assert!(certified_publication::member(&stale, None, &force, &source, &u, &bound).is_err(), "{}: a dropped leaf", pipe.element_id);
        assert!(certified_publication::straight_stress_spans(&stale, &force, &source, &u, &bound, &section, None).is_err());
        let mut anonymous = source.clone();
        anonymous.frame_members.clear();
        assert!(certified_publication::member(pipe, None, &force, &anonymous, &u, &bound).is_err(), "{}: no member identity", pipe.element_id);
    }
}

/// O4 (DEL-04-08 Design `o4-source-extension.md` §3.1, §6; formerly
/// `o4_timoshenko_declines_are_reached_by_published_documents`): the
/// Timoshenko twin of the N05 retained-source control now selects
/// pressure-source-1 in both modes, as its Euler-Bernoulli original does. Its
/// evidence states kappa_E and A_s from the nominal section and the case's nu,
/// its private payload names the frame's shear-area leaf, and the Rust reader
/// reads it eligible with its captured invocation. (W1-E's G-A admission is a
/// later slice's.)
#[test]
fn o4_timoshenko_documents_publish_retained_source() {
    use open_pipe_stress_result_export::reader_verdict as rv;
    let original: Value = serde_json::from_str(N05).unwrap();
    let timoshenko = with_shear_model(original.clone(), TIMOSHENKO);
    let (_, built) = built(&timoshenko);
    for mode in MODES {
        let control = run(&original, mode);
        assert_eq!(control["producer"]["semantic_contract_id"], json!(exact_admission::PRESSURE_SOURCE_SEMANTIC_CONTRACT_ID), "{mode:?}");
        let (envelope, dump) = crate::source_receipt::with_source_dump(|| run(&timoshenko, mode));
        assert_eq!(envelope["status"]["mechanics"], json!("MECHANICS_SOLVED"), "{mode:?}: {}", envelope["diagnostics"]);
        assert_eq!(envelope["producer"]["semantic_contract_id"], json!(exact_admission::PRESSURE_SOURCE_SEMANTIC_CONTRACT_ID), "{mode:?}");
        assert!(envelope.get("source_block_recovery").is_some(), "{mode:?}");
        assert!(!envelope["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == json!("SOURCE_BLOCK_RECOVERY_UNAVAILABLE")),
            "{mode:?}: {}", envelope["diagnostics"]);
        let exact = &envelope["contract_evidence"]["exact_cases"][0];
        let nu = number(&exact["pipe_materials"][0]["nu"]);
        let (_, shear) = assert_timoshenko_evidence(&exact["pipe_sections"][0], nu, &format!("{mode:?}"));
        assert_eq!(built.frame_elements[0].section.shear_area().map(f64::to_bits), Some(shear.to_bits()), "{mode:?}");
        // The private payload's one frame carries its shear-area leaf.
        assert_eq!(dump.len(), 1, "{mode:?}");
        let payload: Value = serde_json::from_str(dump[0]["normalized_source_text"].as_str().unwrap()).unwrap();
        assert_eq!(payload["payload"]["frames"][0]["formation_inputs"]["shear_area_bits"], json!(format!("{:016x}", shear.to_bits())), "{mode:?}");
        let invocation = json!({"request": timoshenko, "solver_mode": mode.as_str()});
        let verdict = rv::validate("pressure-source", &envelope, Some(&timoshenko["model"]), Some(&invocation));
        assert_eq!(verdict, json!({"ok": true, "eligible": true, "findings": []}), "{mode:?}");
    }
}

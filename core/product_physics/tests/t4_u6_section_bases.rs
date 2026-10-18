//! T4-U6 (DEL-05-03 Design `section-bases-and-timoshenko.md` §§1, 2.1, 2.5, 7
//! and 8): the product against the frozen independent references in
//! `validation/references/t4_u6/` (and T4-I8's MILLTOL re-freeze), through
//! the captured entry in both solver modes, under the references' own
//! criteria. Every expected and wrong value is read from a reference file;
//! none is computed through the product. A disagreement goes back to the
//! reference owner and its refuter, never to an edited expectation (§7).

use open_pipe_stress_product_physics::PreviewSolverMode;
use serde_json::{json, Value};
use std::collections::BTreeMap;

#[path = "common/current_model.rs"]
mod current_model;
#[path = "common/frozen_rows.rs"]
mod frozen_rows;
use frozen_rows::*;

const ORACLE: &str = include_str!("../../../validation/references/t4_u6/oracle.json");
const REQUESTS: [(&str, &str); 8] = [
    ("requests/U6-BASES-001-EB.request.json", include_str!("../../../validation/references/t4_u6/requests/U6-BASES-001-EB.request.json")),
    ("requests/U6-BASES-001-EB-SELF-WEIGHT.request.json", include_str!("../../../validation/references/t4_u6/requests/U6-BASES-001-EB-SELF-WEIGHT.request.json")),
    ("requests/U6-BASES-001-TIMO-SELF-WEIGHT.request.json", include_str!("../../../validation/references/t4_u6/requests/U6-BASES-001-TIMO-SELF-WEIGHT.request.json")),
    ("requests/U6-BASES-001-TIMO.request.json", include_str!("../../../validation/references/t4_u6/requests/U6-BASES-001-TIMO.request.json")),
    ("requests/U6-BASES-002.request.json", include_str!("../../../validation/references/t4_u6/requests/U6-BASES-002.request.json")),
    ("requests/U6-BASES-002-GROUP.request.json", include_str!("../../../validation/references/t4_u6/requests/U6-BASES-002-GROUP.request.json")),
    ("requests/U6-BASES-002-TIMO.request.json", include_str!("../../../validation/references/t4_u6/requests/U6-BASES-002-TIMO.request.json")),
    ("requests/U6-BASES-COINCIDENT.request.json", include_str!("../../../validation/references/t4_u6/requests/U6-BASES-COINCIDENT.request.json")),
];
const I8: &str = include_str!("../../../validation/references/t4_i8/rebuilt_reference_cases.json");
const TIMOSHENKO: &str = "timoshenko_annulus_energy_v1";
const DISPLACEMENT_KINDS: [&str; 6] = [
    "global_nodal_displacement_x",
    "global_nodal_displacement_y",
    "global_nodal_displacement_z",
    "global_nodal_rotation_x",
    "global_nodal_rotation_y",
    "global_nodal_rotation_z",
];
const WALL_FORCE_KINDS: [&str; 6] = [
    "pipe_wall_axial_force_v2",
    "pipe_effective_axial_force_v2",
    "pipe_wall_endpoint_action_v2",
    "support_reaction_component_v2",
    "support_reaction_force_magnitude_v2",
    "support_reaction_moment_magnitude_v2",
];

fn oracle() -> Value {
    serde_json::from_str(ORACLE).unwrap()
}

/// One product run of a frozen request (or of its stated shear-model twin).
struct Run {
    case_id: String,
    variant: String,
    form: String,
    /// The document selects Timoshenko where the oracle's variant is
    /// Euler-Bernoulli (the case's `shear_model_note`: the rows are the same).
    shear_twin: bool,
    load_cases: Vec<String>,
    mode: PreviewSolverMode,
    envelope: Value,
}

impl Run {
    fn context(&self) -> String {
        format!("{} {} [{}] {:?}", self.case_id, self.variant, self.form, self.mode)
    }
    fn results(&self) -> &[Value] {
        self.envelope["results"].as_array().unwrap()
    }
}

/// Every frozen request of every variant, bound by its SHA-256, in both modes.
/// A case whose `shear_model_note` states the rows are the same under
/// timoshenko_annulus_energy_v1 is also run with that selection.
fn runs(oracle: &Value) -> Vec<Run> {
    let mut out = Vec::new();
    for (case_id, case) in oracle["cases"].as_object().unwrap() {
        for (variant, body) in case["variants"].as_object().unwrap() {
            // Every frozen load case is carried by the variant's request or its
            // self-weight request; the load/reference-state twin repeats the request's.
            let carried = |form: &str| body.get(form).map_or(Vec::new(), |b| b["load_cases"].as_array().unwrap().clone());
            let mut all = [carried("request"), carried("request_self_weight")].concat();
            all.sort_by_key(|c| c.as_str().unwrap().to_owned());
            let mut frozen: Vec<Value> = body["load_cases"].as_object().unwrap().keys().map(|k| json!(k)).collect();
            frozen.sort_by_key(|c| c.as_str().unwrap().to_owned());
            assert_eq!(all, frozen, "{case_id} {variant}: every frozen load case is authored once");
            if body.get("request_load_state_group").is_some() {
                assert_eq!(carried("request_load_state_group"), carried("request"), "{case_id} {variant}");
            }
            for form in ["request", "request_load_state_group", "request_self_weight"] {
                let Some(binding) = body.get(form) else { continue };
                let path = binding["path"].as_str().unwrap();
                let text = REQUESTS.iter().find(|(p, _)| *p == path).unwrap_or_else(|| panic!("{path} is not included")).1;
                assert_eq!(sha256_hex(text.as_bytes()), binding["sha256"].as_str().unwrap(), "{case_id} {variant} {form}: the request the oracle binds");
                let request: Value = serde_json::from_str(text).unwrap();
                assert_eq!(request["model"]["pressure_contract"]["shear_model"], json!(variant), "{case_id} {form}");
                let load_cases: Vec<String> =
                    binding["load_cases"].as_array().unwrap().iter().map(|c| c.as_str().unwrap().to_owned()).collect();
                let mut documents = vec![(form.to_owned(), false, request.clone())];
                if case.get("shear_model_note").is_some() && variant == "euler_bernoulli" {
                    let mut twin = request.clone();
                    twin["model"]["pressure_contract"]["shear_model"] = json!(TIMOSHENKO);
                    documents.push((format!("{form} under {TIMOSHENKO}"), true, twin));
                }
                for (form, shear_twin, document) in documents {
                    for mode in MODES {
                        let envelope = solve(&document, mode);
                        assert_eq!(envelope["status"]["mechanics"], json!("MECHANICS_SOLVED"), "{case_id} {variant} [{form}] {mode:?}: {}", envelope["diagnostics"]);
                        assert_eq!(envelope["formulation_basis"]["profile_id"], json!("exact_pressure_v3"));
                        assert!(passed(&envelope),
                            "{case_id} {variant} [{form}] {mode:?}: published Passed: {}", envelope["diagnostics"]);
                        out.push(Run { case_id: case_id.clone(), variant: variant.clone(), form: form.clone(), shear_twin, load_cases: load_cases.clone(), mode, envelope });
                    }
                }
            }
        }
    }
    assert!(!out.is_empty());
    out
}

/// Every frozen row of the run's authored load cases that `keep` selects,
/// compared at its own criterion. Returns the number of comparisons.
fn check_rows(oracle: &Value, run: &Run, keep: &dyn Fn(&Value) -> bool, failures: &mut Vec<String>) -> usize {
    let variant = &oracle["cases"][&run.case_id]["variants"][&run.variant];
    let mut checked = 0;
    for case in &run.load_cases {
        for row in variant["load_cases"][case]["rows"].as_array().unwrap().iter().filter(|row| keep(row)) {
            assert_eq!(row["basis_ref"]["ref_id"], json!(case));
            let origin = &row["reference_origin"];
            let expected = number(&oracle.pointer(origin["pointer"].as_str().unwrap()).unwrap()["value"])
                * unit_factor(origin["reference_unit"].as_str().unwrap(), row["unit"].as_str().unwrap());
            let criterion = &row["criterion"];
            let tolerance = number(&criterion["relative_tolerance"]) * expected.abs() + number(&criterion["absolute_tolerance"]);
            checked += compare(run.results(), row, case, expected, tolerance, &format!("{} {case}", run.context()), failures);
        }
    }
    checked
}

/// Each listed row of the selected discriminators (`ids` empty: all) of the
/// run's authored load cases: the product does not publish the wrong
/// mapping's value. Counts evaluated rows per discriminator.
fn check_discriminators(
    oracle: &Value,
    run: &Run,
    ids: &[&str],
    evaluated: &mut BTreeMap<String, usize>,
    failures: &mut Vec<String>,
) {
    let case = &oracle["cases"][&run.case_id];
    for discriminator in case["wrong_result_discriminators"].as_array().unwrap() {
        let id = discriminator["id"].as_str().unwrap();
        let load_case = discriminator["load_case"].as_str().unwrap();
        if discriminator["variant"] != json!(run.variant)
            || !run.load_cases.iter().any(|c| c == load_case)
            || !(ids.is_empty() || ids.contains(&id))
            // The shear-model twin keeps every row, not the shear mutants' rows.
            || (run.shear_twin && id.starts_with("shear_deformation"))
        {
            continue;
        }
        let rows = case["variants"][&run.variant]["load_cases"][load_case]["rows"].as_array().unwrap();
        for listed in discriminator["listed_rows"].as_array().unwrap() {
            let frozen = rows
                .iter()
                .find(|r| {
                    ["kind", "entity_ref", "component", "location", "section_basis"].iter().all(|key| r.get(*key) == listed.get(*key))
                })
                .unwrap_or_else(|| panic!("{id}: the listed row {} is frozen", label(listed, "")));
            let wrong = number(&listed["wrong"]["value"])
                * unit_factor(listed["wrong"]["unit"].as_str().unwrap(), frozen["unit"].as_str().unwrap());
            let absolute = number(&frozen["criterion"]["absolute_tolerance"]);
            for basis in bases_for(listed) {
                match published(run.results(), frozen, load_case, basis).as_slice() {
                    [one] if discriminated(number(&one["value"]), wrong, absolute) => {
                        *evaluated.entry(id.to_owned()).or_default() += 1;
                    }
                    other => failures.push(format!(
                        "{} {load_case} {id} {}: published {:?} is the wrong mapping's {wrong:e}",
                        run.context(),
                        label(listed, basis),
                        other.iter().map(|r| number(&r["value"])).collect::<Vec<_>>()
                    )),
                }
            }
        }
    }
}

fn kind_in(kinds: &'static [&'static str]) -> impl Fn(&Value) -> bool {
    move |row: &Value| kinds.contains(&row["kind"].as_str().unwrap())
}

fn assert_each_evaluated(evaluated: &BTreeMap<String, usize>, ids: &[&str]) {
    for id in ids {
        assert!(evaluated.get(*id).is_some_and(|n| *n > 0), "discriminator {id} was evaluated: {evaluated:?}");
    }
}

/// Design §8 ("Mill tolerance leaking into stiffness", "Allowance leaking into
/// stiffness"): with c_m, c_a > 0 the stiffness section is the nominal wall.
/// Every frozen displacement and rotation row of t4_u6 (both shear models,
/// both document forms) and of MILLTOL matches, and none is the value of a
/// stiffness section on t − c_m, t − c_a or t_r.
#[test]
fn u6_bases_stiffness_uses_the_nominal_wall() {
    const IDS: [&str; 3] = ["mill_tolerance_in_stiffness", "allowance_in_stiffness", "stiffness_on_reduced_section"];
    let oracle = oracle();
    let (mut checked, mut failures, mut evaluated) = (0, Vec::new(), BTreeMap::new());
    for run in runs(&oracle) {
        checked += check_rows(&oracle, &run, &kind_in(&DISPLACEMENT_KINDS), &mut failures);
        check_discriminators(&oracle, &run, &IDS, &mut evaluated, &mut failures);
    }
    let i8: Value = serde_json::from_str(I8).unwrap();
    let mut milltol = BTreeMap::new();
    for (variant, form, mode, envelope) in milltol_runs(&i8) {
        let context = format!("{variant} {form} {mode:?}");
        checked += milltol_rows(&i8, &variant, &envelope, &kind_in(&DISPLACEMENT_KINDS), &context, &mut failures);
        milltol_discriminators(&i8, &variant, &envelope, &IDS, &context, &mut milltol, &mut failures);
    }
    eprintln!("T4-U6 stiffness: {checked} displacement rows; t4_u6 {evaluated:?}; MILLTOL {milltol:?}");
    assert!(failures.is_empty(), "{}", failures[..failures.len().min(20)].join("\n"));
    assert_each_evaluated(&evaluated, &IDS);
    assert_each_evaluated(&milltol, &IDS);
}

/// Design §8 ("ε_p area on the reduced section"): the As in
/// ε_p = (1−2ν)pAi/(E·As) is the stiffness As, so under collinear straight
/// restraint the wall and effective forces, the wall end actions and the
/// reactions are those of t4_u6's restrained case (Nw = 2νP, and
/// 2νP − E·Aₙ·αΔT with thermal; both document forms; also under the
/// Timoshenko selection, which the reference states leaves every row) and of
/// MILLTOL's restrained variant, never the reduced-area mutants'.
#[test]
fn u6_bases_restrained_wall_force_uses_the_stiffness_area() {
    const IDS: [&str; 2] = ["eps_p_area_on_reduced_section", "poisson_eigenstrain_area_on_reduced_section"];
    let oracle = oracle();
    let (mut checked, mut failures, mut evaluated) = (0, Vec::new(), BTreeMap::new());
    let mut forms = Vec::new();
    for run in runs(&oracle).iter().filter(|run| run.case_id == "U6-BASES-002") {
        checked += check_rows(&oracle, run, &kind_in(&WALL_FORCE_KINDS), &mut failures);
        check_discriminators(&oracle, run, &IDS, &mut evaluated, &mut failures);
        forms.push(run.form.clone());
    }
    assert!(forms.iter().any(|f| f.contains("load_state_group")) && forms.iter().any(|f| f.contains(TIMOSHENKO)), "{forms:?}");
    let i8: Value = serde_json::from_str(I8).unwrap();
    let mut milltol = BTreeMap::new();
    for (variant, form, mode, envelope) in milltol_runs(&i8) {
        let context = format!("{variant} {form} {mode:?}");
        checked += milltol_rows(&i8, &variant, &envelope, &kind_in(&WALL_FORCE_KINDS), &context, &mut failures);
        milltol_discriminators(&i8, &variant, &envelope, &IDS, &context, &mut milltol, &mut failures);
    }
    eprintln!("T4-U6 restrained wall force: {checked} rows; t4_u6 {evaluated:?}; MILLTOL {milltol:?}");
    assert!(failures.is_empty(), "{}", failures[..failures.len().min(20)].join("\n"));
    assert_each_evaluated(&evaluated, &IDS);
    assert_each_evaluated(&milltol, &IDS);
}

/// Design §2.1 and §8 ("One basis for both families", "Allowance omitted from
/// the reduced wall or the pressure bore", "Nominal-basis Lamé evaluated at
/// the pressure bore"): every frozen stress row on each section basis
/// matches, nominal Lamé at r_i,n and the reduced section and pressure bore
/// with the allowance, and no row is a wrong mapping's. On the coincident
/// case (c_m = c_a = 0) both families are published with one value (H4).
#[test]
fn u6_bases_stress_rows_on_both_sections() {
    const IDS: [&str; 5] = [
        "reduced_rows_copied_from_nominal",
        "nominal_rows_copied_from_reduced",
        "allowance_omitted_from_reduced_section",
        "nominal_lame_at_pressure_bore",
        "cap_area_on_nominal_bore",
    ];
    let oracle = oracle();
    let (mut checked, mut failures, mut evaluated) = (0, Vec::new(), BTreeMap::new());
    let mut coincident = 0;
    for run in runs(&oracle) {
        checked += check_rows(&oracle, &run, &|row| row.get("section_basis").is_some(), &mut failures);
        check_discriminators(&oracle, &run, &IDS, &mut evaluated, &mut failures);
        // Every published stress row names its basis and has its twin.
        for row in run.results().iter().filter(|r| D5_STRESS_KINDS.contains(&r["kind"].as_str().unwrap())) {
            let basis = section_basis(row);
            assert!(BASES.contains(&basis), "{} {}: a stress row without a section basis", run.context(), row["id"]);
            let twin = BASES.iter().find(|b| **b != basis).unwrap();
            let pair = run.results().iter().filter(|r| {
                ["kind", "entity_ref", "basis_ref"].iter().all(|k| r[*k] == row[*k])
                    && r["metadata"]["component"] == row["metadata"]["component"]
                    && r["metadata"]["location"] == row["metadata"]["location"]
                    && section_basis(r) == *twin
            });
            let pair: Vec<_> = pair.collect();
            assert_eq!(pair.len(), 1, "{} {}: its {twin} twin", run.context(), row["id"]);
            if run.case_id == "U6-BASES-COINCIDENT" {
                assert_eq!(number(&pair[0]["value"]).to_bits(), number(&row["value"]).to_bits(), "{} {}: coincident bases", run.context(), row["id"]);
                coincident += 1;
            }
        }
    }
    let i8: Value = serde_json::from_str(I8).unwrap();
    let mut milltol = BTreeMap::new();
    const MILLTOL_IDS: [&str; 7] = [
        "single_basis_reduced_rows_from_nominal",
        "single_basis_reduced_hoop_from_nominal",
        "single_basis_nominal_rows_from_reduced",
        "single_basis_nominal_hoop_from_reduced",
        "allowance_omitted_from_reduced_section",
        "nominal_lame_at_pressure_bore",
        "nominal_lame_at_pressure_bore_outer",
    ];
    for (variant, form, mode, envelope) in milltol_runs(&i8) {
        let context = format!("{variant} {form} {mode:?}");
        checked += milltol_rows(&i8, &variant, &envelope, &|row| row.get("section_basis").is_some(), &context, &mut failures);
        milltol_discriminators(&i8, &variant, &envelope, &MILLTOL_IDS, &context, &mut milltol, &mut failures);
    }
    eprintln!("T4-U6 stress rows: {checked} rows; {coincident} coincident pairs; t4_u6 {evaluated:?}; MILLTOL {milltol:?}");
    assert!(failures.is_empty(), "{}", failures[..failures.len().min(20)].join("\n"));
    assert!(coincident > 0);
    assert_each_evaluated(&evaluated, &IDS);
    assert_each_evaluated(&milltol, &MILLTOL_IDS);
}

/// Design §1 and §8 ("Contents on the nominal bore; metal mass on the reduced
/// wall", "Allowance omitted from … the pressure bore"): the generated
/// self-weight of t4_u6's frozen self-weight requests (metal 7850 kg/m³ on the
/// nominal wall, contents 1000 kg/m³ on the pressure bore) gives the frozen
/// reactions, displacements, actions and stresses of `case:self-weight` and
/// `case:combined` under both shear models, and none of the mass mutants'
/// values. The product admits each request's generation record as Fresh.
#[test]
fn u6_bases_mass_metal_nominal_contents_pressure_bore() {
    const IDS: [&str; 3] = ["contents_on_nominal_bore", "metal_mass_on_reduced_wall", "allowance_omitted_from_reduced_section"];
    let oracle = oracle();
    let (mut checked, mut failures, mut evaluated) = (0, Vec::new(), BTreeMap::new());
    let mut variants = Vec::new();
    for run in runs(&oracle).iter().filter(|run| run.form.starts_with("request_self_weight")) {
        let reactions = |row: &Value| row["kind"].as_str().unwrap().starts_with("support_reaction");
        let found = check_rows(&oracle, run, &reactions, &mut failures);
        assert!(found > 0, "{}: reaction rows", run.context());
        checked += found + check_rows(&oracle, run, &|row| !reactions(row), &mut failures);
        let mut by_id = BTreeMap::new();
        check_discriminators(&oracle, run, &IDS, &mut by_id, &mut failures);
        for (id, n) in by_id {
            *evaluated.entry(id).or_insert(0) += n;
        }
        variants.push(run.variant.clone());
    }
    variants.sort();
    variants.dedup();
    // The product reads each frozen generation record as its own exact mass
    // method, Fresh against the request's section.
    for body in oracle["cases"]["U6-BASES-001"]["variants"].as_object().unwrap().values() {
        let path = body["request_self_weight"]["path"].as_str().unwrap();
        let request = current_model::reauthor_request(serde_json::from_str(REQUESTS.iter().find(|(p, _)| *p == path).unwrap().1).unwrap());
        let model: open_pipe_stress_product_physics::PreviewModel = serde_json::from_value(request["model"].clone()).unwrap();
        let states = open_pipe_stress_product_physics::self_weight::inspect_applied_self_weight(&model, None);
        assert_eq!(states.len(), 2, "{path}");
        for status in states {
            assert_eq!(status.state, open_pipe_stress_product_physics::self_weight::AppliedSelfWeightState::Fresh, "{path} {}: {:?}", status.case_id, status.diagnostics);
        }
    }
    eprintln!("T4-U6 mass: {checked} rows in {variants:?}; discriminator rows {evaluated:?}");
    assert!(failures.is_empty(), "{}", failures[..failures.len().min(20)].join("\n"));
    assert_eq!(variants, ["euler_bernoulli", TIMOSHENKO]);
    assert_each_evaluated(&evaluated, &IDS[..2]);
}

/// The whole t4_u6 freeze for the authored load cases: every row, the absent
/// kinds, the terminal cap evidence, the `pipe_sections` evidence (keys
/// exactly Design §2.5's; every number at the T4 criterion; κ_E bit-equal to
/// the frozen binary64 exhibit of Design §3.1's order) and every discriminator.
#[test]
fn u6_bases_every_frozen_row_evidence_and_discriminator() {
    let oracle = oracle();
    let (mut checked, mut failures, mut evaluated) = (0, Vec::new(), BTreeMap::new());
    let mut evidence = 0;
    for run in runs(&oracle) {
        checked += check_rows(&oracle, &run, &|_| true, &mut failures);
        check_discriminators(&oracle, &run, &[], &mut evaluated, &mut failures);
        let variant = &oracle["cases"][&run.case_id]["variants"][&run.variant];
        let contract = &run.envelope["contract_evidence"];
        for case in &run.load_cases {
            let lc = &variant["load_cases"][case];
            let absent = any_of_kinds(run.results(), case, lc["absent_row_kinds"].as_array().unwrap());
            assert!(absent.is_empty(), "{} {case}: {absent:?}", run.context());
            for row in lc["evidence_rows"].as_array().unwrap() {
                assert_eq!((row["namespace"].as_str(), row["record"].as_str()), (Some("contract_evidence.pressure"), Some("terminal")));
                let region = contract["pressure"].as_array().unwrap().iter().find(|r| r["load_case_id"] == row["basis_ref"]["ref_id"]);
                let terminal = region.and_then(|r| r["terminals"].as_array().unwrap().iter().find(|t| t["node_ref"] == row["key"]["node_ref"]));
                let Some(terminal) = terminal else {
                    failures.push(format!("{} {case}: no terminal {}", run.context(), row["key"]));
                    continue;
                };
                for (key, value) in row["definition"].as_object().unwrap() {
                    assert_eq!(&terminal[key], value, "{} {case} {}", run.context(), row["key"]);
                }
                let axis = ["x", "y", "z"].iter().position(|a| row["component"] == *a).unwrap();
                let observed = number(&terminal[row["field"].as_str().unwrap()][axis]);
                let expected = number(&lc["expected"][row["expected"].as_str().unwrap()]["value"]);
                let tolerance = number(&row["criterion"]["relative_tolerance"]) * expected.abs() + number(&row["criterion"]["absolute_tolerance"]);
                if (observed - expected).abs() > tolerance {
                    failures.push(format!("{} {case} terminal {} {} {}: {observed:e} vs {expected:e}", run.context(), row["key"], row["field"], row["component"]));
                }
                evidence += 1;
            }
            let frozen = &variant["pipe_sections_evidence"];
            let exact_case = contract["exact_cases"].as_array().unwrap().iter().find(|c| c["load_case_id"] == json!(case)).expect("the case's exact evidence");
            let section = exact_case["pipe_sections"].as_array().unwrap().iter().find(|s| s["pipe_id"] == frozen["pipe_id"]).expect("the pipe's section evidence");
            let keys = |v: &Value| v.as_object().unwrap().keys().cloned().collect::<Vec<_>>();
            assert_eq!(keys(section), keys(frozen), "{} {case}: Design §2.5's keys", run.context());
            for (key, value) in frozen.as_object().unwrap() {
                if key == "beam_theory" && run.shear_twin {
                    assert_eq!(section[key]["model"], json!(TIMOSHENKO));
                    continue;
                }
                compare_evidence(&section[key], value, &format!("{} {case} pipe_sections.{key}", run.context()), &mut failures);
            }
            if let Some(exhibit) = variant.pointer("/binary64_exhibits/shear_factor/value") {
                assert_eq!(number(&section["beam_theory"]["shear_factor"]).to_bits(), number(exhibit).to_bits(),
                    "{} {case}: kappa_E in Design §3.1's binary64 order", run.context());
            }
        }
    }
    let applicable: Vec<String> = oracle["cases"].as_object().unwrap().values()
        .flat_map(|c| c["wrong_result_discriminators"].as_array().unwrap())
        .map(|d| d["id"].as_str().unwrap().to_owned())
        .collect();
    eprintln!("T4-U6: {checked} rows, {evidence} evidence values; discriminator rows {evaluated:?}");
    assert!(failures.is_empty(), "{}", failures[..failures.len().min(20)].join("\n"));
    for id in applicable {
        assert!(evaluated.get(&id).is_some_and(|n| *n > 0), "{id} evaluated: {evaluated:?}");
    }
}

/// A frozen evidence value: a maintained quantity at the T4 1e-9 relative
/// criterion (an exact zero exactly), or text and objects key by key.
fn compare_evidence(observed: &Value, frozen: &Value, context: &str, failures: &mut Vec<String>) {
    match frozen {
        Value::Object(map) if map.contains_key("exact") && map.contains_key("value") => {
            let expected = number(&map["value"]);
            match observed.as_f64() {
                Some(value) if (value - expected).abs() <= 1e-9 * expected.abs() => {}
                other => failures.push(format!("{context}: {other:?} vs {expected:e}")),
            }
        }
        Value::Object(map) => {
            let keys = |v: &Value| v.as_object().map(|m| m.keys().cloned().collect::<Vec<_>>());
            if keys(observed) != keys(frozen) {
                failures.push(format!("{context}: keys {:?} vs {:?}", keys(observed), keys(frozen)));
                return;
            }
            for (key, value) in map {
                compare_evidence(&observed[key], value, &format!("{context}.{key}"), failures);
            }
        }
        other => {
            if observed != other {
                failures.push(format!("{context}: {observed} vs {other}"));
            }
        }
    }
}

//! O-13's arc tests under K-D5's unified member predicate
//! (`DEL-04-01/Design/kd5-member-relative-bound.md` section 3.3: one
//! predicate replaces R-b' and O-13's `arc_group_fires` with its B_u; S* is
//! O-13's coupled [`ActionScales`] over the repaired published groups, arcs
//! included). The protective intents of O-13's G1-G6 and P1-P8
//! (`arc-recovery-guard.md`) are kept against the integrated producer: exact
//! criterion and floor boundaries, each group on its own scale, unavailable
//! bounds fail closed, the named sentence, the merge with the short-bend rule
//! (kept wired until batch 4 removes it on the verified closure, section
//! 4.3), arcs inside S*, and no Passed case publishing an arc row outside the
//! criterion. T4-I34's exact published rows
//! (`validation/references/t4_o13/o13_candidates.json`, 120-digit model
//! solves) stay the reference for the load-bearing case. Inputs are invented.
use super::*;
use crate::formation_guard::{
    member_group_decision, member_scales, merge, recovery_finding, short_bend_finding,
    ActionScales, Bodies, RecoveryGroup, RecoveryRecord, ShortBend, CRITERION,
    SHORT_BEND_PASSED_RATIO,
};

const SHORT: &str = "Short-bend rule";
const CANDIDATES: &str =
    include_str!("../../../../validation/references/t4_o13/o13_candidates.json");
const FLOOR: f64 = 1.0 / 17_179_869_184.0; // 2^-34

fn scales(force: f64, moment: f64) -> ActionScales {
    ActionScales { force, moment }
}

/// The unified decision with q_hat = q (a group whose certified magnitude is
/// its published norm).
fn fires(q: f64, bound: f64, s_star: f64) -> bool {
    member_group_decision(q, q, bound, s_star).unwrap_or(true)
}

fn group(kind: &str, moment: bool, q: f64, bound: Result<f64, String>) -> RecoveryGroup {
    RecoveryGroup {
        location: "end_i".into(),
        kind: kind.into(),
        moment,
        q,
        zero_compatible: false,
        q_hat: q,
        bound,
        absolute_scale: q,
        published_scale: q,
    }
}

fn arc_record(member: &str, groups: Vec<RecoveryGroup>) -> RecoveryRecord {
    RecoveryRecord { member: member.into(), body: Some(0), groups }
}

/// O-13's four end groups of one arc end, each with bound `b`.
fn arc_groups(b: f64) -> Vec<RecoveryGroup> {
    vec![
        group("axial", false, 1.0, Ok(b)),
        group("shear", false, 1.0, Ok(b)),
        group("torsion", true, 1.0, Ok(b)),
        group("bending", true, 1.0, Ok(b)),
    ]
}

fn unit_scales() -> HashMap<usize, ActionScales> {
    HashMap::from([(0, scales(1.0, 1.0))])
}

// ================================================================ unit level

/// G1 (O-13's conjunct boundaries, now the unified predicate's): exact
/// `B > RD(1e-9) q`; no resolution conjunct (a group fires even where
/// q <= 2^10 B, which O-13's B_u path and R-b' needed); the floor
/// q_hat >= 2^-34 S* and the S* >= 2^-988 guard; a nonfinite bound fires.
#[test]
fn g1_unified_predicate_conjuncts_at_their_boundaries() {
    assert!(!fires(1.0, CRITERION, 1.0));
    assert!(fires(1.0, CRITERION.next_up(), 1.0));
    // A binary64 `B > 1e-9 q` would not fire at B = fl(1e-9) (M18's constant).
    assert!(fires(1.0, 1e-9, 1.0));
    // No resolution conjunct: B far above q still fires.
    assert!(fires(1.0, 0.5, 1.0));
    assert!(fires(1.0, 1e6, 1.0));
    // The floor: q = 2^-34 S* is above it; S* one step larger is not.
    let floor_scale = 1.0 / FLOOR;
    assert!(fires(1.0, 1e-6, floor_scale));
    assert!(!fires(1.0, 1e-6, floor_scale.next_up()));
    // The floor judges q_hat, the certified upper magnitude, not q: a gross
    // error cannot shrink a judged row below the floor and hide.
    assert_eq!(member_group_decision(0.0, 1.0, 1.0, floor_scale), Ok(true));
    // S* below 2^-988, or no scale, is below the floor (DESIGN N-2).
    let tiny = f64::from_bits((1023 - 988) << 52);
    assert!(fires(tiny * FLOOR, 1.0, tiny));
    assert!(!fires(1.0, 1.0, tiny.next_down()));
    assert!(!fires(1.0, 1.0, 0.0));
    // A nonfinite bound or comparison input fails closed, whatever the scale.
    assert!(fires(1.0, f64::INFINITY, 0.0));
    assert!(fires(1.0, f64::NAN, 0.0));
    assert!(member_group_decision(1.0, 1.0, 1.0, f64::INFINITY).is_err());
}

/// G2: S* is the moment scale for bending and torsion and the force scale
/// for axial and shear: a group with 2^-34 S_right <= q < 2^-34 S_wrong
/// fires only with the right scale (through the finding, as published).
#[test]
fn g2_each_group_uses_its_own_scale() {
    let (small, large) = (1.0, 1e12);
    let q = 2.0 * FLOOR * small;
    for (kind, moment) in [("axial", false), ("shear", false), ("torsion", true), ("bending", true)] {
        let record = arc_record("A", vec![group(kind, moment, q, Ok(1.0))]);
        let right = if moment { scales(large, small) } else { scales(small, large) };
        let wrong = if moment { scales(small, large) } else { scales(large, small) };
        let fired = |s| recovery_finding(std::slice::from_ref(&record), &HashMap::from([(0, s)]), "c").is_some();
        assert!(fired(right), "{kind}");
        assert!(!fired(wrong), "{kind}");
    }
}

/// G3: an unavailable bound fires, naming the member, group and reason.
#[test]
fn g3_an_unavailable_bound_fires() {
    let quiet = arc_record("A", arc_groups(0.0));
    assert!(recovery_finding(std::slice::from_ref(&quiet), &unit_scales(), "c").is_none());
    let broken = arc_record("B", vec![group("bending", true, 1.0, Err("ResidualNotContracting".into()))]);
    let finding = recovery_finding(&[quiet, broken], &unit_scales(), "c").expect("an unavailable bound fires");
    assert_eq!(finding.fired, ["B.end_i.bending: the member bound is unavailable (ResidualNotContracting)"]);
    assert!(finding.sentence.starts_with(&format!("{RECOVERY}: load case c is Sensitive because 1 member or support group(s)")));
}

/// G4: the sentence names at most six groups, then the count of the rest.
#[test]
fn g4_the_sentence_names_six_then_counts() {
    let mut groups = arc_groups(1.0);
    groups.extend(arc_groups(1.0).into_iter().map(|mut g| {
        g.location = "end_j".into();
        g
    }));
    let finding = recovery_finding(&[arc_record("A", groups)], &unit_scales(), "c").unwrap();
    assert_eq!(finding.fired.len(), 8);
    assert!(finding.sentence.ends_with("; and 2 more"), "{}", finding.sentence);
    assert!(finding.sentence.contains("A.end_i.axial: q=1e0, B=1e0; A.end_i.shear: q=1e0, B=1e0"));
    assert!(!finding.sentence.contains("A.end_j.bending"));
}

/// G5: a single finding is unchanged by the merge (the unified sentence
/// stays byte for byte); several are joined unified, then short bend, one
/// space apart.
#[test]
fn g5_merge_keeps_a_single_sentence_and_orders_several() {
    let unified = recovery_finding(&[arc_record("A", arc_groups(1.0))], &unit_scales(), "c").unwrap();
    let short = short_bend_finding(
        &[ShortBend { component: "component:A".into(), pipe: "A".into(), ratio: 1e-2 }],
        "c",
    )
    .unwrap();
    let alone = merge(vec![unified.clone()]).unwrap();
    assert_eq!(alone.sentence, unified.sentence);
    assert_eq!(alone.guard, formation_guard::Guard::Recovery);
    let all = merge(vec![unified.clone(), short.clone()]).unwrap();
    assert_eq!(all.sentence, format!("{} {}", unified.sentence, short.sentence));
    assert_eq!(all.fired.len(), unified.fired.len() + 1);
    assert!(merge(Vec::new()).is_none());
}

/// G6 (O-13's scale extension): the body's scales include an arc's
/// repaired groups and the force scale couples the moment scale over the
/// body's extent.
#[test]
fn g6_scales_include_arcs_and_couple_force_and_moment() {
    let bodies = Bodies::new(&[[0.0; 3], [2.0, 0.0, 0.0]], &[(0, 1)]);
    let straight = arc_record("S", vec![group("axial", false, 3.0, Ok(0.0)), group("bending", true, 1.0, Ok(0.0))]);
    let arc = arc_record("A", vec![group("shear", false, 10.0, Ok(0.0)), group("torsion", true, 50.0, Ok(0.0))]);
    // S(force) = 10, S(moment) = 50, L_b = 2: fo = max(10, 25), mo = max(50, 20).
    assert_eq!(member_scales(&[straight.clone(), arc], &bodies)[&0], scales(25.0, 50.0));
    assert_eq!(member_scales(&[straight], &bodies)[&0], scales(3.0, 6.0));
}

/// The short-bend rule's threshold is strict and names each bend below it.
#[test]
fn short_bend_finding_is_strict_and_names_each_bend() {
    let at = |ratio: f64| ShortBend {
        component: "component:B".into(),
        pipe: "B".into(),
        ratio,
    };
    assert!(short_bend_finding(&[at(SHORT_BEND_PASSED_RATIO)], "c").is_none());
    let finding = short_bend_finding(&[at(SHORT_BEND_PASSED_RATIO.next_down()), at(0.5)], "c").unwrap();
    assert_eq!(finding.fired, ["component:B (pipe B): 1.600e-2"]);
    assert!(finding.sentence.starts_with(&format!("{SHORT}: load case c is Sensitive because 1 realized bend(s)")));
}

// ============================================================= product level

/// An L line (T4-I22): A-(3 m)-B, bend B-C (R, phi, flexibility k), C-(4 m)-D,
/// anchored at A and D; self-weight w on every member and an optional
/// uniform temperature change. Section OD 0.168 m, wall 0.007 m.
fn l_line(id: &str, origin: [f64; 3], phi: f64, radius: f64, w: f64, delta_t: f64) -> Value {
    let mut model = preview_model(id);
    model["materials"] = json!([material()]);
    let [x0, y0, z0] = origin;
    let b = [x0 + 3.0, y0];
    let c = [b[0] + radius * phi.sin(), b[1] + radius * (1.0 - phi.cos())];
    let d = [c[0] + 4.0 * phi.cos(), c[1] + 4.0 * phi.sin()];
    let section = json!({"outside_diameter": {"value": 0.168, "unit": "m"}, "wall_thickness": {"value": 0.007, "unit": "m"}});
    let with_section = |mut p: Value| {
        p["section"] = section.clone();
        p
    };
    model["nodes"] = json!([node("A", x0, y0, z0), node("B", b[0], b[1], z0), node("C", c[0], c[1], z0), node("D", d[0], d[1], z0)]);
    model["pipe_segments"] = json!([
        with_section(pipe("AB", "A", "B", [0.0, 1.0, 0.0])),
        with_section(pipe("BC", "B", "C", [0.0, -1.0, 0.0])),
        with_section(pipe("CD", "C", "D", [0.0, 0.0, 1.0]))]);
    let mut loads = Vec::new();
    for p in ["AB", "BC", "CD"] {
        if w != 0.0 {
            loads.push(uniform(&format!("w:{p}"), p, "global_z", w));
        }
        if delta_t != 0.0 {
            loads.push(thermal(&format!("t:{p}"), p, delta_t));
        }
    }
    model["load_cases"][0]["primitive_loads"] = json!(loads);
    model["supports"] = json!([support("aA", "A", &ALL), support("aD", "D", &ALL)]);
    let chord = ((c[0] - b[0]).powi(2) + (c[1] - b[1]).powi(2)).sqrt();
    model["components"] = json!([bend_of("BC", "C", radius, 2.0 * (0.5 * chord / radius).asin())]);
    request_of(model)
}

fn bend_of(pipe_id: &str, node_id: &str, radius: f64, angle: f64) -> Value {
    json!({
        "id": format!("component:{pipe_id}"), "label": "Invented bend", "kind": "bend", "node": node_id,
        "geometry": {"bend_pipe_ref": pipe_id, "bend_radius": {"value": radius, "unit": "m"},
            "bend_angle": {"value": angle, "unit": "rad"},
            "bend_plane_orientation": "global_xy_preview",
            "bend_geometry_source_reference": "invented_user_entered_preview_geometry"},
        "modifiers": {"sif_user_value": {"value": 1.0, "unit": "none"},
            "flexibility_factor_user_value": {"value": 1.0, "unit": "none"},
            "source_reference": "invented_user_entered_preview_no_code_table"},
        "mechanics_interface": {"solver_consumption": "curved_bend_macro_element",
            "rule_check_consumption": "user_rule_pack_inputs_only"},
        "completeness": [{"finding_id": format!("finding:{pipe_id}:geometry"), "status": "complete",
            "diagnostic_code": "BEND_GEOMETRY_INCOMPLETE", "missing_field_kinds": []}],
        "provenance": INVENTED})
}

/// T4-I34's arc analogue of T3's INPLANE row: RF-CANCEL-`id` with M2
/// realized as an arc of radius `radius` over its 1 m chord.
fn arc_inplane(id: &str, radius: f64, side: f64, project: &str) -> Value {
    let mut request = rf_request(id);
    let model = &mut request["model"];
    model["project"]["id"] = json!(project);
    for p in model["pipe_segments"].as_array_mut().unwrap() {
        if p["id"] == "M2" {
            p["y_reference"] = json!({"x": 0.0, "y": side, "z": 0.0});
        }
    }
    model["components"] = json!([bend_of("M2", "N2", radius, 2.0 * (0.5 / radius).asin())]);
    request
}

fn project_of(request: &Value) -> String {
    request["model"]["project"]["id"].as_str().unwrap().to_string()
}

/// The bending pair's published (My, Mz) at each end of `member`.
fn bending_rows(envelope: &MechanicsEnvelope, member: &str) -> [[f64; 2]; 2] {
    let value = |component: &str, location: &str| {
        case_rows(envelope, "case")
            .into_iter()
            .find(|r| {
                r.entity_ref == member
                    && r.kind.starts_with("element_local_bending_moment")
                    && r.metadata
                        .as_ref()
                        .is_some_and(|m| m.component == component && m.location == location)
            })
            .map(|r| r.value)
            .unwrap_or_else(|| panic!("{member} {component} {location}"))
    };
    ["end_i", "end_j"].map(|l| [value("bending_moment_y", l), value("bending_moment_z", l)])
}

/// T4-I34's exact published rows f* of one model arc (120-digit solve).
fn exact_rows(label: &str, mode: PreviewSolverMode, pipe: &str) -> [f64; 12] {
    let data: Value = serde_json::from_str(CANDIDATES).unwrap();
    let mode = format!("{mode:?}");
    let entry = data["model_rows"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["label"] == json!(label) && r["mode"] == json!(mode) && r["pipe"] == json!(pipe))
        .unwrap_or_else(|| panic!("{label} {mode} {pipe}"));
    std::array::from_fn(|k| entry["f_star"][k].as_str().unwrap().parse().unwrap())
}

/// The certified records of one run (K-D5's recovery probe).
fn probed(entry: Entry, request: &Value, mode: PreviewSolverMode) -> (MechanicsEnvelope, Vec<RecoveryRecord>) {
    crate::tests::KD5_RECOVERY_PROBE.with(|probe| *probe.borrow_mut() = Some(Vec::new()));
    let envelope = solved(entry, request, mode);
    let records = crate::tests::KD5_RECOVERY_PROBE.with(|probe| probe.borrow_mut().take().unwrap());
    (envelope, records)
}

/// P1 (load-bearing): ARC-INPLANE-M-G1e80 (R = 1 m), whose binary64
/// recovery published a bending pair farther than RD(1e-9) q from T4-I34's
/// exact f* on a Passed report. Under K-D5 (section 3.1) a Passed case
/// publishes the arc's certified recovery: it is either demoted by the
/// unified predicate, or every published bending pair lies within the
/// criterion of f*. No Passed case publishes the breach.
#[test]
fn p1_arc_bending_breach_is_never_published_passed() {
    let label = "ARC-INPLANE RF-CANCEL-M-G1e80-GnG-INPLANE R=1 side=1";
    let request = arc_inplane("RF-CANCEL-M-G1e80-GnG-INPLANE", 1.0, 1.0, "invented:o13:p1");
    // The captured entry refuses G1e80's unsafe integral floats; typed only.
    for mode in MODES {
        let tag = format!("P1 {mode:?}");
        let (envelope, records) = probed(Entry::Typed, &request, mode);
        assert_ordinary_passed(&envelope, "case", &tag);
        assert!(records.iter().any(|r| r.member == "M2"), "{tag}: the arc carries a certified record");
        let d = integrity(&envelope, "case");
        if d.code == "NUMERICAL_INTEGRITY_CHECKS_PASSED" {
            let exact = exact_rows(label, mode, "M2");
            let published = bending_rows(&envelope, "M2");
            for end in 0..2 {
                let [y, z] = published[end];
                let q = y.hypot(z);
                let error = (y - exact[6 * end + 4]).abs().max((z - exact[6 * end + 5]).abs());
                assert!(error <= CRITERION * q, "{tag}: end {end} error {error:e} against q {q:e}");
            }
        } else {
            assert_demoted(&envelope, "case", RECOVERY, &tag);
        }
    }
}

/// P2: realistic arcs are not demoted. Each case is Passed with neither
/// sentence, and every certified arc group above the floor has
/// B < RD(1e-9) q / 10.
#[test]
fn p2_realistic_arcs_are_not_demoted() {
    let deg = |x: f64| x.to_radians();
    let utm = [7.3e6, 4.6e6, 312.5];
    let mut requests = Vec::new();
    for (name, phi) in [("90", deg(90.0)), ("45", deg(45.0)), ("15", deg(15.0))] {
        requests.push(l_line(&format!("invented:o13:p2:w:{name}"), [0.0; 3], phi, 0.2286, -450.0, 0.0));
        requests.push(l_line(&format!("invented:o13:p2:wt:{name}"), [0.0; 3], phi, 0.2286, -450.0, 50.0));
        requests.push(l_line(&format!("invented:o13:p2:utm:{name}"), utm, phi, 0.2286, -450.0, 50.0));
    }
    // A 0.3 m arc at 1e-4 rad (R = 3 km) and at 1 degree.
    for phi in [1e-4, deg(1.0)] {
        requests.push(l_line(&format!("invented:o13:p2:arc03:{phi}"), [0.0; 3], phi, 0.3 / phi, -450.0, 50.0));
    }
    for request in &requests {
        let bodies = formation_bodies(&prepared(request).built);
        for entry in [Entry::Captured, Entry::Typed] {
            for mode in MODES {
                let tag = format!("{} {entry:?} {mode:?}", project_of(request));
                let (envelope, records) = probed(entry, request, mode);
                let d = integrity(&envelope, "case");
                assert_eq!(d.code, "NUMERICAL_INTEGRITY_CHECKS_PASSED", "{tag}: {}", d.message);
                assert!(!d.message.contains(RECOVERY) && !d.message.contains(SHORT), "{tag}");
                let scales = member_scales(&records, &bodies);
                let arc = records.iter().find(|r| r.member == "BC").unwrap_or_else(|| panic!("{tag}: the arc's record"));
                let scale = scales[&arc.body.unwrap()];
                for g in &arc.groups {
                    let s_star = if g.moment { scale.moment } else { scale.force };
                    if g.q_hat < FLOOR * s_star {
                        continue;
                    }
                    let b = *g.bound.as_ref().expect("the bound is available");
                    assert!(b < 0.1 * CRITERION * g.q, "{tag} {}.{}: B {b:e} against q {:e}", g.location, g.kind, g.q);
                }
            }
        }
    }
}

/// P4 (I5 ruling 1 for arcs): with the load-row guard firing, the load-row
/// sentence stands alone; no recovery sentence is added.
#[test]
fn p4_the_load_row_sentence_stands_alone() {
    let mut request = arc_inplane("RF-CANCEL-M-G1e80-GnG-INPLANE", 1.0, 1.0, "invented:o13:p4");
    udl_body(&mut request["model"], "u", 100.0);
    let view = guard_view(&request, 0);
    assert!(view.any_fires(), "precondition: the load-row guard fires");
    for mode in MODES {
        let tag = format!("P4 {mode:?}");
        let envelope = solved(Entry::Typed, &request, mode);
        let d = integrity(&envelope, "case");
        assert_eq!(d.code, "NUMERICAL_INTEGRITY_SENSITIVE", "{tag}");
        assert!(!d.message.contains(RECOVERY) && !d.message.contains(SHORT), "{tag}: {}", d.message);
    }
}

/// P5 (the no-op rule): a case that is already Sensitive by the solver is
/// left untouched although the short-bend rule would fire. (a) The L line
/// at 1 degree (R phi/L = 1.3e-3, below the short-bend limit). (b) T4-I34's
/// nodal-only arc at k = 1e9.
#[test]
fn p5_an_already_sensitive_case_is_left_untouched() {
    let line = l_line("invented:o13:p5:line", [0.0; 3], 1f64.to_radians(), 0.2286, -450.0, 50.0);
    let ratio = prepared(&line).built.curved_bend_elements[0].arc_length / 3.0;
    assert!(ratio < SHORT_BEND_PASSED_RATIO, "precondition: the short-bend rule fires");
    let mut model = preview_model("invented:o13:p5:stiff");
    model["materials"] = json!([material()]);
    curved_arc(
        &mut model,
        "c",
        [0.0, 0.0],
        2.0,
        std::f64::consts::SQRT_2,
        std::f64::consts::FRAC_PI_2,
        1e9,
        vec![nodal("f:z", "c1", "global_z", 100.0), nodal("m:x", "c1", "RX", 30.0)],
    );
    let stiff = request_of(model);
    for request in [&line, &stiff] {
        for entry in [Entry::Captured, Entry::Typed] {
            for mode in MODES {
                let tag = format!("P5 {} {entry:?} {mode:?}", project_of(request));
                let envelope = solved(entry, request, mode);
                let d = integrity(&envelope, "case");
                assert_eq!(d.code, "NUMERICAL_INTEGRITY_SENSITIVE", "{tag}");
                assert!(!d.message.contains("quality: Passed"), "{tag}: Sensitive by the solver");
                assert!(!d.message.contains(RECOVERY) && !d.message.contains(SHORT), "{tag}");
            }
        }
    }
}

/// P6 (the scale extension): ARC-ONLY-INPLANE-M, M1 and M2 both arcs, one
/// body with no straight member. Under entity rule 2b its S* would be 0 and
/// the floor would silence every group; the arcs' certified groups set S*.
#[test]
fn p6_an_arc_only_body_is_guarded() {
    let mut request = rf_request("RF-CANCEL-M-G1e80-GnG-INPLANE");
    let model = &mut request["model"];
    model["project"]["id"] = json!("invented:o13:p6");
    for p in model["pipe_segments"].as_array_mut().unwrap() {
        p["y_reference"] = json!({"x": 0.0, "y": 1.0, "z": 0.0});
    }
    let angle = 2.0 * 0.5_f64.asin();
    model["components"] = json!([bend_of("M1", "N1", 1.0, angle), bend_of("M2", "N2", 1.0, angle)]);
    let bodies = formation_bodies(&prepared(&request).built);
    for mode in MODES {
        let tag = format!("P6 {mode:?}");
        let (envelope, records) = probed(Entry::Typed, &request, mode);
        assert_ordinary_passed(&envelope, "case", &tag);
        let arcs: Vec<_> = records.iter().filter(|r| r.member == "M1" || r.member == "M2").cloned().collect();
        assert_eq!(arcs.len(), 2, "{tag}");
        let scales = member_scales(&arcs, &bodies);
        assert!(scales.values().all(|s| s.moment > 0.0 && s.force > 0.0), "{tag}: the arc groups set S*");
    }
}

/// P8: a stress-free arc (anchored at one end, free at the other, uniform
/// temperature change) publishes rows of rounding size against an exact
/// zero. K-D5 section 4.5 lists O-13's P8 among the all-zero bodies: it stays
/// Passed when its certified rows are exactly zero, and otherwise is
/// Sensitive by the unified predicate (section 3.3: a nonzero certified
/// group above the floor and the S* guard fires against a true zero). The
/// STRESS_FREE_WITHIN_NUMERICAL_PRECISION notice needs every group to be
/// noise-level and zero-compatible, which an unloaded group with a = 0 can
/// correctly suppress (section 3.3, pins), so it is not required here; it
/// never appears on a Passed case.
#[test]
fn p8_a_stress_free_arc_is_passed_or_demoted_as_an_all_zero_body() {
    let mut model = preview_model("invented:o13:p8");
    model["materials"] = json!([material()]);
    curved_arc(
        &mut model,
        "c",
        [0.0, 0.0],
        2.0,
        std::f64::consts::SQRT_2,
        std::f64::consts::FRAC_PI_2,
        1.0,
        vec![thermal("t:c", "cbend", 10.0)],
    );
    // Free the far end: an anchored-free arc expands without stress.
    model["supports"].as_array_mut().unwrap().retain(|s| s["id"] != "cg1");
    let request = request_of(model);
    for mode in MODES {
        let tag = format!("P8 {mode:?}");
        let envelope = solved(Entry::Typed, &request, mode);
        let d = integrity(&envelope, "case");
        if d.code == "NUMERICAL_INTEGRITY_CHECKS_PASSED" {
            assert!(!codes(&envelope).contains("STRESS_FREE_WITHIN_NUMERICAL_PRECISION"), "{tag}");
        } else {
            assert_demoted(&envelope, "case", RECOVERY, &tag);
        }
        // Either way the published arc actions are of rounding size.
        for row in case_rows(&envelope, "case") {
            if row.entity_ref == "cbend" && row.kind.starts_with("element_local_") && (row.unit == "N" || row.unit == "N*m") {
                assert!(row.value.abs() < 1e-20, "{tag}: {} {}", row.kind, row.value);
            }
        }
    }
}

// ----------------------------------------------------- the short-bend rule

/// HELP_HUMAN's R-1, option (c): a realized bend whose arc length is below
/// SHORT_BEND_PASSED_RATIO of its shorter neighbour demotes its Passed case,
/// naming the bend and its ratio; just above the limit the case stays
/// Passed. On the L line (shorter neighbour 3 m) the limit is at
/// phi = 3 * 1.6e-2 / 0.2286 rad.
#[test]
fn short_bend_rule_at_its_limit() {
    let limit_phi = 3.0 * SHORT_BEND_PASSED_RATIO / 0.2286;
    for (factor, demoted) in [(0.995, true), (1.005, false)] {
        let phi = limit_phi * factor;
        let request = l_line(&format!("invented:o13:short:{factor}"), [0.0; 3], phi, 0.2286, -450.0, 50.0);
        let ratio = prepared(&request).built.curved_bend_elements[0].arc_length / 3.0;
        assert_eq!(ratio < SHORT_BEND_PASSED_RATIO, demoted, "precondition: the ratio's side");
        for entry in [Entry::Captured, Entry::Typed] {
            for mode in MODES {
                let tag = format!("{factor} {entry:?} {mode:?}");
                let envelope = solved(entry, &request, mode);
                assert_ordinary_passed(&envelope, "case", &tag);
                let message = &integrity(&envelope, "case").message;
                if demoted {
                    assert_demoted(&envelope, "case", SHORT, &tag);
                    assert!(
                        message.contains(&format!("component:BC (pipe BC): {ratio:.3e}")),
                        "{tag}: {message}"
                    );
                } else {
                    assert_eq!(integrity(&envelope, "case").code, "NUMERICAL_INTEGRITY_CHECKS_PASSED", "{tag}");
                }
            }
        }
    }
}

/// R-1's cases: the L line at 5 and 8 degrees (and the UTM L line at 4
/// degrees) published Passed torsion and shear breaches up to 4.9 times the
/// criterion (T4-I34 section 4.5); each is now Sensitive by the short-bend
/// rule, while 15 degrees stays Passed (`p2`).
#[test]
fn short_bend_rule_demotes_r1_cases() {
    let deg = |x: f64| x.to_radians();
    for (name, origin, phi) in [
        ("5", [0.0; 3], deg(5.0)),
        ("8", [0.0; 3], deg(8.0)),
        ("utm4", [7.3e6, 4.6e6, 312.5], deg(4.0)),
    ] {
        let request = l_line(&format!("invented:o13:r1:{name}"), origin, phi, 0.2286, -450.0, 50.0);
        for mode in MODES {
            let tag = format!("{name} {mode:?}");
            let envelope = solved(Entry::Typed, &request, mode);
            assert_ordinary_passed(&envelope, "case", &tag);
            assert_demoted(&envelope, "case", SHORT, &tag);
        }
    }
}

/// A source's text without its line comments.
fn product_text(source: &str) -> String {
    source
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .map(|line| format!("{line}\n"))
        .collect()
}

/// The body of `fn name` in `text` (to its first line that is a bare `}` at
/// column 0).
fn body<'a>(text: &'a str, name: &str) -> &'a str {
    let start = text.find(&format!("fn {name}(")).unwrap_or_else(|| panic!("fn {name}"));
    let rest = &text[start..];
    let end = rest.find("\n}\n").unwrap_or(rest.len());
    &rest[..end]
}

/// P7: the call site uses the tested predicate. In PP's source the
/// recovery loop forms every member's certified record (straight and arc)
/// through `certified_publication::member`; the case's finding is the
/// unified `recovery_finding`, merged with the short-bend rule's, and
/// `recovery_finding` decides each group with `member_group_decision`,
/// which carries the exact criterion, the guard and the floor and no
/// resolution conjunct.
#[test]
fn p7_source_pin_ties_the_call_site_to_the_tested_predicate() {
    let lib = product_text(include_str!("../lib.rs"));
    let guard = product_text(include_str!("../formation_guard.rs"));
    let solve = body(&lib, "solve_load_case_observed");
    assert!(solve.contains("certified_publication::member(pipe, macro_bend, &force, &displacements, bound)"));
    let site = &solve[solve.find("let scales = formation_guard::member_scales(&recovery_records, &formation_bodies);").expect("the decision site")..];
    for call in [
        "formation_guard::recovery_finding(&recovery_records, &scales, &load_case.id)",
        "formation_guard::short_bend_finding(&short_bends, &load_case.id)",
        "formation_guard::merge(findings.into_iter().flatten().collect())",
        "formation_guard::amend_integrity_report(",
    ] {
        assert!(site.contains(call), "the call site makes {call}");
    }
    assert!(!lib.contains("arc_recovery_record") && !guard.contains("arc_group_fires") && !guard.contains("rb_prime_fires"));
    assert!(body(&guard, "recovery_finding").contains("member_group_decision(group.q,group.q_hat,*bound,scale)"));
    let decision = body(&guard, "member_group_decision");
    assert!(decision.contains("comparison.add_product(-CRITERION, q)"));
    assert!(decision.contains("s_star >= DESIGN_FLOOR_MIN_SCALE && q_hat >= DESIGN_FLOOR * s_star"));
    assert!(!guard.contains("RESOLUTION_FACTOR"));
}

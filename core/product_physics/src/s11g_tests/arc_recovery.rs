//! O-13: the arc recovery guard and the short-bend rule
//! (`DEL-04-01/Design/arc-recovery-guard.md`). Unit tests of the predicate,
//! the scales, the sentences and the merge; product tests through both
//! entries and both solver modes; the source pin. T4-I34's exact published
//! rows (`validation/references/t4_o13/o13_candidates.json`, 120-digit model
//! solves) are the reference for the load-bearing catch. Inputs are invented.
use super::*;
use crate::formation_guard::{
    action_scales, arc_group_fires, arc_recovery_finding, merge, observed, recovery_finding,
    short_bend_finding, ActionScales, ArcRecoveryRecord, Bodies, RecoveryRecord, ShortBend,
    CRITERION, SHORT_BEND_PASSED_RATIO,
};
use open_pipe_stress_frame_kernel::structural::{ArcGroupBound, ArcRowGroup};

const ARC: &str = "S11-G recovery guard (R-b', arcs)";
const SHORT: &str = "Short-bend rule";
const CANDIDATES: &str =
    include_str!("../../../../validation/references/t4_o13/o13_candidates.json");
const FLOOR: f64 = 1.0 / 17_179_869_184.0; // 2^-34

fn bound(formation: f64, resolution: f64) -> ArcGroupBound {
    ArcGroupBound {
        formation,
        resolution,
    }
}

fn scales(force: f64, moment: f64) -> ActionScales {
    ActionScales { force, moment }
}

// ================================================================ unit level

/// G1: each conjunct of the arc predicate at its exact boundary (q = 1,
/// S* = 1 unless stated). Bending fires on R-b' with B = B_f + B_u, or on
/// R-2's B_f > RD(1e-9) q above the floor with no resolution conjunct; the
/// other groups use B_f only.
#[test]
fn g1_arc_predicate_conjuncts_at_their_boundaries() {
    let unit = scales(1.0, 1.0);
    let bending = ArcRowGroup::Bending;
    // R-b' on B = B_f + B_u: equality silent, one step above fires.
    assert!(!arc_group_fires(bending, 1.0, &bound(0.0, CRITERION), unit));
    assert!(arc_group_fires(bending, 1.0, &bound(0.0, CRITERION.next_up()), unit));
    // ... B_f and B_u add (upward): neither alone exceeds, their sum does.
    let half = CRITERION * 0.5;
    assert!(!arc_group_fires(bending, 1.0, &bound(half, 0.0), unit));
    assert!(arc_group_fires(bending, 1.0, &bound(half, half.next_up()), unit));
    // The resolution conjunct binds B_u: q = 2^10 B is unresolved and silent
    // while B_f is below the criterion; one step below resolves and fires.
    let at_noise = 1.0 / 1024.0;
    assert!(!arc_group_fires(bending, 1.0, &bound(0.0, at_noise), unit));
    assert!(arc_group_fires(bending, 1.0, &bound(0.0, at_noise.next_down()), unit));
    // R-2: B_f above the criterion fires with no resolution conjunct, at its
    // exact boundary, even where q <= 2^10 B.
    assert!(arc_group_fires(bending, 1.0, &bound(CRITERION.next_up(), 0.5), unit));
    assert!(!arc_group_fires(bending, 1.0, &bound(CRITERION, 0.5), unit));
    assert!(arc_group_fires(ArcRowGroup::Torsion, 1.0, &bound(0.5, 0.0), unit));
    assert!(!arc_group_fires(ArcRowGroup::Torsion, 1.0, &bound(CRITERION, 0.0), unit));
    // A binary64 `B > 1e-9 q` would not fire at B = fl(1e-9) (M18's constant).
    assert!(arc_group_fires(ArcRowGroup::Axial, 1.0, &bound(1e-9, 0.0), unit));
    // Torsion, axial and shear ignore B_u: a resolved B_u above the
    // criterion fires bending only.
    assert!(arc_group_fires(bending, 1.0, &bound(0.0, 2e-9), unit));
    for group in [ArcRowGroup::Torsion, ArcRowGroup::Axial, ArcRowGroup::Shear] {
        assert!(!arc_group_fires(group, 1.0, &bound(0.0, 2e-9), unit), "{group:?}");
    }
    // The floor, on both paths: q = 2^-34 S* is above it; S* one step larger is not.
    let floor_scale = 1.0 / FLOOR;
    for b in [bound(0.0, 1e-6), bound(1e-6, 0.0)] {
        assert!(arc_group_fires(bending, 1.0, &b, scales(1.0, floor_scale)));
        assert!(!arc_group_fires(bending, 1.0, &b, scales(1.0, floor_scale.next_up())));
    }
    // S* below 2^-988, or no scale, is below the floor (DESIGN N-2).
    let tiny = f64::from_bits((1023 - 988) << 52);
    assert!(arc_group_fires(bending, tiny * FLOOR, &bound(1.0, 0.0), scales(tiny, tiny)));
    assert!(!arc_group_fires(bending, 1.0, &bound(1.0, 0.0), scales(0.0, tiny.next_down())));
    assert!(!arc_group_fires(bending, 1.0, &bound(1.0, 0.0), scales(0.0, 0.0)));
    // A non-finite bound fires (fail-closed), whatever the scale.
    assert!(arc_group_fires(bending, 1.0, &bound(f64::INFINITY, 0.0), scales(0.0, 0.0)));
    assert!(arc_group_fires(ArcRowGroup::Shear, 1.0, &bound(f64::NAN, 0.0), scales(0.0, 0.0)));
}

/// G2: S* is the moment scale for bending and torsion and the force scale
/// for axial and shear: a group with 2^-34 S_right <= q < 2^-34 S_wrong
/// fires only with the right scale.
#[test]
fn g2_each_group_uses_its_own_scale() {
    let (small, large) = (1.0, 1e12);
    let q = 2.0 * FLOOR * small; // above the small scale's floor, below the large one's
    let b = bound(1.0, 0.0);
    for group in [ArcRowGroup::Axial, ArcRowGroup::Shear] {
        assert!(arc_group_fires(group, q, &b, scales(small, large)), "{group:?}");
        assert!(!arc_group_fires(group, q, &b, scales(large, small)), "{group:?}");
    }
    for group in [ArcRowGroup::Bending, ArcRowGroup::Torsion] {
        assert!(arc_group_fires(group, q, &b, scales(large, small)), "{group:?}");
        assert!(!arc_group_fires(group, q, &b, scales(small, large)), "{group:?}");
    }
}

fn record(member: &str, bounds: Result<[[ArcGroupBound; 4]; 2], String>) -> ArcRecoveryRecord {
    ArcRecoveryRecord {
        member: member.to_string(),
        body: Some(0),
        q: [[1.0; 4]; 2],
        bounds,
    }
}

fn unit_scales() -> HashMap<usize, ActionScales> {
    HashMap::from([(0, scales(1.0, 1.0))])
}

/// G3: an unavailable bound fires, naming the member and the reason.
#[test]
fn g3_an_unavailable_bound_fires() {
    let quiet = [[bound(0.0, 0.0); 4]; 2];
    assert!(arc_recovery_finding(&[record("A", Ok(quiet))], &unit_scales(), "c").is_none());
    let finding = arc_recovery_finding(
        &[record("A", Ok(quiet)), record("B", Err("ResidualNotContracting".into()))],
        &unit_scales(),
        "c",
    )
    .expect("an unavailable bound fires");
    assert_eq!(finding.fired, ["B: the formation bound is unavailable (ResidualNotContracting)"]);
    assert!(finding.sentence.starts_with(&format!("{ARC}: load case c is Sensitive because 1 realized-arc end group(s)")));
}

/// G4: the sentence names at most six groups, then the count of the rest,
/// with the group, end, q, B, B_f and S* of each.
#[test]
fn g4_the_sentence_names_six_then_counts() {
    let loud = [[bound(1.0, 0.0); 4]; 2];
    let finding = arc_recovery_finding(&[record("A", Ok(loud))], &unit_scales(), "c").unwrap();
    assert_eq!(finding.fired.len(), 8);
    assert!(finding.sentence.ends_with("; and 2 more"), "{}", finding.sentence);
    assert!(finding.sentence.contains(
        "A.i.bending: q=1e0 N*m, B=1e0 N*m, B_f=1e0 N*m, S*=1e0 N*m; A.i.torsion: q=1e0 N*m"
    ));
    assert!(finding.sentence.contains("A.i.axial: q=1e0 N, B=1e0 N, B_f=1e0 N, S*=1e0 N"));
    assert!(!finding.sentence.contains("A.j.shear"));
}

/// G5: a single finding is unchanged by the merge (the straight R-b'
/// sentence stays byte for byte); several are joined straight, arc, short
/// bend, one space apart.
#[test]
fn g5_merge_keeps_a_single_sentence_and_orders_several() {
    let straight = recovery_finding(
        &[RecoveryRecord {
            member: "M1".into(),
            body: Some(0),
            ends: [(1.0, Ok(1e-6)), (1.0, Ok(0.0))],
        }],
        &unit_scales(),
        "c",
    )
    .unwrap();
    let arc = arc_recovery_finding(&[record("A", Err("x".into()))], &unit_scales(), "c").unwrap();
    let short = short_bend_finding(
        &[ShortBend {
            component: "component:A".into(),
            pipe: "A".into(),
            ratio: 1e-2,
        }],
        "c",
    )
    .unwrap();
    let alone = merge(vec![straight.clone()]).unwrap();
    assert_eq!(alone.sentence, straight.sentence);
    assert_eq!(alone.guard, formation_guard::Guard::Recovery);
    let all = merge(vec![straight.clone(), arc.clone(), short.clone()]).unwrap();
    assert_eq!(all.sentence, format!("{} {} {}", straight.sentence, arc.sentence, short.sentence));
    assert_eq!(all.fired.len(), 3);
    assert!(merge(Vec::new()).is_none());
}

/// G6: the scales include an arc's rows unless the arc is excluded, and
/// the force scale couples the moment scale over the body's extent.
#[test]
fn g6_scales_include_guarded_arcs_and_couple_force_and_moment() {
    let bodies = Bodies::new(&[[0.0; 3], [2.0, 0.0, 0.0]], &[(0, 1)]);
    let entities = HashMap::from([("S".to_string(), 0), ("A".to_string(), 0)]);
    let row = |entity: &str, kind: &str, unit: &str, value: f64| ResultItem {
        id: format!("{entity}:{kind}"),
        kind: kind.into(),
        value,
        unit: unit.into(),
        entity_ref: entity.into(),
        basis_ref: None,
        source_result_refs: Vec::new(),
        metadata: None,
    };
    let rows = [
        row("S", "element_local_axial_force", "N", 3.0),
        row("S", "element_local_bending_moment_y", "N*m", 1.0),
        row("A", "element_local_shear_force_y", "N", -10.0),
        row("A", "element_local_torsional_moment", "kN*m", 0.05),
    ];
    let included = action_scales(&rows, &entities, &HashSet::new(), &bodies)[&0];
    // S(force) = 10, S(moment) = 50, L_b = 2: fo = max(10, 25), mo = max(50, 20).
    assert_eq!(included, scales(25.0, 50.0));
    let excluded = action_scales(&rows, &entities, &HashSet::from(["A".to_string()]), &bodies)[&0];
    assert_eq!(excluded, scales(3.0, 6.0));
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

/// The observations of this request's runs (taken, so they do not leak).
fn observations(request: &Value) -> Vec<observed::ArcObservation> {
    observed::take(&project_of(request))
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

/// P1 (load-bearing): the arc analogue of T11. ARC-INPLANE-M-G1e80 (R = 1 m)
/// publishes a bending pair that differs from the exact f* by more than
/// RD(1e-9) q while its ordinary report is Passed; the arc guard alone
/// demotes it, naming M2's bending group. Removing the guard leaves it Passed.
#[test]
fn p1_arc_bending_breach_is_demoted_by_the_arc_guard() {
    let label = "ARC-INPLANE RF-CANCEL-M-G1e80-GnG-INPLANE R=1 side=1";
    let request = arc_inplane("RF-CANCEL-M-G1e80-GnG-INPLANE", 1.0, 1.0, "invented:o13:p1");
    // The captured entry refuses G1e80's unsafe integral floats; typed only.
    for entry in [Entry::Typed] {
        for mode in MODES {
            let tag = format!("P1 {entry:?} {mode:?}");
            let envelope = solved(entry, &request, mode);
            assert_ordinary_passed(&envelope, "case", &tag);
            let exact = exact_rows(label, mode, "M2");
            let published = bending_rows(&envelope, "M2");
            let breach = (0..2).any(|end| {
                let [y, z] = published[end];
                let q = y.hypot(z);
                let error = (y - exact[6 * end + 4]).abs().max((z - exact[6 * end + 5]).abs());
                error > CRITERION * q
            });
            assert!(breach, "{tag}: precondition, a published bending row breaches its criterion");
            assert_demoted(&envelope, "case", ARC, &tag);
            let message = &integrity(&envelope, "case").message;
            assert!(message.contains("M2.") && message.contains(".bending: q="), "{tag}: {message}");
            assert!(!message.contains(RECOVERY_STRAIGHT), "{tag}: straight R-b' is silent");
            assert!(!message.contains(SHORT), "{tag}: the short-bend rule is silent");
        }
    }
    observations(&request);
}

/// R-b''s own sentence prefix (the arc sentence's is distinct).
const RECOVERY_STRAIGHT: &str = "S11-G recovery guard (R-b'):";

/// P2: realistic arcs are not demoted. Each case is Passed with neither
/// sentence, and every guarded group's B stays below a tenth of RD(1e-9) q
/// where it is above the floor.
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
        for entry in [Entry::Captured, Entry::Typed] {
            for mode in MODES {
                let tag = format!("{} {entry:?} {mode:?}", project_of(request));
                let envelope = solved(entry, request, mode);
                let d = integrity(&envelope, "case");
                assert_eq!(d.code, "NUMERICAL_INTEGRITY_CHECKS_PASSED", "{tag}: {}", d.message);
                assert!(!d.message.contains(ARC) && !d.message.contains(SHORT), "{tag}");
            }
        }
        let seen = observations(request);
        assert_eq!(seen.len(), 4, "{}", project_of(request));
        for observation in seen {
            for record in &observation.records {
                let scale = observation.scales[&record.body.unwrap()];
                let bounds = record.bounds.as_ref().expect("the bound is available");
                for end in 0..2 {
                    for (slot, group) in ArcRowGroup::ALL.into_iter().enumerate() {
                        let (q, b) = (record.q[end][slot], bounds[end][slot]);
                        let s_star = match group {
                            ArcRowGroup::Bending | ArcRowGroup::Torsion => scale.moment,
                            _ => scale.force,
                        };
                        if q < FLOOR * s_star {
                            continue;
                        }
                        let total = if group == ArcRowGroup::Bending { b.with_resolution() } else { b.formation };
                        assert!(
                            total < 0.1 * CRITERION * q,
                            "{} {group:?} end {end}: B {total:e} against q {q:e}",
                            project_of(request)
                        );
                    }
                }
            }
        }
    }
}

/// P4 (I5 ruling 1 for arcs): the load-row guard and the arc guard fire on
/// the same case. The load-row sentence stands alone; the arc finding exists
/// (test view) and its amendment is a no-op.
#[test]
fn p4_both_guards_fire_and_the_load_row_sentence_stands() {
    let mut request = arc_inplane("RF-CANCEL-M-G1e80-GnG-INPLANE", 1.0, 1.0, "invented:o13:p4");
    udl_body(&mut request["model"], "u", 100.0);
    let view = guard_view(&request, 0);
    assert!(view.any_fires(), "precondition: the load-row guard fires");
    for mode in MODES {
        let tag = format!("P4 {mode:?}");
        let envelope = solved(Entry::Typed, &request, mode);
        assert_ordinary_passed(&envelope, "case", &tag);
        assert_demoted(&envelope, "case", LOAD_ROW, &tag);
        assert!(!integrity(&envelope, "case").message.contains(ARC), "{tag}: the arc amendment is a no-op");
    }
    let seen = observations(&request);
    assert_eq!(seen.len(), 2);
    assert!(seen.iter().all(|o| o.finding.is_some()), "the arc guard fired too");
}

/// P5 (the no-op rule): a case that is already Sensitive by the solver is
/// left byte for byte although the arc guard or the short-bend rule fires.
/// (a) The L line at 1 degree (R phi/L = 1.3e-3, below the short-bend
/// limit). (b) T4-I34's nodal-only arc at k = 1e9, whose formation error is
/// far above the criterion: the arc guard fires (test view).
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
                assert!(!d.message.contains(ARC) && !d.message.contains(SHORT), "{tag}");
            }
        }
    }
    observations(&line);
    let seen = observations(&stiff);
    assert_eq!(seen.len(), 4);
    assert!(seen.iter().all(|o| o.finding.is_some()), "the arc guard fired");
}

/// P6 (the scale extension): ARC-ONLY-INPLANE-M, M1 and M2 both arcs, one
/// body with no straight member. Under entity rule 2b its S* would be 0 and
/// the floor would silence every group; with 2b lifted the arc guard demotes.
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
    for mode in MODES {
        let tag = format!("P6 {mode:?}");
        let envelope = solved(Entry::Typed, &request, mode);
        assert_ordinary_passed(&envelope, "case", &tag);
        assert_demoted(&envelope, "case", ARC, &tag);
    }
    for observation in observations(&request) {
        assert!(observation.scales.values().all(|s| s.moment > 0.0), "the arc rows set S*");
    }
}

/// P8: R-b''s accepted structural-zero residual stays silent on arcs: a
/// stress-free arc (anchored at one end, guided at the other, uniform
/// temperature change) publishes rows of rounding size against an exact
/// zero and stays Passed.
#[test]
fn p8_a_stress_free_arc_stays_passed() {
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
        assert_eq!(d.code, "NUMERICAL_INTEGRITY_CHECKS_PASSED", "{tag}: {}", d.message);
    }
    observations(&request);
}

/// The scale extension at the call site (entity rule 2b lifted): a realized
/// arc's rows are excluded from S* only when the arc has no record.
#[test]
fn only_an_unguarded_arc_is_excluded_from_the_scales() {
    let request = l_line("invented:o13:2b", [0.0; 3], 1.5, 0.2286, -450.0, 0.0);
    let built = prepared(&request).built;
    let bends: HashMap<usize, &CurvedBendMacroBuild> =
        built.curved_bend_elements.iter().map(|b| (b.pipe_index, b)).collect();
    assert_eq!(unguarded_arcs(&bends, &[]), HashSet::from(["BC".to_string()]));
    let record = ArcRecoveryRecord {
        member: "BC".into(),
        body: Some(0),
        q: [[0.0; 4]; 2],
        bounds: Err("x".into()),
    };
    assert!(unguarded_arcs(&bends, &[record]).is_empty());
}

/// The record's q: |N|, hypot(Vy, Vz), |T| and hypot(My, Mz) of the
/// published rows, per end (FK's correct norm, as R-b').
#[test]
fn arc_record_q_is_each_group_norm() {
    let request = l_line("invented:o13:q", [0.0; 3], 1.5, 0.2286, -450.0, 0.0);
    let built = prepared(&request).built;
    let bend = &built.curved_bend_elements[0];
    let pipe = &built.pipes[bend.pipe_index];
    let published: Vec<f64> = (1..=12).map(|k| k as f64).collect();
    let record = arc_recovery_record(
        bend,
        pipe,
        &vec![0.0; built.nodes.len() * DOF_PER_NODE],
        &[],
        &[],
        &published,
        Some(0),
    );
    assert_eq!(record.member, "BC");
    assert_eq!(
        record.q,
        [[norm2(5.0, 6.0), 4.0, 1.0, norm2(2.0, 3.0)], [norm2(11.0, 12.0), 10.0, 7.0, norm2(8.0, 9.0)]]
    );
    assert!(record.bounds.is_ok());
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
                assert!(!message.contains(ARC), "{tag}: the arc guard is silent");
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
        observations(&request);
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
        observations(&request);
    }
}

// ------------------------------------------------------------- source pin

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
/// recovery loop records each realized arc's ends through
/// `arc_recovery_record`, which bounds them with FK's
/// `bound_end_actions`; the case's finding comes from
/// `arc_recovery_finding` and `short_bend_finding` merged with R-b''s, and
/// `arc_recovery_finding` decides each group with `arc_group_fires`, which
/// calls `rb_prime_fires` and the exact criterion test.
#[test]
fn p7_source_pin_ties_the_call_site_to_the_tested_predicate() {
    let lib = product_text(include_str!("../lib.rs"));
    let guard = product_text(include_str!("../formation_guard.rs"));
    let solve = body(&lib, "solve_load_case_observed");
    let record_at = solve
        .find("arc_records.push(arc_recovery_record(")
        .expect("the recovery loop records each arc");
    let branch = &solve[..record_at];
    assert!(
        branch.rfind("} else if let (None, Some(bend)) = (recovered_member, macro_bend) {")
            > branch.rfind("recovery_records.push(formation_guard::RecoveryRecord {"),
        "the arc record is formed in the realized-arc branch after the straight one"
    );
    let site = &solve[record_at..];
    for call in [
        "&unguarded_arcs(&curved_bends_by_pipe, &arc_records),",
        "formation_guard::recovery_finding(&recovery_records, &scales, &load_case.id)",
        "formation_guard::arc_recovery_finding(&arc_records, &scales, &load_case.id)",
        "formation_guard::short_bend_finding(&short_bends, &load_case.id)",
        "formation_guard::merge(findings.into_iter().flatten().collect())",
        "formation_guard::amend_integrity_report(",
    ] {
        assert!(site.contains(call), "the call site makes {call}");
    }
    let record = body(&lib, "arc_recovery_record");
    assert!(record.contains("certify_curved_recovery(") && record.contains(".bound_end_actions(&u, free_expansions, &loads, &published)"));
    let finding = body(&guard, "arc_recovery_finding");
    assert!(finding.contains("if arc_group_fires(group, q, bound, scale) {"));
    let predicate = body(&guard, "arc_group_fires");
    assert!(predicate.contains("rb_prime_fires(q, b, s_star)"));
    assert!(predicate.contains("(exceeds_criterion(bound.formation, q) && above_floor(q, s_star))"));
    assert!(body(&guard, "rb_prime_fires").contains("exceeds_criterion(bound, q) && resolved && above_floor(q, s_star_moment)"));
}

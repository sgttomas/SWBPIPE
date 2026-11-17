//! DEL-04-10 AC-001 and AC-002 for the combinations (unit S4): each named refusal, the absence
//! of any default, mutations of the inputs and choices, and repeat-run and permutation identity.
use open_pipe_stress_response_spectrum::combination_json::combine_json;
use open_pipe_stress_response_spectrum::{
    combine, ClusterIndex, CombinationRequest, Direction, DirectionalRule, DisplacementComponent, End, ForceComponent,
    Frame, MissingMass, ModalContribution, ModalMode, ModalRule, ModeIndex, Quantity, RefusalCode, RigidContribution,
    RigidRule,
};
use serde_json::{json, Value};

fn rows() -> Vec<Quantity> {
    vec![
        Quantity::NodeDisplacement { node: "N2".into(), component: DisplacementComponent::Uy },
        Quantity::ElementEndForce { member: "P1".into(), end: End::I, component: ForceComponent::Mz, frame: Frame::MemberAxes },
    ]
}

/// Three modes: a cluster of two unequal frequencies (modes 1 and 2) and a single mode 3, two
/// directions, the correction applied with f_r between the clusters. Invented values.
fn base() -> CombinationRequest {
    let contribution = |mode, direction, values: [f64; 2]| ModalContribution { mode: ModeIndex(mode), direction, values: values.to_vec() };
    CombinationRequest {
        damping_ratio: 0.05,
        modal_rule: ModalRule::Cqc,
        directional_rule: DirectionalRule::HundredFortyForty,
        missing_mass: MissingMass::Applied { rigid_frequency_hz: 20.0, rule: RigidRule::Srss },
        directions: vec![Direction::X, Direction::Z],
        modes: vec![
            ModalMode { mode: ModeIndex(1), cluster: ClusterIndex(1), frequency_hz: 4.0 },
            ModalMode { mode: ModeIndex(2), cluster: ClusterIndex(1), frequency_hz: 4.000001 },
            ModalMode { mode: ModeIndex(3), cluster: ClusterIndex(2), frequency_hz: 4.3 },
            ModalMode { mode: ModeIndex(4), cluster: ClusterIndex(3), frequency_hz: 31.0 },
        ],
        rows: rows(),
        periodic: vec![
            contribution(1, Direction::X, [2.0e-3, 150.0]),
            contribution(2, Direction::X, [-1.5e-3, -40.0]),
            contribution(3, Direction::X, [-0.8e-3, 90.0]),
            contribution(1, Direction::Z, [0.4e-3, -10.0]),
            contribution(2, Direction::Z, [0.3e-3, 12.0]),
            contribution(3, Direction::Z, [1.1e-3, 30.0]),
        ],
        rigid: vec![
            RigidContribution { direction: Direction::X, periodic_clusters_left_out: vec![ClusterIndex(1), ClusterIndex(2)], values: vec![1.0e-4, -20.0] },
            RigidContribution { direction: Direction::Z, periodic_clusters_left_out: vec![ClusterIndex(1), ClusterIndex(2)], values: vec![-2.0e-4, 5.0] },
        ],
    }
}

fn codes(request: &CombinationRequest) -> Vec<&'static str> {
    combine(request).expect_err("refused").iter().map(|r| r.code.as_str()).collect()
}

#[test]
fn the_base_request_combines() {
    let c = combine(&base()).unwrap();
    assert_eq!(c.clusters.len(), 3);
    assert_eq!(c.cluster_sums.len(), 4); // two periodic clusters × two directions
    // Cluster 1's sum under X is 2.0e-3 − 1.5e-3, with signs.
    assert_eq!(c.cluster_sums[0].values[0], 2.0e-3 + -1.5e-3);
    // N2.uy: under Z both cluster sums are positive (0.7e-3 and 1.1e-3) and outweigh X's.
    let parts = &c.rows[0].per_direction;
    assert!(parts[1].combined.value() > parts[0].combined.value());
    assert_eq!(c.rows[0].governing_direction, Some(Direction::Z));
}

#[test]
fn an_empty_extracted_set_without_correction_combines_to_zero() {
    let mut request = base();
    request.missing_mass = MissingMass::NotApplied;
    request.modes.clear();
    request.periodic.clear();
    request.rigid.clear();
    for modal in ModalRule::ALL {
        for directional in DirectionalRule::ALL {
            request.modal_rule = modal;
            request.directional_rule = directional;
            let result = combine(&request).unwrap();
            assert!(result.clusters.is_empty());
            assert!(result.cluster_sums.is_empty());
            assert_eq!(result.rows.len(), request.rows.len());
            for row in &result.rows {
                assert_eq!(row.combined.value(), 0.0);
                assert_eq!(row.per_direction.len(), request.directions.len());
                for part in &row.per_direction {
                    assert_eq!(part.periodic.value(), 0.0);
                    assert_eq!(part.combined.value(), 0.0);
                    assert!(part.rigid.is_none());
                }
            }
            let input = open_pipe_stress_response_spectrum::combination_json::request_to_json(
                &request, "Invented empty-set combination input; not a spectrum solve result.");
            assert_eq!(combine_json(&input.to_string()).unwrap()["outcome"], "combined");
        }
    }
    // Empty extraction does not permit a contribution for a mode absent from the roster.
    request.periodic.push(base().periodic.remove(0));
    assert!(codes(&request).contains(&"SPECTRUM_CONTRIBUTION_INVALID"));
}

#[test]
fn every_refusal_is_named() {
    let mut r = base();
    r.damping_ratio = 0.0;
    assert_eq!(codes(&r), ["SPECTRUM_CQC_DAMPING_UNSUPPORTED"]);
    r.modal_rule = ModalRule::Srss;
    assert!(combine(&r).is_ok(), "SRSS at zero damping is admitted");
    for zeta in [0.21, -0.01, f64::NAN, f64::INFINITY] {
        let mut r = base();
        r.damping_ratio = zeta;
        assert_eq!(codes(&r), ["SPECTRUM_DAMPING_OUT_OF_RANGE"], "{zeta}");
    }
    for fr in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        let mut r = base();
        r.missing_mass = MissingMass::Applied { rigid_frequency_hz: fr, rule: RigidRule::Srss };
        assert!(codes(&r).contains(&"SPECTRUM_RIGID_FREQUENCY_INVALID"), "{fr}");
    }
    let mut r = base();
    r.directions = vec![];
    assert!(codes(&r).contains(&"SPECTRUM_DIRECTION_INVALID"));
    let mut r = base();
    r.directions = vec![Direction::X, Direction::Z, Direction::X];
    assert_eq!(codes(&r), ["SPECTRUM_DIRECTION_INVALID"]);

    // Modal metadata: not 1..n, not ascending, a cluster that does not chain, a bad frequency.
    let metadata: [fn(&mut CombinationRequest); 6] = [
        |r| r.modes[1].mode = ModeIndex(5),
        |r| r.modes[2].frequency_hz = 3.0,
        |r| r.modes[2].cluster = ClusterIndex(3),
        |r| r.modes[0].cluster = ClusterIndex(2),
        |r| r.modes[0].frequency_hz = f64::NAN,
        |r| r.modes.clear(),
    ];
    for (i, mutate) in metadata.iter().enumerate() {
        let mut r = base();
        mutate(&mut r);
        assert!(codes(&r).iter().all(|c| *c == "SPECTRUM_MODAL_METADATA_INVALID"), "metadata mutation {i}: {:?}", codes(&r));
    }

    // Rows: none, repeated, an id with a dot.
    let mut r = base();
    r.rows.push(r.rows[0].clone());
    assert!(codes(&r).contains(&"SPECTRUM_ROW_INVALID"));
    let mut r = base();
    r.rows[0] = Quantity::NodeDisplacement { node: "N.2".into(), component: DisplacementComponent::Uy };
    assert!(codes(&r).contains(&"SPECTRUM_ROW_INVALID"));

    // Contributions: nonfinite, wrong length, missing, repeated, unknown mode, unrequested direction.
    let contributions: [fn(&mut CombinationRequest); 6] = [
        |r| r.periodic[0].values[1] = f64::NAN,
        |r| r.periodic[0].values.push(1.0),
        |r| {
            r.periodic.remove(4);
        },
        |r| r.periodic.push(r.periodic[0].clone()),
        |r| r.periodic[0].mode = ModeIndex(9),
        |r| r.periodic[0].direction = Direction::Y,
    ];
    for (i, mutate) in contributions.iter().enumerate() {
        let mut r = base();
        mutate(&mut r);
        assert!(codes(&r).contains(&"SPECTRUM_CONTRIBUTION_INVALID"), "contribution mutation {i}: {:?}", codes(&r));
    }
    // A contribution of a rigid cluster's mode.
    let mut r = base();
    r.periodic.push(ModalContribution { mode: ModeIndex(4), direction: Direction::X, values: vec![1.0, 1.0] });
    assert_eq!(codes(&r), ["SPECTRUM_CONTRIBUTION_NOT_PERIODIC"]);

    // Rigid responses: missing, without the correction, nonfinite, premise mismatch.
    let mut r = base();
    r.rigid.pop();
    assert_eq!(codes(&r), ["SPECTRUM_RIGID_RESPONSE_INVALID"]);
    let mut r = base();
    r.missing_mass = MissingMass::NotApplied;
    r.periodic.push(ModalContribution { mode: ModeIndex(4), direction: Direction::X, values: vec![0.0, 0.0] });
    r.periodic.push(ModalContribution { mode: ModeIndex(4), direction: Direction::Z, values: vec![0.0, 0.0] });
    assert_eq!(codes(&r), ["SPECTRUM_RIGID_RESPONSE_INVALID", "SPECTRUM_RIGID_RESPONSE_INVALID"]);
    let mut r = base();
    r.rigid[1].values[0] = f64::INFINITY;
    assert_eq!(codes(&r), ["SPECTRUM_RIGID_RESPONSE_INVALID"]);
    let mut r = base();
    r.rigid[0].periodic_clusters_left_out = vec![ClusterIndex(1)];
    assert_eq!(codes(&r), ["SPECTRUM_RIGID_RESPONSE_PREMISE_MISMATCH"]);

    // A value whose combination leaves binary64 is refused, not published.
    let mut r = base();
    r.modal_rule = ModalRule::AbsoluteSum;
    r.periodic[0].values[1] = f64::MAX;
    r.periodic[2].values[1] = f64::MAX;
    assert!(codes(&r).contains(&"SPECTRUM_COMBINATION_NONFINITE"));
}

#[test]
fn a_refusal_names_its_object_and_field() {
    let mut r = base();
    r.periodic[0].values[1] = f64::NAN;
    let refusal = &combine(&r).unwrap_err()[0];
    assert_eq!(refusal.code, RefusalCode::ContributionInvalid);
    assert_eq!(refusal.object, "mode:1 X");
    assert_eq!(refusal.field.as_deref(), Some("P1.i.Mz_m"));
}

#[test]
fn each_choice_changes_the_result_as_its_rule_predicts() {
    let value = |r: &CombinationRequest| combine(r).unwrap().rows[1].combined.value();
    let part = |r: &CombinationRequest, d: usize| combine(r).unwrap().rows[1].per_direction[d];
    let b = base();
    // Absolute sum bounds every other modal rule of the same cluster sums.
    let mut modal = Vec::new();
    for rule in ModalRule::ALL {
        let mut r = b.clone();
        r.modal_rule = rule;
        r.missing_mass = MissingMass::NotApplied;
        r.rigid.clear();
        r.periodic.extend([Direction::X, Direction::Z].map(|d| ModalContribution { mode: ModeIndex(4), direction: d, values: vec![0.0, 0.0] }));
        modal.push((rule, part(&r, 0).periodic.value()));
    }
    let abs = modal.iter().find(|(r, _)| *r == ModalRule::AbsoluteSum).unwrap().1;
    // P1.i.Mz_m under X: cluster sums 110 and 90, so SRSS √(110² + 90²), absolute sum 200, and
    // grouping and ten percent (4.0 and 4.3 Hz are within 10 %) √(110² + 90² + 2·110·90) = 200.
    assert_eq!(abs, 200.0);
    let srss = modal.iter().find(|(r, _)| *r == ModalRule::Srss).unwrap().1;
    assert_eq!(srss, (110.0f64 * 110.0 + 90.0 * 90.0).sqrt());
    for (rule, v) in &modal {
        assert!(*v <= abs, "{rule:?}");
        assert!(*v >= srss || *rule == ModalRule::Cqc, "{rule:?}");
    }
    // Grouping against ten percent: f = 4.0, 4.3, 4.7 chain (4.7 within 10 % of 4.3, not of 4.0).
    let mut chain = b.clone();
    chain.missing_mass = MissingMass::NotApplied;
    chain.rigid.clear();
    chain.modes[3] = ModalMode { mode: ModeIndex(4), cluster: ClusterIndex(3), frequency_hz: 4.7 };
    chain.periodic.extend([Direction::X, Direction::Z].map(|d| ModalContribution { mode: ModeIndex(4), direction: d, values: vec![0.0, 50.0] }));
    chain.modal_rule = ModalRule::Grouping;
    let grouping = part(&chain, 0).periodic.value();
    chain.modal_rule = ModalRule::TenPercent;
    let ten = part(&chain, 0).periodic.value();
    assert_eq!(grouping, (110.0f64 * 110.0 + 90.0 * 90.0 + 50.0 * 50.0 + 2.0 * 110.0 * 90.0).sqrt());
    assert_eq!(ten, (110.0f64 * 110.0 + 90.0 * 90.0 + 50.0 * 50.0 + 2.0 * 110.0 * 90.0 + 2.0 * 90.0 * 50.0).sqrt());
    // The rigid rule: SRSS against the absolute sum.
    let mut r = b.clone();
    let close = |a: f64, b: f64| (a - b).abs() <= 2.0 * f64::EPSILON * b;
    let p = part(&r, 0);
    assert!(close(p.combined.value(), (p.periodic.value().powi(2) + 20.0f64 * 20.0).sqrt()));
    r.missing_mass = MissingMass::Applied { rigid_frequency_hz: 20.0, rule: RigidRule::AbsoluteSum };
    let p = part(&r, 0);
    assert_eq!(p.combined.value(), p.periodic.value() + 20.0);
    // The directional rule.
    let (x, z) = (part(&b, 0).combined.value(), part(&b, 1).combined.value());
    assert_eq!(value(&b), x.max(z) + 0.4 * x.min(z));
    let mut r = b.clone();
    r.directional_rule = DirectionalRule::Srss;
    assert!(close(value(&r), (x * x + z * z).sqrt()));
    r.directional_rule = DirectionalRule::AbsoluteSum;
    assert_eq!(value(&r), x + z);
    // Moving f_r below cluster 2's frequency makes it rigid: its contributions are then refused.
    let mut r = b.clone();
    r.missing_mass = MissingMass::Applied { rigid_frequency_hz: 4.1, rule: RigidRule::Srss };
    assert!(codes(&r).contains(&"SPECTRUM_CONTRIBUTION_NOT_PERIODIC"));
    // f_r equal to a cluster's lowest frequency makes it rigid (periodic means strictly below).
    r.missing_mass = MissingMass::Applied { rigid_frequency_hz: 4.0, rule: RigidRule::Srss };
    assert!(codes(&r).contains(&"SPECTRUM_CONTRIBUTION_NOT_PERIODIC"));
    // Moving a contribution to another cluster changes the cluster sums.
    let mut r = b.clone();
    r.modes[1].cluster = ClusterIndex(2);
    r.modes[2].cluster = ClusterIndex(2);
    r.modes[3].cluster = ClusterIndex(3);
    assert_ne!(combine(&r).unwrap().cluster_sums, combine(&b).unwrap().cluster_sums);
}

#[test]
fn repeated_runs_and_permuted_inputs_give_identical_bytes() {
    let to_json = |r: &CombinationRequest| format!("{:?}", combine(r).unwrap());
    let b = base();
    let first = to_json(&b);
    assert_eq!(first, to_json(&b));
    // The order of the requested directions and of the supplied entries does not matter.
    let mut p = b.clone();
    p.directions.reverse();
    p.periodic.reverse();
    p.rigid.reverse();
    p.rigid[0].periodic_clusters_left_out.reverse();
    assert_eq!(first, to_json(&p));
}

/// A minimal JSON input document; `edit` removes or changes members.
fn document(edit: impl FnOnce(&mut Value)) -> String {
    let mut v = json!({
        "schema": "openpipestress.spectrum_combination.input/v1",
        "provenance": "invented values for a refusal test",
        "damping_ratio": 0.05,
        "modal_combination": "SRSS",
        "directional_combination": "SRSS",
        "missing_mass": {"applied": false},
        "directions": ["X"],
        "modes": [{"mode": 1, "cluster": 1, "frequency_hz": 2.0}],
        "rows": [{"key": "N2.uy", "quantity": "node_displacement", "object": "N2", "component": "uy", "unit": "m", "frame": "global"}],
        "contributions": [{"mode": 1, "direction": "X", "sign_status": "signed_contribution", "values": [-0.003]}],
        "rigid_responses": []
    });
    edit(&mut v);
    v.to_string()
}

fn json_codes(text: &str) -> Vec<String> {
    let out = combine_json(text).unwrap();
    assert_eq!(out["outcome"], "refused", "{out}");
    out["refusals"].as_array().unwrap().iter().map(|r| r["code"].as_str().unwrap().to_string()).collect()
}

#[test]
fn the_json_entry_supplies_no_default() {
    let ok = combine_json(&document(|_| {})).unwrap();
    assert_eq!(ok["outcome"], "combined");
    assert_eq!(ok["rows"][0]["combined"], 0.003);
    assert_eq!(ok["rows"][0]["sign_status"], "unsigned_combined_magnitude");
    assert_eq!(ok["provenance"], "invented values for a refusal test");
    for member in ["modal_combination", "directional_combination", "missing_mass"] {
        for edit in [0, 1] {
            let codes = json_codes(&document(|v| {
                if edit == 0 {
                    v.as_object_mut().unwrap().remove(member);
                } else {
                    v[member] = Value::Null;
                }
            }));
            assert_eq!(codes, ["SPECTRUM_METHOD_CHOICE_MISSING"], "{member}");
        }
    }
    assert_eq!(json_codes(&document(|v| v["modal_combination"] = json!("SRSS_PLUS"))), ["SPECTRUM_METHOD_CHOICE_MISSING"]);
    assert_eq!(json_codes(&document(|v| v["missing_mass"] = json!({"applied": true, "rigid_frequency_hz": 33.0}))),
        ["SPECTRUM_METHOD_CHOICE_MISSING"]);
    assert_eq!(json_codes(&document(|v| v["missing_mass"] = json!({"applied": true, "rigid_combination": "SRSS"}))),
        ["SPECTRUM_RIGID_FREQUENCY_INVALID"]);
    assert_eq!(json_codes(&document(|v| v["contributions"][0]["sign_status"] = json!("unsigned_combined_magnitude"))),
        ["SPECTRUM_CONTRIBUTION_INVALID"]);
    assert_eq!(json_codes(&document(|v| v["rows"][0]["unit"] = json!("mm"))), ["SPECTRUM_ROW_INVALID"]);
    assert_eq!(json_codes(&document(|v| v["rows"][0]["frame"] = json!("member_axes"))), ["SPECTRUM_ROW_INVALID"]);
    assert_eq!(json_codes(&document(|v| v["rows"][0]["key"] = json!("N2.ux"))), ["SPECTRUM_ROW_INVALID"]);
    assert_eq!(json_codes(&document(|v| v["directions"] = json!(["W"]))), ["SPECTRUM_DIRECTION_INVALID"]);
    // A document that does not decode is an error, not an output: a missing damping ratio, an
    // unknown member, the wrong schema.
    assert!(combine_json(&document(|v| {
        v.as_object_mut().unwrap().remove("damping_ratio");
    }))
    .is_err());
    assert!(combine_json(&document(|v| v["extra"] = json!(1))).is_err());
    assert!(combine_json(&document(|v| v["schema"] = json!("openpipestress.spectrum_result/v1"))).is_err());
}

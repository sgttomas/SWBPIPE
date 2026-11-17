//! DEL-04-10 AC-002, the combinations (unit S4), against the frozen `f2_spectrum` references at
//! REQ-008's verification tolerance (tests/support: 1e-9 relative, no floor; an expected zero must
//! be produced exactly).
//!
//! The inputs are the references' own published signed contributions, modal metadata and rigid
//! responses, each read as its binary64. The product supplies none of them; these tests examine
//! the combinations only. The expected values are the references' combined magnitudes, derived
//! there independently of product code.
mod support;

use open_pipe_stress_response_spectrum::{
    combine, ClusterClass, ClusterIndex, CombinationRequest, Direction, DirectionalRule, MissingMass, ModalContribution,
    ModalMode, ModalRule, ModeIndex, Quantity, ReactionComponent, RigidContribution, RigidRule, DisplacementComponent,
    End, ForceComponent, Frame,
};
use serde_json::Value;
use support::*;

const C3: &str = include_str!("../../../../validation/references/f2_spectrum/discrete/c3_cantilever_3d.json");
const C4: &str = include_str!("../../../../validation/references/f2_spectrum/discrete/c4_close_pair.json");
const C7: &str = include_str!("../../../../validation/references/f2_spectrum/discrete/c7_repeated_pair.json");
const C9: &str = include_str!("../../../../validation/references/f2_spectrum/discrete/c9_pipe_diagonal.json");
const C10: &str = include_str!("../../../../validation/references/f2_spectrum/discrete/c10_cluster_witnesses.json");
const CHAIN: &str = include_str!("../../../../validation/references/f2_spectrum/closed_form/two_mass_chain.json");
const DIRECTIONAL: &str = include_str!("../../../../validation/references/f2_spectrum/closed_form/directional.json");

fn cases(text: &str) -> Vec<Value> {
    serde_json::from_str::<Value>(text).unwrap()["cases"].as_array().unwrap().clone()
}

fn rigid_rule(name: &str) -> Option<RigidRule> {
    match name {
        "OFF" => None,
        other => Some(RigidRule::parse(other).unwrap_or_else(|| panic!("rigid rule {other}"))),
    }
}

/// Checks every published combination of one analysis (global blocks, or a named sub-block):
/// the base request's per-direction and combined rows, every modal rule's periodic part, the
/// rigid magnitude, the correction-off periodic parts and every mutation.
fn check_analysis(case: &Value, analysis: &Value, block: Option<&str>, tally: &mut Tally) {
    let a = discrete_analysis(case, analysis, block);
    let source = match block {
        None => analysis,
        Some(name) => &analysis[name],
    };
    let request = &analysis["request"];
    let base_modal = modal_rule(request["modal_combination"].as_str().unwrap());
    let base_directional = directional_rule(request["directional_combination"].as_str().unwrap());
    let base_rigid = request["missing_mass"]["rigid_periodic_combination"].as_str().and_then(rigid_rule);
    let label = format!("{}{}", a.id, block.map(|b| format!(" {b}")).unwrap_or_default());

    let run = |modal, rigid, directional| {
        combine(&a.request(modal, rigid, directional)).unwrap_or_else(|r| panic!("{label}: refused {r:?}"))
    };

    // The base request.
    let base = combined_of(&run(base_modal, base_rigid, base_directional), &a.keys);
    for key in a.keys.iter() {
        let (combined, per) = &base[key];
        tally.check(&format!("{label} combined {key}"), *combined, expected(&source["combined"][key]));
        for d in &a.directions {
            let e = expected(&source["per_direction_combined"][d.as_str()][key]);
            tally.check(&format!("{label} {d} {key}"), per[d].2, e);
            if let Some(rm) = analysis.get("rigid_magnitude").filter(|_| block.is_none()) {
                tally.check(&format!("{label} rigid {d} {key}"), per[d].1.unwrap(), expected(&rm[d.as_str()][key]));
            }
        }
    }

    // Every modal rule's periodic part, with the base correction and with it off.
    for (field, rigid) in [("periodic_modal_combined", base_rigid), ("periodic_modal_combined_correction_off", None)] {
        let Some(blocks) = source.get(field).and_then(Value::as_object) else { continue };
        for (rule, per_direction) in blocks {
            let c = combined_of(&run(modal_rule(rule), rigid, base_directional), &a.keys);
            for key in a.keys.iter() {
                for d in &a.directions {
                    let e = expected(&per_direction[d.as_str()][key]);
                    tally.check(&format!("{label} {field} {rule} {d} {key}"), c[key].1[d].0, e);
                }
            }
        }
    }

    // Every mutation of the method choices.
    for m in source["mutations"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
        let modal = modal_rule(m["modal_combination"].as_str().unwrap());
        let rigid = rigid_rule(m["missing_mass"].as_str().unwrap());
        let directional = directional_rule(m["directional_combination"].as_str().unwrap());
        let c = combined_of(&run(modal, rigid, directional), &a.keys);
        for key in a.keys.iter() {
            let what = format!("{label} mutation {modal:?}/{rigid:?}/{directional:?} {key}");
            tally.check(&what, c[key].0, expected(&m["combined"][key]));
        }
    }
}

#[test]
fn discrete_references_every_rule_and_mutation() {
    let mut tally = Tally::default();
    let mut analyses = 0;
    for text in [C3, C4, C9, C10] {
        for case in cases(text) {
            for analysis in case["analyses"].as_array().unwrap() {
                check_analysis(&case, analysis, None, &mut tally);
                analyses += 1;
                if analysis.get("element_end_forces_member_axes").is_some() {
                    check_analysis(&case, analysis, Some("element_end_forces_member_axes"), &mut tally);
                }
            }
        }
    }
    assert_eq!(analyses, 6, "C3-BASE, C4-BASE, C9-Y, C9-CHAIN-Y, C10-W2, C10-W3");
    eprintln!("discrete combinations: {tally:?}");
    assert!(tally.compared > 10_000);
}

#[test]
fn c4_cqc_on_a_close_pair_with_opposite_signs() {
    let case = &cases(C4)[0];
    let analysis = &case["analyses"][0];
    let a = discrete_analysis(case, analysis, None);
    let c = combine(&a.request(ModalRule::Cqc, Some(RigidRule::AbsoluteSum), DirectionalRule::HundredFortyForty)).unwrap();
    let s = combine(&a.request(ModalRule::Srss, Some(RigidRule::AbsoluteSum), DirectionalRule::HundredFortyForty)).unwrap();
    let rho = c.correlation.as_ref().unwrap();
    let published = expected(&analysis["cqc_correlation"]["rho"][0][1]);
    assert!((rho[0][1] - published).abs() <= 1e-9 * published, "{} against {published}", rho[0][1]);
    let cqc = combined_of(&c, &a.keys);
    let srss = combined_of(&s, &a.keys);
    // Under X, N3.uy, A1.Fy and A1.Mx receive opposite-sign contributions from the pair, so CQC
    // is below SRSS; N3.ux, A1.Fx and A1.My receive same-sign ones, so CQC is above.
    for key in ["N3.uy", "A1.Fy", "A1.Mx"] {
        let (x1, x2) = (a.contributions[&(Direction::X, 1)][a.keys.iter().position(|k| k == key).unwrap()],
            a.contributions[&(Direction::X, 2)][a.keys.iter().position(|k| k == key).unwrap()]);
        assert!(x1 * x2 < 0.0, "{key}");
        assert!(cqc[key].1[&Direction::X].0 < srss[key].1[&Direction::X].0, "{key}");
    }
    for key in ["N3.ux", "A1.Fx", "A1.My"] {
        assert!(cqc[key].1[&Direction::X].0 > srss[key].1[&Direction::X].0, "{key}");
    }
    // A sign flip of one contribution changes CQC, as its rule predicts, and not SRSS.
    let mut flipped = a.request(ModalRule::Cqc, Some(RigidRule::AbsoluteSum), DirectionalRule::HundredFortyForty);
    let i = a.keys.iter().position(|k| k == "N3.uy").unwrap();
    let entry = flipped.periodic.iter_mut().find(|p| p.mode == ModeIndex(2) && p.direction == Direction::X).unwrap();
    entry.values[i] = -entry.values[i];
    let f = combined_of(&combine(&flipped).unwrap(), &a.keys);
    assert!(f["N3.uy"].1[&Direction::X].0 > srss["N3.uy"].1[&Direction::X].0);
}

#[test]
fn c9_chain_grouping_differs_from_ten_percent() {
    let case = &cases(C9)[1];
    let analysis = &case["analyses"][0];
    assert_eq!(analysis["request"]["id"], "C9-CHAIN-Y");
    let a = discrete_analysis(case, analysis, None);
    let g = combine(&a.request(ModalRule::Grouping, None, DirectionalRule::Srss)).unwrap();
    let t = combine(&a.request(ModalRule::TenPercent, None, DirectionalRule::Srss)).unwrap();
    // The groups are the reference's.
    let published: Vec<Vec<u32>> = analysis["grouping_groups"]["periodic"]
        .as_array()
        .unwrap()
        .iter()
        .map(|group| group.as_array().unwrap().iter().map(|unit| unit[0].as_u64().unwrap() as u32).collect())
        .collect();
    let ours: Vec<Vec<u32>> = g.groups.unwrap().iter().map(|group| group.iter().map(|c| c.0).collect()).collect();
    assert_eq!(ours, published);
    // Clusters 21 and 22 group; 23 starts a new group; the ten percent method also couples 22 and 23.
    let pairs = t.close_pairs.unwrap();
    assert!(pairs.contains(&(ClusterIndex(21), ClusterIndex(22))));
    assert!(pairs.contains(&(ClusterIndex(22), ClusterIndex(23))));
    assert!(!pairs.contains(&(ClusterIndex(21), ClusterIndex(23))));
    // The extra coupled pair raises the ten percent value above grouping's where the reference
    // says they differ, in the same direction as the reference's own values. (The difference is
    // about 1e-12 relative here, below the verification tolerance; each rule's value is compared
    // with the reference in `discrete_references_every_rule_and_mutation`.)
    let (gc, tc) = (combined_of_rows(&g.rows), combined_of_rows(&t.rows));
    let mutation = |rule: &str| {
        analysis["mutations"].as_array().unwrap().iter().find(|m| m["modal_combination"] == rule).unwrap()["combined"].clone()
    };
    let (pg, pt) = (mutation("GROUPING"), mutation("TEN_PERCENT"));
    for key in analysis["chain"]["quantities_where_GROUPING_differs_from_TEN_PERCENT"].as_array().unwrap() {
        let key = key.as_str().unwrap();
        assert!(expected(&pt[key]) > expected(&pg[key]), "{key}: reference");
        assert!(tc[key] > gc[key], "{key}: {} against {}", tc[key], gc[key]);
    }
}

fn combined_of_rows(rows: &[open_pipe_stress_response_spectrum::CombinedRow]) -> std::collections::BTreeMap<String, f64> {
    rows.iter().map(|r| (r.quantity.key(), r.combined.value())).collect()
}

#[test]
fn c10_unequal_frequency_clusters_cancellation_and_straddling() {
    let all = cases(C10);
    // W2: two clusters of two distinct modes each; the cluster's lowest frequency stands for it.
    let w2 = &all[0];
    let analysis = &w2["analyses"][0];
    let a = discrete_analysis(w2, analysis, None);
    let c = combine(&a.request(ModalRule::Srss, None, DirectionalRule::Srss)).unwrap();
    assert_eq!(c.clusters.len(), 2);
    for s in &c.clusters {
        assert_eq!(s.modes.len(), 2);
        assert!(s.frequencies_hz[0] < s.frequencies_hz[1], "distinct frequencies inside one cluster");
        assert_eq!(s.standing_frequency_hz, s.frequencies_hz[0]);
        assert_eq!(s.class, ClusterClass::Periodic);
    }
    // N3.uz under Y: the two contributions of ±2.17e-3 m sum to about -4.43e-10 m.
    let i = a.keys.iter().position(|k| k == "N3.uz").unwrap();
    let witness = &analysis["cluster_witness"][0]["per_direction"]["Y"];
    let sum = c.cluster_sums.iter().find(|s| s.cluster == ClusterIndex(1) && s.direction == Direction::Y).unwrap().values[i];
    let reference = expected(&witness["cluster_sum_signed"]["N3.uz"]);
    let operands = a.contributions[&(Direction::Y, 1)][i].abs() + a.contributions[&(Direction::Y, 2)][i].abs();
    assert!(operands > 4e-3 && reference.abs() < 1e-9);
    assert!((sum - reference).abs() <= 1e-9 * operands, "{sum:e} against {reference:e}");
    // The adopted reading is not mode by mode: the reference's mode-by-mode SRSS of N3.uz differs.
    let mode_by_mode = expected(&analysis["not_the_adopted_reading_mode_by_mode"]["SRSS"]["combined"]["N3.uz"]);
    let ours = combined_of_rows(&c.rows)["N3.uz"];
    assert!((ours / mode_by_mode - 1.0).abs() > 0.3, "{ours} against mode by mode {mode_by_mode}");

    // W3: f_r lies strictly between the two modes of cluster 1, which is periodic as a whole.
    let w3 = &all[1];
    let analysis = &w3["analyses"][0];
    let a = discrete_analysis(w3, analysis, None);
    let fr = a.rigid_frequency.unwrap();
    let c = combine(&a.request(ModalRule::Srss, Some(RigidRule::Srss), DirectionalRule::Srss)).unwrap();
    let first = &c.clusters[0];
    assert!(first.frequencies_hz[0] < fr && fr < first.frequencies_hz[1]);
    assert_eq!(first.class, ClusterClass::Periodic);
    assert_eq!(c.clusters[1].class, ClusterClass::Rigid);
    // A rigid response whose load left out only the first mode's cluster as periodic, or none,
    // does not match the classification and is refused.
    let mut wrong = a.request(ModalRule::Srss, Some(RigidRule::Srss), DirectionalRule::Srss);
    for r in &mut wrong.rigid {
        r.periodic_clusters_left_out = vec![ClusterIndex(1), ClusterIndex(2)];
    }
    let refused = combine(&wrong).unwrap_err();
    assert!(refused.iter().all(|r| r.code.as_str() == "SPECTRUM_RIGID_RESPONSE_PREMISE_MISMATCH"));
    assert_eq!(refused.len(), 2);
}

#[test]
fn c7_repeated_pair_two_bases_give_the_same_combinations() {
    let case = &cases(C7)[0];
    let analysis = &case["analyses"][0];
    let witness = &analysis["basis_witness"];
    let published_pair_two: std::collections::BTreeMap<Direction, &Value> = ["Y", "Z"]
        .iter()
        .map(|d| {
            let e = analysis["contributions_signed"][*d].as_array().unwrap().iter().find(|e| e["modes"] == serde_json::json!([3, 4])).unwrap();
            (Direction::parse(d).unwrap(), &e["values_signed_sum_over_repeated_cluster"])
        })
        .collect();
    // The rows the basis witness publishes per vector.
    let keys: Vec<String> =
        witness["bases"]["basis_1"]["vectors"][0]["contributions_signed"]["Y"].as_object().unwrap().keys().cloned().collect();
    let modes: Vec<ModalMode> = analysis["mode_status"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| {
            let mode = m["mode"].as_u64().unwrap() as u32;
            ModalMode { mode: ModeIndex(mode), cluster: ClusterIndex(if mode <= 2 { 1 } else { 2 }), frequency_hz: input(&m["frequency_Hz"]) }
        })
        .collect();
    let mut results = Vec::new();
    for basis in ["basis_1", "basis_2"] {
        let b = &witness["bases"][basis];
        let vectors = b["vectors"].as_array().unwrap();
        let mut periodic = Vec::new();
        for d in [Direction::Y, Direction::Z] {
            for (k, v) in vectors.iter().enumerate() {
                let values: Vec<f64> = keys.iter().map(|key| input(&v["contributions_signed"][d.as_str()][key])).collect();
                periodic.push(ModalContribution { mode: ModeIndex(k as u32 + 1), direction: d, values });
            }
            // The second pair is supplied in the M-orthonormal basis whose first vector lies along
            // the excitation's projection on the eigenspace: the second vector has zero
            // participation, so the first carries the published sum and the second contributes 0.
            let sum: Vec<f64> = keys.iter().map(|key| input(&published_pair_two[&d][key])).collect();
            periodic.push(ModalContribution { mode: ModeIndex(3), direction: d, values: sum });
            periodic.push(ModalContribution { mode: ModeIndex(4), direction: d, values: vec![0.0; keys.len()] });
        }
        let mut per_rule = Vec::new();
        for rule in ModalRule::ALL {
            let request = CombinationRequest {
                damping_ratio: 0.05,
                modal_rule: rule,
                directional_rule: DirectionalRule::Srss,
                missing_mass: MissingMass::NotApplied,
                directions: vec![Direction::Y, Direction::Z],
                modes: modes.clone(),
                rows: keys.iter().map(|k| quantity(k)).collect(),
                periodic: periodic.clone(),
                rigid: Vec::new(),
            };
            per_rule.push((rule, combine(&request).unwrap(), &b["combined_every_modal_rule"]));
        }
        results.push(per_rule);
    }
    let mut tally = Tally::default();
    for per_rule in &results {
        for (rule, c, published) in per_rule {
            let name = rule.as_str();
            for (i, row) in c.rows.iter().enumerate() {
                let key = &keys[i];
                tally.check(&format!("C7 {name} {key}"), row.combined.value(), expected(&published[name]["combined"][key]));
                for p in &row.per_direction {
                    let e = expected(&published[name]["per_direction"][p.direction.as_str()][key]);
                    tally.check(&format!("C7 {name} {} {key}", p.direction), p.combined.value(), e);
                }
            }
        }
    }
    // The two bases agree with each other under every rule.
    for ((_, c1, _), (_, c2, _)) in results[0].iter().zip(&results[1]) {
        for (i, (r1, r2)) in c1.rows.iter().zip(&c2.rows).enumerate() {
            tally.check(&format!("C7 basis agreement {}", keys[i]), r1.combined.value(), r2.combined.value());
        }
    }
    eprintln!("C7 basis witness: {tally:?}");
}

fn chain_rows() -> Vec<Quantity> {
    vec![
        Quantity::NodeDisplacement { node: "N1".into(), component: DisplacementComponent::Ux },
        Quantity::NodeDisplacement { node: "N2".into(), component: DisplacementComponent::Ux },
        // A spring's tension is the force on the spring element at its end j along +x.
        Quantity::ElementEndForce { member: "K1".into(), end: End::J, component: ForceComponent::Fx, frame: Frame::Global },
        Quantity::ElementEndForce { member: "K2".into(), end: End::J, component: ForceComponent::Fx, frame: Frame::Global },
        Quantity::SupportReaction { support: "G".into(), component: ReactionComponent::Fx },
    ]
}

const CHAIN_KEYS: [&str; 5] = ["u1_m", "u2_m", "spring1_force_N", "spring2_force_N", "ground_reaction_N"];

fn val(v: &Value) -> f64 {
    v["binary64"].as_str().unwrap().parse().unwrap()
}

fn dec(v: &Value) -> f64 {
    v["value"].as_str().unwrap().parse().unwrap()
}

#[test]
fn two_mass_chain_sets_a_b_c() {
    let doc: Value = serde_json::from_str(CHAIN).unwrap();
    let zeta = val(&doc["spectrum"]["damping_ratio_for_CQC"]);
    let mut tally = Tally::default();
    for set in ["A", "B", "C"] {
        let s = &doc["sets"][set];
        let modes: Vec<ModalMode> = s["modes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|m| {
                let n = m["mode"].as_u64().unwrap() as u32;
                ModalMode { mode: ModeIndex(n), cluster: ClusterIndex(n), frequency_hz: val(&m["frequency_hz"]) }
            })
            .collect();
        let contributions = |modes_wanted: &[u32]| -> Vec<ModalContribution> {
            modes_wanted
                .iter()
                .map(|&m| ModalContribution {
                    mode: ModeIndex(m),
                    direction: Direction::X,
                    values: CHAIN_KEYS.iter().map(|k| val(&s["without_missing_mass"]["signed_contributions"][k][m as usize - 1])).collect(),
                })
                .collect()
        };
        let request = |rule, missing_mass, periodic, rigid| CombinationRequest {
            damping_ratio: zeta,
            modal_rule: rule,
            directional_rule: DirectionalRule::Srss,
            missing_mass,
            directions: vec![Direction::X],
            modes: modes.clone(),
            rows: chain_rows(),
            periodic,
            rigid,
        };
        let rules = [("SRSS", ModalRule::Srss), ("CQC", ModalRule::Cqc), ("GROUPING", ModalRule::Grouping),
            ("ABSSUM", ModalRule::AbsoluteSum), ("TEN_PERCENT", ModalRule::TenPercent)];
        if set != "C" {
            let all = contributions(&[1, 2]);
            for (name, rule) in rules {
                let c = combine(&request(rule, MissingMass::NotApplied, all.clone(), vec![])).unwrap();
                for (i, k) in CHAIN_KEYS.iter().enumerate() {
                    let e = dec(&s["without_missing_mass"]["combined"][k][name]);
                    tally.check(&format!("chain {set} {name} {k}"), c.rows[i].combined.value(), e);
                }
            }
        }
        for mm in s.get("missing_mass").and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]) {
            let fr = val(&mm["rigid_frequency_hz"]);
            let periodic_modes: Vec<u32> = mm["periodic_modes"].as_array().unwrap().iter().map(|m| m.as_u64().unwrap() as u32).collect();
            let periodic = contributions(&periodic_modes);
            let rigid_values: Vec<f64> = CHAIN_KEYS.iter().map(|k| val(&mm["rigid_response_signed"][k])).collect();
            let rigid = vec![RigidContribution {
                direction: Direction::X,
                periodic_clusters_left_out: periodic_modes.iter().map(|&m| ClusterIndex(m)).collect(),
                values: rigid_values.clone(),
            }];
            for (name, rule) in rules {
                for (field, rigid_rule) in [("with_rigid_SRSS", RigidRule::Srss), ("with_rigid_ABSSUM", RigidRule::AbsoluteSum)] {
                    let missing = MissingMass::Applied { rigid_frequency_hz: fr, rule: rigid_rule };
                    let c = combine(&request(rule, missing, periodic.clone(), rigid.clone())).unwrap();
                    for (i, k) in CHAIN_KEYS.iter().enumerate() {
                        let published = &mm["combined"][k][name];
                        let part = &c.rows[i].per_direction[0];
                        tally.check(&format!("chain {set} f_r={fr} {name} periodic {k}"), part.periodic.value(), dec(&published["periodic"]));
                        tally.check(&format!("chain {set} f_r={fr} {name} {field} {k}"), c.rows[i].combined.value(), dec(&published[field]));
                    }
                }
            }
        }
    }
    eprintln!("two-mass chain: {tally:?}");
    // Set B: ρ12 as published.
    let b = &doc["sets"]["B"];
    let rho = open_pipe_stress_response_spectrum::correlation_coefficient(val(&b["modes"][0]["frequency_hz"]), val(&b["modes"][1]["frequency_hz"]), zeta);
    let published = dec(&b["rho_12"]);
    assert!((rho - published).abs() <= 1e-9 * published, "{rho} against {published}");
}

#[test]
fn directional_d1_to_d7() {
    let doc: Value = serde_json::from_str(DIRECTIONAL).unwrap();
    let u = f64::EPSILON / 2.0;
    for case in doc["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let mut directions = Vec::new();
        let mut periodic = Vec::new();
        for d in Direction::ALL {
            let m = &case["directional_magnitudes"][d.as_str().to_lowercase()];
            if m.is_string() {
                continue; // "not requested": the direction is absent
            }
            let value: f64 = m["binary64"].as_str().unwrap().parse().unwrap();
            directions.push(d);
            periodic.push(ModalContribution { mode: ModeIndex(1), direction: d, values: vec![value] });
        }
        for rule in DirectionalRule::ALL {
            let request = CombinationRequest {
                damping_ratio: 0.05,
                // The modal absolute sum of one mode returns |R| exactly.
                modal_rule: ModalRule::AbsoluteSum,
                directional_rule: rule,
                missing_mass: MissingMass::NotApplied,
                directions: directions.clone(),
                modes: vec![ModalMode { mode: ModeIndex(1), cluster: ClusterIndex(1), frequency_hz: 1.0 }],
                rows: vec![Quantity::NodeDisplacement { node: "N1".into(), component: DisplacementComponent::Ux }],
                periodic: periodic.clone(),
                rigid: vec![],
            };
            let c = combine(&request).unwrap();
            let row = &c.rows[0];
            let (field, bound_ulps) = match rule {
                // One rounding per square and addition and the root: within 4u of the nearest.
                DirectionalRule::Srss => ("SRSS", 4.0),
                // Two products and two additions, plus the factor's (4/9)·2^-54 ≤ u: within 6u.
                DirectionalRule::HundredFortyForty => ("100_40_40", 6.0),
                DirectionalRule::AbsoluteSum => ("ABSSUM", 3.0),
            };
            let b: f64 = case[field]["binary64"].as_str().unwrap().parse().unwrap();
            let deviation = (row.combined.value() - b).abs();
            assert!(deviation <= bound_ulps * u * b, "{id} {field}: {} against {b}", row.combined.value());
            assert!(deviation <= TOLERANCE * b);
            if rule == DirectionalRule::HundredFortyForty {
                let governing = case["governing_permutations"][0].as_str().unwrap();
                assert_eq!(format!("full_{}", row.governing_direction.unwrap().as_str().to_lowercase()), governing, "{id}");
            } else {
                assert_eq!(row.governing_direction, None);
            }
        }
    }
}

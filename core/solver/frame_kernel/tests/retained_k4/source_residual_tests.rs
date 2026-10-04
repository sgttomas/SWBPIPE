//! Actual native owner -> checked view -> conditional source bridge witnesses.
//! The exact oracle is generated independently from closed cantilever equations.
use super::super::product_certificate::{bridge::*, source_residual::*, MaterialOperands};
use super::super::recover::End;
use super::super::source::{
    Component, Constraint, NodalLoad, SourceParts, Station, StraightMember, SupportGroup,
};
use super::*;
use std::cmp::Ordering as Cmp;
#[path = "source_residual_vectors.rs"]
mod oracle;

fn source(loaded: bool, cancelled: bool) -> PrimitiveSource {
    let mut loads = Vec::new();
    for c in [Component::Ux, Component::Uy, Component::Rx] {
        loads.push(NodalLoad {
            dof: Dof {
                node: 1,
                component: c,
            },
            value: if loaded { 1.0 } else { 0.0 },
            source_id: format!("tip-{c:?}"),
        });
    }
    if cancelled {
        for value in [1.0, -1.0] {
            loads.push(NodalLoad {
                dof: Dof {
                    node: 1,
                    component: Component::Ux,
                },
                value,
                source_id: format!("cancel-{value}"),
            });
        }
    }
    PrimitiveSource::new(SourceParts {
        nodes: vec![[0.0; 3], [1.0, 0.0, 0.0]],
        members: vec![StraightMember {
            id: 7,
            node_i: 0,
            node_j: 1,
            elastic_modulus: f64::from_bits(oracle::E),
            shear_modulus: f64::from_bits(oracle::G),
            area: f64::from_bits(oracle::A),
            second_moment_y: f64::from_bits(oracle::I),
            second_moment_z: f64::from_bits(oracle::I),
            torsion_constant: f64::from_bits(oracle::J),
            y_reference: [0.0, 1.0, 0.0],
        }],
        constraints: Component::ALL
            .iter()
            .map(|&c| Constraint {
                dof: Dof {
                    node: 0,
                    component: c,
                },
                value: 0.0,
            })
            .collect(),
        loads,
        stations: [0.0, 0.25, 1.0]
            .iter()
            .enumerate()
            .map(|(i, &fraction)| Station {
                id: 17 + i as u32,
                member: 7,
                fraction,
            })
            .collect(),
        supports: vec![SupportGroup {
            id: 3,
            node: 0,
            restrained: [true; 6],
            springs: vec![],
            directional_springs: vec![],
        }],
        ..SourceParts::default()
    })
    .unwrap()
}
fn solved(source: PrimitiveSource) -> Box<RetainedSolve> {
    match solve_case(
        source,
        CaseLimit::new(u64::MAX),
        &mut InvocationMeter::new(u64::MAX),
    ) {
        CaseOutcome::Selected(s) => s,
        other => panic!("small native source did not select: {other:?}"),
    }
}
fn laws(owner: &RetainedSolve) -> [ProposedMemberLaw<'_>; 1] {
    [ProposedMemberLaw {
        member: &owner.source().members()[0],
        diameter: f64::from_bits(oracle::D),
        effective_wall: f64::from_bits(oracle::T),
        material: MaterialOperands::Ordinary {
            e: f64::from_bits(oracle::E),
            g: f64::from_bits(oracle::G),
        },
        represented_z: f64::from_bits(oracle::Z),
    }]
}
fn endpoint(b: oracle::Bits) -> Wide<16> {
    Wide::<16>::from_parts(b.0, b.1, b.2).unwrap()
}
fn pair(p: (oracle::Bits, oracle::Bits)) -> (Wide<16>, Wide<16>) {
    (endpoint(p.0), endpoint(p.1))
}
fn point(x: f64) -> (Wide<16>, Wide<16>) {
    let v = Wide::<16>::from_f64(x).unwrap();
    (v, v)
}
fn exact(id: QuantityId, geometric: bool, loaded: bool) -> (Wide<16>, Wide<16>) {
    if !loaded {
        return point(0.0);
    }
    match id {
        QuantityId::Displacement(d) => {
            if d.node == 0 {
                point(0.0)
            } else {
                pair(
                    (if geometric {
                        oracle::G_MOTION
                    } else {
                        oracle::K_MOTION
                    })[d.component.index()],
                )
            }
        }
        QuantityId::DisplacementMagnitude(node) => {
            if node == 0 {
                point(0.0)
            } else {
                pair(if geometric {
                    oracle::G_MAG
                } else {
                    oracle::K_MAG
                })
            }
        }
        QuantityId::EndAction { end, component, .. } => point(match (end, component) {
            (End::I, Component::Ux | Component::Uy | Component::Rx | Component::Rz) => -1.0,
            (End::J, Component::Ux | Component::Uy | Component::Rx) => 1.0,
            _ => 0.0,
        }),
        QuantityId::StationAction { station, component } => point(match component {
            Component::Ux | Component::Uy | Component::Rx => 1.0,
            Component::Rz => match station {
                17 => 1.0,
                18 => 0.75,
                19 => 0.0,
                _ => panic!(),
            },
            _ => 0.0,
        }),
        QuantityId::Reaction(d) => point(match d.component {
            Component::Ux | Component::Uy | Component::Rx | Component::Rz => -1.0,
            _ => 0.0,
        }),
        QuantityId::SupportForceMagnitude(_) | QuantityId::SupportMomentMagnitude(_) => {
            pair(oracle::ROOT_TWO)
        }
        _ => panic!("not in closed native oracle"),
    }
}
fn encoded(w: &Wide<16>) -> String {
    let (neg, e, limbs) = w.parts();
    let hex = limbs
        .iter()
        .rev()
        .map(|v| format!("{v:016x}"))
        .collect::<String>();
    format!("[{neg},{e},\"{hex}\"]")
}

fn run<'a>(s: &'a RetainedSolve, law: &'a [ProposedMemberLaw<'a>]) -> ResidualSpent<'a> {
    source_residual(
        s,
        s.source(),
        &s.evidence().source_encoding,
        s.selected_precision(),
        law,
    )
}
#[test]
fn source_residual_actual_zero_loaded_witness() {
    for loaded in [false, true] {
        let s = solved(source(loaded, false));
        let original = s.publish().clone();
        let input = laws(&s);
        let spent = run(&s, &input);
        let native = spent.result().unwrap();
        let label = if loaded { "loaded" } else { "zero" };
        assert_eq!(
            spent.work.correction.calls.exact(),
            Ok(if loaded { 1 } else { 0 })
        );
        assert_eq!(spent.work.member_builds.exact(), Ok(4));
        assert_eq!(spent.work.frame_builds.exact(), Ok(3));
        assert_eq!(s.publish(), &original);
        let mut passed = 0;
        for (i, row) in s.publish().rows.iter().enumerate() {
            let (glo, ghi) = exact(row.id, true, loaded);
            let (lo, hi) = native.rows[i].endpoints();
            assert_ne!(
                lo.cmp_value(&glo),
                Cmp::Greater,
                "{label} {:?} lower",
                row.id
            );
            assert_ne!(hi.cmp_value(&ghi), Cmp::Less, "{label} {:?} upper", row.id);
            let x = row.value.value().unwrap();
            let scale =
                f64::from_bits(s.publish().body_scales[row.body as usize * 4 + row.kind.index()].2);
            let (class, bound) = match row.class {
                RowClass::InputDerived => ("input", 0),
                RowClass::AbsoluteVerified { bound_bits } => ("absolute", bound_bits),
                RowClass::RelativeVerified => {
                    ("relative", sharper_binary64(x, scale).unwrap().to_bits())
                }
                _ => panic!(),
            };
            let ok = native_predicate(row, scale, &native.rows[i]);
            if ok {
                passed += 1;
            }
            println!("I44_ROW {{\"witness\":\"{label}\",\"index\":{i},\"id\":\"{:?}\",\"x\":\"{:016x}\",\"class\":\"{class}\",\"bound\":\"{bound:016x}\",\"scale\":\"{:016x}\",\"lo\":{},\"hi\":{},\"glo\":{},\"ghi\":{},\"pass\":{ok}}}",row.id,x.to_bits(),scale.to_bits(),encoded(lo),encoded(hi),encoded(&glo),encoded(&ghi));
        }
        for a in 0..native.center.len() {
            let r = native.rho[a];
            let i = native.initial[a];
            println!("I44_CENTER {{\"witness\":\"{label}\",\"a\":{a},\"s\":{},\"initial_lo\":{},\"initial_hi\":{},\"rhs\":{},\"correction\":{},\"center\":{},\"rho_lo\":{},\"rho_hi\":{}}}",native.view.scales()[a],encoded(i.endpoints().0),encoded(i.endpoints().1),encoded(spent.work.correction.actual_rhs.get(a).unwrap_or(&Wide::<16>::ZERO)),encoded(native.correction.get(a).unwrap_or(&Wide::<16>::ZERO)),encoded(&native.center[a]),encoded(r.endpoints().0),encoded(r.endpoints().1));
        }
        println!("I44_BLOCK {{\"witness\":\"{label}\",\"p\":{},\"P\":{},\"bound\":\"{:016x}\",\"alpha\":{},\"epsilon\":{},\"passed\":{passed},\"total\":{}}}",s.selected_precision(),s.evidence().verification_precision,s.evidence().certified_bound.first().map_or(0,|x|x.1),encoded(&native.alpha[0]),encoded(&native.epsilon[0]),native.rows.len());
        println!("I44_WORK {label} {:?}", spent.work);
        assert_eq!(passed, native.rows.len());
    }
}

fn parts_from(s: &PrimitiveSource) -> SourceParts {
    SourceParts {
        nodes: s.nodes().to_vec(),
        members: s.members().to_vec(),
        springs: s.springs().to_vec(),
        directional_springs: s.directional_springs().to_vec(),
        constraints: s.constraints().to_vec(),
        loads: s.loads().to_vec(),
        stations: s.stations().to_vec(),
        supports: s.supports().to_vec(),
    }
}
fn contains(
    interval: &super::super::product_certificate::Enclosure,
    truth: (Wide<16>, Wide<16>),
    label: &str,
) {
    let (lo, hi) = interval.endpoints();
    assert_ne!(lo.cmp_value(&truth.0), Cmp::Greater, "{label} lower");
    assert_ne!(hi.cmp_value(&truth.1), Cmp::Less, "{label} upper");
}
fn native_predicate(
    row: &PublishedRow,
    scale: f64,
    interval: &super::super::product_certificate::Enclosure,
) -> bool {
    let x = row.value.value().unwrap();
    let xv = Wide::<16>::from_f64(x).unwrap();
    let ax = xv.abs();
    let mut ctx = WideContext::<16>::new(1024).unwrap();
    let mut sum = ExactWideSum::new();
    let (lo, hi) = interval.endpoints();
    let h0 = super::super::directed::sub_toward(
        &mut ctx,
        &mut sum,
        &xv,
        lo,
        super::super::directed::Toward::Up,
    )
    .unwrap();
    let h1 = super::super::directed::sub_toward(
        &mut ctx,
        &mut sum,
        hi,
        &xv,
        super::super::directed::Toward::Up,
    )
    .unwrap();
    let h = if h0.cmp_value(&h1) == Cmp::Greater {
        h0
    } else {
        h1
    };
    match row.class {
        RowClass::InputDerived => h.is_zero(),
        RowClass::AbsoluteVerified { bound_bits } => {
            h.cmp_value(&Wide::<16>::from_f64(f64::from_bits(bound_bits)).unwrap()) != Cmp::Greater
        }
        RowClass::RelativeVerified => {
            let sv = Wide::<16>::from_f64(scale).unwrap();
            let m = if ax.cmp_value(&sv) == Cmp::Greater {
                ax
            } else {
                sv
            };
            let mut exact = ExactWideSum::new();
            exact.add_wide_scaled(&m, false, 1, -64).unwrap();
            exact.add_wide_scaled(&m, false, 1, -85).unwrap();
            exact.add_wide_scaled(&ax, false, 1, -53).unwrap();
            exact.add_binary64(f64::from_bits(1), false).unwrap();
            exact.add_wide(&h, true).unwrap();
            let exact_ok = exact.signum().unwrap() >= 0;
            let binary64_ok = h
                .cmp_value(&Wide::<16>::from_f64(sharper_binary64(x, scale).unwrap()).unwrap())
                != Cmp::Greater;
            let mut decimal = ExactWideSum::new();
            decimal.add_wide(&ax, false).unwrap();
            decimal.add_wide_scaled(&h, true, 1000000000, 0).unwrap();
            // Raw and SI are identical for this native diagnostic, so both
            // unchanged decimal comparisons have the same exact operands.
            exact_ok && binary64_ok && decimal.signum().unwrap() >= 0
        }
        _ => false,
    }
}
fn predicate_count(
    s: &RetainedSolve,
    rows: &[super::super::product_certificate::Enclosure],
) -> usize {
    s.publish()
        .rows
        .iter()
        .zip(rows)
        .filter(|(row, interval)| {
            native_predicate(
                row,
                f64::from_bits(s.publish().body_scales[row.body as usize * 4 + row.kind.index()].2),
                interval,
            )
        })
        .count()
}
#[test]
fn source_residual_wrong_center_recomputed_full_source_is_not_a_certificate() {
    let s = solved(source(true, false));
    let input = laws(&s);
    let spent = run(&s, &input);
    let native = spent.result().unwrap();
    let center: Vec<_> = (6..12)
        .map(|g| Wide::<16>::from_f64(s.publish().rows[g].value.value().unwrap()).unwrap())
        .collect();
    let (rho, epsilon, rows, work) = test_at_center(native, &center).unwrap();
    assert_eq!(work.correction.calls.exact(), Ok(0));
    assert!(rho.iter().any(|r| !r.endpoints().0.is_zero()));
    assert!(epsilon[0].cmp_value(&native.epsilon[0]) == Cmp::Greater);
    for (i, row) in s.publish().rows.iter().enumerate() {
        contains(
            &rows[i],
            exact(row.id, true, true),
            "wrong-center source containment",
        );
    }
    assert_eq!(predicate_count(&s, &rows), 7);
}
#[test]
fn source_residual_prescribed_columns_and_fully_fixed_loads() {
    let mut p = parts_from(&source(false, false));
    p.constraints[0].value = 0.0001;
    let s = solved(PrimitiveSource::new(p).unwrap());
    let input = laws(&s);
    let spent = run(&s, &input);
    let native = spent.result().unwrap();
    assert_eq!(native.view.data(), &[true]);
    for (i, row) in s.publish().rows.iter().enumerate() {
        let v = match row.id {
            QuantityId::Displacement(d) if d.component == Component::Ux => 0.0001,
            QuantityId::DisplacementMagnitude(_) => 0.0001,
            _ => 0.,
        };
        contains(&native.rows[i], point(v), "prescribed rigid translation");
    }
    for strain in [false, true] {
        let mut p = parts_from(&source(false, false));
        for component in Component::ALL {
            p.constraints.push(Constraint {
                dof: Dof { node: 1, component },
                value: if strain && component == Component::Ux {
                    0.0001
                } else {
                    0.
                },
            });
        }
        p.loads.push(NodalLoad {
            dof: Dof {
                node: 0,
                component: Component::Ux,
            },
            value: 3.,
            source_id: "constrained".into(),
        });
        let s = solved(PrimitiveSource::new(p).unwrap());
        let input = laws(&s);
        let spent = run(&s, &input);
        let native = spent.result().unwrap();
        assert!(native.view.data().is_empty());
        assert_eq!(spent.work.correction.calls.exact(), Ok(0));
        for (i, row) in s.publish().rows.iter().enumerate() {
            if let QuantityId::Reaction(d) = row.id {
                if d.node == 0 && d.component == Component::Ux {
                    if strain {
                        let (a, b) = pair(oracle::FIXED_AXIAL);
                        let mut c = WideContext::<16>::new(1024).unwrap();
                        let three = Wide::<16>::from_f64(3.).unwrap();
                        contains(
                            &native.rows[i],
                            (
                                c.sub(&b.neg(), &three).unwrap(),
                                c.sub(&a.neg(), &three).unwrap(),
                            ),
                            "fixed prescribed action and load",
                        );
                    } else {
                        contains(&native.rows[i], point(-3.), "fixed constrained load");
                    }
                }
            }
        }
    }
}
#[test]
fn source_residual_positive_spring_and_rotated_nonunit_frame() {
    let mut p = parts_from(&source(false, false));
    p.loads.clear();
    p.loads.push(NodalLoad {
        dof: Dof {
            node: 1,
            component: Component::Ux,
        },
        value: 1.,
        source_id: "axial".into(),
    });
    p.members[0].y_reference = [0., 2., 0.];
    p.springs.push(super::super::source::Spring {
        id: 9,
        dof: Dof {
            node: 1,
            component: Component::Ux,
        },
        stiffness: 100000000.,
    });
    p.supports.push(SupportGroup {
        id: 4,
        node: 1,
        restrained: [false; 6],
        springs: vec![9],
        directional_springs: vec![],
    });
    let s = solved(PrimitiveSource::new(p).unwrap());
    let input = laws(&s);
    let spent = run(&s, &input);
    let native = spent.result().unwrap();
    for (i, row) in s.publish().rows.iter().enumerate() {
        match row.id {
            QuantityId::Displacement(d) if d.node == 1 && d.component == Component::Ux => contains(
                &native.rows[i],
                pair(oracle::SPRING_U),
                "positive spring displacement",
            ),
            QuantityId::SpringAction { .. } => contains(
                &native.rows[i],
                pair(oracle::SPRING_ACTION),
                "spring action",
            ),
            QuantityId::Reaction(d) if d.component == Component::Ux => contains(
                &native.rows[i],
                pair(oracle::SPRING_REACTION),
                "spring root reaction",
            ),
            _ => {}
        }
    }
    let mut p = parts_from(&source(false, false));
    p.nodes[1] = [3., 4., 0.];
    p.members[0].y_reference = [-8., 6., 0.];
    p.loads.clear();
    for (component, value) in [(Component::Ux, 3.), (Component::Uy, 4.)] {
        p.loads.push(NodalLoad {
            dof: Dof { node: 1, component },
            value,
            source_id: format!("rotated-{component:?}"),
        });
    }
    let s = solved(PrimitiveSource::new(p).unwrap());
    let input = laws(&s);
    let spent = run(&s, &input);
    let native = spent.result().unwrap();
    for (i, row) in s.publish().rows.iter().enumerate() {
        let expected = match row.id {
            QuantityId::Displacement(d) if d.node == 1 && d.component == Component::Ux => {
                pair(oracle::ROTATED_UX)
            }
            QuantityId::Displacement(d) if d.node == 1 && d.component == Component::Uy => {
                pair(oracle::ROTATED_UY)
            }
            QuantityId::DisplacementMagnitude(1) => pair(oracle::ROTATED_MAG),
            QuantityId::EndAction {
                end,
                component: Component::Ux,
                ..
            } => point(if end == End::I { -5. } else { 5. }),
            QuantityId::StationAction {
                component: Component::Ux,
                ..
            } => point(5.),
            QuantityId::Reaction(d) if d.component == Component::Ux => point(-3.),
            QuantityId::Reaction(d) if d.component == Component::Uy => point(-4.),
            QuantityId::SupportForceMagnitude(_) => point(5.),
            _ => point(0.),
        };
        contains(&native.rows[i], expected, "rotated exact source");
    }
}
#[test]
fn source_residual_cancelled_terms_mixed_no_data_and_identity_refusals() {
    let s = solved(source(false, true));
    let input = laws(&s);
    let spent = run(&s, &input);
    assert_eq!(spent.result().unwrap().view.data(), &[true]);
    assert_eq!(spent.work.correction.calls.exact(), Ok(1));
    let mut p = parts_from(&source(true, false));
    let old = parts_from(&source(false, false));
    p.nodes.extend([[2., 0., 0.], [3., 0., 0.]]);
    let mut m = old.members[0].clone();
    m.id = 8;
    m.node_i = 2;
    m.node_j = 3;
    p.members.push(m);
    for mut c in old.constraints {
        c.dof.node = 2;
        p.constraints.push(c);
    }
    let s = solved(PrimitiveSource::new(p).unwrap());
    let input: Vec<_> = s
        .source()
        .members()
        .iter()
        .map(|m| ProposedMemberLaw {
            member: m,
            ..laws(&s)[0]
        })
        .collect();
    let spent = run(&s, &input);
    let native = spent.result().unwrap();
    assert_eq!(native.view.data(), &[true, false]);
    for (a, &g) in native.view.group().ordering.free.iter().enumerate() {
        if g >= 12 {
            assert!(native.center[a].is_zero());
            assert!(native.epsilon[native.view.group().blocks.of[a] as usize].is_zero());
        }
    }
    let s = solved(source(true, false));
    let input = laws(&s);
    let foreign = s.source().clone();
    assert!(matches!(
        source_residual(&s, &foreign, &s.prep.identity, s.selected, &input).result(),
        Err(BridgeError::View(SourceBridgeViewIssue::ForeignOwner))
    ));
    let mut foreign_law = input;
    foreign_law[0].member = &foreign.members()[0];
    assert!(matches!(
        run(&s, &foreign_law).result(),
        Err(BridgeError::MemberOwner)
    ));
    for fault in 0..4 {
        let mut changed = (*s).clone();
        match fault {
            0 => changed.evidence.verification_precision = 128,
            1 => changed.cache.s256 = None,
            2 => changed.evidence.ledger_encoding[0] ^= 1,
            3 => changed.publication.rows.swap(0, 1),
            _ => unreachable!(),
        };
        let input = laws(&changed);
        let spent = run(&changed, &input);
        assert!(spent.result().is_err());
        assert_eq!(spent.work.correction.calls.exact(), Ok(0));
    }
}
#[test]
fn source_residual_arithmetic_and_factor_cast_prefixes() {
    test_arithmetic_controls();
    let s = solved(source(true, false));
    let view = s
        .source_bridge_view(s.source(), &s.prep.identity, s.selected)
        .result
        .unwrap();
    let (result, work) = view.source_correction(vec![]).into_parts();
    assert_eq!(result, Err(SourceCorrectionError::Identity));
    assert!(work.status().is_exact());
    assert_eq!(work.calls.exact(), Ok(0));
    let huge =
        Wide::<16>::from_parts(false, super::super::wide::EXPONENT_LIMIT, [u64::MAX; 16]).unwrap();
    let mut rhs = vec![Wide::<16>::ONE; 6];
    rhs[1] = huge;
    let (result, work) = view.source_correction(rhs).into_parts();
    assert!(matches!(
        result,
        Err(SourceCorrectionError::Arithmetic(AttemptStop::Exponent))
    ));
    assert_eq!(work.cast.width::<4>().round, 2);
    assert_eq!(work.calls.exact(), Ok(0));
    assert!(work.status().is_exact());
    let huge = Wide::<16>::ONE
        .mul_pow2(super::super::wide::EXPONENT_LIMIT)
        .unwrap();
    let mut observed_failure = false;
    for sign in [false, true] {
        let rhs = (0..6)
            .map(|i| if sign && i % 2 == 0 { huge.neg() } else { huge })
            .collect();
        let spent = view.source_correction(rhs);
        let (result, mut work) = spent.into_parts();
        if let Err(e) = result {
            assert!(matches!(e, SourceCorrectionError::Arithmetic(_)));
            assert_eq!(work.calls.exact(), Ok(1));
            assert!(work.factor.checked_lme().exact().unwrap() > 0);
            assert!(work.status().is_exact());
            println!("I44_FACTOR_FAILURE {e:?} {:?}", work.factor);
            work.visits = WorkTotal::exact_count(u64::MAX).add(WorkTotal::exact_count(1));
            let (result, work) = SourceCorrectionSpent {
                result: Err(e.clone()),
                work,
            }
            .into_parts();
            assert_eq!(result, Err(e));
            assert!(!work.status().is_exact());
            observed_failure = true;
        }
    }
    assert!(
        observed_failure,
        "range control must reach an actual partial factor refusal"
    );
    let input = laws(&s);
    let mut success = run(&s, &input);
    success.work.visits = WorkTotal::exact_count(u64::MAX).add(WorkTotal::exact_count(1));
    assert!(success.result().is_err());
    let mut bad = input;
    bad[0].effective_wall = 0.1;
    let mut failure = run(&s, &bad);
    failure.work.visits = WorkTotal::exact_count(u64::MAX).add(WorkTotal::exact_count(1));
    assert!(matches!(
        failure.result(),
        Err(BridgeError::Numeric(
            super::super::product_certificate::NumericError::InvalidGeometry
        ))
    ));
    assert!(!failure.work.status().is_exact());
}

#[test]
fn source_residual_conservative_alpha_parity_and_unsupported_paths() {
    let s = solved(source(true, false));
    let input = laws(&s);
    let original = source_bridge(&s, s.source(), &s.prep.identity, s.selected, &input);
    let candidate = run(&s, &input);
    assert_eq!(
        original.result().unwrap().alpha,
        candidate.result().unwrap().alpha
    );
    // Diagnostic baseline overlap only; production source_residual never calls
    // the conservative bridge or retains its rows/work arrays.
    let mut p = parts_from(&source(false, false));
    p.directional_springs
        .push(super::super::source::DirectionalSpring {
            id: 10,
            node: 1,
            kind: super::super::source::SpringKind::Translation,
            direction: [1., 2., 0.],
            stiffness: 1e6,
        });
    let s = solved(PrimitiveSource::new(p).unwrap());
    let input = laws(&s);
    let spent = run(&s, &input);
    assert!(matches!(
        spent.result(),
        Err(BridgeError::UnsupportedDirectionalSpring)
    ));
    assert_eq!(spent.work.member_builds.exact(), Ok(0));
    let s = solved(source(true, false));
    let other = solved(source(true, false));
    let view = s
        .source_bridge_view(s.source(), &s.prep.identity, s.selected)
        .result
        .unwrap();
    let foreign_scales = other
        .source_bridge_view(other.source(), &other.prep.identity, other.selected)
        .result
        .unwrap();
    let bad = SourceBridgeView {
        owner: &s,
        scales: foreign_scales.scales(),
        data: view.data().to_vec(),
    };
    let (result, work) = bad
        .source_correction(vec![Wide::<16>::ZERO; 6])
        .into_parts();
    assert_eq!(result, Err(SourceCorrectionError::Identity));
    assert_eq!(work.calls.exact(), Ok(0));
    assert!(work.status().is_exact());
    let mut changed = (*s).clone();
    let old = &changed.group;
    let mut group = GroupPrep {
        structure: old.structure.clone(),
        ordering: old.ordering.clone(),
        geometry: old.geometry.clone(),
        blocks: old.blocks.clone(),
    };
    group.ordering.order.swap(0, 1);
    changed.group = Arc::new(group);
    let input = laws(&changed);
    let spent = run(&changed, &input);
    assert!(matches!(
        spent.result(),
        Err(BridgeError::View(SourceBridgeViewIssue::Ordering))
    ));
    assert_eq!(spent.work.correction.calls.exact(), Ok(0));
    let mut input = laws(&s);
    input[0].diameter = 0.2;
    let spent = run(&s, &input);
    assert!(matches!(spent.result(), Err(BridgeError::Alpha { .. })));
    assert_eq!(spent.work.correction.calls.exact(), Ok(0));
}

#[test]
fn source_residual_collects_actual_failed_factor_with_simultaneous_accounting_loss() {
    let s = solved(source(true, false));
    let shared = s.cache.s256.as_ref().unwrap().as_ref().unwrap();
    // Exact preloaded homogeneous Round work at (2^61-1)*8 LME. Every seed
    // merge remains exact; only collection of the real failed factor overflows.
    let mut one = WideContext::<4>::new(256).unwrap();
    one.round(&Wide::<4>::ONE).unwrap();
    let mut power = AttemptWork::default();
    power.record(&one);
    let mut accumulated = AttemptWork::default();
    for bit in 0..61 {
        accumulated.merge(&power);
        if bit < 60 {
            let copy = power;
            power.merge(&copy);
        }
    }
    assert_eq!(accumulated.checked_lme().exact(), Ok(u64::MAX - 7));
    let mut work = SourceCorrectionWork {
        factor: accumulated,
        ..SourceCorrectionWork::default()
    };
    let huge = Wide::<4>::ONE
        .mul_pow2(super::super::wide::EXPONENT_LIMIT)
        .unwrap();
    let mut ctx = WideContext::<4>::new(256).unwrap();
    let result = source_factor_once(&shared.factor, &[huge; 6], &mut ctx, &mut work);
    assert_eq!(
        result,
        Err(SourceCorrectionError::Arithmetic(AttemptStop::Exponent))
    );
    assert_eq!(ctx.work().mul, 15);
    assert_eq!(ctx.work().sub, 15);
    assert_eq!(ctx.work().div, 5);
    assert_eq!(work.calls.exact(), Ok(1));
    assert!(!work.status().is_exact());
    let (result, work) = SourceCorrectionSpent {
        result: Err(result.unwrap_err()),
        work,
    }
    .into_parts();
    assert_eq!(
        result,
        Err(SourceCorrectionError::Arithmetic(AttemptStop::Exponent))
    );
    assert!(!work.status().is_exact());
    println!("I44_SIMULTANEOUS failed_factor=Exponent actual_prefix_mul=15 sub=15 div=5 collection_status={:?}",work.status());
}
#[test]
fn source_residual_combination_and_missing_uniqueness_are_explicit_refusals() {
    let s = solved(source(true, false));
    let combined = match super::super::combine::RetainedCombination::solve(
        &[(1., &s)],
        CaseLimit::new(u64::MAX),
        &mut InvocationMeter::new(u64::MAX),
    ) {
        super::super::combine::CombinationOutcome::Selected(s) => s,
        other => panic!("{other:?}"),
    };
    let input = laws(&combined);
    let spent = run(&combined, &input);
    assert!(matches!(
        spent.result(),
        Err(BridgeError::View(
            SourceBridgeViewIssue::UnsupportedCombination
        ))
    ));
    assert_eq!(spent.work.correction.calls.exact(), Ok(0));
    let mut p = parts_from(&source(false, false));
    let dof = p.constraints.remove(0).dof;
    p.springs.push(super::super::source::Spring {
        id: 9,
        dof,
        stiffness: 1e9,
    });
    p.supports[0].restrained[0] = false;
    p.supports[0].springs.push(9);
    let s = solved(PrimitiveSource::new(p).unwrap());
    let input = laws(&s);
    let spent = run(&s, &input);
    assert!(matches!(
        spent.result(),
        Err(BridgeError::MissingUniquenessWarrant(0))
    ));
    assert_eq!(spent.work.correction.calls.exact(), Ok(0));
}

#[test]
fn i51_view_failure_keeps_both_entered_mask_capacities() {
    let s=solved(source(true,false));
    let law=laws(&s);
    let early=source_residual(&s,s.source(),&s.prep.identity,512,&law);
    assert!(early.result().is_err());assert_eq!(early.work.view.prescribed_capacity,0);assert_eq!(early.work.data_capacity,0);
    assert_eq!(early.work.correction.calls.exact(),Ok(0));
    let good=s.source_bridge_view(s.source(),&s.prep.identity,s.selected);
    let visits=good.work.visits.exact().unwrap();let mut after_prescribed=false;
    for budget in 0..visits {
        let mut work=SourceBridgeViewWork{visits:WorkTotal::exact_count(u64::MAX-budget),..SourceBridgeViewWork::default()};
        let result=s.build_source_bridge_view(s.source(),&s.prep.identity,s.selected,&mut work);
        if work.prescribed_capacity>0 && work.data_capacity==0 {
            assert!(matches!(result,Err(SourceBridgeViewIssue::Work(_))));after_prescribed=true;break;
        }
    }
    assert!(after_prescribed,"actual view-only accounting prefix after prescribed allocation; no correction is called");
    for fault in 0..2 {
        let mut changed=(*s).clone();
        if fault==0 {changed.cache.s256=None;}else{changed.evidence.certified_bound.clear();}
        let spent=changed.source_bridge_view(changed.source(),&changed.prep.identity,changed.selected);
        assert!(spent.result.is_err());
        assert!(spent.work.prescribed_capacity>=changed.source().dof_count());
        assert!(spent.work.data_capacity>=changed.group.blocks.len());
        let law=laws(&changed);
        let residual=source_residual(&changed,changed.source(),&changed.prep.identity,changed.selected,&law);
        assert!(residual.result().is_err());assert_eq!(residual.work.data_capacity,spent.work.data_capacity);
        assert_eq!(residual.work.correction.calls.exact(),Ok(0));
        println!("I51_FAILED_VIEW_CAPACITY fault={fault} prescribed={} data={}",spent.work.prescribed_capacity,spent.work.data_capacity);
    }
}

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
    // B2-K K-11 (ROOT ruling R-5, the declared re-pin): the 1*s combination is
    // admitted under P2. Its residual reads its own combined ledger, and every
    // source row contains the closed oracle truth of the same loads.
    let input = laws(&combined);
    let spent = run(&combined, &input);
    let native = spent.result().unwrap();
    assert_eq!(spent.work.correction.calls.exact(), Ok(1));
    assert_eq!(native.rows.len(), combined.publish().rows.len());
    for (i, row) in combined.publish().rows.iter().enumerate() {
        contains(&native.rows[i], exact(row.id, true, true), &format!("1*s {:?}", row.id));
    }
    // A nonzero prescribed term in a non-representative operand still refuses.
    let mut p = parts_from(&source(true, false));
    p.constraints[0].value = 1e-3;
    let t = solved(PrimitiveSource::new(p).unwrap());
    let prescribed = match super::super::combine::RetainedCombination::solve(
        &[(1., &s), (1., &t)],
        CaseLimit::new(u64::MAX),
        &mut InvocationMeter::new(u64::MAX),
    ) {
        super::super::combine::CombinationOutcome::Selected(s) => s,
        other => panic!("{other:?}"),
    };
    let input = laws(&prescribed);
    let spent = run(&prescribed, &input);
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

// ---- b7-plan "N06 absorbed-spring certificate extension" -------------------------------
// N06 (validation/benchmarks/numerical_integrity N06): a 2 m member (D 0.2, t 0.01, E 2e11,
// G 8e10, the product's prepared annulus section) whose root is fixed except RX, which
// carries a 1e-12 N*m/rad scalar spring; tip torque +-1e-16 N*m. GJ/L/k is about 2.2e18.
const N06_SPRING: f64 = 1e-12;
const N06_TORQUE: f64 = 1e-16;
fn n06_section() -> [f64; 5] {
    let prepared = super::super::product_certificate::prepare_product_annulus(0.2, 0.01);
    prepared.result().unwrap().section_bits().values()
}
fn n06_source(sign: f64) -> PrimitiveSource {
    let [a, i, j, _, _] = n06_section();
    PrimitiveSource::new(SourceParts {
        nodes: vec![[0.0; 3], [2.0, 0.0, 0.0]],
        members: vec![StraightMember {
            id: 7, node_i: 0, node_j: 1, elastic_modulus: 2e11, shear_modulus: 8e10,
            area: a, second_moment_y: i, second_moment_z: i, torsion_constant: j,
            y_reference: [0.0, 1.0, 0.0],
        }],
        constraints: [Component::Ux, Component::Uy, Component::Uz, Component::Ry, Component::Rz]
            .iter().map(|&c| Constraint { dof: Dof { node: 0, component: c }, value: 0.0 }).collect(),
        springs: vec![super::super::source::Spring { id: 9, dof: Dof { node: 0, component: Component::Rx }, stiffness: N06_SPRING }],
        loads: vec![NodalLoad { dof: Dof { node: 1, component: Component::Rx }, value: sign * N06_TORQUE, source_id: "tip-MX".into() }],
        stations: [0.0, 0.5, 1.0].iter().enumerate()
            .map(|(k, &fraction)| Station { id: 17 + k as u32, member: 7, fraction }).collect(),
        supports: vec![SupportGroup { id: 3, node: 0, restrained: [true, true, true, false, true, true],
            springs: vec![9], directional_springs: vec![] }],
        ..SourceParts::default()
    })
    .unwrap()
}
fn n06_laws(owner: &RetainedSolve, diameter: f64) -> [ProposedMemberLaw<'_>; 1] {
    [ProposedMemberLaw {
        member: &owner.source().members()[0],
        diameter,
        effective_wall: 0.01,
        material: MaterialOperands::Ordinary { e: 2e11, g: 8e10 },
        represented_z: n06_section()[3],
    }]
}
fn f64_of(w: &Wide<16>) -> f64 {
    match w.to_binary64() {
        Binary64Outcome::Normal(v) | Binary64Outcome::Subnormal { value: v, .. } => v,
        o => panic!("{o:?}"),
    }
}
fn directed_quotient(a: f64, b: f64) -> (Wide<16>, Wide<16>) {
    use super::super::directed::{div_toward, Toward};
    let mut ctx = WideContext::<16>::new(1024).unwrap();
    let mut sum = ExactWideSum::new();
    let (a, b) = (Wide::<16>::from_f64(a).unwrap(), Wide::<16>::from_f64(b).unwrap());
    (div_toward(&mut ctx, &mut sum, &a, &b, Toward::Down).unwrap(),
     div_toward(&mut ctx, &mut sum, &a, &b, Toward::Up).unwrap())
}
/// Every lane row of `rows` that the closed form fixes: root rotation M/k, both end
/// torques +-M, the spring action -M, and a relative width below 2^-30 for each. (This lane
/// starts from the published binary64 center; the product's seeded order is the sharper control
/// `n06_seeded_congruence_lane_encloses_the_closed_form`.)
fn n06_closed_form(s: &RetainedSolve, rows: &[super::super::product_certificate::Enclosure], sign: f64) {
    use super::super::directed::{sub_toward, Toward};
    let m = sign * N06_TORQUE;
    let mut checked = 0;
    for (i, row) in s.publish().rows.iter().enumerate() {
        let truth = match row.id {
            QuantityId::Displacement(d) if d.node == 0 && d.component == Component::Rx => directed_quotient(m, N06_SPRING),
            QuantityId::EndAction { end, component: Component::Rx, .. } => point(if end == End::I { -m } else { m }),
            QuantityId::SpringAction { .. } => point(-m), // -k*theta = -M on the structure
            _ => continue,
        };
        contains(&rows[i], truth, &format!("N06 closed form {:?}", row.id));
        let (lo, hi) = rows[i].endpoints();
        let mut ctx = WideContext::<16>::new(1024).unwrap();
        let mut sum = ExactWideSum::new();
        let width = sub_toward(&mut ctx, &mut sum, hi, lo, Toward::Up).unwrap();
        let scale = truth.1.abs().mul_pow2(-30).unwrap();
        assert_ne!(width.cmp_value(&scale), Cmp::Greater, "N06 width {:?}", row.id);
        checked += 1;
    }
    assert_eq!(checked, 4, "root rotation, two end torques, spring action");
}

/// Diagnosis (b7-plan N06 extension): at the published center, the existing contraction
/// condition alpha = RU(2*B_b * max eta) of the annular_source lane exceeds 1 because B_b
/// carries the soft spring's compliance while eta carries the stiff member's coefficient gap.
/// The admitted_k lane (zero gap) has alpha = 0. Operands are printed as N06_ALPHA_OPERANDS.
#[test]
fn n06_contraction_condition_operands() {
    for sign in [1.0, -1.0] {
        let s = solved(n06_source(sign));
        let input = n06_laws(&s, 0.2);
        let view = s.source_bridge_view(s.source(), &s.prep.identity, s.selected).result.unwrap();
        for law in [ReadoutLaw::AdmittedK, ReadoutLaw::AnnularSource] {
            let (blocks, members) = test_alpha_operands(&view, &input, law).unwrap();
            assert_eq!(blocks.len(), 1);
            let (b, beta, eta, alpha) = blocks[0];
            let gaps: Vec<String> = members[0].iter().map(|(c, d)| format!("{:e}/{:e}={:e}", f64_of(d), f64_of(c), f64_of(d) / f64_of(c))).collect();
            println!("N06_ALPHA_OPERANDS sign={sign} law={law:?} block={b} n_b={} scales={:?} beta=2*B_b={:e} eta_max={:e} alpha={:e} gaps(dd/c_K)[EA,GJ,EIz,EIy]={gaps:?}",
                view.group().blocks.positions[b].len(), view.scales(), f64_of(&beta), f64_of(&eta), f64_of(&alpha));
            match law {
                ReadoutLaw::AdmittedK => assert!(alpha.is_zero()),
                ReadoutLaw::AnnularSource => assert_ne!(alpha.cmp_value(&Wide::<16>::ONE), Cmp::Less, "the existing condition fails"),
            }
        }
        // The existing law alone refuses with the same alpha.
        let spent = run(&s, &input);
        assert!(matches!(spent.result(), Err(BridgeError::Alpha { block: 0, .. })), "{:?}", spent.result().err());
    }
}

/// The congruence radius law certifies N06's annular_source lane: rows enclose the closed
/// form, and the lane spends positive extra work only through its existing counters.
#[test]
fn n06_congruence_radius_encloses_the_closed_form() {
    for sign in [1.0, -1.0] {
        let s = solved(n06_source(sign));
        let input = n06_laws(&s, 0.2);
        let contraction = run(&s, &input);
        assert!(matches!(contraction.result(), Err(BridgeError::Alpha { block: 0, .. })));
        let spent = source_residual_with_radius(&s, s.source(), &s.prep.identity, s.selected, &input, RadiusLaw::ContractionOrCongruence);
        let native = spent.result().unwrap_or_else(|e| panic!("congruence lane refused: {e:?}"));
        assert_ne!(native.alpha[0].cmp_value(&Wide::<16>::ONE), Cmp::Less, "alpha is still the existing value");
        println!("N06_CONGRUENCE sign={sign} epsilon={:e} member_builds={:?} visits={:?} contraction_member_builds={:?}",
            f64_of(&native.epsilon[0]), spent.work.member_builds.exact(), spent.work.visits.exact(), contraction.work.member_builds.exact());
        n06_closed_form(&s, &native.rows, sign);
        // The lane's block radius is exactly the documented law, formed here independently:
        // beta = 2*B_b and the coefficient gaps from the diagnosis operands, omega from the
        // lane's own final residual, k = ceil(sqrt(n_b)) by integer search.
        let view = s.source_bridge_view(s.source(), &s.prep.identity, s.selected).result.unwrap();
        let (blocks, members) = test_alpha_operands(&view, &input, ReadoutLaw::AnnularSource).unwrap();
        let beta = blocks[0].1;
        let mut delta = Wide::<16>::ZERO;
        for (c, d) in members.iter().flatten() {
            let gap = directed_op('/', d, c, true);
            if gap.cmp_value(&delta) == Cmp::Greater {
                delta = gap;
            }
        }
        let positions = &view.group().blocks.positions[0];
        let mut omega = Wide::<16>::ZERO;
        for &a in positions {
            let (lo, hi) = native.rho[a].endpoints();
            for v in [lo.abs(), hi.abs()] {
                if v.cmp_value(&omega) == Cmp::Greater {
                    omega = v;
                }
            }
        }
        let k = (1u32..).find(|k| (k * k) as usize >= positions.len()).unwrap();
        let expected = independent_radius(beta, omega, delta, k);
        assert_eq!(native.epsilon[0], expected, "epsilon = RU(RU(RU(beta*k)*omega)/RD(1-delta))");
        assert!(!omega.is_zero() && !delta.is_zero());
        // One extra member build (delta) beyond the contraction law's complete run.
        assert!(spent.work.status().is_exact());
        assert!(spent.work.member_builds.exact().unwrap() > contraction.work.member_builds.exact().unwrap());
        // The admitted_k lane never enters the extension and is unchanged.
        let k = source_residual_for_law(&s, s.source(), &s.prep.identity, s.selected, &input, ReadoutLaw::AdmittedK);
        assert!(k.result().unwrap().alpha[0].is_zero());
    }
}

/// Class predicate and its named refusal: a proposed law whose relative coefficient gap
/// is about 2^-31 (just outside 2^-32) refuses as `Congruence`, published as the alpha
/// condition; a gap of about 2^-36 is inside and certifies its own (perturbed) law.
#[test]
fn n06_congruence_class_refuses_outside_and_admits_inside() {
    let s = solved(n06_source(1.0));
    for (scale, inside) in [(1.0 + 2f64.powi(-31), false), (1.0 + 2f64.powi(-36), true)] {
        let input = n06_laws(&s, 0.2 * scale);
        let spent = source_residual_with_radius(&s, s.source(), &s.prep.identity, s.selected, &input, RadiusLaw::ContractionOrCongruence);
        if inside {
            spent.result().unwrap_or_else(|e| panic!("inside the class: {e:?}"));
        } else {
            let e = spent.result().err().unwrap();
            assert!(matches!(e, BridgeError::Congruence { block: 0, .. }), "{e:?}");
            assert!(matches!(bridge_failure(&e), BridgeFailure::AlphaCondition { block: 0 }));
            assert_eq!(spent.work.correction.calls.exact(), Ok(0), "refused before any correction");
        }
    }
    // A cantilever whose contraction condition holds never forms delta: the lane's work and
    // rows are identical under either law.
    let s = solved(source(true, false));
    let input = laws(&s);
    let a = run(&s, &input);
    let b = source_residual_with_radius(&s, s.source(), &s.prep.identity, s.selected, &input, RadiusLaw::ContractionOrCongruence);
    let (ra, rb) = (a.result().unwrap(), b.result().unwrap());
    assert_eq!(format!("{:?}", ra.rows), format!("{:?}", rb.rows));
    assert_eq!(format!("{:?}{:?}", ra.alpha, ra.epsilon), format!("{:?}{:?}", rb.alpha, rb.epsilon));
    assert_eq!(a.work.member_builds.exact(), b.work.member_builds.exact());
    assert_eq!(a.work.visits.exact(), b.work.visits.exact());
    let (mut ca, mut cb) = (Default::default(), Default::default());
    assert_eq!(format!("{:?}", a.work.typed_trace(&mut ca).numeric.entries), format!("{:?}", b.work.typed_trace(&mut cb).numeric.entries));
}

/// One directed binary1024 operation, independent of the lane's numeric record.
fn directed_op(op: char, a: &Wide<16>, b: &Wide<16>, up: bool) -> Wide<16> {
    use super::super::directed::{add_toward, div_toward, mul_toward, sub_toward, Toward};
    let t = if up { Toward::Up } else { Toward::Down };
    let mut ctx = WideContext::<16>::new(1024).unwrap();
    let mut sum = ExactWideSum::new();
    match op {
        '+' => add_toward(&mut ctx, &mut sum, a, b, t),
        '-' => sub_toward(&mut ctx, &mut sum, a, b, t),
        '*' => mul_toward(&mut ctx, &mut sum, a, b, t),
        _ => div_toward(&mut ctx, &mut sum, a, b, t),
    }
    .unwrap()
}
/// RU(RU(RU(beta*k)*omega)/RD(1-delta)), formed outside the kernel's lane code.
fn independent_radius(beta: Wide<16>, omega: Wide<16>, delta: Wide<16>, k: u32) -> Wide<16> {
    let k = Wide::<16>::from_f64(f64::from(k)).unwrap();
    let numerator = directed_op('*', &directed_op('*', &beta, &k, true), &omega, true);
    directed_op('/', &numerator, &directed_op('-', &Wide::<16>::ONE, &delta, false), true)
}

/// Closed-form controls of RU(RU(RU(beta*ceil(sqrt n))*omega)/RD(1-delta)) and its range refusals.
#[test]
fn congruence_radius_closed_form_and_range_controls() {
    let w = |x: f64| Wide::<16>::from_f64(x).unwrap();
    for (n, k) in [(1usize, 1.0), (4, 2.0), (5, 3.0), (9, 3.0), (10, 4.0), (256, 16.0), (257, 17.0)] {
        let (r, work) = test_congruence_radius(w(2.0), w(3.0), Wide::<16>::ZERO, n);
        assert_eq!(r.unwrap(), w(6.0 * k), "n={n}");
        assert!(work.status().is_exact());
    }
    // delta = 2^-32 exactly: 18/(1-2^-32) rounded up lies in (18, 18*(1+2^-31)].
    let (r, _) = test_congruence_radius(w(2.0), w(3.0), w(2f64.powi(-32)), 5);
    let r = r.unwrap();
    assert_eq!(r.cmp_value(&w(18.0)), Cmp::Greater);
    assert_ne!(r.cmp_value(&w(18.0 * (1.0 + 2f64.powi(-31)))), Cmp::Greater);
    // Inexact quotients: the result is the upward quotient exactly, and among these cases the
    // upward quotient differs from the nearest one (and from the downward one), so a
    // downward or nearest final division cannot pass.
    let mut nearest_differs = false;
    for omega in [3.0, 5.0, 7.0, 11.0, 13.0, 0.1, 1e-36] {
        for delta in [2f64.powi(-32), 3.0 * 2f64.powi(-40), 4e-17] {
            for n in [1usize, 7] {
                let (r, _) = test_congruence_radius(w(4.194304e18), w(omega), w(delta), n);
                let r = r.unwrap();
                let k = (1u32..).find(|k| (k * k) as usize >= n).unwrap();
                let up = independent_radius(w(4.194304e18), w(omega), w(delta), k);
                assert_eq!(r, up, "omega={omega} delta={delta} n={n}");
                let numerator = directed_op('*', &directed_op('*', &w(4.194304e18), &w(f64::from(k)), true), &w(omega), true);
                let denominator = directed_op('-', &Wide::<16>::ONE, &w(delta), false);
                let down = directed_op('/', &numerator, &denominator, false);
                let nearest = WideContext::<16>::new(1024).unwrap().div(&numerator, &denominator).unwrap();
                assert_eq!(up.cmp_value(&down), Cmp::Greater, "inexact: omega={omega} delta={delta}");
                nearest_differs |= up.cmp_value(&nearest) != Cmp::Equal;
            }
        }
    }
    assert!(nearest_differs, "some case's upward quotient is not its nearest quotient");
    // Overflow and underflow of the exponent range refuse as ExponentRange; they never wrap or
    // flush to zero.
    let huge = Wide::<16>::ONE.mul_pow2((1 << 61) + 16).unwrap();
    let tiny = Wide::<16>::ONE.mul_pow2(-(1 << 61) - 16).unwrap();
    for (beta, omega) in [(huge, huge), (tiny, tiny)] {
        let (r, _) = test_congruence_radius(beta, omega, Wide::<16>::ZERO, 4);
        assert!(matches!(r, Err(BridgeError::Numeric(super::super::product_certificate::NumericError::Arithmetic(AttemptStop::Exponent)))), "{r:?}");
    }
    // delta at or above 1 refuses by name.
    let (r, _) = test_congruence_radius(w(2.0), w(3.0), Wide::<16>::ONE, 4);
    assert!(matches!(r, Err(BridgeError::Congruence { .. })));
}

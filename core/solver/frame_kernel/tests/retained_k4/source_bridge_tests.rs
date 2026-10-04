//! Actual native owner -> checked view -> conditional source bridge witnesses.
//! The exact oracle is generated independently from closed cantilever equations.
use super::super::product_certificate::{bridge::*, MaterialOperands};
use super::super::recover::End;
use super::super::source::{
    Component, Constraint, NodalLoad, SourceParts, Station, StraightMember, SupportGroup,
};
use super::*;
use std::cmp::Ordering as Cmp;
#[path = "source_bridge_vectors.rs"]
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
fn run<'a>(s: &'a RetainedSolve, law: &'a [ProposedMemberLaw<'a>]) -> BridgeSpent<'a> {
    source_bridge(
        s,
        s.source(),
        &s.evidence().source_encoding,
        s.selected_precision(),
        law,
    )
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
fn check_rows(label: &str, s: &RetainedSolve, native: &ConditionalNativeSource<'_>, loaded: bool) {
    assert_eq!(native.rows.len(), s.publish().rows.len());
    for (i, row) in s.publish().rows.iter().enumerate() {
        let (glo, ghi) = exact(row.id, true, loaded);
        let (klo, khi) = exact(row.id, false, loaded);
        let (lo, hi) = native.rows[i].source.endpoints();
        assert_ne!(
            lo.cmp_value(&glo),
            Cmp::Greater,
            "{label} {:?} lower",
            row.id
        );
        assert_ne!(hi.cmp_value(&ghi), Cmp::Less, "{label} {:?} upper", row.id);
        let (_, radius) = native.view.row(i);
        let scale =
            f64::from_bits(s.publish().body_scales[row.body as usize * 4 + row.kind.index()].2);
        let x = row.value.value().unwrap();
        let (class, bound) = match row.class {
            RowClass::InputDerived => ("input", 0),
            RowClass::AbsoluteVerified { bound_bits } => ("absolute", bound_bits),
            RowClass::RelativeVerified => ("relative", 0),
            _ => panic!(),
        };
        println!("I42_ROW {{\"witness\":\"{label}\",\"index\":{i},\"id\":\"{:?}\",\"x\":\"{:016x}\",\"radius\":\"{}\",\"class\":\"{class}\",\"bound\":\"{bound:016x}\",\"scale\":\"{:016x}\",\"sharper\":\"{:016x}\",\"error\":{},\"lo\":{},\"hi\":{},\"klo\":{},\"khi\":{},\"glo\":{},\"ghi\":{}}}",row.id,x.to_bits(),radius.map(|r|format!("{:016x}",r.to_bits())).unwrap_or("absent".into()),scale.to_bits(),sharper_binary64(x,scale).unwrap().to_bits(),encoded(&native.rows[i].source_error),encoded(lo),encoded(hi),encoded(&klo),encoded(&khi),encoded(&glo),encoded(&ghi));
    }
}
#[test]
fn source_bridge_zero_native_retained_source_oracle() {
    let s = solved(source(false, false));
    let input = laws(&s);
    let spent = run(&s, &input);
    let native = spent.result().unwrap();
    assert_eq!(native.view.data(), &[false]);
    println!(
        "I42_ZERO_STATE {{\"p\":{},\"P\":{},\"data\":{:?},\"body_bounds\":[]}}",
        s.selected_precision(),
        s.evidence().verification_precision,
        native.view.data()
    );
    assert!(s.evidence().certified_bound.is_empty());
    assert!(native.rows.iter().all(|v| v.source_error.is_zero()));
    assert!(native.tau.iter().all(Wide::<16>::is_zero));
    assert_eq!(spent.work.member_builds.exact(), Ok(3));
    assert_eq!(spent.work.b_products.exact(), Ok(336));
    assert_eq!(spent.work.d_products.exact(), Ok(40));
    assert_eq!(spent.work.h_products.exact(), Ok(16));
    check_rows("zero", &s, native, false);
}
#[test]
fn source_bridge_loaded_native_retained_source_oracle() {
    let s = solved(source(true, false));
    let original = s.publish().clone();
    let input = laws(&s);
    let spent = run(&s, &input);
    let native = spent.result().unwrap();
    assert_eq!(native.view.data(), &[true]);
    assert_eq!(
        s.evidence().verification_precision,
        2 * s.selected_precision()
    );
    assert!(native
        .alpha
        .iter()
        .all(|a| !a.is_zero() && a.cmp_value(&Wide::<16>::ONE) == Cmp::Less));
    assert!(native.tau.iter().all(|t| !t.is_zero()));
    assert_eq!(s.publish(), &original);
    assert_eq!(spent.work.member_builds.exact(), Ok(3));
    assert_eq!(spent.work.b_products.exact(), Ok(336));
    assert_eq!(spent.work.d_products.exact(), Ok(40));
    assert_eq!(spent.work.h_products.exact(), Ok(16));
    println!(
        "I42_BLOCK {{\"scales\":{:?},\"bound\":\"{:016x}\",\"alpha\":{},\"tau\":{}}}",
        native.view.scales(),
        s.evidence().certified_bound[0].1,
        encoded(&native.alpha[0]),
        encoded(&native.tau[0])
    );
    println!(
        "I42_SELECTED_STATE {{\"p\":{},\"P\":{},\"data\":{:?}}}",
        s.selected_precision(),
        s.evidence().verification_precision,
        native.view.data()
    );
    println!("I42_WORK {:?}", spent.work);
    check_rows("loaded", &s, native, true);
}
#[test]
fn source_bridge_cancelled_individual_terms_are_data() {
    let s = solved(source(false, true));
    let input = laws(&s);
    let spent = run(&s, &input);
    let native = spent.result().unwrap();
    assert_eq!(native.view.data(), &[true]);
    assert_eq!(s.evidence().certified_bound.len(), 1);
    assert!(!native.alpha[0].is_zero());
    assert!(native.tau[0].is_zero());
    assert!(native.rows.iter().all(|v| v.source_error.is_zero()));
}
#[test]
fn source_bridge_actual_alpha_refusal_keeps_work_and_publication() {
    let s = solved(source(true, false));
    let original = s.publish().clone();
    let mut input = laws(&s);
    input[0].diameter = 0.2;
    let spent = run(&s, &input);
    assert!(
        matches!(spent.result(), Err(BridgeError::Alpha { block: 0, .. })),
        "{:?}",
        spent.result()
    );
    if let Err(BridgeError::Alpha { alpha_hi, .. }) = spent.result() {
        assert_ne!(alpha_hi.cmp_value(&Wide::<16>::ONE), Cmp::Less);
        println!("I42_ALPHA_REFUSAL {}", encoded(&alpha_hi));
    }
    assert_eq!(spent.work.member_builds.exact(), Ok(2));
    assert!(spent.work.status().fault().is_none());
    assert_eq!(s.publish(), &original);
}
#[test]
fn source_bridge_owner_source_radius_precision_and_absence_discrimination() {
    let s = solved(source(true, false));
    let source_clone = s.source().clone();
    assert!(matches!(
        s.source_bridge_view(&source_clone, &s.prep.identity, s.selected)
            .result,
        Err(SourceBridgeViewIssue::ForeignOwner)
    ));
    let mut foreign_laws = laws(&s);
    foreign_laws[0].member = &source_clone.members()[0];
    assert!(matches!(
        run(&s, &foreign_laws).result(),
        Err(BridgeError::MemberOwner)
    ));
    let wrong_precision = s.source_bridge_view(s.source(), &s.prep.identity, 512);
    assert!(matches!(
        wrong_precision.result,
        Err(SourceBridgeViewIssue::Certificate(
            CertificateIssue::Precision
        ))
    ));
    assert_eq!(wrong_precision.work.visits.exact(), Ok(0));
    let mut id = s.prep.identity.clone();
    id[0] ^= 1;
    let wrong_source = s.source_bridge_view(s.source(), &id, s.selected);
    assert!(matches!(
        wrong_source.result,
        Err(SourceBridgeViewIssue::Certificate(
            CertificateIssue::PairIdentity
        ))
    ));
    assert_eq!(wrong_source.work.visits.exact(), Ok(1));
    for fault in 0..9 {
        let mut changed = (*s).clone();
        match fault {
            0 => changed.publication_radius_bits[6] = ABSENT_RADIUS_BITS,
            1 => changed.publication_radius_bits[0] = 0,
            2 => changed.publication_radius_bits[6] = (-0f64).to_bits(),
            3 => changed.evidence.certified_bound.clear(),
            4 => changed.evidence.verification_precision = 128,
            5 => changed.cache.s256 = None,
            6 => changed.evidence.ledger_encoding[0] ^= 1,
            7 => changed.publication.rows.swap(0, 1),
            8 => changed.publication.rows[8].value = Binary64Outcome::Normal(f64::NAN),
            _ => unreachable!(),
        }
        assert!(
            changed
                .source_bridge_view(changed.source(), &changed.prep.identity, changed.selected)
                .result
                .is_err(),
            "fault {fault}"
        );
    }
    let mut changed = (*s).clone();
    let old = &changed.group;
    let mut group = GroupPrep {
        structure: old.structure.clone(),
        ordering: old.ordering.clone(),
        geometry: old.geometry.clone(),
        blocks: old.blocks.clone(),
    };
    group.ordering.position[6] = usize::MAX;
    changed.group = Arc::new(group);
    assert!(matches!(
        changed
            .source_bridge_view(changed.source(), &changed.prep.identity, changed.selected)
            .result,
        Err(SourceBridgeViewIssue::Ordering)
    ));
}
#[test]
fn source_bridge_numeric_refusal_prefix_is_retained() {
    let s = solved(source(true, false));
    let mut input = laws(&s);
    input[0].material = MaterialOperands::ExactENu {
        e: f64::from_bits(oracle::E),
        nu: 0.5,
    };
    let spent = run(&s, &input);
    assert!(matches!(
        spent.result(),
        Err(BridgeError::Numeric(
            super::super::product_certificate::NumericError::InvalidMaterial
        ))
    ));
    assert_eq!(spent.work.member_builds.exact(), Ok(1));
    assert!(spent.work.status().fault().is_none());
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
#[test]
fn source_bridge_prescribed_coupling_is_data_and_enters_residual_majorant() {
    let mut parts = parts_from(&source(false, false));
    parts.constraints[0].value = 0.0001;
    let s = solved(PrimitiveSource::new(parts).unwrap());
    let input = laws(&s);
    let spent = run(&s, &input);
    let native = spent.result().unwrap();
    assert_eq!(native.view.data(), &[true]);
    assert!(!native.tau[0].is_zero()); // all actual ledger terms zero; prescribed columns remain
                                       // Exact common rigid translation is a source solution: all actions zero,
                                       // node Ux and the native displacement magnitude equal the prescription.
    for (i, row) in s.publish().rows.iter().enumerate() {
        let truth = match row.id {
            QuantityId::Displacement(d) if d.component == Component::Ux => 0.0001,
            QuantityId::DisplacementMagnitude(_) => 0.0001,
            _ => 0.0,
        };
        let v = Wide::<16>::from_f64(truth).unwrap();
        let (lo, hi) = native.rows[i].source.endpoints();
        assert_ne!(lo.cmp_value(&v), Cmp::Greater, "{:?}", row.id);
        assert_ne!(hi.cmp_value(&v), Cmp::Less, "{:?}", row.id);
    }
}
#[test]
fn source_bridge_missing_anchor_is_missing_warrant_not_singularity() {
    let mut parts = parts_from(&source(false, false));
    let dof = parts.constraints.remove(0).dof;
    parts.springs.push(super::super::source::Spring {
        id: 9,
        dof,
        stiffness: 1e9,
    });
    parts.supports[0].restrained[0] = false;
    parts.supports[0].springs.push(9);
    let s = solved(PrimitiveSource::new(parts).unwrap()); // actual native source is selected
    let input = laws(&s);
    let spent = run(&s, &input);
    assert!(matches!(
        spent.result(),
        Err(BridgeError::MissingUniquenessWarrant(0))
    ));
    assert!(s.evidence().certified_bound.is_empty());
    assert_eq!(spent.work.member_builds.exact(), Ok(2));
}
#[test]
fn source_bridge_joined_work_fault_blocks_success_and_keeps_numeric_reason() {
    let s = solved(source(false, false));
    let input = laws(&s);
    let mut spent = run(&s, &input);
    assert!(spent.result().is_ok());
    spent.work.visits = WorkTotal::exact_count(u64::MAX).add(WorkTotal::exact_count(1));
    assert!(matches!(
        spent.result(),
        Err(BridgeError::Numeric(
            super::super::product_certificate::NumericError::Arithmetic(
                AttemptStop::WorkAccounting(_)
            )
        ))
    ));
    let mut input = laws(&s);
    input[0].effective_wall = 0.1;
    let mut refused = run(&s, &input);
    refused.work.visits = WorkTotal::exact_count(u64::MAX).add(WorkTotal::exact_count(1));
    assert!(matches!(
        refused.result(),
        Err(BridgeError::Numeric(
            super::super::product_certificate::NumericError::InvalidGeometry
        ))
    ));
    assert!(refused.work.status().fault().is_some()); // simultaneous cause retained separately
}

#[test]
fn source_bridge_rv56_scalar_actual_prefix_and_legacy_projection() {
    for (value, scale) in [
        (1.0, 1.0),
        (f64::MAX, f64::MAX),
        (f64::from_bits(1), 0.0),
        (-1.0, 2.0),
    ] {
        let spent = sharper_binary64_spent(value, scale);
        assert_eq!(spent.f64_operations, 5);
        assert_eq!(
            spent.result.unwrap().to_bits(),
            sharper_binary64(value, scale).unwrap().to_bits()
        );
    }
    for (value, scale, expected) in [
        (f64::NAN, 1.0, CertificateIssue::NonFinite),
        (f64::INFINITY, 1.0, CertificateIssue::NonFinite),
        (1.0, f64::NAN, CertificateIssue::NonFinite),
        (1.0, -0.0, CertificateIssue::NonCanonicalZero),
        (f64::NAN, -1.0, CertificateIssue::NegativeField), // scale validation precedes x
    ] {
        let spent = sharper_binary64_spent(value, scale);
        assert_eq!(spent.f64_operations, 0);
        assert_eq!(spent.result, Err(expected));
        assert_eq!(sharper_binary64(value, scale), Err(expected));
    }
}

#[test]
fn source_bridge_rv56_radius_early_zero_post_ceiling_five_and_success() {
    let s = solved(source(true, false));
    let index = s
        .publication
        .rows
        .iter()
        .position(|row| matches!(row.class, RowClass::RelativeVerified))
        .unwrap();
    assert_eq!(index, 6); // first actual relative row of the unchanged native fixture
    let meta = s.prep.layout[index];
    let valid = s.publication_radius_checked_spent(index, meta);
    assert_eq!(valid.f64_operations, 5);
    assert_eq!(valid.result, s.publication_radius_checked(index, meta));
    let whole = s.source_bridge_view(s.source(), &s.prep.identity, s.selected);
    assert!(whole.result.is_ok());
    assert_eq!(whole.work.f64_operations.exact(), Ok(145));
    let original = s.publication.clone();
    let mut excessive = (*s).clone();
    // RV56's unchanged corruption control: the finite radius is rejected only
    // after the actual sharper ceiling has been produced. Not natural reach.
    excessive.publication_radius_bits[index] = 1.0f64.to_bits();
    let spent = excessive.publication_radius_checked_spent(index, meta);
    assert_eq!(spent.f64_operations, 5);
    assert_eq!(spent.result, Err(CertificateIssue::RadiusClassMismatch));
    assert_eq!(
        excessive.publication_radius_checked(index, meta),
        spent.result
    );
    let checked = excessive.source_bridge_view(
        excessive.source(),
        &excessive.prep.identity,
        excessive.selected,
    );
    assert!(matches!(
        checked.result,
        Err(SourceBridgeViewIssue::Certificate(
            CertificateIssue::RadiusClassMismatch
        ))
    ));
    assert_eq!(checked.work.f64_operations.exact(), Ok(5));
    let input = laws(&excessive);
    let bridge = run(&excessive, &input);
    assert!(matches!(
        bridge.result(),
        Err(BridgeError::View(SourceBridgeViewIssue::Certificate(
            CertificateIssue::RadiusClassMismatch
        )))
    ));
    assert_eq!(bridge.work.view.f64_operations.exact(), Ok(5));
    assert_eq!(excessive.publication, original);
    println!("RV56_AUTHOR_PREFIX post_ceiling=5 successful_row=5 successful_view=145");
    for fault in 0..7 {
        let mut early = (*s).clone();
        match fault {
            0 => early.publication.rows[index].id = QuantityId::Displacement(Dof::from_global(0)),
            1 => early.publication_radius_bits[index] = ABSENT_RADIUS_BITS,
            2 => early.publication_radius_bits[index] = (-0.0f64).to_bits(),
            3 => early.publication.rows[index].value = Binary64Outcome::Normal(f64::NAN),
            4 => early.publication.body_scales[0].2 = f64::NAN.to_bits(),
            5 => early.publication.body_scales[0].0 = u32::MAX,
            6 => early.publication.rows.truncate(index),
            _ => unreachable!(),
        }
        let spent = early.publication_radius_checked_spent(index, meta);
        assert!(spent.result.is_err(), "fault {fault}");
        assert_eq!(spent.f64_operations, 0, "fault {fault}");
        assert_eq!(early.publication_radius_checked(index, meta), spent.result);
        let checked =
            early.source_bridge_view(early.source(), &early.prep.identity, early.selected);
        assert!(checked.result.is_err(), "view fault {fault}");
        assert_eq!(
            checked.work.f64_operations.exact(),
            Ok(0),
            "view fault {fault}"
        );
    }
}

#[test]
fn source_bridge_rv56_collect_before_propagate_keeps_simultaneous_faults() {
    let s = solved(source(true, false));
    let mut work = SourceBridgeViewWork {
        visits: WorkTotal::zero(),
        f64_operations: WorkTotal::exact_count(u64::MAX),
        ..SourceBridgeViewWork::default()
    };
    let result = s.build_source_bridge_view(s.source(), &s.prep.identity, s.selected, &mut work);
    assert!(matches!(result, Err(SourceBridgeViewIssue::Work(_))));
    assert!(work.f64_operations.status().fault().is_some());
    let mut s = (*s).clone();
    s.publication_radius_bits[6] = 1.0f64.to_bits();
    let mut work = SourceBridgeViewWork {
        visits: WorkTotal::zero(),
        f64_operations: WorkTotal::exact_count(u64::MAX),
        ..SourceBridgeViewWork::default()
    };
    let result = s.build_source_bridge_view(s.source(), &s.prep.identity, s.selected, &mut work);
    assert!(matches!(
        result,
        Err(SourceBridgeViewIssue::Certificate(
            CertificateIssue::RadiusClassMismatch
        ))
    ));
    assert!(work.f64_operations.status().fault().is_some());
    println!(
        "RV56_AUTHOR_COLLECTION numeric_cause=RadiusClassMismatch joined_accounting_fault=true"
    );
}

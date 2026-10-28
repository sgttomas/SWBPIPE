use super::super::formation_check::{IntensityPrimitive, LoadSummandIdentity};
use super::*;
use crate::{FrameNode, FrameSection};
fn bound(n: usize) -> MemberBoundInputs {
    MemberBoundInputs {
        w: vec![0.0; n],
        w2: vec![0.0; n],
        exponents: vec![0; n],
        lambda_sigma: 0.0,
    }
}
// Historical chord control is test-only; the product has one current frame.
fn chord_control_recovery(arc:&CurvedFormation,span:&FrameElement,loads:&CertifiedMemberLoads,u:&[f64],bound:&MemberBoundInputs)->Result<CertifiedMemberRecovery,RecoveryFailure> {
    let mut a=WideArith::new(128)?;
    let matrix=curved_matrix(&mut a,arc)?;
    let (axes,_)=chord_axes(&mut a,span.node_i.coordinates,span.node_j.coordinates,span.y_reference)?;
    let operator=local_operator(&matrix,&axes)?;
    let (equivalent,scale)=local_loads(loads,&axes)?;
    let ends=evaluate_ends(&operator,&element_dof_map(arc.node_i,arc.node_j),&equivalent,&scale,u,bound)?;
    let mut tangent=certify_arc_recovery(arc,loads,u,bound)?;tangent.ends=ends;Ok(tangent)
}
fn frame() -> FrameElement {
    FrameElement::new(
        FrameNode::new(0, [0.0; 3]).unwrap(),
        FrameNode::new(1, [2.0, 0.0, 0.0]).unwrap(),
        FrameSection::new(8.0, 4.0, 2.0, 3.0, 3.0, 6.0).unwrap(),
        [0.0, 1.0, 0.0],
    )
    .unwrap()
}
/// T4-U8 (Design §4 item 2): a certified straight recovery is bound to its
/// member's formation-source frame. These controls' one member is `MEMBER`.
const MEMBER: &str = "member:1:m:";
fn member_source(frame: &FrameElement) -> FormationSource<'static> {
    FormationSource {
        node_count: 2,
        frames: vec![*frame],
        frame_members: vec![MEMBER.into()],
        ..FormationSource::default()
    }
}
#[test]
fn exact_projection_uses_guard_and_sticky_bits() {
    let mut acc = ExactAccumulator::new();
    acc.add(1.0).unwrap();
    acc.add_product(2.0_f64.powi(-70), 2.0_f64.powi(-70))
        .unwrap();
    let ball = sum_ball(&acc, 0.0).unwrap();
    assert_eq!(midpoint_publication(ball.m).unwrap(), 1.0);
    assert!(ball.r > 0.0);
    let mut cancel = ExactAccumulator::new();
    cancel.add(1e16).unwrap();
    cancel.add(-1e16).unwrap();
    cancel.add(1.0).unwrap();
    let ball = sum_ball(&cancel, 0.0).unwrap();
    assert_eq!(midpoint_publication(ball.m).unwrap(), 1.0);
    assert_eq!(ball.r, 0.0);
}
/// v_g = fl(m_g) (DEL-04-01 kd5-member-relative-bound.md §3.1): a nonzero
/// midpoint whose correct rounding is zero publishes 0, with its whole
/// magnitude kept in the row's rounding gap, instead of making the member
/// bound unavailable. Kills the regression that demoted F1b's PHYS-R4 W2 case
/// (E = 1 Pa, Iz ~ 1e-307 m^4) with "recovery publication".
#[test]
fn a_midpoint_below_half_the_least_subnormal_publishes_zero() {
    let tiny = |pow: i64| Wide2::from_f64(f64::from_bits(1)).unwrap().mul_pow2(pow).unwrap();
    for (m, published) in [
        (tiny(-10), 0.0),
        (tiny(-10).neg(), 0.0),
        (tiny(-1), 0.0), // exactly half the least subnormal: ties to even zero
        (tiny(0), f64::from_bits(1)),
    ] {
        let v = midpoint_publication(m).unwrap();
        assert_eq!(v.to_bits(), published.to_bits(), "{m:?}");
        let row = CertifiedRow::finish(Ball::exact(m), 0.0, 0.0).unwrap();
        assert_eq!(row.published.to_bits(), published.to_bits());
        if published == 0.0 {
            assert!(row.rounding_gap > 0.0, "the gap carries |m_g|");
        }
    }
    // Overflow stays unrepresentable.
    let huge = Wide2::from_f64(f64::MAX).unwrap().mul_pow2(1).unwrap();
    assert!(midpoint_publication(huge).is_err());
}
#[test]
fn corrections_are_not_binary64_summed_and_w2_stays_physical() {
    let op = [Ball::exact(Wide2::ONE)];
    let mut b = bound(1);
    let u = [1e16];
    b.w[0] = -1e16;
    b.w2[0] = 1.0;
    let row = evaluate_row(&op, &[0], Ball::ZERO, 0.0, &u, &b).unwrap();
    assert_eq!(row.published, 1.0);
    assert_eq!(row.radius, 0.0);
    // With odd b, D' need not be 2^(-b/2)D. This row uses the attempt's own
    // D'/lambda pair with physical coefficients and unscaled corrections.
    for (exponent, lambda, expected) in [(1, 0.5, 1.0), (-2, 4.0, 1.0), (3, 0.125, 1.0)] {
        b.exponents[0] = exponent;
        b.lambda_sigma = lambda;
        let row = evaluate_row(&op, &[0], Ball::ZERO, 0.0, &u, &b).unwrap();
        assert_eq!(row.published, 1.0);
        assert!(row.remainder >= expected && row.remainder < expected * (1.0 + 1e-14));
    }
}
#[test]
fn remainder_uses_each_free_column_and_l1_row_operator() {
    let op = [
        Ball::exact(Wide2::from_f64(3.0).unwrap()),
        Ball::exact(Wide2::from_f64(-7.0).unwrap()),
        Ball::exact(Wide2::from_f64(11.0).unwrap()),
    ];
    let mut b = bound(3);
    b.exponents = vec![1, -1, i32::MIN];
    b.lambda_sigma = 2.0;
    let row = evaluate_row(&op, &[0, 1, 2], Ball::ZERO, 0.0, &[0.0; 3], &b).unwrap();
    // A nonsymmetric row: (|3|*2 + |-7|/2)*2=19; prescribed 11 is absent.
    assert!(row.remainder >= 19.0 && row.remainder < 19.0 * (1.0 + 1e-14));
}
#[test]
fn uniform_straight_ends_and_all_sections_match_exact_statics() {
    let frame = frame();
    let mut primitives = Vec::new();
    for row in 0..12 {
        primitives.push(LoadPrimitive {
            term_index: row,
            identity: LoadSummandIdentity {
                family: "uniform_straight".into(),
                source: "invented".into(),
                ordinal: row,
            },
            local_row: row,
            formula: LoadFormula::UniformStraight {
                frame: frame.clone(),
                intensity: IntensityPrimitive::Authored([0.0, -6.0, 0.0]),
                start_fraction: 0.0,
                end_fraction: 1.0,
            },
            force_scale_exponent: 0,
        });
    }
    let source = member_source(&frame);
    let loads = CertifiedMemberLoads::from_primitives(0, 1, &primitives).unwrap().with_member_source(&source, MEMBER);
    assert_eq!(loads.uniform_loads().count(), 1);
    let recovered = certify_straight_recovery(&frame, &loads, &[0.0; 12], &bound(12)).unwrap();
    assert_eq!(recovered.ends[1].published, 6.0);
    assert_eq!(recovered.ends[5].published, 2.0);
    assert_eq!(recovered.ends[7].published, 6.0);
    assert_eq!(recovered.ends[11].published, -2.0);
    for (station, &fraction) in recovered.sections.iter().zip(&CERTIFIED_SECTION_FRACTIONS) {
        let x = 2.0 * fraction;
        let truth = [
            0.0,
            -6.0 + 6.0 * x,
            0.0,
            0.0,
            0.0,
            -2.0 + 6.0 * x - 3.0 * x * x,
        ];
        for r in 0..6 {
            assert!(
                (station[r].published - truth[r]).abs() <= station[r].bound(),
                "fraction {fraction}, row {r}: {:?}",
                station[r]
            );
        }
    }
}
/// One full-span authored uniform primitive per element row of `frame`.
fn sourced_uniform_primitives(frame: &FrameElement, source: &str, w: [f64; 3], start: usize) -> Vec<LoadPrimitive> {
    (0..12)
        .map(|row| LoadPrimitive {
            term_index: start + row,
            identity: LoadSummandIdentity {
                family: "uniform_straight".into(),
                source: source.into(),
                ordinal: row,
            },
            local_row: row,
            formula: LoadFormula::UniformStraight {
                frame: frame.clone(),
                intensity: IntensityPrimitive::Authored(w),
                start_fraction: 0.0,
                end_fraction: 1.0,
            },
            force_scale_exponent: 0,
        })
        .collect()
}
fn skew_frame() -> FrameElement {
    FrameElement::new(
        FrameNode::new(0, [0.0; 3]).unwrap(),
        FrameNode::new(1, [2.0, 1.0, 0.5]).unwrap(),
        FrameSection::new(8.0, 4.0, 2.0, 3.0, 3.0, 6.0).unwrap(),
        [0.0, 0.0, 1.0],
    )
    .unwrap()
}

/// Review finding F5 (DEL-04-01 kd5-member-relative-bound.md §3.1: m_g is one
/// exact sum of the stiffness products minus the member's intended load
/// terms). Same-expression +/-G cancellation control: probe A's (G, n, -G)
/// uniform loads at G = 1e80 on a skew member publish, row by row (ends and
/// all five sections), exactly the bits of the net load n alone, because each
/// primitive enters its row's exact sum before the one rounding and each
/// uniform term enters the station rows as its own addend; only the radii
/// grow. Precondition (paths differ): pre-summing the three balls at p, as
/// the recovery did before, absorbs n entirely.
#[test]
fn same_expression_cancelling_loads_publish_the_net_rows() {
    let frame = skew_frame();
    let (g, n) = (1e80, 0.3);
    let mut primitives = sourced_uniform_primitives(&frame, "g", [0.0, 0.0, g], 0);
    primitives.extend(sourced_uniform_primitives(&frame, "n", [0.0, 0.0, n], 12));
    primitives.extend(sourced_uniform_primitives(&frame, "-g", [0.0, 0.0, -g], 24));
    let net = sourced_uniform_primitives(&frame, "n", [0.0, 0.0, n], 0);
    // Precondition: the former 128-bit pre-sum of the three primitives loses n.
    let mut b = Balls::new().unwrap();
    let row = 2;
    let ball = |p: &LoadPrimitive| {
        let (m, r) = load_reformation(p).unwrap();
        Ball { m: m[row], r: r[row] }
    };
    let first = b.add(ball(&primitives[row]), ball(&primitives[12 + row])).unwrap();
    let presum = b.add(first, ball(&primitives[24 + row])).unwrap();
    let alone = ball(&net[row]);
    assert!(!alone.m.is_zero());
    assert_ne!(midpoint_publication(presum.m).unwrap(), midpoint_publication(alone.m).unwrap());
    let source = member_source(&frame);
    let cancelling = CertifiedMemberLoads::from_primitives(0, 1, &primitives).unwrap().with_member_source(&source, MEMBER);
    let reference = CertifiedMemberLoads::from_primitives(0, 1, &net).unwrap().with_member_source(&source, MEMBER);
    assert_eq!(cancelling.uniform_loads().count(), 3);
    let u = [1e-3, -2e-3, 3e-4, 1e-4, -5e-4, 2e-4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let bound = bound(12);
    let a = certify_straight_recovery(&frame, &cancelling, &u, &bound).unwrap();
    let e = certify_straight_recovery(&frame, &reference, &u, &bound).unwrap();
    let rows = |x: &CertifiedMemberRecovery| -> Vec<CertifiedRow> {
        x.ends.iter().chain(x.sections.iter().flatten()).cloned().collect()
    };
    let (a, e) = (rows(&a), rows(&e));
    assert_eq!(a.len(), 42);
    let mut grown = 0;
    for (i, (x, y)) in a.iter().zip(&e).enumerate() {
        assert_eq!(x.published.to_bits(), y.published.to_bits(), "row {i}: {x:?} against {y:?}");
        assert!(x.radius >= y.radius, "row {i}");
        grown += usize::from(x.radius > y.radius);
    }
    assert!(grown > 0, "the +/-G enclosure radii stay in the bound");
    // An arbitrary section takes the same recipe.
    for fraction in [0.125, 0.6] {
        let x = certify_straight_section(&frame, &cancelling, &u, &bound, fraction).unwrap();
        let y = certify_straight_section(&frame, &reference, &u, &bound, fraction).unwrap();
        for r in 0..6 {
            assert_eq!(x[r].published.to_bits(), y[r].published.to_bits(), "fraction {fraction} row {r}");
        }
    }
}

/// Review finding F5: an ordinary multi-primitive row. Two different uniform
/// loads on one member: each element row's intended load is the exact sum of
/// the two re-formed midpoints, projected once to p, with both radii; the
/// published end and station rows match the exact statics of the summed
/// intensity within their bounds.
#[test]
fn multi_primitive_rows_are_one_exact_sum_of_their_primitives() {
    let frame = frame();
    let mut primitives = sourced_uniform_primitives(&frame, "a", [0.0, -6.0, 0.0], 0);
    primitives.extend(sourced_uniform_primitives(&frame, "b", [0.0, 3.0, 1.5], 12));
    let source = member_source(&frame);
    let loads = CertifiedMemberLoads::from_primitives(0, 1, &primitives).unwrap().with_member_source(&source, MEMBER);
    for row in 0..12 {
        let mut sum = ExactAccumulator::new();
        let mut radius = 0.0;
        for p in primitives.iter().filter(|p| p.local_row == row) {
            let (m, r) = load_reformation(p).unwrap();
            add_ball(&mut sum, &mut radius, Ball { m: m[row], r: r[row] }, false).unwrap();
        }
        let expected = sum_ball(&sum, radius).unwrap();
        assert_eq!(loads.equivalent[row].m.cmp_value(&expected.m), std::cmp::Ordering::Equal, "row {row}");
        assert_eq!(loads.equivalent[row].r.to_bits(), expected.r.to_bits(), "row {row}");
    }
    let recovered = certify_straight_recovery(&frame, &loads, &[0.0; 12], &bound(12)).unwrap();
    // Total intensity (0, -3, 1.5) N/m over L = 2 m, fixed-end equivalents;
    // every station truth is a dyadic rational, exact in binary64.
    let (wy, wz) = (-3.0, 1.5);
    for (station, &fraction) in recovered.sections.iter().zip(&CERTIFIED_SECTION_FRACTIONS) {
        let x = 2.0 * fraction;
        let truth = [
            0.0,
            wy + (-wy) * x,
            wz + (-wz) * x,
            0.0,
            -(wz * 4.0 / 12.0 - wz * x + 0.5 * wz * x * x),
            wy * 4.0 / 12.0 - wy * x + 0.5 * wy * x * x,
        ];
        for r in 0..6 {
            assert!(
                (station[r].published - truth[r]).abs() <= station[r].bound(),
                "fraction {fraction}, row {r}: {:?} against {}",
                station[r],
                truth[r]
            );
        }
    }
    assert!(recovered.ends.iter().all(|row| row.radius.is_finite()));
}
#[test]
fn spring_support_on_pipe_sign_and_exact_signed_equilibrium() {
    let u = [0.25, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0];
    let mut b = bound(12);
    b.w[0] = 0.125;
    b.w2[0] = 0.0625;
    let spring = certify_spring_reaction(0, 16.0, 0.0, &u, &b).unwrap();
    assert_eq!(spring.published, -7.0);
    let source = FormationSource {
        node_count: 2,
        frames: vec![frame()],
        springs: vec![(0, 16.0)],
        ..Default::default()
    };
    let mut ledger = crate::load_ledger::LoadLedger::new();
    ledger.push("invented nodal", 0, 9.0);
    let force = ledger.finish(12).unwrap();
    let reaction = certify_support_reaction(&source, &force, 0, &u, &b).unwrap();
    // Axial member contribution: (EA/L)*(7/16-1/2)=-1/2; + spring 7 - input 9.
    assert_eq!(reaction.published, -2.5);
    for e in [-3, -1, 1, 3] {
        let mut scaled = b.clone();
        scaled.exponents.fill(e);
        scaled.lambda_sigma = 2.0_f64.powi(-e);
        let row = certify_spring_reaction(0, 16.0, 0.0, &u, &scaled).unwrap();
        assert_eq!(row.published, -7.0);
        assert!(row.remainder >= 16.0 && row.remainder < 16.0 * (1.0 + 1e-14));
    }
}
/// W-16 R-c: a moved spring base adds the exact +k·g to the row that holds
/// -k·u, -k·w and -k·w₂; a base equal to zero, of either sign, is the
/// fixed-base row bit for bit; the evaluation scale includes |k·g|.
#[test]
fn spring_base_motion_adds_the_exact_product_and_zero_is_the_fixed_base_row() {
    let u = [0.25, 0.0, 0.0, 0.0, 0.0, 0.0];
    let mut b = bound(6);
    b.w[0] = 0.125;
    b.w2[0] = 0.0625;
    let fixed = certify_spring_reaction(0, 16.0, 0.0, &u, &b).unwrap();
    assert_eq!(fixed.published, -7.0);
    for zero in [0.0, -0.0] {
        let row = certify_spring_reaction(0, 16.0, zero, &u, &b).unwrap();
        assert_eq!(format!("{row:?}"), format!("{fixed:?}"), "g = {zero:?}");
        assert_eq!(row.published.to_bits(), fixed.published.to_bits());
    }
    // -16·(0.4375 - 0.5) = +1, exactly.
    let moved = certify_spring_reaction(0, 16.0, 0.5, &u, &b).unwrap();
    assert_eq!(moved.published, 1.0);
    assert!(moved.absolute_scale >= 16.0 * 0.5 + 16.0 * 0.4375);
    // An inexact product k·g: the published row is the once-rounded exact
    // -k·(u + w + w₂) + k·g, with no binary64 fold of k·g or u - g first.
    let (k, g) = (3.0, 0.1);
    let row = certify_spring_reaction(0, k, g, &u, &b).unwrap();
    let mut exact = crate::exact_sum::ExactAccumulator::new();
    for x in [u[0], b.w[0], b.w2[0]] {
        exact.add_product(-k, x).unwrap();
    }
    exact.add_product(k, g).unwrap();
    assert_eq!(row.published.to_bits(), exact.round().unwrap().to_bits());
    assert_ne!(row.published, -k * (0.4375 - g), "the folded form differs here");
    assert!(row.radius >= 0.0 && row.radius.is_finite());
}
#[test]
fn scalar_and_magnitude_publication_precede_stress_scaling() {
    let mut a = WideArith::new(128).unwrap();
    let low = Wide2::from_f64(2.0_f64.powi(-53)).unwrap();
    let m = a.add(&Wide2::ONE, &low).unwrap();
    let row = CertifiedRow::finish(Ball::exact(m), 0.0, 1.0).unwrap();
    assert_eq!(row.published, 1.0);
    let scaled = row.scaled(3.0).unwrap();
    assert_eq!(scaled.published, 3.0_f64.next_up());
    let rows = [row, row, row];
    let magnitude = certified_magnitude(&rows).unwrap();
    let rounded_components =
        open_pipe_stress_correct_norm::norm3(row.published, row.published, row.published);
    assert_ne!(magnitude.published, rounded_components);
    let group = certified_group(&rows).unwrap();
    assert_eq!(group.q, rounded_components);
    assert!(group.q_hat >= group.q);
}

#[test]
fn arc_recovery_uses_the_intended_matrix_and_equilibrium_sections() {
    let arc = CurvedFormation {
        node_i: 0,
        node_j: 1,
        coordinates_i: [1.0, 0.0, 0.0],
        coordinates_j: [0.0, 1.0, 0.0],
        radius: 1.0,
        y_reference: [1.0, 1.0, 0.0],
        elastic_modulus: 8.0,
        shear_modulus: 4.0,
        area: 2.0,
        second_moment: 3.0,
        torsion_constant: 6.0,
        in_plane_flexibility_factor: 1.0,
        out_of_plane_flexibility_factor: 1.0,
    };
    let span = FrameElement::new(
        FrameNode::new(0, arc.coordinates_i).unwrap(),
        FrameNode::new(1, arc.coordinates_j).unwrap(),
        FrameSection::new(8.0, 4.0, 2.0, 3.0, 3.0, 6.0).unwrap(),
        [1.0, 1.0, 0.0],
    )
    .unwrap();
    let mut b = bound(12);
    b.w[7] = 0.0625;
    b.w2[7] = 0.03125;
    let mut u = [0.0; 12];
    u[6] = 0.25;
    u[7] = 0.125;
    u[11] = 0.0625;
    let recovered =
        chord_control_recovery(&arc, &span, &CertifiedMemberLoads::default(), &u, &b).unwrap();
    let tangent = certify_arc_recovery(&arc, &CertifiedMemberLoads::default(), &u, &b).unwrap();
    for (end, section, sign) in [(0,0,-1.0),(1,4,1.0)] {
        for r in 0..6 {
            let row=tangent.ends[6*end+r];let cut=tangent.sections[section][r];
            assert!((row.published-sign*cut.published).abs()<=add_up(row.bound(),cut.bound()),"end {end} component {r}");
        }
        for block in 0..2 {
            let range=6*end+3*block..6*end+3*block+3;
            let tq=certified_magnitude(&tangent.ends[range.clone()]).unwrap();
            let cq=certified_magnitude(&recovered.ends[range]).unwrap();
            assert!((tq.published-cq.published).abs()<=add_up(tq.bound(),cq.bound()));
        }
    }
    let mut a = WideArith::new(128).unwrap();
    let k = curved_matrix(&mut a, &arc).unwrap();
    let (axes, _) = chord_axes(
        &mut a,
        arc.coordinates_i,
        arc.coordinates_j,
        span.y_reference,
    )
    .unwrap();
    let mut global = [0.0; 12];
    for r in 0..12 {
        let mut sum = ExactAccumulator::new();
        for c in 0..12 {
            for value in [u[c], b.w[c], b.w2[c]] {
                k[r][c].add_product_to(&mut sum, value).unwrap();
            }
        }
        global[r] = sum.round().unwrap();
    }
    // A separate 512-bit exact-product path evaluates axes*K_int*(u+w+w2)
    // without the certificate's coefficient balls or fixed-grid projection.
    use super::super::retained::wide::multi::WideContext;
    use super::super::retained::wide_sum::ExactWideSum;
    let mut reference = WideContext::<8>::new(512).unwrap();
    for block in 0..4 {
        for axis in 0..3 {
            let mut sum = ExactWideSum::new();
            for rotation_column in 0..3 {
                let av = axes[axis][rotation_column].widen::<8>();
                for col in 0..12 {
                    let kv = k[3 * block + rotation_column][col].widen::<8>();
                    let coefficient = reference.mul(&av, &kv).unwrap();
                    for value in [u[col], b.w[col], b.w2[col]] {
                        let displacement = Wide2::from_f64(value).unwrap().widen::<8>();
                        let product = reference.mul(&coefficient, &displacement).unwrap();
                        sum.add_wide(&product, false).unwrap();
                    }
                }
            }
            let truth = sum.round(&mut reference).unwrap();
            let row = recovered.ends[3 * block + axis];
            let published = Wide2::from_f64(row.published).unwrap().widen::<8>();
            let difference = reference.sub(&published, &truth).unwrap();
            let error = difference.to_binary64().value().unwrap().abs();
            let error_up = if error == 0.0 { 0.0 } else { error.next_up() };
            assert!(
                error_up <= row.bound(),
                "row {}: error={error_up} bound={}",
                3 * block + axis,
                row.bound()
            );
        }
    }
    // At end j tangent=(-1,0,0), inward=(0,-1,0), normal=(0,0,1).
    for (r, truth) in [
        -global[6],
        -global[7],
        global[8],
        -global[9],
        -global[10],
        global[11],
    ]
    .into_iter()
    .enumerate()
    {
        let row = recovered.sections[4][r];
        assert!((row.published - truth).abs() <= row.bound() + 8.0 * f64::EPSILON * truth.abs());
    }
    // All section forces/moments transfer the same j-end actions; a force
    // component's magnitude is invariant under the section-frame rotation.
    let tipnorm = open_pipe_stress_correct_norm::norm3(global[6], global[7], global[8]);
    for station in &recovered.sections {
        let q = certified_group(&station[..3]).unwrap();
        assert!((q.q - tipnorm).abs() <= q.bound + 8.0 * f64::EPSILON * tipnorm);
    }
}

#[test]
fn u4a_actual_end_axis_balls_enclose_all_inexact_frozen_axes() {
    // The frozen45digit decimals are split independently into four binary64
    // pieces. Their residual is below1e-60; sum_ball includes its own p128
    // projection radius. Checking the actual end operator's axes kills M7.
    const PINS:&str=include_str!("u4a_axis_pins.txt");
    let lines=PINS.lines().filter(|line| !line.starts_with('#')).collect::<Vec<_>>();
    let hex=|word:&str|f64::from_bits(u64::from_str_radix(word,16).unwrap());
    let mut count=0;
    for chunk in lines.chunks_exact(20) {
        assert!(chunk[0].starts_with("case "));
        let args=chunk[1].strip_prefix("args ").unwrap().split_whitespace().map(hex).collect::<Vec<_>>();
        let arc=CurvedFormation {node_i:0,node_j:1,coordinates_i:args[..3].try_into().unwrap(),coordinates_j:args[3..6].try_into().unwrap(),radius:args[6],y_reference:args[7..10].try_into().unwrap(),elastic_modulus:8.0,shear_modulus:4.0,area:2.0,second_moment:3.0,torsion_constant:6.0,in_plane_flexibility_factor:1.0,out_of_plane_flexibility_factor:1.0};
        let mut b=Balls::new().unwrap();let axes=arc_end_axes(&mut b,&arc).unwrap();let mut separated=false;
        // Independent512bit circle construction from exact authored binary64
        // operands, with no Ball or arc_end_axes coefficient in the reference.
        let direct=u4a_direct_circle_axes_512(&args);
        for (i,line) in chunk[2..].iter().enumerate() {
            let mut exact=ExactAccumulator::new();
            let pieces=line.strip_prefix("axis ").unwrap().split_whitespace().map(hex).collect::<Vec<_>>();
            for &piece in &pieces[..4] {exact.add(piece).unwrap();}
            let expected=sum_ball(&exact,pieces[4]).unwrap();let actual=axes[i/9][(i%9)/3][i%3];
            let difference=b.sub(actual,expected).unwrap();
            let distance=abs_up(&difference.m).unwrap();
            assert!(add_up(distance,expected.r)<=actual.r,"{} axis{i}: distance{distance:e}, reference radius{:e}, actual radius{:e}",chunk[0],expected.r,actual.r);
            // A radius-zero binary64 coefficient cannot enclose the same
            //45digit axis, even though product values cannot discriminate it.
            let rounded=Wide2::from_f64(midpoint_publication(expected.m).unwrap()).unwrap();
            let mismatch=b.sub(Ball::exact(rounded),expected).unwrap();
            separated|=abs_up(&mismatch.m).unwrap()>expected.r;
            use super::super::retained::wide::multi::WideContext;
            let mut c=WideContext::<8>::new(512).unwrap();
            let delta=c.sub(&actual.m.widen::<8>(),&direct[i/9][(i%9)/3][i%3]).unwrap();
            let error=delta.to_binary64().value().unwrap().abs();
            let upper=if error==0.0 {0.0} else {add_up(error.next_up(),1e-120)};
            assert!(upper<=actual.r,"{} direct512 axis{i}: error{upper:e} radius{:e}",chunk[0],actual.r);
        }
        assert!(separated,"each marked arc discriminates radius-zero binary64 axes");count+=1;
    }
    assert_eq!(count,9);
}

fn u4a_direct_circle_axes_512(args:&[f64]) -> [[[super::super::retained::wide::Wide<8>;3];3];2] {
    use super::super::retained::wide::{Wide,multi::WideContext};
    type W=Wide<8>;
    fn dot(c:&mut WideContext<8>,a:[W;3],b:[W;3])->W {let mut s=W::ZERO;for k in 0..3 {let p=c.mul(&a[k],&b[k]).unwrap();s=c.add(&s,&p).unwrap();}s}
    fn norm(c:&mut WideContext<8>,a:[W;3])->[W;3] {let sq=dot(c,a,a);let l=c.sqrt(&sq).unwrap();a.map(|x|c.div(&x,&l).unwrap())}
    fn cross(c:&mut WideContext<8>,a:[W;3],b:[W;3])->[W;3] {std::array::from_fn(|k| {let i=(k+1)%3;let j=(k+2)%3;let p=c.mul(&a[i],&b[j]).unwrap();let q=c.mul(&a[j],&b[i]).unwrap();c.sub(&p,&q).unwrap()})}
    let mut c=WideContext::<8>::new(512).unwrap();
    let a=args.iter().map(|&x|Wide2::from_f64(x).unwrap().widen::<8>()).collect::<Vec<_>>();
    let xi:[W;3]=a[..3].try_into().unwrap();let xj:[W;3]=a[3..6].try_into().unwrap();let yr:[W;3]=a[7..10].try_into().unwrap();
    let d=std::array::from_fn(|k|c.sub(&xj[k],&xi[k]).unwrap());let l2=dot(&mut c,d,d);let ex=norm(&mut c,d);let proj=dot(&mut c,yr,ex);
    let bow=std::array::from_fn(|k| {let p=c.mul(&proj,&ex[k]).unwrap();c.sub(&yr[k],&p).unwrap()});let bow=norm(&mut c,bow);
    let r2=c.mul(&a[6],&a[6]).unwrap();let quarter=l2.mul_pow2(-2).unwrap();let inside=c.sub(&r2,&quarter).unwrap();let offset=c.sqrt(&inside).unwrap();
    let centre:[W;3]=std::array::from_fn(|k| {let s=c.add(&xi[k],&xj[k]).unwrap().mul_pow2(-1).unwrap();let p=c.mul(&offset,&bow[k]).unwrap();c.sub(&s,&p).unwrap()});
    let radial:[ [W;3];2]=[xi,xj].map(|end|std::array::from_fn(|k| {let v=c.sub(&end[k],&centre[k]).unwrap();c.div(&v,&a[6]).unwrap()}));
    let normal=cross(&mut c,radial[0],radial[1]);let normal=norm(&mut c,normal);
    radial.map(|r|[cross(&mut c,normal,r),r.map(|x|x.neg()),normal])
}
#[test]
fn pressure_wall_has_its_own_single_rounding_and_bound() {
    let action =
        CertifiedRow::finish(Ball::exact(Wide2::from_f64(-1.0).unwrap()), 0.25, 4.0).unwrap();
    let fluid = certified_pressure_area_product(4.0, 2.0, 0.5).unwrap();
    let wall = action.add_value(fluid).unwrap();
    assert_eq!(wall.remainder, action.remainder);
    let target = std::f64::consts::PI - 1.0;
    assert!((wall.published - target).abs() <= wall.bound());
    let membrane = wall.divided(2.0).unwrap();
    assert!(membrane.remainder >= 0.125);
    let effective = certified_group(&[action]).unwrap();
    let wallgroup = certified_group(&[wall]).unwrap();
    assert_ne!(effective.q, wallgroup.q);
}
#[test]
fn recovery_source_has_no_heap_matrix_or_uniform_cache() {
    let source = include_str!("certified_recovery.rs");
    assert!(!source.contains("Vec<FrameElement>"));
    assert!(!source.contains("Vec<CertifiedUniformLoad>"));
    assert!(!source.contains("Vec<[[Wide2"));
    assert!(source.contains("for value in [u[i], bound.w[i], bound.w2[i]]"));
}

#[test]
fn parallel_members_use_source_qualified_borrowed_loads() {
    let frame = frame();
    let mut primitives = Vec::new();
    for (source, w) in [("member:1:A:load", -6.0), ("member:1:B:load", -12.0)] {
        for row in 0..12 {
            primitives.push(LoadPrimitive {
                term_index: primitives.len(),
                identity: LoadSummandIdentity {
                    family: "uniform_straight".into(),
                    source: source.into(),
                    ordinal: row,
                },
                local_row: row,
                formula: LoadFormula::UniformStraight {
                    frame: frame.clone(),
                    intensity: IntensityPrimitive::Authored([0.0, w, 0.0]),
                    start_fraction: 0.0,
                    end_fraction: 1.0,
                },
                force_scale_exponent: 0,
            });
        }
    }
    // T4-U8: the parallel members each have their own formation-source frame.
    let source = FormationSource {
        node_count: 2,
        frames: vec![frame, frame],
        frame_members: vec!["member:1:A:".into(), "member:1:B:".into()],
        ..FormationSource::default()
    };
    let loads = CertifiedMemberLoads::from_member_primitives(0, 1, &primitives, "member:1:A:")
        .unwrap()
        .with_member_source(&source, "member:1:A:");
    assert_eq!(loads.uniform_loads().count(), 1);
    let recovered = certify_straight_recovery(&frame, &loads, &[0.0; 12], &bound(12)).unwrap();
    assert_eq!(recovered.ends[1].published, 6.0);
}
#[test]
fn support_remainder_streams_triangle_norms_and_omits_prescribed_columns() {
    let source = FormationSource {
        node_count: 2,
        frames: vec![frame(), frame()],
        ..Default::default()
    };
    let force = crate::load_ledger::LoadLedger::new().finish(12).unwrap();
    let mut b = bound(12);
    b.exponents[..6].fill(i32::MIN);
    b.exponents[6] = 1;
    b.lambda_sigma = 1.0;
    let row = certify_support_reaction(&source, &force, 0, &[0.0; 12], &b).unwrap();
    // Each member's free axial column is |-EA/L|*2=16; the stack streaming
    // reaction bound sums the two contribution norms upward, giving32.
    assert!(row.remainder >= 32.0 && row.remainder < 32.0 * (1.0 + 1e-14));
}

#[test]
fn magnitudes_scale_before_ball_products_at_extreme_exponents() {
    for component in [1e305, 1e-305, f64::from_bits(4)] {
        let row = CertifiedRow::finish(
            Ball::exact(Wide2::from_f64(component).unwrap()),
            0.0,
            component,
        )
        .unwrap();
        let magnitude = certified_magnitude(&[row, row, row]).unwrap();
        let expected = open_pipe_stress_correct_norm::norm3(component, component, component);
        assert_eq!(magnitude.published, expected, "component {component}");
        assert!(magnitude.radius.is_finite());
        assert!(magnitude.bound().is_finite());
    }
}

#[test]
fn finite_stress_survives_unrepresentable_notice_scale() {
    let midpoint = 0.35 * f64::MAX;
    let row = CertifiedRow::finish(
        Ball::exact(Wide2::from_f64(midpoint).unwrap()),
        0.0,
        0.75 * f64::MAX,
    )
    .unwrap();
    let stress = row.divided(0.5).unwrap();
    assert_eq!(stress.published, 0.7 * f64::MAX);
    assert!(
        stress.radius.is_finite() && stress.remainder.is_finite() && stress.bound().is_finite()
    );
    assert_eq!(stress.absolute_scale, f64::INFINITY);
    let group = certified_group(&[stress]).unwrap();
    assert!(group.q.is_finite() && group.q_hat.is_finite() && group.bound.is_finite());
    assert!(!group.absolute_scale.is_finite());
    assert!(CertifiedRow::finish(Ball::ZERO, 0.0, f64::NAN).is_err());
    assert!(CertifiedRow::finish(Ball::ZERO, 0.0, -1.0).is_err());
}

#[test]
fn final_scaled_sum_publishes_without_an_overflowing_intermediate() {
    let value = 0.7 * f64::MAX;
    let row =
        CertifiedRow::finish(Ball::exact(Wide2::from_f64(value).unwrap()), 1.0, 1.5e308).unwrap();
    assert!(row.add_value(row.value()).is_err());
    let summary = row.sum_scaled(row, 1e-6).unwrap();
    assert!(summary.published.is_finite() && summary.bound().is_finite());
    assert!(summary.remainder >= 2e-6 && summary.remainder < 2e-6 * (1.0 + 1e-14));
    let mut reference = ExactAccumulator::new();
    reference.add_product(value, 1e-6).unwrap();
    reference.add_product(value, 1e-6).unwrap();
    assert_eq!(summary.published, reference.round().unwrap());
}

#[test]
fn published_magnitude_has_a_separate_error_against_independent_sqrt2() {
    use super::super::retained::wide::multi::WideContext;
    let one = CertifiedRow::finish(Ball::exact(Wide2::ONE), 0.0, 1.0).unwrap();
    let rows = [one, one, CertifiedRow::ZERO];
    let vector = certified_group(&rows).unwrap();
    assert_eq!(vector.bound, 0.0);
    let magnitude = certified_magnitude(&rows).unwrap();
    let scalar = CertifiedGroup::magnitude(&rows).unwrap();
    let mut reference = WideContext::<8>::new(512).unwrap();
    let exact_root = reference
        .sqrt(&super::super::retained::wide::Wide::<8>::from_f64(2.0).unwrap())
        .unwrap();
    let published = Wide2::from_f64(magnitude.published).unwrap().widen::<8>();
    let difference = reference.sub(&published, &exact_root).unwrap();
    let error = difference.to_binary64().value().unwrap().abs();
    assert!(error > vector.bound);
    assert!(scalar.bound >= error.next_up());
    assert_eq!(scalar.q, magnitude.published);
    assert!(scalar.comparison_available());
}
#[test]
fn finite_components_with_out_of_range_group_norm_are_unavailable_statistics() {
    let row = |fraction| {
        CertifiedRow::finish(
            Ball::exact(Wide2::from_f64(fraction * f64::MAX).unwrap()),
            0.0,
            1.0,
        )
        .unwrap()
    };
    let inside = [row(0.7), row(0.7)];
    let group = certified_group(&inside).unwrap();
    assert!(group.comparison_available());
    let outside = [row(0.8), row(0.8)];
    let group = certified_group(&outside).unwrap();
    assert!(outside.iter().all(|row| row.published.is_finite()));
    assert!(!group.comparison_available());
    assert!(group.q.is_infinite() && group.q_hat.is_infinite());
    assert_eq!(group.bound, 0.0); // The unavailable norm is independent of row error.
}

mod independent_loaded_arc {
    use super::super::super::retained::wide::{multi::WideContext, Wide};
    use super::super::super::retained::wide_sum::ExactWideSum;
    use super::*;
    type Ref = Wide<8>;
    fn lift(x: f64) -> Ref {
        Ref::from_f64(x).unwrap()
    }
    fn dot(c: &mut WideContext<8>, x: &[Ref; 3], y: &[Ref; 3]) -> Ref {
        let mut s = ExactWideSum::new();
        for k in 0..3 {
            let p = c.mul(&x[k], &y[k]).unwrap();
            s.add_wide(&p, false).unwrap();
        }
        s.round(c).unwrap()
    }
    fn cross(c: &mut WideContext<8>, x: &[Ref; 3], y: &[Ref; 3]) -> [Ref; 3] {
        std::array::from_fn(|k| {
            let i = (k + 1) % 3;
            let j = (k + 2) % 3;
            let a = c.mul(&x[i], &y[j]).unwrap();
            let b = c.mul(&x[j], &y[i]).unwrap();
            c.sub(&a, &b).unwrap()
        })
    }
    fn normalized(c: &mut WideContext<8>, v: [Ref; 3]) -> [Ref; 3] {
        let sq = dot(c, &v, &v);
        let n = c.sqrt(&sq).unwrap();
        v.map(|x| c.div(&x, &n).unwrap())
    }
    // Independent 512-bit Taylor references. Four half-angle reductions put
    // atan's argument below 1/16; 160 terms leave truncation below 2^-1280.
    fn atan(c: &mut WideContext<8>, mut x: Ref) -> Ref {
        for _ in 0..4 {
            let xx = c.mul(&x, &x).unwrap();
            let one = c.add(&Ref::ONE, &xx).unwrap();
            let r = c.sqrt(&one).unwrap();
            let d = c.add(&Ref::ONE, &r).unwrap();
            x = c.div(&x, &d).unwrap();
        }
        let xx = c.mul(&x, &x).unwrap().neg();
        let mut term = x;
        let mut sum = ExactWideSum::new();
        sum.add_wide(&term, false).unwrap();
        for n in 1..160 {
            term = c.mul(&term, &xx).unwrap();
            let divisor = lift((2 * n + 1) as f64);
            let t = c.div(&term, &divisor).unwrap();
            sum.add_wide(&t, false).unwrap();
        }
        let value = sum.round(c).unwrap();
        value.mul_pow2(4).unwrap()
    }
    fn trig(c: &mut WideContext<8>, x: Ref) -> (Ref, Ref) {
        let xx = c.mul(&x, &x).unwrap().neg();
        let mut s = x;
        let mut co = Ref::ONE;
        let mut st = x;
        let mut ct = Ref::ONE;
        for n in 1..160 {
            let v = c.mul(&st, &xx).unwrap();
            st = c.div(&v, &lift((2 * n * (2 * n + 1)) as f64)).unwrap();
            s = c.add(&s, &st).unwrap();
            let v = c.mul(&ct, &xx).unwrap();
            ct = c.div(&v, &lift(((2 * n - 1) * 2 * n) as f64)).unwrap();
            co = c.add(&co, &ct).unwrap();
        }
        (s, co)
    }
    fn error_up(c: &mut WideContext<8>, published: f64, reference: Ref) -> f64 {
        let d = c.sub(&lift(published), &reference).unwrap();
        let e = d.to_binary64().value().unwrap().abs();
        if e == 0.0 {
            0.0
        } else {
            e.next_up()
        }
    }
    #[test]
    fn loaded_arc_ends_and_all_stations_enclose_independent_reference() {
        const TEXT: &str =
            include_str!("../../../../../validation/references/t4_i9/arc_load_references.txt");
        let lines: Vec<_> = TEXT.lines().collect();
        let at = lines
            .iter()
            .position(|l| *l == "case angle 60 deg")
            .unwrap();
        let hex = |t: &str| f64::from_bits(u64::from_str_radix(t, 16).unwrap());
        let args: Vec<f64> = lines[at + 2]
            .strip_prefix("args ")
            .unwrap()
            .split_whitespace()
            .map(hex)
            .collect();
        let arc = CurvedFormation {
            node_i: 0,
            node_j: 1,
            coordinates_i: [args[0], args[1], args[2]],
            coordinates_j: [args[3], args[4], args[5]],
            radius: args[6],
            y_reference: [args[7], args[8], args[9]],
            elastic_modulus: args[10],
            shear_modulus: args[11],
            area: args[12],
            second_moment: args[13],
            torsion_constant: args[14],
            in_plane_flexibility_factor: args[15],
            out_of_plane_flexibility_factor: args[16],
        };
        let intensity = [args[17], args[18], args[19]];
        let span = FrameElement::new(
            FrameNode::new(0, arc.coordinates_i).unwrap(),
            FrameNode::new(1, arc.coordinates_j).unwrap(),
            FrameSection::new(
                arc.elastic_modulus,
                arc.shear_modulus,
                arc.area,
                arc.second_moment,
                arc.second_moment,
                arc.torsion_constant,
            )
            .unwrap(),
            arc.y_reference,
        )
        .unwrap();
        let primitives: Vec<_> = (0..12)
            .map(|row| LoadPrimitive {
                term_index: row,
                identity: LoadSummandIdentity {
                    family: "uniform_curved".into(),
                    source: "independent-loaded-arc".into(),
                    ordinal: row,
                },
                local_row: row,
                formula: LoadFormula::UniformCurved {
                    arc,
                    intensity: IntensityPrimitive::Authored(intensity),
                },
                force_scale_exponent: 0,
            })
            .collect();
        let loads = CertifiedMemberLoads::from_primitives(0, 1, &primitives).unwrap();
        let mut u = [0.0; 12];
        u[6] = 2.0_f64.powi(-12);
        u[8] = 2.0_f64.powi(-15);
        u[10] = -2.0_f64.powi(-17);
        let mut bound = bound(12);
        bound.w[8] = 2.0_f64.powi(-37);
        bound.w2[8] = 2.0_f64.powi(-62);
        let result = chord_control_recovery(&arc, &span, &loads, &u, &bound).unwrap();
        let mut wide = WideArith::new(128).unwrap();
        let matrix = curved_matrix(&mut wide, &arc).unwrap();
        let (axes, _) = chord_axes(
            &mut wide,
            arc.coordinates_i,
            arc.coordinates_j,
            arc.y_reference,
        )
        .unwrap();
        let mut c = WideContext::<8>::new(512).unwrap();
        let mut global = [Ref::ZERO; 12];
        let mut reference_error = 0.0_f64;
        for r in 0..12 {
            let mut sum = ExactWideSum::new();
            for k in 0..12 {
                for value in [u[k], bound.w[k], bound.w2[k]] {
                    let p = c.mul(&matrix[r][k].widen::<8>(), &lift(value)).unwrap();
                    sum.add_wide(&p, false).unwrap();
                }
            }
            let fields: Vec<_> = lines[at + 3 + r].split_whitespace().collect();
            for word in &fields[2..6] {
                sum.add_binary64(hex(word), true).unwrap();
            }
            reference_error = reference_error.max(hex(fields[6]));
            global[r] = sum.round(&mut c).unwrap();
        }
        for block in 0..4 {
            let g = [
                global[3 * block],
                global[3 * block + 1],
                global[3 * block + 2],
            ];
            for axis in 0..3 {
                let ax = axes[axis].map(|v| v.widen::<8>());
                let reference = dot(&mut c, &ax, &g);
                let row = result.ends[3 * block + axis];
                assert!(
                    error_up(&mut c, row.published, reference)
                        <= add_up(row.bound(), reference_error * 4.0),
                    "end block{block} axis{axis}"
                );
            }
        }
        // Geometry and section equilibrium are computed independently at512:
        // no arc Ball geometry, p128 trig, row operator or recovery helper.
        let d = std::array::from_fn(|k| {
            c.sub(&lift(arc.coordinates_j[k]), &lift(arc.coordinates_i[k]))
                .unwrap()
        });
        let d2 = dot(&mut c, &d, &d);
        let length = c.sqrt(&d2).unwrap();
        let radius = lift(arc.radius);
        let diameter = radius.mul_pow2(1).unwrap();
        let sh = c.div(&length, &diameter).unwrap();
        let ss = c.mul(&sh, &sh).unwrap();
        let one_minus = c.sub(&Ref::ONE, &ss).unwrap();
        let ch = c.sqrt(&one_minus).unwrap();
        let t = c.div(&sh, &ch).unwrap();
        let phi = atan(&mut c, t).mul_pow2(1).unwrap();
        let (se, ce) = trig(&mut c, phi);
        let dh = normalized(&mut c, d);
        let yr = arc.y_reference.map(lift);
        let projection = dot(&mut c, &yr, &dh);
        let nr = std::array::from_fn(|k| {
            let along = c.mul(&projection, &dh[k]).unwrap();
            c.sub(&yr[k], &along).unwrap()
        });
        let nh = normalized(&mut c, nr);
        let ex = std::array::from_fn(|k| {
            let a = c.mul(&ch, &nh[k]).unwrap();
            let b = c.mul(&sh, &dh[k]).unwrap();
            c.sub(&a, &b).unwrap()
        });
        let ey = std::array::from_fn(|k| {
            let a = c.mul(&ch, &dh[k]).unwrap();
            let b = c.mul(&sh, &nh[k]).unwrap();
            c.add(&a, &b).unwrap()
        });
        let ez = cross(&mut c, &nh, &dh);
        let center = [ex, ey, ez];
        let fg = [global[6], global[7], global[8]];
        let mg = [global[9], global[10], global[11]];
        let tipf = center.map(|axis| dot(&mut c, &axis, &fg));
        let tipm = center.map(|axis| dot(&mut c, &axis, &mg));
        let wg = intensity.map(lift);
        let w = center.map(|axis| dot(&mut c, &axis, &wg));
        for (location, &fraction) in CERTIFIED_SECTION_FRACTIONS.iter().enumerate() {
            let theta = c.mul(&phi, &lift(fraction)).unwrap();
            let remaining = c.sub(&phi, &theta).unwrap();
            let (sin, cos) = trig(&mut c, theta);
            let dc = c.sub(&ce, &cos).unwrap();
            let ds = c.sub(&se, &sin).unwrap();
            let arm = [
                c.mul(&radius, &dc).unwrap(),
                c.mul(&radius, &ds).unwrap(),
                Ref::ZERO,
            ];
            let r2 = c.mul(&radius, &radius).unwrap();
            let rc = c.mul(&remaining, &cos).unwrap();
            let rs = c.mul(&remaining, &sin).unwrap();
            let dx = c.sub(&ds, &rc).unwrap();
            let dy = c.sub(&dc.neg(), &rs).unwrap();
            let distarm = [
                c.mul(&r2, &dx).unwrap(),
                c.mul(&r2, &dy).unwrap(),
                Ref::ZERO,
            ];
            let transfer = cross(&mut c, &arm, &tipf);
            let distributed = cross(&mut c, &distarm, &w);
            let loaded_length = c.mul(&radius, &remaining).unwrap();
            let force = std::array::from_fn(|k| {
                let term = c.mul(&loaded_length, &w[k]).unwrap();
                c.add(&tipf[k], &term).unwrap()
            });
            let moment = std::array::from_fn(|k| {
                let a = c.add(&tipm[k], &transfer[k]).unwrap();
                c.add(&a, &distributed[k]).unwrap()
            });
            let section_axes = [
                [sin.neg(), cos, Ref::ZERO],
                [cos.neg(), sin.neg(), Ref::ZERO],
                [Ref::ZERO, Ref::ZERO, Ref::ONE],
            ];
            for block in 0..2 {
                let vector = if block == 0 { force } else { moment };
                for axis in 0..3 {
                    let reference = dot(&mut c, &section_axes[axis], &vector);
                    let row = result.sections[location][3 * block + axis];
                    assert!(
                        error_up(&mut c, row.published, reference)
                            <= add_up(row.bound(), reference_error * 16.0),
                        "station{fraction} block{block} axis{axis},error={} B={}",
                        error_up(&mut c, row.published, reference),
                        row.bound()
                    );
                }
            }
        }
    }
}

fn uniform_primitives(
    frame: &FrameElement,
    intensity: [f64; 3],
    start_fraction: f64,
    end_fraction: f64,
) -> Vec<LoadPrimitive> {
    (0..12)
        .map(|row| LoadPrimitive {
            term_index: row,
            identity: LoadSummandIdentity {
                family: "uniform_straight".into(),
                source: "source-control".into(),
                ordinal: row,
            },
            local_row: row,
            formula: LoadFormula::UniformStraight {
                frame: frame.clone(),
                intensity: IntensityPrimitive::Authored(intensity),
                start_fraction,
                end_fraction,
            },
            force_scale_exponent: 0,
        })
        .collect()
}
#[test]
fn source_bernstein_controls_match_exact_uniform_statics_and_fixed_sections() {
    let frame = frame();
    let primitives = uniform_primitives(&frame, [0.0, -6.0, 0.0], 0.0, 1.0);
    let source = member_source(&frame);
    let loads = CertifiedMemberLoads::from_primitives(0, 1, &primitives).unwrap().with_member_source(&source, MEMBER);
    let u = [0.0; 12];
    let bound = bound(12);
    let spans = certify_straight_stress_spans(&frame, &loads, &u, &bound, 2.0, 4.0, None).unwrap();
    assert_eq!(spans.len(), 1);
    for (coefficient, truth) in spans[0].bending_z.into_iter().zip([-0.5, 1.0, -0.5]) {
        assert!(coefficient.lower <= truth && coefficient.upper >= truth);
    }
    let member = certify_straight_recovery(&frame, &loads, &u, &bound).unwrap();
    for (s, &fraction) in CERTIFIED_SECTION_FRACTIONS.iter().enumerate() {
        let section = certify_straight_section(&frame, &loads, &u, &bound, fraction).unwrap();
        for r in 0..6 {
            assert_eq!(section[r].published, member.sections[s][r].published);
            assert_eq!(section[r].radius, member.sections[s][r].radius);
        }
    }
    assert!(spans[0].publication_radius_pa.is_finite() && spans[0].publication_radius_pa > 0.0);
}
#[test]
fn partial_span_controls_keep_the_three_exact_polynomials_and_both_boundary_sides() {
    let frame = frame();
    let primitives = uniform_primitives(&frame, [0.0, -6.0, 0.0], 0.25, 0.75);
    let source = member_source(&frame);
    let loads = CertifiedMemberLoads::from_primitives(0, 1, &primitives).unwrap().with_member_source(&source, MEMBER);
    let spans =
        certify_straight_stress_spans(&frame, &loads, &[0.0; 12], &bound(12), 2.0, 1.0, None)
            .unwrap();
    assert_eq!(
        spans.iter().map(|s| (s.start, s.end)).collect::<Vec<_>>(),
        vec![(0.0, 0.25), (0.25, 0.75), (0.75, 1.0)]
    );
    let truths = [
        [-1.375, -0.625, 0.125],
        [0.125, 1.625, 0.125],
        [0.125, -0.625, -1.375],
    ];
    for (span, truths) in spans.iter().zip(truths) {
        for (coefficient, truth) in span.bending_z.iter().zip(truths) {
            assert!(
                coefficient.lower <= truth && coefficient.upper >= truth,
                "{span:?}, truth{truth}"
            );
        }
    }
    for adjacent in spans.windows(2) {
        let left = adjacent[0].bending_z[2];
        let right = adjacent[1].bending_z[0];
        assert!(left.lower <= right.upper && right.lower <= left.upper);
    }
}
#[test]
fn source_controls_use_the_corrections_and_exclude_the_solution_remainder() {
    let frame = frame();
    let source = member_source(&frame);
    let loads = CertifiedMemberLoads::default().with_member_source(&source, MEMBER);
    let mut u = [0.0; 12];
    u[6] = 1e16;
    let mut first = bound(12);
    first.w[6] = -1e16;
    first.w2[6] = 1.0;
    let a = certify_straight_stress_spans(&frame, &loads, &u, &first, 2.0, 4.0, None).unwrap();
    for coefficient in a[0].axial {
        assert!(coefficient.lower <= 4.0 && coefficient.upper >= 4.0);
    }
    let mut second = first.clone();
    second.lambda_sigma = 1e100;
    second.exponents.fill(300);
    let b = certify_straight_stress_spans(&frame, &loads, &u, &second, 2.0, 4.0, None).unwrap();
    assert_eq!(a[0].axial, b[0].axial);
    assert_eq!(a[0].bending_y, b[0].bending_y);
    assert_eq!(a[0].publication_radius_pa, b[0].publication_radius_pa);
}
#[test]
fn source_controls_preserve_zero_and_hold_the_same_pressure_wall_increment() {
    let frame = frame();
    let source = member_source(&frame);
    let loads = CertifiedMemberLoads::default().with_member_source(&source, MEMBER);
    let u = [0.0; 12];
    let bound = bound(12);
    let zero = certify_straight_stress_spans(&frame, &loads, &u, &bound, 2.0, 4.0, None).unwrap();
    assert_eq!(zero[0].publication_radius_pa, 0.0);
    assert!(zero[0]
        .axial
        .iter()
        .chain(&zero[0].bending_y)
        .chain(&zero[0].bending_z)
        .all(|b| b.lower == 0.0 && b.upper == 0.0));
    let fluid = certified_pressure_area_product(4.0, 2.0, 0.5).unwrap();
    let increment = fluid.product(CertifiedValue::exact(0.5).unwrap()).unwrap();
    let pressure =
        certify_straight_stress_spans(&frame, &loads, &u, &bound, 2.0, 4.0, Some(increment))
            .unwrap();
    let wall = CertifiedRow::ZERO
        .add_value(increment)
        .unwrap()
        .divided(2.0)
        .unwrap();
    for coefficient in pressure[0].axial {
        assert!(coefficient.lower <= wall.published && coefficient.upper >= wall.published);
    }
}
#[test]
fn exact_catalog_division_differs_from_a_binary64_reciprocal_and_is_enclosed() {
    let mut witness = None;
    for n in 1..1000 {
        let value = 1.0 + (n as f64) / 1024.0;
        let row =
            CertifiedRow::finish(Ball::exact(Wide2::from_f64(value).unwrap()), 0.0, value).unwrap();
        let divided = row.divided(1_000_000.0).unwrap().published;
        let multiplied = row.scaled(1e-6).unwrap().published;
        if divided != multiplied {
            witness = Some((value, divided, multiplied));
            break;
        }
    }
    let (value, divided, multiplied) = witness.expect("rational unit witness");
    assert_ne!(divided, multiplied);
    // Check the exact rational rounding cell using fixed-grid products:
    // divided*1e6 brackets value to within the derived transport radius.
    let transport = mpa_publication_transport(value).unwrap();
    let mut residual = ExactAccumulator::new();
    residual.add_product(divided, 1_000_000.0).unwrap();
    residual.add(-value).unwrap();
    assert!(residual.round().unwrap().abs() <= transport);
}

#[test]
fn source_control_resource_atoms_are_in_build_type_sizes() {
    let sizes = certified_stress_control_sizes();
    assert_eq!(
        sizes.span,
        std::mem::size_of::<CertifiedStraightStressSpan>()
    );
    assert_eq!(
        sizes.context,
        std::mem::size_of::<StraightRecoveryContext>()
    );
    eprintln!("source_control_resource_atoms={sizes:?}");
}

// ---------------------------------------------------------------- nonlinear K-D5
// DEL-04-01 kd5-nonlinear-active-set.md §1.3, §1.6, §2.1, §2.2.

fn friction_primitive(formula: LoadFormula) -> LoadPrimitive {
    LoadPrimitive {
        term_index: 0,
        identity: LoadSummandIdentity {
            family: "nonlinear_sliding_friction".into(),
            source: "NL".into(),
            ordinal: 0,
        },
        local_row: 0,
        formula,
        force_scale_exponent: 0,
    }
}

/// The explicit force is the exact product -dir*mu*|N| at p, not fl(mu*N):
/// 0.3*70 is inexact in binary64, so the ball's midpoint differs from the
/// binary64 product by its two-product error and the published row's bound
/// is exactly that rounding gap (radius 0).
#[test]
fn explicit_sliding_friction_is_the_exact_product_of_its_inputs() {
    let (mu, normal) = (0.3_f64, -70.0_f64);
    let hi = mu * normal.abs();
    let lo = mu.mul_add(normal.abs(), -hi);
    assert_ne!(lo, 0.0, "the product is inexact in binary64");
    for direction in [1.0, -1.0] {
        let p = friction_primitive(LoadFormula::SlidingFrictionExplicit { global_dof: 6, coefficient: mu, normal, direction });
        let (m, r) = load_reformation(&p).unwrap();
        assert_eq!(r[0], 0.0);
        // The midpoint equals -dir*(hi + lo) exactly (one exact sum).
        let split = m[0].split_binary64().unwrap();
        let mut difference = ExactAccumulator::new();
        for &part in split.terms() {
            difference.add(part).unwrap();
        }
        difference.add(direction * hi).unwrap();
        difference.add(direction * lo).unwrap();
        assert!(difference.is_zero());
        let row = certify_load_primitive(&p).unwrap();
        assert_eq!(row.published, -direction * hi);
        assert_eq!(row.radius, 0.0);
        assert_eq!(row.bound(), row.rounding_gap);
        assert!(row.bound() > 0.0);
    }
    for (mu, normal, direction) in [(0.0, 70.0, 1.0), (0.3, f64::NAN, 1.0), (0.3, 70.0, 0.5)] {
        let p = friction_primitive(LoadFormula::SlidingFrictionExplicit { global_dof: 6, coefficient: mu, normal, direction });
        assert!(load_reformation(&p).is_err());
    }
}

/// The coupled force re-forms as the Stage-A ball itself: its radius is a load
/// enclosure radius (carried into psi by the residual and into B here).
#[test]
fn coupled_sliding_friction_re_forms_as_its_stage_a_ball() {
    let p = friction_primitive(LoadFormula::SlidingFrictionCoupled {
        global_dof: 6,
        coefficient: 0.5,
        direction: 1.0,
        beta: -1.0,
        source_dof: 7,
        midpoint: -0.25,
        radius: 2.0_f64.powi(-60),
    });
    let (m, r) = load_reformation(&p).unwrap();
    assert_eq!(m[0].split_binary64().unwrap().terms(), &[-0.25]);
    assert_eq!(r[0], 2.0_f64.powi(-60));
    let row = certify_load_primitive(&p).unwrap();
    assert_eq!(row.published, -0.25);
    assert_eq!(row.bound(), 2.0_f64.powi(-60));
    for (beta, radius) in [(0.0, 0.0), (1.0, -1.0), (1.0, f64::INFINITY)] {
        let bad = friction_primitive(LoadFormula::SlidingFrictionCoupled { global_dof: 6, coefficient: 0.5, direction: 1.0, beta, source_dof: 7, midpoint: -0.25, radius });
        assert!(load_reformation(&bad).is_err());
    }
}

/// §2.1: the displacement ball u + w + w2 with the remainder 2^e*LambdaSigma,
/// and an exact value at a prescribed DOF.
#[test]
fn displacement_ball_carries_both_corrections_and_the_remainder() {
    let bound = MemberBoundInputs {
        w: vec![0.0, 2.0_f64.powi(-40)],
        w2: vec![0.0, 2.0_f64.powi(-100)],
        exponents: vec![i32::MIN, 3],
        lambda_sigma: 2.0_f64.powi(-80),
    };
    let u = [0.0, 1.0];
    let fixed = certify_displacement(0, &u, &bound).unwrap();
    assert_eq!((fixed.published, fixed.decision_radius()), (0.0, 0.0));
    let free = certify_displacement(1, &u, &bound).unwrap();
    assert_eq!(free.published, 1.0 + 2.0_f64.powi(-40));
    // 8*2^-80, formed upward (a few units in the last place).
    assert!(free.remainder >= 2.0_f64.powi(-77) && free.remainder <= 2.0_f64.powi(-77) * (1.0 + 2.0_f64.powi(-48)));
    // The corrections are in the exact midpoint 1 + 2^-40 + 2^-100, whose
    // radius 2^-77 decides against 1 + 2^-40 but not against 1.
    assert!(free.lower_exceeds(1.0));
    assert!(!free.lower_exceeds(1.0 + 2.0_f64.powi(-40)));
    assert!(free.upper_below(1.0 + 2.0_f64.powi(-39)));
    assert!(!free.upper_below(1.0 + 2.0_f64.powi(-40)));
}

/// §2.2: exact decisions, strict at the boundary and with both radii.
#[test]
fn certified_signs_and_stick_capacity_are_decided_exactly() {
    let row = |m: f64, r: f64| CertifiedValue::with_radius(m, r).unwrap().into_row().unwrap();
    assert_eq!(row(1.0, 0.5).established_sign(), Some(1.0));
    assert_eq!(row(-1.0, 0.5).established_sign(), Some(-1.0));
    assert_eq!(row(1.0, 1.0).established_sign(), None);
    assert_eq!(row(0.0, 0.0).established_sign(), None);
    // 0.1*10 = 1 + 2^-54*... exactly, so |m_t| = 1 is strictly below mu*N.
    assert!(CertifiedRow::stick_capacity_exceeds_explicit(0.1, 10.0, &row(1.0, 0.0)));
    assert!(!CertifiedRow::stick_capacity_exceeds_explicit(0.5, 10.0, &row(5.0, 0.0)));
    assert!(CertifiedRow::stick_capacity_exceeds(&row(-100.0, 1.0), 0.5, &row(49.4, 0.05)));
    assert!(!CertifiedRow::stick_capacity_exceeds(&row(-100.0, 1.0), 0.5, &row(49.4, 0.15)));
    assert!(!CertifiedRow::stick_capacity_exceeds(&row(0.5, 1.0), 0.5, &row(0.0, 0.0)));
    // The enclosure folds the remainder into the radius.
    let mut with_remainder = row(2.0, 0.25);
    with_remainder.remainder = 0.5;
    let enclosure = with_remainder.enclosure().into_row().unwrap();
    assert!(enclosure.radius >= 0.75 && enclosure.radius <= 0.75_f64.next_up());
}

/// T4-U5b item 6 (Codex scrutiny 1): the published displacement is the exact
/// sum rounded once. u = 1, w = 2^-53 (half an ulp of 1) and w2 = 2^-130 sum
/// just above the midpoint 1 + 2^-53: once rounded it is 1 + 2^-52; the p=128
/// midpoint (1 + 2^-53, the 2^-130 dropped) rounded again would tie to 1.
#[test]
fn displacement_publication_rounds_the_exact_sum_once() {
    let bound = MemberBoundInputs {
        w: vec![2.0_f64.powi(-53)],
        w2: vec![2.0_f64.powi(-130)],
        exponents: vec![0],
        lambda_sigma: 0.0,
    };
    let row = certify_displacement(0, &[1.0], &bound).unwrap();
    assert_eq!(row.published.to_bits(), (1.0 + 2.0_f64.powi(-52)).to_bits());
    assert_eq!(row.value().published().unwrap(), 1.0, "the double rounding it replaces");
    // The certificate encloses the exact sum for the published value: the
    // rounding term covers |published - m| (about 2^-53) and the ball's radius
    // covers |m - exact| (2^-130 below the grid of p=128).
    assert!(row.bound() >= 2.0_f64.powi(-53) && row.bound() <= 2.0_f64.powi(-52), "{}", row.bound());
    // Decisions are the ball's, unchanged by the publication.
    assert!(row.lower_exceeds(1.0) && row.upper_below(1.0 + 2.0_f64.powi(-52)));
    // The mirrored negative sum rounds once to -(1 + 2^-52).
    let negative = MemberBoundInputs { w: vec![-(2.0_f64.powi(-53))], w2: vec![-(2.0_f64.powi(-130))], exponents: vec![0], lambda_sigma: 0.0 };
    let row = certify_displacement(0, &[-1.0], &negative).unwrap();
    assert_eq!(row.published.to_bits(), (-(1.0 + 2.0_f64.powi(-52))).to_bits());
}

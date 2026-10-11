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
fn frame() -> FrameElement {
    FrameElement::new(
        FrameNode::new(0, [0.0; 3]).unwrap(),
        FrameNode::new(1, [2.0, 0.0, 0.0]).unwrap(),
        FrameSection::new(8.0, 4.0, 2.0, 3.0, 3.0, 6.0).unwrap(),
        [0.0, 1.0, 0.0],
    )
    .unwrap()
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
    let loads = CertifiedMemberLoads::from_primitives(0, 1, &primitives).unwrap();
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
#[test]
fn spring_support_on_pipe_sign_and_exact_signed_equilibrium() {
    let u = [0.25, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0];
    let mut b = bound(12);
    b.w[0] = 0.125;
    b.w2[0] = 0.0625;
    let spring = certify_spring_reaction(0, 16.0, &u, &b).unwrap();
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
        let row = certify_spring_reaction(0, 16.0, &u, &scaled).unwrap();
        assert_eq!(row.published, -7.0);
        assert!(row.remainder >= 16.0 && row.remainder < 16.0 * (1.0 + 1e-14));
    }
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
        crate::correct_norm::norm3(row.published, row.published, row.published);
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
        certify_arc_recovery(&arc, &span, &CertifiedMemberLoads::default(), &u, &b).unwrap();
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
    let tipnorm = crate::correct_norm::norm3(global[6], global[7], global[8]);
    for station in &recovered.sections {
        let q = certified_group(&station[..3]).unwrap();
        assert!((q.q - tipnorm).abs() <= q.bound + 8.0 * f64::EPSILON * tipnorm);
    }
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
    let loads =
        CertifiedMemberLoads::from_member_primitives(0, 1, &primitives, "member:1:A:").unwrap();
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
        let expected = crate::correct_norm::norm3(component, component, component);
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

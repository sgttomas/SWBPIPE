use super::*;

fn declared(label: &str, terms: &[(usize, f64)], offset: f64) -> FunctionalDescriptor {
    FunctionalDescriptor {
        key: FunctionalKey {
            case_id: "case".into(),
            quantity: FunctionalQuantity::DeclaredAffine {
                label: label.into(),
            },
            unit: FunctionalUnit::Dimensionless,
            convention: FunctionalConvention::DeclaredAffine,
        },
        offset: vec![vec![offset]],
        terms: terms
            .iter()
            .map(|&(dof, value)| AffineTerm {
                dof,
                products: vec![vec![value]],
            })
            .collect(),
    }
}
fn contributions(k: &[Vec<f64>]) -> Vec<StiffnessContribution> {
    k.iter()
        .enumerate()
        .flat_map(|(row, entries)| {
            entries
                .iter()
                .enumerate()
                .map(move |(col, &value)| StiffnessContribution { row, col, value })
        })
        .collect()
}
fn context(
    k: &[Vec<f64>],
    f: &[f64],
    free: &[usize],
    fixed: &[(usize, f64)],
    cs: &[StiffnessContribution],
    budget: &mut AttemptBudget,
) -> Context {
    Context::prepare_with_budget(
        &StructuralSystem {
            stiffness: k,
            force: f,
            free_dofs: free,
            prescribed: fixed,
            contributions: Some(cs),
            symmetry: None,
        },
        "source",
        ForceBasis::DeclaredVector,
        budget,
    )
    .result
    .unwrap()
}
fn expected(r: &Ratio, numerator: f64, denominator: f64) {
    // The oracle is the independently derived n/d stated in each test. Only the
    // exact equality cross-product uses the already-reviewed expansion kernel.
    let mut w = Work {
        limits: Limits::default(),
        used: 0,
        rejected: 0,
    };
    let n = w.scalar(numerator).unwrap();
    let d = w.scalar(denominator).unwrap();
    let lhs = w.mul(&r.numerator, &d).unwrap();
    let rhs = w.mul(&r.denominator, &n).unwrap();
    assert!(w.sum(&lhs, &rhs, -1.0).unwrap().is_zero());
}
fn retain(
    response: &Response<'_>,
    plan: &FunctionalPlan<'_>,
    budget: &mut AttemptBudget,
) -> RetainedFunctionalSet {
    let set = response.evaluate_functionals(plan, budget).result.unwrap();
    let mut leaves = Vec::new();
    for i in 0..response.context.f.len() {
        for quantity in [Quantity::Displacement, Quantity::Reaction] {
            leaves.push(
                response
                    .project_with_budget(quantity, i, 1e-9, budget)
                    .result
                    .unwrap(),
            );
        }
    }
    let projections: Vec<_> = (0..set.len())
        .map(|i| set.project(i, 1e-9, budget).result.unwrap())
        .collect();
    set.retain(&[], &leaves, &projections, budget)
        .result
        .unwrap()
}

#[test]
fn tiny_twist_recovers_signed_member_and_spring_actions_before_projection() {
    // Equilibrium of two torsional coordinates: root=T/k, tip=T/k+T/a.
    // With a=2^60 and k=1, both projected coordinates lose the tiny twist.
    for torque in [1.0, -3.0] {
        let a = 2.0_f64.powi(60);
        let k = vec![vec![a, -a], vec![-a, a]];
        let mut cs = contributions(&k);
        cs.push(StiffnessContribution {
            row: 0,
            col: 0,
            value: 1.0,
        });
        let mut budget = AttemptBudget::new(Limits::default());
        let c = context(&k, &[0.0, torque], &[0, 1], &[], &cs, &mut budget);
        let r = c.solve_with_budget(&mut budget).result.unwrap();
        let mut end_i = declared("i", &[(0, a), (1, -a)], 0.0);
        end_i.key.quantity = FunctionalQuantity::MemberEnd {
            member: "beam".into(),
            end: MemberEnd::I,
            row: 3,
        };
        end_i.key.unit = FunctionalUnit::NewtonMetre;
        end_i.key.convention = FunctionalConvention::NodeOnElement;
        let mut end_j = declared("j", &[(0, -a), (1, a)], 0.0);
        end_j.key = end_i.key.clone();
        end_j.key.quantity = FunctionalQuantity::MemberEnd {
            member: "beam".into(),
            end: MemberEnd::J,
            row: 3,
        };
        let mut spring = declared("spring", &[(0, -1.0)], 0.0);
        spring.key.quantity = FunctionalQuantity::GroundSpring {
            support: "root spring".into(),
            dof: 0,
        };
        spring.key.unit = FunctionalUnit::NewtonMetre;
        spring.key.convention = FunctionalConvention::SpringOnStructure;
        let ds = vec![end_i, end_j, spring];
        let p = FunctionalPlan::new(&c, &ds, &mut budget).result.unwrap();
        let fs = r.evaluate_functionals(&p, &mut budget).result.unwrap();
        for (i, value) in [-torque, torque, -torque].into_iter().enumerate() {
            expected(fs.ratio(i).unwrap(), value, 1.0);
            assert_eq!(
                fs.project(i, 1e-9, &mut budget).result.unwrap().value(),
                value
            );
        }
        expected(r.reaction(0).unwrap(), 0.0, 1.0);
        let u0 = r
            .project_with_budget(Quantity::Displacement, 0, 1e-9, &mut budget)
            .result
            .unwrap()
            .value();
        let u1 = r
            .project_with_budget(Quantity::Displacement, 1, 1e-9, &mut budget)
            .result
            .unwrap()
            .value();
        assert_eq!(u0, u1);
        assert_eq!(a * (u1 - u0), 0.0);
        assert!(budget.report().charged < Limits::default().operations);
    }
}

#[test]
fn coefficient_product_recipe_is_not_a_rounded_coefficient() {
    let k = vec![vec![1.0]];
    let mut budget = AttemptBudget::new(Limits::default());
    let c = context(&k, &[1.0], &[0], &[], &contributions(&k), &mut budget);
    let r = c.solve_with_budget(&mut budget).result.unwrap();
    let e = 2.0_f64.powi(-52);
    let mut d = declared("product cancellation", &[], 0.0);
    d.terms.push(AffineTerm {
        dof: 0,
        products: vec![vec![1.0 + e, 1.0 - e], vec![-1.0]],
    });
    assert_eq!((1.0 + e) * (1.0 - e) - 1.0, 0.0);
    let p = FunctionalPlan::new(&c, &[d], &mut budget).result.unwrap();
    let set = r.evaluate_functionals(&p, &mut budget).result.unwrap();
    // (1+e)(1-e)-1 = -e² = -2^-104, without rounded coefficient formation.
    expected(set.ratio(0).unwrap(), -2.0_f64.powi(-104), 1.0);
    assert_eq!(
        set.project(0, 1e-9, &mut budget).result.unwrap().value(),
        -2.0_f64.powi(-104)
    );
}

#[test]
fn all_fixed_and_mixed_partition_include_affine_offsets() {
    let k = vec![vec![2.0, 0.0], vec![0.0, 3.0]];
    let mut b = AttemptBudget::new(Limits::default());
    let c = context(
        &k,
        &[4.0, -3.0],
        &[],
        &[(0, 2.0), (1, -1.0)],
        &contributions(&k),
        &mut b,
    );
    let r = c.solve_with_budget(&mut b).result.unwrap();
    let p = FunctionalPlan::new(
        &c,
        &[declared("fixed", &[(0, 3.0), (1, -4.0)], 7.0)],
        &mut b,
    )
    .result
    .unwrap();
    let fs = r.evaluate_functionals(&p, &mut b).result.unwrap();
    expected(fs.ratio(0).unwrap(), 17.0, 1.0);

    let k = vec![vec![4.0, 1.0], vec![1.0, 3.0]];
    let mut b = AttemptBudget::new(Limits::default());
    let c = context(
        &k,
        &[9.0, 0.0],
        &[0],
        &[(1, 2.0)],
        &contributions(&k),
        &mut b,
    );
    let r = c.solve_with_budget(&mut b).result.unwrap();
    // 4*u0+2=9 => u0=7/4; 5+2*u0-3*2=5/2; r1=7/4+6.
    let p = FunctionalPlan::new(
        &c,
        &[declared("mixed", &[(0, 2.0), (1, -3.0)], 5.0)],
        &mut b,
    )
    .result
    .unwrap();
    expected(
        r.evaluate_functionals(&p, &mut b)
            .result
            .unwrap()
            .ratio(0)
            .unwrap(),
        5.0,
        2.0,
    );
    expected(r.reaction(1).unwrap(), 31.0, 4.0);
}

#[test]
fn generic_coupled_and_multiple_independent_blocks_have_rational_oracles() {
    let k = vec![vec![4.0, 1.0], vec![1.0, 3.0]];
    let mut b = AttemptBudget::new(Limits::default());
    let c = context(&k, &[1.0, 2.0], &[0, 1], &[], &contributions(&k), &mut b);
    let r = c.solve_with_budget(&mut b).result.unwrap();
    // Independent elimination: 11*u0=1 and 11*u1=7, hence 2*u0+3*u1=23/11.
    let p = FunctionalPlan::new(
        &c,
        &[declared("coupled", &[(0, 2.0), (1, 3.0)], 0.0)],
        &mut b,
    )
    .result
    .unwrap();
    let set = r.evaluate_functionals(&p, &mut b).result.unwrap();
    expected(set.ratio(0).unwrap(), 23.0, 11.0);
    assert!(
        set.project(0, 1e-9, &mut b)
            .result
            .unwrap()
            .relative_error_bound()
            <= 1e-9
    );
    // Shared denominator is exactly one determinant, not determinant squared.
    assert_eq!(set.ratio(0).unwrap().denominator_terms(), &[11.0]);

    let k = vec![
        vec![2.0, 0.0, 0.0, 0.0],
        vec![0.0, 4.0, 1.0, 0.0],
        vec![0.0, 1.0, 3.0, 0.0],
        vec![0.0, 0.0, 0.0, 5.0],
    ];
    let mut b = AttemptBudget::new(Limits::default());
    let c = context(
        &k,
        &[1.0, 1.0, 2.0, 3.0],
        &[0, 1, 2, 3],
        &[],
        &contributions(&k),
        &mut b,
    );
    let r = c.solve_with_budget(&mut b).result.unwrap();
    // u=(1/2,1/11,7/11,3/5); 1+2u0+3u1-2u2+5u3 = 4.
    let p = FunctionalPlan::new(
        &c,
        &[
            declared(
                "three blocks",
                &[(0, 2.0), (1, 3.0), (2, -2.0), (3, 5.0)],
                1.0,
            ),
            declared("two blocks", &[(0, 1.0), (3, 1.0)], 0.0),
        ],
        &mut b,
    )
    .result
    .unwrap();
    let set = r.evaluate_functionals(&p, &mut b).result.unwrap();
    expected(set.ratio(0).unwrap(), 4.0, 1.0);
    expected(set.ratio(1).unwrap(), 11.0, 10.0);
}

#[test]
fn exact_zero_and_nonzero_unprojectable_are_distinct() {
    let k = vec![vec![2.0]];
    let mut b = AttemptBudget::new(Limits::default());
    let c = context(
        &k,
        &[f64::from_bits(1)],
        &[0],
        &[],
        &contributions(&k),
        &mut b,
    );
    let r = c.solve_with_budget(&mut b).result.unwrap();
    let p = FunctionalPlan::new(
        &c,
        &[
            declared("zero", &[(0, 1.0), (0, -1.0)], 0.0),
            declared("tiny", &[(0, 1.0)], 0.0),
        ],
        &mut b,
    )
    .result
    .unwrap();
    let set = r.evaluate_functionals(&p, &mut b).result.unwrap();
    let zero = set.project(0, 1e-9, &mut b).result.unwrap();
    assert_eq!(zero.exact_sign(), Sign::Zero);
    assert_eq!(zero.basis(), ProjectionBasis::ExactZero);
    assert_eq!(set.ratio(1).unwrap().sign(), Sign::Positive);
    assert!(matches!(
        set.project(1, 1e-9, &mut b).result,
        Err(Error::ProjectionUnresolved(_))
    ));
}

#[test]
fn retention_replays_expected_plan_and_rejects_corruption_order_and_source_changes() {
    let k = vec![vec![2.0, 0.0], vec![0.0, 3.0]];
    let cs = contributions(&k);
    let mut b = AttemptBudget::new(Limits::default());
    let c = context(&k, &[1.0, 1.0], &[0, 1], &[], &cs, &mut b);
    let r = c.solve_with_budget(&mut b).result.unwrap();
    let ds = vec![
        declared("sum", &[(0, 1.0), (1, 1.0)], 0.0),
        declared("difference", &[(0, 1.0), (1, -1.0)], 0.0),
    ];
    let p = FunctionalPlan::new(&c, &ds, &mut b).result.unwrap();
    let retained = retain(&r, &p, &mut b);
    let system = StructuralSystem {
        stiffness: &k,
        force: &[1.0, 1.0],
        free_dofs: &[0, 1],
        prescribed: &[],
        contributions: Some(&cs),
        symmetry: None,
    };
    // Replay is charged to the SAME attempt here, after every construction step.
    let mut fresh = AttemptBudget::new(Limits::default());
    assert!(retained.check_binding_with_budget(
        &system, "source", ForceBasis::DeclaredVector, &ds, &mut fresh
    ).result.is_err());
    assert!(retained.clone().replay_against(
        &system, "source", ForceBasis::DeclaredVector, &ds, &[], 1e-9, &mut fresh
    ).result.is_err());
    assert!(retained.clone().check_binding_with_budget(
        &system, "source", ForceBasis::DeclaredVector, &ds, &mut b
    ).result.is_ok());
    assert_eq!(
        retained
            .replay_against(
                &system,
                "source",
                ForceBasis::DeclaredVector,
                &ds,
                &[],
                1e-9,
                &mut b
            )
            .result
            .unwrap()
            .functionals(),
        2
    );
    for change in 0..7 {
        let mut altered = ds.clone();
        match change {
            0 => altered.swap(0, 1),
            1 => altered[0].terms[0].products[0][0] = 2.0,
            2 => altered[0].key.case_id.push('x'),
            3 => {
                altered[0].key.quantity = FunctionalQuantity::DeclaredAffine {
                    label: "other".into(),
                }
            }
            4 => altered[0].offset[0][0] = -0.0,
            5 => altered[0].terms.swap(0, 1),
            _ => altered[0].key.unit = FunctionalUnit::Metre,
        }
        assert!(retained
            .replay_against(
                &system,
                "source",
                ForceBasis::DeclaredVector,
                &altered,
                &[],
                1e-9,
                &mut b
            )
            .result
            .is_err());
    }
    let wrong_source = StructuralSystem {
        stiffness: &k,
        force: &[1.0, 2.0],
        free_dofs: &[0, 1],
        prescribed: &[],
        contributions: Some(&cs),
        symmetry: None,
    };
    assert!(retained
        .replay_against(
            &wrong_source,
            "source",
            ForceBasis::DeclaredVector,
            &ds,
            &[],
            1e-9,
            &mut b
        )
        .result
        .is_err());
    let mut corrupt = retained.clone();
    corrupt.values[0].numerator.terms.reverse();
    corrupt.values[0].numerator.terms.push(1.0);
    assert!(corrupt
        .replay_against(
            &system,
            "source",
            ForceBasis::DeclaredVector,
            &ds,
            &[],
            1e-9,
            &mut b
        )
        .result
        .is_err());
    let mut corrupt = retained.clone();
    corrupt.projections.swap(0, 1);
    assert!(corrupt
        .replay_against(
            &system,
            "source",
            ForceBasis::DeclaredVector,
            &ds,
            &[],
            1e-9,
            &mut b
        )
        .result
        .is_err());
    let mut corrupt = retained.clone();
    corrupt.projections[0].exact_sign = Sign::Negative;
    assert!(corrupt
        .replay_against(
            &system,
            "source",
            ForceBasis::DeclaredVector,
            &ds,
            &[],
            1e-9,
            &mut b
        )
        .result
        .is_err());
    assert!(retained
        .replay_against(
            &system,
            "source",
            ForceBasis::DeclaredVector,
            &ds,
            &[],
            1e-8,
            &mut b
        )
        .result
        .is_err());
}

#[test]
fn plan_binding_and_typed_identity_and_metering_cannot_be_bypassed() {
    let k = vec![vec![1.0]];
    let cs = contributions(&k);
    let mut b = AttemptBudget::new(Limits::default());
    let c = context(&k, &[1.0], &[0], &[], &cs, &mut b);
    let c2 = context(&k, &[1.0], &[0], &[], &cs, &mut b);
    let r = c.solve_with_budget(&mut b).result.unwrap();
    let other_plan = FunctionalPlan::new(&c2, &[declared("d", &[(0, 1.0)], 0.0)], &mut b)
        .result
        .unwrap();
    assert!(r.evaluate_functionals(&other_plan, &mut b).result.is_err());
    let mut fresh = AttemptBudget::new(Limits::default());
    assert!(c.solve_with_budget(&mut fresh).result.is_err());
    assert!(r
        .project_with_budget(Quantity::Displacement, 0, 1e-9, &mut fresh)
        .result
        .is_err());
    let p = FunctionalPlan::new(&c, &[declared("d", &[(0, 1.0)], 0.0)], &mut b)
        .result
        .unwrap();
    let unmetered = c.solve().unwrap();
    assert!(unmetered.evaluate_functionals(&p, &mut b).result.is_err());
    let fs = r.evaluate_functionals(&p, &mut b).result.unwrap();
    let ps = [fs.project(0, 1e-9, &mut b).result.unwrap()];
    let legacy = [
        r.project_displacement(0, 1e-9, 100_000).result.unwrap(),
        r.project_reaction(0, 1e-9, 100_000).result.unwrap(),
    ];
    assert!(fs.retain(&[], &legacy, &ps, &mut b).result.is_err());
    let mut bad = declared("bad typed", &[], 0.0);
    bad.key.quantity = FunctionalQuantity::MemberEnd {
        member: "beam".into(),
        end: MemberEnd::I,
        row: 9,
    };
    assert!(FunctionalPlan::new(&c, &[bad], &mut b).result.is_err());
    let duplicate = declared("same", &[], 0.0);
    assert!(
        FunctionalPlan::new(&c, &[duplicate.clone(), duplicate], &mut b)
            .result
            .is_err()
    );
}

#[test]
fn denied_reservations_long_ids_range_failures_and_retries_keep_the_ledger() {
    let k = vec![vec![1.0]];
    let cs = contributions(&k);
    let mut b = AttemptBudget::new(Limits::default());
    let c = context(&k, &[1.0], &[0], &[], &cs, &mut b);
    let r = c.solve_with_budget(&mut b).result.unwrap();
    let mut range = declared("range", &[], 0.0);
    range.offset = vec![vec![f64::MAX, 2.0]];
    let p = FunctionalPlan::new(&c, &[range], &mut b).result.unwrap();
    let before = b.report().charged;
    assert!(matches!(
        r.evaluate_functionals(&p, &mut b).result,
        Err(Error::Arithmetic(_))
    ));
    assert!(b.report().charged > before);
    let before = b.report();
    let long = declared(&"x".repeat(2_000_000), &[], 0.0);
    assert_eq!(
        FunctionalPlan::new(&c, &[long], &mut b).result.unwrap_err(),
        Error::Budget
    );
    assert!(b.report().charged >= before.charged && b.report().rejected > before.rejected);
    let remaining = b.remaining();
    b.charge(AttemptStage::SourceClosure, remaining).unwrap();
    let charged = b.report().charged;
    for _ in 0..2 {
        assert_eq!(
            FunctionalPlan::new(&c, &[declared("retry", &[], 0.0)], &mut b)
                .result
                .unwrap_err(),
            Error::Budget
        );
        assert_eq!(b.report().charged, charged);
    }
    assert!(b.report().rejected > before.rejected);

    let mut tiny = AttemptBudget::new(Limits {
        operations: 1,
        ..Limits::default()
    });
    let system = StructuralSystem {
        stiffness: &k,
        force: &[1.0],
        free_dofs: &[0],
        prescribed: &[],
        contributions: Some(&cs),
        symmetry: None,
    };
    let failure =
        Context::prepare_with_budget(&system, "source", ForceBasis::DeclaredVector, &mut tiny);
    assert_eq!(failure.result.unwrap_err(), Error::Budget);
    assert_eq!(failure.work.charged, 0);
    assert!(failure.work.rejected > 0);
    assert_eq!(tiny.stage(), AttemptStage::Preparation);
}

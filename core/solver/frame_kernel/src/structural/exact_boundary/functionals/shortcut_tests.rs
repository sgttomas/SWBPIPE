//! Independent R-A killing inputs, from source-work-nondyadic-inputs §3.1–3.2.
//! Recognition is observed through row paths; every successful value is also
//! compared with the test-only evaluation that executes both full recipes.
use super::super::exact_operation_count::{take_row_paths, RowPath};
use super::*;

fn bits(xs: &[f64]) -> Vec<u64> {
    xs.iter().map(|x| x.to_bits()).collect()
}
fn ratio_bits(r: &Ratio) -> (Vec<u64>, Vec<u64>) {
    (bits(&r.numerator.terms), bits(&r.denominator.terms))
}
fn source(terms: Vec<AffineTerm>, offset: Products, component: u8) -> FunctionalDescriptor {
    FunctionalDescriptor {
        key: FunctionalKey {
            case_id: "case".into(),
            quantity: FunctionalQuantity::MemberEnd {
                member: "beam".into(),
                end: MemberEnd::I,
                row: component,
            },
            unit: if component < 3 {
                FunctionalUnit::Newton
            } else {
                FunctionalUnit::NewtonMetre
            },
            convention: FunctionalConvention::NodeOnElement,
        },
        terms,
        offset,
    }
}
fn term(dof: usize, products: Products) -> AffineTerm {
    AffineTerm { dof, products }
}
fn section(s: &FunctionalDescriptor, station: f64) -> FunctionalDescriptor {
    let mut d = s.clone();
    let FunctionalQuantity::MemberEnd { member, row, .. } = &s.key.quantity else {
        unreachable!()
    };
    d.key.quantity = FunctionalQuantity::MemberSection {
        member: member.clone(),
        station_bits: station.to_bits(),
        component: *row,
    };
    d.key.convention = FunctionalConvention::SectionLocal;
    for p in d
        .offset
        .iter_mut()
        .chain(d.terms.iter_mut().flat_map(|t| &mut t.products))
    {
        p.push(-1.0);
    }
    d
}
fn contributions(k: &[Vec<f64>]) -> Vec<StiffnessContribution> {
    k.iter()
        .enumerate()
        .flat_map(|(row, xs)| {
            xs.iter()
                .enumerate()
                .map(move |(col, &value)| StiffnessContribution { row, col, value })
        })
        .collect()
}
fn base() -> FunctionalDescriptor {
    source(
        vec![
            term(1, vec![vec![2.5, 0.75]]),
            term(3, vec![vec![4.5, 0.5]]),
        ],
        vec![],
        0,
    )
}

fn b1_compare(
    label: &str,
    ds: &[FunctionalDescriptor],
    hints: &[Option<u32>],
) -> (Vec<RowPath>, Vec<(Vec<u64>, Vec<u64>)>) {
    // Unit stiffness, a multi-term load numerator, two independent free
    // blocks, one nonzero prescription and one signed-zero prescription.
    let k = vec![
        vec![2.0, 0.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0, 0.0],
        vec![0.0, 0.0, 5.0, 0.0],
        vec![0.0, 0.0, 0.0, 2.0],
    ];
    let loads = vec![
        ForceContribution {
            source: "main".into(),
            dof: 1,
            value: 1.0,
        },
        ForceContribution {
            source: "tail".into(),
            dof: 1,
            value: 2.0f64.powi(-54),
        },
        ForceContribution {
            source: "low tail".into(),
            dof: 1,
            value: 2.0f64.powi(-108),
        },
        ForceContribution {
            source: "other block".into(),
            dof: 2,
            value: 3.25,
        },
    ];
    let mut budget = AttemptBudget::new(Limits::default());
    let cs = contributions(&k);
    let c = Context::prepare_with_budget(
        &StructuralSystem {
            stiffness: &k,
            force: &[0.0, 1.0, 3.25, 0.0],
            free_dofs: &[1, 2],
            prescribed: &[(0, 0.5), (3, -0.0)],
            contributions: Some(&cs),
            contribution_groups: None,
            symmetry: None,
        },
        "killing fixture",
        ForceBasis::IdentifiedContributions(&loads),
        &[],
        &mut budget,
    )
    .result
    .unwrap();
    let r = c.solve_with_budget(&mut budget).result.unwrap();
    assert!(r.displacement[1].numerator.terms.len() > 1);
    // Without a hint per descriptor the plan takes no rule, so B1 is off.
    let p = if hints.len() == ds.len() {
        FunctionalPlan::new_with_hints(&c, ds, &|i| hints[i], &mut budget)
    } else {
        FunctionalPlan::new(&c, ds, &mut budget)
    }
        .result
        .unwrap();
    take_row_paths();
    let fast = r.evaluate_functionals(&p, &mut budget).result.unwrap();
    let paths = take_row_paths();
    assert_eq!(paths.len(), 1, "{label}");
    let long = r.evaluate_functionals_long(&p, &mut budget).result.unwrap();
    let long_paths = take_row_paths();
    assert_eq!(
        long_paths,
        vec![vec![RowPath::Long { b2_skipped: 0 }; ds.len()]],
        "{label}"
    );
    let fast: Vec<_> = fast.ratios().iter().map(ratio_bits).collect();
    let long: Vec<_> = long.ratios().iter().map(ratio_bits).collect();
    assert_eq!(fast, long, "{label}: Ratio bytes");
    (paths.into_iter().next().unwrap(), fast)
}

#[test]
fn b1_recognition_refuses_near_matches() {
    let good = base();
    let mut cases = Vec::new();
    let mut d = section(&good, 0.0);
    d.terms[0].products[0][0] = f64::from_bits(2.5f64.to_bits() + 1);
    cases.push(("one atom differs", good.clone(), d, Some(0)));
    let mut d = section(&good, 0.0);
    d.terms.push(term(1, vec![vec![2.5, 0.75, -1.0]]));
    cases.push(("extra term without zero", good.clone(), d, Some(0)));
    // The appended atom must be exactly -1.0 (Opus review, round 1: a recognition
    // accepting +1.0 survived and gave the source unnegated row negated).
    for (name, appended) in [
        ("appended +1.0", 1.0),
        ("appended -2.0", -2.0),
        ("appended -0.0", -0.0),
        ("appended +0.0", 0.0),
    ] {
        let mut d = section(&good, 0.0);
        for t in &mut d.terms {
            for p in &mut t.products {
                *p.last_mut().unwrap() = appended;
            }
        }
        cases.push((name, good.clone(), d, Some(0)));
    }
    for (name, products) in [
        ("three atoms", vec![vec![2.5, 0.75, 0.25]]),
        ("positive unit atom", vec![vec![1.0, 2.5]]),
        ("negative unit atom", vec![vec![-1.0, 2.5]]),
        ("dynamic positive unit coefficient", vec![vec![2.0, 0.5]]),
        ("dynamic negative unit coefficient", vec![vec![-2.0, 0.5]]),
    ] {
        let s = source(vec![term(1, products)], vec![], 0);
        cases.push((name, s.clone(), section(&s, 0.0), Some(0)));
    }
    let s = source(
        vec![term(1, vec![vec![-2.0]]), term(0, vec![vec![2.0]])],
        vec![],
        0,
    );
    cases.push((
        "reviewer fixed unit: 2*u_prescribed - 2*u_free",
        s.clone(),
        section(&s, 0.0),
        Some(0),
    ));
    let mut s = good.clone();
    s.offset = vec![vec![0.25]];
    cases.push(("nonempty offset", s.clone(), section(&s, 0.0), Some(0)));
    let s = source(
        vec![term(1, vec![vec![2.5]]), term(2, vec![vec![3.5]])],
        vec![],
        0,
    );
    cases.push((
        "two contributing blocks",
        s.clone(),
        section(&s, 0.0),
        Some(0),
    ));
    let mut d = section(&good, 0.0);
    d.terms[0].dof = 2;
    cases.push(("different DOF", good.clone(), d, Some(0)));
    let mut d = section(&good, 0.0);
    d.terms.pop();
    cases.push(("missing term", good.clone(), d, Some(0)));
    let mut d = section(&good, 0.0);
    d.offset.push(vec![0.25]);
    cases.push(("different offset", good.clone(), d, Some(0)));
    let mut d = section(&good, 0.0);
    d.terms.push(term(1, vec![vec![2.5, 0.75, 2.0, 0.0]]));
    cases.push(("nonunit before extra zero", good.clone(), d, Some(0)));
    // The two cases above add one term to a two-term source, so a term-count
    // check refuses them first. These match every term of the source row and
    // so reach the zero test itself (Codex review, round 1).
    let extended = |tail: &[f64]| -> Vec<AffineTerm> {
        good.terms
            .iter()
            .map(|t| {
                let products = t
                    .products
                    .iter()
                    .map(|p| p.iter().chain(tail).copied().collect())
                    .collect();
                term(t.dof, products)
            })
            .collect()
    };
    let mut d = section(&good, 0.0);
    d.terms.extend(extended(&[-1.0]));
    cases.push(("every extra term unit-only, no zero", good.clone(), d, Some(0)));
    let mut d = section(&good, 0.0);
    d.terms.extend(extended(&[2.0, 0.0]));
    cases.push(("every extra term nonunit before its zero", good.clone(), d, Some(0)));
    for (name, alter) in [
        ("other component", 0),
        ("other member", 1),
        ("other case", 2),
        ("J end", 3),
    ] {
        let mut s = good.clone();
        match alter {
            0 => {
                if let FunctionalQuantity::MemberEnd { row, .. } = &mut s.key.quantity {
                    *row = 1;
                }
            }
            1 => {
                if let FunctionalQuantity::MemberEnd { member, .. } = &mut s.key.quantity {
                    *member = "other beam".into();
                }
            }
            2 => s.key.case_id = "other case".into(),
            _ => {
                if let FunctionalQuantity::MemberEnd { end, .. } = &mut s.key.quantity {
                    *end = MemberEnd::J;
                }
            }
        }
        cases.push((name, s, section(&good, 0.0), Some(0)));
    }
    for (name, hint) in [
        ("missing hint", None),
        ("self hint", Some(1)),
        ("later hint", Some(2)),
        ("out of range", Some(u32::MAX)),
    ] {
        cases.push((name, good.clone(), section(&good, 0.0), hint));
    }
    for (name, s, d, hint) in cases {
        // The trailing i-end row makes index 2 a real later index, as distinct
        // from an out-of-range hint. Its member identity prevents duplicates.
        let mut later = good.clone();
        if let FunctionalQuantity::MemberEnd { member, .. } = &mut later.key.quantity {
            *member = "later".into();
        }
        let (paths, _) = b1_compare(name, &[s, d, later], &[None, hint, None]);
        assert!(
            matches!(paths[1], RowPath::Long { .. }),
            "{name}: {paths:?}"
        );
    }
    let ds = [good.clone(), section(&good, 0.0)];
    let (paths, _) = b1_compare("no hint rule", &ds, &[None]);
    assert!(paths.iter().all(|p| matches!(p, RowPath::Long { .. })));
}

#[test]
fn b1_refuses_a_difference_in_the_last_atom() {
    // The near-match cases above alter a product's first atom; this one alters
    // the last atom before the appended -1.0 (mutant run, Codex: a recognition
    // that skipped it survived).
    let s = base();
    let mut d = section(&s, 0.0);
    let atom = &mut d.terms[0].products[0][1];
    *atom = f64::from_bits(atom.to_bits() + 1);
    let (paths, _) = b1_compare("last atom differs", &[s, d], &[None, Some(0)]);
    assert!(matches!(paths[1], RowPath::Long { .. }), "{paths:?}");
}

#[test]
fn b1_negates_valid_rows_and_preserves_zero_sources() {
    let good = base();
    let zero = source(vec![term(3, vec![vec![2.5, 0.75]])], vec![], 1);
    let mut extra = section(&zero, 0.5);
    extra.terms.extend(good.terms.iter().cloned().map(|mut t| {
        for p in &mut t.products {
            p.extend([-1.0, -0.0, 2.5]);
        }
        t
    }));
    let ds = [
        good.clone(),
        zero.clone(),
        section(&good, 0.0),
        section(&zero, 0.0),
        extra,
    ];
    let (paths, values) = b1_compare(
        "valid and exact zero",
        &ds,
        &[None, None, Some(0), Some(1), Some(1)],
    );
    assert_eq!(
        paths,
        [
            RowPath::Long { b2_skipped: 1 },
            RowPath::Long { b2_skipped: 1 },
            RowPath::Negated,
            RowPath::Negated,
            RowPath::Negated
        ]
    );
    assert!(!values[0].0.is_empty()); // an unnegated-reuse mutant changes these bits
    assert_eq!(
        values[2].0,
        values[0]
            .0
            .iter()
            .map(|b| b ^ (1 << 63))
            .collect::<Vec<_>>()
    );
    assert!(values[1].0.is_empty() && values[3].0.is_empty() && values[4].0.is_empty());
}

#[derive(Debug)]
struct B2Run {
    result: Result<Vec<(Vec<u64>, Vec<u64>)>, Error>,
    paths: Vec<Vec<RowPath>>,
    charged: usize,
    total: usize,
}
fn b2_run(
    products: &[Products],
    prescribed: f64,
    long: bool,
    operations: usize,
    expansion_terms: usize,
) -> B2Run {
    let mut budget = AttemptBudget::new(Limits {
        operations,
        expansion_terms,
        ..Limits::default()
    });
    let k = vec![vec![2.0]];
    let cs = contributions(&k);
    let c = Context::prepare_with_budget(
        &StructuralSystem {
            stiffness: &k,
            force: &[0.0],
            free_dofs: &[],
            prescribed: &[(0, prescribed)],
            contributions: Some(&cs),
            contribution_groups: None,
            symmetry: None,
        },
        "B2 window",
        ForceBasis::DeclaredVector,
        &[],
        &mut budget,
    )
    .result
    .unwrap();
    let r = c.solve_with_budget(&mut budget).result.unwrap();
    let ds = [FunctionalDescriptor {
        key: FunctionalKey {
            case_id: "case".into(),
            quantity: FunctionalQuantity::DeclaredAffine {
                label: "zero prescribed".into(),
            },
            unit: FunctionalUnit::Dimensionless,
            convention: FunctionalConvention::DeclaredAffine,
        },
        offset: vec![],
        terms: products.iter().cloned().map(|p| term(0, p)).collect(),
    }];
    let p = FunctionalPlan::new(&c, &ds, &mut budget).result.unwrap();
    let before = budget.report().charged;
    take_row_paths();
    let a = if long {
        r.evaluate_functionals_long(&p, &mut budget)
    } else {
        r.evaluate_functionals(&p, &mut budget)
    };
    B2Run {
        result: a
            .result
            .map(|s| s.ratios().iter().map(ratio_bits).collect()),
        paths: take_row_paths(),
        charged: budget.report().charged - before,
        total: budget.report().charged,
    }
}

#[test]
fn b2_window_keeps_every_refusal() {
    // Expected paths come directly from the window's three inclusive bounds.
    let mut cases: Vec<(&str, Vec<Products>, bool, bool)> = Vec::new();
    for (name, exponent, inside) in [
        ("upper inside", 150, true),
        ("upper outside", 151, false),
        ("lower inside", -150, true),
        ("lower outside", -151, false),
    ] {
        cases.push((
            name,
            vec![vec![vec![1.25 * 2.0f64.powi(exponent), 0.75]]],
            inside,
            false,
        ));
    }
    cases.extend([
        ("five atoms", vec![vec![vec![1.25; 5]]], true, false),
        ("six atoms", vec![vec![vec![1.25; 6]]], false, false),
        ("64 products", vec![vec![vec![1.25, 0.75]; 64]], true, false),
        (
            "65 products",
            vec![vec![vec![1.25, 0.75]; 65]],
            false,
            false,
        ),
        (
            "64 across terms",
            vec![vec![vec![1.25, 0.75]; 32]; 2],
            true,
            false,
        ),
        (
            "65 across terms",
            vec![vec![vec![1.25, 0.75]; 32], vec![vec![1.25, 0.75]; 33]],
            false,
            false,
        ),
        (
            "positive zero atom",
            vec![vec![vec![2.5, 0.0]]],
            false,
            false,
        ),
        (
            "negative zero atom",
            vec![vec![vec![2.5, -0.0]]],
            false,
            false,
        ),
        (
            "positive unit atom",
            vec![vec![vec![1.25, 1.0]]],
            true,
            false,
        ),
        (
            "negative unit atom",
            vec![vec![vec![1.25, -1.0]]],
            true,
            false,
        ),
        (
            "subnormal",
            vec![vec![vec![f64::from_bits(1)]]],
            false,
            false,
        ),
        ("empty product sum", vec![vec![]], true, false),
    ]);
    let k = f64::from_bits(0x0021_abe4_b73f_efb7); // 0x1.1abe4b73fefb7p-1021
    for sign in [1.0, -1.0] {
        cases.push((
            "rescued intermediate underflow",
            vec![vec![vec![k, sign, 0.25, 4.0]]],
            false,
            true,
        ));
    }
    // hi fits, but the nonzero low tail is below the subnormal grid.
    let a = (1.0 + f64::EPSILON) * 2.0f64.powi(-520);
    assert!(a * a > 0.0);
    cases.push(("low-tail bit loss", vec![vec![vec![a, a]]], false, true));
    cases.push((
        "addition overflow",
        vec![vec![vec![f64::MAX], vec![f64::MAX]]],
        false,
        true,
    ));
    // A zero later in a product cannot excuse a refusal that precedes it.
    cases.push((
        "zero after low-tail loss",
        vec![vec![vec![a, a, 0.0]]],
        false,
        true,
    ));
    cases.push((
        "unit before low-tail loss",
        vec![vec![vec![1.0, a, a]]],
        false,
        true,
    ));
    for prescribed in [0.0, -0.0] {
        for (label, ps, inside, refuses) in &cases {
            let fast = b2_run(ps, prescribed, false, Limits::default().operations, 256);
            let long = b2_run(ps, prescribed, true, Limits::default().operations, 256);
            assert_eq!(
                fast.result, long.result,
                "{label}, prescribed={prescribed:?}: fast={fast:?}, long={long:?}"
            );
            if *refuses {
                assert!(
                    matches!(
                        long.result,
                        Err(Error::Arithmetic(StructuralError::Range(_)))
                    ),
                    "{label}: {long:?}"
                );
            } else {
                assert!(long.result.is_ok(), "{label}: {long:?}");
                assert_eq!(
                    fast.paths,
                    vec![vec![RowPath::Long {
                        b2_skipped: if *inside { ps.len() as u32 } else { 0 }
                    }]],
                    "{label}"
                );
            }
        }
    }
    // No skip is allowed for either sign of a nonzero prescription.
    for prescribed in [0.5, -0.5] {
        let ps = vec![vec![vec![1.25, 0.75]]];
        let fast = b2_run(&ps, prescribed, false, Limits::default().operations, 256);
        let long = b2_run(&ps, prescribed, true, Limits::default().operations, 256);
        assert_eq!(fast.result, long.result);
        assert_eq!(fast.paths, vec![vec![RowPath::Long { b2_skipped: 0 }]]);
        assert!(!fast.result.unwrap()[0].0.is_empty());
    }
    // The window proves absence of Range, not absence of an expansion cap.
    // With two allowed terms, this three-atom product exceeds the cap on the long path;
    // skipping it removes that Budget refusal and spends less evaluation work.
    let ps = vec![vec![vec![
        1.0 + f64::EPSILON,
        1.0 - f64::EPSILON,
        1.0 + f64::EPSILON,
    ]]];
    let fast = b2_run(&ps, 0.0, false, Limits::default().operations, 2);
    let long = b2_run(&ps, 0.0, true, Limits::default().operations, 2);
    assert_eq!(long.result, Err(Error::Budget));
    assert!(fast.result.is_ok());
    assert!(fast.charged < long.charged);
}

#[test]
fn b2_declined_scans_charge_exactly_their_overhead() {
    // §3.1 ("Consequence"): the window's charges are paid whether or not a
    // recipe is skipped. This recipe is immediately outside the exponent
    // window, so the long path's work still runs, plus exactly the window's
    // overhead from the as-built schedule: at the start, 3n + |prescribed| +
    // the sum of block orders = 3 + 1 + 0; for the row, |terms| = 1, then
    // atoms(P) of the zero-prescribed term = 2 atoms + 1 product.
    let ps = vec![vec![vec![1.25 * 2.0f64.powi(151), 0.75]]];
    let long = b2_run(&ps, 0.0, true, Limits::default().operations, 256);
    let fast = b2_run(&ps, 0.0, false, Limits::default().operations, 256);
    assert!(long.result.is_ok(), "{long:?}");
    assert_eq!(fast.result, long.result);
    assert_eq!(fast.paths, vec![vec![RowPath::Long { b2_skipped: 0 }]]);
    assert_eq!(fast.charged, long.charged + (3 + 1 + 0) + 1 + (2 + 1));
    // So a limit between the two totals refuses the shortcut evaluation with
    // `Budget` where the long one completes: the overhead is charged, never
    // assumed away, and the refusal is `Budget`, never `Range`.
    let fast_short = b2_run(&ps, 0.0, false, long.total, 256);
    let long_short = b2_run(&ps, 0.0, true, long.total, 256);
    assert!(long_short.result.is_ok());
    assert_eq!(fast_short.result, Err(Error::Budget));
}

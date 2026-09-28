//! K2b kernel tests (T3 D1 revision 5a.2 §4.7, W2 force-radix scaling and the
//! kernel half of formation-time scaling; ROOT's K2b rulings of 2026-09-28):
//! the force scale, exact scaling at formation, the b-rule's census and
//! window, the ledger's force-scaled terms, the sparse assembly option, and
//! the unscaling outcomes. The adapter-level tests (entries, the orchestrator,
//! the formation-range cases, the interactions) are in `nonlinear_integration`
//! (`structural_adapter/k2b_tests.rs`).
//!
//! Invented inputs only; every value is stated here. The K2a product-reach
//! member is PP `tests/k2a_formation_range_runtime.rs`'s `EXACT_ZERO`, with
//! its section formed as PP `derive_pipe_section` forms it; its expected
//! census (-1079, -333, b = 734) is the independent generator's
//! (`IMPLEMENTATION/K2B/_run_records/k2b_models.py.txt`).
use open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator;
use open_pipe_stress_frame_kernel::load_ledger::{
    AssembledForce, ForceTerm, ForceTermKind, Formation, LoadLedger,
};
use open_pipe_stress_frame_kernel::structural::{
    assemble_sparse_stiffness, solve_assembled_structural_dense, unscale_descriptive,
    unscale_for_publication, unscale_structural_error, unscale_structural_solution,
    ContributionRounding, ForceScaleReason, ForceScaledError, ForceScalingRefusal, PublishedValue,
    RecordRepresentability, Representability, ResidualRow, SparseAssemblyOptions, SparseStiffness,
    StiffnessBlock, StructuralError, StructuralSystem,
};
use open_pipe_stress_frame_kernel::{
    force_scaled_matrix, force_scaled_value, ForceScale, ForceScaleCensus, FrameElement,
    FrameKernelError, FrameNode, FrameSection, Matrix12, UserStiffnessElement,
};
use std::f64::consts::PI;

fn pow2(k: i32) -> f64 {
    // Exact for -1074 <= k <= 1023 (two steps keep every factor exact).
    let half = k / 2;
    2f64.powi(half) * 2f64.powi(k - half)
}

fn scale(b: i32) -> ForceScale {
    ForceScale::new(b).expect("even b")
}

/// The section as PP `derive_pipe_section` forms it (as K2a's tests do).
fn product_section(e: f64, g: f64, od: f64, t: f64) -> FrameSection {
    let id = od - 2.0 * t;
    let area = PI * (od.powi(2) - id.powi(2)) / 4.0;
    let second_moment = PI * (od.powi(4) - id.powi(4)) / 64.0;
    FrameSection::new(
        e,
        g,
        area,
        second_moment,
        second_moment,
        2.0 * second_moment,
    )
    .unwrap()
}

fn x_member(section: FrameSection, length: f64) -> FrameElement {
    FrameElement::new(
        FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
        FrameNode::new(1, [length, 0.0, 0.0]).unwrap(),
        section,
        [0.0, 1.0, 0.0],
    )
    .unwrap()
}

/// K2a's `EXACT_ZERO` product-reach member (L = 2^-39 m, OD 1e-11 m, wall
/// 1e-12 m, E 6.4e-280 Pa, G 1e-100 Pa), with its spring and load.
const REACH_ZERO_SPRING: f64 = 3.7e-289;
const REACH_ZERO_LOAD: f64 = 9.25e-290;
fn reach_zero_member() -> FrameElement {
    x_member(
        product_section(6.4e-280, 1.0e-100, 1.0e-11, 1.0e-12),
        1.8189894035458565e-12,
    )
}

fn census_of(exponents: &[i32]) -> ForceScaleCensus {
    let mut census = ForceScaleCensus::new();
    for &e in exponents {
        census.spring(pow2(e));
    }
    census
}

fn window(e_min: i32, e_max: i32) -> ForceScaleReason {
    ForceScaleReason::InfeasibleWindow { e_min, e_max }
}

// ------------------------------------------------------------------ the scale

#[test]
fn k2b_force_scale_is_even_only() {
    assert_eq!(ForceScale::new(0), Some(ForceScale::UNSCALED));
    assert_eq!(ForceScale::default(), ForceScale::UNSCALED);
    for b in [-1024, -2, 2, 734, 2046] {
        assert_eq!(ForceScale::new(b).unwrap().exponent(), b);
    }
    for b in [-1023, -3, -1, 1, 3, 735] {
        assert_eq!(ForceScale::new(b), None, "{b}");
    }
    assert!(ForceScale::UNSCALED.is_unscaled());
    assert!(!scale(2).is_unscaled());
}

#[test]
fn k2b_force_scaled_value_is_exact_and_refuses_what_cannot_stay_normal() {
    let name = "x";
    // Exact: every normal-to-normal power-of-two step, at both ends.
    for (value, b) in [
        (1.5, 1022),
        (1.5, -1022),
        (f64::MAX, -2044),
        (f64::MIN_POSITIVE, 2044),
        (-0.1, 700),
        (3.0e-300, 1000),
    ] {
        let scaled = force_scaled_value(name, value, scale(b)).unwrap();
        let mut exact = ExactAccumulator::new();
        exact.add(value).unwrap();
        assert_eq!(scaled, exact.round_scaled(b).unwrap(), "{value} {b}");
        assert!(scaled.is_normal());
    }
    // A zero keeps its sign; UNSCALED returns the value, even a subnormal.
    assert_eq!(
        force_scaled_value(name, -0.0, scale(64)).unwrap().to_bits(),
        (-0.0f64).to_bits()
    );
    assert_eq!(
        force_scaled_value(name, 5e-324, ForceScale::UNSCALED).unwrap(),
        5e-324
    );
    let range = Err(FrameKernelError::NumericalRange { name });
    // A subnormal input, or a result leaving the normal range, is refused.
    assert_eq!(force_scaled_value(name, 5e-324, scale(1100)), range);
    assert_eq!(force_scaled_value(name, 1.0, scale(1024)), range);
    assert_eq!(force_scaled_value(name, 1.0, scale(-1024)), range);
    assert_eq!(force_scaled_value(name, 1.0, scale(4000)), range);
    assert!(matches!(
        force_scaled_value(name, f64::INFINITY, scale(2)),
        Err(FrameKernelError::NonFiniteInput { .. })
    ));
    // A matrix, entry by entry, with the first failing entry's name.
    let mut m: Matrix12 = [[0.0; 12]; 12];
    m[0][0] = 3.0;
    m[5][7] = -0.25;
    let scaled = force_scaled_matrix("m", &m, scale(-8)).unwrap();
    assert_eq!(scaled[0][0], 3.0 / 256.0);
    assert_eq!(scaled[5][7], -0.25 / 256.0);
    assert_eq!(scaled[1][1].to_bits(), 0);
    m[3][3] = 1.0e-310;
    assert_eq!(
        force_scaled_matrix("m", &m, scale(2)),
        Err(FrameKernelError::NumericalRange { name: "m" })
    );
}

#[test]
fn k2b_elements_scale_e_g_and_user_stiffnesses_only() {
    let member = x_member(product_section(2.0e11, 8.0e10, 0.2, 0.01), 3.0);
    let scaled = member.force_scaled(scale(-40)).unwrap();
    assert_eq!(scaled.section.elastic_modulus, 2.0e11 * pow2(-40));
    assert_eq!(scaled.section.shear_modulus, 8.0e10 * pow2(-40));
    assert_eq!(scaled.section.area, member.section.area);
    assert_eq!(
        scaled.section.torsion_constant,
        member.section.torsion_constant
    );
    assert_eq!(scaled.node_j, member.node_j);
    assert_eq!(member.force_scaled(ForceScale::UNSCALED).unwrap(), member);
    // The scaled formation is exactly 2^b times today's (normal range).
    let local = member.global_stiffness().unwrap();
    let local_scaled = scaled.global_stiffness().unwrap();
    for (row, row_scaled) in local.iter().zip(&local_scaled) {
        for (&v, &w) in row.iter().zip(row_scaled) {
            assert_eq!(w.to_bits(), (v * pow2(-40)).to_bits());
        }
    }
    assert_eq!(
        member.force_scaled(scale(1024)),
        Err(FrameKernelError::NumericalRange {
            name: "E*2^b (force scale)"
        })
    );
    let user = UserStiffnessElement::new(
        FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
        FrameNode::new(1, [1.0, 0.0, 0.0]).unwrap(),
        [0.0, 1.0, 0.0],
        1.0e6,
        2.0e6,
        3.0e6,
        4.0e6,
    )
    .unwrap();
    let u = user.force_scaled(scale(10)).unwrap();
    assert_eq!(
        [
            u.axial_stiffness,
            u.lateral_stiffness,
            u.angular_stiffness,
            u.torsional_stiffness
        ],
        [
            1.0e6 * 1024.0,
            2.0e6 * 1024.0,
            3.0e6 * 1024.0,
            4.0e6 * 1024.0
        ]
    );
}

// ------------------------------------------------------------------ the b-rule

/// Steps 2-3: b_lo = -1022 + 64 - e_min, b_hi = 1023 - 8 - e_max,
/// m = floor((b_lo + b_hi)/2) toward -inf, then the even b of ROOT's K2b
/// ruling 1. Each row pins one branch; the comments give the arithmetic.
#[test]
fn k2b_the_window_the_floor_and_the_parity_rule() {
    // (exponents, expected b or refusal)
    let rows: &[(&[i32], Result<i32, ForceScaleReason>)] = &[
        // [-958, 1015]: sum 57, m = 28 (even).
        (&[0], Ok(28)),
        // [-958, 1013]: sum 55, m = 27 (odd), m - 1 = 26 >= b_lo.
        (&[0, 2], Ok(26)),
        // [-1057, 916]: sum -141, m = floor(-70.5) = -71 (odd) -> -72;
        // truncation toward zero would give -70.
        (&[99], Ok(-72)),
        // [5, 6]: m = 5 (odd), m - 1 = 4 < b_lo, m + 1 = 6 <= b_hi.
        (&[-963, 1009], Ok(6)),
        // [5, 5]: a single odd point, refused with the window reason.
        (&[-963, 1010], Err(window(-963, 1010))),
        // [5, 3]: infeasible.
        (&[-963, 1012], Err(window(-963, 1012))),
        // [4, 4]: a single even point: b = 4, so e_min + b = -958 exactly
        // (the 64-bit lower margin) and e_max + b = 1015 (the 8-bit upper).
        (&[-962, 1011], Ok(4)),
        // [-1968, 5]: m = -982 (even).
        (&[1010, 1010], Ok(-982)),
    ];
    for (exponents, expected) in rows {
        let census = census_of(exponents);
        let got = census.force_scale().map(ForceScale::exponent);
        assert_eq!(&got, expected, "{exponents:?}");
    }
    // No nonzero value: nothing to scale.
    assert_eq!(
        ForceScaleCensus::new().force_scale(),
        Ok(ForceScale::UNSCALED)
    );
    let mut zeros = ForceScaleCensus::new();
    zeros.spring(0.0);
    zeros.spring(-0.0);
    assert_eq!(zeros.span(), None);
}

#[test]
fn k2b_the_refusal_texts_are_the_designs() {
    assert_eq!(
        window(-963, 1012).to_string(),
        "range: exponent span [-963, 1012] exceeds the binary64 normal window after exact power-of-two scaling"
    );
    assert_eq!(
        ForceScaleReason::SubnormalAtFormation.to_string(),
        "range: subnormal stiffness or load at formation"
    );
    assert_eq!(
        ForceScaleReason::ScaledEvaluation.to_string(),
        "range: scaled evaluation outside normal range"
    );
    assert_eq!(
        ForceScaleReason::PublicationOutsideBinary64 {
            global_dof: Some(3)
        }
        .to_string(),
        "range: publication outside binary64"
    );
    let refusal = ForceScaledError::Refused(ForceScalingRefusal {
        reason: ForceScaleReason::ScaledEvaluation,
        trigger: None,
    });
    assert_eq!(
        refusal.to_string(),
        "range: scaled evaluation outside normal range"
    );
}

#[test]
fn k2b_the_census_refuses_every_kind_of_subnormal_input() {
    let refused = Err(ForceScaleReason::SubnormalAtFormation);
    let normal = || census_of(&[0]);
    let mut c = normal();
    c.spring(5e-324);
    assert_eq!(c.force_scale(), refused);
    let mut c = normal();
    c.load_term(&ForceTerm {
        source: "t".into(),
        dof: 0,
        kind: ForceTermKind::Term(-1.0e-310),
    });
    assert_eq!(c.force_scale(), refused);
    let mut c = normal();
    c.load_term(&ForceTerm {
        source: "p".into(),
        dof: 0,
        kind: ForceTermKind::Product(1.0e300, 1.0e-310),
    });
    assert_eq!(c.force_scale(), refused);
    let mut c = normal();
    let mut m: Matrix12 = [[0.0; 12]; 12];
    m[2][2] = 2.0e-310;
    c.matrix(&m);
    assert_eq!(c.force_scale(), refused);
    let mut c = normal();
    let user = UserStiffnessElement {
        node_i: FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
        node_j: FrameNode::new(1, [1.0, 0.0, 0.0]).unwrap(),
        y_reference: [0.0, 1.0, 0.0],
        axial_stiffness: 1.0,
        lateral_stiffness: 0.0,
        angular_stiffness: 1.0e-310,
        torsional_stiffness: 1.0,
    };
    c.user(&user);
    assert_eq!(c.force_scale(), refused);
    // A frame operand that does not scale with b (I here) is still refused,
    // and so is a subnormal E.
    for section in [
        FrameSection::new(2.0e11, 8.0e10, 0.01, 1.0e-310, 1.0e-6, 2.0e-6).unwrap(),
        FrameSection::new(1.0e-310, 8.0e10, 0.01, 1.0e-6, 1.0e-6, 2.0e-6).unwrap(),
    ] {
        let mut c = normal();
        c.frame(&x_member(section, 2.0)).unwrap();
        assert!(c.has_subnormal());
        assert_eq!(c.force_scale(), refused);
    }
    // A zero load term or product factor takes no part.
    let mut c = normal();
    c.load_term(&ForceTerm {
        source: "z".into(),
        dof: 0,
        kind: ForceTermKind::Product(0.0, 1.0e-310),
    });
    assert_eq!(c.force_scale(), Ok(scale(28)));
}

/// The frame census uses the predicted exponents (the sum of the operand
/// exponents), never the formed coefficients. For K2a's `EXACT_ZERO` member
/// the formed (unchecked) 12EI/L^3, 6EI/L^2, 4EI/L and 2EI/L are exactly 0
/// and take no part, so a census of formed coefficients gives another b
/// (precondition); the predicted census gives the generator's (-1079, -333)
/// and b = 734.
#[test]
fn k2b_the_frame_census_uses_predicted_exponents_not_formed_coefficients() {
    let member = reach_zero_member();
    let s = member.section;
    let l = member.length().unwrap();
    let load = ForceTerm {
        source: "load".into(),
        dof: 7,
        kind: ForceTermKind::Term(REACH_ZERO_LOAD),
    };
    let mut census = ForceScaleCensus::new();
    census.frame(&member).unwrap();
    census.spring(REACH_ZERO_SPRING);
    census.load_term(&load);
    assert_eq!(census.span(), Some((-1079, -333)));
    assert_eq!(census.force_scale(), Ok(scale(734)));
    // Precondition: the formed (unchecked, as before K2a) coefficients.
    let (e, g, i) = (s.elastic_modulus, s.shear_modulus, s.second_moment_y);
    let formed = [
        e * s.area / l,
        g * s.torsion_constant / l,
        12.0 * e * i / (l * l * l),
        6.0 * e * i / (l * l),
        4.0 * e * i / l,
        2.0 * e * i / l,
    ];
    assert_eq!(formed[2], 0.0, "12EI/L^3 rounds to 0 unchecked");
    let mut by_formed = census_of(&[]);
    by_formed.spring(e);
    by_formed.spring(g);
    for value in formed {
        by_formed.spring(value);
    }
    by_formed.spring(REACH_ZERO_SPRING);
    by_formed.load_term(&load);
    let formed_b = by_formed.force_scale().unwrap();
    assert_ne!(
        formed_b,
        scale(734),
        "precondition: the formed census differs"
    );
    // Formation at b = 734 passes K2a's checks, and 12EIz/L^3 is 2^734 times
    // the exact coefficient to within binary64 rounding of the chain.
    let scaled = member.force_scaled(scale(734)).unwrap();
    let k = scaled.local_stiffness().unwrap();
    let exact = 12.0 * (e * pow2(734)) * i / (l * l * l);
    assert!(((k[1][1] - exact) / exact).abs() < 1e-15);
    assert_eq!(
        member.local_stiffness(),
        Err(FrameKernelError::NumericalRange {
            name: "12EIy/L^3: (12*E)*Iy"
        })
    );
}

// ------------------------------------------------------------------ the ledger

fn net_bits(force: &AssembledForce) -> Vec<u64> {
    force.values().iter().map(|v| v.to_bits()).collect()
}

#[test]
fn k2b_the_ledger_force_scaled_is_exact_term_by_term_and_rounds_each_net_once() {
    let mut ledger = LoadLedger::new();
    ledger.push("a", 0, 3.0);
    ledger.push("a2", 0, 2f64.powi(-60));
    ledger.push("a3", 0, 2f64.powi(-60));
    ledger.push("b", 1, -1.0e-300);
    ledger.push_product("c", 2, pow2(1000), pow2(-1000));
    ledger.push_product("d", 3, pow2(300), pow2(300));
    ledger.push("zero", 4, -0.0);
    ledger.push_product("zero-factor", 4, 0.0, 7.0);
    ledger.push_product("under", 5, pow2(-600), pow2(-600));
    let force = ledger.finish(6).unwrap();
    assert_eq!(force.evidence().underflowed_dofs, vec![5]);

    // b = +400: every term scales in its first factor, or in its second.
    let up = force.force_scaled(scale(400)).unwrap();
    let kinds: Vec<ForceTermKind> = up.terms().iter().map(|t| t.kind).collect();
    assert_eq!(kinds[0], ForceTermKind::Term(3.0 * pow2(400)));
    assert_eq!(kinds[3], ForceTermKind::Term(-1.0e-300 * pow2(400)));
    assert_eq!(kinds[4], ForceTermKind::Product(pow2(1000), pow2(-600)));
    assert_eq!(kinds[5], ForceTermKind::Product(pow2(700), pow2(300)));
    assert_eq!(kinds[6], ForceTermKind::Term(-0.0));
    assert_eq!(kinds[7], ForceTermKind::Product(0.0, 7.0));
    assert_eq!(kinds[8], ForceTermKind::Product(pow2(-200), pow2(-600)));
    // Each net is the exact sum at scale, rounded once: 2^b times today's
    // where both are normal, and now normal where today's underflowed.
    for dof in 0..5 {
        assert_eq!(
            up.values()[dof].to_bits(),
            (force.values()[dof] * pow2(400)).to_bits(),
            "{dof}"
        );
    }
    assert_eq!(up.values()[5], pow2(-800));
    assert!(up.evidence().underflowed_dofs.is_empty());
    for (term, scaled) in force.terms().iter().zip(up.terms()) {
        assert_eq!((&term.source, term.dof), (&scaled.source, scaled.dof));
    }

    // b = -2000: 2^500 * 2^500 splits b across both factors (neither alone
    // stays normal); the product is exactly 2^-1000.
    let mut ledger = LoadLedger::new();
    ledger.push_product("split", 0, pow2(500), pow2(500));
    let force = ledger.finish(1).unwrap();
    let down = force.force_scaled(scale(-2000)).unwrap();
    assert_eq!(
        down.terms()[0].kind,
        ForceTermKind::Product(pow2(-1022), pow2(22))
    );
    assert_eq!(down.values()[0], pow2(-1000));

    // A term that cannot stay normal: the scaled evaluation is refused.
    let mut ledger = LoadLedger::new();
    ledger.push("big", 0, pow2(1000));
    assert_eq!(
        ledger.finish(1).unwrap().force_scaled(scale(100)).err(),
        Some(ForceScaleReason::ScaledEvaluation)
    );

    // UNSCALED gives the same values and terms.
    let mut ledger = LoadLedger::new();
    ledger.push("a", 0, 1.25);
    let force = ledger.finish(2).unwrap();
    let same = force.force_scaled(ForceScale::UNSCALED).unwrap();
    assert_eq!(net_bits(&same), net_bits(&force));
    assert_eq!(same.terms(), force.terms());
}

/// ROOT's K2b ruling 4: the S11-G formation records are not carried, and the
/// kernel never reads them: no kernel source outside `load_ledger.rs` names
/// `formation_rows` or `has_formation_records` (scan), and a force with
/// records solves exactly as the same force without (the adapter test
/// `k2b_the_kernel_reads_no_s11g_formation_record`).
#[test]
fn k2b_the_s11g_records_are_dropped_and_unread_by_the_kernel() {
    let mut ledger = LoadLedger::new();
    ledger.push_formed(
        "formed",
        0,
        1.5,
        Formation::Bounded { bound: 1.0e-20 },
        1.0e-20,
        false,
    );
    let force = ledger.finish(1).unwrap();
    assert!(force.has_formation_records());
    let scaled = force.force_scaled(scale(2)).unwrap();
    assert!(!scaled.has_formation_records());
    assert_eq!(scaled.values(), &[6.0]);
    let sources = [
        ("FK/lib.rs", include_str!("../src/lib.rs")),
        ("FK/structural.rs", include_str!("../src/structural.rs")),
        (
            "FK/structural/sparse.rs",
            include_str!("../src/structural/sparse.rs"),
        ),
        (
            "FK/structural/formation_check.rs",
            include_str!("../src/structural/formation_check.rs"),
        ),
        (
            "FK/structural/exact_boundary.rs",
            include_str!("../src/structural/exact_boundary.rs"),
        ),
        (
            "SA/structural_adapter.rs",
            include_str!("../../nonlinear_integration/src/structural_adapter.rs"),
        ),
        (
            "sparse_direct/structural.rs",
            include_str!("../../sparse_direct/src/structural.rs"),
        ),
    ];
    for (name, text) in sources {
        for token in ["formation_rows(", "has_formation_records("] {
            assert!(!text.contains(token), "{name} reads {token}");
        }
    }
}

// ------------------------------------------------------------------ assembly

fn assembly_inputs() -> (
    Vec<FrameElement>,
    Vec<UserStiffnessElement>,
    Vec<StiffnessBlock>,
    Vec<(usize, f64)>,
) {
    let node = |i: usize, p: [f64; 3]| FrameNode::new(i, p).unwrap();
    let section = product_section(2.0e11, 8.0e10, 0.2, 0.01);
    let frames = vec![
        FrameElement::new(
            node(0, [0.0; 3]),
            node(1, [2.0, 0.0, 0.0]),
            section,
            [0.0, 1.0, 0.0],
        )
        .unwrap(),
        FrameElement::new(
            node(1, [2.0, 0.0, 0.0]),
            node(2, [3.0, 1.5, 0.5]),
            section,
            [0.0, 0.0, 1.0],
        )
        .unwrap(),
    ];
    let users = vec![UserStiffnessElement::new(
        node(2, [3.0, 1.5, 0.5]),
        node(3, [3.0, 2.5, 0.5]),
        [1.0, 0.0, 0.0],
        1.0e8,
        2.0e7,
        3.0e6,
        4.0e6,
    )
    .unwrap()];
    let mut block = [[0.0; 12]; 12];
    for (k, row) in block.iter_mut().enumerate() {
        row[k] = 1.0e7 + k as f64;
    }
    let blocks = vec![StiffnessBlock {
        node_i: 0,
        node_j: 3,
        stiffness: block,
    }];
    (frames, users, blocks, vec![(5, 3.0e5), (13, 7.0e4)])
}

#[test]
fn k2b_the_sparse_assembly_at_b_is_the_exactly_scaled_assembly() {
    let (frames, users, blocks, springs) = assembly_inputs();
    let base = assemble_sparse_stiffness(
        4,
        &frames,
        &users,
        &blocks,
        &springs,
        &SparseAssemblyOptions::new(),
    )
    .unwrap();
    let same = assemble_sparse_stiffness(
        4,
        &frames,
        &users,
        &blocks,
        &springs,
        &SparseAssemblyOptions::new().with_force_scale(ForceScale::UNSCALED),
    )
    .unwrap();
    assert_eq!(same, base);
    assert_eq!(
        SparseAssemblyOptions::default(),
        SparseAssemblyOptions::new()
    );
    for b in [-400, -64, 64, 400] {
        let options = SparseAssemblyOptions::new().with_force_scale(scale(b));
        assert_eq!(options.force_scale(), scale(b));
        let scaled =
            assemble_sparse_stiffness(4, &frames, &users, &blocks, &springs, &options).unwrap();
        assert_eq!(scaled.pattern(), base.pattern());
        for (&v, &w) in base.values().iter().zip(scaled.values()) {
            assert_eq!(w.to_bits(), (v * pow2(b)).to_bits(), "b = {b}");
        }
    }
    // A formation-range member: refused at b = 0 (K2a), formed at its b.
    let member = reach_zero_member();
    let refused = assemble_sparse_stiffness(
        2,
        &[member],
        &[],
        &[],
        &[(7, REACH_ZERO_SPRING)],
        &SparseAssemblyOptions::new(),
    );
    assert_eq!(
        refused,
        Err(FrameKernelError::NumericalRange {
            name: "12EIy/L^3: (12*E)*Iy"
        })
    );
    let formed = assemble_sparse_stiffness(
        2,
        &[member],
        &[],
        &[],
        &[(7, REACH_ZERO_SPRING)],
        &SparseAssemblyOptions::new().with_force_scale(scale(734)),
    )
    .unwrap();
    assert!(formed.values().iter().all(|v| *v == 0.0 || v.is_normal()));
    // Scaling that leaves the normal range is refused by the scaled name.
    assert_eq!(
        assemble_sparse_stiffness(
            4,
            &frames,
            &users,
            &blocks,
            &springs,
            &SparseAssemblyOptions::new().with_force_scale(scale(1000)),
        ),
        Err(FrameKernelError::NumericalRange {
            name: "E*2^b (force scale)"
        })
    );
}

// ------------------------------------------------------------------ publication

fn published(value: f64) -> PublishedValue {
    PublishedValue {
        value,
        representability: Representability::Normal,
    }
}

#[test]
fn k2b_the_publication_outcomes() {
    // Normal: exact.
    assert_eq!(
        unscale_for_publication(3.0, scale(2), None),
        Ok(published(0.75))
    );
    assert_eq!(
        unscale_for_publication(pow2(900) * 1.1, scale(900), Some(4)),
        Ok(published(1.1))
    );
    // A zero keeps its sign.
    let zero = unscale_for_publication(-0.0, scale(600), None).unwrap();
    assert_eq!(zero.value.to_bits(), (-0.0f64).to_bits());
    // Subnormal: published with its stated precision 2^-1075/|v|, rounded up.
    let sub = unscale_for_publication(pow2(-500), scale(560), Some(1)).unwrap();
    assert_eq!(sub.value, 2f64.powi(-530) * 2f64.powi(-530));
    assert_eq!(
        sub.representability,
        Representability::Subnormal {
            relative_precision: pow2(-15)
        }
    );
    let three = unscale_for_publication(3.0 * pow2(-500), scale(560), None).unwrap();
    let Representability::Subnormal { relative_precision } = three.representability else {
        panic!("subnormal expected");
    };
    // 2^-1075 / (3 * 2^-1060) = 2^-15/3, rounded upward.
    assert!(relative_precision >= pow2(-15) / 3.0);
    assert!(relative_precision * 3.0 >= pow2(-15));
    assert_eq!(relative_precision, (pow2(-15) / 3.0).next_up());
    // Underflow and overflow: refused with the DOF, never flushed.
    let outside = Err(ForceScaleReason::PublicationOutsideBinary64 {
        global_dof: Some(9),
    });
    assert_eq!(
        unscale_for_publication(pow2(-600), scale(600), Some(9)),
        outside
    );
    assert_eq!(
        unscale_for_publication(pow2(600), scale(-600), Some(9)),
        outside
    );
    // Descriptive fields: the same single rounding, never refused.
    assert_eq!(unscale_descriptive(pow2(-600), scale(600)), 0.0);
    assert_eq!(
        unscale_descriptive(-pow2(-600), scale(600)).to_bits(),
        (-0.0f64).to_bits()
    );
    assert_eq!(unscale_descriptive(pow2(600), scale(-600)), f64::INFINITY);
    assert_eq!(unscale_descriptive(pow2(-500), scale(560)), pow2(-1060));
    // One rounding, never two (checkpoint C): the exact scaled value is
    // rounded once at 2^-b. Stepwise power-of-two multiplication (an
    // intermediate subnormal, then a second rounding) would give 0xff90c for
    // the first value and 0 (a flushed least subnormal) for the second. The
    // expected bits are an independent exact-rational rounding of each value
    // (the K2b run record's double-rounding search).
    let once = f64::from_bits(0x1eaf_f219_0006_b700);
    let least = f64::from_bits(0x1d40_0563_f122_86dd);
    for (value, b, bits) in [(once, 522, 0xf_f90d_u64), (least, 520, 1)] {
        let published = unscale_for_publication(value, scale(b), None).unwrap();
        assert_eq!(published.value.to_bits(), bits, "{value:e} at b = {b}");
        assert!(matches!(
            published.representability,
            Representability::Subnormal { .. }
        ));
        assert_eq!(unscale_descriptive(value, scale(b)).to_bits(), bits);
    }
}

#[test]
fn k2b_force_scaled_reactions_unscale_with_the_outcomes() {
    let k = SparseStiffness::from_dense(&[vec![1.0]]).unwrap();
    let force = LoadLedger::new().finish(1).unwrap();
    let react = |u: f64, b: i32| k.force_scaled_reactions(&[u], &force, scale(b), &[0]);
    assert_eq!(react(3.0, 2), Ok(vec![published(0.75)]));
    assert_eq!(
        react(pow2(-500), 560).unwrap()[0].representability,
        Representability::Subnormal {
            relative_precision: pow2(-15)
        }
    );
    let outside = Err(ForceScaledError::Refused(ForceScalingRefusal {
        reason: ForceScaleReason::PublicationOutsideBinary64 {
            global_dof: Some(0),
        },
        trigger: None,
    }));
    assert_eq!(react(pow2(-600), 600), outside);
    assert_eq!(react(pow2(600), -600), outside);
    // An exact zero is +0.0, as `reactions` gives it.
    assert_eq!(react(0.0, 400), Ok(vec![published(0.0)]));
    // With a load: the terms are taken at 2^b (force passed unscaled).
    let mut ledger = LoadLedger::new();
    ledger.push("p", 0, 0.5);
    let force = ledger.finish(1).unwrap();
    let k2 = SparseStiffness::from_dense(&[vec![2.0 * pow2(64)]]).unwrap();
    let r = k2
        .force_scaled_reactions(&[1.0], &force, scale(64), &[0])
        .unwrap();
    assert_eq!(r, vec![published(1.5)]);
    // Unscaled: the bits of `reactions`.
    let k3 = SparseStiffness::from_dense(&[vec![2.0]]).unwrap();
    assert_eq!(
        k3.force_scaled_reactions(&[1.0], &force, ForceScale::UNSCALED, &[0])
            .unwrap()[0]
            .value
            .to_bits(),
        k3.reactions(&[1.0], &force).unwrap()[0].to_bits()
    );
}

/// A two-DOF system the gate solves exactly: K = diag(4, 16), f = (2, 8),
/// so the equilibrated matrix is the identity and u = (0.5, 0.5). The residual
/// rows then have exact zero residuals, so their published records are the
/// denominators and allowances.
fn exact_two_dof() -> (Vec<Vec<f64>>, AssembledForce) {
    let mut ledger = LoadLedger::new();
    ledger.push("p0", 0, 2.0);
    ledger.push("p1", 1, 8.0);
    (
        vec![vec![4.0, 0.0], vec![0.0, 16.0]],
        ledger.finish(2).unwrap(),
    )
}

/// ROOT's K2b checkpoint-A ruling B: unscaling a solution never refuses the
/// case. Each physical field of a residual row is re-formed from its
/// normalized value at the unscaled exponent, rounded once, and carries an
/// explicit outcome: normal (exact, unlisted), subnormal with its precision,
/// underflow (a zero of its sign) or overflow (an infinity of its sign). The
/// descriptive `contribution_rounding` fields are unscaled by the same single
/// rounding, with no outcome.
#[test]
fn k2b_unscaled_residual_records_carry_their_outcome_and_never_refuse() {
    let (k, force) = exact_two_dof();
    let system = StructuralSystem::assembled(&k, &force, &[0, 1], &[], None, None);
    let mut solution = solve_assembled_structural_dense(&system).unwrap();
    assert_eq!(solution.displacements, vec![0.5, 0.5]);
    // A descriptive record far below the residual records.
    solution
        .report
        .contribution_rounding
        .push(ContributionRounding {
            row: 0,
            col: 1,
            accumulated_high: 1.0,
            accumulated_low: pow2(-80),
            stored_difference_high: pow2(-80),
            stored_difference_low: 0.0,
            accumulated_expansion: vec![pow2(-80), 1.0],
            difference_expansion: vec![pow2(-80)],
        });
    // b = 0: unchanged.
    let same = unscale_structural_solution(solution.clone(), ForceScale::UNSCALED, &force);
    assert_eq!(same.solution, solution);
    assert!(same.records.is_empty());
    let exact = |normalized: f64, exponent: i32| {
        let mut sum = ExactAccumulator::new();
        sum.add(normalized).unwrap();
        sum.round_scaled(exponent)
    };
    let fields = |row: &ResidualRow| {
        [
            ("residual_rows.residual", row.normalized_residual),
            ("residual_rows.denominator", row.normalized_denominator),
            (
                "residual_rows.evaluation_allowance",
                row.normalized_evaluation_allowance,
            ),
        ]
    };
    // b = 1000 (subnormal allowances), 1100 (subnormal denominators,
    // underflowed allowances) and -1100 (overflowed fields). Every field is the
    // single rounding of its normalized value, listed unless normal, and the
    // case is never refused.
    let mut seen = Vec::new();
    for b in [1000, 1100, -1100] {
        let unscaled = unscale_structural_solution(solution.clone(), scale(b), &force);
        let report = &unscaled.solution.report;
        for (e, e0) in report
            .scale_exponents
            .iter()
            .zip(&solution.report.scale_exponents)
        {
            assert_eq!(*e, e0 + b / 2);
        }
        let mut listed = unscaled.records.iter();
        for (row, row0) in report
            .residual_rows
            .iter()
            .zip(&solution.report.residual_rows)
        {
            assert_eq!(row.row_scale_exponent, row0.row_scale_exponent - b);
            assert_eq!(row.guarded_ratio, row0.guarded_ratio);
            let published = [row.residual, row.denominator, row.evaluation_allowance];
            for ((name, normalized), value) in fields(row0).into_iter().zip(published) {
                let expected = match exact(normalized, row.row_scale_exponent) {
                    Ok(v) if v == 0.0 => 0.0_f64.copysign(normalized),
                    Ok(v) => v,
                    Err(_) => f64::INFINITY.copysign(normalized),
                };
                assert_eq!(value.to_bits(), expected.to_bits(), "b={b} {name}");
                if normalized == 0.0 || value.is_normal() {
                    continue;
                }
                let record = listed.next().expect("a listed outcome");
                assert_eq!((record.record, record.global_dof), (name, row.global_dof));
                assert_eq!(record.value.to_bits(), value.to_bits());
                let outcome = match record.representability {
                    RecordRepresentability::Subnormal { relative_precision } => {
                        assert!(value.is_subnormal());
                        // 2^-1075/|value| = 0.5/m for value = m * 2^-1074, rounded up.
                        let quanta = value.abs().to_bits() as f64;
                        assert!(relative_precision.mul_add(quanta, -0.5) >= 0.0);
                        assert!(relative_precision.next_down().mul_add(quanta, -0.5) < 0.0);
                        "subnormal"
                    }
                    RecordRepresentability::Underflow => {
                        assert_eq!(value, 0.0);
                        assert!(normalized != 0.0);
                        "underflow"
                    }
                    RecordRepresentability::Overflow => {
                        assert!(value.is_infinite());
                        "overflow"
                    }
                };
                seen.push((b, outcome));
            }
        }
        assert!(
            listed.next().is_none(),
            "b={b}: an unexpected listed record"
        );
    }
    // Not vacuous: each outcome occurs.
    for outcome in ["subnormal", "underflow", "overflow"] {
        assert!(seen.iter().any(|s| s.1 == outcome), "{outcome}: {seen:?}");
    }
    // The descriptive record at b = 1000: 1.0 -> 2^-1000, 2^-80 -> 0.
    let unscaled = unscale_structural_solution(solution.clone(), scale(1000), &force);
    let entry = &unscaled.solution.report.contribution_rounding[0];
    assert_eq!(entry.accumulated_high, pow2(-1000));
    assert_eq!(entry.accumulated_low, 0.0);
    assert_eq!(entry.difference_expansion, vec![0.0]);
    // A row with no nonzero term (no load, u = 0 on it) carries the exponent
    // sentinel 0 and zero records at every scale: f = (2, 0) gives u = (0.5, 0).
    let mut ledger = LoadLedger::new();
    ledger.push("p0", 0, 2.0);
    let force = ledger.finish(2).unwrap();
    let system = StructuralSystem::assembled(&k, &force, &[0, 1], &[], None, None);
    let solution = solve_assembled_structural_dense(&system).unwrap();
    let row = &solution.report.residual_rows[1];
    assert_eq!((row.global_dof, row.row_scale_exponent), (1, 0));
    assert_eq!(row.normalized_denominator, 0.0);
    let unscaled = unscale_structural_solution(solution.clone(), scale(1000), &force);
    assert_eq!(unscaled.solution.report.residual_rows[1], *row);
    assert!(unscaled.records.iter().all(|r| r.global_dof != 1));
}

#[test]
fn k2b_unscaled_error_payloads() {
    let diagonal = StructuralError::NegativeEnergy {
        direction: vec![0.0, 1.0],
        energy: -3.0 * pow2(40),
        allowance: 0.0,
    };
    assert_eq!(
        unscale_structural_error(diagonal.clone(), scale(40)),
        StructuralError::NegativeEnergy {
            direction: vec![0.0, 1.0],
            energy: -3.0,
            allowance: 0.0,
        }
    );
    let witness = StructuralError::NegativeEnergy {
        direction: vec![0.5 * pow2(-20), -pow2(-20)],
        energy: -0.25,
        allowance: 1.0e-14,
    };
    assert_eq!(
        unscale_structural_error(witness.clone(), scale(40)),
        StructuralError::NegativeEnergy {
            direction: vec![0.5, -1.0],
            energy: -0.25,
            allowance: 1.0e-14,
        }
    );
    assert_eq!(
        unscale_structural_error(witness.clone(), ForceScale::UNSCALED),
        witness
    );
    let other = StructuralError::Range("arithmetic outside normal range");
    assert_eq!(unscale_structural_error(other.clone(), scale(40)), other);
}

// ------------------------------------------------------------------ RV11 fixes
// (T3 `ROOT_RULINGS_V1.md`, "K2b: rulings on RV11's review (ROOT)"; RV11's
// review `REVIEW/K2B_REVIEW.md`, findings RV11-1, RV11-2 and RV11-4.)

/// RV11-2: a scaled value is accepted only when the **exact** result is
/// normal. RV11's probe F-B: (2 - 2^-52)*2^-1021 at b = -2 is exactly
/// (2 - 2^-52)*2^-1023, below 2^-1022; binary64 rounds it up to 2^-1022,
/// which is normal but inexact, so it is refused. Through the ledger, at the
/// rule's b of F-B (-706), the factor x = (2 - 2^-52)*2^-317 of a product
/// cannot be scaled exactly, so y = 2^760 is scaled instead (to 2^54), and the
/// scaled net is exactly 2^b times the net.
#[test]
fn k2b_rv11_scaling_refuses_a_result_that_rounds_up_into_the_normal_range() {
    let boundary = f64::from_bits(0x002F_FFFF_FFFF_FFFF);
    assert_eq!(boundary, (2.0 - pow2(-52)) * pow2(-1021));
    assert_eq!(
        force_scaled_value("x", boundary, scale(-2)),
        Err(FrameKernelError::NumericalRange { name: "x" })
    );
    // The exact neighbours: 2^-1020 and the largest mantissa at 2^-1020 land
    // exactly on normal values at b = -2.
    assert_eq!(
        force_scaled_value("x", pow2(-1020), scale(-2)),
        Ok(pow2(-1022))
    );
    let top = f64::from_bits(0x0030_0000_0000_0000 | 0x000F_FFFF_FFFF_FFFF);
    assert_eq!(
        force_scaled_value("x", top, scale(-2)).map(f64::to_bits),
        Ok(0x0010_0000_0000_0000 | 0x000F_FFFF_FFFF_FFFF)
    );
    let x = f64::from_bits((((-317_i64 + 1023) as u64) << 52) | 0x000F_FFFF_FFFF_FFFF);
    let y = pow2(760);
    let mut ledger = LoadLedger::new();
    ledger.push_product("p", 0, x, y);
    let force = ledger.finish(1).unwrap();
    let scaled = force.force_scaled(scale(-706)).unwrap();
    assert_eq!(scaled.terms()[0].kind, ForceTermKind::Product(x, pow2(54)));
    assert_eq!(
        scaled.values()[0].to_bits(),
        (force.values()[0] * pow2(-353) * pow2(-353)).to_bits()
    );
}

/// RV11-4: the census records a load product x*y at e(x) + e(y), whichever
/// factor carries the magnitude (ruling 2), so a product of factors with very
/// different exponents bounds the span at its own exponent.
#[test]
fn k2b_rv11_the_census_records_a_product_at_its_factors_exponent_sum() {
    let mut ledger = LoadLedger::new();
    ledger.push("t", 0, 1.0);
    ledger.push_product("p", 0, pow2(-500), pow2(400));
    ledger.push_product("q", 1, pow2(10), pow2(-900));
    let force = ledger.finish(2).unwrap();
    let mut census = ForceScaleCensus::new();
    for term in force.terms() {
        census.load_term(term);
    }
    assert_eq!(census.span(), Some((-890, 0)));
    let mut product = ForceScaleCensus::new();
    product.load_term(&force.terms()[1]);
    assert_eq!(product.span(), Some((-100, -100)));
}

/// A stiffness from a dense matrix (stored where nonzero), its reactions at
/// `b` for DOF 0, with no load.
fn react_row0(
    dense: &[Vec<f64>],
    u: &[f64],
    b: i32,
) -> Result<Vec<PublishedValue>, ForceScaledError> {
    let k = SparseStiffness::from_dense(dense).unwrap();
    let force = LoadLedger::new().finish(dense.len()).unwrap();
    k.force_scaled_reactions(u, &force, scale(b), &[0])
}

/// RV11-1 (BLOCKING), fixed fail-closed: `force_scaled_reactions` checks the
/// formed row K*u at 2^b. Each product of nonzero operands, and each partial
/// sum, must be normal (or the partial sum an exact zero); otherwise the
/// reaction is refused with step 5's `PublicationOutsideBinary64`. It is never
/// published as a flushed or truncated value labelled `Normal`. The check
/// holds at every b (b = 0 and b = 2 here, as the row is given at scale).
#[test]
fn k2b_rv11_force_scaled_reactions_check_every_product_and_partial_sum() {
    let refused = Err(ForceScaledError::Refused(ForceScalingRefusal {
        reason: ForceScaleReason::PublicationOutsideBinary64 {
            global_dof: Some(0),
        },
        trigger: None,
    }));
    let two = |k01: f64| vec![vec![1.0, k01], vec![k01, 1.0]];
    for b in [0, 2] {
        // A product that underflows to zero (the true reaction is 2^-1200).
        assert_eq!(react_row0(&two(pow2(-600)), &[0.0, pow2(-600)], b), refused);
        // A subnormal product (2^-1040), truncated in binary64.
        assert_eq!(react_row0(&two(pow2(-500)), &[0.0, pow2(-540)], b), refused);
        // A product that overflows.
        assert_eq!(react_row0(&two(pow2(600)), &[0.0, pow2(600)], b), refused);
        // Normal products whose partial sum is subnormal: 2^-1000 and
        // -(2^-1000 - 2^-1052) leave 2^-1052.
        let cancel = vec![
            vec![0.0, 1.0, -1.0],
            vec![1.0, 1.0, 0.0],
            vec![-1.0, 0.0, 1.0],
        ];
        let near = pow2(-1000) - pow2(-1052);
        assert_eq!(near, (2.0 - pow2(-51)) * pow2(-1001));
        assert_eq!(react_row0(&cancel, &[0.0, pow2(-1000), near], b), refused);
        // An exact zero partial sum, and a zero operand, are fine.
        let zero = react_row0(&cancel, &[0.0, pow2(-10), pow2(-10)], b).unwrap();
        assert_eq!(zero[0].value.to_bits(), 0.0f64.to_bits());
        assert_eq!(zero[0].representability, Representability::Normal);
        assert_eq!(
            react_row0(&two(pow2(-600)), &[0.0, 0.0], b),
            Ok(vec![published(0.0)])
        );
    }
    // Control: a normal row publishes the bits of `reactions` at b = 0, and
    // exactly 2^-b times the scaled row at b = 2.
    let k = SparseStiffness::from_dense(&two(3.0)).unwrap();
    let force = LoadLedger::new().finish(2).unwrap();
    let u = [0.0, 0.5];
    let today = k.reactions(&u, &force).unwrap()[0];
    let published0 = k
        .force_scaled_reactions(&u, &force, ForceScale::UNSCALED, &[0])
        .unwrap();
    assert_eq!(published0, vec![published(today)]);
    assert_eq!(react_row0(&two(3.0), &u, 2), Ok(vec![published(0.375)]));
}

/// RV11's long member W (probe F-A2): N0 -> N1 along x, L = 2^300 m,
/// E = G = 2^200 Pa, A = I = J = 1; the solve's rotation at N0 RZ.
fn long_member_w() -> FrameElement {
    let section = FrameSection::new(pow2(200), pow2(200), 1.0, 1.0, 1.0, 1.0).unwrap();
    FrameElement::new(
        FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
        FrameNode::new(1, [pow2(300), 0.0, 0.0]).unwrap(),
        section,
        [0.0, 1.0, 0.0],
    )
    .unwrap()
}

/// The straight pipe's formed elastic end actions at b = 0 (`+=` from +0.0,
/// T * u_e, then K_local * that), as the independent reference.
fn formed_end_actions(frame: &FrameElement, u: &[f64]) -> [f64; 12] {
    let k = frame.local_stiffness().unwrap();
    let t = frame.orientation().unwrap().transformation_matrix();
    let map =
        open_pipe_stress_frame_kernel::element_dof_map(frame.node_i.index, frame.node_j.index);
    let mut local = [0.0; 12];
    for row in 0..12 {
        for col in 0..12 {
            local[row] += t[row][col] * u[map[col]];
        }
    }
    let mut actions = [0.0; 12];
    for row in 0..12 {
        for col in 0..12 {
            actions[row] += k[row][col] * local[col];
        }
    }
    actions
}

/// RV11-1: the kernel's member end actions at 2^b
/// (`FrameElement::force_scaled_end_actions`), built as the reactions are:
/// every product and partial sum checked, then unscaled once. On RV11's long
/// member, the end shear 6EI/L^2 * theta is about 1.38e-300 N: normal at b = 0
/// and at b = 64 (bit for bit the straight pipe's formed value), but formed
/// below the normal range at the rule's b of F-A2 (-138), where it is
/// refused, as it is with a rotation 2^80 larger (a subnormal product).
#[test]
fn k2b_rv11_force_scaled_end_actions_are_checked_and_unscaled_once() {
    let w = long_member_w();
    let theta = f64::from_bits(0x1a83_c083_126e_978d);
    let mut u = vec![0.0; 12];
    u[5] = theta;
    let reference = formed_end_actions(&w, &u);
    assert!(reference[1].is_normal() && reference[1] > 1.3e-300 && reference[1] < 1.4e-300);
    for b in [0, 64] {
        let actions = w.force_scaled_end_actions(&u, scale(b)).unwrap();
        for (published, expected) in actions.iter().zip(reference) {
            assert_eq!(published.value.to_bits(), expected.to_bits(), "b = {b}");
            assert_eq!(published.representability, Representability::Normal);
        }
    }
    let refused = Err(ForceScaledError::Refused(ForceScalingRefusal {
        reason: ForceScaleReason::PublicationOutsideBinary64 { global_dof: None },
        trigger: None,
    }));
    assert_eq!(w.force_scaled_end_actions(&u, scale(-138)), refused);
    u[5] = theta * pow2(80);
    assert_eq!(w.force_scaled_end_actions(&u, scale(-138)), refused);
    assert_eq!(
        w.force_scaled_end_actions(&u[..6], ForceScale::UNSCALED),
        Err(ForceScaledError::Structural(StructuralError::InvalidInput(
            "displacement vector"
        )))
    );
}

// ------------------------------------------------------------------ RV11 delta fixes
// (T3 `ROOT_RULINGS_V1.md`, ROOT's rulings on RV11's delta check; RV11's
// review `REVIEW/K2B_REVIEW.md`, "Delta check at f385a8bc8", RV11D-1 and
// RV11D-2.)

/// RV11D-2: `force_scaled_end_actions` checks the local-displacement stage
/// T * u_e too, at every b, b = 0 included. A member along (3, 4, 0) m has
/// direction cosines 0.6 and 0.8, so a displacement of 2^-1022 m (the least
/// normal) at N1 UX gives subnormal products 0.6 * 2^-1022 and 0.8 * 2^-1022
/// in T * u_e. Those bits are lost before the stiffness stage, and with
/// E = G = 2^100 Pa the stiffness stage would lift them back to normal values
/// published as exact. So the actions are refused at b = 0 and at b = 64. With
/// a displacement of 2^-1000 m every stage is normal and the actions are
/// published, bit for bit the same at both scales.
#[test]
fn k2b_rv11d_end_actions_check_the_local_displacement_stage_at_every_b() {
    let section = FrameSection::new(pow2(100), pow2(100), 1.0, 1.0, 1.0, 1.0).unwrap();
    let frame = FrameElement::new(
        FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
        FrameNode::new(1, [3.0, 4.0, 0.0]).unwrap(),
        section,
        [0.0, 0.0, 1.0],
    )
    .unwrap();
    let t = frame.orientation().unwrap().transformation_matrix();
    let mut u = vec![0.0; 12];
    u[6] = pow2(-1022);
    // Precondition: a product of T * u_e is subnormal, and the stiffness stage
    // alone would not see it (its products of those values are normal).
    assert!((0..12).any(|r| {
        let product = t[r][6] * u[6];
        product != 0.0 && product.is_subnormal()
    }));
    let k = frame.local_stiffness().unwrap();
    assert!((k[0][6] * (t[6][6] * u[6])).is_normal());
    let refused = Err(ForceScaledError::Refused(ForceScalingRefusal {
        reason: ForceScaleReason::PublicationOutsideBinary64 { global_dof: None },
        trigger: None,
    }));
    for b in [0, 64] {
        assert_eq!(
            frame.force_scaled_end_actions(&u, scale(b)),
            refused,
            "b = {b}"
        );
    }
    u[6] = pow2(-1000);
    let at0 = frame
        .force_scaled_end_actions(&u, ForceScale::UNSCALED)
        .unwrap();
    let at64 = frame.force_scaled_end_actions(&u, scale(64)).unwrap();
    assert_eq!(at0, at64);
    assert!(at0
        .iter()
        .all(|p| p.representability == Representability::Normal));
}

/// RV11D-1: the kernel's spring action at 2^b
/// (`force_scaled_spring_action`), built as the member actions are. The
/// product -(k * 2^b) * u of nonzero operands must be normal, or the action is
/// refused with step 5's `PublicationOutsideBinary64`; it is never a flushed
/// value labelled `Normal`. It is then unscaled once. At b = 0 a published
/// value is today's -(k * u), bit for bit.
#[test]
fn k2b_rv11d_force_scaled_spring_action_checks_its_product_at_every_b() {
    use open_pipe_stress_frame_kernel::force_scaled_spring_action as action;
    let refused = |dof: usize| {
        Err(ForceScaledError::Refused(ForceScalingRefusal {
            reason: ForceScaleReason::PublicationOutsideBinary64 {
                global_dof: Some(dof),
            },
            trigger: None,
        }))
    };
    // RV11's F-S at the kernel: k = 2^-600 N/m, u about 2^-400 m. At b = 0 the
    // action is normal; at b = -138 the product is formed below 2^-1022.
    let k = pow2(-600);
    let u = [0.0, -1.2345 * pow2(-397)];
    let today = -(k * u[1]);
    assert!(today.is_normal());
    assert_eq!(
        action((1, k), &u, ForceScale::UNSCALED),
        Ok(published(today))
    );
    assert_eq!(action((1, k), &u, scale(-138)), refused(1));
    // A subnormal product (about 2^-1060) is refused too, at b = 0.
    let small = [0.0, pow2(-460)];
    assert_eq!(action((1, k), &small, ForceScale::UNSCALED), refused(1));
    // Normal at scale: exact, and the same bits as at b = 0.
    assert_eq!(action((1, k), &u, scale(64)), Ok(published(today)));
    // A zero displacement gives a zero action (its sign kept), not a refusal.
    let zero = action((1, k), &[0.0, 0.0], scale(-138)).unwrap();
    assert_eq!(zero.value, 0.0);
    assert_eq!(zero.representability, Representability::Normal);
    // Invalid input and an unscalable stiffness.
    assert_eq!(
        action((2, k), &u, ForceScale::UNSCALED),
        Err(ForceScaledError::Structural(StructuralError::InvalidInput(
            "displacement vector"
        )))
    );
    assert_eq!(
        action((1, pow2(1000)), &u, scale(100)),
        Err(ForceScaledError::Formation(
            FrameKernelError::NumericalRange {
                name: "spring stiffness*2^b (force scale)"
            }
        ))
    );
}

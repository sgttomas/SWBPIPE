//! K1 adapter tests (T3 D1 revision 5a.2 §4.8; the K1 row of §6): the sparse
//! `AssemblyEvidence` and the pattern-taking solve entries, against the dense
//! evidence and today's entries. Invented inputs only; every value is stated
//! here or in K-D5's generated `kd5_models.rs` (included read-only below).
//!
//! The dense-derived path is `AssemblyEvidence` on the dense view of the
//! same assembly (`SparseStiffness::to_dense`, itself asserted bit-identical
//! to the product's dense assembly order). Every comparison is of `Debug`
//! bytes or bits, never within a tolerance; the reference comparisons (N, T0R,
//! relabelling) use the protected 1e-9 relative criterion.
use super::kd5_tests::{bits, MemberData, ModelData, SectionData, MODES};
use super::*;
use crate::{
    solve_active_set_frame_with_mode_and_springs, ConvergenceControl, ConvergencePolicyStatus,
    NonlinearFrameSolveInput,
};
use open_pipe_stress_frame_kernel::load_ledger::LoadLedger;
use open_pipe_stress_frame_kernel::structural::{
    assemble_sparse_stiffness, FormationCheckReason, SolveQuality, SparseAssemblyOptions,
    StiffnessBlock,
};
use open_pipe_stress_frame_kernel::{
    assemble_global_stiffness_with_connectors, FrameDof, FrameNode, FrameSection,
};
use open_pipe_stress_nonlinear_supports::{
    ActiveSetState, GapDirection, NonlinearSupport, SupportStateRecord,
};

#[path = "kd5_models.rs"]
mod kd5_models;
use kd5_models::*;

// ------------------------------------------------------------------ builder

/// One model in both representations: the product's element families, the
/// sparse assembly and its dense view, both evidences, and the loads.
struct Case {
    node_count: usize,
    frames: Vec<FrameElement>,
    connectors: Vec<open_pipe_stress_frame_kernel::connector::ObjectiveConnector>,
    macros: Vec<CurvedBendMacroElement>,
    slots: Vec<CurvedBendStiffnessElement>,
    springs: Vec<(usize, f64)>,
    free: Vec<usize>,
    prescribed: Vec<(usize, f64)>,
    loads: Vec<(usize, f64)>,
}

impl Case {
    fn blocks(&self) -> Vec<StiffnessBlock> {
        self.slots
            .iter()
            .map(|s| StiffnessBlock {
                node_i: s.node_i,
                node_j: s.node_j,
                stiffness: s.global_stiffness,
            })
            .collect()
    }
    fn sparse(&self) -> SparseStiffness {
        assemble_sparse_stiffness(
            self.node_count,
            &self.frames,
            &self.connectors,
            &self.blocks(),
            &self.springs,
            &SparseAssemblyOptions::new(),
        )
        .unwrap()
    }
    /// The product's dense assembly order (`PP`: frames and connectors, then
    /// `add_curved_bend_stiffness_contributions`, then the springs).
    fn product_dense(&self) -> Vec<Vec<f64>> {
        let mut k = assemble_global_stiffness_with_connectors(
            self.node_count,
            &self.frames,
            &self.connectors,
        )
        .unwrap();
        for s in &self.slots {
            let map = element_dof_map(s.node_i, s.node_j);
            for (r, &row) in map.iter().enumerate() {
                for (c, &col) in map.iter().enumerate() {
                    k[row][col] += s.global_stiffness[r][c];
                }
            }
        }
        for &(dof, value) in &self.springs {
            k[dof][dof] += value;
        }
        k
    }
    fn dense_evidence(&self) -> AssemblyEvidence {
        AssemblyEvidence::new(
            self.node_count,
            &self.frames,
            &self.connectors,
            &self.slots,
            &self.springs,
        )
        .unwrap()
    }
    fn sparse_evidence(&self, k: &SparseStiffness) -> SparseAssemblyEvidence {
        SparseAssemblyEvidence::new(
            k.pattern(),
            self.node_count,
            &self.frames,
            &self.connectors,
            &self.slots,
            &self.springs,
        )
        .unwrap()
    }
    fn force(&self) -> Vec<f64> {
        let mut f = vec![0.0; 6 * self.node_count];
        for &(dof, v) in &self.loads {
            f[dof] += v;
        }
        f
    }
    fn ledger(&self) -> AssembledForce {
        let mut ledger = LoadLedger::new();
        for (index, &(dof, v)) in self.loads.iter().enumerate() {
            ledger.push(format!("load:{index}"), dof, v);
        }
        ledger.finish(6 * self.node_count).unwrap()
    }
    fn set_boundary(&mut self, prescribed: Vec<(usize, f64)>) {
        let n = 6 * self.node_count;
        self.free = (0..n)
            .filter(|d| !prescribed.iter().any(|p| p.0 == *d))
            .collect();
        self.prescribed = prescribed;
    }
    fn from_model(m: &ModelData) -> Self {
        let node = |i: usize| FrameNode::new(i, m.nodes[i]).unwrap();
        let s = &m.section;
        let section = FrameSection::new(s.e, s.g, s.a, s.i, s.i, s.j).unwrap();
        let mut case = Case {
            node_count: m.nodes.len(),
            frames: Vec::new(),
            connectors: Vec::new(),
            macros: Vec::new(),
            slots: Vec::new(),
            springs: m.springs.to_vec(),
            free: Vec::new(),
            prescribed: Vec::new(),
            loads: m.loads.iter().copied().filter(|l| l.1 != 0.0).collect(),
        };
        for (index, member) in m.members.iter().enumerate() {
            match member.bend {
                None => case.frames.push(
                    FrameElement::new(node(member.i), node(member.j), section, member.y_reference)
                        .unwrap(),
                ),
                Some((radius, y_reference, factor)) => {
                    let e = CurvedBendMacroElement::new(
                        node(member.i),
                        node(member.j),
                        radius,
                        y_reference,
                        s.e,
                        s.g,
                        s.a,
                        s.i,
                        s.j,
                        factor,
                        factor,
                    )
                    .unwrap();
                    case.slots.push(
                        CurvedBendStiffnessElement::from_macro_element(format!("bend-{index}"), &e)
                            .unwrap(),
                    );
                    case.macros.push(e);
                }
            }
        }
        case.set_boundary(m.rigid.iter().map(|&d| (d, 0.0)).collect());
        case
    }
}

const KD5: [&ModelData; 10] = [
    &F122,
    &F345,
    &PROBE_C,
    &PROBE_D,
    &BENDING_SOFT,
    &E1,
    &E6,
    &CSKEW_8_5,
    &M11,
    &PP_UTM_2,
];

fn render(result: &Result<StructuralSolution, StructuralError>) -> String {
    format!("{result:?}")
}

/// The dense entries on the dense view and the pattern entries, in `mode`.
fn both_plain(case: &Case, mode: LinearSolveMode) -> (String, String) {
    let k = case.sparse();
    let dense = k.to_dense();
    let f = case.ledger();
    let d = case
        .dense_evidence()
        .solve_assembled(&dense, &f, &case.free, &case.prescribed, mode);
    let s = case
        .sparse_evidence(&k)
        .solve_assembled(&k, &f, &case.free, &case.prescribed, mode);
    (render(&d), render(&s))
}

fn both_checked(
    case: &Case,
    mode: LinearSolveMode,
    macros: &[CurvedBendMacroElement],
) -> (
    Result<StructuralSolution, StructuralError>,
    Result<StructuralSolution, StructuralError>,
) {
    let k = case.sparse();
    let dense = k.to_dense();
    let f = case.ledger();
    let d = case.dense_evidence().solve_assembled_with_formation_check(
        &dense,
        &f,
        &case.free,
        &case.prescribed,
        mode,
        macros,
        true,
    );
    let s = case
        .sparse_evidence(&k)
        .solve_assembled_with_formation_check(
            &k,
            &f,
            &case.free,
            &case.prescribed,
            mode,
            macros,
            true,
        );
    (d, s)
}

// ------------------------------------------------------------------ evidence

#[test]
fn k1_sparse_evidence_is_the_dense_evidence_on_the_pattern() {
    for m in KD5 {
        let case = Case::from_model(m);
        let k = case.sparse();
        // The sparse assembly is the product's dense assembly, bit for bit.
        let product = case.product_dense();
        let view = k.to_dense();
        assert_eq!(
            product.iter().map(|r| bits(r)).collect::<Vec<_>>(),
            view.iter().map(|r| bits(r)).collect::<Vec<_>>(),
            "{}",
            m.name
        );
        let dense = case.dense_evidence();
        let sparse = case.sparse_evidence(&k);
        assert_eq!(
            format!("{:?}", dense.contributions),
            format!("{:?}", sparse.contributions()),
            "{}",
            m.name
        );
        let (roundoff, counts) = sparse.dense_symmetry_view();
        assert_eq!(
            dense
                .absolute_roundoff
                .iter()
                .map(|r| bits(r))
                .collect::<Vec<_>>(),
            roundoff.iter().map(|r| bits(r)).collect::<Vec<_>>(),
            "{}",
            m.name
        );
        assert_eq!(dense.operation_counts, counts, "{}", m.name);
        assert_eq!(
            dense.qualified_passive_family(),
            sparse.qualified_passive_family()
        );
        assert_eq!(
            sparse.storage_counts().contributions,
            dense.contributions.len()
        );
        assert_eq!(
            sparse.storage_counts().pattern_entries,
            k.pattern().entry_count()
        );
        // A stiffness on another pattern is refused, never solved.
        let other = Case::from_model(&F345).sparse();
        if other.pattern() != k.pattern() {
            assert!(sparse
                .solve_assembled(
                    &other,
                    &case.ledger(),
                    &case.free,
                    &case.prescribed,
                    LinearSolveMode::SparseInteractive
                )
                .is_err());
        }
    }
}

// ------------------------------------------------------------------ reports

#[test]
fn k1_pattern_entries_are_byte_identical_to_todays_entries_in_both_modes() {
    for m in KD5 {
        let case = Case::from_model(m);
        for mode in MODES {
            let (d, s) = both_plain(&case, mode);
            assert_eq!(d, s, "{} {mode:?} plain", m.name);
            let (d, s) = both_checked(&case, mode, &case.macros);
            assert_eq!(
                render(&d),
                render(&s),
                "{} {mode:?} formation-checked",
                m.name
            );
        }
    }
}

#[test]
fn k1_kd5_parity_cases_demote_identically_in_both_representations() {
    // P1's 122 (required true positive) and the conditioning-driven curved
    // true positive (F122 realized as a bend, R = 10 m; T4-U1 phase B, in
    // CSKEW_8_5's former place) demote in both representations and both
    // modes; R5-4's realistic elbows E1 and E6 do not; the records are
    // identical.
    for (m, demotes) in [
        (&F122, true),
        (&C122_R10_Y100, true),
        (&E1, false),
        (&E6, false),
    ] {
        let case = Case::from_model(m);
        for mode in MODES {
            let (d, s) = both_checked(&case, mode, &case.macros);
            let (d, s) = (d.unwrap(), s.unwrap());
            assert_eq!(format!("{d:?}"), format!("{s:?}"), "{} {mode:?}", m.name);
            assert_eq!(s.formation_check.is_some(), demotes, "{} {mode:?}", m.name);
            let expected = if demotes {
                SolveQuality::Sensitive
            } else {
                SolveQuality::Passed
            };
            assert_eq!(s.report.quality, expected, "{} {mode:?}", m.name);
            if demotes {
                assert_eq!(
                    s.formation_check.as_ref().unwrap().reason,
                    FormationCheckReason::Estimate
                );
            }
        }
    }
    // A formation_check_unavailable seed: no macro element for E1's slot.
    let case = Case::from_model(&E1);
    for mode in MODES {
        let (d, s) = both_checked(&case, mode, &[]);
        let (d, s) = (d.unwrap(), s.unwrap());
        assert_eq!(format!("{d:?}"), format!("{s:?}"), "E1 unmatched {mode:?}");
        match &s.formation_check.as_ref().expect("demoted").reason {
            FormationCheckReason::FormationCheckUnavailable { detail } => {
                assert!(detail.starts_with("curved_bend_source_unmatched:"))
            }
            other => panic!("expected formation_check_unavailable, got {other:?}"),
        }
    }
    // Not selected (a nonlinear support in the invocation): both run their
    // plain entry unchanged.
    let case = Case::from_model(&F122);
    let k = case.sparse();
    let dense = k.to_dense();
    for mode in MODES {
        let d = case.dense_evidence().solve_assembled_with_formation_check(
            &dense,
            &case.ledger(),
            &case.free,
            &case.prescribed,
            mode,
            &case.macros,
            false,
        );
        let s = case
            .sparse_evidence(&k)
            .solve_assembled_with_formation_check(
                &k,
                &case.ledger(),
                &case.free,
                &case.prescribed,
                mode,
                &case.macros,
                false,
            );
        assert_eq!(render(&d), render(&s));
        assert_eq!(s.unwrap().report.quality, SolveQuality::Passed);
    }
}

#[test]
fn k1_c3_detect_solve_is_identical_in_both_representations() {
    // The crate's `&[f64]` solve with S11-K's force terms: a cancelling
    // ledger (G, n, -G) at F122's tip, solved as its binary64 fold.
    let case = Case::from_model(&F122);
    let k = case.sparse();
    let dense = k.to_dense();
    let mut ledger = LoadLedger::new();
    ledger.push("load:big", 9, 1e8);
    ledger.push("load:net", 9, 0.0048);
    ledger.push("load:cancel", 9, -1e8);
    let force = ledger.finish(12).unwrap();
    let mut folded = vec![0.0; 12];
    folded[9] = (1e8 + 0.0048) - 1e8;
    for mode in MODES {
        let d = case.dense_evidence().with_force_terms(force.terms()).solve(
            &dense,
            &folded,
            &case.free,
            &case.prescribed,
            mode,
        );
        let s = case
            .sparse_evidence(&k)
            .with_force_terms(force.terms())
            .solve(&k, &folded, &case.free, &case.prescribed, mode);
        assert_eq!(render(&d), render(&s), "{mode:?}");
        assert!(s.unwrap().load_fidelity.is_some(), "{mode:?}");
        // Without terms: the legacy vector solve.
        let d = case
            .dense_evidence()
            .solve(&dense, &folded, &case.free, &case.prescribed, mode);
        let s = case
            .sparse_evidence(&k)
            .solve(&k, &folded, &case.free, &case.prescribed, mode);
        assert_eq!(render(&d), render(&s), "{mode:?}");
    }
}

// ------------------------------------------------------------------ N references

/// The references' pipe section (fixtures.json: OD 0.2 m, ID 0.18 m, E 200
/// GPa, G 80 GPa), formed in binary64.
fn n_section() -> FrameSection {
    let (od, id) = (0.2_f64, 0.18_f64);
    let area = std::f64::consts::PI * (od * od - id * id) / 4.0;
    let inertia = std::f64::consts::PI * (od.powi(4) - id.powi(4)) / 64.0;
    FrameSection::new(200e9, 80e9, area, inertia, inertia, 2.0 * inertia).unwrap()
}

fn frame(
    a: usize,
    pa: [f64; 3],
    b: usize,
    pb: [f64; 3],
    s: FrameSection,
    y: [f64; 3],
) -> FrameElement {
    FrameElement::new(
        FrameNode::new(a, pa).unwrap(),
        FrameNode::new(b, pb).unwrap(),
        s,
        y,
    )
    .unwrap()
}

fn simple_case(
    node_count: usize,
    frames: Vec<FrameElement>,
    springs: Vec<(usize, f64)>,
    rigid: &[usize],
    loads: Vec<(usize, f64)>,
) -> Case {
    let mut case = Case {
        node_count,
        frames,
        connectors: Vec::new(),
        macros: Vec::new(),
        slots: Vec::new(),
        springs,
        free: Vec::new(),
        prescribed: Vec::new(),
        loads,
    };
    case.set_boundary(rigid.iter().map(|&d| (d, 0.0)).collect());
    case
}

/// Solves `case` through the pattern entry in `mode` and checks that it is
/// byte-identical to today's entry on the dense view.
fn solve_checked(
    case: &Case,
    mode: LinearSolveMode,
) -> Result<StructuralSolution, StructuralError> {
    let k = case.sparse();
    let dense = k.to_dense();
    let f = case.ledger();
    let d = case
        .dense_evidence()
        .solve_assembled(&dense, &f, &case.free, &case.prescribed, mode);
    let s = case
        .sparse_evidence(&k)
        .solve_assembled(&k, &f, &case.free, &case.prescribed, mode);
    assert_eq!(render(&d), render(&s), "{mode:?}");
    s
}

fn class(result: &Result<StructuralSolution, StructuralError>) -> String {
    match result {
        Ok(s) => format!("{:?}", s.report.quality),
        // The variant's name (the refusal kind), without its payload.
        Err(e) => format!("{e:?}")
            .split(|c: char| !c.is_alphanumeric())
            .next()
            .unwrap_or_default()
            .to_string(),
    }
}

fn close(observed: f64, expected: f64, scale: f64, what: &str) {
    let tolerance = 1e-9 * expected.abs().max(scale);
    assert!(
        (observed - expected).abs() <= tolerance,
        "{what}: {observed} vs {expected} (tolerance {tolerance})"
    );
}

#[test]
fn k1_n_references_have_the_same_outcome_class_and_values_in_both_modes() {
    let s = n_section();
    let o = [0.0; 3];
    let root: Vec<usize> = (0..6).collect();
    let mut cases: Vec<(&str, Case, Vec<(usize, f64)>)> = Vec::new();
    // N01 (L = 2) and N09 (L = 10) tip bending; N08/N09 tip torsion.
    let n01 = "0.0004936472016032422937485975019793020824951738551608613317752597663937867136081466630690745069807210192";
    let n01_rz = "0.0003702354012024317203114481264844765618713803913706459988314448247953400352061099973018058802355407645";
    cases.push((
        "N01",
        simple_case(
            2,
            vec![frame(0, o, 1, [2.0, 0.0, 0.0], s, [0.0, 1.0, 0.0])],
            vec![],
            &root,
            vec![(7, 1000.0)],
        ),
        vec![(7, n01.parse().unwrap()), (11, n01_rz.parse().unwrap())],
    ));
    let n09 = "0.006170590020040528671857468774741276031189673189510766647190747079922333920101833288363431337259012740";
    let n09_rz = "0.0009255885030060793007786203162111914046784509784266149970786120619883500880152749932545147005888519113";
    cases.push((
        "N09-bending",
        simple_case(
            2,
            vec![frame(0, o, 1, [10.0, 0.0, 0.0], s, [0.0, 1.0, 0.0])],
            vec![],
            &root,
            vec![(7, 100.0)],
        ),
        vec![(7, n09.parse().unwrap()), (11, n09_rz.parse().unwrap())],
    ));
    let n09_t = "2.313971257515198251946550790527978511696127446066537492696530154970875220038187483136286751472129779E-7";
    cases.push((
        "N09-torsion",
        simple_case(
            2,
            vec![frame(0, o, 1, [10.0, 0.0, 0.0], s, [0.0, 1.0, 0.0])],
            vec![],
            &root,
            vec![(9, 0.1)],
        ),
        vec![(9, n09_t.parse().unwrap())],
    ));
    let theta = "4.627942515030396503893101581055957023392254892133074985393060309941750440076374966272573502944259557E-7";
    let theta: f64 = theta.parse().unwrap();
    for t in [1.0, -1.0, 0.1, -0.1] {
        cases.push((
            "N08",
            simple_case(
                2,
                vec![frame(0, o, 1, [2.0, 0.0, 0.0], s, [0.0, 1.0, 0.0])],
                vec![],
                &root,
                vec![(9, t)],
            ),
            vec![(9, theta * t)],
        ));
    }
    // N05, N06: the soft torsion spring at the root (classes only: the
    // references are the intended-answer controls these cases fail by design).
    for (name, k, t) in [("N05", 1e-4, 1e-8), ("N06", 1e-12, 1e-16)] {
        cases.push((
            name,
            simple_case(
                2,
                vec![frame(0, o, 1, [2.0, 0.0, 0.0], s, [0.0, 1.0, 0.0])],
                vec![(3, k)],
                &[0, 1, 2, 4, 5],
                vec![(9, t)],
            ),
            vec![],
        ));
    }
    // N02 (the common rotation about (3/5, 4/5, 0) is free), N03 (a root RX
    // restores it; a root RZ does not), N04 (a separate free body).
    let tip = [1.2, 1.6, 0.0];
    let n02 = || frame(0, o, 1, tip, s, [0.0, 0.0, 1.0]);
    for (name, extra, load) in [
        ("N02-loaded", None, 1.0),
        ("N02-unloaded", None, 0.0),
        ("N03-RX", Some(3), 1.0),
        ("N03-RZ", Some(5), 1.0),
    ] {
        let mut rigid = vec![0, 1, 2, 6, 7, 8];
        rigid.extend(extra);
        cases.push((
            name,
            simple_case(
                2,
                vec![n02()],
                vec![],
                &rigid,
                vec![(9, 0.6 * load), (10, 0.8 * load)],
            ),
            vec![],
        ));
    }
    cases.push((
        "N04",
        simple_case(
            4,
            vec![
                frame(0, o, 1, [2.0, 0.0, 0.0], s, [0.0, 1.0, 0.0]),
                frame(2, [0.0, 3.0, 0.0], 3, [2.0, 3.0, 0.0], s, [0.0, 1.0, 0.0]),
            ],
            vec![],
            &root,
            vec![(7, 1000.0)],
        ),
        vec![],
    ));
    let mut classes = Vec::new();
    for (name, case, expected) in &cases {
        let dense_mode = solve_checked(case, LinearSolveMode::DenseScrutiny);
        let sparse_mode = solve_checked(case, LinearSolveMode::SparseInteractive);
        assert_eq!(class(&dense_mode), class(&sparse_mode), "{name}");
        classes.push(format!("{name}: {}", class(&sparse_mode)));
        if let Ok(solution) = &sparse_mode {
            for &(dof, value) in expected {
                close(
                    solution.displacements[dof],
                    value,
                    0.0,
                    &format!("{name} dof {dof}"),
                );
            }
        } else {
            assert!(expected.is_empty(), "{name}: refused, {sparse_mode:?}");
        }
    }
    eprintln!("k1 N classes: {classes:?}");
}

// ------------------------------------------------------------------ T0R references

/// T0R's C1 section (OD 0.12 m, wall 0.01 m; E 200 GPa, G 76.923076923 GPa).
fn c1() -> FrameSection {
    let (od, id) = (0.12_f64, 0.10_f64);
    let area = std::f64::consts::PI * (od * od - id * id) / 4.0;
    let inertia = std::f64::consts::PI * (od.powi(4) - id.powi(4)) / 64.0;
    FrameSection::new(200e9, 76.923076923e9, area, inertia, inertia, 2.0 * inertia).unwrap()
}

/// Anchor (node 0) reactions from sparse rows, and the check that the dense
/// form of the same reactions (the product's E12) gives the same bits.
fn anchor(case: &Case, solution: &StructuralSolution) -> [f64; 6] {
    let k = case.sparse();
    let f = case.ledger();
    let reactions = k.reactions(&solution.displacements, &f).unwrap();
    let dense = k.to_dense();
    for dof in 0..reactions.len() {
        let internal: f64 = dense[dof]
            .iter()
            .zip(&solution.displacements)
            .map(|(a, b)| a * b)
            .sum();
        let mut a = open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator::new();
        let expected = a
            .add(internal)
            .and_then(|()| f.accumulate_dof(dof, &mut a, true))
            .and_then(|()| a.round())
            .unwrap_or(f64::NAN);
        assert_eq!(reactions[dof].to_bits(), expected.to_bits());
    }
    std::array::from_fn(|i| reactions[i])
}

#[test]
fn k1_t0r_reactions_from_sparse_rows_match_the_references() {
    // REF-M05-T: 1 m cantilever along x, tip torque 500 N m about x.
    let t = simple_case(
        2,
        vec![frame(
            0,
            [0.0; 3],
            1,
            [1.0, 0.0, 0.0],
            c1(),
            [0.0, 1.0, 0.0],
        )],
        vec![],
        &[0, 1, 2, 3, 4, 5],
        vec![(9, 500.0)],
    );
    // REF-M05-R1 (its point loads): 2 m cantilever along x, tip force
    // (10, -20, 30) N and moment (4, 5, -6) N m; anchor F = (-10, 20, -30),
    // M = (-4, 55, 46). Rotated by the permutation xyz -> yzx and by 30
    // degrees about z (expected: Q times the unrotated reactions), and
    // translated to (5, -3, 2) (moments about the node: unchanged).
    let (c30, s30) = (30f64.to_radians().cos(), 30f64.to_radians().sin());
    let rotations: [(&str, Box<dyn Fn([f64; 3]) -> [f64; 3]>, [f64; 3]); 4] = [
        ("unrotated", Box::new(|v: [f64; 3]| v), [0.0; 3]),
        ("perm", Box::new(|v: [f64; 3]| [v[2], v[0], v[1]]), [0.0; 3]),
        (
            "rot30",
            Box::new(move |v: [f64; 3]| [c30 * v[0] - s30 * v[1], s30 * v[0] + c30 * v[1], v[2]]),
            [0.0; 3],
        ),
        ("translated", Box::new(|v: [f64; 3]| v), [5.0, -3.0, 2.0]),
    ];
    for mode in MODES {
        let solution = solve_checked(&t, mode).unwrap();
        let a = anchor(&t, &solution);
        for (i, e) in [0.0, 0.0, 0.0, -500.0, 0.0, 0.0].iter().enumerate() {
            close(a[i], *e, 500.0, &format!("M05-T {mode:?} {i}"));
        }
        for (label, q, origin) in &rotations {
            let tip = q([2.0, 0.0, 0.0]);
            let tip = [tip[0] + origin[0], tip[1] + origin[1], tip[2] + origin[2]];
            let f = q([10.0, -20.0, 30.0]);
            let c = q([4.0, 5.0, -6.0]);
            let case = simple_case(
                2,
                vec![frame(0, *origin, 1, tip, c1(), q([0.0, 0.0, 1.0]))],
                vec![],
                &[0, 1, 2, 3, 4, 5],
                (0..3)
                    .map(|i| (6 + i, f[i]))
                    .chain((0..3).map(|i| (9 + i, c[i])))
                    .collect(),
            );
            let solution = solve_checked(&case, mode).unwrap();
            let a = anchor(&case, &solution);
            let ef = q([-10.0, 20.0, -30.0]);
            let em = q([-4.0, 55.0, 46.0]);
            for i in 0..3 {
                close(a[i], ef[i], 60.0, &format!("M05-R1 {label} {mode:?} F{i}"));
                close(
                    a[3 + i],
                    em[i],
                    60.0,
                    &format!("M05-R1 {label} {mode:?} M{i}"),
                );
            }
        }
        // REF-M05-SPRING: the 1 m cantilever with a 1e6 N/m UY spring at the
        // tip and a 1000 N UY tip force.
        let spring = simple_case(
            2,
            vec![frame(
                0,
                [0.0; 3],
                1,
                [1.0, 0.0, 0.0],
                c1(),
                [0.0, 1.0, 0.0],
            )],
            vec![(7, 1e6)],
            &[0, 1, 2, 3, 4, 5],
            vec![(7, 1000.0)],
        );
        let solution = solve_checked(&spring, mode).unwrap();
        let a = anchor(&spring, &solution);
        close(
            solution.displacements[7],
            0.00024026835057874228,
            0.0,
            "M05-SPRING tip u",
        );
        close(
            -1e6 * solution.displacements[7],
            -240.2683505787423,
            1000.0,
            "M05-SPRING spring",
        );
        close(a[1], -759.7316494212578, 1000.0, "M05-SPRING anchor Fy");
        close(a[5], -759.7316494212578, 1000.0, "M05-SPRING anchor Mz");
        for i in [0, 2, 3, 4] {
            close(a[i], 0.0, 1000.0, "M05-SPRING anchor zero");
        }
    }
}

// ------------------------------------------------------------------ relabelling

#[test]
fn k1_relabelled_model_gives_the_same_answers() {
    // §4.8 item 6 / mutation 10: a skew chain with a connector and springs,
    // numbered forwards and backwards (every member reversed). The pattern
    // path is byte-identical to the dense-derived path in both numberings, and
    // the two numberings agree within the protected criterion.
    let s = n_section();
    let p = |k: usize| [1.3 * k as f64, 0.4 * k as f64, -0.25 * k as f64];
    let build = |reverse: bool| -> Case {
        let label = |k: usize| if reverse { 4 - k } else { k };
        let mut frames = Vec::new();
        for k in 0..3 {
            let (a, b) = if reverse { (k + 1, k) } else { (k, k + 1) };
            frames.push(frame(label(a), p(a), label(b), p(b), s, [0.0, 0.0, 1.0]));
        }
        let connectors = vec![chord_connector(
            FrameNode::new(label(3), p(3)).unwrap(),
            FrameNode::new(label(4), p(4)).unwrap(),
            [4.0e7, 3.0e6, 2.0e5, 1.0e5],
        )];
        let mut case = simple_case(
            5,
            frames,
            vec![(6 * label(2) + 2, 5.0e5), (6 * label(4) + 3, 2.0e3)],
            &(0..6).map(|d| 6 * label(0) + d).collect::<Vec<_>>(),
            vec![
                (6 * label(4) + 1, -900.0),
                (6 * label(4) + 5, 40.0),
                (6 * label(2), 250.0),
            ],
        );
        case.connectors = connectors;
        case
    };
    for mode in MODES {
        let forward = solve_checked(&build(false), mode).unwrap();
        let backward = solve_checked(&build(true), mode).unwrap();
        let scale_t = (0..30)
            .filter(|d| d % 6 < 3)
            .map(|d| forward.displacements[d].abs())
            .fold(0.0, f64::max);
        let scale_r = (0..30)
            .filter(|d| d % 6 >= 3)
            .map(|d| forward.displacements[d].abs())
            .fold(0.0, f64::max);
        for k in 0..5 {
            for c in 0..6 {
                let scale = if c < 3 { scale_t } else { scale_r };
                close(
                    backward.displacements[6 * (4 - k) + c],
                    forward.displacements[6 * k + c],
                    scale,
                    &format!("{mode:?} node {k} component {c}"),
                );
            }
        }
    }
}

// ------------------------------------------------------------------ KREV-01

#[test]
fn krev01_on_sparse_geometry_witnesses_are_unchanged() {
    // A valid oblique witness: a (3, 4, 0) member with both nodes' translations
    // grounded has a free rigid rotation about its axis; both entries refuse
    // with the same mechanism direction, in both modes and origins.
    let s = n_section();
    for shift in [0.0, 8.0] {
        let case = simple_case(
            2,
            vec![frame(
                0,
                [shift, shift, 0.0],
                1,
                [shift + 3.0, shift + 4.0, 0.0],
                s,
                [0.0, 0.0, 1.0],
            )],
            vec![],
            &[0, 1, 2, 6, 7, 8],
            vec![(9, 1.0)],
        );
        for mode in MODES {
            let result = solve_checked(&case, mode);
            assert!(
                matches!(result, Err(StructuralError::Mechanism { .. })),
                "{result:?}"
            );
        }
    }
    // A coarsened witness is refuted by the original geometry: grounded
    // translations of a nearly collinear three-node frame produce no
    // mechanism claim in either representation.
    let points = [
        [1e16, 1e16, 0.0],
        [-1e16, -1e16, 0.0],
        [-1e16, -9999999999999998.0, 0.0],
    ];
    let frames = vec![
        frame(0, points[0], 1, points[1], s, [0.0, 0.0, 1.0]),
        frame(1, points[1], 2, points[2], s, [0.0, 0.0, 1.0]),
    ];
    let case = simple_case(
        3,
        frames,
        vec![],
        &[0, 1, 2, 6, 7, 8, 12, 13, 14],
        vec![(9, 1.0)],
    );
    for mode in MODES {
        let result = solve_checked(&case, mode);
        assert!(
            !matches!(result, Err(StructuralError::Mechanism { .. })),
            "{result:?}"
        );
    }
}

// ------------------------------------------------------------------ the loop pin

/// P1's 122 skew cantilever with an open gap at the tip (as K-D5's loop pin).
fn loop_input(case: &Case) -> NonlinearFrameSolveInput {
    NonlinearFrameSolveInput {
        node_count: 2,
        elements: case.frames.clone(),
        connectors: Vec::new(),
        curved_bend_elements: vec![],
        force: case.force(),
        base_restrained_dofs: F122.rigid.to_vec(),
        nonlinear_supports: vec![NonlinearSupport::gap(
            "k1-open-gap",
            1,
            FrameDof::Uz,
            1.0,
            GapDirection::PositiveDisplacement,
        )
        .unwrap()],
        initial_states: vec![SupportStateRecord::new(
            "k1-open-gap",
            ActiveSetState::Inactive,
        )],
        friction_normal_reactions: vec![],
        derived_friction_normal_reactions: vec![],
        convergence: ConvergenceControl::new(
            "DEC-046-fixture-active-set-count-tightening",
            ConvergencePolicyStatus::Accepted,
            0.0,
            0.0,
            4,
        )
        .unwrap(),
    }
}

/// K1 pin extension (3), ROOT: the nonlinear loop reaches neither the dense
/// nor the pattern formation-checked entry. Precondition: both entries demote
/// the same linear system (F122), while the loop's first iteration is Passed
/// and bit-equal to the named binary64 solve.
#[test]
fn k1_nonlinear_loop_reaches_neither_formation_entry() {
    let case = Case::from_model(&F122);
    let k = case.sparse();
    let dense = k.to_dense();
    let input = loop_input(&case);
    for mode in MODES {
        let (d, s) = both_checked(&case, mode, &case.macros);
        assert_eq!(
            d.unwrap().report.quality,
            SolveQuality::Sensitive,
            "{mode:?}"
        );
        assert_eq!(
            s.unwrap().report.quality,
            SolveQuality::Sensitive,
            "{mode:?}"
        );
        let result =
            solve_active_set_frame_with_mode_and_springs(&input, mode, &case.springs).unwrap();
        assert!(result.converged, "{mode:?}");
        let first = result.iterations.first().expect("iteration");
        assert_eq!(
            first.structural_report.quality,
            SolveQuality::Passed,
            "{mode:?}"
        );
        let binary64 = case
            .dense_evidence()
            .solve_binary64(&dense, &case.force(), &case.free, &case.prescribed, mode)
            .unwrap();
        assert_eq!(
            bits(&first.displacements),
            bits(&binary64.displacements),
            "{mode:?}"
        );
        assert_eq!(
            format!("{:?}", first.structural_report),
            format!("{:?}", binary64.report),
            "{mode:?}"
        );
    }
}

// ------------------------------------------------------------------ RV8 (review findings)

// RV8-1 to RV8-3 (T3 REVIEW/K1_REVIEW.md): three test gaps the independent
// review found. Each case is RV8's (invented), and each first asserts, on the
// dense side only, that it discriminates the alternative its mutant takes.

/// The product's dense assembly in a chosen order: frames and connectors
/// (`assemble_global_stiffness_with_connectors`), then the curved blocks
/// in order (or reversed), then the springs (or the springs before the
/// blocks). `(false, false)` is the product's order, `Case::product_dense`.
fn dense_in_order(case: &Case, blocks_reversed: bool, springs_first: bool) -> Vec<Vec<f64>> {
    let mut k =
        assemble_global_stiffness_with_connectors(case.node_count, &case.frames, &case.connectors)
            .unwrap();
    let add_springs = |k: &mut Vec<Vec<f64>>| {
        for &(dof, value) in &case.springs {
            k[dof][dof] += value;
        }
    };
    if springs_first {
        add_springs(&mut k);
    }
    let mut slots: Vec<&CurvedBendStiffnessElement> = case.slots.iter().collect();
    if blocks_reversed {
        slots.reverse();
    }
    for s in slots {
        let map = element_dof_map(s.node_i, s.node_j);
        for (r, &row) in map.iter().enumerate() {
            for (c, &col) in map.iter().enumerate() {
                k[row][col] += s.global_stiffness[r][c];
            }
        }
    }
    if !springs_first {
        add_springs(&mut k);
    }
    k
}

fn matrix_bits(k: &[Vec<f64>]) -> Vec<Vec<u64>> {
    k.iter().map(|row| bits(row)).collect()
}

/// K1's representation of `case` against the product's dense assembly, bit
/// for bit: the pattern values, the evidence contributions coalesced on the
/// pattern, K*u (PP's `multiply_matrix_vector` form) and E12's reactions.
fn assert_assembly_bit_identical(case: &Case, ctx: &str) {
    let product = case.product_dense();
    let k = case.sparse();
    assert_eq!(
        matrix_bits(&k.to_dense()),
        matrix_bits(&product),
        "{ctx}: K bits"
    );
    let coalesced = SparseStiffness::from_pattern_and_contributions(
        k.pattern().clone(),
        &case.dense_evidence().contributions,
    )
    .unwrap();
    assert_eq!(
        bits(coalesced.values()),
        bits(k.values()),
        "{ctx}: coalesced contributions"
    );
    let u: Vec<f64> = (0..6 * case.node_count)
        .map(|d| match d % 4 {
            0 => 1e-3 * (d as f64 + 1.0).sin(),
            1 => -0.0,
            2 => -2.5e-4 * d as f64,
            _ => 7e-5,
        })
        .collect();
    let product_ku: Vec<f64> = product
        .iter()
        .map(|row| row.iter().zip(&u).map(|(a, b)| a * b).sum())
        .collect();
    assert_eq!(
        bits(&k.multiply(&u).unwrap()),
        bits(&product_ku),
        "{ctx}: K*u"
    );
    let f = case.ledger();
    let expected: Vec<f64> = product_ku
        .iter()
        .enumerate()
        .map(|(dof, &internal)| {
            let mut a = open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator::new();
            a.add(internal)
                .and_then(|()| f.accumulate_dof(dof, &mut a, true))
                .and_then(|()| a.round())
                .unwrap_or(f64::NAN)
        })
        .collect();
    assert_eq!(
        bits(&k.reactions(&u, &f).unwrap()),
        bits(&expected),
        "{ctx}: reactions"
    );
}

/// The adapter's entries on the dense view and the pattern entries, plain and
/// formation-checked, byte-identical in both modes and solved (not vacuous).
fn assert_entries_byte_identical(case: &Case, ctx: &str) {
    for mode in MODES {
        let (d, s) = both_plain(case, mode);
        assert_eq!(d, s, "{ctx} {mode:?} plain");
        assert!(s.starts_with("Ok("), "{ctx} {mode:?} plain: {s}");
        let (d, s) = both_checked(case, mode, &case.macros);
        assert_eq!(render(&d), render(&s), "{ctx} {mode:?} formation-checked");
        assert!(s.is_ok(), "{ctx} {mode:?} formation-checked: {s:?}");
    }
}

/// RV8's bend geometry (invented): R = 0.6 m bends in the XY plane on K-D5's
/// pipe section (E 200 GPa, G 80 GPa), with flexibility factor `flex` in both
/// planes.
const RV8_R: f64 = 0.6;

fn rv8_frame_section() -> FrameSection {
    FrameSection::new(
        2.0e11,
        8.0e10,
        0.005969026041820614,
        2.700984283923829e-05,
        2.700984283923829e-05,
        5.401968567847658e-05,
    )
    .unwrap()
}

fn rv8_bend(
    i: usize,
    pi: [f64; 3],
    j: usize,
    pj: [f64; 3],
    y_reference: [f64; 3],
    flex: f64,
) -> CurvedBendMacroElement {
    let radius = RV8_R;
    CurvedBendMacroElement::new(
        FrameNode::new(i, pi).unwrap(),
        FrameNode::new(j, pj).unwrap(),
        radius,
        y_reference,
        2.0e11,
        8.0e10,
        0.005969026041820614,
        2.700984283923829e-05,
        5.401968567847658e-05,
        flex,
        flex,
    )
    .unwrap()
}

/// A case with realized bends: each bend's slot is built from its macro
/// element, as the product builds it, and added after the frames.
fn rv8_case(
    node_count: usize,
    frames: Vec<FrameElement>,
    bends: Vec<CurvedBendMacroElement>,
    springs: Vec<(usize, f64)>,
    rigid: &[usize],
    loads: Vec<(usize, f64)>,
) -> Case {
    let slots = bends
        .iter()
        .enumerate()
        .map(|(k, b)| {
            CurvedBendStiffnessElement::from_macro_element(format!("rv8-bend-{k}"), b).unwrap()
        })
        .collect();
    let mut case = Case {
        node_count,
        frames,
        connectors: Vec::new(),
        macros: bends,
        slots,
        springs,
        free: Vec::new(),
        prescribed: Vec::new(),
        loads,
    };
    case.set_boundary(rigid.iter().map(|&d| (d, 0.0)).collect());
    case
}

/// RV8-1: the realized bends are added in the product's order. A tee at a
/// bend-bend junction: bends b1 (node 0 to 1, about the origin) and b2 (node 1
/// to 2, about (0, 2R, 0)), anchored straight runs 3-0 and 2-4, and a straight
/// branch 1-5, so node 1 carries the branch and both bends. Kills
/// RV8-BLOCK-ORDER (the blocks added in reverse).
#[test]
fn k1_bend_bend_tee_adds_the_realized_bends_in_the_products_order_rv8_1() {
    let r = RV8_R;
    let p = [
        [r, 0.0, 0.0],
        [0.0, r, 0.0],
        [-r, 2.0 * r, 0.0],
        [r, -2.0, 0.0],
        [-r, 2.0 * r + 2.0, 0.0],
        [0.0, r, 2.2],
    ];
    let s = rv8_frame_section();
    for flex in [1.0, 1.7] {
        let case = rv8_case(
            6,
            vec![
                frame(3, p[3], 0, p[0], s, [0.0, 0.0, 1.0]),
                frame(2, p[2], 4, p[4], s, [0.0, 0.0, 1.0]),
                frame(1, p[1], 5, p[5], s, [1.0, 0.0, 0.0]),
            ],
            vec![
                rv8_bend(0, p[0], 1, p[1], [1.0, 1.0, 0.0], flex),
                rv8_bend(1, p[1], 2, p[2], [-1.0, -1.0, 0.0], flex),
            ],
            Vec::new(),
            &(18..36).collect::<Vec<_>>(),
            vec![(7, -2500.0), (8, 400.0), (11, 55.0), (13, 900.0)],
        );
        let ctx = format!("bend-bend tee flex={flex}");
        // Precondition (dense only): the bends in reverse order change bits.
        assert_ne!(
            matrix_bits(&dense_in_order(&case, true, false)),
            matrix_bits(&case.product_dense()),
            "{ctx}: precondition, the block order changes the assembled bits"
        );
        assert_assembly_bit_identical(&case, &ctx);
        assert_entries_byte_identical(&case, &ctx);
    }
}

/// RV8-2: the springs are added after the realized bends, as the product adds
/// them. A bend (node 0 to 1) between anchored straight runs 2-0 and 1-3, with
/// support springs on every DOF of both bend ends, over RV8's eight
/// magnitudes (1.1e3 to 9.7e8 N/m) and two flexibility factors. Kills
/// RV8-SPRING-FIRST (the springs added before the blocks).
#[test]
fn k1_bend_support_springs_follow_the_bend_in_the_products_order_rv8_2() {
    let r = RV8_R;
    let p = [[r, 0.0, 0.0], [0.0, r, 0.0], [r, -2.0, 0.0], [-2.4, r, 0.3]];
    let s = rv8_frame_section();
    let mut discriminating = Vec::new();
    for flex in [1.0, 1.7] {
        for kk in [1.1e3, 3.7e4, 2.9e5, 1.3e6, 7.9e6, 4.4e7, 2.1e8, 9.7e8] {
            let springs: Vec<(usize, f64)> = (0..6)
                .flat_map(|d| {
                    [
                        (d, kk * (1.0 + 0.07 * d as f64)),
                        (6 + d, kk * (1.3 - 0.05 * d as f64)),
                    ]
                })
                .collect();
            let case = rv8_case(
                4,
                vec![
                    frame(2, p[2], 0, p[0], s, [0.0, 0.0, 1.0]),
                    frame(1, p[1], 3, p[3], s, [0.0, 0.0, 1.0]),
                ],
                vec![rv8_bend(0, p[0], 1, p[1], [1.0, 1.0, 0.0], flex)],
                springs,
                &(12..24).collect::<Vec<_>>(),
                vec![(1, -1800.0), (8, 350.0), (9, 42.0), (10, 700.0)],
            );
            let ctx = format!("bend supports flex={flex} k={kk:e}");
            if matrix_bits(&dense_in_order(&case, false, true))
                != matrix_bits(&case.product_dense())
            {
                discriminating.push(ctx.clone());
            }
            assert_assembly_bit_identical(&case, &ctx);
            assert_entries_byte_identical(&case, &ctx);
        }
    }
    // Precondition (dense only; checked last so that a wrong order fails at
    // the bitwise assertion above): some magnitudes round differently when a
    // spring precedes the bend on a shared diagonal, so the test discriminates.
    assert!(
        !discriminating.is_empty(),
        "precondition: no magnitude discriminates the spring-against-bend order"
    );
    eprintln!("k1 rv8-2 discriminating cases: {discriminating:?}");
}

/// RV8-3: the sparse formation check reads the same load inputs as the dense
/// one: the ledger terms, not the folded force. K-D5's demoting models F122
/// and C122-R10-Y100 (the curved true positive) with each load split into three ledger terms (0.1v, 0.7v and
/// v - 0.1v - 0.7v), whose exact sum is not the rounded net on some load. In
/// both modes the records are identical and the case demotes in both
/// representations. Kills RV8-FC-TERMS (the sparse check drops the terms), in
/// SparseInteractive.
#[test]
fn k1_split_ledger_formation_check_reads_the_ledger_terms_in_both_representations_rv8_3() {
    for m in [&F122, &C122_R10_Y100] {
        let case = Case::from_model(m);
        let mut split = LoadLedger::new();
        let mut parts = Vec::new();
        for (i, &(dof, v)) in case.loads.iter().enumerate() {
            let (a, b) = (v * 0.1, v * 0.7);
            let c = v - a - b;
            split.push(format!("load:{i}:a"), dof, a);
            split.push(format!("load:{i}:b"), dof, b);
            split.push(format!("load:{i}:c"), dof, c);
            parts.push((dof, [a, b, c]));
        }
        let split = split.finish(6 * case.node_count).unwrap();
        // The folded values as one-term ledger entries: what the check reads
        // when the terms are dropped.
        let mut folded = LoadLedger::new();
        for (dof, &v) in split.values().iter().enumerate() {
            if v != 0.0 {
                folded.push(format!("folded:{dof}"), dof, v);
            }
        }
        let folded = folded.finish(6 * case.node_count).unwrap();
        // Precondition 1: on some load the three terms' exact sum is not the
        // rounded net.
        assert!(
            parts.iter().any(|&(dof, terms)| {
                let mut exact = open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator::new();
                for t in terms {
                    exact.add(t).unwrap();
                }
                exact.add(-split.values()[dof]).unwrap();
                !exact.is_zero()
            }),
            "{}: precondition, exact net equals the rounded net on every load",
            m.name
        );
        let k = case.sparse();
        let dense = k.to_dense();
        for mode in MODES {
            let dense_entry = |f: &AssembledForce| {
                case.dense_evidence().solve_assembled_with_formation_check(
                    &dense,
                    f,
                    &case.free,
                    &case.prescribed,
                    mode,
                    &case.macros,
                    true,
                )
            };
            let d = dense_entry(&split).unwrap();
            let s = case
                .sparse_evidence(&k)
                .solve_assembled_with_formation_check(
                    &k,
                    &split,
                    &case.free,
                    &case.prescribed,
                    mode,
                    &case.macros,
                    true,
                )
                .unwrap();
            // Precondition 2 (dense only): the record depends on the terms.
            let d_folded = dense_entry(&folded).unwrap();
            assert_ne!(
                format!("{:?}", d.formation_check),
                format!("{:?}", d_folded.formation_check),
                "{} {mode:?}: precondition, the terms and the folded force give different records",
                m.name
            );
            // Parity: the same solution and record in both representations.
            assert_eq!(
                format!("{d:?}"),
                format!("{s:?}"),
                "{} {mode:?}: split ledger",
                m.name
            );
            // A demoting case in both representations.
            for (representation, solution) in [("dense", &d), ("sparse", &s)] {
                assert_eq!(
                    solution.report.quality,
                    SolveQuality::Sensitive,
                    "{} {mode:?} {representation}",
                    m.name
                );
                assert_eq!(
                    solution.formation_check.as_ref().expect("demoted").reason,
                    FormationCheckReason::Estimate,
                    "{} {mode:?} {representation}",
                    m.name
                );
            }
        }
    }
}

// ------------------------------------------- I9: the skew M03 pin (append)
//
// I9 (T3; tests only): RV7's accepted skew rows (K2A_REVIEW.md B1) through
// the adapter's two evidences, `AssemblyEvidence` and K1's
// `SparseAssemblyEvidence`, in both modes. K2a's `local_stiffness` refuses
// these formations, so main's pre-K2a matrix is formed here and carried as an
// explicit slot with its own `transform_roundoff` evidence: the same bounds
// and counts a frame's evidence records. (The slot differs from a frame, in
// both evidences alike, only in being unqualified for the rigid-body screen
// and in the symmetry-basis text of the report.) The
// refused rows are refused by `transform_roundoff` itself, before either
// evidence exists; the kernel pin is FK `tests/m03_skew_scope.rs`.

use open_pipe_stress_frame_kernel::{transform_global_stiffness, FrameProperties, RZ};

/// Main's pre-K2a local matrix, operation for operation (FK `local_stiffness`
/// at 134eefc24, `lib.rs:716-741`). FK's pin asserts this formation equals
/// FK's own, bit for bit, on normal inputs.
fn i9_pre_k2a_local(p: &FrameProperties) -> Matrix12 {
    let s = p.section;
    let (e, g, iy, iz) = (
        s.elastic_modulus,
        s.shear_modulus,
        s.second_moment_y,
        s.second_moment_z,
    );
    let length = p.length;
    let length2 = length * length;
    let length3 = length2 * length;
    let axial = e * s.area / length;
    let torsion = g * s.torsion_constant / length;
    let (y12, y6, y4, y2) = (
        12.0 * e * iy / length3,
        6.0 * e * iy / length2,
        4.0 * e * iy / length,
        2.0 * e * iy / length,
    );
    let (z12, z6, z4, z2) = (
        12.0 * e * iz / length3,
        6.0 * e * iz / length2,
        4.0 * e * iz / length,
        2.0 * e * iz / length,
    );
    let mut k = [[0.0; 12]; 12];
    for (d, v) in [(0, axial), (3, torsion)] {
        k[d][d + 6] = -v;
        k[d + 6][d] = -v;
        k[d][d] = v;
        k[d + 6][d + 6] = v;
    }
    let mut add = |idx: [usize; 4], t: [[f64; 4]; 4]| {
        for r in 0..4 {
            for c in 0..4 {
                k[idx[r]][idx[c]] += t[r][c];
            }
        }
    };
    add(
        [1, 5, 7, 11],
        [
            [z12, z6, -z12, z6],
            [z6, z4, -z6, z2],
            [-z12, -z6, z12, -z6],
            [z6, z2, -z6, z4],
        ],
    );
    add(
        [2, 4, 8, 10],
        [
            [y12, -y6, -y12, -y6],
            [-y6, y4, y6, y2],
            [-y12, y6, y12, y6],
            [-y6, y2, y6, y4],
        ],
    );
    k
}

#[test]
fn i9_skew_m03_accepted_rows_are_identical_through_both_evidences_in_both_modes() {
    use std::f64::consts::PI;
    // Invented: L = 2^-39 m; OD 1e-11 m, wall 1e-12 m (the section as PP forms
    // it); G = 1e-100 Pa; E so that (12E)*I rounds to 2^t; a global RZ moment
    // of 0.1*EI/L at N1, as RV7's product probe loads it.
    let l = 2f64.powi(-39);
    let (od, wall) = (1.0e-11_f64, 1.0e-12_f64);
    let id = od - 2.0 * wall;
    let area = PI * (od.powi(2) - id.powi(2)) / 4.0;
    let inertia = PI * (od.powi(4) - id.powi(4)) / 64.0;
    let s = l / 3f64.sqrt();
    let mut runs = 0;
    for t in [-1030, -1040, -1045, -1050] {
        let e = (1.0 / (12.0 * inertia)) * 2f64.powi(-1000) * 2f64.powi(t + 1000);
        for (label, node_j, y_reference) in [
            ("skew (1,1,1), yref +z", [s, s, s], [0.0, 0.0, 1.0]),
            (
                "skew (1,2,2), yref +x",
                [l / 3.0, 2.0 * l / 3.0, 2.0 * l / 3.0],
                [1.0, 0.0, 0.0],
            ),
        ] {
            let ctx = format!("(12E)I=2^{t} {label}");
            let element = FrameElement::new(
                FrameNode::new(0, [0.0; 3]).unwrap(),
                FrameNode::new(1, node_j).unwrap(),
                FrameSection::new(e, 1.0e-100, area, inertia, inertia, 2.0 * inertia).unwrap(),
                y_reference,
            )
            .unwrap();
            // K2a refuses the formation by name; main's matrix is formed here.
            assert!(element.local_stiffness().is_err(), "{ctx}");
            let local = i9_pre_k2a_local(&element.properties().unwrap());
            let orientation = element.orientation().unwrap();
            let r = structural::transform_roundoff(&local, &orientation.transformation_matrix())
                .expect("M03's element-entry floor accepts the skew member");
            let mut case = Case {
                node_count: 2,
                frames: Vec::new(),
                connectors: Vec::new(),
                macros: Vec::new(),
                slots: vec![CurvedBendStiffnessElement {
                    element_id: "i9-pre-k2a-element".to_string(),
                    node_i: 0,
                    node_j: 1,
                    global_stiffness: transform_global_stiffness(&local, &orientation),
                    symmetry_formation: Some((r.absolute_roundoff, r.operation_counts)),
                }],
                springs: Vec::new(),
                free: Vec::new(),
                prescribed: Vec::new(),
                loads: vec![(
                    6 + RZ,
                    0.1 * (e * 2f64.powi(600) * inertia / l) * 2f64.powi(-600),
                )],
            };
            case.set_boundary((0..6).map(|d| (d, 0.0)).collect());
            for mode in MODES {
                let (dense, sparse) = both_plain(&case, mode);
                assert_eq!(dense, sparse, "{ctx} {mode:?}");
                eprintln!(
                    "I9 adapter {ctx} {mode:?}: {}",
                    dense.chars().take(160).collect::<String>()
                );
                runs += 1;
            }
        }
    }
    assert_eq!(runs, 16);
}

/// T4-U3: an invented objective connector between two nodes, standing in for
/// the deleted user element: Q.x along the chord (offsets zero), an uncoupled
/// K = diag(axial, lateral, lateral, torsional, angular, angular) at Ls = 1 m.
fn chord_connector(
    i: FrameNode,
    j: FrameNode,
    [axial, lateral, angular, torsional]: [f64; 4],
) -> open_pipe_stress_frame_kernel::connector::ObjectiveConnector {
    use open_pipe_stress_frame_kernel::connector::{
        ConnectorAttachment, ObjectiveConnector, ScaledWorkMatrix,
    };
    let d = [
        j.coordinates[0] - i.coordinates[0],
        j.coordinates[1] - i.coordinates[1],
        j.coordinates[2] - i.coordinates[2],
    ];
    let unit = |v: [f64; 3]| {
        let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        [v[0] / n, v[1] / n, v[2] / n]
    };
    let cross = |a: [f64; 3], b: [f64; 3]| {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    };
    let x = unit(d);
    let helper = if x[2].abs() < 0.9 { [0.0, 0.0, 1.0] } else { [1.0, 0.0, 0.0] };
    let y = unit(cross(helper, x));
    let z = cross(x, y);
    let mut h = [0.0; 21];
    for (index, value) in [(0, axial), (6, lateral), (11, lateral), (15, torsional), (18, angular), (20, angular)] {
        h[index] = value;
    }
    ObjectiveConnector::new(
        i,
        j,
        ConnectorAttachment::global([0.0; 3]),
        ConnectorAttachment::global([0.0; 3]),
        [[x[0], y[0], z[0]], [x[1], y[1], z[1]], [x[2], y[2], z[2]]],
        ScaledWorkMatrix {
            upper_triangle: h,
            translation_scale: 1.0,
        },
        [0.0; 6],
    )
    .unwrap()
}

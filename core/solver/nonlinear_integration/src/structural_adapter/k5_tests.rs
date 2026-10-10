//! K5 adapter tests (T3 D1 revision 5a.2 §4.9, W4; ROOT's K5 rulings Q1(b),
//! Q2(b), Q5(a) and Q7(a)): the constrained-body witness runs in the four
//! selected branches of the formation-checked entries only.
//!
//! Invented inputs only; every value is stated here or in K-D5's generated
//! `kd5_models.rs` (included read-only). The expected witness directions are
//! exact by construction (dyadic geometry; derivations beside each model):
//! they never pass through the platform libm. A product-formed curved element
//! (through `sin`, `cos` and `atan2`) is asserted by outcome only.
use super::kd5_tests::{bits, MemberData, ModelData, SectionData, MODES};
use super::*;
use crate::{
    solve_active_set_frame_with_mode_and_springs, ConvergenceControl, ConvergencePolicyStatus,
    NonlinearFrameSolveInput, NonlinearIntegrationError,
};
use open_pipe_stress_frame_kernel::connector::{ConnectorAttachment, ScaledWorkMatrix};
use open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator;
use open_pipe_stress_frame_kernel::load_ledger::LoadLedger;
use open_pipe_stress_frame_kernel::structural::{
    assemble_sparse_stiffness, SparseAssemblyOptions, StiffnessBlock,
};
use open_pipe_stress_frame_kernel::{FrameDof, FrameNode, FrameSection};
use open_pipe_stress_nonlinear_supports::{
    ActiveSetState, GapDirection, NonlinearSupport, SupportStateRecord,
};

#[allow(dead_code)]
#[path = "kd5_models.rs"]
mod kd5_models;
use kd5_models::*;

// ------------------------------------------------------------------ models

/// One model: its element families, loads and boundary; both evidences and
/// both stiffness representations are formed from it.
#[derive(Clone)]
struct Model {
    node_count: usize,
    frames: Vec<FrameElement>,
    connectors: Vec<ObjectiveConnector>,
    macros: Vec<CurvedBendMacroElement>,
    slots: Vec<CurvedBendStiffnessElement>,
    springs: Vec<(usize, f64)>,
    prescribed: Vec<(usize, f64)>,
    free: Vec<usize>,
    loads: Vec<(usize, f64)>,
}

fn section() -> FrameSection {
    let s = &E1.section;
    FrameSection::new(s.e, s.g, s.a, s.i, s.i, s.j).unwrap()
}

fn node(index: usize, coordinates: [f64; 3]) -> FrameNode {
    FrameNode::new(index, coordinates).unwrap()
}

impl Model {
    fn empty(node_count: usize) -> Self {
        Model {
            node_count,
            frames: Vec::new(),
            connectors: Vec::new(),
            macros: Vec::new(),
            slots: Vec::new(),
            springs: Vec::new(),
            prescribed: Vec::new(),
            free: Vec::new(),
            loads: Vec::new(),
        }
    }
    fn frame(&mut self, i: (usize, [f64; 3]), j: (usize, [f64; 3]), y: [f64; 3]) {
        self.frames
            .push(FrameElement::new(node(i.0, i.1), node(j.0, j.1), section(), y).unwrap());
    }
    fn bend(&mut self, i: (usize, [f64; 3]), j: (usize, [f64; 3]), center: [f64; 3]) {
        let s = &E1.section;
        let e = CurvedBendMacroElement::new(
            node(i.0, i.1),
            node(j.0, j.1),
            center,
            s.e,
            s.g,
            s.a,
            s.i,
            s.j,
            2.0,
            2.0,
        )
        .unwrap();
        let id = format!("bend-{}", self.slots.len());
        self.slots
            .push(CurvedBendStiffnessElement::from_macro_element(id, &e).unwrap());
        self.macros.push(e);
    }
    fn pin(&mut self, dofs: &[usize]) {
        self.prescribed = dofs.iter().map(|&d| (d, 0.0)).collect();
        self.free = (0..6 * self.node_count)
            .filter(|d| !dofs.contains(d))
            .collect();
    }
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
    /// The kernel's assembly at 2^b (K2b), as the force-scaled entries take it.
    fn sparse_at(&self, scale: ForceScale) -> SparseStiffness {
        assemble_sparse_stiffness(
            self.node_count,
            &self.frames,
            &self.connectors,
            &self.blocks(),
            &self.springs,
            &SparseAssemblyOptions::new().with_force_scale(scale),
        )
        .unwrap()
    }
    fn sparse(&self) -> SparseStiffness {
        self.sparse_at(ForceScale::UNSCALED)
    }
    fn dense_evidence_at(&self, scale: ForceScale) -> AssemblyEvidence {
        AssemblyEvidence::new_force_scaled(
            self.node_count,
            &self.frames,
            &self.connectors,
            &self.slots,
            &self.springs,
            scale,
        )
        .unwrap()
    }
    fn sparse_evidence_at(&self, k: &SparseStiffness, scale: ForceScale) -> SparseAssemblyEvidence {
        SparseAssemblyEvidence::new_force_scaled(
            k.pattern(),
            self.node_count,
            &self.frames,
            &self.connectors,
            &self.slots,
            &self.springs,
            scale,
        )
        .unwrap()
    }
    fn ledger(&self) -> AssembledForce {
        let mut ledger = LoadLedger::new();
        for (index, &(dof, v)) in self.loads.iter().enumerate() {
            ledger.push(format!("load:{index}"), dof, v);
        }
        ledger.finish(6 * self.node_count).unwrap()
    }
    fn force(&self) -> Vec<f64> {
        let mut f = vec![0.0; 6 * self.node_count];
        for &(dof, v) in &self.loads {
            f[dof] = v;
        }
        f
    }
    /// W4's per-body record for the whole model (one connected body), as the
    /// selected branch at this evidence's scale forms it.
    fn w4_body(&self, sources: &[CurvedBendMacroElement], scale: ForceScale) -> W4Body {
        let e = self.dense_evidence_at(scale);
        let evidence = BodyEvidence {
            coordinates: &e.coordinates,
            edges: &e.edges,
            spring_ground: &e.spring_ground,
            w4: None,
        };
        let context = W4Context {
            formation: &e.formation,
            curved_sources: sources,
            force_scale: e.force_scale,
        };
        let all = (0..self.node_count).collect::<Vec<_>>();
        context.body(&evidence, &all, &self.prescribed)
    }
}

/// A frame a-b, a realized 90° bend b-c (R = 0.25 m, centre (o+1, o+0.25, 0);
/// dyadic, so arc-consistent in binary64) and a frame c-d; translation pins at
/// a and d only. The rotation about the line a-d is a physical mechanism:
/// W4's canonical witness is θ = (1, 1, 0) with u = θ × (x − a), exactly
/// u(a) = u(d) = 0 and u(b) = u(c) = (0, 0, −1). `rx` adds RX at a (its
/// stabilized companion).
fn curved_mechanism(o: f64, rx: bool) -> Model {
    let (a, b, c, d) = (
        [o, o, 0.0],
        [o + 1.0, o, 0.0],
        [o + 1.25, o + 0.25, 0.0],
        [o + 1.25, o + 1.25, 0.0],
    );
    let mut m = Model::empty(4);
    m.frame((0, a), (1, b), [0.0, 1.0, 0.0]);
    m.bend((1, b), (2, c), [o + 1.0, o + 0.25, 0.0]);
    m.frame((2, c), (3, d), [1.0, 0.0, 0.0]);
    let mut pins = vec![0, 1, 2, 18, 19, 20];
    if rx {
        pins.push(3);
    }
    m.pin(&pins);
    m.loads = vec![(8, 1000.0), (6, 1000.0)];
    m
}
const CURVED_MECHANISM: [[f64; 6]; 4] = [
    [0.0, 0.0, 0.0, 1.0, 1.0, 0.0],
    [0.0, 0.0, -1.0, 1.0, 1.0, 0.0],
    [0.0, 0.0, -1.0, 1.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0, 1.0, 0.0],
];

/// A lone bend (R = 0.25 m) between two translation-pinned nodes, no frame:
/// its nodes carry no coordinates in the evidence, so W4 takes them from the
/// macro source. The rotation about the chord (1, 1, 0) is free:
/// θ = (1, 1, 0), u = 0 at both nodes.
fn curved_only() -> Model {
    let mut m = Model::empty(2);
    m.bend((0, [0.0; 3]), (1, [0.25, 0.25, 0.0]), [0.0, 0.25, 0.0]);
    m.pin(&[0, 1, 2, 6, 7, 8]);
    m.loads = vec![(9, 1.0)];
    m
}
const CURVED_ONLY: [[f64; 6]; 2] = [[0.0, 0.0, 0.0, 1.0, 1.0, 0.0]; 2];

/// T4-U3: two frames joined by a positive definite connector (a link). The
/// translation pins at nodes 0 and 1 lie on the x axis and nothing else is
/// grounded, so the linked body turns about that axis: θ = (1, 0, 0) at every
/// node, u = θ × x = (0, 0, y), i.e. u_z = 1 at nodes 2 and 3 (y = 1). `rx`
/// adds RX at node 3 (restrained).
fn connector_mechanism(rx: bool) -> Model {
    let p = [
        [0.0, 0.0, 0.0],
        [2.0, 0.0, 0.0],
        [2.0, 1.0, 0.0],
        [4.0, 1.0, 0.0],
    ];
    let mut m = Model::empty(4);
    m.frame((0, p[0]), (1, p[1]), [0.0, 0.0, 1.0]);
    m.frame((2, p[2]), (3, p[3]), [0.0, 0.0, 1.0]);
    m.connectors
        .push(connector((1, p[1]), (2, p[2]), AXES_Y, PD, 0.0));
    let mut pins = vec![0, 1, 2, 6, 7, 8];
    if rx {
        pins.push(21);
    }
    m.pin(&pins);
    m.loads = vec![(14, 1000.0)];
    m
}
const CONNECTOR_MECHANISM: [[f64; 6]; 4] = [
    [0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 1.0, 0.0, 0.0],
];

fn from_kd5(m: &ModelData) -> Model {
    let s = &m.section;
    let section = FrameSection::new(s.e, s.g, s.a, s.i, s.i, s.j).unwrap();
    let mut model = Model::empty(m.nodes.len());
    for (index, member) in m.members.iter().enumerate() {
        let (i, j) = (
            node(member.i, m.nodes[member.i]),
            node(member.j, m.nodes[member.j]),
        );
        match member.bend {
            None => model
                .frames
                .push(FrameElement::new(i, j, section, member.y_reference).unwrap()),
            Some((center, factor)) => {
                let e = CurvedBendMacroElement::new(
                    i, j, center, s.e, s.g, s.a, s.i, s.j, factor, factor,
                )
                .unwrap();
                model.slots.push(
                    CurvedBendStiffnessElement::from_macro_element(format!("bend-{index}"), &e)
                        .unwrap(),
                );
                model.macros.push(e);
            }
        }
    }
    model.springs = m.springs.to_vec();
    model.loads = m.loads.iter().copied().filter(|l| l.1 != 0.0).collect();
    model.pin(m.rigid);
    model
}

/// K-D5's models with realized bends (E1, E6 and the curved controls).
fn kd5_curved() -> Vec<(&'static str, Model)> {
    [
        &E1,
        &E6,
        &CSKEW_8_5,
        &CSKEW_30_RADIUS_MISMATCH,
        &CPLANAR_60,
        &CSKEW_30_N122,
        &PP_UTM_2,
    ]
    .into_iter()
    .map(|m| (m.name, from_kd5(m)))
    .collect()
}

fn flat(motion: &[[f64; 6]]) -> Vec<f64> {
    motion.iter().flatten().copied().collect()
}

fn scale(b: i32) -> ForceScale {
    ForceScale::new(b).expect("even b")
}

// ------------------------------------------------------------------ outcomes of every entry

fn structural(
    result: Result<ForceScaledSolution, ForceScaledError>,
) -> Result<StructuralSolution, StructuralError> {
    match result {
        Ok(scaled) => Ok(scaled.solution),
        Err(ForceScaledError::Structural(error)) => Err(error),
        Err(other) => panic!("not a structural outcome: {other:?}"),
    }
}

/// The four selected branches (dense and sparse, unscaled and force-scaled)
/// at 2^b, with `sources`.
fn selected(
    m: &Model,
    mode: LinearSolveMode,
    b: i32,
    sources: &[CurvedBendMacroElement],
) -> [Result<StructuralSolution, StructuralError>; 4] {
    let s = scale(b);
    let k = m.sparse_at(s);
    let dense = k.to_dense();
    let (de, se) = (m.dense_evidence_at(s), m.sparse_evidence_at(&k, s));
    let f = m.ledger();
    let checked = if b == 0 {
        de.solve_assembled_with_formation_check(
            &dense,
            &f,
            &m.free,
            &m.prescribed,
            mode,
            sources,
            true,
        )
    } else {
        structural(de.solve_force_scaled_with_formation_check(
            &dense,
            &f,
            &m.free,
            &m.prescribed,
            mode,
            sources,
            true,
        ))
    };
    let sparse_checked = if b == 0 {
        se.solve_assembled_with_formation_check(&k, &f, &m.free, &m.prescribed, mode, sources, true)
    } else {
        structural(se.solve_force_scaled_with_formation_check(
            &k,
            &f,
            &m.free,
            &m.prescribed,
            mode,
            sources,
            true,
        ))
    };
    [
        checked,
        structural(de.solve_force_scaled_with_formation_check(
            &dense,
            &f,
            &m.free,
            &m.prescribed,
            mode,
            sources,
            true,
        )),
        sparse_checked,
        structural(se.solve_force_scaled_with_formation_check(
            &k,
            &f,
            &m.free,
            &m.prescribed,
            mode,
            sources,
            true,
        )),
    ]
}

/// Every unselected entry, unscaled: (name, outcome).
fn unselected(
    m: &Model,
    mode: LinearSolveMode,
) -> Vec<(&'static str, Result<StructuralSolution, StructuralError>)> {
    let k = m.sparse();
    let dense = k.to_dense();
    let (de, se) = (
        m.dense_evidence_at(ForceScale::UNSCALED),
        m.sparse_evidence_at(&k, ForceScale::UNSCALED),
    );
    let (f, ledger) = (m.force(), m.ledger());
    let (free, prescribed, sources) = (&m.free, &m.prescribed, &m.macros);
    vec![
        ("dense solve", de.solve(&dense, &f, free, prescribed, mode)),
        (
            "dense solve_binary64",
            de.solve_binary64(&dense, &f, free, prescribed, mode),
        ),
        (
            "dense solve_assembled",
            de.solve_assembled(&dense, &ledger, free, prescribed, mode),
        ),
        (
            "dense solve_force_scaled",
            structural(de.solve_force_scaled(&dense, &ledger, free, prescribed, mode)),
        ),
        (
            "dense checked, not selected",
            de.solve_assembled_with_formation_check(
                &dense, &ledger, free, prescribed, mode, sources, false,
            ),
        ),
        (
            "dense force-scaled checked, not selected",
            structural(de.solve_force_scaled_with_formation_check(
                &dense, &ledger, free, prescribed, mode, sources, false,
            )),
        ),
        ("sparse solve", se.solve(&k, &f, free, prescribed, mode)),
        (
            "sparse solve_assembled",
            se.solve_assembled(&k, &ledger, free, prescribed, mode),
        ),
        (
            "sparse solve_force_scaled",
            structural(se.solve_force_scaled(&k, &ledger, free, prescribed, mode)),
        ),
        (
            "sparse checked, not selected",
            se.solve_assembled_with_formation_check(
                &k, &ledger, free, prescribed, mode, sources, false,
            ),
        ),
        (
            "sparse force-scaled checked, not selected",
            structural(se.solve_force_scaled_with_formation_check(
                &k, &ledger, free, prescribed, mode, sources, false,
            )),
        ),
    ]
}

/// The selected branch's outcome is the plain entry's, up to the D-5 check's
/// demotion (quality and its record; values unchanged): the geometry is today's.
fn same_geometry(
    selected: &Result<StructuralSolution, StructuralError>,
    plain: &Result<StructuralSolution, StructuralError>,
    what: &str,
) {
    match (selected, plain) {
        (Ok(a), Ok(b)) => {
            assert_eq!(bits(&a.displacements), bits(&b.displacements), "{what}");
            let mut report = a.report.clone();
            report.quality = b.report.quality;
            assert_eq!(format!("{report:?}"), format!("{:?}", b.report), "{what}");
        }
        _ => assert_eq!(render(selected), render(plain), "{what}"),
    }
}

fn render(result: &Result<StructuralSolution, StructuralError>) -> String {
    format!("{result:?}")
}

fn is_mechanism(result: &Result<StructuralSolution, StructuralError>) -> bool {
    matches!(result, Err(StructuralError::Mechanism { .. }))
}

fn direction(result: &Result<StructuralSolution, StructuralError>) -> Vec<u64> {
    match result {
        Err(StructuralError::Mechanism { direction }) => bits(direction),
        other => panic!("not a witnessed mechanism: {other:?}"),
    }
}

// ------------------------------------------------------------------ E: the wiring

/// E (Q1(b)): on mixed-body mechanisms (a bend in a frame line, a lone bend,
/// a positive definite connector link), the four selected branches give W4's `Mechanism` with
/// the exact direction, identical in both representations; every unselected
/// entry gives the matrix gate's outcome (never a `Mechanism`: only the
/// geometric screen produces one), with each typed entry's `selected = false`
/// branch equal to its plain entry and the dense entries equal to the sparse.
#[test]
fn k5_w4_runs_in_the_four_selected_branches_only() {
    let cases = [
        (
            "curved mechanism at 0",
            curved_mechanism(0.0, false),
            flat(&CURVED_MECHANISM),
        ),
        (
            "curved mechanism at 5e6",
            curved_mechanism(5.0e6, false),
            flat(&CURVED_MECHANISM),
        ),
        ("lone bend", curved_only(), flat(&CURVED_ONLY)),
        (
            "connector link",
            connector_mechanism(false),
            flat(&CONNECTOR_MECHANISM),
        ),
    ];
    for (name, m, expected) in cases {
        for mode in MODES {
            for result in selected(&m, mode, 0, &m.macros) {
                assert_eq!(direction(&result), bits(&expected), "{name} {mode:?}");
            }
            let outcomes = unselected(&m, mode);
            for (entry, result) in &outcomes {
                assert!(!is_mechanism(result), "{name} {mode:?} {entry}: {result:?}");
            }
            let get = |entry: &str| render(&outcomes.iter().find(|o| o.0 == entry).unwrap().1);
            for side in ["dense", "sparse"] {
                assert_eq!(
                    get(&format!("{side} checked, not selected")),
                    get(&format!("{side} solve_assembled")),
                    "{name} {mode:?}"
                );
                assert_eq!(
                    get(&format!("{side} force-scaled checked, not selected")),
                    get(&format!("{side} solve_force_scaled")),
                    "{name} {mode:?}"
                );
            }
            for entry in [
                "solve_assembled",
                "solve_force_scaled",
                "checked, not selected",
            ] {
                assert_eq!(
                    get(&format!("dense {entry}")),
                    get(&format!("sparse {entry}")),
                    "{name} {mode:?} {entry}"
                );
            }
        }
    }
}

/// E (Q1(b)), the loop: with an open gap support the invocation is never
/// selected, and the nonlinear loop keeps today's frame-only geometry. Its
/// first solve is the named binary64 variant's, bit for bit (precondition:
/// the selected branches refuse this linear system as a W4 `Mechanism`).
#[test]
fn k5_nonlinear_loop_keeps_todays_geometry() {
    let m = curved_mechanism(0.0, false);
    let dense = m.sparse().to_dense();
    for mode in MODES {
        assert!(is_mechanism(&selected(&m, mode, 0, &m.macros)[0]));
        let input = NonlinearFrameSolveInput {
            node_count: m.node_count,
            elements: m.frames.clone(),
            connectors: Vec::new(),
            curved_bend_elements: m.slots.clone(),
            force: m.force(),
            base_restrained_dofs: m.prescribed.iter().map(|p| p.0).collect(),
            nonlinear_supports: vec![NonlinearSupport::gap(
                "k5-open-gap",
                1,
                FrameDof::Uz,
                1.0,
                GapDirection::PositiveDisplacement,
            )
            .unwrap()],
            initial_states: vec![SupportStateRecord::new(
                "k5-open-gap",
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
        };
        let looped = solve_active_set_frame_with_mode_and_springs(&input, mode, &m.springs);
        let binary64 = m.dense_evidence_at(ForceScale::UNSCALED).solve_binary64(
            &dense,
            &m.force(),
            &m.free,
            &m.prescribed,
            mode,
        );
        assert!(!is_mechanism(&binary64), "{mode:?}");
        match (&looped, &binary64) {
            (Err(NonlinearIntegrationError::Structural(error)), Err(expected)) => {
                assert!(!matches!(error, StructuralError::Mechanism { .. }));
                assert_eq!(error, expected, "{mode:?}");
            }
            (Ok(result), Ok(expected)) => {
                let first = result.iterations.first().expect("iteration");
                assert_eq!(
                    bits(&first.displacements),
                    bits(&expected.displacements),
                    "{mode:?}"
                );
                assert_eq!(
                    format!("{:?}", first.structural_report),
                    format!("{:?}", expected.report)
                );
            }
            (looped, binary64) => panic!("{mode:?}: loop {looped:?} against {binary64:?}"),
        }
    }
}

/// E: the first failing body in seed order decides, as today. A mixed
/// mechanism seeded first gives W4's direction; a frame-only mechanism seeded
/// first gives today's frame direction (the unchanged `geometry`'s bits).
#[test]
fn k5_first_failing_body_in_seed_order_decides() {
    let line = |first: usize| {
        let mut m = Model::empty(2);
        m.frame(
            (first, [10.0, 10.0, 0.0]),
            (first + 1, [13.0, 14.0, 0.0]),
            [0.0, 0.0, 1.0],
        );
        m
    };
    let join = |a: &Model, b: &Model, pins: &[usize]| {
        let mut m = Model::empty(a.node_count + b.node_count);
        m.frames = [a.frames.clone(), b.frames.clone()].concat();
        m.macros = [a.macros.clone(), b.macros.clone()].concat();
        m.slots = [a.slots.clone(), b.slots.clone()].concat();
        m.loads = [a.loads.clone(), b.loads.clone()].concat();
        m.pin(pins);
        m
    };
    // Mixed body at nodes 0..3, the frame line (pinned in translation at
    // both ends: rotation about its axis free) at nodes 4 and 5.
    let mixed_first = join(
        &curved_mechanism(0.0, false),
        &line(4),
        &[0, 1, 2, 18, 19, 20, 24, 25, 26, 30, 31, 32],
    );
    let mut expected = flat(&CURVED_MECHANISM);
    expected.extend([0.0; 12]);
    // The frame line at nodes 0 and 1, the mixed body shifted to 2..5.
    let mut shifted = Model::empty(4);
    let (a, b, c, d) = (
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.25, 0.25, 0.0],
        [1.25, 1.25, 0.0],
    );
    shifted.frame((2, a), (3, b), [0.0, 1.0, 0.0]);
    shifted.bend((3, b), (4, c), [1.0, 0.25, 0.0]);
    shifted.frame((4, c), (5, d), [1.0, 0.0, 0.0]);
    let frame_first = join(
        &line(0),
        &shifted,
        &[0, 1, 2, 6, 7, 8, 12, 13, 14, 30, 31, 32],
    );
    for mode in MODES {
        for result in selected(&mixed_first, mode, 0, &mixed_first.macros) {
            assert_eq!(direction(&result), bits(&expected), "{mode:?}");
        }
        // Today's geometry skips the mixed body and witnesses the frame line.
        let today = mixed_first
            .dense_evidence_at(ForceScale::UNSCALED)
            .geometry(&mixed_first.prescribed);
        assert!(matches!(today, Err(StructuralError::Mechanism { .. })));
        let frame = match frame_first
            .dense_evidence_at(ForceScale::UNSCALED)
            .geometry(&frame_first.prescribed)
        {
            Err(StructuralError::Mechanism { direction }) => bits(&direction),
            other => panic!("frame line: {other:?}"),
        };
        for result in selected(&frame_first, mode, 0, &frame_first.macros) {
            assert_eq!(direction(&result), frame, "{mode:?}");
        }
    }
}

/// Q1(b): a body of frames only keeps today's screen in the selected
/// branches, with today's witness and not W4's. A free translation separates
/// the two. Today's witness moves each node by the characteristic length
/// max |x − o| = hypot(4, 0) = 4 (exact: IEEE hypot(x, ±0) = |x|). W4's
/// canonical representative moves each node by k = 1 (checked below on the
/// same body).
#[test]
fn k5_frame_only_bodies_keep_todays_witness() {
    let (a, b) = ([10.0, 10.0, 0.0], [14.0, 10.0, 0.0]);
    let mut m = Model::empty(2);
    m.frame((0, a), (1, b), [0.0, 0.0, 1.0]);
    // Node 0 held in uy, uz and every rotation: only the x translation is free.
    m.pin(&[1, 2, 3, 4, 5]);
    let expected = bits(&flat(&[[4.0, 0.0, 0.0, 0.0, 0.0, 0.0]; 2]));
    match m
        .dense_evidence_at(ForceScale::UNSCALED)
        .geometry(&m.prescribed)
    {
        Err(StructuralError::Mechanism { direction }) => assert_eq!(bits(&direction), expected),
        other => panic!("frame line: {other:?}"),
    }
    let grounds = [1, 2, 3, 4, 5].map(ConstrainedGround::Dof);
    let w4 = assess_constrained_bodies(&[a, b], &[vec![0, 1]], &[], &grounds).unwrap();
    assert_eq!(w4.status, RigidBodyStatus::MechanismWitnessed);
    assert_eq!(w4.node_motion.unwrap(), [[1.0, 0.0, 0.0, 0.0, 0.0, 0.0]; 2]);
    for mode in MODES {
        for result in selected(&m, mode, 0, &m.macros) {
            assert_eq!(direction(&result), expected, "{mode:?}");
        }
    }
}

/// Lexed source with comments removed and literal contents blanked
/// (lifetimes kept).
fn lex(src: &str) -> String {
    let b: Vec<char> = src.chars().collect();
    let n = b.len();
    let mut out = String::with_capacity(n);
    let mut i = 0;
    while i < n {
        let c = b[i];
        if c == '/' && i + 1 < n && b[i + 1] == '/' {
            while i < n && b[i] != '\n' {
                i += 1;
            }
            continue;
        }
        if c == '/' && i + 1 < n && b[i + 1] == '*' {
            let mut depth = 1;
            i += 2;
            while i < n && depth > 0 {
                if b[i] == '/' && i + 1 < n && b[i + 1] == '*' {
                    depth += 1;
                    i += 2;
                } else if b[i] == '*' && i + 1 < n && b[i + 1] == '/' {
                    depth -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            continue;
        }
        if c == '"' {
            out.push('"');
            i += 1;
            while i < n && b[i] != '"' {
                if b[i] == '\\' {
                    i += 1;
                }
                i += 1;
            }
            out.push('"');
            i += 1;
            continue;
        }
        if c == '\'' && i + 1 < n && b[i + 1] == '\\' {
            i += 2;
            while i < n && b[i] != '\'' {
                i += 1;
            }
            out.push_str("' '");
            i += 1;
            continue;
        }
        if c == '\'' && i + 2 < n && b[i + 2] == '\'' {
            out.push_str("' '");
            i += 3;
            continue;
        }
        out.push(c);
        i += 1;
    }
    out
}

/// The byte range of the `{…}` block that starts at the first `{` at or
/// after `from`.
fn block(code: &str, from: usize) -> std::ops::Range<usize> {
    let open = from + code[from..].find('{').unwrap();
    let mut depth = 0;
    for (i, c) in code[open..].char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return open..open + i + 1;
                }
            }
            _ => {}
        }
    }
    panic!("unbalanced");
}

/// `code` without `#[cfg(test)]` items (`mod x;` or a braced item).
fn strip_cfg_test(code: &str) -> String {
    let mut code = code.to_string();
    while let Some(at) = code.find("#[cfg(test)]") {
        let semi = code[at..].find(';').map(|k| at + k);
        let brace = code[at..].find('{').map(|k| at + k);
        let end = match (semi, brace) {
            (Some(s), Some(b)) if s < b => s + 1,
            (_, Some(b)) => block(&code, b).end,
            (Some(s), None) => s + 1,
            (None, None) => code.len(),
        };
        code.replace_range(at..end, "");
    }
    code
}

fn tokens(code: &str, token: &str) -> usize {
    code.match_indices(token)
        .filter(|(at, _)| {
            let before = code[..*at].chars().next_back();
            let after = code[at + token.len()..].chars().next();
            !before.is_some_and(|c| c.is_alphanumeric() || c == '_')
                && !after.is_some_and(|c| c.is_alphanumeric() || c == '_')
        })
        .count()
}

/// The body of `fn <name>(` inside the impl block that starts with `header`.
fn body_in_impl<'c>(code: &'c str, header: &str, name: &str) -> &'c str {
    let at = code.find(header).unwrap_or_else(|| panic!("{header}"));
    let imp = block(code, at);
    let within = &code[imp.clone()];
    let signature = format!("fn {name}(");
    assert_eq!(within.matches(&signature).count(), 1, "{header} {name}");
    let f = within.find(&signature).unwrap();
    &within[block(within, f)]
}

/// E (Q1(b)): the W4 entry `constrained_geometry` is defined once in each
/// evidence's impl and called exactly once in each of the four selected
/// bodies, after their `if !selected` return; every other entry calls
/// today's `geometry`; the loop's and the product's sources never name W4.
#[test]
fn k5_w4_entry_is_named_only_in_the_four_selected_bodies() {
    let control = strip_cfg_test(&lex("// constrained_geometry(\nlet s = \"constrained_geometry(\";\n#[cfg(test)]\nmod t { fn f() { constrained_geometry(); } }\nfn g() { x.constrained_geometry(a); }"));
    assert_eq!(tokens(&control, "constrained_geometry"), 1);
    let code = strip_cfg_test(&lex(include_str!("../structural_adapter.rs")));
    assert_eq!(tokens(&code, "constrained_geometry"), 6);
    assert_eq!(code.matches("fn constrained_geometry(").count(), 2);
    for header in ["impl AssemblyEvidence {", "impl SparseAssemblyEvidence {"] {
        assert_eq!(
            tokens(
                body_in_impl(&code, header, "constrained_geometry"),
                "constrained_geometry"
            ),
            0
        );
        for name in [
            "solve_assembled_with_formation_check",
            "solve_force_scaled_with_formation_check",
        ] {
            let body = body_in_impl(&code, header, name);
            assert_eq!(tokens(body, "constrained_geometry"), 1, "{header} {name}");
            assert_eq!(tokens(body, "geometry"), 0, "{header} {name}");
            let call = body.find("constrained_geometry(").unwrap();
            assert!(body.find("if !selected").unwrap() < call, "{header} {name}");
        }
        for name in ["solve", "solve_assembled", "solve_force_scaled"] {
            let body = body_in_impl(&code, header, name);
            assert_eq!(tokens(body, "constrained_geometry"), 0, "{header} {name}");
            assert_eq!(
                body.matches("self.geometry(prescribed)").count(),
                1,
                "{header} {name}"
            );
        }
    }
    let binary64 = body_in_impl(&code, "impl AssemblyEvidence {", "solve_binary64");
    assert_eq!(tokens(binary64, "constrained_geometry"), 0);
    assert_eq!(binary64.matches("self.geometry(prescribed)").count(), 1);
    // The loop and the product never name W4.
    let lib = strip_cfg_test(&lex(include_str!("../lib.rs")));
    let product =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../product_physics/src");
    let mut sources = vec![("NI lib.rs".to_string(), lib)];
    let mut stack = vec![product];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                let text = std::fs::read_to_string(&path).unwrap();
                sources.push((path.display().to_string(), lex(&text)));
            }
        }
    }
    assert!(sources.len() > 10);
    for (name, text) in &sources {
        for token in [
            "constrained_geometry",
            "assess_constrained_bodies",
            "W4Context",
            "W4Body",
        ] {
            assert_eq!(tokens(text, token), 0, "{name} names {token}");
        }
    }
}


// ------------------------------------------------------------------ D: curved elements

/// D (Q2(b)): a curved slot joins its sub-body only when it is matched to its
/// macro source (K-D5's predicate) with consistent node coordinates. An
/// explicit slot, an unmatched slot (no sources, or one ulp off) and a
/// matched source whose node disagrees with the frame's by one ulp leave the
/// body unqualified, and the outcome is today's.
#[test]
fn k5_curved_slots_qualify_by_their_matched_source() {
    let m = curved_mechanism(0.0, false);
    assert!(matches!(
        m.w4_body(&m.macros, ForceScale::UNSCALED),
        W4Body::Assessed { .. }
    ));
    let id = || "bend-0".to_string();
    let mut explicit = m.clone();
    let slot = &explicit.slots[0];
    explicit.slots[0] = CurvedBendStiffnessElement::new(
        slot.element_id.clone(),
        slot.node_i,
        slot.node_j,
        slot.global_stiffness,
    )
    .unwrap();
    let mut ulp = m.clone();
    let entry = &mut ulp.slots[0].global_stiffness[0][0];
    *entry = f64::from_bits(entry.to_bits() + 1);
    let mut moved = Model::empty(4);
    let (a, b, c, d) = (
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.25, 0.25, 0.0],
        [1.25, 1.25, 0.0],
    );
    let c_off = [f64::from_bits(1.25_f64.to_bits() + 1), 0.25, 0.0];
    moved.frame((0, a), (1, b), [0.0, 1.0, 0.0]);
    moved.bend((1, b), (2, c_off), [1.0, 0.25, 0.0]);
    moved.frame((2, c), (3, d), [1.0, 0.0, 0.0]);
    moved.pin(&[0, 1, 2, 18, 19, 20]);
    moved.loads = m.loads.clone();
    let cases: [(&str, &Model, &[CurvedBendMacroElement], W4Unqualified); 4] = [
        (
            "explicit",
            &explicit,
            &m.macros,
            W4Unqualified::CurvedExplicit { element_id: id() },
        ),
        (
            "no sources",
            &m,
            &[],
            W4Unqualified::CurvedUnmatched { element_id: id() },
        ),
        (
            "one ulp",
            &ulp,
            &m.macros,
            W4Unqualified::CurvedUnmatched { element_id: id() },
        ),
        (
            "inconsistent",
            &moved,
            &moved.macros,
            W4Unqualified::CurvedCoordinates {
                element_id: id(),
                node: 2,
            },
        ),
    ];
    for (name, model, sources, reason) in cases {
        assert_eq!(
            model.w4_body(sources, ForceScale::UNSCALED),
            W4Body::Unqualified(reason),
            "{name}"
        );
        for mode in MODES {
            let plain = unselected(model, mode);
            let today = &plain
                .iter()
                .find(|o| o.0 == "sparse solve_assembled")
                .unwrap()
                .1;
            let [_, _, checked, _] = selected(model, mode, 0, sources);
            assert!(!is_mechanism(&checked), "{name} {mode:?}");
            same_geometry(&checked, today, &format!("{name} {mode:?}"));
        }
    }
    // The lone bend takes its coordinates from the macro source: the
    // evidence records none for its nodes.
    let lone = curved_only();
    assert_eq!(
        lone.dense_evidence_at(ForceScale::UNSCALED).coordinates,
        vec![None, None]
    );
}

/// D: W4's match agrees with K-D5's `formation_source` (and, at 2^b, with
/// `force_scaled_formation_source`) for every curved slot of the corpus.
#[test]
fn k5_curved_matching_agrees_with_the_formation_source() {
    let mut models = kd5_curved();
    models.push(("curved mechanism", curved_mechanism(0.0, false)));
    models.push(("lone bend", curved_only()));
    for (name, m) in &models {
        for (b, sources) in [
            (0, &m.macros[..]),
            (0, &[][..]),
            (2, &m.macros[..]),
            (-40, &m.macros[..]),
        ] {
            let e = m.dense_evidence_at(scale(b));
            let source = force_scaled_formation_source(&e.formation, sources, e.force_scale);
            for slot in &e.formation.curved {
                let w4 = w4_curved_source(slot, sources, e.force_scale).is_some() && !slot.explicit;
                let kd5 = !source
                    .unavailable
                    .iter()
                    .any(|u| u.ends_with(&format!(":{}", slot.element_id)));
                assert_eq!(w4, kd5, "{name} b={b} {}", slot.element_id);
            }
        }
    }
}

/// D (2^b): the force-scaled selected branches give W4's outcome at b ≠ 0
/// with the bits of b = 0 (slots matched at 2^b; coordinates unscaled).
#[test]
fn k5_force_scaled_branches_give_the_same_witness() {
    for (m, expected) in [
        (curved_mechanism(0.0, false), flat(&CURVED_MECHANISM)),
        (curved_only(), flat(&CURVED_ONLY)),
        (connector_mechanism(false), flat(&CONNECTOR_MECHANISM)),
    ] {
        for mode in MODES {
            for b in [2, -2, 40, -40] {
                let [_, dense, _, sparse] = selected(&m, mode, b, &m.macros);
                assert_eq!(direction(&dense), bits(&expected), "b={b} {mode:?}");
                assert_eq!(direction(&sparse), bits(&expected), "b={b} {mode:?}");
            }
        }
        for b in [2, -40] {
            let W4Body::Assessed { assessment, .. } = m.w4_body(&m.macros, scale(b)) else {
                panic!("matched at 2^{b}");
            };
            assert_eq!(assessment.status, RigidBodyStatus::MechanismWitnessed);
        }
    }
}

/// The screen ratio of one curved slot (Q2(b): recorded evidence only, never
/// a gate): max over the six rigid vectors r_k at the element's nodes and the
/// twelve rows of |K_e r_k| (exact) over Σ_j bound·|r_kj| (binary64).
fn screen_ratio(slot: &CurvedBendStiffnessElement, source: &CurvedBendMacroElement) -> f64 {
    let bounds = &slot.symmetry_formation.as_ref().unwrap().0;
    let k = &slot.global_stiffness;
    let (xi, xj) = (source.node_i.coordinates, source.node_j.coordinates);
    let mut worst = 0.0_f64;
    for mode in 0..6 {
        // Each entry of r_k: (sign, axis) terms of x_j − x_i, or a unit.
        let entry = |col: usize| -> Vec<(f64, f64)> {
            if mode < 3 {
                return if col % 6 == mode {
                    vec![(1.0, 1.0)]
                } else {
                    vec![]
                };
            }
            let axis = mode - 3;
            match col {
                3..=5 | 9..=11 => {
                    if col % 3 == axis {
                        vec![(1.0, 1.0)]
                    } else {
                        vec![]
                    }
                }
                6..=8 => {
                    // (e_axis × d)_m
                    let m = col - 6;
                    let (sign, c) = match (axis, m) {
                        (0, 1) => (-1.0, 2),
                        (0, 2) => (1.0, 1),
                        (1, 0) => (1.0, 2),
                        (1, 2) => (-1.0, 0),
                        (2, 0) => (-1.0, 1),
                        (2, 1) => (1.0, 0),
                        _ => return vec![],
                    };
                    vec![(sign, xj[c]), (-sign, xi[c])]
                }
                _ => vec![],
            }
        };
        let approx = |col: usize| -> f64 {
            entry(col)
                .iter()
                .map(|&(s, v)| s * v)
                .fold(0.0, |a, x| a + x)
        };
        for row in 0..12 {
            let mut exact = ExactAccumulator::new();
            let mut allowance = 0.0_f64;
            for col in 0..12 {
                for (sign, value) in entry(col) {
                    exact.add_product(sign * k[row][col], value).unwrap();
                }
                allowance += bounds[row][col] * approx(col).abs();
            }
            let action = exact.round().unwrap().abs();
            if action > 0.0 {
                worst = worst.max(if allowance > 0.0 {
                    action / allowance
                } else {
                    f64::INFINITY
                });
            }
        }
    }
    worst
}

/// D: the screen's ratio per element on the curved corpus (an observation,
/// recorded with `--nocapture`; no assertion on a product-formed element).
#[test]
fn k5_screen_ratio_is_recorded_per_element() {
    let mut models = kd5_curved();
    models.push(("curved mechanism at 0", curved_mechanism(0.0, false)));
    models.push(("curved mechanism at 1024", curved_mechanism(1024.0, false)));
    models.push(("curved mechanism at 5e6", curved_mechanism(5.0e6, false)));
    models.push(("lone bend", curved_only()));
    for (name, m) in models {
        for (slot, source) in m.slots.iter().zip(&m.macros) {
            let ratio = screen_ratio(slot, source);
            assert!(ratio.is_finite() || ratio == f64::INFINITY);
            eprintln!("k5 screen ratio: {name} {} {ratio:e}", slot.element_id);
        }
    }
}

// ------------------------------------------------------------------ A: nothing existing moves

/// A (Q7(a)): the mixed family's basis text is unchanged on the selected
/// branches (W4 under-claims there; recorded, T3-close and F2a); the
/// qualified-family flag is unchanged for mixed evidence; a restrained mixed
/// body's selected outcome is its plain one.
#[test]
fn k5_basis_text_and_family_flag_are_unchanged() {
    const MIXED: &str = "mixed or explicit-matrix family: physical rigid-null witness unqualified for bodies containing user/curved elements; matrix positivity remains mandatory";
    for (name, m) in [
        ("curved companion", curved_mechanism(0.0, true)),
        ("connector companion", connector_mechanism(true)),
    ] {
        let e = m.dense_evidence_at(ForceScale::UNSCALED);
        assert!(!e.qualified_passive_family(), "{name}");
        let W4Body::Assessed { assessment, .. } = m.w4_body(&m.macros, ForceScale::UNSCALED) else {
            panic!("{name}");
        };
        assert_eq!(assessment.status, RigidBodyStatus::Restrained, "{name}");
        for mode in MODES {
            let plain = unselected(&m, mode);
            let today = &plain
                .iter()
                .find(|o| o.0 == "dense solve_assembled")
                .unwrap()
                .1;
            for result in selected(&m, mode, 0, &m.macros) {
                let solution = result.as_ref().unwrap_or_else(|e| panic!("{name}: {e:?}"));
                assert!(solution
                    .report
                    .symmetry_basis
                    .as_deref()
                    .unwrap()
                    .ends_with(MIXED));
                if solution.formation_check.is_none() {
                    assert_eq!(render(&result), render(today), "{name} {mode:?}");
                }
            }
        }
    }
}

// ------------------------------------------------------------------ RV14's review (the addendum)

/// Each selected entry against its own unselected sibling (the same entry with
/// `selected = false`): the matrix gate's outcome, as today.
fn selected_is_unselected(m: &Model, sources: &[CurvedBendMacroElement], what: &str) {
    for mode in MODES {
        let plain = unselected(m, mode);
        let sibling = |name: &str| &plain.iter().find(|o| o.0 == name).unwrap().1;
        let names = [
            "dense checked, not selected",
            "dense force-scaled checked, not selected",
            "sparse checked, not selected",
            "sparse force-scaled checked, not selected",
        ];
        for (result, name) in selected(m, mode, 0, sources).iter().zip(names) {
            assert!(!is_mechanism(result), "{what} {mode:?} {name}");
            same_geometry(result, sibling(name), &format!("{what} {mode:?} {name}"));
        }
    }
}

/// RV14-2 (Q2(b), RETURN §2.3 Step 2): two matched bends meet at node 1, which
/// no frame or user element touches, and their macro sources disagree on it:
/// A ends at (0.25, 0.25, 0), B starts at (0.5, 0.25, 0). W4 takes a
/// curved-only node's coordinates from its matched sources, so they must agree
/// with each other, not only with a frame. The body is unqualified
/// (`CurvedCoordinates` at node 1, reported by the second slot), and every
/// selected entry gives the matrix gate's outcome, never a `Mechanism`.
/// Translation pins at nodes 0 and 2.
#[test]
fn k5_curved_sources_agree_at_a_curved_only_node() {
    let mut m = Model::empty(3);
    m.bend(
        (0, [0.0, 0.0, 0.0]),
        (1, [0.25, 0.25, 0.0]),
        [0.0, 0.25, 0.0],
    );
    m.bend(
        (1, [0.5, 0.25, 0.0]),
        (2, [0.75, 0.5, 0.0]),
        [0.5, 0.5, 0.0],
    );
    m.pin(&[0, 1, 2, 12, 13, 14]);
    m.loads = vec![(8, 1.0)];
    assert_eq!(
        m.dense_evidence_at(ForceScale::UNSCALED).coordinates,
        vec![None, None, None]
    );
    assert_eq!(
        m.w4_body(&m.macros, ForceScale::UNSCALED),
        W4Body::Unqualified(W4Unqualified::CurvedCoordinates {
            element_id: "bend-1".to_string(),
            node: 1,
        })
    );
    selected_is_unselected(&m, &m.macros, "curved sources disagree");
}

/// RV14-N2: a positive spring is a W4 ground, as it is for frames. The curved
/// mechanism held about the line a-d by an RX spring of 1e6 at a (in place of
/// K5's RX pin) is `Restrained` for W4, and every selected entry publishes as
/// its unselected sibling.
#[test]
fn k5_positive_springs_ground_w4_bodies() {
    let mut m = curved_mechanism(0.0, false);
    m.springs = vec![(3, 1.0e6)];
    match m.w4_body(&m.macros, ForceScale::UNSCALED) {
        W4Body::Assessed { assessment, .. } => {
            assert_eq!(assessment.status, RigidBodyStatus::Restrained)
        }
        other => panic!("spring-held curved line: {other:?}"),
    }
    selected_is_unselected(&m, &m.macros, "spring-held curved line");
}

// ------------------------------------------------------------------ T4-U3: connectors (S11)

/// A connector between two nodes with no offsets, Q.x along the chord
/// (`axes`, columns are axes), and the stiffness diag(`k`) at Ls = 1 m, with
/// `coupling` at (0, 3) for a coupled law.
fn connector(
    i: (usize, [f64; 3]),
    j: (usize, [f64; 3]),
    axes: [[f64; 3]; 3],
    k: [f64; 6],
    coupling: f64,
) -> ObjectiveConnector {
    let mut h = [0.0; 21];
    for (d, index) in [0, 6, 11, 15, 18, 20].into_iter().enumerate() {
        h[index] = k[d];
    }
    h[3] = coupling;
    ObjectiveConnector::new(
        node(i.0, i.1),
        node(j.0, j.1),
        ConnectorAttachment::global([0.0; 3]),
        ConnectorAttachment::global([0.0; 3]),
        axes,
        ScaledWorkMatrix {
            upper_triangle: h,
            translation_scale: 1.0,
        },
        [0.0; 6],
    )
    .unwrap()
}

const AXES_X: [[f64; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
/// Columns x = (0, 1, 0), y = (−1, 0, 0), z = (0, 0, 1).
const AXES_Y: [[f64; 3]; 3] = [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]];
const PD: [f64; 6] = [2.0e5, 8.0e4, 1.2e5, 600.0, 900.0, 1200.0];

/// Frames a-b and c-d joined by a connector b-c on the x axis; a anchored,
/// a transverse load at d.
fn connected_line(k: [f64; 6], coupling: f64) -> Model {
    let p = [
        [0.0, 0.0, 0.0],
        [2.0, 0.0, 0.0],
        [2.5, 0.0, 0.0],
        [4.5, 0.0, 0.0],
    ];
    let mut m = Model::empty(4);
    m.frame((0, p[0]), (1, p[1]), [0.0, 0.0, 1.0]);
    m.frame((2, p[2]), (3, p[3]), [0.0, 0.0, 1.0]);
    m.connectors
        .push(connector((1, p[1]), (2, p[2]), AXES_X, k, coupling));
    m.pin(&[0, 1, 2, 3, 4, 5]);
    m.loads = vec![(19, 1000.0), (20, -500.0)];
    m
}

/// S11: a positive definite connector links its ends: the body is assessed
/// and restrained, and every selected branch passes.
#[test]
fn k5_positive_definite_connector_links() {
    let m = connected_line(PD, 0.0);
    let W4Body::Assessed { nodes, assessment } = m.w4_body(&m.macros, ForceScale::UNSCALED) else {
        panic!("a positive definite connector links");
    };
    assert_eq!(nodes, vec![0, 1, 2, 3]);
    assert_eq!(assessment.status, RigidBodyStatus::Restrained);
    for mode in MODES {
        for b in [0, 20] {
            for result in selected(&m, mode, b, &m.macros) {
                assert!(result.is_ok(), "{mode:?} b = {b}: {}", render(&result));
            }
        }
    }
}

/// S11: the frames of the deleted tie case (FK npc_internal), joined by a positive definite
/// connector instead of the old tie, are restrained: the objective link
/// carries the rigid-body moment the tie did not (the virtual pins of the
/// tie reduction are gone).
#[test]
fn k5_connector_link_is_objective_where_the_old_tie_was_a_mechanism() {
    let p = [
        [0.0, 0.0, 0.0],
        [2.0, 0.0, 0.0],
        [2.0, 1.0, 0.0],
        [4.0, 1.0, 0.0],
    ];
    let mut m = Model::empty(4);
    m.frame((0, p[0]), (1, p[1]), [0.0, 0.0, 1.0]);
    m.frame((2, p[2]), (3, p[3]), [0.0, 0.0, 1.0]);
    m.connectors
        .push(connector((1, p[1]), (2, p[2]), AXES_Y, PD, 0.0));
    m.pin(&[0, 1, 2, 6, 7, 8, 18, 19, 20]);
    m.loads = vec![(14, 1000.0)];
    let W4Body::Assessed { assessment, .. } = m.w4_body(&m.macros, ForceScale::UNSCALED) else {
        panic!("linked");
    };
    assert_eq!(assessment.status, RigidBodyStatus::Restrained);
    for mode in MODES {
        for result in selected(&m, mode, 0, &m.macros) {
            assert!(result.is_ok(), "{mode:?}: {}", render(&result));
        }
    }
}

/// S11 (S-5(c)): a semidefinite connector leaves its body to the matrix gate
/// (`ConnectorSemidefinite`). The hinge (no stiffness about the local z) is
/// a real mechanism of an otherwise anchored body: a "semidefinite as link"
/// rule would assess that body as restrained; here no branch passes it.
#[test]
fn k5_semidefinite_connectors_are_unqualified_and_the_hinge_is_not_passed() {
    let coupled = connected_line([4.0, 0.0, 0.0, 9.0, 0.0, 0.0], 1.0);
    assert_eq!(
        coupled.connectors[0].definiteness(),
        ConnectorDefiniteness::PositiveSemidefinite
    );
    assert_eq!(
        coupled.w4_body(&coupled.macros, ForceScale::UNSCALED),
        W4Body::Unqualified(W4Unqualified::ConnectorSemidefinite { element: 0 })
    );
    let mut hinge = Model::empty(2);
    hinge.connectors.push(connector(
        (0, [0.0, 0.0, 0.0]),
        (1, [0.5, 0.0, 0.0]),
        AXES_X,
        [2.0e5, 8.0e4, 1.2e5, 600.0, 900.0, 0.0],
        0.0,
    ));
    hinge.pin(&[0, 1, 2, 3, 4, 5]);
    hinge.loads = vec![(7, 100.0), (11, 10.0)];
    assert_eq!(
        hinge.w4_body(&hinge.macros, ForceScale::UNSCALED),
        W4Body::Unqualified(W4Unqualified::ConnectorSemidefinite { element: 0 })
    );
    // The hinge's mechanism is real: d = (θ_j = e_z, u_j = e_z × r/2) has
    // q = 0 except q_r,z, on which K is zero.
    let mut d = [0.0; 12];
    d[11] = 1.0;
    d[7] = 0.25;
    let q = hinge.connectors[0].recover(&d).unwrap();
    assert_eq!(q.g, [0.0; 6]);
    assert_eq!(q.q, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0]);
    for m in [&coupled, &hinge] {
        for mode in MODES {
            for result in selected(m, mode, 0, &m.macros) {
                assert!(result.is_err(), "{mode:?}: {}", render(&result));
            }
        }
    }
}

/// S5: the nonlinear loop refuses a connector (fail closed until T5).
#[test]
fn k5_nonlinear_loop_refuses_connectors() {
    let m = connected_line(PD, 0.0);
    let input = NonlinearFrameSolveInput {
        node_count: m.node_count,
        elements: m.frames.clone(),
        connectors: m.connectors.clone(),
        curved_bend_elements: vec![],
        force: m.force(),
        base_restrained_dofs: m.prescribed.iter().map(|p| p.0).collect(),
        nonlinear_supports: vec![],
        initial_states: vec![],
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
    };
    for mode in MODES {
        assert!(matches!(
            solve_active_set_frame_with_mode_and_springs(&input, mode, &m.springs),
            Err(NonlinearIntegrationError::InvalidInput { .. })
        ));
    }
}

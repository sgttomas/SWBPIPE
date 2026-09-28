//! K2b adapter tests (T3 D1 revision 5a.2 §4.7, W2 force-radix scaling and the
//! kernel half of formation-time scaling; ROOT's K2b rulings of 2026-09-28):
//! the force-scaled evidence and entries in both representations and both
//! modes, the orchestrator `solve_with_force_scaling` (the b-rule's steps
//! 1-5), the formation-range cases solved at kernel level, and the
//! interactions with K-D5, S11-K, K1 and K2a.
//!
//! Invented inputs only: K-D5's generated `kd5_models.rs` (read-only), K2b's
//! generated `k2b_models.rs` (R1's frozen RF-RANGE LEF-large references, the
//! K2a product-reach constants and a synthetic PHYS-R4 element, with exact
//! references and the independent generator's b), and the models stated here.
//! Comparisons are of `Debug` bytes or bits, or, against the references, the
//! protected 1e-9 criterion |observed - expected| <= 1e-9 max(|expected|, scale).
use super::kd5_tests::{bits, MemberData, ModelData, SectionData, MODES};
use super::*;
use crate::{
    solve_active_set_frame_with_mode_and_springs, ConvergenceControl, ConvergencePolicyStatus,
    NonlinearFrameSolveInput,
};
use open_pipe_stress_frame_kernel::load_ledger::{ForceTermKind, Formation, LoadLedger};
use open_pipe_stress_frame_kernel::structural::{
    unscale_for_publication, FormationCheckReason, PublishedValue, RecordOutcome,
    RecordRepresentability, Representability, SolveQuality,
};
use open_pipe_stress_frame_kernel::{FrameDof, FrameNode, FrameSection};
use open_pipe_stress_nonlinear_supports::{
    ActiveSetState, GapDirection, NonlinearSupport, SupportStateRecord,
};
use std::f64::consts::PI;

#[path = "kd5_models.rs"]
mod kd5_models;
use kd5_models::*;

#[path = "k2b_models.rs"]
mod k2b_models;
use k2b_models::*;

const KD5: [&ModelData; 13] = [
    &F122,
    &F345,
    &PROBE_C,
    &PROBE_D,
    &BENDING_SOFT,
    &E1,
    &E6,
    &CSKEW_8_5,
    &CSKEW_30_RADIUS_MISMATCH,
    &M11,
    &CPLANAR_60,
    &CSKEW_30_N122,
    &PP_UTM_2,
];

const REPRESENTATIONS: [EvidenceRepresentation; 2] = [
    EvidenceRepresentation::Dense,
    EvidenceRepresentation::Pattern,
];

/// The forced scales of the exactness tests (even; ROOT's K2b ruling 1).
const FORCED: [i32; 6] = [-400, -64, -2, 2, 64, 400];

fn scale(b: i32) -> ForceScale {
    ForceScale::new(b).expect("even b")
}

fn pow2(k: i32) -> f64 {
    let half = k / 2;
    2f64.powi(half) * 2f64.powi(k - half)
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

// ------------------------------------------------------------------ builder

/// One model in the product's element families, with its ledger terms.
struct Model {
    node_count: usize,
    frames: Vec<FrameElement>,
    macros: Vec<CurvedBendMacroElement>,
    slots: Vec<CurvedBendStiffnessElement>,
    springs: Vec<(usize, f64)>,
    prescribed: Vec<(usize, f64)>,
    terms: Vec<(usize, ForceTermKind)>,
}

impl Model {
    fn from_kd5(m: &ModelData) -> Self {
        let node = |i: usize| FrameNode::new(i, m.nodes[i]).unwrap();
        let s = &m.section;
        let section = FrameSection::new(s.e, s.g, s.a, s.i, s.i, s.j).unwrap();
        let mut model = Model {
            node_count: m.nodes.len(),
            frames: Vec::new(),
            macros: Vec::new(),
            slots: Vec::new(),
            springs: m.springs.to_vec(),
            prescribed: m.rigid.iter().map(|&d| (d, 0.0)).collect(),
            terms: m
                .loads
                .iter()
                .filter(|l| l.1 != 0.0)
                .map(|&(d, v)| (d, ForceTermKind::Term(v)))
                .collect(),
        };
        for (index, member) in m.members.iter().enumerate() {
            match member.bend {
                None => model.frames.push(
                    FrameElement::new(node(member.i), node(member.j), section, member.y_reference)
                        .unwrap(),
                ),
                Some((center, factor)) => {
                    let e = CurvedBendMacroElement::new(
                        node(member.i),
                        node(member.j),
                        center,
                        s.e,
                        s.g,
                        s.a,
                        s.i,
                        s.j,
                        factor,
                        factor,
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
        model
    }

    fn from_lef(c: &LefCase) -> Self {
        let node = |i: usize| FrameNode::new(i, c.nodes[i]).unwrap();
        let section = product_section(c.e, c.g, c.od, c.wall);
        Model {
            node_count: c.nodes.len(),
            frames: c
                .members
                .iter()
                .map(|&(_, i, j)| {
                    FrameElement::new(node(i), node(j), section, [0.0, 1.0, 0.0]).unwrap()
                })
                .collect(),
            macros: Vec::new(),
            slots: Vec::new(),
            springs: c.springs.to_vec(),
            prescribed: c.rigid.iter().map(|&d| (d, 0.0)).collect(),
            terms: c
                .loads
                .iter()
                .map(|&(d, v)| (d, ForceTermKind::Term(v)))
                .collect(),
        }
    }

    /// One x member N0 -> N1 of length L; N0 anchored; N1 anchored except
    /// the free DOF (or free in all six); the spring on the free DOF.
    fn from_reach(r: &ReachCase) -> Self {
        let section = product_section(r.e, r.g, r.od, r.wall);
        // The generator's binary64 emulation of the section is the product's.
        assert_eq!(
            (
                section.area.to_bits(),
                section.second_moment_y.to_bits(),
                section.torsion_constant.to_bits()
            ),
            (r.area.to_bits(), r.i.to_bits(), r.j.to_bits()),
            "{}: section",
            r.id
        );
        let frame = FrameElement::new(
            FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
            FrameNode::new(1, [r.length, 0.0, 0.0]).unwrap(),
            section,
            [0.0, 1.0, 0.0],
        )
        .unwrap();
        let prescribed = (0..12)
            .filter(|&d| d < 6 || r.free.is_some_and(|f| d != 6 + f))
            .map(|d| (d, 0.0))
            .collect();
        Model {
            node_count: 2,
            frames: vec![frame],
            macros: Vec::new(),
            slots: Vec::new(),
            springs: match (r.free, r.spring) {
                (Some(f), Some(k)) => vec![(6 + f, k)],
                _ => Vec::new(),
            },
            prescribed,
            terms: r
                .loads
                .iter()
                .map(|&(d, v)| (6 + d, ForceTermKind::Term(v)))
                .collect(),
        }
    }

    fn ledger(&self) -> AssembledForce {
        let mut ledger = LoadLedger::new();
        for (index, &(dof, kind)) in self.terms.iter().enumerate() {
            match kind {
                ForceTermKind::Term(v) => ledger.push(format!("load:{index}"), dof, v),
                ForceTermKind::Product(a, b) => {
                    ledger.push_product(format!("load:{index}"), dof, a, b)
                }
            }
        }
        ledger.finish(6 * self.node_count).unwrap()
    }

    fn free(&self) -> Vec<usize> {
        (0..6 * self.node_count)
            .filter(|d| !self.prescribed.iter().any(|p| p.0 == *d))
            .collect()
    }

    fn rigid(&self) -> Vec<usize> {
        self.prescribed.iter().map(|p| p.0).collect()
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

    fn stiffness(&self, s: ForceScale) -> Result<SparseStiffness, FrameKernelError> {
        assemble_sparse_stiffness(
            self.node_count,
            &self.frames,
            &[],
            &self.blocks(),
            &self.springs,
            &SparseAssemblyOptions::new().with_force_scale(s),
        )
    }

    fn case<'a>(
        &'a self,
        force: &'a AssembledForce,
        mode: LinearSolveMode,
        representation: EvidenceRepresentation,
    ) -> ForceScalingCase<'a> {
        ForceScalingCase {
            node_count: self.node_count,
            frames: &self.frames,
            users: &[],
            curved: &self.slots,
            curved_sources: &self.macros,
            springs: &self.springs,
            force,
            prescribed: &self.prescribed,
            mode,
            selected: true,
            representation,
        }
    }

    /// Today's entries at b = 0 (the typed formation-checked entry when
    /// `selected`, otherwise `solve_assembled`).
    fn existing(
        &self,
        force: &AssembledForce,
        mode: LinearSolveMode,
        representation: EvidenceRepresentation,
        selected: bool,
    ) -> Result<StructuralSolution, StructuralError> {
        let k = self.stiffness(ForceScale::UNSCALED).unwrap();
        let free = self.free();
        match representation {
            EvidenceRepresentation::Dense => AssemblyEvidence::new(
                self.node_count,
                &self.frames,
                &[],
                &self.slots,
                &self.springs,
            )?
            .solve_assembled_with_formation_check(
                &k.to_dense(),
                force,
                &free,
                &self.prescribed,
                mode,
                &self.macros,
                selected,
            ),
            EvidenceRepresentation::Pattern => SparseAssemblyEvidence::new(
                k.pattern(),
                self.node_count,
                &self.frames,
                &[],
                &self.slots,
                &self.springs,
            )?
            .solve_assembled_with_formation_check(
                &k,
                force,
                &free,
                &self.prescribed,
                mode,
                &self.macros,
                selected,
            ),
        }
    }

    /// The force-scaled entry at a forced scale (the test hook: the scale is
    /// given, not chosen by the rule).
    fn forced(
        &self,
        force: &AssembledForce,
        s: ForceScale,
        mode: LinearSolveMode,
        representation: EvidenceRepresentation,
        selected: bool,
    ) -> Result<ForceScaledSolution, ForceScaledError> {
        let k = self.stiffness(s).map_err(ForceScaledError::Formation)?;
        let free = self.free();
        match representation {
            EvidenceRepresentation::Dense => AssemblyEvidence::new_force_scaled(
                self.node_count,
                &self.frames,
                &[],
                &self.slots,
                &self.springs,
                s,
            )
            .map_err(ForceScaledError::Structural)?
            .solve_force_scaled_with_formation_check(
                &k.to_dense(),
                force,
                &free,
                &self.prescribed,
                mode,
                &self.macros,
                selected,
            ),
            EvidenceRepresentation::Pattern => SparseAssemblyEvidence::new_force_scaled(
                k.pattern(),
                self.node_count,
                &self.frames,
                &[],
                &self.slots,
                &self.springs,
                s,
            )
            .map_err(ForceScaledError::Structural)?
            .solve_force_scaled_with_formation_check(
                &k,
                force,
                &free,
                &self.prescribed,
                mode,
                &self.macros,
                selected,
            ),
        }
    }
}

/// A force-scaled result as today's entry would give it.
fn as_today(
    result: Result<ForceScaledSolution, ForceScaledError>,
) -> Result<StructuralSolution, StructuralError> {
    match result {
        Ok(scaled) => {
            assert!(scaled.records.is_empty());
            Ok(scaled.solution)
        }
        Err(ForceScaledError::Structural(error)) => Err(error),
        Err(other) => panic!("not an ordinary outcome: {other:?}"),
    }
}

/// The 12 local end actions of a frame at 2^b (the element formed at 2^b,
/// the displacements never scaled), each unscaled for publication.
fn end_actions(frame: &FrameElement, u: &[f64], s: ForceScale) -> [PublishedValue; 12] {
    let k = frame.force_scaled(s).unwrap().local_stiffness().unwrap();
    let t = frame.orientation().unwrap().transformation_matrix();
    let map = element_dof_map(frame.node_i.index, frame.node_j.index);
    let global: [f64; 12] = map.map(|d| u[d]);
    let local: [f64; 12] =
        std::array::from_fn(|r| (0..12).map(|c| t[r][c] * global[c]).sum::<f64>());
    std::array::from_fn(|r| {
        let action = (0..12).map(|c| k[r][c] * local[c]).sum::<f64>();
        unscale_for_publication(action, s, None).unwrap()
    })
}

fn values(published: &[PublishedValue]) -> Vec<u64> {
    published.iter().map(|p| p.value.to_bits()).collect()
}

// ------------------------------------------------------------------ b = 0

/// Every existing entry keeps today's behaviour at b = 0, and the new entries
/// at b = 0 are today's entries, byte for byte: the siblings (both
/// selections), and the orchestrator (step 1 succeeds, b = 0, the stiffness is
/// the kernel's assembly).
#[test]
fn k2b_at_b0_the_siblings_and_the_orchestrator_are_todays_entries_byte_for_byte() {
    let mut demoted = 0;
    for m in KD5 {
        let model = Model::from_kd5(m);
        let force = model.ledger();
        for mode in MODES {
            for representation in REPRESENTATIONS {
                for selected in [true, false] {
                    let today = model.existing(&force, mode, representation, selected);
                    let sibling = as_today(model.forced(
                        &force,
                        ForceScale::UNSCALED,
                        mode,
                        representation,
                        selected,
                    ));
                    assert_eq!(
                        format!("{sibling:?}"),
                        format!("{today:?}"),
                        "{} {mode:?} {representation:?} selected={selected}",
                        m.name
                    );
                }
                let today = model.existing(&force, mode, representation, true);
                let outcome = solve_with_force_scaling(&model.case(&force, mode, representation));
                match (&today, outcome) {
                    (Ok(today), Ok(outcome)) => {
                        assert_eq!(outcome.solution.force_scale, ForceScale::UNSCALED);
                        assert!(outcome.solution.records.is_empty());
                        assert_eq!(
                            format!("{:?}", outcome.solution.solution),
                            format!("{today:?}"),
                            "{} {mode:?} {representation:?}",
                            m.name
                        );
                        assert_eq!(
                            outcome.stiffness,
                            model.stiffness(ForceScale::UNSCALED).unwrap()
                        );
                        demoted += usize::from(today.formation_check.is_some());
                    }
                    (Err(today), Err(ForceScaledError::Structural(error))) => {
                        assert_eq!(&error, today, "{}", m.name)
                    }
                    (today, outcome) => panic!("{}: {today:?} against {outcome:?}", m.name),
                }
            }
        }
    }
    // Not vacuous: K-D5 demotes some of these models (F122 among them).
    assert!(demoted > 0);
}

// ------------------------------------------------------------------ forced b

/// ROOT's K2b ruling 1, pinned: on normal-range models solved at a forced
/// even b != 0, in both representations and both modes, the stiffness is
/// exactly 2^b times today's, the displacements are bit-identical, the
/// unscaled solution (report, load fidelity, formation check) is today's in
/// `Debug`, today's errors are today's errors, the reactions and actions
/// unscale exactly (normal outcomes), and the two representations agree.
#[test]
fn k2b_forced_even_b_is_bitwise_invariant_and_unscales_exactly() {
    let mut solved = 0;
    for m in KD5 {
        let model = Model::from_kd5(m);
        let force = model.ledger();
        let k0 = model.stiffness(ForceScale::UNSCALED).unwrap();
        let rigid = model.rigid();
        for b in FORCED {
            let s = scale(b);
            let ks = model.stiffness(s).unwrap();
            for (&v, &w) in k0.values().iter().zip(ks.values()) {
                assert_eq!(w.to_bits(), (v * pow2(b)).to_bits(), "{} b={b}", m.name);
            }
            for mode in MODES {
                let mut rendered = Vec::new();
                for representation in REPRESENTATIONS {
                    let today = model.existing(&force, mode, representation, true);
                    let scaled = model.forced(&force, s, mode, representation, true);
                    let ctx = format!("{} b={b} {mode:?} {representation:?}", m.name);
                    match (&today, &scaled) {
                        (Ok(today), Ok(scaled)) => {
                            assert_eq!(scaled.force_scale, s, "{ctx}");
                            assert!(scaled.records.is_empty(), "{ctx}");
                            let u = &scaled.solution.displacements;
                            assert_eq!(bits(u), bits(&today.displacements), "{ctx}");
                            assert_eq!(
                                format!("{:?}", scaled.solution),
                                format!("{today:?}"),
                                "{ctx}"
                            );
                            let reactions =
                                ks.force_scaled_reactions(u, &force, s, &rigid).unwrap();
                            let expected = k0.reactions(u, &force).unwrap();
                            for (published, &dof) in reactions.iter().zip(&rigid) {
                                assert_eq!(
                                    published.value.to_bits(),
                                    expected[dof].to_bits(),
                                    "{ctx} R{dof}"
                                );
                                assert_eq!(published.representability, Representability::Normal);
                            }
                            for frame in &model.frames {
                                assert_eq!(
                                    values(&end_actions(frame, u, s)),
                                    values(&end_actions(frame, u, ForceScale::UNSCALED)),
                                    "{ctx} actions"
                                );
                            }
                            if m.name == "F122" {
                                assert_eq!(scaled.solution.report.quality, SolveQuality::Sensitive);
                                assert!(matches!(
                                    scaled.solution.formation_check.as_ref().map(|c| &c.reason),
                                    Some(FormationCheckReason::Estimate)
                                ));
                            }
                            solved += 1;
                        }
                        (Err(today), Err(ForceScaledError::Structural(error))) => {
                            assert_eq!(error, today, "{ctx}")
                        }
                        (today, scaled) => panic!("{ctx}: {today:?} against {scaled:?}"),
                    }
                    rendered.push(format!("{:?}", as_today(scaled)));
                }
                assert_eq!(
                    rendered[0], rendered[1],
                    "{} b={b} {mode:?}: K1 parity",
                    m.name
                );
            }
        }
    }
    assert!(solved > 0);
}

// ------------------------------------------------------------------ formation range

fn within(observed: f64, expected: f64, scale: f64) -> bool {
    (observed - expected).abs() <= 1e-9 * expected.abs().max(scale)
}

/// The observed value of one of R1's expected values, from the published
/// displacements, reactions, spring actions and member end actions.
fn observed(
    c: &LefCase,
    model: &Model,
    target: &Target,
    name: &str,
    u: &[f64],
    reactions: &[PublishedValue],
    s: ForceScale,
) -> f64 {
    match *target {
        Target::Dof(dof) if name.starts_with("R.") => {
            reactions[c.rigid.iter().position(|&d| d == dof).unwrap()].value
        }
        Target::Dof(dof) => u[dof],
        Target::Spring(spring, component) => {
            if component == spring {
                let k = c.springs.iter().find(|p| p.0 == spring).unwrap().1;
                let action = -(k * pow2(s.exponent())) * u[spring];
                unscale_for_publication(action, s, None).unwrap().value
            } else {
                0.0
            }
        }
        Target::Member(i, kind) => {
            let f = end_actions(&model.frames[i], u, s).map(|p| p.value);
            match kind {
                "N" => f[6],
                "T" => f[9],
                "Mbi" => f[4].hypot(f[5]),
                "Mbj" => f[10].hypot(f[11]),
                other => panic!("{other}"),
            }
        }
    }
}

/// ROOT's K2b ruling 2, with its accuracy clause restated by checkpoint-A
/// ruling C: R1's RF-RANGE LEF-large cases (CHAIN, SKEW, CONT) reach
/// `local_stiffness` and are refused by K2a at b = 0; the orchestrator solves
/// them through formation-time scaling, at the generator's b.
///
/// RF-RANGE is an exact power-of-two scaling of a base case (lengths 2^pl,
/// moduli 2^pm, loads 2^pf), and so is the force-scaled solve: its
/// displacements, standing, formation-check record, reactions and member
/// actions are the base case's today (today's entry, b = 0) times exact
/// powers of two, bit for bit (translations 2^(pf-pm-pl), rotations
/// 2^(pf-pm-2pl), forces 2^pf, moments 2^(pf+pl)), with the base case's
/// standing. K2b adds no accuracy beyond the ordinary path:
/// - CONT is published Passed, and every expected value meets the 1e-9
///   criterion against R1's exact references;
/// - CHAIN and SKEW are published Sensitive, as their r1e-08 bases are
///   today: flagged, and not claimed accurate (the worst ratio is recorded).
#[test]
fn k2b_lef_large_solves_bit_identically_to_its_base_case_with_the_bases_standing() {
    let mut passed = 0;
    for (c, base) in LEF_LARGE.iter().zip(LEF_BASE) {
        let model = Model::from_lef(c);
        let base_model = Model::from_lef(base);
        let force = model.ledger();
        let base_force = base_model.ledger();
        assert!(
            matches!(
                model.stiffness(ForceScale::UNSCALED),
                Err(FrameKernelError::NumericalRange { .. })
            ),
            "{}: precondition, K2a refuses at b = 0",
            c.id
        );
        let translation = pow2(c.pf - c.pm - c.pl);
        let rotation = pow2(c.pf - c.pm - 2 * c.pl);
        let (force_unit, moment_unit) = (pow2(c.pf), pow2(c.pf + c.pl));
        let k_base = base_model.stiffness(ForceScale::UNSCALED).unwrap();
        for mode in MODES {
            for representation in REPRESENTATIONS {
                let ctx = format!("{} {mode:?} {representation:?}", c.id);
                let outcome = solve_with_force_scaling(&model.case(&force, mode, representation))
                    .unwrap_or_else(|e| panic!("{ctx}: {e:?}"));
                let s = outcome.solution.force_scale;
                assert_eq!(s.exponent(), c.b, "{ctx}");
                let solution = &outcome.solution.solution;
                let today = base_model
                    .existing(&base_force, mode, representation, true)
                    .unwrap();
                let u = &solution.displacements;
                let expected_u: Vec<f64> = today
                    .displacements
                    .iter()
                    .enumerate()
                    .map(|(d, &v)| v * if d % 6 < 3 { translation } else { rotation })
                    .collect();
                assert_eq!(bits(u), bits(&expected_u), "{ctx}: u");
                assert_eq!(solution.report.quality, today.report.quality, "{ctx}");
                let standing = if c.id == "RF-RANGE-CONT-LEF-large" {
                    SolveQuality::Passed
                } else {
                    SolveQuality::Sensitive
                };
                assert_eq!(
                    solution.report.quality, standing,
                    "{ctx}: the base's standing"
                );
                assert_eq!(
                    format!("{:?}", solution.formation_check),
                    format!("{:?}", today.formation_check),
                    "{ctx}"
                );
                let reactions = outcome
                    .stiffness
                    .force_scaled_reactions(u, &force, s, c.rigid)
                    .unwrap();
                let base_reactions = k_base.reactions(&today.displacements, &base_force).unwrap();
                for (published, &dof) in reactions.iter().zip(c.rigid) {
                    let unit = if dof % 6 < 3 { force_unit } else { moment_unit };
                    assert_eq!(
                        published.value.to_bits(),
                        (base_reactions[dof] * unit).to_bits(),
                        "{ctx} R{dof}"
                    );
                }
                for (frame, base_frame) in model.frames.iter().zip(&base_model.frames) {
                    let actions = end_actions(frame, u, s);
                    let base_actions =
                        end_actions(base_frame, &today.displacements, ForceScale::UNSCALED);
                    for (k, (a, b)) in actions.iter().zip(&base_actions).enumerate() {
                        let unit = if k % 6 < 3 { force_unit } else { moment_unit };
                        assert_eq!(a.value.to_bits(), (b.value * unit).to_bits(), "{ctx} f{k}");
                    }
                }
                let mut worst: f64 = 0.0;
                for (name, target, expected, scale) in c.expected {
                    let value = observed(c, &model, target, name, u, &reactions, s);
                    let ratio = (value - expected).abs() / (1e-9 * expected.abs().max(*scale));
                    worst = worst.max(ratio);
                    if solution.report.quality == SolveQuality::Passed {
                        assert!(
                            within(value, *expected, *scale),
                            "{ctx} {name}: {value:e} against {expected:e} (scale {scale:e})"
                        );
                    }
                }
                passed += usize::from(solution.report.quality == SolveQuality::Passed);
                eprintln!(
                    "K2B-LEF {ctx} b={} quality={:?} worst_ratio={worst:e}",
                    s.exponent(),
                    solution.report.quality
                );
            }
        }
    }
    // CONT is published Passed in every mode and representation.
    assert_eq!(passed, 4);
}

/// ROOT's K2b ruling 2 (as restated by checkpoint-A rulings B and C): K2a's
/// product-reach cases with normal geometry, reach_zero, reach_lef and the
/// partial underflow, and a synthetic PHYS-R4 element, are solved at kernel
/// level, each at the generator's b, published Passed, and within 1e-9 of its
/// exact reference. Preconditions: at b = 0 the three product-reach members
/// are refused by K2a, and PHYS-R4's element is formed but refused by M03's
/// allowance range (`transform_roundoff`).
///
/// The partial underflow's load is 1e-307 N (2^-1020). Its intended-action
/// residual record at N1 UY is about 2^-56.5 relative to the load, so that
/// physical field underflows when unscaled: it is published as a zero of its
/// sign with the explicit outcome `Underflow` (ruling B), and the case is
/// solved. Every listed field's value is the one published.
#[test]
fn k2b_the_product_reach_cases_and_phys_r4_are_solved_accurately() {
    for id in ["reach-zero", "reach-lef", "partial-underflow", "phys-r4"] {
        let r = REACH.iter().find(|r| r.id == id).unwrap();
        let model = Model::from_reach(r);
        let force = model.ledger();
        if r.id == "phys-r4" {
            assert!(model.stiffness(ForceScale::UNSCALED).is_ok());
            assert!(matches!(
                AssemblyEvidence::new(2, &model.frames, &[], &[], &model.springs),
                Err(StructuralError::Range(_))
            ));
        } else {
            assert!(
                matches!(
                    model.stiffness(ForceScale::UNSCALED),
                    Err(FrameKernelError::NumericalRange { .. })
                ),
                "{}: precondition",
                r.id
            );
        }
        let (mut translation, mut rotation): (f64, f64) = (0.0, 0.0);
        for &(dof, v) in r.exact {
            if dof < 3 {
                translation = translation.max(v.abs());
            } else {
                rotation = rotation.max(v.abs());
            }
        }
        for mode in MODES {
            for representation in REPRESENTATIONS {
                let ctx = format!("{} {mode:?} {representation:?}", r.id);
                let outcome = solve_with_force_scaling(&model.case(&force, mode, representation))
                    .unwrap_or_else(|e| panic!("{ctx}: {e:?}"));
                assert_eq!(outcome.solution.force_scale.exponent(), r.b, "{ctx}");
                let solution = &outcome.solution.solution;
                assert_eq!(solution.report.quality, SolveQuality::Passed, "{ctx}");
                let u = &solution.displacements;
                for &(dof, exact) in r.exact {
                    let class = if dof < 3 { translation } else { rotation };
                    assert!(
                        within(u[6 + dof], exact, class),
                        "{ctx} dof {dof}: {} against {exact:e}",
                        u[6 + dof]
                    );
                }
                let report = &solution.report;
                for record in &outcome.solution.records {
                    let rows = if record.record.starts_with("intended") {
                        &report.intended_residual_rows
                    } else {
                        &report.residual_rows
                    };
                    let row = rows
                        .iter()
                        .find(|x| x.global_dof == record.global_dof)
                        .unwrap();
                    let field = match record.record.rsplit('.').next().unwrap() {
                        "residual" => row.residual,
                        "denominator" => row.denominator,
                        _ => row.evaluation_allowance,
                    };
                    assert_eq!(field.to_bits(), record.value.to_bits(), "{ctx}");
                }
                if r.id == "partial-underflow" {
                    let intended = &report.intended_residual_rows[0];
                    assert_eq!(intended.global_dof, 7);
                    assert!(intended.normalized_residual != 0.0);
                    assert_eq!(intended.row_scale_exponent, -1020);
                    assert_eq!(intended.residual, 0.0);
                    assert!(outcome.solution.records.contains(&RecordOutcome {
                        record: "intended_residual_rows.residual",
                        global_dof: 7,
                        value: intended.residual,
                        representability: RecordRepresentability::Underflow,
                    }));
                }
                eprintln!(
                    "K2B-REACH {ctx} b={} quality={:?} u={:?} records={:?}",
                    r.b,
                    solution.report.quality,
                    r.exact.iter().map(|&(d, _)| u[6 + d]).collect::<Vec<_>>(),
                    outcome
                        .solution
                        .records
                        .iter()
                        .map(|x| (x.record, x.representability))
                        .collect::<Vec<_>>()
                );
            }
        }
    }
}

/// The M03 evaluation at a forced b through today's FK functions (no
/// orchestrator), to show where a scaled evaluation stops.
fn solve_at_scale_unpublished(
    model: &Model,
    force: &AssembledForce,
    s: ForceScale,
) -> Result<StructuralSolution, StructuralError> {
    let k = model.stiffness(s).unwrap().to_dense();
    let evidence = AssemblyEvidence::new_force_scaled(
        model.node_count,
        &model.frames,
        &[],
        &[],
        &model.springs,
        s,
    )?;
    let scaled = force.force_scaled(s).unwrap();
    let free = model.free();
    let basis = "k2b".to_string();
    let system = StructuralSystem::assembled(
        &k,
        &scaled,
        &free,
        &model.prescribed,
        Some(&evidence.contributions),
        Some(SymmetryEvidence {
            absolute_roundoff: &evidence.absolute_roundoff,
            operation_counts: &evidence.operation_counts,
            basis: &basis,
        }),
    );
    structural::solve_assembled_structural_dense(&system)
}

fn exponent_of(x: f64) -> i32 {
    ((x.to_bits() >> 52) & 0x7ff) as i32 - 1023
}

/// ROOT's K2b checkpoint-A ruling A: the spring-carried case (G = 1e-300 Pa)
/// stays a named refusal, with K2a's `GJ/L: G*J` as its trigger. The
/// mechanism, asserted: at every even b in the census window, formation passes
/// K2a, GJ/L is 2^-1082 times the 1 N*m/rad spring it is absorbed into at N1
/// RX (the stored diagonal is the spring's value exactly), and M03's
/// contribution audit (FK `audit_contributions`, unchanged) refuses the
/// absorbed difference, whose measure in the equilibrated units lies below
/// binary64: `Range("exact radix loses represented bits")`. Force scaling
/// keeps every ratio, so no b restores the case (routed to W1/K4).
#[test]
fn k2b_spring_carried_stays_a_named_refusal_at_m03s_contribution_audit() {
    let model = spring_carried();
    let force = model.ledger();
    for b in [200, 548, 900] {
        let s = scale(b);
        let k = model.stiffness(s).unwrap();
        let spring = model.springs[0].1 * pow2(b);
        let torsion = model.frames[0]
            .force_scaled(s)
            .unwrap()
            .local_stiffness()
            .unwrap()[3][3];
        assert!(torsion.is_normal(), "b={b}: K2a passes at scale");
        assert_eq!(exponent_of(spring) - exponent_of(torsion), 1082, "b={b}");
        assert_eq!(k.get(9, 9), spring, "b={b}: GJ/L is absorbed");
        assert_eq!(
            solve_at_scale_unpublished(&model, &force, s),
            Err(StructuralError::Range("exact radix loses represented bits")),
            "b={b}"
        );
    }
    for mode in MODES {
        for representation in REPRESENTATIONS {
            let refusal = solve_with_force_scaling(&model.case(&force, mode, representation));
            assert_eq!(
                refusal,
                Err(ForceScaledError::Refused(ForceScalingRefusal {
                    reason: ForceScaleReason::ScaledEvaluation,
                    trigger: Some(RangeTrigger::Formation(FrameKernelError::NumericalRange {
                        name: "GJ/L: G*J"
                    })),
                }))
            );
            assert_eq!(
                refusal.unwrap_err().to_string(),
                "range: scaled evaluation outside normal range"
            );
        }
    }
}

/// ROOT's K2b ruling 2: RF-RANGE LEF-small never reaches formation; it stays
/// a geometry refusal (FK's 1e-12 m axis tolerance).
#[test]
fn k2b_lef_small_stays_a_geometry_refusal() {
    let [a, b] = LEF_SMALL_M1;
    let section = product_section(2.0e11, 8.0e10, 0.2, 0.01);
    assert_eq!(
        FrameElement::new(
            FrameNode::new(0, a).unwrap(),
            FrameNode::new(1, b).unwrap(),
            section,
            [0.0, 1.0, 0.0],
        ),
        Err(FrameKernelError::DegenerateAxis {
            detail: "element length"
        })
    );
}

// ------------------------------------------------------------------ the b-rule's refusals

fn spring_carried() -> Model {
    Model::from_reach(REACH.iter().find(|r| r.id == "spring-carried").unwrap())
}

/// Step 4: a feasible b exists (the census window is wide), but the one
/// evaluation at that b also leaves the normal range, because force scaling
/// never changes the displacements (u = 2^-1100 here). The refusal carries
/// the step-1 trigger, and there is no third attempt.
#[test]
fn k2b_step_four_refuses_a_scaled_evaluation_outside_the_normal_range() {
    let section = FrameSection::new(pow2(500), pow2(500), 1.0, 1.0, 1.0, 1.0).unwrap();
    let frame = FrameElement::new(
        FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
        FrameNode::new(1, [1.0, 0.0, 0.0]).unwrap(),
        section,
        [0.0, 1.0, 0.0],
    )
    .unwrap();
    let model = Model {
        node_count: 2,
        frames: vec![frame],
        macros: Vec::new(),
        slots: Vec::new(),
        springs: Vec::new(),
        prescribed: (0..6).map(|d| (d, 0.0)).collect(),
        terms: vec![(6, ForceTermKind::Term(pow2(-600)))],
    };
    let force = model.ledger();
    for mode in MODES {
        for representation in REPRESENTATIONS {
            let today = model.existing(&force, mode, representation, true);
            assert!(
                matches!(today, Err(StructuralError::Range(_))),
                "precondition: {today:?}"
            );
            match solve_with_force_scaling(&model.case(&force, mode, representation)) {
                Err(ForceScaledError::Refused(ForceScalingRefusal {
                    reason: ForceScaleReason::ScaledEvaluation,
                    trigger: Some(RangeTrigger::Evaluation(StructuralError::Range(_))),
                })) => {}
                other => panic!("{mode:?} {representation:?}: {other:?}"),
            }
        }
    }
}

/// Steps 2 and 3, from a K2a trigger: the spring-carried member (GJ/L's G*J
/// underflows) with, on an anchored DOF, a spring of 2^996 N/m (the window is
/// then infeasible: [-1082, 996]) or of 1e-310 N/m (subnormal). Each refusal
/// keeps K2a's name as its trigger (ROOT's K2b ruling 7).
#[test]
fn k2b_the_window_and_subnormal_refusals_keep_the_k2a_trigger() {
    let trigger = Some(RangeTrigger::Formation(FrameKernelError::NumericalRange {
        name: "GJ/L: G*J",
    }));
    for (spring, reason) in [
        (
            pow2(996),
            ForceScaleReason::InfeasibleWindow {
                e_min: -1082,
                e_max: 996,
            },
        ),
        (1.0e-310, ForceScaleReason::SubnormalAtFormation),
    ] {
        let mut model = spring_carried();
        model.springs.push((0, spring));
        let force = model.ledger();
        for mode in MODES {
            for representation in REPRESENTATIONS {
                let result = solve_with_force_scaling(&model.case(&force, mode, representation));
                assert_eq!(
                    result,
                    Err(ForceScaledError::Refused(ForceScalingRefusal {
                        reason: reason.clone(),
                        trigger: trigger.clone(),
                    }))
                );
            }
        }
        if let ForceScaleReason::InfeasibleWindow { .. } = reason {
            assert_eq!(
                reason.to_string(),
                "range: exponent span [-1082, 996] exceeds the binary64 normal window after exact power-of-two scaling"
            );
        }
    }
}

/// An x chain of unit-length axial bars, UX free at each node but N0, with
/// section E and the given UX loads.
fn axial_chain(e: f64, xs: &[f64], loads: &[(usize, f64)]) -> Model {
    let section = FrameSection::new(e, 1.0, 1.0, 1.0, 1.0, 1.0).unwrap();
    let node = |i: usize| FrameNode::new(i, [xs[i], 0.0, 0.0]).unwrap();
    let frames = (1..xs.len())
        .map(|i| {
            let (a, b) = if xs[i] > xs[0] { (i - 1, i) } else { (i, 0) };
            FrameElement::new(node(a), node(b), section, [0.0, 1.0, 0.0]).unwrap()
        })
        .collect();
    let prescribed = (0..6 * xs.len())
        .filter(|&d| d < 6 || d % 6 != 0)
        .map(|d| (d, 0.0))
        .collect();
    Model {
        node_count: xs.len(),
        frames,
        macros: Vec::new(),
        slots: Vec::new(),
        springs: Vec::new(),
        prescribed,
        terms: loads
            .iter()
            .map(|&(node, v)| (6 * node, ForceTermKind::Term(v)))
            .collect(),
    }
}

/// Step 5 through the entries, at a forced b:
/// - a subnormal reaction: an axial chain N0-N1-N2 (EA/L = 2^-100 N/m) with
///   UX loads 2^-1000 N at N1 and -(2^-1000 - 2^-1052) N at N2. The reaction
///   at N0 is -2^-1052 N (subnormal) with normal loads, stiffnesses and
///   displacements (it is about -2^-1052 N; the solve's rounding sets its
///   exact bits). At b = 600 it is published exactly, with the subnormal
///   outcome and its stated precision, and with the bits today's E12 gives at
///   b = 0;
/// - an overflowing reaction: N2-N0-N1 (EA/L = 2^1000 N/m) with UX loads
///   1.5 * 2^1023 N at N1 and N2. The reaction at N0 is -3 * 2^1023 N. At
///   b = -4 everything is normal at scale, and the reaction is refused, never
///   flushed. The entry itself publishes, N1 UX's residual-record
///   denominator carrying the explicit outcome `Overflow` (ruling B).
///
/// (A nonzero reaction cannot underflow with normal loads and displacements
/// here; the underflow outcome is pinned by the kernel test
/// `k2b_force_scaled_reactions_unscale_with_the_outcomes`.)
#[test]
fn k2b_published_reactions_take_the_step_five_outcomes() {
    let delta = pow2(-1052);
    let small = axial_chain(
        pow2(-100),
        &[0.0, 1.0, 2.0],
        &[(1, pow2(-1000)), (2, -(pow2(-1000) - delta))],
    );
    let force = small.ledger();
    let k0 = small.stiffness(ForceScale::UNSCALED).unwrap();
    for mode in MODES {
        for representation in REPRESENTATIONS {
            let today = small
                .existing(&force, mode, representation, true)
                .unwrap_or_else(|e| panic!("{mode:?} {representation:?}: {e:?}"));
            let e12 = k0.reactions(&today.displacements, &force).unwrap()[0];
            assert!(
                e12.is_subnormal(),
                "precondition: a subnormal reaction, {e12:e}"
            );
            let s = scale(600);
            let scaled = small.forced(&force, s, mode, representation, true).unwrap();
            let u = &scaled.solution.displacements;
            assert_eq!(bits(u), bits(&today.displacements));
            let published = small
                .stiffness(s)
                .unwrap()
                .force_scaled_reactions(u, &force, s, &[0])
                .unwrap();
            assert_eq!(published[0].value.to_bits(), e12.to_bits());
            let quanta = e12.abs().to_bits() as f64;
            let precision = 0.5 / quanta;
            assert_eq!(
                published[0].representability,
                Representability::Subnormal {
                    relative_precision: if precision * quanta < 0.5 {
                        precision.next_up()
                    } else {
                        precision
                    }
                }
            );
        }
    }
    let large = axial_chain(
        pow2(1000),
        &[0.0, 1.0, -1.0],
        &[(1, 1.5 * pow2(1023)), (2, 1.5 * pow2(1023))],
    );
    let force = large.ledger();
    let s = scale(-4);
    let outside = |dof| {
        ForceScaledError::Refused(ForceScalingRefusal {
            reason: ForceScaleReason::PublicationOutsideBinary64 {
                global_dof: Some(dof),
            },
            trigger: None,
        })
    };
    // The solve at 2^-4 is normal throughout; its u is published unscaled.
    let unpublished = solve_at_scale_unpublished(&large, &force, s).unwrap();
    let reaction = large.stiffness(s).unwrap().force_scaled_reactions(
        &unpublished.displacements,
        &force,
        s,
        &[0],
    );
    assert_eq!(reaction, Err(outside(0)));
    for mode in MODES {
        for representation in REPRESENTATIONS {
            // The entry publishes, with N1 UX's residual-record denominator
            // |f| + sum |K||u| overflowing when unscaled: an infinity with the
            // explicit outcome `Overflow` (ruling B), not a refusal.
            let scaled = large
                .forced(&force, s, mode, representation, true)
                .unwrap_or_else(|e| panic!("{mode:?} {representation:?}: {e:?}"));
            assert!(scaled.records.iter().any(|r| r.global_dof == 6
                && r.record.ends_with("denominator")
                && r.representability == RecordRepresentability::Overflow
                && r.value == f64::INFINITY));
            assert_eq!(
                bits(&scaled.solution.displacements),
                bits(&unpublished.displacements)
            );
        }
    }
}

// ------------------------------------------------------------------ interactions

/// The existing entries refuse an evidence formed at 2^b != 1 (fail closed);
/// only the force-scaled entries unscale for publication.
#[test]
fn k2b_existing_entries_refuse_a_force_scaled_evidence() {
    let model = Model::from_kd5(&F122);
    let force = model.ledger();
    let s = scale(64);
    let k = model.stiffness(s).unwrap();
    let dense = k.to_dense();
    let free = model.free();
    let refused = Err(StructuralError::InvalidInput(
        "force-scaled assembly evidence: use its force-scaled entry",
    ));
    let evidence =
        AssemblyEvidence::new_force_scaled(2, &model.frames, &[], &[], &model.springs, s).unwrap();
    assert_eq!(evidence.force_scale(), s);
    let pattern = SparseAssemblyEvidence::new_force_scaled(
        k.pattern(),
        2,
        &model.frames,
        &[],
        &[],
        &model.springs,
        s,
    )
    .unwrap();
    assert_eq!(pattern.force_scale(), s);
    for mode in MODES {
        let p = &model.prescribed;
        assert_eq!(
            evidence.solve_assembled(&dense, &force, &free, p, mode),
            refused
        );
        assert_eq!(
            evidence.solve_assembled_with_formation_check(
                &dense,
                &force,
                &free,
                p,
                mode,
                &[],
                true
            ),
            refused
        );
        assert_eq!(
            evidence.solve_binary64(&dense, force.values(), &free, p, mode),
            refused
        );
        assert_eq!(
            evidence.solve(&dense, force.values(), &free, p, mode),
            refused
        );
        assert_eq!(pattern.solve_assembled(&k, &force, &free, p, mode), refused);
        assert_eq!(
            pattern.solve_assembled_with_formation_check(&k, &force, &free, p, mode, &[], true),
            refused
        );
        assert_eq!(pattern.solve(&k, force.values(), &free, p, mode), refused);
        // The force-scaled entries solve it.
        assert!(evidence
            .solve_force_scaled_with_formation_check(&dense, &force, &free, p, mode, &[], true)
            .is_ok());
    }
}

/// S11-K under b: each load split into three ledger terms (RV8-3's split,
/// 0.1v, 0.7v and v - 0.1v - 0.7v), and an unaudited load-fidelity row (terms
/// 1e80, -1e80 and 1e-300 on one free DOF, which leave the audit's radix
/// range at every scale). At forced b the unscaled solution (KS1, KS3, the
/// load audit's record and K-D5's ρ) is today's, byte for byte.
#[test]
fn k2b_s11k_ledger_terms_and_audit_are_carried_under_b() {
    for m in [&F122, &CSKEW_8_5, &F345] {
        let mut model = Model::from_kd5(m);
        model.terms = model
            .terms
            .iter()
            .flat_map(|&(dof, kind)| {
                let ForceTermKind::Term(v) = kind else {
                    unreachable!()
                };
                [0.1 * v, 0.7 * v, v - 0.1 * v - 0.7 * v]
                    .map(|part| (dof, ForceTermKind::Term(part)))
            })
            .collect();
        let dof = model.free()[0];
        if m.name == "F345" {
            model.terms.extend([
                (dof, ForceTermKind::Term(1.0e80)),
                (dof, ForceTermKind::Term(-1.0e80)),
                (dof, ForceTermKind::Term(1.0e-300)),
            ]);
        }
        let force = model.ledger();
        for mode in MODES {
            for representation in REPRESENTATIONS {
                let today = model.existing(&force, mode, representation, true);
                if m.name == "F345" {
                    let fidelity = today.as_ref().unwrap().load_fidelity.as_ref().unwrap();
                    assert!(fidelity.rows.iter().any(|r| r.unaudited.is_some()));
                }
                for b in [-64, 64, 400] {
                    if m.name == "F345" && b < 0 {
                        // 1e-300 * 2^-64 would be subnormal: the scaled
                        // ledger refuses it (pinned in the kernel tests).
                        continue;
                    }
                    let scaled =
                        as_today(model.forced(&force, scale(b), mode, representation, true));
                    assert_eq!(
                        format!("{scaled:?}"),
                        format!("{today:?}"),
                        "{} b={b} {mode:?} {representation:?}",
                        m.name
                    );
                }
            }
        }
    }
}

/// ROOT's K2b ruling 4: the kernel never reads the S11-G formation records,
/// which the force-scaled ledger drops. The same loads pushed with and without
/// a record solve identically, at b = 0 and at b = 64.
#[test]
fn k2b_the_kernel_reads_no_s11g_formation_record() {
    let model = Model::from_kd5(&F122);
    let plain = model.ledger();
    let mut ledger = LoadLedger::new();
    for (index, &(dof, kind)) in model.terms.iter().enumerate() {
        let ForceTermKind::Term(v) = kind else {
            unreachable!()
        };
        ledger.push_formed(
            format!("load:{index}"),
            dof,
            v,
            Formation::Bounded { bound: 1.0e-20 },
            1.0e-20,
            false,
        );
    }
    let formed = ledger.finish(12).unwrap();
    assert!(formed.has_formation_records() && !plain.has_formation_records());
    for mode in MODES {
        for representation in REPRESENTATIONS {
            for s in [ForceScale::UNSCALED, scale(64)] {
                assert_eq!(
                    format!("{:?}", model.forced(&plain, s, mode, representation, true)),
                    format!("{:?}", model.forced(&formed, s, mode, representation, true)),
                );
            }
        }
    }
}

/// The behavioural pin behind the source pin: the nonlinear loop reaches no
/// force-scaled entry. On K2a's reach_zero case (with an open gap on its free
/// DOF, so the invocation is nonlinear), the loop is refused by K2a's unscaled
/// formation, while the orchestrator solves the same linear system at
/// b = 734 (precondition: the paths differ).
#[test]
fn k2b_nonlinear_loop_reaches_no_force_scaled_entry() {
    let model = Model::from_reach(REACH.iter().find(|r| r.id == "reach-zero").unwrap());
    let force = model.ledger();
    for mode in MODES {
        let outcome =
            solve_with_force_scaling(&model.case(&force, mode, EvidenceRepresentation::Dense))
                .unwrap();
        assert_eq!(outcome.solution.force_scale.exponent(), 734);
        let input = NonlinearFrameSolveInput {
            node_count: 2,
            elements: model.frames.clone(),
            user_stiffness_elements: vec![],
            curved_bend_elements: vec![],
            force: force.values().to_vec(),
            base_restrained_dofs: model.rigid(),
            nonlinear_supports: vec![NonlinearSupport::gap(
                "k2b-open-gap",
                1,
                FrameDof::Uy,
                1000.0,
                GapDirection::PositiveDisplacement,
            )
            .unwrap()],
            initial_states: vec![SupportStateRecord::new(
                "k2b-open-gap",
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
        let result = solve_active_set_frame_with_mode_and_springs(&input, mode, &model.springs);
        let rendered = format!("{result:?}");
        assert!(result.is_err(), "{mode:?}: {rendered}");
        assert!(
            rendered.contains("12EIy/L^3: (12*E)*Iy"),
            "{mode:?}: {rendered}"
        );
    }
}

//! S11-K tests for this crate (S11 section 9, K7 and K12, and the T3 manager's
//! conformance terms for the typed nonlinear siblings). The crate is serialized
//! with T5: only the typed signatures, the bit-for-bit force check and the
//! adapter's audit seam are added; the loop is unchanged.
use crate::structural_adapter::AssemblyEvidence;
use crate::*;
use open_pipe_stress_frame_kernel::load_ledger::{ForceTerm, ForceTermKind, LoadLedger};
use open_pipe_stress_frame_kernel::structural::SolveQuality;
use open_pipe_stress_frame_kernel::{assemble_global_stiffness, FrameDof, FrameNode, FrameSection};
use open_pipe_stress_nonlinear_supports::{
    ActiveSetState, GapDirection, NonlinearSupport, SupportStateRecord,
};

fn adjacent_input() -> NonlinearFrameSolveInput {
    let section = FrameSection::new(100.0, 40.0, 1.0, 1.0, 1.0, 1.0).unwrap();
    let elements = (0..2)
        .map(|i| {
            FrameElement::new(
                FrameNode::new(i, [i as f64, 0.0, 0.0]).unwrap(),
                FrameNode::new(i + 1, [(i + 1) as f64, 0.0, 0.0]).unwrap(),
                section,
                [0.0, 1.0, 0.0],
            )
            .unwrap()
        })
        .collect();
    let mut force = vec![0.0; 18];
    force[12] = 12.5;
    NonlinearFrameSolveInput {
        node_count: 3,
        elements,
        user_stiffness_elements: vec![],
        curved_bend_elements: vec![],
        force,
        base_restrained_dofs: (0..18).filter(|&i| i != 6 && i != 12).collect(),
        nonlinear_supports: vec![
            NonlinearSupport::gap(
                "g1",
                1,
                FrameDof::Ux,
                f64::from_bits(0.125_f64.to_bits() - 1),
                GapDirection::PositiveDisplacement,
            )
            .unwrap(),
            NonlinearSupport::gap(
                "g2",
                2,
                FrameDof::Ux,
                0.25,
                GapDirection::PositiveDisplacement,
            )
            .unwrap(),
        ],
        initial_states: vec![
            SupportStateRecord::new("g1", ActiveSetState::Active),
            SupportStateRecord::new("g2", ActiveSetState::Inactive),
        ],
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

fn ledger_force(values: &[f64]) -> open_pipe_stress_frame_kernel::load_ledger::AssembledForce {
    let mut ledger = LoadLedger::new();
    for (dof, &v) in values.iter().enumerate() {
        if v != 0.0 {
            ledger.push(format!("load:{dof}"), dof, v);
        }
    }
    ledger.finish(values.len()).unwrap()
}

type Sibling = fn(
    &NonlinearFrameSolveInput,
    &open_pipe_stress_frame_kernel::load_ledger::AssembledForce,
) -> Result<NonlinearFrameSolveResult, NonlinearIntegrationError>;
type Untyped =
    fn(&NonlinearFrameSolveInput) -> Result<NonlinearFrameSolveResult, NonlinearIntegrationError>;

fn siblings() -> Vec<(&'static str, Sibling, Untyped)> {
    vec![
        (
            "solve_active_set_frame",
            |i, f| solve_active_set_frame_assembled(i, f),
            |i| solve_active_set_frame(i),
        ),
        (
            "solve_active_set_frame_with_mode",
            |i, f| {
                solve_active_set_frame_with_mode_assembled(i, f, LinearSolveMode::SparseInteractive)
            },
            |i| solve_active_set_frame_with_mode(i, LinearSolveMode::SparseInteractive),
        ),
        (
            "solve_active_set_frame_with_mode_and_springs",
            |i, f| {
                solve_active_set_frame_with_mode_and_springs_assembled(
                    i,
                    f,
                    LinearSolveMode::DenseScrutiny,
                    &[(7, 5.0)],
                )
            },
            |i| {
                solve_active_set_frame_with_mode_and_springs(
                    i,
                    LinearSolveMode::DenseScrutiny,
                    &[(7, 5.0)],
                )
            },
        ),
    ]
}

#[test]
fn k12_typed_nonlinear_siblings_delegate_on_bit_equal_force() {
    let input = adjacent_input();
    let force = ledger_force(&input.force);
    for (name, typed, untyped) in siblings() {
        assert_eq!(typed(&input, &force), untyped(&input), "{name}");
    }
}

#[test]
fn k12_typed_nonlinear_siblings_refuse_a_one_ulp_difference_in_any_entry() {
    let input = adjacent_input();
    for dof in [0, 6, 12, 17] {
        let mut shifted = input.force.clone();
        shifted[dof] = if shifted[dof] == 0.0 {
            f64::from_bits(1)
        } else {
            f64::from_bits(shifted[dof].to_bits() + 1)
        };
        let force = ledger_force(&shifted);
        for (name, typed, _) in siblings() {
            assert!(
                matches!(
                    typed(&input, &force),
                    Err(NonlinearIntegrationError::InvalidInput { .. })
                ),
                "{name} dof {dof}"
            );
        }
    }
    // A different length is refused too.
    let force = ledger_force(&input.force[..17]);
    for (name, typed, _) in siblings() {
        assert!(
            matches!(
                typed(&input, &force),
                Err(NonlinearIntegrationError::InvalidInput { .. })
            ),
            "{name}"
        );
    }
}

#[test]
fn k12_typed_nonlinear_siblings_treat_negative_zero_as_different_bits() {
    // The ledger's zero is +0.0; an input carrying -0.0 differs bit for bit.
    let mut input = adjacent_input();
    input.force[3] = -0.0;
    let force = ledger_force(&input.force);
    assert_eq!(force.values()[3].to_bits(), 0);
    for (name, typed, _) in siblings() {
        assert!(
            matches!(
                typed(&input, &force),
                Err(NonlinearIntegrationError::InvalidInput { .. })
            ),
            "{name}"
        );
    }
}

// ------------------------------------------------------------- adapter

fn cantilever_evidence() -> (AssemblyEvidence, Vec<Vec<f64>>) {
    let section = FrameSection::new(2.0e11, 7.7e10, 0.01, 8.0e-6, 9.0e-6, 1.7e-5).unwrap();
    let element = FrameElement::new(
        FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
        FrameNode::new(1, [2.0, 0.0, 0.0]).unwrap(),
        section,
        [0.0, 1.0, 0.0],
    )
    .unwrap();
    let k = assemble_global_stiffness(2, &[element.clone()]).unwrap();
    (
        AssemblyEvidence::new(2, &[element], &[], &[], &[]).unwrap(),
        k,
    )
}

fn cancelling_tip_terms(order: [f64; 3]) -> Vec<ForceTerm> {
    order
        .iter()
        .enumerate()
        .map(|(i, &v)| ForceTerm {
            source: format!("load:{i}"),
            dof: 7,
            kind: ForceTermKind::Term(v),
        })
        .collect()
}

#[test]
fn k7_adapter_with_force_terms_flags_the_folded_force_in_both_modes() {
    let (evidence, k) = cantilever_evidence();
    let free: Vec<usize> = (6..12).collect();
    let fixed: Vec<(usize, f64)> = (0..6).map(|d| (d, 0.0)).collect();
    for (g, n) in [(1e8, 0.3), (1e80, 1e-8)] {
        let terms = cancelling_tip_terms([g, n, -g]);
        let mut folded = vec![0.0; 12];
        folded[7] = g + n - g;
        assert_ne!(folded[7], n, "precondition");
        let audited = evidence.clone().with_force_terms(&terms);
        for mode in [
            LinearSolveMode::DenseScrutiny,
            LinearSolveMode::SparseInteractive,
        ] {
            let legacy = evidence.solve(&k, &folded, &free, &fixed, mode).unwrap();
            let detect = audited.solve(&k, &folded, &free, &fixed, mode).unwrap();
            assert_eq!(legacy.load_fidelity, None);
            assert_eq!(detect.displacements, legacy.displacements);
            let report = detect.load_fidelity.expect("flagged");
            assert!(report.rows.iter().any(|r| r.global_dof == 7));
            assert_eq!(detect.report.quality, SolveQuality::Sensitive);
            // The typed route solves the exact net and passes.
            let mut ledger = LoadLedger::new();
            for t in &terms {
                if let ForceTermKind::Term(v) = t.kind {
                    ledger.push(t.source.clone(), t.dof, v);
                }
            }
            let force = ledger.finish(12).unwrap();
            assert_eq!(force.values()[7], n);
            let typed = evidence
                .solve_assembled(&k, &force, &free, &fixed, mode)
                .unwrap();
            assert!(typed.load_fidelity.is_none());
            assert_eq!(typed.report.quality, SolveQuality::Passed);
            let mut net = vec![0.0; 12];
            net[7] = n;
            let reference = evidence.solve(&k, &net, &free, &fixed, mode).unwrap();
            assert_eq!(typed.displacements, reference.displacements);
            assert_eq!(
                format!("{:?}", typed.report),
                format!("{:?}", reference.report)
            );
        }
    }
}

// ------------------------------------ ROOT option (c): the loop's legacy pin

/// Comment- and string-aware stripper, copied from the S11 site table's `lex`
/// (`frame_kernel/tests/s11_site_table.rs`; that file is another crate's
/// integration test and cannot be imported). Comments are removed and string
/// and char literal contents blanked, keeping line structure (RV1-S1).
fn lex(src: &str) -> String {
    let b: Vec<char> = src.chars().collect();
    let n = b.len();
    let mut out = String::with_capacity(n);
    let mut i = 0;
    let ident = |c: char| c.is_alphanumeric() || c == '_';
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
                    if b[i] == '\n' {
                        out.push('\n');
                    }
                    i += 1;
                }
            }
            continue;
        }
        let prev_ident = i > 0 && ident(b[i - 1]);
        let raw_start = if !prev_ident && c == 'r' {
            Some(i + 1)
        } else if !prev_ident && c == 'b' && i + 1 < n && b[i + 1] == 'r' {
            Some(i + 2)
        } else {
            None
        };
        if let Some(mut j) = raw_start {
            let mut hashes = 0;
            while j < n && b[j] == '#' {
                hashes += 1;
                j += 1;
            }
            if j < n && b[j] == '"' {
                j += 1;
                loop {
                    if j >= n {
                        break;
                    }
                    if b[j] == '"' && (0..hashes).all(|h| j + 1 + h < n && b[j + 1 + h] == '#') {
                        j += 1 + hashes;
                        break;
                    }
                    if b[j] == '\n' {
                        out.push('\n');
                    }
                    j += 1;
                }
                out.push_str("\"\"");
                i = j;
                continue;
            }
        }
        if c == '"' || (!prev_ident && c == 'b' && i + 1 < n && b[i + 1] == '"') {
            let mut j = if c == 'b' { i + 2 } else { i + 1 };
            while j < n && b[j] != '"' {
                if b[j] == '\\' {
                    j += 1;
                }
                if j < n && b[j] == '\n' {
                    out.push('\n');
                }
                j += 1;
            }
            out.push_str("\"\"");
            i = j + 1;
            continue;
        }
        if c == '\'' {
            if i + 2 < n && b[i + 1] != '\\' && b[i + 2] == '\'' {
                out.push_str("' '");
                i += 3;
                continue;
            }
            if i + 1 < n && b[i + 1] == '\\' {
                let mut j = i + 2;
                while j < n && b[j] != '\'' {
                    j += 1;
                }
                out.push_str("' '");
                i = j + 1;
                continue;
            }
        }
        out.push(c);
        i += 1;
    }
    out
}

/// Blanks every `#[cfg(test)]` item (copied from the site table as above).
fn strip_cfg_test(code: &str) -> String {
    const ATTR: &str = "#[cfg(test)]";
    let mut out = String::with_capacity(code.len());
    let mut rest = code;
    while let Some(at) = rest.find(ATTR) {
        out.push_str(&rest[..at]);
        let item = &rest[at..];
        let mut depth = 0usize;
        let mut started = false;
        let mut end = item.len();
        for (k, ch) in item.char_indices().skip(ATTR.len()) {
            match ch {
                '{' => {
                    depth += 1;
                    started = true;
                }
                '}' => {
                    depth -= 1;
                    if started && depth == 0 {
                        end = k + 1;
                        break;
                    }
                }
                ';' if !started => {
                    end = k + 1;
                    break;
                }
                _ => {}
            }
        }
        out.extend(item[..end].chars().filter(|&c| c == '\n'));
        rest = &item[end..];
    }
    out.push_str(rest);
    out
}

fn function_body<'a>(code: &'a str, signature: &str) -> &'a str {
    let start = code.find(signature).expect("function present");
    let body_start = start + code[start..].find('{').unwrap();
    let mut depth = 0usize;
    for (k, ch) in code[body_start..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return &code[body_start..body_start + k];
                }
            }
            _ => {}
        }
    }
    panic!("unbalanced body of {signature}");
}

/// Every exact kernel entry point (S11-K KS1-KS3 and the typed seams).
const EXACT_ENTRY_POINTS: &[&str] = &[
    "reduce_system_with_prescribed_displacements(",
    "reduce_assembled_system",
    ".solve(",
    ".solve_assembled(",
    // RV1-B-N1: fully qualified call syntax, `AssemblyEvidence::solve(`.
    "::solve(",
    "::solve_assembled(",
    "solve_structural_dense(",
    "solve_structural_sparse(",
    "prepare_structural(",
    "prepare_assembled_structural(",
    "solve_assembled_structural_dense(",
    "evaluate_assembled_original_residual(",
    "_with_force_terms(",
    "with_force_terms(",
    "StructuralSystem::assembled(",
    // K1 (pin extension): the pattern path's exact entries.
    "reduce_assembled_sparse_system(",
    "SparseStructuralSystem::new(",
    "SparseStructuralSystem::assembled(",
    "prepare_sparse_structural(",
    "prepare_assembled_sparse_structural(",
    "solve_sparse_structural(",
    "solve_assembled_sparse_structural(",
    "solve_sparse_prepared(",
];

/// K1 (pin extension): the occurrences of `token` in `code` that begin on an
/// identifier boundary when `token` begins with a letter, so that
/// `SparseStructuralSystem::assembled(` is not counted as
/// `StructuralSystem::assembled(`. A token beginning with `_`, `.` or `:`
/// matches anywhere, as before (`_with_force_terms(` still matches
/// `prepare_structural_with_force_terms(`).
fn token_indices(code: &str, token: &str) -> Vec<usize> {
    let letter = token.starts_with(|c: char| c.is_ascii_alphabetic());
    code.match_indices(token)
        .map(|(at, _)| at)
        .filter(|&at| {
            !letter
                || !code[..at]
                    .chars()
                    .next_back()
                    .is_some_and(|c| c.is_alphanumeric() || c == '_')
        })
        .collect()
}

/// K1 (pin extension): the body of `signature` inside the impl block that
/// begins with `header`, located in `code` (byte range).
fn body_in_impl(code: &str, header: &str, signature: &str) -> std::ops::Range<usize> {
    let block = function_body(code, header);
    let body = function_body(block, signature);
    let at = body.as_ptr() as usize - code.as_ptr() as usize;
    at..at + body.len()
}

/// The two structural-adapter impls whose named entries define the exact and
/// formation-checked variants: the dense originals and (K1) their pattern
/// siblings, and nothing else.
const ADAPTER_IMPLS: [&str; 2] = ["impl AssemblyEvidence {", "impl SparseAssemblyEvidence {"];

/// The nonlinear active-set loop's closed-gap prescribed solves go through
/// `solve_linearized_system_evidence` (the loop, the influence solves and the
/// unit-force solves). ROOT's option (c) keeps them on the named, unchanged
/// binary64 kernel path. The source is lexed first (comments and literals
/// removed, `#[cfg(test)]` items blanked): the four legacy targets must be
/// real calls in that function, and no exact kernel entry point may appear
/// anywhere in `lib.rs` outside test code (RV1-S1), so neither a comment nor a
/// helper can hide one. The behavioural pin below backs it.
///
/// RV1-B-N1: this is a text scan. The behavioural pins
/// (`option_c_closed_gap_loop_solves_are_bit_equal_to_the_binary64_legacy_path`
/// and `option_c_active_set_loop_first_closed_gap_iteration_is_binary64`) are
/// authoritative; a same-named helper defined in another file is caught only
/// by them. S11-F narrowed `AssemblyEvidence::solve` to `pub(crate)`, which
/// closes the fully qualified route for callers outside this crate at compile
/// time, but not inside it, where the behavioural pins remain the guarantee.
#[test]
fn option_c_nonlinear_loop_is_pinned_to_the_binary64_kernel_path() {
    let code = strip_cfg_test(&lex(include_str!("lib.rs")));
    let body = function_body(&code, "fn solve_linearized_system_evidence(");
    for required in [
        "product_equilibrium::evaluate(",
        "reduce_system_with_prescribed_displacements_binary64(",
        "assembly.solve_binary64(",
        "structural::solve_structural_dense_binary64(",
        "structural_adapter::solve_structural_sparse_binary64(",
    ] {
        assert!(
            body.contains(required),
            "missing legacy call target {required}"
        );
    }
    for forbidden in EXACT_ENTRY_POINTS {
        assert!(
            !code.contains(forbidden),
            "exact kernel entry point {forbidden} in nonlinear_integration/src/lib.rs"
        );
    }
    // The stripper itself: a commented or quoted call is not a call.
    let probe =
        lex("// assembly.solve_binary64(\nlet s = \"solve_structural_dense(\";\na.solve(k)");
    assert!(!probe.contains("solve_binary64(") && !probe.contains("dense("));
    assert!(probe.contains(".solve("));
}

/// RV1-B-N1: the scan extended to `structural_adapter.rs`. Outside the
/// functions that define the exact variants (`solve`, `solve_assembled`, the
/// `with_force_terms` C3-detect binding, and K-D5's typed linear entry
/// `solve_assembled_with_formation_check`, which is `solve_assembled` plus the
/// D-5 check), no exact kernel entry point may
/// appear, so the named legacy variants (`solve_binary64`,
/// `solve_structural_sparse_binary64`) and the rest of the adapter reach only
/// the binary64 kernel path. The behavioural pins remain authoritative.
#[test]
fn option_c_structural_adapter_legacy_variants_reach_only_binary64_entry_points() {
    let mut code = strip_cfg_test(&lex(include_str!("structural_adapter.rs")));
    // K1 (pin extension, ROOT): the defining bodies are blanked by impl block:
    // `impl AssemblyEvidence`'s originals, as before, and
    // `impl SparseAssemblyEvidence`'s pattern siblings of the same names only.
    for header in ADAPTER_IMPLS {
        for defining in [
            "fn solve(",
            "fn solve_assembled(",
            "fn with_force_terms(",
            "fn solve_assembled_with_formation_check(",
        ] {
            let range = body_in_impl(&code, header, defining);
            code.replace_range(range, "{");
        }
    }
    for required in ["fn solve_binary64(", "fn solve_structural_sparse_binary64("] {
        assert!(code.contains(required), "missing legacy variant {required}");
    }
    let legacy = function_body(&code, "fn solve_binary64(");
    assert!(legacy.contains("prepare_structural_binary64("));
    let sparse = function_body(&code, "fn solve_structural_sparse_binary64(");
    assert!(sparse.contains("prepare_structural_binary64("));
    // `.solve(` is replaced by `self.solve(` here: the adapter's
    // `scrutinize_gaps` calls the exact-boundary retained context's own
    // `context.solve()` (exact rationals, not a KS1-KS3 entry point).
    let forbidden_list = EXACT_ENTRY_POINTS
        .iter()
        .copied()
        .filter(|p| *p != ".solve(")
        .chain(["self.solve("]);
    for forbidden in forbidden_list {
        // The defining signatures themselves remain (`fn solve(` is a
        // definition, not a call); only their bodies were blanked.
        let hits = token_indices(&code, forbidden)
            .into_iter()
            .filter(|at| !code[..*at].trim_end().ends_with("fn"))
            .count();
        assert_eq!(
            hits, 0,
            "exact kernel entry point {forbidden} in structural_adapter.rs outside the exact variants"
        );
    }
}

// -------------------------------- ROOT option (c): the behavioural pin

/// A probe-P-like closed-gap active-set case: members of 2 m and 4 m along x,
/// both end nodes restrained except UY, which is a gap closed at g on the
/// left and 4g on the right (the loop prescribes UY at DOFs 1 and 13; the
/// middle RZ row's two couplings, 27 * 2^18 * g and -27 * 2^16 * 4g, are
/// exact negatives), a 0.0137 N*m moment and a 100 N UY
/// force at the middle node (the force keeps both gap reactions admissible).
/// Invented section data (E = 2^38 Pa, Iz = 9 * 2^-19 m^4) make the formed
/// coefficients exact dyadics. The unequal lengths couple the middle node's
/// UY and RZ, so the reduced system is not diagonal and the loop's sparse
/// observation (the only consumer of its reduced force) depends on that
/// force's bits.
const PROBE_M: f64 = 0.0137;
const PROBE_F: f64 = 100.0;

fn probe_p_elements() -> Vec<FrameElement> {
    let section = FrameSection::new(
        2.0_f64.powi(38),
        7.7e10,
        0.01,
        1.0e-5,
        9.0 * 2.0_f64.powi(-19),
        1.7e-5,
    )
    .unwrap();
    let x = [0.0, 2.0, 6.0];
    (0..2)
        .map(|i| {
            FrameElement::new(
                FrameNode::new(i, [x[i], 0.0, 0.0]).unwrap(),
                FrameNode::new(i + 1, [x[i + 1], 0.0, 0.0]).unwrap(),
                section,
                [0.0, 1.0, 0.0],
            )
            .unwrap()
        })
        .collect()
}

const PROBE_BASE_RESTRAINTS: [usize; 10] = [0, 2, 3, 4, 5, 12, 14, 15, 16, 17];

fn probe_boundary(g: f64) -> BoundaryState {
    // The loop's active boundary: base restraints at 0.0 plus the two closed
    // gaps at +g, sorted by DOF (`active_boundary`).
    let mut pairs: Vec<(usize, f64)> = PROBE_BASE_RESTRAINTS.iter().map(|&d| (d, 0.0)).collect();
    pairs.push((1, g));
    pairs.push((13, 4.0 * g));
    pairs.sort_by_key(|p| p.0);
    BoundaryState {
        dofs: pairs.iter().map(|p| p.0).collect(),
        displacements: pairs.iter().map(|p| p.1).collect(),
    }
}

fn probe_force() -> Vec<f64> {
    let mut force = vec![0.0; 18];
    force[7] = PROBE_F;
    force[11] = PROBE_M;
    force
}

/// Exact `f_r - sum_c K_rc * g_c`, one exact sum rounded once (S11-K's
/// accumulator, verified against Fraction references by frame_kernel K1).
fn exact_reduced_row(k: &[Vec<f64>], f: &[f64], boundary: &BoundaryState, r: usize) -> f64 {
    let mut acc = open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator::new();
    acc.add(f[r]).unwrap();
    for (&c, &g) in boundary.dofs.iter().zip(&boundary.displacements) {
        acc.add_product(-k[r][c], g).unwrap();
    }
    acc.round().unwrap()
}

/// The legacy reduced system, folded in the test in the legacy order:
/// `b = f_r; b -= K_rc * g_c` over the boundary in DOF order.
fn legacy_reduced(
    k: &[Vec<f64>],
    f: &[f64],
    boundary: &BoundaryState,
) -> (Vec<usize>, Vec<Vec<f64>>, Vec<f64>) {
    let free: Vec<usize> = (0..f.len())
        .filter(|d| !boundary.dofs.contains(d))
        .collect();
    let mut stiffness = Vec::new();
    let mut force = Vec::new();
    for &r in &free {
        let mut b = f[r];
        for (&c, &g) in boundary.dofs.iter().zip(&boundary.displacements) {
            b -= k[r][c] * g;
        }
        force.push(b);
        stiffness.push(free.iter().map(|&c| k[r][c]).collect());
    }
    (free, stiffness, force)
}

struct Expected {
    displacements: Vec<f64>,
    residual_rows: String,
    sparse_evidence: String,
}

fn expected_binary64(
    k: &DenseMatrix,
    f: &[f64],
    boundary: &BoundaryState,
    assembly: Option<&AssemblyEvidence>,
    mode: LinearSolveMode,
) -> Expected {
    let (free, reduced_k, reduced_f) = legacy_reduced(k, f, boundary);
    let prescribed: Vec<(usize, f64)> = boundary
        .dofs
        .iter()
        .copied()
        .zip(boundary.displacements.iter().copied())
        .collect();
    let system = open_pipe_stress_frame_kernel::structural::StructuralSystem {
        stiffness: k,
        force: f,
        free_dofs: &free,
        prescribed: &prescribed,
        contributions: None,
        symmetry: None,
    };
    // The legacy kernel path (its right-hand side is pinned to today's fold
    // by frame_kernel's option_c_binary64_variants_keep_todays_fold_on_probe_p).
    let checked = match (assembly, mode) {
        (Some(a), mode) => a.solve_binary64(k, f, &free, &prescribed, mode).unwrap(),
        (None, LinearSolveMode::DenseScrutiny) => {
            open_pipe_stress_frame_kernel::structural::solve_structural_dense_binary64(&system)
                .unwrap()
        }
        (None, LinearSolveMode::SparseInteractive) => {
            crate::structural_adapter::solve_structural_sparse_binary64(&system).unwrap()
        }
    };
    let dense_reference = match mode {
        LinearSolveMode::DenseScrutiny => {
            Some(open_pipe_stress_frame_kernel::solve_dense(&reduced_k, &reduced_f).unwrap())
        }
        LinearSolveMode::SparseInteractive => None,
    };
    let sparse = crate::observe_sparse_linearized_solve(
        k,
        &free,
        &reduced_f,
        dense_reference.as_deref(),
        mode,
    );
    Expected {
        displacements: checked.displacements,
        residual_rows: format!("{:?}", checked.report.residual_rows),
        sparse_evidence: format!("{sparse:?}"),
    }
}

fn bits(values: &[f64]) -> Vec<u64> {
    values.iter().map(|v| v.to_bits()).collect()
}

#[test]
fn option_c_closed_gap_loop_solves_are_bit_equal_to_the_binary64_legacy_path() {
    let elements = probe_p_elements();
    let k = assemble_global_stiffness(3, &elements).unwrap();
    let f = probe_force();
    for g in [0.05, 0.20] {
        let boundary = probe_boundary(g);
        // Precondition: the legacy fold of the RZ row differs from the
        // correctly rounded exact value.
        let (free, _, legacy_f) = legacy_reduced(&k, &f, &boundary);
        let rz = free.iter().position(|&d| d == 11).unwrap();
        let exact_rz = exact_reduced_row(&k, &f, &boundary, 11);
        assert_ne!(
            legacy_f[rz].to_bits(),
            exact_rz.to_bits(),
            "precondition g={g}"
        );
        // The exact path gives a different answer, so the pins discriminate.
        let exact_reduced =
            open_pipe_stress_frame_kernel::reduce_system_with_prescribed_displacements(
                &k,
                &f,
                &boundary.dofs,
                &boundary.displacements,
            )
            .unwrap();
        assert_eq!(exact_reduced.force[rz].to_bits(), exact_rz.to_bits());
        let assembly = AssemblyEvidence::new(3, &elements, &[], &[], &[]).unwrap();
        for mode in [
            LinearSolveMode::DenseScrutiny,
            LinearSolveMode::SparseInteractive,
        ] {
            for with_assembly in [false, true] {
                let a = with_assembly.then_some(&assembly);
                let expected = expected_binary64(&k, &f, &boundary, a, mode);
                let ctx = format!("g={g} {mode:?} assembly={with_assembly}");
                // Discrimination: the exact solve and the exact reduced force
                // would give different displacements and sparse evidence.
                let prescribed: Vec<(usize, f64)> = boundary
                    .dofs
                    .iter()
                    .copied()
                    .zip(boundary.displacements.iter().copied())
                    .collect();
                let exact = assembly.solve(&k, &f, &free, &prescribed, mode).unwrap();
                assert_ne!(
                    bits(&exact.displacements),
                    bits(&expected.displacements),
                    "{ctx}"
                );
                let exact_sparse = crate::observe_sparse_linearized_solve(
                    &k,
                    &free,
                    &exact_reduced.force,
                    None,
                    LinearSolveMode::SparseInteractive,
                );
                let legacy_sparse = crate::observe_sparse_linearized_solve(
                    &k,
                    &free,
                    &legacy_f,
                    None,
                    LinearSolveMode::SparseInteractive,
                );
                assert_ne!(
                    format!("{exact_sparse:?}"),
                    format!("{legacy_sparse:?}"),
                    "{ctx}"
                );
                // The loop's linearized solve.
                let solved =
                    crate::solve_linearized_system_evidence(a, &k, &f, &boundary, mode).unwrap();
                assert_eq!(
                    bits(&solved.displacements),
                    bits(&expected.displacements),
                    "{ctx}"
                );
                assert_eq!(
                    format!("{:?}", solved.structural_report.residual_rows),
                    expected.residual_rows,
                    "{ctx}"
                );
                assert_eq!(
                    format!("{:?}", solved.sparse_evidence),
                    expected.sparse_evidence,
                    "{ctx}"
                );
            }
        }
    }
}

/// The same probe through the public active-set loop: both gaps start closed.
/// The loop always carries AssemblyEvidence. Its first iteration's ordinary
/// solve (retained by the strict-gap evidence as `ordinary_displacements`,
/// before any exact-ratio gap projection replaces the published displacement),
/// its structural residual rows and its sparse observation equal the binary64
/// legacy values bit for bit.
#[test]
fn option_c_active_set_loop_first_closed_gap_iteration_is_binary64() {
    let elements = probe_p_elements();
    let k = assemble_global_stiffness(3, &elements).unwrap();
    for g in [0.05, 0.20] {
        let input = NonlinearFrameSolveInput {
            node_count: 3,
            elements: elements.clone(),
            user_stiffness_elements: vec![],
            curved_bend_elements: vec![],
            force: probe_force(),
            base_restrained_dofs: PROBE_BASE_RESTRAINTS.to_vec(),
            nonlinear_supports: vec![
                NonlinearSupport::gap(
                    "gap:i",
                    0,
                    FrameDof::Uy,
                    g,
                    GapDirection::PositiveDisplacement,
                )
                .unwrap(),
                NonlinearSupport::gap(
                    "gap:j",
                    2,
                    FrameDof::Uy,
                    4.0 * g,
                    GapDirection::PositiveDisplacement,
                )
                .unwrap(),
            ],
            initial_states: vec![
                SupportStateRecord::new("gap:i", ActiveSetState::Active),
                SupportStateRecord::new("gap:j", ActiveSetState::Active),
            ],
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
        let boundary = probe_boundary(g);
        let assembly = AssemblyEvidence::new(3, &elements, &[], &[], &[]).unwrap();
        for mode in [
            LinearSolveMode::DenseScrutiny,
            LinearSolveMode::SparseInteractive,
        ] {
            let result = solve_active_set_frame_with_mode(&input, mode).unwrap();
            let first = &result.iterations[0];
            assert_eq!(first.active_restrained_dofs, boundary.dofs);
            assert_eq!(
                bits(&first.active_prescribed_displacements),
                bits(&boundary.displacements)
            );
            let expected = expected_binary64(&k, &input.force, &boundary, Some(&assembly), mode);
            let ctx = format!("g={g} {mode:?}");
            let ordinary = match &first.strict_gap {
                StrictGapEvidence::Qualified(report) => report.ordinary_displacements.clone(),
                _ => first.displacements.clone(),
            };
            assert_eq!(bits(&ordinary), bits(&expected.displacements), "{ctx}");
            assert_eq!(
                format!("{:?}", first.structural_report.residual_rows),
                expected.residual_rows,
                "{ctx}"
            );
            assert_eq!(
                format!("{:?}", first.sparse_evidence),
                expected.sparse_evidence,
                "{ctx}"
            );
        }
    }
}

/// ROOT on V1 R5-3: `product_equilibrium::evaluate` (the loop's final
/// equilibrium and the gap scrutiny) measures with the public, binary64
/// `structural::evaluate_original_residual`; FK pins that function's bytes
/// (`option_c_public_original_residual_stays_binary64_on_coupled_rows`).
#[test]
fn option_c_product_equilibrium_uses_the_binary64_residual() {
    let src = include_str!("product_equilibrium.rs");
    assert!(src.contains("structural::evaluate_original_residual(system, u)"));
    assert!(!src.contains("evaluate_assembled_original_residual"));
    let adapter = include_str!("structural_adapter.rs");
    assert!(adapter.contains("crate::product_equilibrium::evaluate(&system, &u)"));
}

// ---------------------------- K-D5: the nonlinear loop reaches no formation check
// (T3 D1 §4.3.1 callers; mutation 32). Source pin backed by a behavioural pin
// whose precondition shows the formation-checked and loop paths differ.

/// Every K-D5 entry point: none may appear in the loop's sources.
const FORMATION_ENTRY_POINTS: &[&str] = &[
    "solve_assembled_with_formation_check",
    "with_formation_source",
    "prepare_formation_checked_structural",
    "solve_formation_checked_structural_dense",
    "FormationSource",
    "FormationCheckedSystem",
    // K1 (pin extension): the pattern path's formation-check plumbing.
    "FormationCheckedSparseSystem",
    "prepare_formation_checked_sparse_structural",
    "solve_formation_checked_sparse_structural",
];

/// The non-test source of the crate rooted at `src_dir`. The module tree is
/// walked from `lib.rs` through every `mod name;` declaration (honouring
/// `#[path]` and `mod.rs`). A module is a test module when it is declared
/// under `#[cfg(test)]` (the declaration disappears under `strip_cfg_test`)
/// or declared inside a test module: `s11k_tests`, `kd5_tests` and its
/// `kd5_models`, and the product crate's test modules are excluded by their
/// own declarations, never by file path. A non-test module added later is
/// scanned. Every `.rs` file under `src_dir` must be reached, so nothing is
/// skipped silently (RV5 E4). Returns (path relative to `src_dir`, non-test
/// code with comments, literals and `#[cfg(test)]` items blanked).
fn non_test_modules(src_dir: &std::path::Path) -> Vec<(String, String)> {
    use std::path::{Path, PathBuf};
    fn rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                rs_files(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push(path);
            }
        }
    }
    // `mod name;` declarations in lexed code: (name, byte offset).
    fn declarations(code: &str) -> Vec<(String, usize)> {
        let mut out = Vec::new();
        for (at, _) in code.match_indices("mod ") {
            if at > 0 {
                let prev = code[..at].chars().next_back().unwrap();
                if prev.is_alphanumeric() || prev == '_' {
                    continue;
                }
            }
            let rest = code[at + 4..].trim_start();
            let name: String = rest
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            if !name.is_empty() && rest[name.len()..].trim_start().starts_with(';') {
                out.push((name, at));
            }
        }
        out
    }
    fn child_dir(file: &Path) -> PathBuf {
        let stem = file.file_stem().unwrap();
        if stem == "lib" || stem == "mod" {
            file.parent().unwrap().to_path_buf()
        } else {
            file.with_extension("")
        }
    }
    // The file a declaration names. `lex` keeps line structure, so a
    // `#[path = "..."]` attribute is read from the raw lines before it.
    fn target(file: &Path, raw: &str, code: &str, name: &str, at: usize) -> PathBuf {
        let head = &code[..at];
        let from = head.rfind([';', '}', '{']).map_or(0, |k| k + 1);
        if head[from..].contains("#[path") {
            let line = head.matches('\n').count();
            let raw_lines: Vec<&str> = raw.lines().collect();
            let attr = (0..=line)
                .rev()
                .map(|l| raw_lines[l])
                .find(|l| l.contains("#[path"))
                .unwrap();
            return file.parent().unwrap().join(attr.split('"').nth(1).unwrap());
        }
        let dir = child_dir(file);
        let flat = dir.join(format!("{name}.rs"));
        if flat.is_file() {
            flat
        } else {
            dir.join(name).join("mod.rs")
        }
    }
    let mut reached: Vec<(PathBuf, bool, String)> = Vec::new();
    let mut stack = vec![(src_dir.join("lib.rs"), false)];
    while let Some((file, test)) = stack.pop() {
        let raw =
            std::fs::read_to_string(&file).unwrap_or_else(|e| panic!("{}: {e}", file.display()));
        let lexed = lex(&raw);
        let code = strip_cfg_test(&lexed);
        let non_test: Vec<PathBuf> = declarations(&code)
            .into_iter()
            .map(|(name, at)| target(&file, &raw, &code, &name, at))
            .collect();
        for (name, at) in declarations(&lexed) {
            let child = target(&file, &raw, &lexed, &name, at);
            let child_test = test || !non_test.contains(&child);
            stack.push((child, child_test));
        }
        reached.push((file, test, code));
    }
    let mut all = Vec::new();
    rs_files(src_dir, &mut all);
    for path in &all {
        assert!(
            reached.iter().any(|(f, _, _)| f == path),
            "{} is not reached from lib.rs by any module declaration",
            path.display()
        );
    }
    reached
        .into_iter()
        .filter(|(_, test, _)| !test)
        .map(|(f, _, code)| {
            let rel = f.strip_prefix(src_dir).unwrap().display().to_string();
            (rel, code)
        })
        .collect()
}

/// Occurrences of `name` in `code` outside the body of the function whose
/// signature is `allowed_in` (the whole text when `None`).
fn occurrences_outside(code: &str, name: &str, allowed_in: Option<&str>) -> usize {
    let mut code = code.to_string();
    if let Some(signature) = allowed_in {
        if code.contains(signature) {
            // K1 (pin extension, ROOT): the entry is allowed in each adapter
            // impl that defines it (the dense original and its pattern
            // sibling), and nowhere else.
            for header in ADAPTER_IMPLS {
                if code.contains(header) {
                    let range = body_in_impl(&code, header, signature);
                    code.replace_range(range, "{");
                }
            }
        }
    }
    token_indices(&code, name).len()
}

/// RV5 E4 (strengthened): every non-test module of this crate (not only
/// `lib.rs` and `product_equilibrium.rs`) and of product_physics is scanned.
/// - `solve_assembled_with_formation_check` appears once in this crate: its
///   definition in `structural_adapter.rs`. No call to it exists here, under
///   any name, helper or fully qualified form.
/// - The formation-check plumbing (`with_formation_source`,
///   `prepare_formation_checked_structural`, `.formation_source(`,
///   `FormationCheckedSystem`, `solve_formation_checked_structural_dense`,
///   `formation_check::`) appears only inside that definition's body, and
///   `FormationSource` only in `structural_adapter.rs`. No `solve_with_formation…`
///   name exists (the legacy entry is removed).
/// - In product_physics the entry is called exactly once, inside
///   `solve_preview_reduced_system`.
/// The behavioural pins below back this scan.
#[test]
fn kd5_nonlinear_sources_name_no_formation_check_entry_point() {
    // The stripper itself: a commented, quoted or test-only call is not a call.
    let control = strip_cfg_test(&lex(
        "fn a() { b.solve_binary64(); } // solve_assembled_with_formation_check(\n\
         const S: &str = \"solve_assembled_with_formation_check\";\n\
         #[cfg(test)]\nmod t { fn c() { d.solve_assembled_with_formation_check(); } }\n",
    ));
    assert!(control.contains("solve_binary64("));
    assert!(!control.contains("solve_assembled_with_formation_check"));
    const SA_ENTRY: &str = "fn solve_assembled_with_formation_check(";
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let modules = non_test_modules(&manifest.join("src"));
    let names: Vec<&str> = modules.iter().map(|(n, _)| n.as_str()).collect();
    // The walk reaches the known modules and none of the test modules.
    for known in ["lib.rs", "product_equilibrium.rs", "structural_adapter.rs"] {
        assert!(names.contains(&known), "{known} not reached: {names:?}");
    }
    assert!(!names
        .iter()
        .any(|n| n.contains("tests") || n.contains("kd5_models")));
    for (name, code) in &modules {
        let sa = name == "structural_adapter.rs";
        // K1 (pin extension, ROOT): two definitions, one in each adapter impl
        // (the dense entry and its pattern sibling); a third anywhere fails.
        let defined = if sa { 2 } else { 0 };
        assert_eq!(
            token_indices(code, "solve_assembled_with_formation_check").len(),
            defined,
            "{name}: the formation-checked entry is named outside its definitions"
        );
        if sa {
            assert_eq!(code.matches(SA_ENTRY).count(), 2, "{name}");
            for header in ADAPTER_IMPLS {
                assert_eq!(
                    function_body(code, header).matches(SA_ENTRY).count(),
                    1,
                    "{name}: {header} defines the formation-checked entry once"
                );
            }
        }
        for plumbing in FORMATION_ENTRY_POINTS
            .iter()
            .copied()
            .filter(|p| *p != "solve_assembled_with_formation_check" && *p != "FormationSource")
            .chain([
                ".formation_source(",
                "formation_check::",
                "solve_with_formation",
            ])
        {
            assert_eq!(
                occurrences_outside(code, plumbing, sa.then_some(SA_ENTRY)),
                0,
                "{name} reaches {plumbing} outside the typed formation-checked entry"
            );
        }
        if !sa {
            assert_eq!(code.matches("FormationSource").count(), 0, "{name}");
        }
    }
    // product_physics: the one product call, inside solve_preview_reduced_system.
    let product = non_test_modules(&manifest.join("../../product_physics/src"));
    assert!(product.iter().any(|(n, _)| n == "lib.rs"));
    let mut calls = 0;
    for (name, code) in &product {
        assert_eq!(code.matches("solve_with_formation").count(), 0, "{name}");
        for plumbing in FORMATION_ENTRY_POINTS
            .iter()
            .filter(|p| **p != "solve_assembled_with_formation_check")
        {
            assert_eq!(
                code.matches(plumbing).count(),
                0,
                "{name} reaches {plumbing}"
            );
        }
        let here = code
            .matches("solve_assembled_with_formation_check(")
            .count();
        if here > 0 {
            assert_eq!(name, "lib.rs");
            let body = function_body(code, "fn solve_preview_reduced_system(");
            assert_eq!(
                body.matches(".solve_assembled_with_formation_check(").count(),
                here,
                "product_physics calls the formation-checked entry outside solve_preview_reduced_system"
            );
        }
        calls += here;
    }
    assert_eq!(
        calls, 1,
        "product_physics calls the formation-checked entry {calls} times"
    );
    // The loop's structural solve stays the named binary64 variant.
    let lib = strip_cfg_test(&lex(include_str!("lib.rs")));
    assert!(lib.contains("assembly.solve_binary64("));
}

/// P1's 122 skew cantilever with an open gap support at the tip: the loop
/// solves the same linear system in its first iteration.
fn kd5_loop_input(built: &crate::structural_adapter::kd5_tests::Built) -> NonlinearFrameSolveInput {
    NonlinearFrameSolveInput {
        node_count: 2,
        elements: built.frames.clone(),
        user_stiffness_elements: vec![],
        curved_bend_elements: vec![],
        force: built.f.clone(),
        base_restrained_dofs: crate::structural_adapter::kd5_tests::skew_122_rigid().to_vec(),
        nonlinear_supports: vec![NonlinearSupport::gap(
            "kd5-open-gap",
            1,
            FrameDof::Uz,
            1.0,
            GapDirection::PositiveDisplacement,
        )
        .unwrap()],
        initial_states: vec![SupportStateRecord::new(
            "kd5-open-gap",
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

#[test]
fn kd5_nonlinear_loop_reaches_no_formation_check() {
    use crate::structural_adapter::kd5_tests::{bits, skew_122, MODES};
    let built = skew_122();
    let input = kd5_loop_input(&built);
    for mode in MODES {
        // Precondition: the same linear system demotes through the formation
        // check and is Passed through the ordinary typed solve.
        assert_eq!(
            built.checked(mode).report.quality,
            SolveQuality::Sensitive,
            "{mode:?}"
        );
        assert_eq!(
            built.plain(mode).report.quality,
            SolveQuality::Passed,
            "{mode:?}"
        );
        // The loop: its solve is the unchanged binary64 path, never the check.
        let result =
            solve_active_set_frame_with_mode_and_springs(&input, mode, &built.springs).unwrap();
        assert!(result.converged, "{mode:?}");
        let first = result.iterations.first().expect("iteration");
        assert_eq!(
            first.structural_report.quality,
            SolveQuality::Passed,
            "{mode:?}"
        );
        let binary64 = built
            .assembly()
            .solve_binary64(&built.k, &built.f, &built.free, &built.prescribed, mode)
            .unwrap();
        assert_eq!(binary64.formation_check, None);
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

// ------------- RV5 E4: the derived-friction unit-force solves (behavioural)

/// Probe P's beam and gaps (seeded closed at g and 4g) with a sliding friction
/// support at the middle node's UX, whose normal is derived from the left
/// gap's UY reaction, and a 50 N UX load there. Seeded sliding defers the
/// friction force past the first iterate, so from the second iteration on the
/// loop runs its derived-friction base and unit-force solves. The right gap
/// opens after the first iterate; the left stays closed at g, and for
/// g = 0.03 and 0.09 m its prescribed fold makes the exact and binary64
/// reductions give different displacement bits (asserted below). Invented
/// inputs.
fn kd5_friction_probe_input(g: f64) -> NonlinearFrameSolveInput {
    let mut force = probe_force();
    force[6] = 50.0;
    NonlinearFrameSolveInput {
        node_count: 3,
        elements: probe_p_elements(),
        user_stiffness_elements: vec![],
        curved_bend_elements: vec![],
        force,
        base_restrained_dofs: PROBE_BASE_RESTRAINTS.to_vec(),
        nonlinear_supports: vec![
            NonlinearSupport::gap(
                "gap:i",
                0,
                FrameDof::Uy,
                g,
                GapDirection::PositiveDisplacement,
            )
            .unwrap(),
            NonlinearSupport::gap(
                "gap:j",
                2,
                FrameDof::Uy,
                4.0 * g,
                GapDirection::PositiveDisplacement,
            )
            .unwrap(),
            NonlinearSupport::friction("friction:m", 1, FrameDof::Ux, 0.3).unwrap(),
        ],
        initial_states: vec![
            SupportStateRecord::new("gap:i", ActiveSetState::Active),
            SupportStateRecord::new("gap:j", ActiveSetState::Active),
            SupportStateRecord::new("friction:m", ActiveSetState::Sliding),
        ],
        friction_normal_reactions: vec![],
        derived_friction_normal_reactions: vec![
            DerivedFrictionNormalReaction::from_support_reaction(
                "friction:m",
                0,
                FrameDof::Uy,
                "gap:i",
            )
            .unwrap(),
        ],
        convergence: ConvergenceControl::new(
            "DEC-046-fixture-active-set-count-tightening",
            ConvergencePolicyStatus::Accepted,
            0.0,
            0.0,
            6,
        )
        .unwrap(),
    }
}

/// The derived sliding force of one iteration, recomputed from the loop's own
/// previous iterate and boundary with the given base and unit reactions (the
/// loop's one-candidate Coulomb coupling, in its operation order).
fn kd5_derived_force(
    previous: &NonlinearFrameIteration,
    base_reactions: &[f64],
    unit_reactions: &[f64],
) -> f64 {
    const TANGENT: usize = 6;
    const SOURCE: usize = 1;
    let direction = if previous.displacements[TANGENT] != 0.0 {
        previous.displacements[TANGENT].signum()
    } else {
        -previous.reactions[TANGENT].signum()
    };
    // The loop's normalized sign: zero stays zero.
    let sign = |r: f64| if r == 0.0 { 0.0 } else { r.signum() };
    let reaction = previous.reactions[SOURCE];
    let branch = if reaction == 0.0 {
        sign(base_reactions[SOURCE])
    } else {
        sign(reaction)
    };
    let c = direction * 0.3 * branch;
    let right_hand_side = -c * base_reactions[SOURCE];
    let mut coupling = c * (unit_reactions[SOURCE] - base_reactions[SOURCE]);
    coupling += 1.0;
    open_pipe_stress_frame_kernel::solve_dense(&[vec![coupling]], &[right_hand_side]).unwrap()[0]
}

/// RV5 E4: the loop's derived-friction unit-force solves (and every other
/// linearized solve over several gap iterations) stay on the binary64 legacy
/// path and never reach the formation-checked typed entry. For every
/// iteration after the first that applies the derived friction force, the
/// force is recomputed from the loop's own previous iterate and boundary with
/// base and unit solves on the binary64 path (`solve_linearized_system_evidence`,
/// pinned above). Precondition (the paths differ): the same unit solve through
/// `solve_assembled_with_formation_check` gives different displacement bits
/// and a different derived force, so a loop that routed any of these solves
/// through the formation-checked entry fails the bitwise assertion.
#[test]
fn kd5_nonlinear_loop_unit_force_solves_reach_no_formation_check() {
    const TANGENT: usize = 6;
    let elements = probe_p_elements();
    let k = assemble_global_stiffness(3, &elements).unwrap();
    let assembly = AssemblyEvidence::new(3, &elements, &[], &[], &[]).unwrap();
    for g in [0.03, 0.09] {
        let input = kd5_friction_probe_input(g);
        for mode in [
            LinearSolveMode::DenseScrutiny,
            LinearSolveMode::SparseInteractive,
        ] {
            let ctx = format!("g={g} {mode:?}");
            let result = solve_active_set_frame_with_mode(&input, mode).unwrap();
            let mut checked_iterations = 0;
            for pair in result.iterations.windows(2) {
                let (previous, current) = (&pair[0], &pair[1]);
                let Some(applied) = current
                    .applied_sliding_friction_forces
                    .iter()
                    .find(|a| a.global_dof == TANGENT)
                else {
                    continue;
                };
                let boundary = BoundaryState {
                    dofs: current.active_restrained_dofs.clone(),
                    displacements: current.active_prescribed_displacements.clone(),
                };
                assert!(!boundary.dofs.contains(&TANGENT), "{ctx}: friction sliding");
                let mut unit_force = input.force.clone();
                unit_force[TANGENT] += 1.0;
                let legacy = |force: &Vec<f64>| {
                    crate::solve_linearized_system_evidence(
                        Some(&assembly),
                        &k,
                        force,
                        &boundary,
                        LinearSolveMode::DenseScrutiny,
                    )
                    .unwrap()
                };
                let base = legacy(&input.force);
                let unit = legacy(&unit_force);
                let expected = kd5_derived_force(previous, &base.reactions, &unit.reactions);
                // Precondition: the formation-checked typed entry gives a
                // different unit solve and a different derived force.
                let prescribed: Vec<(usize, f64)> = boundary
                    .dofs
                    .iter()
                    .copied()
                    .zip(boundary.displacements.iter().copied())
                    .collect();
                let free: Vec<usize> = (0..unit_force.len())
                    .filter(|d| !boundary.dofs.contains(d))
                    .collect();
                let checked_unit = assembly
                    .solve_assembled_with_formation_check(
                        &k,
                        &ledger_force(&unit_force),
                        &free,
                        &prescribed,
                        LinearSolveMode::DenseScrutiny,
                        &[],
                        true,
                    )
                    .unwrap();
                assert_ne!(
                    bits(&checked_unit.displacements),
                    bits(&unit.displacements),
                    "{ctx}: precondition, the unit solve paths differ"
                );
                let checked_reactions: Vec<f64> =
                    crate::multiply_matrix_vector(&k, &checked_unit.displacements)
                        .unwrap()
                        .into_iter()
                        .zip(&unit_force)
                        .map(|(internal, applied)| internal - applied)
                        .collect();
                let through_check =
                    kd5_derived_force(previous, &base.reactions, &checked_reactions);
                assert_ne!(
                    through_check.to_bits(),
                    expected.to_bits(),
                    "{ctx}: precondition, the derived force differs through the check"
                );
                // The loop: its derived force is the binary64 path's, bit for bit.
                assert_eq!(
                    applied.force.to_bits(),
                    expected.to_bits(),
                    "{ctx} iteration {}: derived friction force {} (binary64 path {}, formation-checked path {})",
                    current.iteration,
                    applied.force,
                    expected,
                    through_check
                );
                checked_iterations += 1;
            }
            // Not vacuous: the derived-friction solves ran in at least one
            // iteration after the first.
            assert!(
                checked_iterations >= 1 && result.iterations.len() >= 2,
                "{ctx}: {} iterations, {checked_iterations} with a derived friction force",
                result.iterations.len()
            );
        }
    }
}

//! S11-F tests for RV1-N5 (S11 section 6: a detected loss or an unauditable
//! row is Sensitive, never a refusal). Invented dyadic stiffness data; the
//! probe-A cantilever and its terms are S11-K's (`s11k_tests`), rebuilt here.
use super::*;
use crate::load_ledger::{ForceTerm, ForceTermKind, LoadLedger};
use crate::{assemble_global_stiffness, FrameElement, FrameNode, FrameSection};

/// RV1-N5: a ledger row (1e80, -1e80, 1e-300). The audit's radix-scaled
/// expansion cannot represent the 1e-300 net against the 2^265 row scale
/// (`exact_radix` returns a range error), which before S11-F turned the
/// solve into `Err`. Now the solve succeeds with its values unchanged, the
/// row is reported unaudited and the case is Sensitive, never Passed.
#[test]
fn n5_unauditable_ledger_row_is_sensitive_never_an_error() {
    let k = vec![vec![1.0]];
    let mut ledger = LoadLedger::new();
    ledger.push("load:g", 0, 1e80);
    ledger.push("load:minus-g", 0, -1e80);
    ledger.push("load:tiny", 0, 1e-300);
    let force = ledger.finish(1).unwrap();
    assert_eq!(force.values(), &[1e-300]);
    // Precondition: the old audit arithmetic fails on this row.
    assert!(matches!(
        exact_radix(1e-300, -265),
        Err(StructuralError::Range(_))
    ));
    let system = StructuralSystem::assembled(&k, &force, &[0], &[], None, None);
    let solution = solve_assembled_structural_dense(&system).expect("never an Err (section 6)");
    assert_eq!(solution.displacements, vec![1e-300]);
    assert_eq!(solution.report.quality, SolveQuality::Sensitive);
    let report = solution
        .load_fidelity
        .expect("an unaudited row is reported");
    assert_eq!(report.rows.len(), 1);
    let row = &report.rows[0];
    assert_eq!(row.global_dof, 0);
    assert!(row.unaudited.is_some(), "{row:?}");
    assert_eq!(row.guarded_ratio, f64::INFINITY);
    assert_eq!(row.exact_net_bits, 1e-300_f64.to_bits());
    assert_eq!(
        row.sources,
        vec![
            "load:g".to_string(),
            "load:minus-g".to_string(),
            "load:tiny".to_string()
        ]
    );
    // The typed audit entry point itself returns the row instead of an error.
    let direct = audit_load_fidelity(system.system(), &[1e-300], force.terms()).unwrap();
    assert_eq!(direct.rows.len(), 1);
    assert!(direct.rows[0].unaudited.is_some());
}

/// RV1-N5: an audit that cannot run at all (an invalid identified term on the
/// C3-detect entry point) is a Sensitive report, never an `Err`.
#[test]
fn n5_audit_input_error_is_sensitive_never_an_error() {
    let k = vec![vec![2.0]];
    let force = [1.0];
    let system = StructuralSystem {
        stiffness: &k,
        force: &force,
        free_dofs: &[0],
        prescribed: &[],
        contributions: None,
        symmetry: None,
    };
    let bad = [ForceTerm {
        source: "load:a".into(),
        dof: 3,
        kind: ForceTermKind::Term(1.0),
    }];
    assert!(
        audit_load_fidelity(&system, &[0.5], &bad).is_err(),
        "precondition"
    );
    let solution = solve_structural_dense_with_force_terms(&system, &bad).expect("never an Err");
    assert_eq!(solution.report.quality, SolveQuality::Sensitive);
    let report = solution.load_fidelity.unwrap();
    assert!(report.rows.is_empty());
    assert!(report.audit_error.is_some());
}

// ------------------------------------------------ control: audited rows unchanged

const L: f64 = 2.0;

fn cantilever() -> (Vec<Vec<f64>>, Vec<usize>, Vec<(usize, f64)>) {
    let section = FrameSection::new(2.0e11, 7.7e10, 0.01, 8.0e-6, 9.0e-6, 1.7e-5).unwrap();
    let element = FrameElement::new(
        FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
        FrameNode::new(1, [L, 0.0, 0.0]).unwrap(),
        section,
        [0.0, 1.0, 0.0],
    )
    .unwrap();
    (
        assemble_global_stiffness(2, &[element]).unwrap(),
        (6..12).collect(),
        (0..6).map(|d| (d, 0.0)).collect(),
    )
}

/// S11-K's probe-A terms (`s11k_tests::uniform_terms`, full span).
fn uniform_terms(q: f64) -> [(usize, f64); 4] {
    let (a, b) = (0.0_f64, 1.0_f64);
    [
        (
            1,
            q * L * ((b - b.powi(3) + 0.5 * b.powi(4)) - (a - a.powi(3) + 0.5 * a.powi(4))),
        ),
        (
            5,
            q * L
                * L
                * ((0.5 * b * b - (2.0 / 3.0) * b.powi(3) + 0.25 * b.powi(4))
                    - (0.5 * a * a - (2.0 / 3.0) * a.powi(3) + 0.25 * a.powi(4))),
        ),
        (
            7,
            q * L * ((b.powi(3) - 0.5 * b.powi(4)) - (a.powi(3) - 0.5 * a.powi(4))),
        ),
        (
            11,
            q * L
                * L
                * (((-b.powi(3) / 3.0) + 0.25 * b.powi(4))
                    - ((-a.powi(3) / 3.0) + 0.25 * a.powi(4))),
        ),
    ]
}

/// The audited rows' fields (DOF, restrained, m_i, target, ratio, limit and
/// the two bit patterns) for the C3-detect probe-A cases.
pub(super) fn audited_row_signature() -> String {
    let (k, free, fixed) = cantilever();
    let mut out = String::new();
    for (g, n) in [(1e5, 0.3), (1e8, 0.3), (1e80, 1e-8)] {
        let mut terms = Vec::new();
        for (index, q) in [g, n, -g].into_iter().enumerate() {
            for (dof, value) in uniform_terms(q) {
                terms.push(ForceTerm {
                    source: format!("load:{index}"),
                    dof,
                    kind: ForceTermKind::Term(value),
                });
            }
        }
        let mut force = vec![0.0; 12];
        for t in &terms {
            if let ForceTermKind::Term(v) = t.kind {
                force[t.dof] += v;
            }
        }
        let system = StructuralSystem {
            stiffness: &k,
            force: &force,
            free_dofs: &free,
            prescribed: &fixed,
            contributions: None,
            symmetry: None,
        };
        let solution = solve_structural_dense_with_force_terms(&system, &terms).unwrap();
        for row in solution.load_fidelity.expect("flagged").rows {
            out.push_str(&format!(
                "{g:e} {} {} {} {:016x} {:016x} {:016x} {:016x} {:016x}\n",
                row.global_dof,
                row.restrained,
                row.operation_count,
                row.target.to_bits(),
                row.guarded_ratio.to_bits(),
                row.completeness_limit.to_bits(),
                row.exact_net_bits,
                row.actual_bits
            ));
        }
    }
    out
}

/// The same signature computed by the S11-K audit at main `3488a236a`
/// (before the RV1-N5 refactor), with the probe recorded in
/// `T3/IMPLEMENTATION/S11F/_run_records/n5_control/`.
const BASE_SIGNATURE: &str = include_str!("s11f_n5_base_signature.txt");

/// RV1-N5 control: the refactor moved the per-row audit into
/// `audit_load_row` without changing its arithmetic. Every audited
/// (flagged) row's m_i, target, ratio and completeness limit are bit-identical
/// to the S11-K audit's on the same cases.
#[test]
fn n5_audited_rows_are_bit_identical_to_the_s11k_audit() {
    let signature = audited_row_signature();
    assert!(signature.lines().count() >= 6, "{signature}");
    assert_eq!(signature, BASE_SIGNATURE);
}

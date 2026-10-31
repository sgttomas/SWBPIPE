//! The admission range (design §3.2 item 5, "Admission range"; review
//! round 3): a nonzero formed or re-formed entry outside [2⁻³⁰⁰, 2³⁰⁰] in
//! magnitude refuses the pencil at entry, by name
//! (`MODAL_PENCIL_RANGE_UNSUPPORTED`). Each review-round-3 range witness is
//! asserted refused here; the witness that the review called "the shape
//! witness with h = 2¹⁰⁰²" is reconstructed as a pencil entry h, since only
//! that parameter reached this suite.

use open_pipe_stress_modal_eigen::admission::{
    PencilMatrix, PencilRangeUnsupported, PENCIL_RANGE_UNSUPPORTED,
};
use open_pipe_stress_modal_eigen::dd::scale_by_pow2;
use open_pipe_stress_modal_eigen::dense::{solve_dense_pencil, DenseError, DenseOptions};
use open_pipe_stress_modal_eigen::modal_quantities::{
    Dof, FreeDof, ModalBasis, ModalQuantityError,
};
use open_pipe_stress_modal_eigen::orthogonality::{m_orthogonality, OrthogonalityError};
use open_pipe_stress_modal_eigen::reformation::mass::ComponentBody;
use open_pipe_stress_modal_eigen::reformation::section::MassBasis;
use open_pipe_stress_modal_eigen::reformation::{
    formation_difference, ElementOperand, FormationError, ReformationFailure, ReformedPencil,
    SpringOperand,
};
use open_pipe_stress_modal_eigen::sturm::{ShiftPencil, SturmError};
use open_pipe_stress_modal_eigen::SymmetricMatrix;
use open_pipe_stress_sparse_direct::SymmetricMatrixEntry;

/// 2^e exactly, subnormals included.
fn p(e: i32) -> f64 {
    scale_by_pow2(1.0, e)
}

/// Test-local resolution-floor parameters c_μ and κ_μ (design §3.2 item 5);
/// the production values are recorded under DEC-026, not here. On the
/// equilibrated pencil (review round 4), with c_μ = 1, the review-round-3
/// artifact lies 2.4× below δ_μ and the smallest μ of the admitted fixture
/// cases at least 1.6·10⁷ above it, except E-RIGID-SCALE-1e6, whose six
/// stiff modes lie 5 to 330 times below it (README: an open conflict).
const RESOLUTION_FLOOR_CONSTANT: f64 = 1.0;
const RESOLUTION_FLOOR_MULTIPLE: f64 = 10.0;

/// Test-local dense options (as in `sturm_counts.rs`).
fn options() -> DenseOptions {
    DenseOptions {
        infinite_mu_relative_threshold: 1e-12,
        mass_pivot_relative_threshold: 1e-12,
        resolution_floor_constant: RESOLUTION_FLOOR_CONSTANT,
        resolution_floor_multiple: RESOLUTION_FLOOR_MULTIPLE,
        max_ql_iterations_per_eigenvalue: 30,
    }
}

fn range(matrix: PencilMatrix, row: usize, col: usize, value: f64) -> PencilRangeUnsupported {
    PencilRangeUnsupported {
        matrix,
        row,
        col,
        value,
    }
}

/// Dense and Sturm entry: the scalar K = 2⁻⁵²⁰, M = 2⁵²⁰; K = 1 with
/// M = 2⁻¹⁰⁴⁰ (subnormal); and the shape witness K = diag(1, h), h = 2¹⁰⁰².
#[test]
fn dense_and_sturm_refuse_pencils_outside_the_range() {
    let scalar = |x: f64| SymmetricMatrix::from_diagonal(&[x]).unwrap();
    let cases = [
        (
            scalar(p(-520)),
            scalar(p(520)),
            range(PencilMatrix::Stiffness, 0, 0, p(-520)),
        ),
        (
            scalar(1.0),
            scalar(p(-1040)),
            range(PencilMatrix::Mass, 0, 0, p(-1040)),
        ),
        (
            SymmetricMatrix::from_diagonal(&[1.0, p(1002)]).unwrap(),
            SymmetricMatrix::from_diagonal(&[1.0, 1.0]).unwrap(),
            range(PencilMatrix::Stiffness, 1, 1, p(1002)),
        ),
    ];
    for (k, m, expected) in &cases {
        assert_eq!(
            solve_dense_pencil(k, m, &options()),
            Err(DenseError::PencilRangeUnsupported(*expected))
        );
        assert_eq!(
            ShiftPencil::from_dense(k, m),
            Err(SturmError::PencilRangeUnsupported(*expected))
        );
        assert_eq!(expected.code(), PENCIL_RANGE_UNSUPPORTED);
    }
    // The boundaries themselves are admitted.
    let k = SymmetricMatrix::from_diagonal(&[p(300), p(-300)]).unwrap();
    let m = SymmetricMatrix::from_diagonal(&[p(-300), p(300)]).unwrap();
    assert!(solve_dense_pencil(&k, &m, &options()).is_ok());
    assert!(ShiftPencil::from_dense(&k, &m).is_ok());
}

/// One free Ux at each of two nodes, an axis spring of stiffness k and a
/// component body of weight w (a = 1, so m = w) on node 0's Ux, and the
/// second node's Ux spring and body at 1.
fn pencil(k: f64, w: f64) -> Result<ReformedPencil, open_pipe_stress_modal_eigen::reformation::ReformationError> {
    let mut map = vec![None; 12];
    map[0] = Some(0);
    map[6] = Some(1);
    let body = |node, weight| {
        ElementOperand::ComponentBody(ComponentBody {
            node,
            weight,
            acceleration: 1.0,
            center_of_gravity: [0.0; 3],
        })
    };
    let elements = [
        ElementOperand::Spring(SpringOperand::Axis { dof: 0, stiffness: k }),
        ElementOperand::Spring(SpringOperand::Axis { dof: 6, stiffness: 1.0 }),
        body(0, w),
        body(1, 1.0),
    ];
    ReformedPencil::new(2, &map, MassBasis::EffectiveWall, &elements)
}

/// Re-formation entry: the residual witnesses K = M = 2⁻¹⁰²⁰ and
/// K = M = 2¹⁰²², and the formation-difference witness's re-formed
/// K̂ = 2¹⁰⁰⁰, are refused on the assembled re-formed stiffness; its formed
/// K = 2⁻¹⁰⁰⁰ is refused by `formation_difference`.
#[test]
fn reformation_refuses_re_formed_and_formed_entries_outside_the_range() {
    for (k, w) in [(p(-1020), p(-1020)), (p(1022), p(1022)), (p(1000), 1.0)] {
        let error = pencil(k, w).unwrap_err();
        assert_eq!(error.element, 0, "k = {k:e}");
        assert_eq!(
            error.failure,
            ReformationFailure::PencilRangeUnsupported(range(PencilMatrix::Stiffness, 0, 0, k))
        );
    }
    // The mass alone out of range: the body (element 2) is named.
    let error = pencil(1.0, p(-1020)).unwrap_err();
    assert_eq!(error.element, 2);
    assert_eq!(
        error.failure,
        ReformationFailure::PencilRangeUnsupported(range(PencilMatrix::Mass, 0, 0, p(-1020)))
    );
    let admitted = pencil(1.0, 1.0).unwrap();
    let entry = |row, col, value| SymmetricMatrixEntry { row, col, value };
    assert_eq!(
        formation_difference(
            &admitted,
            &[entry(0, 0, p(-1000)), entry(1, 1, 1.0)],
            &[entry(0, 0, 1.0), entry(1, 1, 1.0)],
        ),
        Err(FormationError::PencilRangeUnsupported(range(
            PencilMatrix::Stiffness,
            0,
            0,
            p(-1000)
        )))
    );
}

/// Orthogonality entry: M = diag(2⁻¹⁰²², 2⁻¹⁰²², 0).
#[test]
fn orthogonality_refuses_a_mass_outside_the_range() {
    let m = SymmetricMatrix::from_diagonal(&[p(-1022), p(-1022), 0.0]).unwrap();
    let (a, b) = ([1.0, 0.0, 0.0], [0.8, 0.6, 0.0]);
    assert_eq!(
        m_orthogonality(&m, &[&a, &b]),
        Err(OrthogonalityError::PencilRangeUnsupported(range(
            PencilMatrix::Mass,
            0,
            0,
            p(-1022)
        )))
    );
}

/// Participation entry: the shape witness's h = 2¹⁰⁰² as a mass entry.
#[test]
fn modal_basis_refuses_a_mass_outside_the_range() {
    let m = SymmetricMatrix::from_diagonal(&[1.0, p(1002)]).unwrap();
    let dofs = [
        FreeDof {
            node: 1,
            dof: Dof::Ux,
        },
        FreeDof {
            node: 1,
            dof: Dof::Rz,
        },
    ];
    let z = [0.0, 0.0];
    assert_eq!(
        ModalBasis::new(&m, &dofs, [&z, &z, &z], [0.0; 3], 1.0, 1e-6).unwrap_err(),
        ModalQuantityError::PencilRangeUnsupported(range(PencilMatrix::Mass, 1, 1, p(1002)))
    );
}

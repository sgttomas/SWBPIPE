//! The Λ step's gate (DEL-04-01 REQ-015): `verified_inverse_operand` returns Q
//! only when the inverse-norm certificate verified ‖A⁻¹‖∞ ≤ B ≤ Q for the matrix
//! A that the evaluation's `projected` reads, with the elimination of the factor
//! the solve used, and the certificate result belongs to that factor alone.
//!
//! Invented matrices only. The witness-like matrix is
//! blockdiag(4(I − (1 − λ)wwᵀ), 4, 4), w = (1, 1, −1, −1)/2, every entry exact
//! in binary64 (diagonal 3 + λ, off-diagonal ±(1 − λ)); its eigenvalue along w
//! is 4λ, so with λ = 2⁻²⁰ its inverse norm is at least 2¹⁸, and with λ = 1/2
//! at most 1/2 (eigenvalues 2 and 4). Both share one sparsity pattern, order
//! and envelope; only the values differ.
use super::super::inverse_certificate::{CertificateSlot, InverseNormInput, CERTIFICATE_RUNS};
use super::super::sparse::{
    factor_sparse_structural_profile, finish_sparse_confirmation, prepare_sparse_structural,
    SparsePreparedSystem, SparseStiffness, SparseStructuralSystem,
};
use super::super::{
    factor_structural_cholesky, factor_structural_profile, finish_confirmation, prepare_structural,
    PreparedGate, PreparedSystem,
};
use super::*;

fn witness_like(lambda: f64) -> Vec<Vec<f64>> {
    let w = [0.5, 0.5, -0.5, -0.5];
    let mut k = vec![vec![0.0; 6]; 6];
    for i in 0..4 {
        for j in 0..4 {
            // 4·(δ_ij − (1 − λ)·w_i·w_j), exact: 4·w_i·w_j = ±1.
            let coupling = (1.0 - lambda) * (4.0 * w[i] * w[j]);
            k[i][j] = if i == j { 4.0 - coupling } else { -coupling };
        }
    }
    k[4][4] = 4.0;
    k[5][5] = 4.0;
    k
}

const FREE: [usize; 6] = [0, 1, 2, 3, 4, 5];
const FORCE: [f64; 6] = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0];

fn dense_system(k: &[Vec<f64>]) -> StructuralSystem<'_> {
    StructuralSystem {
        stiffness: k,
        force: &FORCE,
        free_dofs: &FREE,
        prescribed: &[],
        contributions: None,
        contribution_groups: None,
        symmetry: None,
    }
}

/// The natural order and the envelope of the dense prepared matrix.
fn natural_envelope(prepared: &PreparedSystem<'_>) -> (Vec<usize>, Vec<usize>) {
    let n = prepared.matrix.len();
    let first = (0..n)
        .map(|i| (0..i).find(|&j| prepared.matrix[i][j] != 0.0).unwrap_or(i))
        .collect();
    ((0..n).collect(), first)
}

/// A non-identity order with its envelope (the reverse order, full profile).
fn reversed(n: usize) -> (Vec<usize>, Vec<usize>) {
    ((0..n).rev().collect(), vec![0; n])
}

fn runs() -> usize {
    CERTIFICATE_RUNS.with(std::cell::Cell::get)
}

/// Every entry the certificate reads is bit for bit the entry `projected` reads
/// (`gate_value`, the completion's callback; the attempts read the same
/// prepared matrix), and the elimination is the factor's.
fn assert_certifies_projected(input: &InverseNormInput<'_>, projected: &dyn Fn(usize, usize) -> f64, ctx: &str) {
    let n = input.dimension();
    assert_eq!(n, FREE.len(), "{ctx}");
    for r in 0..n {
        for c in 0..n {
            assert_eq!(input.entry(r, c).to_bits(), projected(r, c).to_bits(), "{ctx}: ({r}, {c})");
        }
    }
}

// --------------------------------------------- item e: the matrix certified

#[test]
fn the_certified_matrix_is_the_matrix_projected_reads_in_every_representation() {
    for lambda in [0.5, 2f64.powi(-20)] {
        let k = witness_like(lambda);
        let system = dense_system(&k);
        let prepared = prepare_structural(&system).unwrap();
        // The scaled matrix differs from K (equilibration), so certifying K
        // itself would be caught here.
        assert!(prepared.scale_exponents.iter().any(|&e| e != 0));
        let projected = |r: usize, c: usize| prepared.gate_value(r, c);

        let cholesky = factor_structural_cholesky(&prepared).unwrap();
        let input = cholesky.inverse_norm();
        assert_certifies_projected(&input, &projected, "dense Cholesky");
        assert!(input.supplied().is_none(), "dense Cholesky: natural elimination");

        for (name, (order, first)) in [("natural", natural_envelope(&prepared)), ("reversed", reversed(6))] {
            let profile = factor_structural_profile(&prepared, &order, &first).unwrap();
            let input = profile.inverse_norm();
            assert_certifies_projected(&input, &projected, &format!("dense profile {name}"));
            assert_eq!(input.supplied(), Some((order.as_slice(), first.as_slice())), "dense profile {name}");
        }

        // The confirmation attempt evaluates on its factor's input.
        let gate = factor_structural_cholesky(&prepared).unwrap();
        let (_, attempt) = finish_confirmation(&gate).unwrap();
        assert!(attempt.factor.inverse_norm().same_slot(&gate.inverse_norm()));
        assert_certifies_projected(&attempt.factor.inverse_norm(), &|r, c| prepared.matrix[r][c], "dense attempt");

        let stiffness = SparseStiffness::from_dense(&k).unwrap();
        let sparse_system = SparseStructuralSystem::new(&stiffness, &FORCE, &FREE, &[], None, None);
        let sparse = prepare_sparse_structural(&sparse_system).unwrap();
        let sparse_projected = |r: usize, c: usize| sparse.gate_value(r, c);
        for (name, (order, first)) in [("natural", natural_envelope(&prepared)), ("reversed", reversed(6))] {
            let factor = factor_sparse_structural_profile(&sparse, &order, &first).unwrap();
            let input = factor.inverse_norm();
            assert_certifies_projected(&input, &sparse_projected, &format!("sparse {name}"));
            assert_eq!(input.supplied(), Some((order.as_slice(), first.as_slice())), "sparse {name}");
            // The kept factor: its stored bits against a later prepared system
            // that `finish_sparse_structural_on` admits.
            let shared = factor.share();
            let later = prepare_sparse_structural(&sparse_system).unwrap();
            let input = shared.inverse_norm();
            assert_certifies_projected(&input, &|r, c| later.gate_value(r, c), &format!("kept {name}"));
            assert_eq!(input.supplied(), Some((order.as_slice(), first.as_slice())), "kept {name}");
        }
        // The sparse attempt's factor field is private to `sparse`; that its
        // `evaluate_at` passes `self.factor.inverse_norm()` is pinned by the scan
        // below, and its factor's input is the sparse factor's checked above.
        let (order, first) = natural_envelope(&prepared);
        let gate = factor_sparse_structural_profile(&sparse, &order, &first).unwrap();
        assert!(finish_sparse_confirmation(&gate).is_ok());
    }
}

// ------------------------------------------------------ the helper's outcomes

fn verified_or_detail(est: f64, input: InverseNormInput<'_>) -> Result<InverseOperand, String> {
    verified_inverse_operand(est, input).map_err(|f| f.detail())
}

#[test]
fn the_helper_returns_q_only_when_verified_in_every_representation() {
    let good = witness_like(0.5);
    let bad = witness_like(2f64.powi(-20));
    for (k, verifies) in [(&good, true), (&bad, false)] {
        let system = dense_system(k);
        let prepared = prepare_structural(&system).unwrap();
        let (order, first) = natural_envelope(&prepared);
        let stiffness = SparseStiffness::from_dense(k).unwrap();
        let sparse_system = SparseStructuralSystem::new(&stiffness, &FORCE, &FREE, &[], None, None);
        let sparse = prepare_sparse_structural(&sparse_system).unwrap();
        let cholesky = factor_structural_cholesky(&prepared).unwrap();
        let profile = factor_structural_profile(&prepared, &order, &first).unwrap();
        let sparse_factor = factor_sparse_structural_profile(&sparse, &order, &first).unwrap();
        let kept = factor_sparse_structural_profile(&sparse, &order, &first).unwrap().share();
        let inputs = [
            ("dense Cholesky", cholesky.inverse_norm()),
            ("dense profile", profile.inverse_norm()),
            ("sparse", sparse_factor.inverse_norm()),
            ("kept", kept.inverse_norm()),
        ];
        // est = 1: Q = mul_up(1024, 1), above the good matrix's scaled inverse
        // norm and far below the bad one's (≥ 2¹⁸ before scaling).
        for (name, input) in inputs {
            let q = mul_up(1024.0, 1.0);
            match verified_or_detail(1.0, input) {
                Ok(operand) => {
                    assert!(verifies, "{name}: the witness-like matrix must not verify");
                    assert_eq!(operand.limit.to_bits(), q.to_bits(), "{name}: the operand is Q, unchanged");
                    assert!(operand.bound > 0.0 && operand.bound <= operand.limit, "{name}: {operand:?}");
                }
                Err(detail) => {
                    assert!(!verifies, "{name}: the well-conditioned matrix must verify");
                    assert_eq!(detail, "inverse-norm bound not verified", "{name}");
                }
            }
            // Not verified whatever the matrix, when Q is not usable or is
            // below the true norm (est 2⁻³⁰: Q = 2⁻²⁰ < 1/4 ≤ ‖A⁻¹‖∞ here).
            for est in [f64::NAN, f64::INFINITY, 0.0, -1.0, 2f64.powi(-30)] {
                assert_eq!(
                    verified_or_detail(est, input).unwrap_err(),
                    INVERSE_NORM_NOT_VERIFIED,
                    "{name}: est {est:e}"
                );
            }
        }
    }
}

/// The not-verified path through `check`: the published reason is the
/// existing unavailable failure with the stable detail, and no bound inputs.
#[test]
fn check_publishes_the_unavailable_detail_when_not_verified() {
    let matrix = vec![vec![2.0]];
    let k = {
        let mut k = vec![vec![0.0; 6]; 6];
        k[0][0] = 2.0;
        k
    };
    let mut ledger = crate::load_ledger::LoadLedger::new();
    ledger.push("test input", 0, 2.0);
    let force = ledger.finish(6).unwrap();
    let source = FormationSource {
        node_count: 1,
        springs: vec![(0, 2.0)],
        ..FormationSource::default()
    };
    let contributions = [super::super::StiffnessContribution { row: 0, col: 0, value: 2.0 }];
    let prescribed = [(1, 0.0), (2, 0.0), (3, 0.0), (4, 0.0), (5, 0.0)];
    let system = StructuralSystem {
        stiffness: &k,
        force: force.values(),
        free_dofs: &[0],
        prescribed: &prescribed,
        contributions: Some(&contributions),
        contribution_groups: None,
        symmetry: None,
    };
    let u = [1.0 + 2.0f64.powi(-40), 0.0, 0.0, 0.0, 0.0, 0.0];
    let inexact = |rhs: &[f64]| Ok(vec![rhs[0] / 2.0 * (1.0 + 2.0f64.powi(-25))]);
    let run = |est: f64| {
        let slot = CertificateSlot::default();
        let input = InverseNormInput::new(
            super::super::inverse_certificate::CertifiedMatrix::Dense {
                matrix: &matrix,
                elimination: super::super::inverse_certificate::Elimination::Natural,
            },
            &slot,
        );
        check(&system, &source, Some(&force), &[0], &u, (1.0, est, 2.0), &|i, j| k[i][j], &|_, _| 2.0, &inexact, input)
    };
    // ‖[2]⁻¹‖∞ = 1/2: est 2⁻¹² gives Q = 1/4 + ulp < 1/2, not verifiable;
    // est 1/2 gives Q ≈ 512, verified, and the bound inputs are published.
    let (finding, bound) = run(2f64.powi(-12));
    assert!(bound.is_none());
    match finding.map(|f| f.reason) {
        Some(FormationCheckReason::FormationCheckUnavailable { detail }) => {
            assert_eq!(detail, "inverse-norm bound not verified")
        }
        other => panic!("{other:?}"),
    }
    let (finding, bound) = run(0.5);
    assert!(finding.is_none(), "{finding:?}");
    assert!(bound.unwrap().lambda_sigma > 0.0);
}

// ------------------------------------- same-case binding: no result crosses

/// Two factors of matrices with identical dimension, pattern, order and
/// envelope, one verifying and one not, asked with identical est (so
/// identical Q bits), in both orders: each obtains its own result, the
/// certificate runs once per factor, and repeating either changes nothing.
#[test]
fn no_factor_obtains_another_factors_result() {
    let good = witness_like(0.5);
    let bad = witness_like(2f64.powi(-20));
    let (gs, bs) = (dense_system(&good), dense_system(&bad));
    let (gp, bp) = (prepare_structural(&gs).unwrap(), prepare_structural(&bs).unwrap());
    let (go, gf) = natural_envelope(&gp);
    let (bo, bf) = natural_envelope(&bp);
    assert_eq!((go.as_slice(), gf.as_slice()), (bo.as_slice(), bf.as_slice()), "identical order and envelope");
    for r in 0..6 {
        for c in 0..6 {
            assert_eq!(gp.matrix[r][c] != 0.0, bp.matrix[r][c] != 0.0, "identical pattern at ({r}, {c})");
        }
    }
    let (gst, bst) = (SparseStiffness::from_dense(&good).unwrap(), SparseStiffness::from_dense(&bad).unwrap());
    let (gss, bss) = (
        SparseStructuralSystem::new(&gst, &FORCE, &FREE, &[], None, None),
        SparseStructuralSystem::new(&bst, &FORCE, &FREE, &[], None, None),
    );
    let (gsp, bsp): (SparsePreparedSystem<'_>, SparsePreparedSystem<'_>) =
        (prepare_sparse_structural(&gss).unwrap(), prepare_sparse_structural(&bss).unwrap());
    let est = 1.0;
    for good_first in [true, false] {
        // Dense Cholesky, dense profile and sparse factors, both matrices.
        let gc = factor_structural_cholesky(&gp).unwrap();
        let bc = factor_structural_cholesky(&bp).unwrap();
        let gpf = factor_structural_profile(&gp, &go, &gf).unwrap();
        let bpf = factor_structural_profile(&bp, &bo, &bf).unwrap();
        let gsf = factor_sparse_structural_profile(&gsp, &go, &gf).unwrap();
        let bsf = factor_sparse_structural_profile(&bsp, &bo, &bf).unwrap();
        let pairs = [
            ("dense Cholesky", gc.inverse_norm(), bc.inverse_norm()),
            ("dense profile", gpf.inverse_norm(), bpf.inverse_norm()),
            ("sparse", gsf.inverse_norm(), bsf.inverse_norm()),
        ];
        for (name, g, b) in pairs {
            assert!(!g.same_slot(&b), "{name}: each factor owns its slot");
            let before = runs();
            let order: [(InverseNormInput<'_>, bool); 2] = if good_first { [(g, true), (b, false)] } else { [(b, false), (g, true)] };
            for (input, verifies) in order {
                assert_eq!(verified_inverse_operand(est, input).is_ok(), verifies, "{name}, good first {good_first}");
            }
            assert_eq!(runs() - before, 2, "{name}: one certificate per factor");
            // Repeated requests return each factor's own cached result.
            for _ in 0..2 {
                assert!(verified_inverse_operand(est, g).is_ok(), "{name}");
                assert!(verified_inverse_operand(est, b).is_err(), "{name}");
            }
            assert_eq!(runs() - before, 2, "{name}: cached, not recomputed");
        }
        // The kept factor takes the slot of the factor it was kept from, and a
        // fresh factor of the other matrix still gets its own result.
        let kept_bad = factor_sparse_structural_profile(&bsp, &bo, &bf).unwrap();
        assert!(verified_inverse_operand(est, kept_bad.inverse_norm()).is_err());
        let kept_bad = kept_bad.share();
        let before = runs();
        assert!(verified_inverse_operand(est, kept_bad.inverse_norm()).is_err());
        assert_eq!(runs(), before, "the kept factor reuses its own factor's result");
        let kept_good = factor_sparse_structural_profile(&gsp, &go, &gf).unwrap().share();
        assert!(verified_inverse_operand(est, kept_good.inverse_norm()).is_ok());
        assert_eq!(runs(), before + 1);
    }
}

/// The slot is keyed by Q's bits: another Q is computed afresh and not
/// stored, so the first Q's result is what later requests at that Q see.
#[test]
fn another_q_is_recomputed_and_does_not_overwrite() {
    let good = witness_like(0.5);
    let system = dense_system(&good);
    let prepared = prepare_structural(&system).unwrap();
    let factor = factor_structural_cholesky(&prepared).unwrap();
    let input = factor.inverse_norm();
    let before = runs();
    assert!(verified_inverse_operand(2f64.powi(-30), input).is_err());
    assert_eq!(runs(), before + 1);
    assert!(verified_inverse_operand(1.0, input).is_ok());
    assert_eq!(runs(), before + 2);
    assert!(verified_inverse_operand(2f64.powi(-30), input).is_err());
    assert_eq!(runs(), before + 2, "the first Q's result is cached");
    assert!(verified_inverse_operand(1.0, input).is_ok());
    assert_eq!(runs(), before + 3, "another Q is not stored");
}

// ------------------------------------------------ the only way to Q (scan)

/// The production text before each file's first `#[cfg(test)]` module.
fn production(source: &str) -> &str {
    source.find("\n#[cfg(test)]\n").map_or(source, |at| &source[..at])
}

fn body_of<'a>(source: &'a str, header: &str) -> &'a str {
    let start = source.find(header).unwrap_or_else(|| panic!("{header}"));
    let open = start + source[start..].find('{').unwrap();
    let mut depth = 0;
    for (i, ch) in source[open..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return &source[start..open + i + 1];
                }
            }
            _ => {}
        }
    }
    panic!("{header}: unbalanced")
}

/// Pins meaning, not wording: in the Λ files, the only formation of Q (the
/// 1024 multiple of est) is inside the helper; each Λ site obtains its inverse
/// operand from one helper call before Λ is formed; Λσ and the bound inputs
/// are formed only at those two sites; only the factor objects construct the
/// gate's input and slot.
#[test]
fn the_helper_is_the_only_place_q_becomes_an_operand() {
    let check = production(include_str!("formation_check.rs"));
    let skew = include_str!("formation_check/skew.rs");
    let structural = production(include_str!("../structural.rs"));
    let sparse = production(include_str!("sparse.rs"));
    let recovery = production(include_str!("certified_recovery.rs"));
    let basis = production(include_str!("../restraint_basis.rs"));
    let helper = body_of(check, "fn verified_inverse_operand(");
    for (name, text) in [("formation_check.rs", check), ("skew.rs", skew), ("structural.rs", structural), ("sparse.rs", sparse), ("certified_recovery.rs", recovery), ("restraint_basis.rs", basis)] {
        let q = text.matches("1024.0").count();
        let expected = usize::from(name == "formation_check.rs");
        assert_eq!(q, expected, "{name}: Q is formed only in the helper");
        let calls = text.matches("verified_inverse_operand(").count();
        let expected = match name {
            "formation_check.rs" => 2, // its definition and the ordinary site
            "skew.rs" => 1,
            _ => 0,
        };
        assert_eq!(calls, expected, "{name}: helper calls");
        let lambda = text.matches("let lambda_sigma =").count();
        assert_eq!(lambda, usize::from(name == "formation_check.rs" || name == "skew.rs"), "{name}: Λσ formation");
    }
    assert!(helper.contains("mul_up(1024.0, est)"));
    assert!(helper.contains("Failure::Unavailable(INVERSE_NORM_NOT_VERIFIED)"));
    for (name, site) in [("ordinary", body_of(check, "\nfn evaluate<")), ("skew", body_of(skew, "pub(super) fn evaluate<"))] {
        let gate = site.find("verified_inverse_operand(est, inverse)?").unwrap_or_else(|| panic!("{name}: gate"));
        let operand = site.find("let inverse = operand.limit;").unwrap_or_else(|| panic!("{name}: operand"));
        let neumann = site.find("mul_up(inverse, epsilon)").unwrap_or_else(|| panic!("{name}: Neumann"));
        let lambda = site.find("div_up(inverse, denominator)").unwrap_or_else(|| panic!("{name}: Λ"));
        let inputs = site.find("Some(MemberBoundInputs {").unwrap_or_else(|| panic!("{name}: inputs"));
        assert!(gate < operand && operand < neumann && neumann < lambda && lambda < inputs, "{name}: gate before every use of Q");
        assert_eq!(site.matches("let inverse").count(), 1, "{name}: one inverse operand");
    }
    // Production `MemberBoundInputs` literals: the two Λ sites, and the skew
    // lift, which copies Λσ from a coordinate bound it is handed.
    assert_eq!(check.matches("Some(MemberBoundInputs {").count(), 1);
    assert_eq!(skew.matches("Some(MemberBoundInputs {").count(), 1);
    assert_eq!(basis.matches("MemberBoundInputs {").count(), 1);
    assert!(basis.contains("lambda_sigma: bound.lambda_sigma,"));
    for text in [structural, sparse, recovery] {
        assert_eq!(text.matches("MemberBoundInputs {").count(), 0);
    }
    // The gate's input and slot are made only by the factor objects.
    assert_eq!(structural.matches("InverseNormInput::new(").count(), 1);
    assert_eq!(sparse.matches("InverseNormInput::new(").count(), 2);
    assert_eq!(check.matches("InverseNormInput::new(").count() + skew.matches("InverseNormInput::new(").count(), 0);
    assert_eq!(structural.matches("certificate: CertificateSlot::default(),").count(), 2);
    assert_eq!(sparse.matches("certificate: CertificateSlot::default(),").count(), 1);
    assert_eq!(sparse.matches("certificate: self.certificate,").count(), 1);
    // Each confirmation attempt evaluates on its own factor's input.
    assert_eq!(body_of(structural, "pub fn evaluate_at(").matches("self.factor.inverse_norm()").count(), 1);
    assert_eq!(body_of(sparse, "pub fn evaluate_at(").matches("self.factor.inverse_norm()").count(), 1);
}

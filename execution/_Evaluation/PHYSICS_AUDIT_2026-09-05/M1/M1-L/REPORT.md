# M1-L linear numerical audit

## Basis and method

Frozen production basis2be412ccea62bdc4bd96deb082c46d7a792076ea; approved plan seal4732089daf9fbed72d25cbe9e8c57072211b64eb5956d55f096a340d5176d4c2; derivative evidence under decomposition0.12/SCA009/DAG010 parent basis. This specialist creates no authority or lifecycle transition. Task scope is all sparse_direct, diagnostics, performance_harness production logic and frame_kernel reduction/dense-solve helpers. COVERAGE.csv binds exact line spans and hashes. All scoped production logic bodies were read; the rest of frame_kernel belongs to M1-E.

V1 owns existing full-suite execution. This specialist performed two lightweight isolated offline Cargo runs (exit0), plus independent Python Decimal/LAPACK comparison. No production or fixture was edited. A successful harness execution means observations were captured, not that every subject behavior passed.

## Condition observation

**M1-L-001, high severity / high confidence reproduced:** The helper named true_condition_number_2norm returns substantially inaccurate conditioning for an actual existing fixture. Chain8 returns619027.975799394; independent eigh gives2158698.8565037264. Analytical tridiagonal32 returns87.92827178907146 vs440.6885603836566. Diagonal[1e-10,1] returns1 instead of1e10, and singular diagonal[0,1] returns1.

Source cause: jacobi_symmetric_eigenvalues allows max(24,12*n) individual plane rotations, despite naming the loop count sweeps. It silently returns the diagonal even if off-diagonal entries have not converged. true_symmetric_condition_number_2norm ignores eigenvalues at/below1e-9 and all negative eigenvalues when choosing its denominator. These two mechanisms invalidate a general true-condition claim.

The production source was copied byte-for-byte and two exposure wrappers appended solely to reach private helpers. ENVIRONMENT.json records prefix byte count/hash, complete copy hash and exact wrapper bytes. No algorithm was changed. The existing chain8 matrix was constructed by the real frame assembly and reduction. Public reachability is static: run_sparse_default_promotion_observation_suite (597) -> run_sparse_default_promotion_observation (607) -> true_condition_number_for_fixture (950) -> audited helper. The first public suite specification is Chain8 (704). Full nine-record suite was not rerun here. This is an observation-evidence defect; it does not itself prove displacement errors.

## Nonfinite success

**M1-L-002, high severity / high confidence reproduced:** Public solve_dense returns Ok([NaN,NaN]) for finite symmetric A=[[1e308,1e308],[1e308,-1e308]], f=[1e308,-1e308], whose exact finite solution is[0,1]. Sparse factorization correctly rejects this case with a nonfinite pivot error. Public dense and sparse both return successful infinity for scalar0.5*x=1e308. The latter exact result is outside f64 and should be rejected; it is not a finite accuracy comparison.

Both solve implementations validate inputs, but do not validate the final arithmetic/output. Dense elimination additionally fails to reject intermediate overflow. These are extreme numerical robustness witnesses. Their occurrence in everyday piping loads was not established.

## Residual observation

**M1-L-003, high severity / high confidence reproduced:** The existing generated chain1 with finite lateral load1e308 yields dense vector[NaN,inf,0,0,0,3.125e301] and all six independently evaluated residual entries NaN. Its public run_fixture_repeat nonetheless records max_abs_residual0, repeat_delta0, and only the two ordinary configuration warnings. Sparse solution is finite in this case but residual summation overflows to NaNs in two entries; the sparse maximum is also0. The sparse/dense delta is infinity, so the whole record is not universally zero or an overall declared pass.

The cause is f64::max folding from0, which discards NaNs in max_abs_residual/max_abs_delta. Failure records with zero placeholders and zero observations must also not be interpreted as valid zero residuals. The repair should distinguish a finite residual, an unavailable observation, and nonfinite numerical failure without inventing acceptance thresholds.

## Other observations

- Six deterministic integer SPD systems through dense, sparse dense-input and sparse direct-entry paths produced maximum forward error1.332e-15 and maximum independently recomputed backward error2.179e-16. These are measured case results, not all-model accuracy proof.
- Exact prescribed elimination and full-restraint zero-dimensional solve behaved as expected. Repeated/out-of-range constraints, malformed vectors/matrices, nonfinite inputs, invalid permutations and invalid entry indices returned errors. Zero-load SPD solved to zero. Singular matrix was rejected. Negative pivots are recorded; nonsingular indefinite zero-leading-pivot matrices may fail unpivoted LDLt while dense solves them, consistent with the documented sparse limitation.
- **M1-L-004:** k=f=2^-40 has exact solution1 and condition1, but the documented absolute1e-12 pivot guard rejects it. Preserve as a scale-policy limitation; changing numerical classification policy requires its owning decision.
- Sparse symmetry is explicitly a caller contract: only the original lower triangle is consumed. This audit does not label arbitrary nonsymmetric inputs a supported sparse capability.
- Condition/diagnostic classification is supplied-threshold driven. Exact boundary equality is implemented as warning/failure respectively and invalid thresholds/nonfinite ratio fail. Public status records are caller-constructed; enum existence is not proof of consumer handling.

## Reachability

product_physics solve_preview_linear_system (1924-2027) consumes sparse direct-entry output by default, and emits an explicit diagnostic/result basis before dense fallback when sparse assembly or solve fails. Dense scrutiny directly consumes solve_dense. The inspected seam lacks an output finite check. It records negative-pivot counts/proxy/residual as observations. Full downstream publication and its input envelope are owned by I1; no native end-to-end extreme-load result claim is made here.

## Limitations and handoff

The major kernels under this specialist scope are covered statically, with bounded numerical witnesses. No full spectrum of ill-conditioned practical piping models, platform/compiler determinism envelope, resource-exhaustion/allocation behavior, or engineering condition/convergence bar is verified. No fuzz campaign or isolated mutation campaign was run by this specialist. Hardware timing and RSS remain observation-only. No native app was launched. Sparse/dense agreement shares assembly and is not a mechanics oracle. Eight additional DEC053 condition records require post-repair independent rerun; they were not silently assumed correct.

Recommend preserve all three confirmed findings and repair after the whole baseline/fresh-review gate. Solve arithmetic/nonfinite rejection and trustworthy observation aggregation can be bounded independently of new engineering thresholds. Condition evidence needs converged eigenvalue calculation or an honest unavailable result, with correctly handled singular/near-zero spectra and independent analytical tests; do not merely relabel current numbers as verified. Regenerate dependent DEC053 observation evidence after acceptance without rewriting historical packets.

All technical findings remain baseline findings, not repair/lifecycle closure. Rerun commands and evidence bindings are in RERUN.md and MANIFEST.json. Missing broader assurance remains UNKNOWN in the register. Runtime model identifier is unexposed/unknown; Agent2 nondelegation is instruction+config asserted. No children were created.

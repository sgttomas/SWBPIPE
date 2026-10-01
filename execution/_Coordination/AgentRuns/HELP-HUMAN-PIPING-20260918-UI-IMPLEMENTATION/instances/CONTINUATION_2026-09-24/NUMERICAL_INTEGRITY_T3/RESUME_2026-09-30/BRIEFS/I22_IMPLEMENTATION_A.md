# I22 checkpoint A — bounded SI kernel implementation grant (ROOT, 2026-10-01)

ROOT selects DESIGN_PROPOSAL.md plus design_a1/addendum_01/CORRECTION.md,
as independently verified by RV28 and its backcheck. Original proposal SHA256
926dea73178b0ecde07203fdf7fc85e5ce752b1a5f6ead7cf31a30673249b178;
correction 30c907589e80a60407acc139a5805d9941b5df1a5b9dd5a2f1dc845159bae80a;
backcheck 1a71f9b5d4adb9fa85ad26a850e117f1f8f4d306e4b7529a4ca0f19d21172247.
The choice is bare-b conservative certification, not qualified/outward public
radii or a changed physical domain. No additional owner permission is required
for this already accepted direction. A1 stays BLOCKING until corrected code is
independently verified and merged; F2a qualification remains separate and held.

The verified implementation_0 plan is approved with its accounting addendum:
PLAN cc18d71ac1ebe1533ecd022a7807f794062d8fd7cf6f8e27a6804ab0e5e111d8;
plan seal 61c0ecebd66c7ad2516dc3f0d2df375247ea51f449dd8bd757392cd2e7a77125;
accounting clarification c37f2e12e82633fca52f363493e19293679484fb00bc4266f1aed91ccf5b87e2.
ROOT has merged current main into the A1 checkout at
0ce33d7e89a306cfc01f3ce29f421f4dbd02e716; numerical source is unchanged.
Use that as implementation starting HEAD, recording the actual read.

## Exact maintained write set

Only these four files, plus additive I22/implementation_a/** and owned scratch:
- P/core/solver/frame_kernel/src/structural/retained/adaptive.rs
- P/core/solver/frame_kernel/src/structural.rs
- P/core/solver/frame_kernel/tests/retained_k4/publication_tests.rs (new)
- P/core/solver/performance_harness/tests/k6b_export.rs

Nothing else, including verify.rs, exact_sum.rs, existing protected tests/oracles,
VR snapshots/expected-unresolved, H estimates, schemas, unit/interval engine,
libraries, CI, instructions or tools. Return an exact dependency before expansion.

ROOT reserves corrected kernel policy M03-INTEGRITY-MP-v2 in the existing
adaptive.rs exported POLICY / RetainedEvidence.policy mechanism. METHOD_TOKEN
stays contribution_preserving_multiprecision_v1. No separate maintained registry
was found; do not invent one. Old v1 evidence is never relabelled certified.
Later F2a/D2 must recognize the corrected policy explicitly before reliance.

Authorize the plan's typed PublicationPredicate/CertificateIssue and numeric
PublicationEnclosure versus terminal PublicationCertificate semantics, reexports
only as needed. Keep old rejection/terminal precedence and precision ceiling.
The Arc-bound prep/verification-state bundle is approved as the smallest actual
pairing invariant; validate layout/row/precision/source identity before indexing.

Use exactly the reviewed H/error formulas, bare b, public relative predicate
and min(A_exact,A_f64); all old R7 gates remain first and unchanged.
Certify final prescribed-replaced provisional publication; move the exact draft
plus private RU64(H) Box into RetainedSolve. Sentinel is absence, not infinity
as allowance. Preserve the reviewed zero/subnormal/overflow and clone semantics.
No report retention/recompute or public radius field/accessor is granted.

RV28-N1 is mandatory: account for new helper/clone/product-reaches work and
scratch on success/rejection/stop, excluding inherited counters once, preserving
the 19-stage schema, exact case/invocation identities and budget precedence.
Keep the explicitly disclosed legacy ExactAccumulator boundary; no unrelated
instrumentation or invented LME prices. Frozen independent absolute-work
ledgers from the plan addendum must precede reliance on accounting tests.
No self-reported counter may serve as its own oracle.

## Checkpoint and runtime boundary

This grants source/test authoring and formatting of those exact owned files only.
No Cargo build, Rust test, solver case, new probe, mutant, generator --write,
observation refresh, scale job or host-tool operation yet. ROOT will grant a
bounded build/test slot after source checkpoint / current isolated gate check.
Do not silently run checks while writing. Use GIT_OPTIONAL_LOCKS=0 for reads.

Return the small core diff, file hashes, APIs/accounting/lifetimes and unrun tests.
No test/repair pass is implied by authored tests. A code or criterion conflict
comes back immediately; a protected ordinary availability change is not resolved
by changing a fixture, tolerance or accepted outcome. Independent source review
will be a fresh reviewer distinct from designer, RV28 and I22.

The three new fixed source controls in the test plan still require independent
raw-input truth before acceptance; ROOT routes that oracle work. Embedding their
proposed raw primitives is allowed, but do not invent expected results or claim
them verified. C17/B truth stays frozen. Preserve all old evidence.


# Handoff state — vocabulary completion round 2

- State: `N1_FAILED_N2_HELD_N3_PASS_PENDING_CHANGE_CLOSEOUT`.
- Accepted upstream basis: `main@7d5a3f558dfa2e8e902df25fc9a3e813a9ab7048` (PR #599 merge); DAG-010; accepted SCA-009 and DEL-07-09 coverage ledger.
- Authoritative/derivative boundary: decomposition/SCA-009 and landed product bytes remain authoritative upstream truth. The public Chirality App export and this AgentRuns package are derivative evidence.

## Node returns

- N1: `FAILED_INTEGRATED_REVIEW_STOPPED`; focused engine/UI implementation checks passed, but fresh 100%-diff review found two terminal issues. No N1 product or shared state is selected for commit. See `N1_FAILURE.md` and `N1_INTEGRATED_REVIEW.md`.
- N2: `NOT_STARTED_DEPENDENCY_HELD`; row 14 unchanged. See `N2_RETURN.md`.
- N3: `PASS`; deferred G4 derivative export regenerated deterministically with zero boundary findings. See `N3_RETURN.md` and `N3_CHECKS.json`.

## Coverage, audits, and reruns

- Row 14 remains `OPEN / NOT_CLOSED / NOT_LANDED`.
- Row 15 remains `PARTIAL / NOT_CLOSED / PARTIALLY_LANDED`; bend only.
- Row 16 remains `CLOSED / CLOSED / LANDED`.
- Formal pre/post `AUDIT_DECOMP` is parked for the third time because no clearly applicable in-session runner was identified.
- N1 rerun requirements are the exact five-point minimum repair matrix in `N1_INTEGRATED_REVIEW.md`; N2 remains sequenced after accepted-and-committed N1.
- Owner-held PKG-02 runtime/transport/permission choices, `.opsproj`, PDU-011/PDU-047, and `MAINTAINER_REVIEWED` promotion remain unchanged.

## Publication state

- Branch: `codex/piping-vocabulary-r2-20260821`.
- Selected landing: N3 export derivative plus truthful plan/returns/failure/handoff/receipt evidence only.
- Explicit exclusions: all five dirty N1 product files; no unsigned-artifact lane or `artifact-proof` label; no lifecycle, release, issuance, reliance, or professional-approval effect.
- Next owner: `CHANGE` for scoped commit, clean-checkout verification, push, and one PR against `main` without merge.

# Observation probe prepared — not applied or executed

Following the deliberately selected software-defect-diagnosis skill, this TASK
froze the observed log/source basis, separated the five assertion failures and
prepared one bounded observation patch. CAUSE_TABLE.md distinguishes facts from
provisional causes and identifies what each probe must establish. No expectation,
status, fixture, protected criterion or production capability was changed.

PROBE.patch targets only core/product_physics/src/lib.rs after the parent's
review/approval of the temporary probe. All paths in the patch/freeze are relative
to REPO_ROOT, the active numerical checkout. Exact source bytes, candidate bytes
and runtime origins are under _run_records. PROBE_FREEZE.json pins all hashes.
The current real source remains exactly
`37d19221af9900e40d500370c30c156e49f5469a905f20acb3b2e3db6ae54d80`.

The patch adds one temporary test inside the existing tests module, reusing the
exact original private fixture helpers and mutations. It performs14 public runs:
nonfinite-computation2, failed-recovery2, original forward controls4, reverse4,
and unchanged mixed fixture2. Both modes are covered; reverse/forward use both
original seeds. It prints actual typed input, status, numerical quality, complete
diagnostics and complete rows without asserting a new expected code or status.
Existing tests and criteria stay byte-identical.

A cfg(test) print-only observation is added at the existing failed-qualification
branch for the actual mixed load case. It reads outcome, converged flag,
strict_gap_unqualified_reason, final states/u/reactions and every iteration's
strict-gap kind/reason, contact admissibility, active set, DOF/prescribed maps,
u/reactions, sliding forces, residual observations, ordinary structural report
and selected product equilibrium. The original qualifier is still called once;
no predicate, return, state or report is replaced. The probe passes if observation
completes, not if a regression is repaired, so its test status is not acceptance.

## Parent-owned insertion, command and removal

1. Verify actual lib SHA against PROBE_FREEZE.json. If it differs, stop and rebase
   the observation patch deliberately; do not overwrite newer work. Review the
   two patch hunks and preserve the original source as pinned.
2. Apply only PROBE.patch using the parent's ordinary patch tool. Verify the
   resulting SHA equals probe_candidate_sha256 before execution. No other source
   file or manifest is required.
3. Run one focused command under the parent's CPU/target/lock lease:
   `cargo test --manifest-path projects/chirality-piping/core/product_physics/Cargo.toml --lib tests::diagnosis_probe_product_full_five_failures -- --exact --nocapture --test-threads=1`
   Preserve raw output and actual candidate/environment/lock identities. A build
   failure is not a diagnostic reproduction.
4. Remove only the two marked DIAGNOSIS_PRODUCT_FULL blocks (the test and cfg(test)
   observation), or reverse the exact patch if no intervening edit occurred.
   Verify the held before SHA is restored. If newer changes exist, remove only
   the probe hunks and verify them separately; never restore a whole stale file.
5. Interpret the actual printed evidence alongside CAUSE_TABLE.md and the parent's
   contract/whitelist comparison before selecting any expectation or production
   repair. Original failure evidence stays preserved.

The static preparation check proves removing only these additions recovers the
entire original source exactly. This TASK did not apply the patch or execute
Cargo, Rust, build, Git, native or browser commands. No observed replacement codes
or passing mixed metrics are claimed. Actual author: TASK
/root/solver_manager/evidence_primitives under /root/solver_manager via native
followup delegation; no descendants.

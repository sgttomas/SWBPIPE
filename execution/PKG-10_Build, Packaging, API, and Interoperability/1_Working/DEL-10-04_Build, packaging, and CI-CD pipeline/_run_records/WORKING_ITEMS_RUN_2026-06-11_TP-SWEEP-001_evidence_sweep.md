# WORKING_ITEMS RUN — 2026-06-11 — DEC-025 five-surface evidence sweep + F-4 atomic wasm build (TP-SWEEP-001)

- **Tranche:** `TP-SWEEP-001` — human-approved implementation of the D-05
  ruling (`DEC-025` in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12,
  selecting Option D of
  `execution/_Coordination/_DECISIONS/D-05_ci_provider_workflow.md`):
  codify the five-surface local evidence sweep as one deterministic
  commit-bound entrypoint, with the F-4 atomic temp-write-and-rename
  wasm-build fix as rider
  (`plans/VERIFICATION_2026-06-11_operation_seam_unification.md` F-4).
- **Run by:** WORKING_ITEMS (Type 1 persona) working locally inside the
  sealed app tranche; no TASK workers (write scopes were tightly coupled).
- **Authority basis:** `DEC-025` ruling; DEL-10-04 as the owning build/CI
  deliverable (`CHECKING`, mature design authority); no lifecycle change.

## What landed

1. **New deterministic sweep entrypoint**
   `tools/release/run_evidence_sweep.py`: runs the five evidence surfaces
   sequentially in F-4-safe order — (1) cargo crate sweep via
   `check_release_readiness.py --profile cargo --execute`, (2)
   `python3 -m pytest -q tests`, (3) desktop Vitest with
   `npm run build:wasm:desktop` first, (4) Playwright e2e via
   `npm run test:e2e:desktop`, (5) desktop production build — fail-fast,
   with skipped surfaces recorded as `not_run`. Each execute run writes a
   machine-readable summary to
   `validation/evidence/sweeps/SWEEP_<utc>_<commit12>[-dirty].json`
   binding the commit hash, branch, working-tree deltas, runtime versions,
   per-command exit codes/durations, and overall status. Exit code 0 only
   when all five surfaces pass. Dry-run by default; `--execute` runs.
2. **F-4 fix** in `apps/desktop/scripts/build-wasm-engine.mjs`:
   wasm-bindgen glue now writes to a sibling
   `__generated__.tmp-<pid>` directory and is renamed into place
   (old dir swapped out and removed), so a concurrent Vitest/dev-server
   reader never sees a half-written artifact set; stale `.tmp-*`/`.old-*`
   staging dirs from interrupted runs are cleaned first; `.gitignore`
   covers the staging dirs.
3. **Focused tests** `tests/test_evidence_sweep.py` (11 tests): the five
   surfaces in DEC-025 order; wasm build precedes Vitest; local-only
   commands; cargo surface reuses the readiness-script profile;
   commit-hash binding; fail-fast/`not_run` semantics; surface stops at
   its first failing command; summary filename binds commit + dirty
   state; JSON validity; dry-run executes nothing; execute writes the
   summary and returns exit 1 on failure.
4. **Documentation:** `docs/BUILD_AND_RELEASE.md` — §2 records the
   DEC-025 deferral and the private-monorepo hosted-CI prohibition absent
   the §7 authorization; §3 records the atomic swap; new §5.1 documents
   the sweep, summary artifact, merge-gate role, and the
   commit→sweep→evidence-commit→push gate pattern; §7 notes the deferral;
   §9 closes the CI-provider TBD with the `D-05b` follow-up pointer.
   `docs/RELEASE_QUALITY_GATES.md` §10 splits the bundled TBD line into
   ruled items (`DEC-025` evidence-execution location, `DEC-027`
   maintainer quorum) and the remaining D-06 TBDs.

## Evidence

Validation run (working tree, pre-commit, all five surfaces green,
exit 0): summary
`validation/evidence/sweeps/SWEEP_20260612T031241Z_0f402fc48424-dirty.json`
bound to base commit `0f402fc48424` with the tranche deltas listed —
cargo crate sweep pass (5.6s warm), pytest pass (11.1s; includes the 11
new sweep tests), Vitest pass (wasm build 0.9s + 22.5s), Playwright e2e
pass (20.3s; 1/1 in 17.5s), production build pass (11.6s; index chunk at
the standing ~536 kB/500 kB-warning baseline). Focused pre-sweep check:
`python3 -m pytest -q tests/test_evidence_sweep.py
tests/test_release_readiness_script.py` — 19 passed. F-4 path proven in
the same sweep: build logged
`wasm-bindgen --target web --out-dir src/services/wasmEngine/__generated__.tmp-<pid>`
then swapped; no stray staging dirs in `git status`.

The post-commit gate run at the clean tranche HEAD is recorded in the
follow-up summary committed with the evidence-only closeout commit (per
`docs/BUILD_AND_RELEASE.md` §5.1) and in `apps/desktop/SMOKE.md`
TP-MAC-112.

## Boundary review

Local-only evidence tooling: no network surface, no telemetry, no hosted
CI workflow file created; GitHub Actions on the private monorepo remains
prohibited absent the recorded §7 authorization. Invented fixture data
only; no protected standards content; no private project data in the
summary artifact (commands, exit codes, durations, versions, hashes
only). No release-readiness, professional approval, certification,
sealing, authentication, or code-compliance claims; a green sweep is
development evidence only.

## Residuals

- Hosted CI re-decision deferred to `D-05b` (public sanitized-export CI,
  prepared with D-06 at the Phase E lead-up) — register row exists.
- Coverage telemetry in sweeps awaits `D-04b` tooling selection.
- Playwright budgets remain calibrated to maintainer macOS hardware; any
  future hosted location must re-measure (D-05 packet §2.2).

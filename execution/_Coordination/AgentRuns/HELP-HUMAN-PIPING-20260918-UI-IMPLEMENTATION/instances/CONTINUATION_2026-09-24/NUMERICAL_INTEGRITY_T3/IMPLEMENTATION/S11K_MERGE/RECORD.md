# S11-K merge record

- **PR:** https://github.com/sgttomas/chirality/pull/973, merged 2026-09-27 as `3488a236af293762ef47b0101a4c209845f372e7` (merge commit) by ROOT (HELP_HUMAN) under the owner's standing Git authorization.
- **Candidate head:** `1251df9868c98d0ebd5dd0592bed494ad7a49199` (branch `codex/piping-s11k-pr-20260926`; base `6bb3ee490`, main merged forward to `efe938506` with no piping change).
- **Gates on the candidate head:**
  - **RV1 independent complete-diff review:** PASS, 0 BLOCKING (`REVIEW/S11K_REVIEW.md`). The SHOULD-FIX items S1, S2 and S3 and the note N4 were fixed in `1251df986`, and the delta backcheck PASSED (`55b4ac362`).
  - **Hosted CI:** green (all check runs success or skipped).
  - **Surface-4 dual-viewport dispatch:** Piping Desktop E2E run 36280839136, `workflow_dispatch`, target_base `efe938506`, success.
  - **DEC-025 sandboxed sweep:** `dec025/SWEEP_20260927T001934Z_1251df9868c9.json`, overall pass, `working_tree_dirty: false`, toolchain 1.97.1, per-crate targets with a background pruner.
    - cargo crate sweep: pass.
    - pytest: 3023 passed, 32 skipped.
    - vitest: 134 files, 2822/2822.
    - production build: pass.
- **Failed attempt, kept as evidence:** `dec025/failed_attempts/`. Surface 1 passed. Surface 2 then had 487 failed and 23 errors, all `CHECKED-JSON-AUTHORITY-MISSING` or `UNITS-AUTHORITY-MISSING`, because the pre-sweep `git clean -fdX` had removed `core/serialization/canonical_json/target` and `core/units/target`. The fix: both authorities were rebuilt with `tools/serialization/build_checked_json.py` and `tools/units/build_units_authority.py`, excluded from the pruner, and the sweep was re-run. `_COMMON.md` now makes them standing prerequisites (`c4815813b`).
- **Not run for this merge:** the src-tauri suite, which is outside the sweep and hosted CI (T9 item). I1 ran it on the blob-identical development head (114/114), and I2 ran it on K3a over S11-K (114/114). The native macOS witnesses are not applicable: S11-K touches no native path.
- **Paths:** placeholders only — `<WORKTREE>` is the PR worktree, `<SCRATCH>` the ROOT scratch, and `<VENV>`, `<NODE>`, `<HOME>` and `<ROOTHOME>` are host paths.

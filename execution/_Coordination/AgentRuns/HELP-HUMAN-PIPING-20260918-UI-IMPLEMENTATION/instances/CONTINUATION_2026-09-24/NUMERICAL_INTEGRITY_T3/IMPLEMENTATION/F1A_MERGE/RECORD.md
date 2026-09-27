# F1a merge record

- **PR:** https://github.com/sgttomas/chirality/pull/1025. ROOT (HELP_HUMAN) merged it on 2026-09-27 as `134eefc24327dd71ccee5d090e29170140e81f8a` (a merge commit, expectedHeadSha `b91201ee5`), under the owner's standing Git authorization. Main was at `3bfa115b0` (App SCA-APP-012), which has no piping product, `tools/` or `.github/` change, and the merge state was clean.
- **Candidate head:** `b91201ee5fea5028da8511378f024effcf8de3d1`, on branch `codex/piping-f1a-20260927`.
- **Scope (ROOT, F1 split):** F1a is the D-5 evidence line and SUP-17. F1b (sparse wiring and W2 at formation) follows after K1 and K2b.

## The chain

- **F1a:** `559ecb64a` (I7), on K-D5 base `5ae22926e`. It changes:
  - `product_physics/src/lib.rs`: K-D5's `FormationCheck` is carried on `PreviewLinearSolve` and rendered by the linear `append_integrity_report` as one `formation_check:` evidence line, after the S11-G step, in ROOT's F1a templates. SUP-17's under-restraint wording is applied;
  - new `src/f1a_tests.rs` (7 tests);
  - `tests/formation_check_runtime.rs` (a comment only).

  Its records are `IMPLEMENTATION/F1A/`: CHANGE_RECORD, RETURN, and SHA256SUMS over 34 files.
- **Main merge:** `b91201ee5` (ROOT, main `7e0125a7b`). It brings no piping product, `tools/` or `.github/` change (RV6 checked this).

## Gates

- **RV6's independent complete-diff review** of `5ae22926e..559ecb64a`, at head `b91201ee5` (`REVIEW/F1A_REVIEW.md`, sha256 `bf49fb47…`, commit `9cef35cda`): **PASS**, with 0 BLOCKING, 0 SHOULD-FIX and 4 NOTE findings. RV6 verified:
  - the templates, byte for byte;
  - that the nonlinear call site's `None` drops no record: `FormationCheck` exists only on the adapter's formation-checked path, which PP selects only when there are no nonlinear supports;
  - that nothing live reads SUP-17's old text. `V1-C/TEST_ASSERTIONS.json` is a static snapshot, referenced only by hash manifests;
  - that the "both demoted" composition gives the line only.

  RV6 ran the baseline targeted tests (8 lib and 5 in formation_check_runtime) and 6 evasion mutants, all killed at assertions, and matched I7's M1–M9b logs.
- **The DEC-025 sandboxed sweep,** on `b91201ee5`: `dec025/SWEEP_20260927T224505Z_b91201ee5fea.json` (sha256 `83c293ec2852b69b5868fd3b54ab6d3f5384858fb5d9e327c4b0c57568d47065` for the original). Overall **pass**, `working_tree_dirty: false`, `--only-capability sandboxed`. All four surfaces passed:
  - the cargo crate sweep (39 manifests discovered);
  - pytest: 3023 passed, 32 skipped;
  - desktop vitest: 134 files and 2822/2822 tests;
  - the production build.

  The run window was 22:45:01Z–23:25:26Z (2420 s), exit 0 (`dec025/meta.txt`). The full log is `dec025/sweep.log`. Machine paths are replaced with `<WORKTREE>`, `<wt>`, `<VENV>`, `<home>` and `<scratch>`.
- **Hosted CI on `b91201ee5`:** all green.
  - the pull_request E2E run **36356220908**;
  - the full-SHA dispatch run **36356220088** (target_base `7e0125a7b`);
  - the Numerical cargo suite, harness and pec.

## Evidence (I7; `IMPLEMENTATION/F1A/`)

- **Suites:** product_physics 523 passed (1 pre-existing ignore), headless 84, operation_applier 194, self_weight_wasm 14, physics_audit_regression 15, result_export 91, src-tauri 114. numerical_integrity builds.
- **T9** against `5ae22926e`: 112/112 byte-identical. The base hashes equal K-D5's recorded combined-candidate hashes. No committed request is under-restrained, so SUP-17 reaches no T9 output.
- **Mutations:** the no-patch control passes. M1–M8, M9a and M9b are killed at behavioural assertions.
- **Callers:** 83 sites, all classified.
- **The dense trigger ratio:** F1a renders 4.8526 on 122 (sparse 2.4279), where K-D5's CHANGE_RECORD quotes 4.827. This is reconciled from records in RETURN §9. The 4.827 figure is K-D5's adapter-level model, not a tree difference, and F1a changes no solve.
- **Not run:** the gate (ROOT's call: T9's byte-identity and the unchanged solve make it redundant); the Python and desktop TS suites, which have no consumer of the new text (both ran in the sweep); rustfmt, which is not installed for toolchain 1.97.1 (CI has no fmt step).

## Follow-ups

These can ride F1b or the next slice that touches PP:
- **N1 (PP:1116-1118):** `row=none` is outside the ruled template and unreachable (an Estimate record always carries a DOF). No test pins it. Add a one-line code comment saying why.
- **N2 (`source_receipt.rs:914-921`):** the receipt's publication reservation charges 12× the diagnostic message bytes. The line adds about 150–170 bytes per K-D5-demoted case in a receipted captured invocation, negligible against the 64M limit, and it fails closed. Disclose it.

No action:
- **N3:** the SUP-17 code comment at PP:1791 is unchanged. That is disclosed in RETURN §3(ii), and the comment is accurate.
- **N4:** the unit byte-identity oracle covers `equilibrium=None` only. T9 covers the product level.

## Not run for this merge

The native macOS witnesses do not apply, because F1a changes no native path. src-tauri ran in I7's suites (114/114).

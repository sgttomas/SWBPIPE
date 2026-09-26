# WP7 quiet-host runs on the frozen T1 candidate

- **Candidate:** `f3270ea7987b8373266fed93ef1241741b1004af`. The first freeze was `6f42b9be5`; its only change is the literal-include test fix. The merge base is `82b43f9bd`.
- **Toolchain:** `RUSTUP_TOOLCHAIN=1.97.1` (`rustc 1.97.1 (8bab26f4f 2026-07-14)`) set in the environment, matching hosted CI. The host rustup default, stable 1.94.1, was left unchanged (ROOT).
- **Paths:** placeholders only: `<WORKTREE>` is the load-state worktree and `<SCRATCH>` is the manager scratch.
- **Load average:** `/proc/loadavg` (1-, 5- and 15-minute), sampled at the start and end of each run. Other sessions were active on the host.

## Results

| Run | Command (from WORKING_ROOT) | Window (UTC) | Load avg start → end | Result |
|---|---|---|---|---|
| DEC-025 sweep | `run_evidence_sweep.py --execute --only-capability sandboxed --output-dir LSI/_run_records/session7` with `CARGO_INCREMENTAL=0` and **no** `CARGO_TARGET_DIR`, so each crate builds in its own gitignored `target/` (ROOT option (a)). A background pruner removed each crate's `target/`; leftovers were removed after the sweep. | 17:29:04–18:10:39 | 1.28 → 3.20 | **PASS**, `working_tree_dirty: false` (`SWEEP_20260926T172907Z_f3270ea7987b.json`) |
| └ surface 1, cargo crate sweep | `check_release_readiness.py --profile cargo --execute` | | | 39 crates: 1634 passed, 0 failed, 1 ignored |
| └ surface 2, pytest | `pytest -q tests` | | | 3023 passed, 32 skipped, 130 subtests |
| └ surface 3, desktop vitest | `build:wasm:desktop`, then `test:desktop` | | | 134 files, 2822/2822 passed. No timeout; no timeout raised |
| └ surface 5, production build | `build:desktop` | | | pass |
| src-tauri full suite | `cargo test --locked --offline -j 3` in `apps/desktop/src-tauri` (its own target, debuginfo off) | 18:11:19–18:14:29 | 1.70 → 7.46 | 114/114 passed |
| Chromium e2e, source mode, both projects | `CI=1 npx playwright test` with the local override `PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH` (chromium-1194). **Not surface-4 evidence**: the hosted DEC-093 dispatch is ROOT's. | 18:38:27–19:46:15 | 2.58 → 3.39 | 481 passed, 20 skipped. All 20 skips are the pre-existing owner-gated tests in `e2e/ui-foundation/full-cohort-controller.spec.ts` |
| VP-STATIC scratch rerun | `LSI/_run_records/session4/t1_vp_static_run.py` against the runner built from the candidate (sha256 prefix `5f57e567ef4dd9d3`) | 19:46:59–19:48:19 | 1.82 → 1.31 | 507/507 in both modes (467 positive, 40 negative, 98 structural, 14 `checks_passed`). All 42 runner stdin/stdout/stderr artifacts per mode are byte-identical to the recorded run |
| D-GOV-45 leak check | `tools/validation/validate_run_record_leaks.py --base 82b43f9bd --head f3270ea79` (repository root) | | | PASS: 754 changed run-record files, 0 possible credentials |

## Failed attempts, kept as evidence (`failed_attempts/`)

1. **`SWEEP_20260926T165412Z_6f42b9be528a`**, on the first candidate.
   - Conditions: toolchain 1.94.1 (the host default) and a shared `CARGO_TARGET_DIR`.
   - Outcome: surface 1 failed in `operation_applier` (`generated_self_weight`) with E0308, two `serde_json` versions (1.0.150 against 1.0.151).
2. **`SWEEP_20260926T165609Z_6f42b9be528a`.** The same conditions after clearing the `self_weight_wasm` artifacts, as S6 §8 does. It failed the same way, with more tests affected, through `canonical_json`.
3. **`SWEEP_20260926T165749Z_6f42b9be528a`.**
   - Conditions: 1.97.1 and a fresh shared target.
   - Outcome: surface 1 passed. Surface 2 had 1 real failure:
     - the failing test was `test_ci_numerical.py::test_real_rust_literal_includes_require_numerical`;
     - the cause was that `load_reference_source_contract.rs` used `concat!`-built `include_str!` paths (from WP1, `bfef71b19`).
   - Fix: `f3270ea79`.
4. **`SWEEP_20260926T172743Z_f3270ea7987b`.** On 1.97.1 with the warm shared target, surface 1 hit the stale-artifact collision again. The next run switched to ROOT's option (a) and passed.

**Collision cause (routed to T9).** Crates with different `Cargo.lock` files resolve different `serde_json` versions. When those crates share one `CARGO_TARGET_DIR`, stale path-dependency artifacts are reused. T1 changed no `Cargo.lock` or `Cargo.toml`.

## e2e environment note

The first local e2e attempt used hardlinked copies of the sibling worktree's `node_modules`. Those carried that worktree's Vite dependency cache, which duplicated React in the page ("Invalid hook call"; `b3-accessibility` failed to render). With symlinks instead, the same spec passed and the full run above is green. The hardlinked copies had been used only so that the DEC-025 sweep would see a clean tree; that sweep's vitest surface passed.

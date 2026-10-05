# I61 RETURN: U9 gate G8 on the PR head, through the Direct entry (pressure, coexistence, sweep, callers)

**Verdict: PASS on all four items, with no difference and no stop.**
- **The sweep is byte-identical to `u3_grant2_02`'s,** registered and Stale. Main's PR1080 (the source-blocks receipt-order row walk) therefore changes no published byte on the F2a routes.
- **Pressure.** A pressure-bearing request is refused at its D1 clause, with the exact ordinary bytes and no notice. The no-pressure milestone publishes U1's pinned successor (registered) or the ordinary bytes (Stale).
- **Coexistence.** n05 and n06 are admitted and settle by Coexistence, with the exact ordinary bytes.
- **Callers.** No product caller of the retained entries exists.

**Who and when:** I61 (TASK, Type 2), dispatched directly by ROOT, 2026-10-04 from 22:16Z to 22:35Z.

**The candidate:** PR #1082, head `6b9bb19a5f66143101b6364757ecdfa12da455ef` (`codex/piping-f2a-d1-20261004`), on main `5fdc5ab601`.

**Host:**
- **Where it ran:** a `git archive` of the PR head (P/core, P/fixtures and P/schemas) in WT/scratch/i61_u9g8_01/tree, never in WT/f2a-pr.
- **Cargo:** one job of mine at a time, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`.
- **Targets:** WT/targets/i61-u9g8/reg (registered: debug, no RUSTFLAGS) and WT/targets/i61-u9g8/stale (Stale: `RUSTFLAGS=--cfg=i61_u9g8_stale`).
- **The memory guard** (PID 5387) was running.
- **Git:** no writes. Reads used `GIT_OPTIONAL_LOCKS=0`; `hash-object` ran without `-w`.
- **Not run:** nothing native, at solver scale, or DEC-025. The sweep is the usual bounded fixture set.
- **Machine paths:** none in these records.

## 0. The copy is the PR head

`git ls-tree -r` of the head over the three trees gives 1,034 blobs. `git hash-object` of the extracted copy matches every one, except `lib.rs`. That file has 5 appended lines declaring the two disposable `#[cfg(test)]` harness modules (`outputs/lib_rs_delta.diff`):
- `zz_i61_u3g2_sweep.rs`, byte-identical to `R/I61/u3_grant2_01/_run_records/zz_i61_u3g2_sweep.rs`;
- `zz_i61_u9g8_pressure.rs`, new (`_run_records/`).

Both builds report their intended status in every row: `Registered` and `Stale`. Details are in `outputs/archive_and_runs.txt`.

## 1. Pressure refusal with no-pressure success, through the actual Direct entry

The new harness calls `run_linear_static_preview_value_with_retained_direct` and `run_linear_static_preview_value_with_mode` on four requests, in both modes and both builds. `outputs/pressure_{reg,stale}.json` record:
- the value route's status and blocking codes;
- the Direct publication's sha256 and class;
- the count of `RETAINED_PRECISION_UNAVAILABLE` notices;
- the build status, G-A's refusal and the private W1 result.

| Request | Ordinary (value route) | Registered Direct | Stale Direct |
|---|---|---|---|
| **milestone** (`fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json`) | `MECHANICS_SOLVED` | **successor**, admitted (no refusal); its value equals the PR head's committed `retained_precision_milestone_successor_<mode>.json` `source` (sparse and dense) | exact ordinary bytes; refused at D1.1 (`Profile(Stale)`); 0 notices |
| **milestone with pressure** (one `pressure_regions` entry on its only case; nothing else changed) | `MODEL_INCOMPLETE`, `PREVIEW_CONTRACT_VERSION_MISMATCH` | exact ordinary bytes; **refused at D1.5, `Family(Case, PressureRegions)`**; cause `none`; 0 notices | exact; D1.1; 0 notices |
| **PHYS-R4, pressurized** (verbatim port of `tests/f1b_w2_runtime.rs::phys_r4(true)`) | `MODEL_INCOMPLETE`, **`NUMERICAL_INTEGRITY_UNRESOLVED`** (the named refusal) | exact ordinary bytes; refused at D1.3, `Family(Namespace, SchemaVersion)`; 0 notices | exact; D1.1; 0 notices |
| **PHYS-R4 without pressure** (`phys_r4(false)`) | **`MECHANICS_SOLVED`**, no blocking diagnostic (the no-pressure success) | exact ordinary bytes; refused at D1.3; 0 notices | exact; D1.1; 0 notices |

**Every check holds** (`outputs/compare_g8.json`, `pressure[*].checks`):
- the value route's bytes are identical in both builds;
- every non-milestone Direct publication is the exact ordinary bytes, with cause `none` and no notice;
- Stale is always exact at D1.1;
- the milestone's registered successor equals U1's pinned fixture.

**Two notes on what the pressure rows show:**
- **D1's pressure clause itself** (D1.5 `PressureRegions`) is shown by the milestone variant. Its ordinary route refuses on the contract, `PREVIEW_CONTRACT_VERSION_MISMATCH`, because a 0.1.0 request with pressure regions is not an accepted ordinary request. The published bytes are exactly that refusal.
- **The named `NUMERICAL_INTEGRITY_UNRESOLVED` refusal** is PHYS-R4's. Its 0.3.0 namespace is refused earlier, at D1.3. The D1.3 `PressureContract` fact sits behind `SchemaVersion` in clause order.

The sweep's two pressure fixtures (`model_operations/exact_pressure_authoring_model.json`, `product_preview/load_reference/pressure.request.json`) are likewise refused at D1.3, with exact bytes in both modes.

## 2. Exact-block coexistence through the Direct entry

The sweep's Direct rows for the n05 and n06 source-block fixtures, top-level and `ui/`, in both modes: 16 rows. Registered, every one reads:
- class `exact`, publication equal to the value route's bytes;
- report `(Registered, None)`, that is, **admitted**;
- private result **`Coexistence`**.

Stale, every one is exact and refused at D1.1, with cause `none`.

**The grant-2 set, registered Direct** (70 non-error rows):

| Class | Rows | What they are |
|---|---|---|
| exact | 64 | 18 `Coexistence` (the 16 above, plus `numerical_sensitive_torsion_model.json` in both modes), and 46 G-A refusals (D1.3 ×32, D1.4 ×12, D1.9 ×2) |
| successor | 2 | the milestone |
| notice | 4 | `Preparation`, on the `rejected_stress_range` fixtures, as RV89 N-2 established |

The 2 `ERR` rows (`domain/invented_physical_source_of_truth_model.json`, a request parse error) equal base's. This is exactly grant 2's account, and the committed `u3g2_direct_entry_no_w1_refusals_keep_exact_bytes` covers n05 in-tree.

## 3. The control sweep on the PR head

The sweep covers 36 request-shaped fixtures × 9 route/mode outputs = 324 lines per build.

| Build | sha256 | `u3_grant2_02` | Differing rows |
|---|---|---|---|
| Registered | `9a74ff16d42c2b5afe321ee1889e0f307af946f65d3c1ab8f6dd01bd3c539b62` | `9a74ff16…` | **0** |
| Stale | `0e2db8b89745fdf11c1cc3a263e66d901ba134490ab14a1c5e2e83947c452586` | `0e2db8b8…` | **0** |

**PR1080.** The rows include every source-blocks fixture: `rejected_stress_range`, n05/n06, `ui/`, `physics_source` and `load_reference_source`. Through every route (typed, value, Direct, Headless), in both modes and both builds, they are byte-identical. **PR1080 changes no published byte on the F2a routes**, and the Direct classes and causes are unchanged.

A first run, before the successor dump was added to the pressure harness, gave byte-identical sweeps. The figures above are from the second run (`outputs/run_summary.txt`).

## 4. The Direct-caller scan (D-U7-1)

`git grep` at the PR head, over the whole repository except execution records, for `with_retained_direct`, `with_retained_headless`, `run_preview_model_value_with_retained_headless` and `RetainedHeadlessContext`: 39 hits (`outputs/caller_scan.txt`).

| Where | What |
|---|---|
| PP `src/lib.rs` | `:2259` and `:2266`, the two definitions, plus the re-export at `:122` |
| PP `src/retained_memory.rs` | `:2759`/`:2764`/`:2870`, the context type. `:2988`, a call inside `#[cfg(test)]` (the module starts at `:2960`) |
| PP tests | `retained_facade_tests.rs`, `retained_memory_law_tests.rs`, `retained_wire_tests.rs`, `tests/retained_memory_challenge.rs`, `tests/retained_precision_admission.rs` |
| runner/headless `src/lib.rs` | `:740`, the `run_preview_model_value_with_retained_headless` wrapper, which only calls the **Headless** entry (`:772–774`, refused at D1.0). Nothing in the runner's sources or binaries calls the wrapper outside tests |
| runner/headless `tests/retained_precision_admission.rs` | The reviewed registered runner test: Direct at `:97`, on the refused two-case variant, comparing the profile. Its `:165` asserts that the runner source never contains `with_retained_direct` |
| Everywhere else | **None:** `apps/` (src-tauri, desktop TS), `self_weight_wasm`, `operation_applier`, the runner's `main` and bins, and every other crate |

**So no product caller of the Direct entry exists. No workspace other than PP calls it, except the reviewed runner test.**

## 5. Files

- `_run_records/`: `zz_i61_u9g8_pressure.rs` (the harness), `run_g8.sh` (the runner), `compare_g8.py` (the comparer).
- `outputs/`:
  - `sweep_{reg,stale}.tsv` and `pressure_{reg,stale}.json`;
  - `compare_g8.json` (every check) and `compare_g8.txt` (its printout);
  - `caller_scan.txt`, `lib_rs_delta.diff`, `archive_and_runs.txt`, `run_summary.txt`.
- **Kept in WT/scratch/i61_u9g8_01, not pruned:** the archive copy, full logs, the first run (`run1/`) and the two successor dumps (hashes in `archive_and_runs.txt`).
- `SHA256SUMS`.

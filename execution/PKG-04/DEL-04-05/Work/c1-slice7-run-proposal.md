# C1 slice 7 run proposal

Serves RT-U4 / criterion 9 and the C1 plan's slice 7. This is an offline-prepared
proposal, not a window grant, measurement, acceptance or memory-fit result.

**Execution status:** five separately owner-authorized 500-node attempts
have executed and closed:
- the initial attempt stopped on uninitialized telemetry;
- the repaired A500 timed out at the coarse reader-start mark;
- the reader-phase diagnostic at clean `cf8129cd1` timed out with the first header's
  native `validate` pending;
- the route-header run at clean `ee7a7f13b` completed the metadata header in 5.1 s and
  timed out with compulsory registration's native `validate` pending;
- the integrated-main run of `app-c1-main-2470f0108` at clean `01b2655d5` completed
  compulsory registration (native `validate` 103.3 s) and timed out with model-bound
  standing's native `validate` pending (below).

All five grants are consumed. `app-c1-main-4d0bb1858` was never launched and is
**superseded, not qualified** (its compiled native table lacked RT-A3 #99's recorded
pressure-2 bases); its record is kept as history. **No retry, B1000 or further native
interval is authorized.** Exact selected identities and all raw packets remain preserved.

## RT-A4 integration candidate: built offline (2026-11-04; nothing run)

This prepares the next candidate, the **A-only** candidate Agent 0 selected. It was built under
`t3_slot.sh` (bindings below); it is not a run, a measurement or a qualification claim. No App,
worker, solver, sampler or large result was launched, and no timing or RSS was taken. Any run
needs Agent 0's explicit readiness clearance and the owner's approval.

**Source.** Code and corpora head `e37f3dd07f6ff3e860911f6098161a8de6171911` (`e37f3dd07`): the reviewed code at
`1b45e1b1169213fe18e74428fb3874e6904ef222` plus a corpus fix for hosted CI (below). This section
first named `480201bee942d3337b1e3df825ff636779335605`; the review's repairs made that stale.
`1b45e1b11` is not a candidate source either, because hosted CI failed on it.
- `731fb44e8` merges main `bbf384dec` (RT-A4 #98);
- `419678f37`, `861f53e94` and `d16371c24` add the native mirror and regenerate the corpora;
- `480201bee` merges main `aa794eb68` (#102's region binding and `Index::any` determinism,
  §8.47; #97's modal I2b2). `aa794eb68` is still `origin/main` and the merge base;
- the review repairs, `14364f85d` to `1b45e1b11` (below);
- the corpus fix `e37f3dd07` (below), on top of this section's first two documents commits.

The plan's slice 5 entry "On main at `bbf384dec`…" records the conflicts, the mirror, its
regressions and fail-without checks, and the corpora at `480201bee`. The repairs' regressions and
fail-without checks are recorded in their commit messages. The documents commits that carry this
section change nothing the build reads.

**Changed since `480201bee`** (source facts; no attributed cost):
- **F1, the immutable held table** (`14364f85d`). The recorded table now lives in the immutable
  `HeldResult` (`recorded_table`, set by `read_under` at restoration, before the bytes are held).
  A reader's one share, taken under the registry lock (`SolveJobRegistry::held_result`), is the
  bytes and the table together. S, the detail index (rows, lookup, navigator, recovery) and row
  semantics read the share's table only. `SolveJobRegistry::recorded_table` and
  `RestoredSource::recorded_table` are removed, so a disposal or reset during a read can no longer
  drop a refused binding to the current tables. Regression:
  `a_disposal_or_reset_during_a_read_never_changes_the_recorded_table_it_reads`.
- **F2, historical health, display only** (`257a41e69`). `restoredNativeResult` reads the bare
  verdict first, derives the standalone historical standing (`restoredHistoricalStanding`: the
  recorded dispatch's standing under a recorded table, else the bare verdict's; none on a retained
  route), and reads S with those findings and `eligible: false`. The custody keeps it as
  `historicalStanding`, display only; `binding.standing` stays null, so a restored handle is never
  bound to a current standing. `NativeResultsPanel` shows S's health for it with no hold.
- **F3, retained labels and notices** (`a119a2f08`). The native panel's row labels take the
  whole-value panel's routes (`preview_physics`, `retained_preview_physics`, `retained_physics`).
  `knownSemanticNotices` reads a native handle's retained notices as a whole result's
  (`retained-precision-unvalidated`, no class counts). Standing is unchanged.
- **The endpoint-pair CSV dimensions** (`4934aeb3d`). The native Result Detail's endpoint-pair
  CSV took each row's dimension from a member rows do not carry; it now reads the rows' semantics
  from the handle under its held table (`readNativeEndpointPairRows`), as the whole-value export
  does. This export is not one of the driver's steps (the driver streams a group CSV).
- **`recorded_tables.rs:29` and the RT-A4-HS pin** (`43cea6384`, `60d76f2e9`). The module
  documentation line now says the desktop's native custody makes its recorded reads synchronously
  inside `entry_scope` on the worker thread (the plan's open matter for the core owner). It is a
  comment; no code changed. Retained pricing's RT-A4-HS whole-file pin
  (`tools/retained_pricing/rules/o10_rules.py`) moves
  `a8c3984a2d516f1ceb29ad6ad1d726d366e1423fa4b505822fcac33f07d0b718` →
  `c8cf6c1c670c76387b61a5f030555be418af3c622a2c84f5900c2c051c9d3e5d`, and
  `tools/hosted-ci-routing.json`'s `result_export` tree digest moves `45e77c44…` →
  `e5d94960a31f54f85a4920488e65da9fb0b8cfedb06c84c78a51a6881fda4c88`. Neither tool file is a
  build input.
- **The corpora** (`c5a6e0f1c`, `1b45e1b11`; test inputs, not build inputs). The parity test
  `historicalNativeParity.test.tsx` (twelve cases) is new. The native replay requests go from 426
  to **846** (sources 12 → **23**), each answered by its actual command; no earlier key's
  outcome changed. The reopen requests go 8 → **7** (`native_reopen_requests.json`) and 7 → **6**
  (`native_reopen_pre_rt_a4_requests.json`): F2's S request replaces the superseded
  null-standing one. A request on a restored handle read under a recorded table now names that
  table in its key.

- **The App suites' replay keys** (`e37f3dd07`; test inputs, not build inputs). Hosted CI on
  `1b45e1b11` failed Desktop behaviour 1/2 and 2/2: 12 tests in `App.test.tsx`,
  `App.shell.test.tsx` and `App.projectHandlers.test.tsx`, each with no
  `historical-run-context`. Cause: F2's S request (`standing: {eligible: false, findings}`) is a
  new key. The `1b45e1b11` regeneration recorded only four suites, so the App-level reopen paths'
  five new keys had no reply, and the emulated restore failed on the unrecorded request.
  **Classification: test infrastructure, not a product defect.** The actual Rust command answers
  all five with a summary. The fix adds them from a whole-suite recording pass (846 → **851**
  requests, sources unchanged at 23), answered by the actual command; every earlier outcome is
  byte-identical. No assertion, test or product file changed. Full desktop vitest: 393 files,
  7,555 passed, 1 todo. `tsc -b` is clean.

**Repair basis.** This candidate carries **repair A only**: #102's region binding and the
`Index::any` determinism, on main at `aa794eb68`. It does **not** carry repair B (#106, one
pressure evidence pass per dispatch, open at `2b21c367c`; not an ancestor of this head). B
enters only after the #105 + B registration, which follows RT-A3 increment 4. The candidate
carries neither RT-A3 increment 4's contract nor PF1b's.

**What the candidate contains** (re-read from the source at `1b45e1b11`: the constants in
`semantic_contract.rs`, `recorded_tables::VARIANTS` and `RECORDED_RUN_TABLE_BASES`, and each
table file's SHA-256, which equals its constant; nothing else is claimed):

| Item | Contents |
| --- | --- |
| Analysis-run record schema | 0.3.0 only (current admission); 0.1.0 and 0.2.0 refused by name. No schema change. |
| Store schema | Unchanged; no migration. Native Save, Record Run and restored-project re-save are unchanged. An unchanged re-save of main's pre-RT-A4 PF project leaves the stored result and record texts byte-identical, and equals the submitted path's bytes (`a_run_recorded_before_rt_a4_reopens_through_the_native_commands`). |
| Current tables (#98) | pressure-2 `10455bf3…`, preview-physics-2 `e7b93f41…`, Timoshenko `ce49ebe0…`, preview retained `26dce9c2…`, physics retained `040ef9a6…`; the pressure-source table `84f37c63…` is unchanged. Unchanged since `480201bee`. |
| Recorded-run bases of pressure-2 | `3e2aac3c…`, `5451fbf3…`, `d46e1963…`, `032e861f…`, `a2db16a5…`, `be29fabc…`, each read as current `10455bf3…` (record validator only). |
| Historical registry (`recorded_tables::VARIANTS`, five members) | preview-physics-2 `1ccc9561…`, Timoshenko `a7b83c37…`, preview retained `b001070e…` (base `1ccc9561…`), physics retained `cf29fd5f…` and `045f6f0d…` (#101's; binding only, `reads_under_variant: false`). Each fixture's bytes hash to its member. Read only on a restored handle holding the record's variant, and on the reopen's findings. The variant is now part of the held result's immutable share (F1). |
| Restored-handle display | F2's standalone historical health (S read with the historical findings, `eligible: false`; never bound to a standing) and F3's retained labels and notices. A fresh solve has neither (`historicalStanding` null). |
| History variants with native regressions | Recorded-basis fixtures 0.7.0, 0.8.0, pre-2a, pre-2b, pre-ce, pre-RT-A4 (each a recorded revision; current model documents also saved runs). The five pre-RT-A4 PF variants as saved runs. Main's recorded revision of the PF-2 run. An unknown-digest control read under the current tables. |

**Path changes relative to `app-c1-main-2470f0108`.** These are source facts, with no
attributed cost. Main moved from `5e12ca44e` to `aa794eb68` through #100, #103, #101, #98, #102
and #97 (first-parent merges); #100 and #103 change tools only.
- **Producer side.** #98 changes the preview-physics producer (`product_physics`:
  `combination_plan.rs`, `preview_physics.rs`, `exact_combinations.rs`, `document_members.rs`;
  `load_case_algebra::fixed_load_count`) and the operation applier, which the rebuilt
  operation-applier Wasm engine carries. #101 (RT-A3 constant effort) changes
  `exact_admission.rs`, `exact_combinations.rs` and `retained_memory.rs`. #97's modal module is
  in `product_physics` but off the session's result path, as at `2470f0108`.
- **Compulsory readers.** They now include #98's PF combination-gate and unsigned-row checks
  (`validate_preview_physics_evidence_with_model`, `preview_combination_gates::check`, with
  `combination_graph.rs` and `preview_combination_operands.rs`), #101's
  `exact_combination_evidence.rs` additions, and #102's region binding
  (`physics_evidence.rs`) and `Index::any` determinism (`u4b_evidence.rs`). Which of these a
  given route reaches is not restated here.
- **Tables** (each file hashed at both heads). #101 moves pressure-2 `a2db16a5…` → `be29fabc…`
  and physics retained `cf29fd5f…` → `045f6f0d…`. #98 moves pressure-2 → `10455bf3…`,
  preview-physics-2 `1ccc9561…` → `e7b93f41…`, Timoshenko `a7b83c37…` → `ce49ebe0…`, preview
  retained `b001070e…` → `26dce9c2…` and physics retained → `040ef9a6…`. Pressure-source
  `84f37c63…` is unchanged. Every earlier digest is now a recorded basis or a registry variant
  (table above), read on a restored handle only.
- **Native custody (C1).** `entry_scope` wraps every native reader entry. A fresh solve's reads
  are the current reader's; the recorded reads, the held-table share (F1) and the historical
  health (F2) apply to restored handles only. On a restored handle the reopen now reads the
  bare verdict before S (it read S first), with the same number of reader calls; S's health
  takes the historical findings.
- **Webview.** #98's display rules (derived-row notes, combination notes, overlays) and their
  native ports; F3's label routes and notices; the endpoint-pair CSV's dimensions. #102 adds
  solve-timeline codes (`solveTimelineCodes.ts`).

The 500-node rack is a fresh solve. No claim is made that #102 changes call 3's
model-bound standing time.

**Rebuild (the recipe; executed once, below).** Use the maintained recipe that built
`app-c1-main-2470f0108`, through `t3_slot.sh`:
- run it from a clean checkout of the head that carries this section;
- first run `build:wasm:desktop`. At `480201bee` the engines were current, with
  operation-applier stamp `sha256:77a2a5ebebe912873d17d8585a04f4084d6d077ebb8779c46b9fff225d759726`,
  self-weight `sha256:1bbefdb42ba0e76451c9ba79660f7782719964de57d991ee0cf5319d390bdd0f`, and
  MBF `sha256:83ec9a7f0eb50f69db89f8fbef8a30b90a432d01993e4fc65c6724f7377aec46` (unchanged);
- use a new App directory `scratch/rt-perf-c1-slice7/app-c1-main-<head>` and an unused
  evidence directory.

These stamps did **not** hold at the built head (below): the one review line in
`recorded_tables.rs` is an input of the operation-applier and self-weight stamps. The harness
bytes are unchanged since `2470f0108` (re-hashed at `1b45e1b11` and at the build):
- `perf.mjs` `7329cca1…a5`;
- `perf-driver.js` `e53468ab…f1`;
- `perf-sampler.py` `846f5493…3d3b`;
- `perf-plan-c1.json` `25e8a62b…6435`.

The binding to record at build is the same set as the table above: App/worker SHA-256,
`perf-binding.json`, embedded `dist_digest`, engine stamps and files, harness, rack-generator
and model hashes, and the Python interpreter. The five consumed attempts, their packets and
the six earlier App directories stay untouched.

**Superseded build: `app-c1-main-24f6b6568`** (kept; not the candidate). It was built from
`24f6b6568`, whose corpora fail hosted CI (above). Its record follows as built; a rebuild from the
fixed head replaces it as the candidate.

**Build as run (2026-11-04, superseded).** From this worktree, exactly clean at
`24f6b656828450405e253cccab6acc4b63d2dc90` (`1b45e1b11` plus the first documents commit of this
section), with the destination and both log names checked absent first:

```sh
cd /Users/ryan/dev/chirality-t3/swbpipe-rt-perf-c1
RT_C1_KIT=/Users/ryan/dev/chirality-t3/scratch/rt-perf-c1-slice7
RT_C1_TARGET=/Users/ryan/dev/chirality-t3/targets/rt-perf-c1
env CARGO_TARGET_DIR="$RT_C1_TARGET" /Users/ryan/dev/chirality-t3/tools/t3_slot.sh npm run build:wasm:desktop > "$RT_C1_KIT/c1-main-24f6b6568-wasm-build.log" 2>&1
env CARGO_TARGET_DIR="$RT_C1_TARGET" /Users/ryan/dev/chirality-t3/tools/t3_slot.sh node apps/desktop/native-smoke/perf.mjs --build --app-dir "$RT_C1_KIT/app-c1-main-24f6b6568" --target-dir "$RT_C1_TARGET" > "$RT_C1_KIT/c1-main-24f6b6568-release-build.log" 2>&1
```

Both exited 0. The slot log records the engine job 09:18:56–09:19:52Z and the App job
09:20:27–09:21:28Z (slot 2), with memguard running and no `exclusive.pending`. The cargo target is
the lane's `targets/rt-perf-c1`. `perf.mjs --build` runs `npm run build` and
`cargo build --release --locked --offline` with `--features native-smoke,tauri/custom-protocol`,
then copies the binary. It does no bundling, code signing, notarization, keychain access or
upload, and none was done. The release log has 44 warnings and no error. No source changed.

**Engine stamps at the built head.** `build:wasm:desktop` rebuilt two engines. The only stamp
input changed since `480201bee` is `core/reporting/result_export/src/recorded_tables.rs`
(`43cea6384`, the documentation line); this was computed by intersecting each engine's input set
(`engineInputs`, 340 / 317 / 28 inputs) with the diff. Whether the rebuilt Wasm bytes equal the
`480201bee` engines' was not checked: those artifacts were not kept.
- operation-applier `sha256:77a2a5eb…` → **`sha256:49a67a3a5c301ba0b4304991f8d5c69e9592148076a90eebfab729be7dbaf947`**;
- self-weight `sha256:1bbefdb4…` → **`sha256:1606bfdd18d01b1527e979c6223b3f3541126d3a83d58012739d4e66730b0c9e`**;
- MBF **`sha256:83ec9a7f0eb50f69db89f8fbef8a30b90a432d01993e4fc65c6724f7377aec46`** (current; nothing built).

**Bindings recorded at build.**

| Selected item | Identity |
| --- | --- |
| Source (clean; binding `source_dirty:false`) | `24f6b656828450405e253cccab6acc4b63d2dc90`; code head `1b45e1b1169213fe18e74428fb3874e6904ef222` |
| App directory | `/Users/ryan/dev/chirality-t3/scratch/rt-perf-c1-slice7/app-c1-main-24f6b6568` |
| App / same-executable worker | `openpipestress-desktop`, 52,755,392 B, SHA-256 `106a944c236f1ae193967c063858d2d7da7fc1abd141a9213a76657c35642107` (built 2026-11-04T09:21:28.335Z) |
| `perf-binding.json` | SHA-256 `230488c30e6f1df680a33615a9711acd3bbffc72442832377a7ece16703fb845` |
| Profile | `cargo build --release --locked --offline`, features `native-smoke,tauri/custom-protocol`, the smoke's `TAURI_CONFIG` (window 1440×920, `backgroundThrottling` disabled) |
| Embedded frontend `dist_digest` | `sha256:82b598a29389d63440a5d10133e48f5c8643fcc9a1a2da8e853fff90f9a3dbc7` (recomputed from `dist/` after the build: equal) |
| Engine stamps | the three above; every engine file's SHA-256 (15 files) in the preparation record (`engines_sha256`) |
| External `perf.mjs` | SHA-256 `7329cca1c7c4de7a8758f3457c54dc00c8f5de9dbbe61db65e13f331844be8a5` (unchanged) |
| External `perf-driver.js` | SHA-256 `e53468ab84d333a83df2305c76e26fc9d33d97f52215e4086fc27b69c14dfff1` (unchanged) |
| External `perf-sampler.py` | SHA-256 `846f5493cc9cc309140cced0b6ff40ec9b92468db10a4f63e85fe654f9e03d3b` (unchanged) |
| `perf-plan-c1.json` | SHA-256 `25e8a62b828c6251ac36836d11f8b5448fc3138dbd4903e99ef453a732966435` (unchanged) |
| Maintained rack generator | SHA-256 `1d8155885f2fa60f481943e989268bef77bc2543860a27291fb9c46685ef314c` (unchanged) |
| Model | `linear-ex-n0500.json`, 1,828,475 B, raw SHA-256 `42567114a27415b7b042bb9a91d49c8dc640d534953616e5b7efb66b2a2025eb`, JCS SHA-256 `0d45797d8aacada487ef418ce0a776239f6fd9801ed315c1a0f6e1ca2e25466c` (raw bytes unchanged; JCS carried from the `2470f0108` record, where it was recomputed) |
| Python (bound) | `/usr/bin/python3` → Xcode 3.9.6, real executable SHA-256 `b971331ab9beb1dac075006955f035a97332eca21187c4de615ae58346713e2f` (unchanged; see the readiness blocker below) |
| Preparation record | `scratch/rt-perf-c1-slice7/c1-main-24f6b6568-preparation.json`, SHA-256 `6ebe439904b9d8dac7eda2715cf8b3ada19dedabd08a33846ce998c3dfa010da` |

**Runtime equivalence against the bound source.** The build's actual reads come from cargo's
dep-info for the binary (`targets/rt-perf-c1/release/openpipestress-desktop.d`), which lists
**362** repository files: 333 tracked sources and fixtures, plus the 29 `dist/` files the binary
embeds. It also lists 30 generated files under the target directory, and one absent path
(`src-tauri/capabilities`). The preparation record binds every one of the 362 by SHA-256.
- None of them, and no `.md` file, changed between `1b45e1b11` and the build source. The only
  file that changed is this proposal.
- Since `480201bee`, the build read five changed files: `result_detail.rs`, `result_summary.rs`,
  `solve_jobs.rs` and `stored_custody.rs` (F1), and `recorded_tables.rs` (the comment).
  `cfg(test)` modules such as `native_replay_tests.rs` are not in the dep-info.
- Since `app-c1-main-2470f0108`'s source, the build read 54 changed tracked files, all named
  in the record. They cover the six desktop files of the C1 mirror, the producer and reader
  files under "Path changes", the five registry fixtures and five current tables,
  `exact_report_statements.json`, and two schemas.
- The frontend: Vite emits no module list, so `dist_digest` binds it, recomputed after the
  build and equal. It covers the three engines copied from `public/`. Four runtime webview
  sources changed since `480201bee`: `NativeResultsPanel.tsx`, `knownSemanticLimitations.ts`,
  `nativeReopen.ts` and `nativeResult.ts`. A grep of the bundle finds no replay, reopen or
  parity corpus in `dist/`. Since `2470f0108`, 31 non-test frontend files changed.
- The later documents commits change only this `.md` file, which the build does not read. The
  pre-launch check re-verifies all of this before any launch.

**History registry, display and current admission, as built.** These come from the built sources.
The record binds each of them, and every one is in the dep-info with the same bytes:
`recorded_tables.rs`, `semantic_contract.rs`, `analysis_run_v03_validate.rs`, the five registry
fixtures and the six current tables. It also binds the bundled TypeScript twins
(`analysisRunCompatibility.ts`, `recordedPreRtA4.ts`) and the Python twin `compatibility.py`,
which is not a build input.
- `recorded_tables::VARIANTS`: five members as in the contents table, each fixture's SHA-256
  equal to its member. `reads_under_variant` is true for the first three and false for the two
  physics-retained members.
- `RECORDED_RUN_TABLE_BASES`: six pressure-2 rows (`3e2aac3c…`, `5451fbf3…`, `d46e1963…`,
  `032e861f…`, `a2db16a5…`, `be29fabc…`), each read as current `10455bf3…`.
- Current tables (constant = file SHA-256): pressure-2 `10455bf3…`, pressure-source `84f37c63…`,
  preview-physics-2 `e7b93f41…`, Timoshenko `ce49ebe0…`, preview retained `26dce9c2…`, physics
  retained `040ef9a6…`.

**Readiness blocker found at preparation: the interpreter under `RT_C1_PATH`.** The recorded
recipe derives `RT_C1_PATH` by removing the mise Python directories. Under that PATH `python3` now
resolves to `/opt/homebrew/bin/python3`, Homebrew `python@3.15` 3.15.2 (real executable SHA-256
`0f407a0ac747337b659538c9560e4fdd48b354dbdbe0a143276b56ee23899fb7`). It was linked on the host at
2026-11-03 08:27 local, after the last run's 12:31Z check, ahead of `/usr/bin` on PATH. The
recipe's own `test` line would therefore fail, and the pre-launch check refuses (below).
`/usr/bin/python3` is unchanged. This is a host-environment change, not a source defect. No PATH
change is adopted here; the choice is Agent 0's. The candidate amendment checked below also
removes `/opt/homebrew/` directories. Under it `python3` resolves to `/usr/bin/python3`, but
`perf.mjs`'s `git` changes too: it becomes `/usr/bin/git` instead of `/opt/homebrew/bin/git`.

**Offline evidence at `480201bee`** (before the review repairs; kept as recorded):
- Desktop Vitest: 392 files, 7,543 passed, 1 todo.
- src-tauri lib: 1,145 passed, 13 ignored, 1 failure.
  - The failure is `embedded_agent::provider_store::…corrupted_binding…`, an SQLite `Busy`
    under suite load. That module is untouched by this branch. It passed in the previous full
    run and 3 of 3 times alone.
  - At `d16371c24` the lib suite was 1,146 passed, 13 ignored.
- Python (recorded pre-RT-A4, compatibility, comparison and parity, recorded runs, identity
  inventory) with routing (`test_ci_numerical.py`, `test_hosted_ci.py`): 181 passed and 120
  subtests passed.
- Declarations: 0 errors, the one existing cycle.

At `1b45e1b11` the independent re-review cleared the repairs; their regressions and
fail-without checks are in the commit messages. Hosted CI on the pushed head is still required.
No usable-size, memory-fit, C1-completion, base-machine or acceptance claim follows.

## Integrated-main run result and closure (2026-11-03)

The owner approved exactly this run ("I approve this single run."), and Agent0 delivered
fresh App resource concurrence. A pre-launch check at **12:31:41Z** passed: clean head
`01b2655d5` (only test and documentation files changed since build source `2470f0108`),
binary `bc8e18fc…e134` and binding `58338b55…8b4c` by full SHA-256, frontend, engines,
harness, rack generator and model hashes, Python `/usr/bin/python3` (`b971331a…`) under the
run's own `RT_C1_PATH`, all five earlier candidates and the 30 closure-hashed packet files,
fresh output, no prior attempt, no App process and memguard. App packaging's notarytool
upload (network and local artifact only) was disclosed as possibly overlapping.

`WAIT-EXCLUSIVE` **12:32:03 UTC**; `START-EXCLUSIVE` **12:41:15 UTC** after an RT-A4 slot job
drained (9 min 12 s queue, outside the budget); `END-EXCLUSIVE rc=1` **12:46:18 UTC**:
**303 s** from one-second wrapper timestamps, supervisor **302,188 ms**. No overrun or held
exception. Afterwards no App, worker, sampler or runner process remained, all four lock
slots had no holder and `exclusive.pending` was absent; nothing unrelated was signalled.

The worker published 918,639,329 B at **61,010 ms** from solve request (lifetime footprint
3,653,995,856 B, peak grant 6,375,342,080 B). Host readings stayed known-normal, available
54–60%, pressure 1, no unknown host samples. The 300,000 ms first-paint gate failed
(observed wait 300,474 ms). Cancellation was requested with success_claimed=false.

The fsynced journal has **50 boundary records plus its terminal summary**, no overflow.
Completed native intervals (instrumented monotonic, not CPU or allocation):

| Call | Boundary | Elapsed |
| --- | --- | ---: |
| 1 (`route_header`) | source_prescan / source_parse | 1,530.1 / 2,327.9 ms |
| 1 | validate (metadata only) / source_drop | 435.6 / 1,052.6 ms |
| 2 (registration) | source_prescan / source_parse | 1,657.6 / 2,039.7 ms |
| 2 | **validate (full)** | **103,322.7 ms** |
| 2 | serialize / source_drop | 12.0 / 1,919.1 ms |
| 3 (model-bound standing) | source_prescan / source_parse | 1,883.3 / 1,973.6 ms |

Web `header` returned at 66,867 ms, web `registration` at 176,096 ms, and web `standing`
entered at 176,121 ms. Native call 3 entered its full `validate` at native 118,510.6 ms and
had been pending about **121 s** on the journal's receipt clock when the run ended; it
never emitted an exit. No summary/health, full-verdict agreement, first paint or later gate
ran. In the terms of the proposal's discriminating observations, **the stall moved**:
compulsory registration, still pending at about 219 s in the route-header attempt (binary
without #95's `Index::any` repair), now completes, and model-bound standing is the observed
delay. This journal does not locate an inner validator in call 3, prove CPU use or establish
that it never terminates.

Partial physical observations, separately labelled: App peak footprint 9,177,456,272 B;
aligned sampled App+WebContent maximum 9,659,835,176 B with worker absence inferred zero
after `job_done`, **27 unknown aligned sums**; sequential PID reads, not atomic maxima. The
store passed `quick_check`, with no saved project or run. Records:
`evidence/a500-c1-main1-{launch-check,admission,console,wrapper,summary,closure}` and the
five-file packet `evidence/n0500-c1-main1/`. No usable-size, memory-fit, C1 completion,
base-machine, 1,000/2,000-node or acceptance claim.

## Integrated-main executable (prepared; its one approved run is closed above)

**Source.** `origin/main` at `5e12ca44e6ff8ee1ab94e9edce21214c7fa78ccc` was merged with
`--no-ff` at `4cc0e117d` (no textual conflicts), after the earlier `3983f51cf` merge
(`9df2d8abb`, reconciliation `4d0bb1858`). The independent review of that integration
blocked on a persistence gap. RT-A3 2a/2b moved pressure-2's table digest twice without
keeping a recorded basis, and main's #99 repaired the TypeScript and Python tables. The code
head `2470f0108273f88458bad357d5bb98308a287414` mirrors #99 natively:
- `RECORDED_RUN_TABLE_BASES` gains `d46e1963…` and `032e861f…`, and a test requires it to
  equal both twins;
- main's four verbatim recorded-basis fixtures reopen natively with the webview's findings.

The plan's slice 5 "On main at `5e12ca44e`" entry records the mirror, the regression, its
failure without the rows and the corpora. Relative to `app-c1-main-4d0bb1858`, the runtime
adds:
- the two recorded bases (record-validator reading only);
- main's dynamics I2a/I2b modal module in `product_physics` (a separate crate the desktop
  does not call on the session's result path);
- its `self_weight.rs` change, compiled into the rebuilt self-weight Wasm engine.

Everything else carries over from `4d0bb1858`: #95's `Index::any` map inside the compulsory
readers, RT-A3 2a/2b, S2, the MBF export and the hanger pieces. No C1 gate, limit, journal
boundary or harness byte changed. The perf scripts, plan and rack generator are
byte-identical to the `4d0bb1858` bindings.

The maintained recipe ran through `t3_slot.sh` from clean `2470f0108`, after
`build:wasm:desktop` had rebuilt the two engines that were stale against the merged source
(operation-applier and self-weight; the MBF engine was current). It finished at
**2026-11-03T11:47:51.036Z** without launching the App, worker, sampler or large result.
The binding records `source_dirty:false`.

| Selected item | Identity |
| --- | --- |
| App directory | `/Users/ryan/dev/chirality-t3/scratch/rt-perf-c1-slice7/app-c1-main-2470f0108` |
| App / same-executable worker | `openpipestress-desktop`, 48,854,176 B, SHA-256 `bc8e18fc15def4a42b59d7f7819ae1b90fa831d5c9edd607b97f34d045a0e134` |
| `perf-binding.json` | SHA-256 `58338b553abfce3778e5db18784a9c7e0635b7c8071a19fe55ee87c5474e8b4c` |
| Embedded frontend `dist_digest` | `sha256:41739f12d9511ee688b060aff66c37ac8b5b6043a374a237e27b81459d4c154f` (recomputed from `dist/` after the build: equal) |
| Operation-applier Wasm source stamp | `sha256:9c38b9a631c7f5556234950820ceb7e03919ede0912fd043002a451a5caa5d3a` (rebuilt) |
| Self-weight Wasm source stamp | `sha256:c877840927c00871d56a539e26116655a02217f67327d74cd3f48dd97682f30c` (rebuilt) |
| MBF Wasm source stamp | `sha256:83ec9a7f0eb50f69db89f8fbef8a30b90a432d01993e4fc65c6724f7377aec46` (unchanged) |
| Engine binaries and glue | every file's SHA-256 in the preparation record (`engines_sha256`) |
| External `perf.mjs` | SHA-256 `7329cca1c7c4de7a8758f3457c54dc00c8f5de9dbbe61db65e13f331844be8a5` (unchanged) |
| External `perf-driver.js` | SHA-256 `e53468ab84d333a83df2305c76e26fc9d33d97f52215e4086fc27b69c14dfff1` (unchanged) |
| External `perf-sampler.py` | SHA-256 `846f5493cc9cc309140cced0b6ff40ec9b92468db10a4f63e85fe654f9e03d3b` (unchanged) |
| `perf-plan-c1.json` | SHA-256 `25e8a62b828c6251ac36836d11f8b5448fc3138dbd4903e99ef453a732966435` (unchanged) |
| Maintained rack generator | SHA-256 `1d8155885f2fa60f481943e989268bef77bc2543860a27291fb9c46685ef314c` (unchanged since `4d0bb1858`) |

The selected model is unchanged: `linear-ex-n0500.json`, **1,828,475 B**, 500 nodes / 499
members, raw SHA-256
`42567114a27415b7b042bb9a91d49c8dc640d534953616e5b7efb66b2a2025eb`, JCS SHA-256
`0d45797d8aacada487ef418ce0a776239f6fd9801ed315c1a0f6e1ca2e25466c`. The JCS digest was
recomputed at preparation with `openpipestress_jcs_ijson`, whose sources are unchanged and
which the slot build at `2470f0108` reported current. The corpus depth and actual-record
reopening limits below remain.

Python resolves as `/usr/bin/python3`, Xcode **3.9.6**, real executable
SHA-256 `b971331ab9beb1dac075006955f035a97332eca21187c4de615ae58346713e2f`
(unchanged). The preparing shell resolves `python3` to a newer interpreter first
on PATH, as at the route-header launch. `perf.mjs` runs bare `python3` for the sampler and
host status (`perf.mjs` lines 322, 635, 699 and 827). The proposed command below therefore
sets its own PATH (`RT_C1_PATH`, with the mise Python directories removed), and the
pre-launch check verifies the interpreter under exactly that environment.

The private `scratch/rt-perf-c1-slice7/c1-main-2470f0108-preparation.json` (SHA-256
`994111c6a04e31ac743017df03b2f6c23c85682002837639ca1d451c0bdf2f42`) sits beside the
unmodified `c1-main-4d0bb1858-preparation.json` and `route-header-preparation.json`. It
binds:

- the binding and the frontend;
- all engine files, the scripts and the generator;
- both models and Python;
- the five earlier candidates, `app-c1-main-4d0bb1858` marked superseded;
- both older preparation records' digests;
- the four closure-hashed packets;
- the build log `c1-main-2470f0108-release-build.log` (warnings only, no error) and the
  engine rebuild log.

All five earlier bound copies (`app`, `app-p3`, `app-reader-phases`, `app-route-header`,
`app-c1-main-4d0bb1858`) keep their binary and binding digests. All **30 packet files**
named by the four closure hash maps verified unchanged. The proposed output
`evidence/n0500-c1-main1` does not exist.

Build recipe as run (do not rerun over the preserved candidate; the destination
was checked absent first):

```sh
cd /Users/ryan/dev/chirality-t3/swbpipe-rt-perf-c1
RT_C1_KIT=/Users/ryan/dev/chirality-t3/scratch/rt-perf-c1-slice7
RT_C1_TARGET=/Users/ryan/dev/chirality-t3/targets/rt-perf-c1
env CARGO_TARGET_DIR="$RT_C1_TARGET" /Users/ryan/dev/chirality-t3/tools/t3_slot.sh npm run build:wasm:desktop
env CARGO_TARGET_DIR="$RT_C1_TARGET" /Users/ryan/dev/chirality-t3/tools/t3_slot.sh node apps/desktop/native-smoke/perf.mjs --build --app-dir "$RT_C1_KIT/app-c1-main-2470f0108" --target-dir "$RT_C1_TARGET"
node apps/desktop/native-smoke/perf.mjs --plan --plan-file apps/desktop/native-smoke/perf-plan-c1.json --subset c1-500
```

`perf.mjs --build` refuses stale engines. The plan check lists one run,
`native-linear-ex-n0500`, with a 720 s timeout inside an 840 s budget. Its output is byte
for byte the `4d0bb1858` plan log.

**Checks on this source.** None used the large workload.

- Desktop Rust crate, `cargo test --locked`: **1,142 passed / 13 ignored / 0 failed** in
  each of three full runs after the review fixes (2026-11-03, host load averages about
  7–20). This includes the user-release and native-smoke reach scans, the native verdict,
  route-header, reopen, persistence, solve-job, memory-pool and command-ingress tests.
  - The earlier load-sensitive failure
    (`the_persistence_commands_run_off_the_async_executor_and_reply_with_a_json_body`, about
    38 writers on one store against the 5 s busy timeout) is resolved in test code only. The
    executor test now gives each writer its own store.
  - The new
    `overlapping_record_runs_on_one_store_commit_whole_or_are_refused_busy` asserts the
    store's contract: a complete commit or the named `DatabaseBusy` refusal, no lost
    success, no partial state, `quick_check` ok, and a forced refusal that leaves the store
    unchanged. In the three runs it saw 35 committed / 3 busy, 37 / 1 and 38 / 0.
  - No product code or timeout changed. The plan records the open product matter.
  - Before the fixes, the full crate had 1,140 passed / 1 failed; the same test also failed
    at the merge commit `4cc0e117d`, which has none of these changes.
- Desktop vitest after the engine rebuild: **7,485 passed / 1 todo** (386 files), with zero
  unanswered native replay or reopen requests. This includes the base-reader and
  retained-precision Wasm equivalence tests.
- `tsc -b`, `build:desktop`, and 85 native-smoke node tests.
- CI routing (`test_ci_numerical.py`, `test_hosted_ci.py`): 77 passed (120 subtests).
- `tools.deliverables check`: 0 errors.
- The diff's other selected checks against `e33f73d33`, without the full piping-pytest
  suite, each pytest with the five pinned binaries:
  - retained-pricing tool: 881 passed;
  - model-version literals: 8 passed;
  - benchmark references: 23 passed;
  - analysis-run compatibility and model-document schema: 56 passed.

After the build, the offline connecting checks passed:
- **29 tests** in `nativeResult.connect.test.ts` and `exactSessionSolve.session.test.tsx`;
- **4 native route-header tests**;
- **17 native-verdict tests** (1 ignored).

Logs are kept beside the preparation record. The native replay requests were rebuilt from
empty on this tree (**426**, byte-identical) and answered by the real Rust commands. No
launch test of this binary is claimed.

**One proposed run, pending a new owner decision and resource clearance:**

```sh
cd /Users/ryan/dev/chirality-t3/swbpipe-rt-perf-c1
RT_C1_KIT=/Users/ryan/dev/chirality-t3/scratch/rt-perf-c1-slice7
# The bound interpreter: the mise Python directories removed from PATH.
RT_C1_PATH="$(printf %s "$PATH" | tr ':' '\n' | grep -v '/mise/installs/python' | paste -sd: -)"
# Pre-launch: under exactly this PATH, python3 is /usr/bin/python3 and its real executable is the bound one.
test "$(env PATH="$RT_C1_PATH" sh -c 'command -v python3')" = /usr/bin/python3
test "$(env PATH="$RT_C1_PATH" python3 -I -c 'import hashlib,os,sys; print(hashlib.sha256(open(os.path.realpath(sys.executable),"rb").read()).hexdigest())')" = b971331ab9beb1dac075006955f035a97332eca21187c4de615ae58346713e2f
/Users/ryan/dev/chirality-t3/tools/t3_exclusive.sh env -u SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY PATH="$RT_C1_PATH" node apps/desktop/native-smoke/perf.mjs --run --app-dir "$RT_C1_KIT/app-c1-main-2470f0108" --models "$RT_C1_KIT/models" --plan-file apps/desktop/native-smoke/perf-plan-c1.json --subset c1-500 --budget-s 840 --out "$RT_C1_KIT/evidence/n0500-c1-main1"
```

The run's environment selects the interpreter itself. The two `test` lines are the
pre-launch Python check: under exactly the run's `PATH="$RT_C1_PATH"`, `command -v python3`
must resolve to `/usr/bin/python3`, and its real executable (Xcode 3.9.6) must have
SHA-256 `b971331ab9beb1dac075006955f035a97332eca21187c4de615ae58346713e2f`. If either test
fails, do not launch. Both tests passed against this shell on 2026-11-03, offline and with
no launch.

Before launch, also verify:

- the submitting source is clean, and its runtime is unchanged from the selected
  `2470f0108`;
- every table, model, binding, engine and interpreter hash above;
- the five earlier candidates and the 30 packet files;
- the output is still absent.

Static preparation grants no host admission.

The same resource conditions apply as before:

- normal queue, drain and memguard;
- all four exclusive locks, with actual acquisition and closure (or a held
  exception) announced promptly;
- no nested slot or build inside the interval.

The limits are unchanged:

- **900 s** maximum including teardown;
- supervisor **840 s**;
- child **810 s**, with the **30 s** post-run reserve;
- single-run timeout **720 s**;
- first paint **300,000 ms**;
- the fixed **256-boundary** scalar, fsynced journal;
- all existing interaction timeouts.

Any pre-App refusal, timeout, journal overflow, invalid journal, journal write
failure or other failure consumes the attempt. There is no automatic retry, and
no B1000 or 2,000-node attempt. The 12 GiB product limit, fresh host admission,
fresh-unknown stopping rules, process attribution, owned teardown and closure
checks in "Resource limits, cancellation and cleanup" apply in full. Partial
journals, samples and records are preserved on failure.

**What the journal can discriminate.** In the route-header attempt, call 1
(`route_header`) finished all of its native phases in 5.1 s. Call 2 (the
compulsory registration) then ran:

- `args_admit`: 2,064,276-byte body;
- `source_prescan`;
- `inputs_admit`;
- `source_admit`;
- `source_parse`;
- `inputs_parse`.

It entered its full `validate` at native monotonic 8,800.54 ms and had no exit.

**Clock domains.** The figure of record is about **219 s** pending when the supervisor
window ended. Both of its ends are read on one clock, the journal's receipt (epoch) clock:
call 2's `validate` entry at epoch `…316,027` ms and the window end at about `…535,500` ms.
The manager's first report said "about 292 s". That figure subtracted a native monotonic
value from the web first-paint wait. The two clocks have different origins, so their
difference is not a duration, and the 292 s figure is withdrawn.

#95 replaced `Index::any`'s linear scan, O(rows) per
lookup, with a map built once per `Index`. That scan is called per station by
`member_stations` inside these readers. PR #95's record gives shared-host
functional native-runner timings (not measurements): `validate` at 250 nodes
79.8 s on main against 11.8 s with the repair, and at 82 nodes 8.7 s against
2.6 s, with byte-identical outputs. They were not reproduced here, and they do
not predict the 500-node desktop path.

The same fixed IDs separate the outcomes. These are possible observations, not
predictions.

1. **The repair removed this stall.** Call 2 `validate` emits `exit ok`, followed
   by call 2 `serialize` and `source_drop`, then web `registration exit ok`.
   Next, web `standing` enters, with a native call for the model-bound full
   standing (its own prescan, admission, parse and `validate` enter/exit). Then
   web `summary` enters, with `summary_health` / `summary_build` enter/exit. Then
   the actual full-verdict agreement (which may reuse the exact-input full-verdict
   cache) and first paint. Only after first paint do the unchanged paged UI,
   CSV, Record/Save, reopen and P3 steps run.
2. **The stall moved later.** The journal's terminal `incomplete` list names the
   first pending pair. Each pending phase means:
   - `standing` with its native `validate` pending: the model-bound full standing
     is now the slow reader;
   - `summary` with `summary_health` or `summary_build` pending: health or summary
     construction;
   - a pending `source_parse` or `source_drop` in any call: repeated full-source
     materialization or release;
   - all four web phases exited but first paint missing: the delay is after the
     reader awaits, such as result construction or the later session gates.
3. **The repair was not enough here.** Call 2 `validate` is again entered with no
   exit, and the same pair is incomplete. The 500-node full validation is then
   still beyond the window on this build. The journal still cannot name an inner
   validator, CPU use or allocations.

An exit `ok` only means a call returned. Each successful phase grants nothing
beyond its own gate. Registration, model-bound standing, health and actual
full-verdict agreement all stay compulsory, and a restored result stays
historical. No speed, memory fit, first-paint pass, usable size or C1 completion
is promised. Nothing earlier is relabelled as qualified. #59 remains draft;
shared core, clone and pricing writes stay held. ScopeOfWork and dependency
conditions are unchanged: this section reconciles artifact identity and the
combined source only.

## Separate route-header executable: offline ready, no run authorized

Agent0 authorized a separate build/preflight from reviewed, CI-green clean
`c21ec1048244cc8535f130988878d277984fde49`. The maintained release recipe
completed through `t3_slot.sh` at **2026-11-02T05:09:22.537Z**, without launching
the App, worker, sampler or large result. The binding records `source_dirty:false`.
This document's later reconciliation changes no runtime or external harness bytes.
The independent gpt-6-astra/medium review is CLEAR at c21ec1048; exact-head CI
completed with 16 successes including `harness`, one broad-matrix skip, no failures.
No new product code or observer was needed for this preparation.

| Selected item | Identity |
| --- | --- |
| App directory | `/Users/ryan/dev/chirality-t3/scratch/rt-perf-c1-slice7/app-route-header` |
| App / same-executable worker | `openpipestress-desktop`, 47,960,208 B, SHA-256 `5212d86b839c8babcadb918835759804a95bfc53647585f211313f61fbaf0e53` |
| `perf-binding.json` | SHA-256 `1c7fab5b52abfb7b06bb688a42e25271c3a55ebc5f6b889c680d56dd8cab0113` |
| Embedded frontend `dist_digest` | `sha256:54cd4e5121ea2862328bc878be6c3bbbe8ca995d90fddfda8bc29e412ffb59af` |
| Operation-applier Wasm source stamp / binary SHA-256 | `sha256:14ab23c574227e35d6cc5c94e5f7a1dde7d8343d8e4eebfdbbd3c35c624cd54a` / `6f584adcbc7b5233c8418075aa23f15fe33d68999042e30261984f34dedb1a3a` |
| Self-weight Wasm source stamp / binary SHA-256 | `sha256:04c15ac0dded329c0e873985a34b3f73c68fdeab27331cbe02e7925a9539c938` / `5adff8809a205605654b1998d2f0bef405aae7462604b880d5f0647a0ab4a5ce` |
| External `perf.mjs` | SHA-256 `7329cca1c7c4de7a8758f3457c54dc00c8f5de9dbbe61db65e13f331844be8a5` |
| External `perf-driver.js` | SHA-256 `e53468ab84d333a83df2305c76e26fc9d33d97f52215e4086fc27b69c14dfff1` |
| External `perf-sampler.py` | SHA-256 `846f5493cc9cc309140cced0b6ff40ec9b92468db10a4f63e85fe654f9e03d3b` |
| Unchanged `perf-plan-c1.json` | SHA-256 `25e8a62b828c6251ac36836d11f8b5448fc3138dbd4903e99ef453a732966435` |
| Maintained rack generator | SHA-256 `9bbc6b9b1f6f78c11233da9906bf1346d4322a16bfe1b5c5e2069c7914456559` |

Python resolves as `/usr/bin/python3`, Xcode **3.9.6**, to
`/Applications/Xcode.app/Contents/Developer/Library/Frameworks/Python3.framework/Versions/3.9/bin/python3.9`,
SHA-256 `b971331ab9beb1dac075006955f035a97332eca21187c4de615ae58346713e2f`.
Only interpreter/version/clock introspection was executed; no telemetry query or
sampler mode ran. Recheck the resolving interpreter and digest before a run;
a different selection requires reconciliation, not silent substitution.

The existing `linear-ex-n0500.json` is unchanged: **1,828,475 B**, 500 nodes /
499 members, raw SHA-256 `42567114a27415b7b042bb9a91d49c8dc640d534953616e5b7efb66b2a2025eb`,
JCS SHA-256 `0d45797d8aacada487ef418ce0a776239f6fd9801ed315c1a0f6e1ca2e25466c`.
Its static container depth is 7; the corpus depth/actual-record reopening limits
below remain. No deep-header acceptance is inferred. The 1,000-node fixture is
also unchanged and retained, but is not selected by this proposal.

The private `scratch/rt-perf-c1-slice7/route-header-preparation.json` binds the
full engine output hashes, Python, both models, prior candidates and packet
checks. It is a preparation observation, not a new required product tool or
acceptance receipt. All three old bound copies (`app`, `app-p3`,
`app-reader-phases`) retain their original identities; all **25 packet files**
named by the three closure hash maps verified unchanged. Their private result
stores and copies remain for the owner/comparison consumer.

Reproducible build recipe, already completed from clean c21ec1048 (do not rerun
over the preserved candidate; the destination was checked absent first):

```sh
cd /Users/ryan/dev/chirality-t3/swbpipe-rt-perf-c1
RT_C1_KIT=/Users/ryan/dev/chirality-t3/scratch/rt-perf-c1-slice7
RT_C1_TARGET=/Users/ryan/dev/chirality-t3/targets/rt-perf-c1
env CARGO_TARGET_DIR="$RT_C1_TARGET" /Users/ryan/dev/chirality-t3/tools/t3_slot.sh node apps/desktop/native-smoke/perf.mjs --build --app-dir "$RT_C1_KIT/app-route-header" --target-dir "$RT_C1_TARGET"
node apps/desktop/native-smoke/perf.mjs --plan --plan-file apps/desktop/native-smoke/perf-plan-c1.json --subset c1-500
```

The recipe rebuilt the frontend, then Cargo release with
`--locked --offline --features native-smoke,tauri/custom-protocol`; the binding
contains the exact Cargo command, Tauri window configuration and reported
executable path. Reusing the lane's target directory did not overwrite any
preserved private candidate. Build warnings are retained in
`scratch/rt-perf-c1-slice7/route-header-release-build.log`; no build error occurred.

Offline connecting checks after the build: **29 tests** in
`nativeResult.connect.test.ts` and `exactSessionSolve.session.test.tsx`, and
**4 native route-header tests**, pass through normal slots. These use existing
small maintained replay/corpus fixtures, not the large native workload. They
exercise the exact operation/requested-job response, distinct decoder/cache,
compulsory reader registration and model-bound actual verdict, disagreement,
forged numeric evidence, unsupported metadata and disposal/refusal. Prior
451-reply Rust replay / 419-old-outcome invariance and full-suite evidence were
not repeated. No launch test of this particular binary is claimed.

**One proposed run, pending a new owner decision and resource clearance:**

```sh
cd /Users/ryan/dev/chirality-t3/swbpipe-rt-perf-c1
RT_C1_KIT=/Users/ryan/dev/chirality-t3/scratch/rt-perf-c1-slice7
/Users/ryan/dev/chirality-t3/tools/t3_exclusive.sh env -u SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY node apps/desktop/native-smoke/perf.mjs --run --app-dir "$RT_C1_KIT/app-route-header" --models "$RT_C1_KIT/models" --plan-file apps/desktop/native-smoke/perf-plan-c1.json --subset c1-500 --budget-s 840 --out "$RT_C1_KIT/evidence/n0500-route-header1"
```

`evidence/n0500-route-header1` does not exist at preparation. Before launch,
verify clean submitting source, unchanged runtime relative to selected c21ec1048,
all table/model/binding/interpreter hashes and still-absent output. The maintained
runner rechecks the binary/private copy and model; static preparation is no host
admission. Normal queue/drain/memguard and all four exclusive locks govern
acquisition. Announce actual acquisition and closure/held exception promptly.
No nested slot/build inside the interval. Maximum **900 s including teardown**,
supervisor **840 s**, child **810 s** with unchanged **30 s** post-run reserve,
single-run timeout **720 s**, first paint **300,000 ms**, fixed **256-boundary**
scalar/fsynced journal and all existing interaction timeouts remain unchanged.
Pre-App refusal, timeout, journal overflow/invalid/write failure or any other
failure consumes the attempt. No automatic retry, B1000 or 2,000-node attempt.

Expected distinguishable progress, not promised completion: web `header` now
requests `{jobId,operation:"route_header"}`; its native `validate` phase means
metadata route/binding guards only, still after full prescan/admission/parse.
It emits no `inputs_admit`/`inputs_parse` phases because that operation accepts
no extra inputs. A returned header must serialize/drop and exit before web
`registration` enters the full route-specific readers, followed by model-bound
`standing`, `summary`/health and actual full-verdict agreement (which may reuse
the exact-input full-verdict cache). Only then can a result reach the unchanged
run-preview/session/report/store gates. The journal's native call IDs and pending
web phase distinguish this progress without logging source/model/result data.
An exit `ok` only means the call returned; successful metadata grants no standing.
The same bottleneck may move to compulsory registration/standing or a later
reader; no speed, memory-fit or full-gate completion is promised.

If first paint and the later gates complete, retain the unchanged planned paged
UI/navigation/group-CSV/Record/Save/native reopen/re-save/History/retirement
sequence and existing scalar P3 probes. The resource/cancellation/owned-cleanup
criteria below apply in full: fresh normal host/pool admission, unchanged 12 GiB
limits, fresh-unknown stopping rules, process attribution, teardown and verified
process/lock closure. Preserve partial journals/samples/records on failure.
Direct reader/parse allocation counts, health's inferred working upper bound,
per-PID physical/lifetime readings and sequential aligned sums/unknowns remain
separate; no component result or successful header proves usable size. Only
completion of the actual interaction and persistence sequence can establish
those paths at this size/host. No screenshot/capture, embedded LLM, provider,
auth/keychain/network, signing or release is needed or authorized.

The executable prerequisite is now present. The only next execution authority
would be **one** exact 500-node interval plus current shared-resource clearance;
this offline return asks for neither and launches nothing. #59 remains draft;
shared core/bridge/clone/pricing writes stay held. ScopeOfWork and dependency
conditions are unchanged; this reconciles readiness and artifact identity only.

## Route-header run result and closure (2026-11-02)

The owner approved exactly one run of the separate route-header executable
("I approve the reviewed single 500-node route-header run"); Agent0 delivered
fresh App resource clearance. A pre-launch check at **23:46:59Z** passed: clean
`ee7a7f13b`, runtime equivalence to `c21ec1048`, binary `5212d86b…0e53`, binding,
frontend, engines, scripts, plan, model raw/JCS hashes, Python, all retained
candidates and the 25 closure-hashed packets, fresh output, no prior attempt in
the slot log, no App process, no pending exclusive, memguard running. The
launching shell resolved `python3` to a newer interpreter first on PATH; the
check refused that, and the run was launched with that directory removed from
PATH so `python3` resolved to the bound `/usr/bin/python3` (Xcode 3.9.6,
`b971331a…`). No artifact was substituted.

Actual acquisition **23:47:12 UTC**, `END-EXCLUSIVE rc=1` **23:52:16 UTC**: **304 s**
from one-second wrapper timestamps; supervisor **302,645 ms**. No overrun, held
exception or second queue. After the wrapper ended, no App, worker, sampler or
runner process remained, all four lock slots had no holder and
`exclusive.pending` was absent; nothing unrelated was signalled.

The worker published 918,638,964 B at **72,555 ms** from solve request, lifetime
footprint 3,485,847,128 B, peak grant 6,375,342,080 B. Host readings stayed
known-normal, available 70–71%, pressure 1, no unknown host samples. The
300,000 ms first-paint gate failed (observed wait 300,711 ms). Cancellation was
requested with success_claimed=false.

The fsynced journal has **15 boundary records plus its terminal summary**, no
overflow. Completed instrumented monotonic intervals:

| Call | Boundary | Elapsed |
| --- | --- | ---: |
| 1 (`route_header`) | source_prescan | 1,533.9 ms |
| 1 | source_parse | 2,204.3 ms |
| 1 | validate (metadata only) | 357.6 ms |
| 1 | serialize | 0.02 ms |
| 1 | source_drop | 1,044.8 ms |
| 2 (registration) | args_admit (2,064,276-byte body) | 8.0 ms |
| 2 | source_prescan | 1,544.3 ms |
| 2 | source_parse | 1,933.4 ms |
| 2 | inputs_parse | 9.3 ms |

Web `header` returned ok at 78,281 ms (5.16 s after entry). Web `registration`
entered at 78,281 ms; native call 2 entered its full `validate` at native
monotonic **8,800.5 ms** and never emitted an exit before the App ended. No
standing, summary, first paint, navigation, CSV, persistence or P3 probe ran.
The metadata header removed the first-call stall; compulsory registration's full
route-specific validation is now the observed delay. This journal does not
locate an inner validator, CPU use or allocations, or establish that it never
terminates. A successful header grants no standing.

Partial physical observations, separately labelled: App peak footprint
6,825,106,944 B; aligned sampled App+WebContent maximum 7,254,352,488 B with
worker absence inferred zero after `job_done`, **28 unknown aligned sums**;
sequential PID reads, not atomic maxima. The store passed `quick_check`, with no
saved project or run. Records: `evidence/a500-route-header1-{launch-check,
admission,console,wrapper,summary,closure}.json|log` and the five-file packet
`evidence/n0500-route-header1/`. No usable-size, memory-fit, C1 completion,
base-machine, 1,000/2,000-node or acceptance claim.

## Reader-phase diagnostic result and closure (2026-11-02)

The owner approved exactly one diagnostic attempt on clean
`cf8129cd1e9726c4c4706ec084d35c3ddfe3f9f3`, with reviewed code at `d4a880680`,
bound App/worker source `973461eb0`, executable SHA-256
`eff0f8d6ef68e8db65c4eb43cf0f5d80446477c7f78e69fc42e7272ddc0fab81` and
fresh `evidence/n0500-reader-phases1`. All script/plan/artifact/model/interpreter
hashes passed preflight, source was clean, previous packet hashes matched, and
App scheduling clearance preceded normal exclusive admission. Selected Python:
Xcode 3.9.6, real executable SHA-256
`b971331ab9beb1dac075006955f035a97332eca21187c4de615ae58346713e2f`.

Actual acquisition **04:00:57 UTC**, `END-EXCLUSIVE rc=1` **04:06:00 UTC**:
**303 s** from one-second wrapper timestamps; the supervisor window was
**303,635 ms**. No overrun, held exception or second queue. App/worker/runner
and unique private executable PIDs/groups were gone in supervisor and independent
control checks; all owned lock custody and `exclusive.pending` ended. No unrelated
process was signalled or cleaned. Later shared-slot users are not held custody.

The worker published 918,638,964 B after **72,411 ms** from solve request,
reported lifetime maximum footprint 3,485,830,744 B and peak grant 6,375,342,080 B.
Host readings remained known-normal, available 64–68%, with zero unknown host
samples. The 300,000 ms first-paint gate failed (observed wait 300,684 ms).
Cancellation was requested with success_claimed=false; actual process closure is
separate evidence, not a fabricated cancellation receipt or normal lease retirement.

The fsynced journal contains **14 boundary records plus its terminal summary**,
no overflow, incomplete web `header` and native `validate` entries. Native call 1
has no model or invocation; its header is still an ordinary full dispatch at
this selected source. Completed instrumented monotonic intervals:

| Boundary | Elapsed | Scalar size meaning |
| --- | ---: | --- |
| args_admit | 0.082083 ms | 31-byte argument body |
| source_prescan | 1,545.133208 ms | 918,638,964 UTF-8 source bytes |
| inputs_admit | 0.008209 ms | no extra model/invocation allocation requested |
| source_admit | 0.053708 ms | 7,527,838,816-byte **parse lease bound**, not measured heap |
| source_parse | 2,273.918292 ms | full source materialization returned |
| inputs_parse | 0.011084 ms | unspecified/zero input text; no model/invocation supplied |

`validate` entered at native monotonic **3,819.3515 ms** and never emitted an
exit before the App ended. No serialization/drop entry, header exit, registration,
model-bound standing or summary await was reached. No P3 allocation probe or
navigation/CSV/persistence/reopen/retirement sequence ran. The observation locates
this attempt's delay in the first full `rv::dispatch`/standing validation after
parse/admission; it does **not** locate a particular inner validator, prove CPU
utilization, separate its allocations/clones/internal drops, or establish that
it never terminates. It does not retroactively identify the earlier A's phase.
No five-pass/parse count is presented as observed progress: this journal shows
only the first source parse completed and first full dispatch still pending.

Partial physical observations remain separately labelled: sampled App footprint
maximum 8,512,412,872 B; aligned sampled App+worker+WebContent maximum
8,993,628,560 B, sequential PID reads with worker absence inferred zero after
`job_done`, **31 unknown aligned sums**. These are not atomic concurrent maxima
or reader/parse/P3 allocation measurements; host telemetry itself was known.
The store passed `quick_check`, with no saved project/run. No usable-size,
memory-fit, C1 completion, base-machine, 1,000/2,000-node or acceptance claim.

Raw packet/private copy preserved at
`scratch/rt-perf-c1-slice7/evidence/n0500-reader-phases1/`: journal
`perf-reader-phases-perf-BTgJlc-native-linear-ex-n0500.jsonl`, window
`perf-window-1793592057290.json`, matching run/report/samples and `perf-BTgJlc/`.
Adjacent preflight/admission/console/wrapper/summary/closure records bind the
selection and closure; `a500-reader-phases1-closure.json` records packet hashes.
The two previous packets remain byte-identical. Acquisition, observed phase and
actual failure/closure were reported to Agent0 promptly. All further work below
is ordinary offline proposal work; no further execution grant is requested.

## Approved route_header implementation (review CLEAR; c21ec1048 CI green)

Agent0 approved the exact distinct contract below for C1 native/frontend writes
only. The implementation is in `native_verdict.rs`, `nativeResult.ts` and
`previewService.ts`, with one non-allocating `Args::keys` accessor so new mode
can reject unknown/mixed fields **including null** without changing old decoding.
No core, operation-applier bridge, handler macro, worker, allocator, store,
pricing source or permission policy was edited. No new ordinary command is added.
The selected measurement executables/packets are preserved; none is silently
relabelled as a build of this product change. No native/919 MB replay/profile or
new execution grant is involved.

Exact old/new consumer contract:

- Old first await: `{jobId}` -> full `ReaderDispatch` including standing; only
  its `route` was consumed at that point. The command/API remains unchanged for
  every operation-absent caller, including all other full-dispatch consumers.
- New first await: `{jobId, operation:"route_header"}` -> the exact distinct
  `NativeRouteHeader` document specified below, with no `ok`/standing fields.
  The native path calls unchanged public route/binding/error helpers only,
  after full-source admission/parse, and never falls back to full dispatch.
- The frontend validates the exact document shape, job, status/route/refusal
  combination and known route-to-identity vocabulary, with a separate cache.
  A SHA-256-formatted header digest is metadata, not a numerical proof: **before
  result construction or the reader-end mark**, route/status/binding digest/error
  must agree with the compulsory actual full verdict. Wrong digest or foreign
  late response refuses, with no cache alias/fallback/promotion.
- Unchanged captured-input registration, model-bound full standing, summary and
  health, model/producer/retained checks, actual final full verdict/run preview,
  generation/custody/stale checks and session/report/store gates remain in place.
  Unsupported metadata follows the prior full-reading/inspection path, not a
  newly invented early route-header error. Existing decoders structurally reject
  the new reply; its type cannot supply the missing result/standing members.

Native regression checks cover the file-backed corpus's header-field parity,
absence of authority fields, strict mixed/null/unknown arguments, first full-reader
path not entered by metadata, disposed handles, forged numerical rows and exact
unsupported/downgrade metadata refusals. The operation-absent command's text and
all **419 pre-existing recorded Rust outcomes** remain unchanged. New route
requests come from actual small session/replay calls and are answered by the real
Rust command. Only the per-launch handle is normalized to `$JOB` after Rust
asserts the reply names its requested handle, then substituted for the JS replay
job; routing/refusal/binding fields are never fabricated by a permissive fake.

Frontend/session checks reject wrong/extra/missing fields, wrong handle, route/id
or digest disagreement, old/new decoder mixing, cache alias and disposed/foreign
late replies. A successful metadata reply followed by a registration resource
refusal still refuses without summary/fallback. Small sessions with forged numeric
evidence or unsupported metadata reach the expected product refusal and leave
result, Current, record, manifest and solve proof empty; their held jobs release.
The tests explicitly reject replay-coverage errors as a substitute for those
expected refusals. Original positive Current/persistence tests remain unchanged.

The metadata request still parses the complete source. Removing its redundant
full-standing computation may move the observed cost into registration or standing;
it is **not** evidence this patch alone meets 300 s or 500-node usability. The
reader-phase diagnostic keeps its fixed phase IDs and limits; `header` now selects
the distinct metadata operation, and later compulsory calls remain observable.
All numerical arithmetic, validation functions/order within each full reader,
source bytes, input, timeout, memory budget and reserve are unchanged.

Author verification at the implementation candidate: full desktop Vitest
**381 files / 7,429 passed / 1 todo**, after the actual Rust replay additions;
focused connection/session/resource/source-integration **63 passed**; native verdict
suite with smoke feature **19 passed / 1 ignored**; actual-request Rust replay
**451 replies**, retaining all 419 old outcomes unchanged; reach scan **35 passed**;
TypeScript, desktop build and model-version checks (8 passed, no Cargo build)
pass. Initial full Vitest failed on missing new
replay requests; those were recorded from actual calls, generated through Rust,
and the complete suite then passed without weaker assertions or fallback.
The new negative session cases explicitly demand the product refusal instead
of accepting replay-coverage errors. No full local Rust suite is claimed here;
exact-head hosted checks passed at c21ec1048, including `harness`.

The old selected App/worker binary and all failed packet hashes are unchanged.
The ordinary frontend build output is new source output, not a re-binding of
`app-reader-phases` or permission to launch it with the new client. Any future
native candidate must be built and explicitly bound from the selected source
before a separately authorized run. The separate offline binding above now
fulfils that executable preparation; no new measurement is authorized.

Independent numerical/permissions review on owner-selected **gpt-6-astra /
medium** cleared the exact c21ec1048 implementation/contract/consumer graph.
It independently verified all 419 prior requests/outcomes unchanged and all
451 source digests valid; suites above are inherited author evidence, not
reviewer reruns. Shared clone-projection implementation remains held below.

## Distinct route/binding-only header proposal — approved interface basis

**Motivation and scope.** The diagnostic observed the initial header dispatch
blocked in its full validation. The consumer at `readNativeFreshResult` uses only
that first reply's `route`, while later registration and model-bound standing
perform compulsory checks. The approved change removes that redundant *initial* full standing computation
using a distinct header contract, not by weakening or changing `rv::dispatch`.
It makes no claim about the next validator's duration or full 500-node usability;
the actual implementation and verification are recorded immediately above.

**Exact native interface.** Reuse the existing raw-body handler
`read_preview_mechanics_job_verdict` with a separately discriminated request
`{jobId, operation:"route_header"}`. Absence of `operation` keeps the current
full-dispatch/reader-validation contract and reply byte-for-byte. Unknown
operations or a non-string operation refuse typed INVALID_REQUEST. The new mode
accepts no `reader`, `modelJson` or `invocationJson` members, including null forms:
reject mixed requests before held-source parse. It uses the same held-result
lookup, ingress/prescan/adaptive parse admission, executor offload, custody
sharing and release behavior. No handler, capability, route/export permission,
new input source, budget, cache, worker or schema migration is added.

Successful native transport returns one parsed JSON body with **exactly**:

```json
{
  "document_kind": "swbpipe.desktop.result_route_header",
  "version": 1,
  "job_id": "<requested held handle>",
  "header_status": "recognized",
  "route": "pressure",
  "binding": {"id": "<pinned semantic contract id>", "sha256": "<pinned table hash>"},
  "refusal": null
}
```

Metadata refusal is the same shape with header_status `refused`, route
`unsupported`, binding null, and refusal `rv::error_json(code)` from `rv::route`.
These are fixed route/table facts and the existing header refusal, not a checksum
of numerical correctness or native provenance. `job_id` correlates the held
source only, not a solver/capture/Current assertion. Do not include `ok`,
`standing`, `eligible`, `token`, findings, attestation, run/model identity,
result rows/text or any evidence payload. The absence of `ok` is deliberate:
current full-dispatch and validation decoders reject the header structurally;
it must not be accidentally accepted as `{ok:true}` validation.

Compute facts by unchanged `rv::route(&source)` and
`rv::binding_of(route,&source)`; use the existing exported `rv::error_json`, not
copied tables or a substitute error parser. Do not call `rv::dispatch`,
`standing`, `for_source` or any full numerical validator for this metadata mode.
Preserve all of `rv::route`'s downgrade-member and **raw-row W1-token scan**,
carrier namespace, source metadata/identity, evidence-object and receipt-shape
checks in their existing order. It still parses/materializes the full held text
and may inspect rows/retained transport metadata: this is route-only authority,
**not** a promise of header-byte-only or constant-time processing.

**Webview type and consumer.** Add a strict `NativeRouteHeader` decoder with the
new document_kind/version, exact member set, valid route/status/refusal/binding
combinations and response job_id equal to the requested handle. Freeze it; keep
it a distinct type (optional private brand), with no standing/SessionResult
conversion. `readNativeRouteHeader(jobId)` caches only under its own
`route_header` key; never alias a dispatch or validation cache entry. The only
initial consumer is the first `header` await in `readNativeFreshResult`; it uses
route selection only. It must not feed this reply to `presentedStanding`,
`readerDispatch`, native registration/persistence, exact report binding or a
Current/inspection display. A type cast is not a security boundary: the runtime
adapters and session gates must reject inappropriate promotion, and tests must
exercise them.

**Callgraph and compulsory checks before publication/use.**

```text
completed native solve / existing capture
  -> route_header (metadata only; never a result or standing)
  -> validateNativeReaders (same route-selected reader and captured inputs)
  -> readNativeStanding (existing full dispatch, current model/capture binding)
  -> readNativeSummary (existing numerical health + S)
  -> nativeEnvelopeRefusal and existing model/producer/retained exclusions
  -> existing unregistered/inspection handling, final full verdict, run-preview
  -> existing generation/custody/capture/record gates before session publication
```

The fresh source still needs its pressure/preview/pressure-source reader against
the captured model/invocation, model data/capture identity and stale/disposal
checks. Standing/health and model/producer envelope consistency are unchanged.
The final `NativeCustody.verdict` remains the actual full dispatch, not this
header. Report/persistence keep their own custody/digest/record-binding checks.
A refused header supplies no registration/Current. Choose the compatibility
branch explicitly: use route `unsupported` and continue the existing full
standing/S and envelope-refusal flow, exactly as the old first dispatch's
unsupported route did; do not add an early route-header throw or present its
metadata refusal as a new final UI outcome. The existing unsupported standing,
unregistered/inspection and report/persistence gates remain responsible for
refusal and source identity. Preserve final findings/refusal order on the corpus;
if that cannot be shown, stop the patch for a design decision rather than change
expectations. Metadata recognition never becomes numerical eligibility. Full
dispatch semantics for historical, Wasm, Python, comparison and all other
consumers remain untouched.

**Acceptance checks for the proposed interface.** Compare only route, binding
and metadata refusal with unchanged `rv::route`/dispatch header fields across
the maintained corpus, including downgraded W1 rows, foreign identities, malformed
receipt and retained sources. A forged numerical row may still have a recognized
metadata header, but must fail the unchanged registration/standing checks before
any eligible use. Existing full-dispatch replies and first-refusal outcomes stay
byte-identical; old commands need no regenerated expectations merely to pass.
Test old decoders rejecting the new shape, header decoder rejecting old replies,
mixed requests/unknown keys, wrong handle, wrong table binding, unsupported
route, disposal and stale generation. A small connection replay must show the
first header makes no full-dispatch call but later validation/standing still do.
No 919 MB replay, solver, App or allocation/timing run is needed to establish the
interface semantics; no performance/fit promise follows from those tests.

**Ownership/order.** C1 can own native_verdict/nativeResult/previewService,
strict decoders and test integration after this exact interface/consumer proposal
is reviewed and Agent0 selects implementation. Existing public `result_export`
APIs suffice; core and operation-applier require **no writes** for this proposal.
Any unexpected shared API mismatch returns to Agent0; there is no authority to
change core reader, numerical tables/pricing, handler macro or permission policy.
New independent reviewers use owner-selected gpt-6-astra/medium, resolved through
the current capabilities; no new reviewer or implementation was dispatched for
this proposal return.

## Separate minimal clone-projection proposal — shared-core write hold

`exact_combination_evidence::direct_view` currently checks the combination record
is present and nonempty, clones the complete source, removes exact_combinations,
then retains primitive rows. Proposed replacement keeps those **same checks and
first refusal order**, then clones only the members that the existing final view
retains. For `contract_evidence` objects omit only exact_combinations; for results
arrays clone only rows for which the existing `is_combination_row` is false;
clone every other value unchanged, including diagnostics, unknown members,
strings, numbers and member/row order. Wrong-typed members behave exactly as the
current guarded remove/retain operations. No source/held bytes are rewritten.

Required proof: old and new projected `Value` trees are equal and serialize to
identical serde/checked-canonical bytes on valid/forged corpus and small edge
shapes (unknown fields, escaped/Unicode keys, negative zero and wide numbers).
Then run the *same* primitive and combination validation functions in the same
order; every accepted/refused code/detail/first failure and numerical/derived
row byte stays identical. No arithmetic, rederivation, standing, receipt, hash,
schema or output change. This proves avoided cloning of discarded material, not
a quantified heap reduction, CPU dominance or sufficient cure for this timeout.

RT-A3's coordinated source basis at clean reducer2a `d4441e1` is evidence of
proof dependencies, not current-main adoption or a released write turn:
`exact_combination_evidence.rs` was unchanged there (SHA-256
`ecbc565fb8566538568c6c946a93bb534e79e0bb9c215e646c7c1a769f2677f6`), as was
`reader_verdict.rs` (`f3d933138f471f38bbd0e8db109e03066cbd13242292bc25d46706d55c5b9de3`).
W16B-RD in `o10_rules.py` around 3378–3435 guards present/dispatch predicates and
proves the projector/validator unreached on D1; a projection repair needs its own
dataflow/ownership and ordinary byte/first-refusal equivalence. OP1A and
TIM-PF/RD/CD-ID full-file pins can invalidate on a core edit even if public helpers
are unchanged. The 2a semantic-contract edits change PRESSURE_TABLE_SHA256 and
PHYSICS_RETAINED_SHA256; b7a_domain_model and the retained EXACT_TABLE_HASH/proofs
bind those sources. #8/#11/carriers and profile `8cd8fcf3` are consumers. No
operation-applier edit or planned 2b projector algorithm change was reported.
Recheck selected current-main sources and combined registration state when a
repair turn is actually authorized. Nonoverlapping edits do not prove coefficient
neutrality, and this notice triggers no repricing/pin transplant/core write.

This is a separate, narrow shared `core/reporting/result_export` patch requiring
Agent0 sequencing with RT-A3 and D3/retained reader consumers, plus independent
numerical/memory scrutiny. Any registration/pricing effect must be rederived on
the combined tree; no coefficient/tolerance/control is silently adjusted. No
borrowed-view redesign, indexing or broad cache is assumed. **No shared-core
write or clone repair implementation is authorized in this return.**

## Bounded reader subphase candidate (offline; no execution grant)

Agent0 authorized only the diagnostic delta needed by the missing phase claim.
It adds no optimization, route-only reader contract, parse cache or core source
change. The historical A500 facts below remain immutable, and the counts of
possible pressure passes/parses remain source-path counts, never observed A
behavior. No third App/solver/profile/large-input attempt is authorized.

The existing four session awaits emit fixed `header`, `registration`, `standing`
and `summary` entry/exit records only when the native-smoke driver installs its
narrow reporter. The timeline module's helper accepts no arbitrary phase, input
or message. It awaits the existing `native_smoke_log` delivery before entering
the product await; errors record `error`, while pending work has no invented
successful exit. Boundary `ok` means the function returned, never numerical
eligibility or a Current grant; validation/refusal payloads are unchanged and
not logged. A launch permits at most 32 observed awaits. Ordinary sessions
without the smoke reporter perform the same calls without reporting.

The native command boundaries use launch-local call numbers and monotonic times:
`args_admit`, `source_prescan`, `inputs_admit`, `source_admit`, `source_parse`,
`inputs_parse`, `validate`, `serialize`, `source_drop`, `summary_health` and
`summary_build`. Sizes are scalar raw/parse-lease bytes, not measured heap.
No job id, model/result/record contents, diagnostic messages or unbounded
standing are emitted. RAII emits an `incomplete` exit on unwind; a killed
process leaves a missing exit. Process-local clocks have separate origins;
runner receipt epoch times align delivery only, not atomic cross-process time.

For diagnostic-enabled smoke builds, native verdict expands only the existing
operation-applier text wrapper locally: same source/model/invocation parse order,
null handling, exact parse-error text, unchanged public Value dispatch/validation
calls and same serde serialization. This exposes parse/validate/drop boundaries
without writing `operation_applier` or shared `result_export`. Ordinary product
builds and smoke launches without the flag still use the original wrapper.
The diagnostic branch preserves replies/errors against the maintained file-backed
reader corpus and malformed/tiny inputs. It adds logging/branch overhead and
slightly explicit Value drop sequencing; it is a newly selected runtime, not
the old bound `app-p3` executable or a claim of identical performance.

Summary's own source parse, input parse, build/response and drop are separated.
The private core `numerical_health_json` entry internally owns its parse and
work; **`summary_health` deliberately combines that parse/validate/serialize/drop**.
Splitting inside it is not possible under this assignment's core no-write
boundary. If that combined phase is pending, further core observation would
require a separately reviewed interface; no finer attribution is claimed.

The runner validates fixed IDs and bounded numeric fields, and synchronously
writes/fsyncs each received event to an exclusive-created JSONL file outside
throwaway scratch before retaining/reporting it. It accepts at most **256
boundary records**, one overflow notice and one terminal summary; the tested
maximum is below 100 KiB. Native emission is separately capped at 256 events;
web emission at 64 boundaries. Unknown fields/IDs, mismatched boundaries, write
failure or overflow **stop the entire diagnostic window** through the existing
supervisor, not just its evidence classification. Such a stop can leave no final
run or terminal journal record; earlier fsynced entries remain partial evidence.
The 256-record cap is shared by web and native events. Independent source review
estimates roughly 200 events on a successful full sequence and roughly 240 with
the re-read branch; those are unmeasured estimates, not evidence that the live
sequence fits the cap. The cap and stop behavior are unchanged by this note.
Pending
entries survive a blocked await, native task failure or App exit without needing
a final reply; a killed runner may lack its terminal summary, and the already
persisted entries still show the last received boundary. Pipe delivery before
persistence is not a power-loss/instantaneous-process-kill guarantee.

After App exit the runner drains stderr for at most one second; uncertainty
marks evidence incomplete. Completion requires all four web awaits and all fixed
native boundaries, not merely an empty pending set. Phase scopes/overflows and
missing exits are reported with the existing run/partial/report path. No new
ordinary handler, file chooser, export, native window or general tracer is added.
The journal, fsyncs, IPC acknowledgements and additional drop scopes add overhead;
a subsequent authorized run would diagnose phase location, not silently replace
the selected uninstrumented throughput/memory basis.

Offline checks use only maintained small fixtures, replay, injected closures and
tiny fake Node processes. They stall each of the four real session awaits and
inspect native phase entries before injected work, error and unwind; preserve
pending cancellation/disposal exits, reject contents/overflow/empty evidence,
and recover an entered phase after killing only their fake child. No native App,
solver, sampler mode, 919 MB parse, allocation profile or exclusive interval ran.
The initial fake-child test had an escaping error and was stopped under its own
ownership; it now has early-exit detection and a timeout. A TypeScript import
was initially put before the existing Vite triple-reference and was corrected;
no ambient declarations were weakened.

Current offline evidence: **45 Node tests** pass, including journal persistence,
interruption, all-fixed-phase coverage and bounded retention; **29 TypeScript
connection/timeline/global tests** pass, including actual small replay disposal
while a header is pending; the native diagnostic wrapper matches the maintained
file-backed corpus and malformed inputs, and the native boundary error/unwind
check passes. Default and smoke library compilation pass. The reach scan's
initial combined test attribute was replaced by its existing separate test-only
form; all 35 reach tests then pass. The scanner and its rules are unchanged. TypeScript's Vite reference order
was repaired rather than adding ambient exceptions. These are offline functional
checks, not parse/allocation/timing or native usability measurements.

The new diagnostic App/worker was built **offline, without launch**, from clean
`973461eb08e4676011d87108d0763c28008c9786` by the maintained `perf.mjs --build` through the
normal slot wrapper. Binding: `/Users/ryan/dev/chirality-t3/scratch/rt-perf-c1-slice7/app-reader-phases/perf-binding.json`.
The executable is 51,102,912 B, SHA-256
`eff0f8d6ef68e8db65c4eb43cf0f5d80446477c7f78e69fc42e7272ddc0fab81`. Frontend/dist is
`sha256:dd3b5127ba256f9ddda8da79f08efae12dd0234c0f3f7673e625546f69c4bf5a`; the existing operation/self-weight Wasm source stamps
remain `sha256:14ab23c574227e35d6cc5c94e5f7a1dde7d8343d8e4eebfdbbd3c35c624cd54a` and
`sha256:04c15ac0dded329c0e873985a34b3f73c68fdeab27331cbe02e7925a9539c938`. The worker is this same new executable
with the unchanged solve-worker flag/protocol, not the selected old worker.
The later external journal write fix loops over short writes before fsync; it
changes no native/frontend/Wasm bytes and is separately identified by the current
harness submitting revision. Its four focused journal/interruption tests pass.

Current diagnostic external source hashes (the older preparation tables below
are historical):

| File | SHA-256 |
| --- | --- |
| `perf.mjs` | `7329cca1c7c4de7a8758f3457c54dc00c8f5de9dbbe61db65e13f331844be8a5` |
| `perf-driver.js` | `e53468ab84d333a83df2305c76e26fc9d33d97f52215e4086fc27b69c14dfff1` |
| `perf-sampler.py` | `846f5493cc9cc309140cced0b6ff40ec9b92468db10a4f63e85fe654f9e03d3b` |
| `perf-plan-c1.json` | `25e8a62b828c6251ac36836d11f8b5448fc3138dbd4903e99ef453a732966435` |

Focused measurement/permissions review is **CLEAR at
`d4a880680414a6e1fce47b3f9fd5cb8a2600d3c3`**, no blockers. The reviewer
independently checked wrapper parity/lifetimes, persistence and completeness,
identities and the absent proposed output, ran six phase-focused Node tests,
and obtained a zero-error declaration check. Other test/build results above are
inherited author evidence. No live event count, IPC/fsync overhead or pipe-drain
behavior has been measured. Exact-head hosted CI remains required; review grants
no window.

Additional nonblocking limits retained for the next owner decision:

- Native records identify a launch-local call and phase, not the command/reader.
  Correlate with pending web phases or the phase set; do not invent exact matching.
  Verdict `inputs_parse` has `bytes:0` as **unspecified**, while summary gives its
  input length. Zero is not a measured parse size or zero allocation claim.
- IPC/copy, executor queueing, argument decoding/held lookup, some admission-bound
  calculation, summary health response parsing/health-view construction, reply
  transfer/decode and JS work between awaits are uninstrumented gaps. A gap is
  not validator CPU time. `summary_health` remains explicitly combined.
- A web reporter delivery failure prevents an enter's product call, or replaces
  an exit's success/error with the logging failure. It leaves missing evidence;
  this is a diagnostic-induced stop/failure, not a product numerical refusal.
  Ordinary sessions also incur additional async microtask ticks per await.
- Opening the journal precedes startup admission. A pre-App failure can leave an
  empty file/no terminal record; its descriptor closes with the stopped runner.
  A write failure can leave a partial line. Treat it as incomplete, never repair
  or rewrite raw evidence into a successful record.
- Rust stderr line emission can involve multiple writes; interleaving could make
  an event invalid and stop the window. No such interleaving was observed in the
  earlier packet. The fixed drain and interpreter timeouts remain unchanged.

The selected runtime is separately built/bound in **`app-reader-phases/`**,
preserving `app-p3/` and both failed packets. Plan/input/budget remain the maintained500
case, 300 s first-paint wait and 840/810 s supervisor/child limits, with only
`c1_reader_diagnostics:true` added. The exact recipe for a future owner decision
uses fresh **`evidence/n0500-reader-phases1`**, not either consumed path:

```sh
/Users/ryan/dev/chirality-t3/tools/t3_exclusive.sh env -u SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY node apps/desktop/native-smoke/perf.mjs --run --app-dir "$RT_C1_KIT/app-reader-phases" --models "$RT_C1_KIT/models" --plan-file apps/desktop/native-smoke/perf-plan-c1.json --subset c1-500 --budget-s 840 --out "$RT_C1_KIT/evidence/n0500-reader-phases1"
```

This is not permission to run it. Before returning readiness: bind the clean
runtime/frontend/worker/scripts/plan, reconcile these identities, pass focused
measurement/permission scrutiny and exact-head CI. Before any actual run: a new
explicit owner/resource decision, exact hash/fresh-output checks and normal
shared admission are required. No retry, B1000, 2,000-node, screenshot/capture,
LLM/provider/auth/network or release authority follows. Core readers/semantics,
validation/refusal/source identities, budgets/reserves and existing failed raw
samples/lifetime maxima remain unchanged.

## Repaired A500 result and closure (2026-11-02)

The owner explicitly approved one repaired attempt on clean `eb77d0067`, using
the unchanged bound `app-p3` executable and maintained 500-node input, with fresh
`evidence/n0500-refresh1`. Exact source, script/plan/model/Wasm/dist/binary hashes
were reverified. The original failed packet's hashes are unchanged. Python was
recorded as `/Applications/Xcode.app/Contents/Developer/usr/bin/python3`, 3.9.6;
its reported monotonic implementation is `mach_absolute_time()`.

The normal wrapper recorded WAIT at **01:58:28 UTC**, acquisition at
**01:58:29 UTC**, and `END-EXCLUSIVE rc=1` at **02:03:32 UTC**. The elapsed value
from those one-second timestamps is **303 s**; the supervisor window was
**303,399 ms**. This is below the 900 s interval cap, with no extension or build
inside it. The separate startup host attempt was fresh (sent 01:58:29.210,
sampled 01:58:29.246, received 01:58:29.249 UTC), available 74%, pressure normal.
Every streaming host sample remained known-normal, with available level 69–74%.
Thus the initial-refresh correction worked in this attempt; the gate still failed.

The App shell became ready, opened the selected model, started sparse solve and
received the worker's published result. The solve timeline records `job_done`
**73,378 ms** after `solve_requested`, **918,638,964 result bytes**, worker
lifetime-maximum footprint **3,485,617,728 B** and peak grant **6,375,342,080 B**.
Native `reader_verdict_start` was the next mark. No `reader_verdict_end` or first
paint arrived before the configured **300,000 ms** first-paint timeout (observed
wait 300,651 ms). The classifier is `hang` for that deadline; this does not prove
the reader could never finish. No threshold, input, result or standing was changed.

The table/navigation/filter/CSV, Record/Save, native reopen/re-save, History and
normal result-retirement sequence was **not reached**. The scalar allocation
probe was also not reached: there is no direct reader-working or parse-allocation
peak and no health-working upper bound for this run. The UI cancellation attempt
reported requested=true and success_claimed=false. That is not a completed
cancellation/reap receipt. The App then exited with code 0 via the harness's
failure report, and the runner exited 1; actual process closure was checked
separately. The last native pool report still held one result, no worker, and
8,446,497,008 reserved bytes. Normal lease retirement was not demonstrated before
App exit, and must not be inferred from later process disappearance.

Partial physical observations on this Apple M5 Max/18-CPU, 128 GiB host,
Darwin 25.6.0:

| Observation | Bytes | Interpretation |
| --- | ---: | --- |
| App sampled footprint maximum | 8,377,883,872 | Direct per-PID sampled value; not reader-only or parse-only |
| App kernel lifetime maximum | 8,617,729,272 | Direct kernel high-water observation; includes all App phases |
| Worker kernel lifetime maximum | 3,485,617,728 | Also reported by the completed worker |
| WebContent sampled footprint maximum | 824,968,320 | One new baseline-attributed WebContent; parentage remains inferred |
| Aligned sampled App + worker + WebContent maximum | 8,852,152,792 | Sequential PID readings at one sample, not an atomic concurrent peak; worker absence is inferred zero after `job_done` |

The aligned sum has **21 unknown samples** (including incomplete process
attribution/readings); this count is not unknown host telemetry. GPU/networking
are retained separately in the raw packet, not added as independently timed
maxima. These observations do not establish the full gate, memory fit, a material
reader share, 500-node usable size, 1,000/2,000-node capability, or the 16 GiB base
machine criterion. SQLite `quick_check` returned `ok`, with zero project, run and
state rows; no persistence step was attempted.

Closure: the supervisor reaped runner group 22336 and verified all private-exe
PIDs gone, with no group errors. Independent control-only lookup returned `[]`;
`ps` found none of the owned wrapper/runner/App/worker PIDs or group, and no
A-owned lock/`exclusive.pending` remained. A later shared slot's unrelated
`lockf` holders are explicitly distinguished from this closed interval; they
were not signalled or changed. **No held exception.** Acquisition and actual
closure/failure were reported to Agent0 promptly. No B or second run was queued.

Preserve the new packet/private store/App copy for the owner and failure diagnosis
consumer, alongside the unchanged first packet:

- `scratch/rt-perf-c1-slice7/evidence/n0500-refresh1/perf-run-perf-vvmNnv-native-linear-ex-n0500.json`
- `scratch/rt-perf-c1-slice7/evidence/n0500-refresh1/perf-window-1793584709038.json`
- The same directory's raw sampler JSONL, aggregate report and `perf-vvmNnv/`.
- Adjacent `a500-refresh1-preflight.json`, `a500-refresh1-admission.json`, console,
  wrapper, summary and closure files. The closure records raw packet hashes and
  verification time; it does not rewrite raw evidence.

No new implementation, measurement or third-run proposal is selected by this
return. The observed outstanding issue is native reader validation failing to
reach first paint within the approved gate; source diagnosis is distinct from
another solve/profile or a timeout extension. #59 stays draft and unmerged.

## Offline diagnosis of the repaired first-paint timeout

**Supported conclusion:** the packet locates the timeout inside the awaited
native-registration sequence, but cannot identify its active subcall or split
CPU, parse, clone and drop costs. Its `trace.counts` is empty. No native result
text, native phase checkpoints, stacks or P3 metrics survived in this packet.
The last pool reservation total and physical samples cannot supply that missing
attribution. They remain the observed values above, not parse/reader metrics.

The exact path at selected `eb77d0067` is:

1. `previewService.ts:642` marks `reader_verdict_start`.
2. `readNativeVerdict(job,{})` sends the native verdict command. Despite its
   local variable name `header`, this calls `reader_dispatch_json`, not a
   route-only reader. The native path admits args, prescans held bytes without
   materializing them, acquires the parse lease, then deserializes the entire
   result via `serde_json::from_str<Value>`.
3. `validateNativeReaders` sends a separate route-specific validation request
   with the captured model or invocation. It performs another held-result
   prescan, parse and validation.
4. `readNativeStanding` dispatches again with the current binding, with another
   held-result parse. Its exact-input cache differs from the header request.
5. `readNativeSummary` prescans once, calls the health text API (which parses
   the held source), then separately parses it again to build summary/planes.
   The first health source tree is dropped before the summary source is parsed.
6. Only after these awaits and envelope refusal handling does the session mark
   `reader_verdict_end`. An envelope-registration refusal may repeat standing
   and summary without binding. Later handle construction requests standing
   again, but the exact-input cache reuses the successful standing request.

Four bounded regressions in the maintained `nativeResult.connect.test.ts`
freeze header, registration, standing and summary separately on the existing
small pressure replay, using committed Rust responses. Every pending case gives
exactly the same final `reader_verdict_start` mark as A500. The normal completed
sequence issues three verdict requests plus one summary; there is no fourth
verdict command at handle construction. **All 11 connection tests pass.** These
are mocked/replay control-flow checks, not measurements or reproduction of the
919 MB native workload.

For an admitted ordinary pressure result, `reader_verdict::dispatch` calls
`standing`. That computes its token using `numerical_use_standing_with_context`
-> `for_source` -> full pressure evidence validation **before** the empty-basis
return, and then the pressure branch validates evidence again to obtain findings.
Thus the successful pressure sequence above invokes full pressure evidence checks
**2 + 1 + 2 = 5 times**, independently of the five sequential result parses
(three verdicts plus health plus summary). This is a source-path count, not an
assertion that A reached all five. The selected model has exact combinations;
the packet does not carry its returned route, so the pressure-route work count
is stated conditionally rather than presented as observed call progress.

Inside a pressure evidence pass with combinations,
`exact_combination_evidence::direct_view` first does `source.clone()`, then removes
`exact_combinations` and combination rows. This definitely allocates a second
full `Value` tree before discarding those members; it is not just another copy
of the raw UTF-8 buffer. The primitive check then builds borrowed ID/evidence
indexes, and the combination check rederives expected rows/evidence and retains
expected combination rows across records. It scans rows per operand/case;
its support-magnitude reconstruction performs a linear row search for each
support/component. Model binding also has member/section searches. These are
concrete allocation/repeated-work and potential scaling sites, **not a proved
CPU bottleneck for this packet**. Value destruction and allocator-retained
blocks can contribute to later CPU/physical readings as well.

The existing `exact_combination_contract` corpus-admission test passes under the
normal slot wrapper on its five maintained small cases (735–1,828 rows), with
and without models. No expected rows, numeric comparisons or authority changed.
No core source was modified, and no allocation/timing benchmark, native App,
worker solve, large-result parse or new measurement layer was run.

**Smallest missing observation:** a bounded entry/exit checkpoint for the
existing header/registration/standing/summary awaits, and within the pending
native command, prescan/admission, parse, validate and source-drop boundaries.
A checkpoint must be durable before the long phase starts; an end-only reply
cannot identify a timeout. This is a proposal for narrowly extending existing
smoke/timeline evidence, not a request to run the App. Replay tests can validate
its ordering without the large input. No CPU/allocation value should be invented
from the phase marker.

**Smallest supported product repair plan, conditional on the identified phase:**

- For the redundant initial dispatch, expose a route/binding-only reading using
  the existing downgrade/identity/receipt-shape guards. It must have a separate
  return type with no standing/eligibility; registration and model-bound standing
  still run their full checks before the session grants Current. Merely deleting
  validation from the existing dispatch API would change its contract and is
  not proposed.
- For combination projection, clone only the retained primitive members/rows
  rather than cloning the full source and then deleting combination material.
  Pin projected data equality and existing first-refusal/numeric outcomes on the
  maintained corpus before considering a borrowed-view refactor or indexes.
  Do not remove evidence checks, change their order or tune any tolerance.
- A broader single-parse registration operation could remove later full parses,
  but it is not the first patch: it changes lease lifetimes and the connection
  contract. Reusing parsed trees globally without a custody-bound lease/cache
  decision is not justified by this packet.

Ownership: native verdict/summary, session connection and smoke boundaries are
C1 surfaces. The combination projection and shared reader/standing APIs are
`core/reporting/result_export` (DEL-05-02/DEL-04-07 numerical reading) and the
operation-applier bridge; coordinate exact edits through Agent0 with RT-A3 and
other consumers before shared writes. Any product change needs independent
numerical/permission scrutiny and byte/refusal parity as applicable. No new
handler, window, input, memory budget/reserve, timeout enlargement, authenticity
or numerical-eligibility contract is selected by this diagnostic return.

## A500 execution and closure (not performance evidence)

All approved source/script/plan/model/Wasm/dist/binary identities passed the
19:34:13 UTC offline preflight, and the output directory was fresh. The normal
exclusive wrapper recorded acquisition on 2026-11-01 at **19:34:58 UTC** and
`END-EXCLUSIVE rc=1` at **19:34:59 UTC** (one-second timestamp precision).
The supervisor's internal window was 19:34:58.740–19:34:59.287 UTC, **547 ms**;
that is not a subsecond measurement of the full lock-hold duration.

The App process was launched (PID 7456). The supervisor stopped its runner with
`host_headroom_unknown` after two early samples. Both had null available/pressure
values and a null host refresh timestamp. No UI-ready, model-open, solve,
reader-profile or completed-run evidence exists. The receipt is explicitly
`evidence_complete:false`; these execution facts establish no usable size,
component memory peak, host memory shortage or memory fit.

The wrapper exited and the supervisor verified the runner group and private
executable PIDs gone, with no group errors. Control-only lookup, `ps` and lock
inspection at 19:36:24.968 UTC found no owned processes, no lock holders and no
`exclusive.pending`. Later shared-slot reuse by other jobs is not an A-held
resource. There is no held exception. App Agent0 was notified by Agent0, and no B
was queued. The failed packet/private copy is preserved byte-for-byte for the
owner and initial-refresh diagnosis/review consumer:

- `scratch/rt-perf-c1-slice7/evidence/n0500/perf-window-1793561698740.json`
- `scratch/rt-perf-c1-slice7/evidence/n0500/perf-samples-perf-SaUjMu-native-linear-ex-n0500.jsonl`
- `scratch/rt-perf-c1-slice7/evidence/n0500/perf-SaUjMu/`
- Adjacent preflight, wrapper and closure records; `a500-closure.json` records
  the preserved packet hashes and distinguishes A's closure from later lock use.

## Confirmed initial-refresh cause and bounded offline correction

The sampler initialized `host_at=0.0` and queried only when
`monotonic()-host_at >= 1`. A process-relative monotonic clock near zero therefore
emitted the untouched initial host object until one second elapsed. A failed
sysctl attempt would have assigned a timestamp, so the observed null timestamps
identify the unattempted-refresh path. The deterministic old-source regression
starts at **0.015 s** and reproduces a first query at **1.015 s**, after the
stream guard could stop on the initial unknowns. No native clock or sampler was
re-run to obtain that diagnosis.

The minimal initialization repair uses `None` for "no attempt yet" and reads
before the first emitted sample, independent of clock origin. One shared bounded
host-reading function retains actual values or unknowns; failed/stalled reads
never manufacture normal values. The streaming stop policy is unchanged: two
genuinely fresh unknown attempts stop; repeated cached sample ticks do not count
as new attempts. The pure AST tests execute only extracted functions with fake
process/time/sysctl/signal services; no module import, libproc, real sampler mode
or host probe runs. They cover the first valid sample near zero, cached ticks,
two fresh failed readings and bounded stalled readings.

The original entry contract requires fresh known-normal host state before App
launch. The old runner started the App before sampling and did not enforce that
ordering. A separate `--host-status` control in the existing sampler now performs
one fresh attempt (0.5 s sysctl timeout, no PID memory counters). The runner awaits
it immediately before App launch, with a 1.5 s interpreter timeout/SIGKILL and
4 KiB output cap, all under the existing independent supervisor. It admits only
normal pressure, at least 30% available and a timestamp from that request (1 ms
rounding tolerance); failed, stalled, malformed, unknown or stale telemetry
refuses before App launch. There is no entry retry. Streaming still tolerates
one genuine unknown refresh after admission. Native pool preflight, caps,
reserves and the 840/810 s budgets are unchanged.

That newly enforced startup ordering is a safeguard change, so it requires one
focused independent review under the existing honest-local-mistake threat model.
It does not reopen the three closed P1 teardown rounds or change their controls.
The fake-child timeout test and all **41 Node tests** pass offline; no new App,
worker, solve, sampler mode or exclusive interval has run. Exact-head CI and
focused scrutiny remain required before returning this repair candidate.

## Candidate and prepared identities

The runtime basis is `a37d1ef1f535155e8ea3f0e330bb7cd7a056bb71`, built clean.
Its only changes from independently cleared
`55fd8f4fccb334e91fe2929e4f93ba0aa3165b1b` are two Work documents. Runtime,
tests, schemas, core and permissions are identical. This proposal and
`apps/desktop/native-smoke/perf-plan-c1.json` and the 500/1,000-node extensions of
`rackModelOpen.test.ts` are later preparation changes, not part of that binary.
Their submitting revisions and external hashes are recorded below. Any later
runtime, frontend, reader, worker, driver, sampler or plan change must be named
and hashed in the run record; do not silently call it the a37d candidate.

The maintained `perf.mjs --build` completed under `t3_slot.sh`, without launching
its output. Binding: `/Users/ryan/dev/chirality-t3/scratch/rt-perf-c1-slice7/app/perf-binding.json`.

| Component | Prepared identity |
| --- | --- |
| App | Mach-O arm64, 48,353,808 bytes; SHA-256 `94a2cc83f390163c15c6a3e60e5ed4befbe0a2c68612ca2beaaf1115f520ba19` |
| App path | `/Users/ryan/dev/chirality-t3/scratch/rt-perf-c1-slice7/app/openpipestress-desktop` |
| Worker | The exact same executable, `--swbpipe-solve-worker`, launched by `CurrentExeLauncher` with an empty environment; record its PID/argv and the product's reported lifetime footprint/grant |
| Frontend | `dist_digest` = `sha256:37882871bcee062a482c72c236f8a5356115b6d3bae9f0dbd7b8e24868c18422`, embedded by the custom protocol |
| Operation Wasm bytes | SHA-256 `6f584adcbc7b5233c8418075aa23f15fe33d68999042e30261984f34dedb1a3a` |
| Self-weight Wasm bytes | SHA-256 `5adff8809a205605654b1998d2f0bef405aae7462604b880d5f0647a0ab4a5ce` |
| Wasm source stamps | operation `14ab23c574227e35d6cc5c94e5f7a1dde7d8343d8e4eebfdbbd3c35c624cd54a`; self-weight `04c15ac0dded329c0e873985a34b3f73c68fdeab27331cbe02e7925a9539c938` (source stamps, not artifact hashes) |

Build flags are `--release --locked --offline --features native-smoke,tauri/custom-protocol`.
The smoke window is 1440×920 with background throttling disabled. This binary
contains the injection hook and must not be distributed. These runs use native
mode, not injection. Node v24.18.0 and `node:sqlite` are available.

Prepared external harness identities (SHA-256, recheck before execution):

- `perf.mjs`: `9e79ae82f10f7ff84de7ecb732209fca870b6fd691c8225209f66906a1d11714`.
- `perf-driver.js`: `95894a5d6a672ad30f1c5dcff15bcf1149fb7254021837fb32828fe21832807c`.
- `perf-sampler.py`: `99bc61de966068e1e1acf550219a3dd8df4e35e82bc7723fe0c4d69412f69295`.
- `rack/rack_model.py`: `9bbc6b9b1f6f78c11233da9906bf1346d4322a16bfe1b5c5e2069c7914456559`.

Those are the original prepared a37d harness identities. The later P1 preparation
changes `perf.mjs`, `perf-sampler.py` and their tests. P2 also changes the
external driver and plan. Neither changes the bound App/worker/frontend. Bind its submitting revision and re-hash those
external scripts before execution. The old identities do not certify the new
supervision path, and green CI at 867e51b does not cover that later code.

## Later instrumented executable (prepared offline)

The P2 probe source is commit `e989364ec`; P3 is
`284c71f48165fac5ec34cf0cf8ba5eb20b74113f`. The latter was built from a clean tree through the
same maintained `--build` recipe, with no launch. The source delta from the
reviewed runtime is explicit: native-smoke reader instrumentation, inactive
allocator hooks under that feature, external driver/plan/reporting and tests,
and the narrowly parsed TLS scan form. It changes no core, pricing, schema,
ordinary product handler or frontend source. The current frontend/dist and
both Wasm identities equal the original a37d binding above. Later Work-only
binding reconciliation does not change this runtime/test identity.

- Binding: `/Users/ryan/dev/chirality-t3/scratch/rt-perf-c1-slice7/app-p3/perf-binding.json`.
- App/worker: `/Users/ryan/dev/chirality-t3/scratch/rt-perf-c1-slice7/app-p3/openpipestress-desktop`,
  50,985,168 bytes, SHA-256 `288811f283e2e979fa17a452d478f378a5d85d717662fb4751d9dfbb94c48dcf`.
- Worker is the same new executable with `--swbpipe-solve-worker`; inactive
  counter checks add instrumentation overhead even outside P3. Neither App
  binary has been launched.

Current external source identities (SHA-256; recorded in each run and
inspection checkpoint as well as the final report, with current source commit
and dirty flag, separately from the native App binding):

| File | Digest |
| --- | --- |
| `perf.mjs` | `42ee95e79221cd46f7a24deb314fc8ffa9304c322f27c7dcb4e7e8a46ec41a13` |
| `perf-driver.js` | `92e06d6b38c25341d86381fd51fcfebccaa0ba832e39a1f5fc71df7e459dfa84` |
| `perf-sampler.py` | `846f5493cc9cc309140cced0b6ff40ec9b92468db10a4f63e85fe654f9e03d3b` |
| `perf-plan-c1.json` | `663a0affddcdd40323e11f3f4a89e1a10e961ab34e7e1f301d89825f5fbec2cc` |

The original a37d App remains preserved in `app/`; execution recipes below
select `app-p3/`. P1 was independently clear at dee93a72 and its control block
is byte-identical. P2/P3 connected measurement/permissions scrutiny, including the TLS scan
delta, cleared `92983690e`; the external-script follow-ups below change no
App/worker/frontend/Wasm bytes. No successful allocation probe or native
interaction is claimed by compilation or these hashes.

## Inputs and reproducible commands

Generate only the maintained invented `linear-ex` racks. They carry model 0.9.0,
seven primitive cases and five subtraction combinations. Keep the generator's
pending-family ledger: these are its current interim racks, not a claim that all
downstream mechanics, nonlinear EX, or criterion 9 is delivered.

| Model | Nodes / members | File bytes | File SHA-256 | JCS SHA-256 (the solve-proof comparison) |
| --- | --- | --- | --- | --- |
| `linear-ex-n0500.json` | 500 / 499 | 1,828,475 | `42567114a27415b7b042bb9a91d49c8dc640d534953616e5b7efb66b2a2025eb` | `0d45797d8aacada487ef418ce0a776239f6fd9801ed315c1a0f6e1ca2e25466c` |
| `linear-ex-n1000.json` | 1,000 / 999 | 3,649,735 | `fc33acd8f38c2275e5432d4a8b5f1e5aaeb4407532ae83a1c7f484ceb01bb892` | `299aedb4b44b81e9916c4b85650d53d9e96c4c1c728bde8789ed3dddb6f2f312` |

Offline reproduction, from this checkout. Generation, build and plan validation
were executed; the original App build was bound at clean a37d. The `app-p3` build recipe
selects the later instrumentation candidate and preserves that original copy:

```sh
cd /Users/ryan/dev/chirality-t3/swbpipe-rt-perf-c1
RT_C1_KIT=/Users/ryan/dev/chirality-t3/scratch/rt-perf-c1-slice7
RT_C1_TARGET=/Users/ryan/dev/chirality-t3/targets/rt-perf-c1
RT_C1_APP="$RT_C1_KIT/app-p3"
mkdir -p "$RT_C1_KIT/models" "$RT_C1_APP"
/Users/ryan/dev/chirality-t3/tools/t3_slot.sh python3 core/solver/performance_harness/rack/rack_model.py --profile linear-ex --nodes 500 --output "$RT_C1_KIT/models/linear-ex-n0500.json"
/Users/ryan/dev/chirality-t3/tools/t3_slot.sh python3 core/solver/performance_harness/rack/rack_model.py --profile linear-ex --nodes 1000 --output "$RT_C1_KIT/models/linear-ex-n1000.json"
env CARGO_TARGET_DIR="$RT_C1_TARGET" /Users/ryan/dev/chirality-t3/tools/t3_slot.sh node apps/desktop/native-smoke/perf.mjs --build --app-dir "$RT_C1_APP" --target-dir "$RT_C1_TARGET"
node apps/desktop/native-smoke/perf.mjs --plan --plan-file apps/desktop/native-smoke/perf-plan-c1.json --subset c1-500
node apps/desktop/native-smoke/perf.mjs --plan --plan-file apps/desktop/native-smoke/perf-plan-c1.json --subset c1-1000
```

The P3 command requires the later `app-p3` binding; the preserved a37d
App cannot execute it. Its submitting revision and artifact digest are recorded above. The two `--plan` commands
have passed. The new plan's default selects only 500; it has no 2,000-node run.
Do not use the old `perf-plan.json` default window, which includes 2,000.

## Repaired one-run recipe (approved, consumed and closed)

After focused review and exact-head CI, the owner approved **one 500-node
attempt only** using this repaired harness and the unchanged bound `app-p3` executable.
It executed and failed as recorded above. The following recipe is historical,
not permission to execute it again. Use the
existing variables above; do not build or nest a slot wrapper inside the window.
Reverify the selected clean source and every current external/binary/model hash.
The output below now contains the failed packet and must be preserved; the
original `n0500` packet is also immutable.

```sh
/Users/ryan/dev/chirality-t3/tools/t3_exclusive.sh env -u SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY node apps/desktop/native-smoke/perf.mjs --run --app-dir "$RT_C1_APP" --models "$RT_C1_KIT/models" --plan-file apps/desktop/native-smoke/perf-plan-c1.json --subset c1-500 --budget-s 840 --out "$RT_C1_KIT/evidence/n0500-refresh1"
```

That one-run grant is consumed. There is no automatic retry or new window,
no B1000 even if a later separate attempt passes, and no 2,000-node scope. The
same no-capture/LLM/auth/network/release restrictions remain. Any further native
work requires a separately concrete proposal and fresh authorization.

## Interactions and usable-size gate

One native solve per fresh private App launch/throwaway store; no injected
result, C2 full copy, worker-budget increase or pool reconfiguration. First verify
the solve proof matches the model JCS hash, sparse mode, native result custody,
and nonzero displayed rows. Record every refusal verbatim; do not alter input,
result or standing to make a run progress.

The baseline driver already opens the document, chooses sparse, runs
`analyze.run`, waits for `first_paint`, records a named run and saves the project.
P2 adds the following bounded probes to that same maintained driver:

1. Wait for actual result rows as well as the two-animation-frame timeline mark.
   Select a row, next/previous entity and case, next/previous page, and one filter
   followed by clearing it. Log selected published row IDs, page positions and
   action-to-settled-DOM times. A visible no-match result is distinct from a typed
   refusal. Check the result detail resolves and controls continue responding.
2. Stream one displayed group's CSV through its existing button into the private
   Downloads directory. Check its receipt, native byte length and complete row
   count/CSV structure outside the webview; hold no full row array in the driver.
   Existing byte-parity tests remain the format oracle, not a claim that a live
   file has already been checked.
3. Record Run and Save Project through the actual UI. Inspect SQLite only after
   the App has closed. A named SQLite-size refusal or model-only save is an
   honest outcome, but does not pass this round-trip usability gate. The combined
   SQLite row limit remains; C1 never promised its removal.
4. When a result was saved, use `file.open-local` in the same App session. Require
   historical origin, integrity findings as shown, native paged rows, navigation,
   and a native re-save with preserved evidence. Check the recorded history run
   via the existing History check (read/check/dispose). A restored run must never
   acquire fresh solve/Current authority. Unexpected Replace prompts are cancelled
   and reported, never automatically confirmed.
5. Release held results through normal close/reset and observe pool counters
   returning to their starting retained/worker/custody counts. Do not equate lower
   RSS with lease release or force JS GC to improve the result.

Report each action's latency and completion, not a new accepted usability time
threshold. Passing this sequence establishes use of those C1 paths at the
observed size on the recorded host. First paint, component tests or a small
footprint alone do not. Comparisons/derived JSON and other bounded C2 consumers
remain explicitly excluded. Two sizes on the development host establish neither
2,000-node usability nor the 16 GiB base-machine criterion.

## Memory and attribution

Use the existing sampler at 200 ms: `proc_pid_rusage` resident size,
`phys_footprint` and kernel lifetime maximum for the App PID, workers running the
private executable, and new WebKit PIDs relative to the prelaunch baseline.
Keep the timeline/progress/store report produced by `perf.mjs`. Record actual
host CPU/RAM/OS from that report; the development host is not the base machine.

- Per-PID readings and kernel lifetime maxima are direct observations. A short
  process or unreadable PID can be missed; absence is unknown, not zero. `c1_physical` reports the aligned sampled peak for the
  complete interval, before/during/after P3, with unknown-sample counts. An
  absent worker contributes zero only after the product `job_done` mark, and
  that zero is explicitly labelled inferred. Per-PID lifetime maxima in a
  phase are still lifetime observations, not phase-only maxima. Match
  worker PID/argv and its product-reported lifetime maximum with the sampler.
- New WebContent is attributed by executable path and launch baseline, not proven
  parentage: WebKit launches from launchd. More than one candidate, replacement
  WebContent, an unrelated WebKit launch or missing readings makes attribution
  ambiguous. Do not choose the smallest PID footprint or progress to 1,000 on
  that evidence.
- Compute `max_t(app + worker + WebContent)` from the same timeline rows, with
  missing values marked unknown. Those PID reads occur sequentially in one
  polling interval: this is an aligned sampled sum, not an atomic concurrent
  maximum. Never add separately timed per-process maxima and call it concurrent
  peak. GPU/networking peaks are reported separately; the compact timeline lacks
  their synchronous sum. Shared pages, compression and unobserved transients
  prevent a proof of total base-machine fit from these sums.
- The existing `reader_verdict_start/end` marks cover registration and can contain
  several native reads; `job_done` separates producer completion from delivery
  work. They do not isolate the reader's own heap from parsing. Prescan M is an
  admission bound, not measured resident memory. Subtracting it from App footprint
  would not measure reader working peak.
- P3 must separately report actual parser allocation peak and reader-only peak
  (requested bytes and existing allocation-envelope bytes), with model/input
  parsing already held outside the reader-only count. Cover dispatch and every
  registration reader, hashed row-ID sets/evidence indexes, and the summary and
  semantics-batch working passes reached by these interactions. Reuse the
  maintained `ingress::counting`/allocator test facility and the same Value reader
  APIs behind `operation_applier`; no core/pricing edit or generic profiling layer.
  The existing counter is per-thread and covers Rust allocator requests; identify
  any off-thread or native-library allocation it does not count.
  The P3 command in `src-tauri/src/native_smoke.rs` is
  `native_smoke_profile_readers`: raw args `{jobId}`, with the existing ingress
  header. The driver invokes it once after fresh table/CSV probes and before
  persistence, within the same 810 s child budget and a 90 s profile wait.
  It has no standalone model/result arguments or export. It reads only this
  launch's plan model path, checks JSON.stringify bytes against the job's intake
  digest and checks held bytes against its publication digest. Model/invocation
  wrapper and solver mode match the production registration. Source, model and
  invocation parse allocations are counted separately. Dispatch and the route's
  actual registration reader run on held parsed Values; all rows pass through
  borrowed semantics batches of at most 1,000, reporting maxima rather than the
  sum of batch peaks. Summary includes its response/admission work, with the
  health-view map measured separately.

  Numerical health's Value API is private. Its text entry reports a directly
  counted combined parse/work peak and an **inferred upper bound** for working
  requested/envelope bytes: combined peak minus the separately measured live
  source tree. It does not subtract prescan M or physical memory, and it is not
  labelled an isolated health working measurement. It includes findings parsing
  and serialization, and can overstate health work because parser transients
  also contribute. No core API is changed to make the bound smaller.

  The command returns only bounded scalar metrics, phase epochs, lengths and
  digests (64 KiB cap); no standing, result, model, record, rows or capture.
  It rechecks custody after all phases and never registers or upgrades a handle.
  Pool leases for raw args, file/texts, each parse, normalization scratch and
  summary response precede their allocations under the existing capped pool.
  Reader working/health text allocations remain the existing unpriced residuals
  being observed, not newly claimed bounded allocations. All work is off the
  executor. Allocation accounting disarms on unwind and rejects nesting.

  The later App/worker includes inactive per-thread counter checks in allocation
  calls even outside the P3 phase. Record that instrumentation overhead; it is
  not byte-identical to the reviewed a37d runtime. Driver/native epoch markers
  separate P3 from ordinary pre-probe App/worker/WebContent samples. Later
  persistence/reopen samples can retain allocator caches created by P3; they
  cannot be described as the uninstrumented product's physical footprint.
  These component allocations do not replace the live P2 observations. If the large input or probe is absent,
  this part of the gate is incomplete rather than inferred from small fixtures.

Keep held result bytes, parse M/reservations, counted working heap, response
buffers, App physical peak and WebContent physical peak as separate columns.
SQLite cache/journal/FTS, allocator cache, thread stacks and Tauri delivery remain
physical-process residuals unless directly attributed. Report absolute working
peak and its context; do not adopt a new materiality threshold. A material reader
share needs its bound/lease decision before leaving draft, per the C1 plan.

## Resource limits, cancellation and cleanup

The original A/B approval is consumed/blocked as recorded above. The new
one-run proposal is 500 only and holds all four host slots for at most **900 seconds including teardown**;
the runner work budget is 840, with no builds inside either interval. Queue/drain
time precedes acquisition and is reported separately from UTC acquisition/release
and actual held duration. Preserve reservation order. No automatic second window
after failure or timeout, and no 2,000-node attempt: that needs
a separate decision after the observed evidence and reader-residual disposition.

Entry: memguard running, exclusive locks acquired, host pressure normal and
known, `kern.memorystatus_level >= 30`, T3 resident use below 52 GiB, and fresh
native pool status with no prior retained/queued/running jobs, its unchanged
12 GiB invocation budget, and known headroom at least that budget. Do not raise
pool/budget or bypass a named admission refusal. Product admissions keep their
existing fresh-sample/normal-pressure/resampling rules; there is no external
retry of a refused solve.

P1 must monitor host/pool status and owned process samples. Stop on pressure
warning/critical/unknown, host available level below 30, unknown attribution,
typed resource refusal, process crash/replacement, a hung required interaction,
or an aligned App+worker+WebContent footprint sum above 12 GiB. That sum is a
run-only protective stopping criterion, not a new product budget or a proof of
16 GiB fit. Any owned PID RSS above 16 GiB is an emergency stop. The host's
22%-available memguard remains enabled as the last backstop.

On stop, try `analyze.cancel` for an active solve; allow at most 5 seconds for
its cancellation/reap receipt. A completed native reader may not be cancellable;
do not wait indefinitely. Terminate the runner/App/helper group, and identify
product workers separately: `solve_worker.rs` deliberately gives them their own
process groups. Freeze the current App PID before enumerating PIDs whose executable
path is this window's unique private App copy; stop those worker groups/PIDs
before removing that copy. Never infer worker termination from App exit alone.
Leave the runner's parent-death guard runnable during that freeze.
The deadline must begin teardown by 840 seconds and finish/release
the exclusive wrapper by 900, even if store inspection hangs. Do not kill a
global process name or an ambiguously attributed WebKit process. Stop the owned
sampler/caffeinate, close the App, and check no owned worker remains. Unexpected
remaining processes or ambiguous WebKit teardown are reported and prohibit B.

After all writers exit, retain the compact JSON/progress/PID evidence, verify
SQLite `quick_check`, recorded IDs, stored lengths and integrity findings against
the reported outcomes, and check CSV receipt/file cleanup. A killed writer is
checked as an interrupted transaction, not assumed to have committed. Remove
only this run's private HOME/store/downloads and temporary App copy after those
checks. On failure keep only the named private artifacts needed to diagnose it;
report their path/size and remove them when that consumer finishes. Keep the
bound App/lane cache while this proposed window still needs them. No broad host
cleanup utility or other lane's files. Report cleanup and lock-release failures
truthfully; do not extend the interval to hide them.

## Verification status and smallest grant

P1 — **Independent round 3 clear at `dee93a72`.** The first independent
review at `3faf9ea12` blocked supervisor-death cleanup and forced-stop evidence
loss (F1/F2); both new in-model kinds were reported to Agent 0. The parent repaired
them and the smaller failure-handling notes without an implementation delegate.
This is no risk acceptance or native-window grant. `perf.mjs --run` now
forks the runner into its own process group under an independent supervising
event loop. The App/helpers inherit that group; synchronous SQLite inspection
can block only the runner. The supervisor begins stopping at the work deadline
(maximum 840 s), escalates after 5 s, bounds a missing exit receipt by a further
5 s, bounds worker discovery by 1.5 s, checks disappearance for up to 2 s and
bounds its asynchronous report/cleanup by 10 s. These limits leave teardown
margin within 900 s; actual held duration and cleanup are still run evidence.
The child receives a smaller run budget (810 s for the proposed 840 s window),
reserving 30 s for post-run work before the supervisor's deadline. That reserve
does not guarantee a multi-GB SQLite check completes; an unfinished check remains
partial evidence, not a successful save verification.

The child arms IPC-disconnect self-group termination and a same-module guard
thread before starting the App. The thread checks reparenting every 100 ms even
when the runner's main thread is blocked in SQLite. SIGHUP uses the supervisor's
bounded stop path. Normal handover removes the disconnect handler and unreferences
the guard without keeping the runner alive; it still protects delayed helper/I/O
exit. Abnormal supervisor loss supplies no normal window receipt: recovery uses
the durable partial carriers and verifies process closure before another window.
The round-2 recheck cleared F1–F6 at `f0bf1c5bd` but found that self-stop hid a
runner error's cause (F7). The child now writes that cause synchronously to stderr
before self-group termination, with an optional bounded IPC stop reason; the new
fake-process regression checks the cause survives. This is a failure-reporting
repair within the existing threat model, with no additional native authority.
The final reviewer independently ran all 26 tests and cleared F7. Its mutation
also showed the small fixture can pass with asynchronous console output, so the
tests do not establish that synchronous writing is necessary. A full non-blocking
stderr pipe can still make this diagnostic best-effort; it is not an unconditional
durable cause in the window JSON. The optional IPC reason and incomplete/failed
receipt semantics are unchanged. No fourth P1 review is opened for these notes.

The product worker's own process-group behavior is preserved. Before final
cleanup the supervisor freezes the current App PID, not its guard thread, and uses the sampler's
control-only `--list-owned` mode (no memory counters) to find PIDs/groups by the
unique private executable path. It kills only that closed set plus its own runner
group, verifies disappearance, and reports failed/unknown lookup as unverified
cleanup. WebKit candidates are never targeted by name or PID. Private scratch
is removed asynchronously only after successful outcome and process verification;
failures retain its named path. `--keep` retains stores and removes the private
App copy after verification. There is no automatic next run after a failure.

The existing sampler additionally supplies host available-percent and
`kern.memorystatus_vm_pressure_level` readings (at most once per second: 1 normal,
2 warning, 4 critical, other values unknown). Elevated pressure, stale samples,
low headroom, an aligned sum above
12 GiB, PID RSS above 16 GiB, and ambiguous/replacement WebContent stop the window.
One transient unknown PID reading is retained but tolerated; two consecutive
unknowns for that PID stop the run. A vanished PID is removed from the pending
set. Unknown host status needs two distinct 1 Hz readings, not two cached 200 ms
samples; known elevated pressure or a known exceeded cap still stops immediately.
These are run controls, not changes to product pool/budget admission. Native pool
pressure/preflight and UI cancellation probes still belong to P2. A supervisor
forced stop does not claim a successful UI cancellation or transaction commit.

Raw sampler JSONL is exclusively created outside disposable HOME/scratch and
flushed before the line is sent to its consumer. Each completed run is atomically
recorded before the next App starts; its ok HOME is then removed unless `--keep`.
Thus prior stores do not accumulate for a multi-run window. Samples/records survive
normal scratch removal. The supervisor keeps the latest/triggering compact sample
and completed-record paths, labels forced-stop evidence incomplete, and records
inspection-start/completion separately. Failed/unknown inspection is not a commit
receipt. Transient group-probe EPERM is polled as still live within the existing
2 s limit; persistent uncertainty fails verification.

Twenty-six maintained tests pass under `t3_slot.sh`, including only tiny fake
Node processes for blocked inspection, workers in separate groups, supervisor
death with a stopped fake App, normal guard handover, SIGHUP, repeated interruption
and failed/stalled lookup. Added regressions check preserved records/raw samples,
cleanup containment and transient host/PID readings. The production sampler writer
was exercised only as an isolated pure AST function with synthetic data; its module,
ctypes, sampler modes and sysctls were not executed. No native App, worker solve, sampler mode or
memory measurement was launched. The own-executable discovery is source-reviewed
preparation until separately authorized live evidence exists. Scope for the
required safeguard review is the existing honest-local-mistake threat model:
finite teardown, closed owned-PID/path and scratch containment, honest failure
reporting and no unrelated-process kill. Hostile concurrent path/PID substitution
remains outside the existing kit's threat model; this is not risk acceptance.

P2 — **Implemented offline; connected review clear at `92983690e`.**
The C1 plan now enables bounded probes for all three navigator axes, next/previous
pages, row detail, no-match/clear filtering, streamed group CSV, Record Run,
native project reopen/re-save, recorded-run check/disposal, normal model-open
retirement and pool counts returning to their starting state. The trace rejects
whole-value/legacy persistence and budget-changing commands. It retains only
counts, bounded native headers, small receipts and metadata; no row page, full
result, record, model or CSV chunk is retained by the observer.

The input plan binds the two existing maintained rack hashes and node counts;
changed bytes, wrong node count or a size escalation is refused before App launch.
The solve proof must match the model JCS hash. A separate native-table-ready time
waits for settled rows and two animation frames after Results activation; the
older solve `first_paint` mark alone does not show that table. These are DOM/frame
observations, not pixel verification. Required interactions record latencies and
published IDs/positions, without a new accepted usability threshold.

Native persistence is observed through its actual DTOs (`run`/`message` for Record
Run, small result outcome for saves). Reopen and re-save must preserve producer
identity, handle/origin, envelope evidence and the complete findings list. A
model-only SQLite save or named refusal fails the round-trip usability probe;
it is not called a half-written result. CSV inspection streams with Python's
standard CSV reader outside the webview, checks BOM/preamble/width/receipt bytes
and compares data-row count with the native group's query total. Inspection
failure is unverified evidence, classified `refused_with_message` at step
`csv_inspection` with `needs_review`, not assumed file corruption. Python CSV
inspection uses isolated mode (`-I`).

The preflight refuses prior held jobs, changed budget or unknown/insufficient
headroom. Live monitoring uses native pool status, stops immediately on known
elevated pressure, after two unknown replies, or after an outstanding status
request exceeds 3 s on two consecutive timer ticks. Staleness starts at request
dispatch; a delayed webview tick before a request is not low-memory evidence. It attempts
bounded cancellation on failure without inventing a successful receipt. Terminal
UI outcomes remain verbatim; the separate low-level refusal trace is bounded
and marks truncation/omission. Transient admission retries remain observable,
not reclassified as terminal product failures.

The current thirty-eight Node tests pass under the slot wrapper. Added tests use DOM/IPC
emulation and the committed Rust response shapes, plus synthetic CSV files and
pool readings; they are source-mode evidence only. They do not establish actual
native interaction, cleanup, first table paint, memory fit or the complete P3
gate. The P1 control block is byte-identical to its reviewed `dee93a72` source;
P2 changes input validation, observation, UI probes and read-only file inspection.
No new product command or scope is needed for P2.

P3 — **Implemented and bound offline; connected review clear at `92983690e`.**
The existing native-smoke feature now supplies the read-only scalar command
above and the C1 plan enables it. The existing allocator facility is also
compiled under that feature, remaining inactive except on the probe thread.
Default product builds do not compile these hooks or the new command. A standard
TLS macro is now visible to the user-release scan: its one qualified invocation
is admitted only in ingress, its body must be parsed statics and every initializer
is scanned with the existing rules. No opaque macro exemption or handler-macro
re-pin is added; the ordinary product handler definition is unchanged.

The independent review at `92983690e65ba3292a3d3ea87183fb383b92272c`
found no blockers or new in-model failure kind. It independently checked hashes,
binding/leases, production DTOs, the health upper bound and TLS scan. Its own
checks: 36 Node tests; 69 targeted Rust tests across ingress, native-smoke and
reach scans (including the real-source scans); non-test smoke `cargo check`.
Inherited author checks remain the default product check, TypeScript/release
build, model-version 8, timeline/plan validation and graph 0 errors/one existing
cycle. No full local Rust/vitest run is claimed after P2/P3; exact-head hosted
verification remains required. No successful probe, allocation profile or native
App invocation has run.

Parent follow-ups to the nonblocking notes change only external scripts/tests
and Work text. The pool monitor tolerates delayed ticks and requires two stale
pending-request ticks. Before potentially blocking store/CSV inspection, the
runner atomically checkpoints the exited App's report, memory timeline, source
identities and inspection-pending state outside disposable scratch. Its class
is `refused_with_message` and `evidence_complete` is false; it is never announced
as a completed run. A committed final record supersedes/removes it. A deadline
while inspecting still fails the gate; the checkpoint supplies partial evidence,
not a commit receipt or permission to exceed the window. CSV inspection uses
isolated Python. All 38 Node tests pass, including delayed-page/pending-request
and unfinished-inspection regressions. P1's control block remains byte-identical;
no additional reviewer is requested for these contained note repairs.

Remaining interpretation limits, recorded rather than expanded:

- No successful profiling command has run. The maintained generator writes
  sorted keys; native Open reads those through serde, and the session adopts
  the opened document unchanged. Existing open/solve source-mode and JS-text
  writer tests support that input basis, but JCS equality alone would not
  establish exact intake key order. The first authorized 500-node run is the
  connecting-path canary. An intake-order refusal ends that gate without retry.
- `semantics_working_max` has maximum requested/priced/live byte values across
  all bounded batches; `elapsed_us` and allocation count are **sums**. Its rows
  are borrowed, so this is the per-row reader/output work compared with the
  existing 4 MiB working allowance, not total production command memory.
- `model_parse_bound_bytes` names the raw model file's admission bound. The
  measured model parse reads its JSON.stringify-normalized text. The raw bound
  covers that same model tree; it is not labelled an exact normalized-text size.
- The inferred health bound depends on the selected engine keeping its parsed
  source alive through health construction/serialization. That order was
  independently read at the bound source; no test pins its lifetime. A change
  there invalidates this inference until rechecked. Parser transients remain
  separately observed, not inferred from a partly built tree.
- The smoke-only non-test build warns about unused `counting::live_priced`;
  this does not affect the probe. The 30 s inspection reserve does not promise
  that large store/CSV checks finish; incomplete inspection fails the gate.

The live allocation/timing run belongs inside the authorized interval. If it
cannot fit with the App sequence inside A/B, return a bounded additional-interval
proposal; do not silently add one or extend either interval.

The original narrow launch/data grant was given and A consumed it; it supplies
no retry, B, capture or model-provider authority. After the focused repair review
and exact-head CI, the smallest new request is the one 500-node interval above
with the same local UI/data/read-only evidence scope. **No screenshot, video or
screen recording is needed.** There is no embedded LLM, provider authentication,
credential/keychain, network model, supplier, signing or release requirement.

Both generated models have measured static container depth **7**. The committed
v0.3 builder corpus's records have maximum depth **14**, well below the parser's
127-container boundary. These are static JSON checks, not a native reopening
measurement. The selected racks add array entries, not a new nesting form; they
do not exercise the forged/deep stored-record edge. Actual generated record/header
depth and native reopen success still belong in the run evidence. Do not turn
this source/corpus assessment into acceptance or an assertion that every stored
record can reopen.

The maintained `rackModelOpen.test.ts` now also passes both exact-size generated
documents through File > Open Model Document's `checkNativeOutcome` shape/Wasm
validation: **6 tests pass**. The generator subprocess needed an explicit 8 MiB
stdout buffer for the 1.8/3.6 MB documents; the initial run hit Node's default
`ENOBUFS`, corrected in that test transport only. No product limit or expected
result changed. This is source-mode validation; it launches neither the App nor
a solve and establishes no first-paint, usable-size, depth-safe actual reopen or
memory-fit result. Both plan subsets validate offline; the deliverables check
reports 0 errors. No additional reviewer or broad suite replay was started for
this preparation.

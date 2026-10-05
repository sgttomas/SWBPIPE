# I65 U4 G5 part 2: return

**Status.** Part 2 is complete. Every control passes, apart from the recorded survivors (§Controls).
- **Code:** WT/f2a-memory, branch `codex/piping-f2a-memory-20261004`, on top of `1e323058f3`, uncommitted. It touches 7 files, all inside the fence (IMPLEMENTATION_PART2.md §1).
- **Records:** this folder. IMPLEMENTATION_PART2.md has the map, the record and every routed item. `_run_records/` holds the generator, the profile tree, the text run, the printed record, the witnesses, the challenge, the mutants and the controls. `SHA256SUMS` covers this folder.
- **The registry is empty.** `REGISTERED_PROFILES` is `&[]`, so no permit is constructible. The bound also stays `Unpriced` while 42 Estimate atoms remain (§Estimates). The law therefore fails closed twice over until G6.

## The in-build maximum (the record G6 uses)

Debug test build, rustc 1.97.1, aarch64-apple-darwin. Layouts do not depend on the opt level.

| Mode | E_mov,max (W3) | + R | Fraction of M | Below 0.9 M by |
|---|---|---|---|---|
| Sparse | 3,493,720,906 | 3,560,829,770 | **0.8843** | 63,048,886 B |
| Dense | 3,513,431,354 | 3,580,540,218 | **0.8892** | 43,338,438 B |

- **The 0.9 M rule holds at in-build strides, so nothing was adjusted.**
- The per-phase table is in IMPLEMENTATION_PART2.md §2.1 and `_run_records/profile_record_p2.txt`.
- A lib test asserts the table, so any change to the profile must regenerate the record.
- The Estimate atoms weigh 6,053,120 B in the maximum.

## Controls

| Control | Result |
|---|---|
| 1. No published byte changes | The fixture sweep is **byte-identical to base `8abb5274a9`** (sha256 `0690bc64…41e1` for both, every public admission-report field included) |
| 2. Nothing weakened | – **PP:** 694 passed, 1 failed, 10 ignored, against base's 661 / 1 / 2. The extra are part 1's 23 and part 2's 9 law tests, the challenge, and the 8 ignored witnesses. The failure is the known Mac t13, as at base.<br>– **runner/headless, FK and SR:** outcomes identical to base (85/2; 546/0/1 ignored; 48).<br>– **Lib warnings:** 8, as at base.<br>– No reader, schema, fixture or existing check changed. There is no test permit |
| 3. Mutants | **148 run, 146 killed by a test, 0 compile-only.** That is part 1's 84, RV89's 24 and part 2's 40, covering the profile (the Estimate gate, maximum, combinators, forms, node law, bindings, FK exports), the gate caps, the budgets, push capacity and N-4.<br>**The 2 survivors are recorded:**<br>– "build_status ignores bindings" is equivalent by decision 7;<br>– RV89's V19 (build.rs) cannot be observed in-crate, and RV89's standalone run covers it.<br>The first run's five gaps were closed with tests (IMPLEMENTATION_PART2.md §7.1) |
| Witnesses | W1–W7 and the 1 MiB headroom witness pass in debug at R/16 = 4 MiB, one process each (IMPLEMENTATION_PART2.md §4). W5 is the dense halves |
| Challenge | The peaks are 533,234 / 540,831 B (milestone) and 13,191,619 / 13,199,237 B (large D1 input). Each is ≤ 0.72% of the in-build W1 phase |

## Routed items, each separately

**RV84:**
- **C-N1(b):** done. L_BODY = max(L_ROW, L_DIAGID = 2,330) in T25's receipt body. C-N1(a)'s L_PUB also enters the chain.
- **C-N2:** done. Each factor term carries its `dof` number.
- **C-N3(a):** done. T19 is in every phase, X1 and W1–W4 included.
- **C-N3(b):** done, as ruled. The reader statics are in every phase.
- **C-N3(c):** done. RECEIPT_X is removed.

**RV87:**
- **S-1:** done. T16 P2 carries the third `run_v`/`selection_v` copy.
- **S-2:** done. The broadened result-id rule matches **16 sites, 10 more than RV87's six**, and all 10 are genuine `ResultItem` ids:
  - retained_product.rs:2239 and :2434;
  - source_receipt/rows.rs:294, :522, :608, :610, :631 and :633;
  - preview_physics.rs:517 and :690.
  
  It adds +71,882,496 B to TAV (TAV_W +53.1 MB), against RV87's 9.75 MB estimate. **RV83 confirms R-4, and RV87 confirms S-2,** using `text_p2/` (the rebase run is included for the comparison).
- **N-1:** each part done:
  - (a) the diagnostics push doubling;
  - (b) the `source_result_refs` backings;
  - (c) `row_ids` doubling;
  - (d) the corrected grammar;
  - (e) grant 1b's exact notice reservation.
- **N-2:** done. The `basis_ref` clones and the per-object walkers.

**Carries:**
- **Carry 3:** done. `collect_string` at lib.rs:1764, +16,080 B.
- **Carry 8:** the headroom witness passes at 1 MiB. Whether W1 drives the 36-level `$ref` chain needs reader instrumentation, which is outside the fence, so it stays a residual for G6.
- **Carry 11:** done. Every G4 expression is evaluated in the build (the record above).

**RV85 U1** (optional): **not done.**
- A permit-to-invocation binding needs `CapturePermit` and `permitted_run` changes in lib.rs, which is outside D-5.
- A compile-time not-`Clone` assertion would be killable only by a compile error, against control 3.
- I leave it to ROOT or G6.

**RV89, on part 1:**
- **S-1:** three tests. V03, V06, V07 and V08 are now killed.
- **N-2:** one test. V17 and V24 are now killed, and V19 needs no test.
- **N-4:** the code guard in `bindings_hold`, plus a test. Its two mutants are killed.
- **N-1:** part 1's tally is restated as 81 behavioural kills, 2 text pins and 1 equivalent.
- **N-6:** corrected in IMPLEMENTATION_PART2.md §9, with the 10 new multiplicity entries listed.
- **N-3:** accepted. **N-5:** noted.

## The bound's binding status (for G6)

There are 244 atoms:
- 190 InBuild;
- 6 SourceUpper: serde's Content pair, and field-sum upper bounds for `Projection`, `RowTreatment`, `Derived`, the `(String, RowTreatment)` node and `Option<FormationRecord>`;
- 6 Text;
- **42 Estimate.**

Part 2 closed 6 of the 48. Among them, `Projection` rose from 80 to 192 B. The 42 remaining are listed with their strides in IMPLEMENTATION_PART2.md §8, and together weigh 6,053,120 B in the maximum. **G6 must close all 42 before a profile can register.** G6 should also witness each SourceUpper against `size_of` where the type is nameable (source_receipt and load_ledger, outside this fence).

## Decisions for ROOT

1. **Is the pinned record acceptable?** `profile_in_build_record` asserts this build's per-phase values, and `challenge_bounds_are_the_profile` ties the challenge to them.
   - Both are specific to the layouts of rustc 1.97.1 on aarch64-apple-darwin. A build with other layouts fails them until the record is regenerated. That matches G6's per-identity registration.
   - No CI job runs the PP cargo tests today.
   - The alternative is print-only. Four of the 40 P2 mutants are killed only by the pinned record: X2 dropping the reserve, the Content pair halved, the Projection upper losing a String, and Wide<16> read as Wide<8>. All four would then survive.
2. **N-6 was recorded as an erratum, not an in-place edit.** Part 1's sealed R4_CALLGRAPH.md is unchanged. ROOT may direct an in-place edit instead.

## Execution record

- **Who:** I65, TASK (Type 2) under ROOT. No descendants.
- **When:** 2026-10-04.
- **Memory guard:** `memguard.sh` PID 5387 ran throughout, and every cargo job checked it first.
- **Cargo:** the default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2` (1 for the witnesses and the challenge), one cargo job at a time.
- **Writes:**
  - the 7 code files in WT/f2a-memory;
  - this folder;
  - WT/scratch/i65_u4_g5_01/ (copies, logs, mutant and control trees);
  - WT/targets/i65-g5/.
  
  TMPDIR was not redirected for cargo, as in part 1.
- **Not run:** no Git writes or index operations (Git reads used `GIT_OPTIONAL_LOCKS=0`), no installs, no new tooling, and no native, solver-at-scale or DEC-025 jobs. The Python scripts use the standard library only.
- **Records:** placeholder paths only (WT, R, NUM, PP, FK, SR); a check for machine paths finds none. `SHA256SUMS` covers every file in `part2/`. Part 1's SHA256SUMS and files are unchanged.


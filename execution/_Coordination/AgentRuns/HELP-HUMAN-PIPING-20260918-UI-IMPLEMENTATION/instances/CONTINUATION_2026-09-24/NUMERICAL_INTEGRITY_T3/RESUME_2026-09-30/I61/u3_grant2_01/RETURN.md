# I61 RETURN: U3 grant 2 (the permitted path on the actual Direct entry; the live milestone)

**Status: complete, with no stop.** The work is uncommitted in WT/f2a-memory, on top of the registration commit `0c7827b6ad`.
- **The milestone's actual Direct entry publishes U1's pinned successor in both modes** in the registered dev/test build (sparse `ac6986b0…`, dense `6cd1d249…`), from exactly one ordinary run.
- **Every W1 fallback publishes the ordinary bytes plus exactly one N1 notice.** Every refusal before W1 work publishes the exact ordinary bytes. Both are pinned by committed tests on the actual entry.
- **Controls 1–4 hold:**
  - Builds without a registration publish base's bytes on all 324 outputs.
  - In the registered build, only the Direct entry differs, and only as ruled.
  - Nothing is weakened.
  - 11 of 11 mutants are killed, none by a compile error.
- **U5 rerun on the live bytes:** `u5_report.json` and `u5_run.log` are byte-identical to U5's. RV86's limit 4 is discharged.
- **Deliverable 5 (D-U6-5) is deferred by ROOT,** not missing (ROOT's scope note: U6's fixtures are not on this branch; a follow-on after ROOT merges NUM with U6).
- **Deliverable 7 (optional) is not done.** See §6.

**Run facts.**
- **Role and basis:** TASK Type 2 under ROOT, with no descendants. The basis is:
  - the brief `BRIEFS/I61_U3_GRANT2.md` (NUM `f9eeb24f76`);
  - ROOT's dispatch message and its scope note deferring D-U6-5;
  - RR "Registration applied; M = 4,026,531,840 B selected under D-7; U3 grant 2 dispatched", and the rulings it cites;
  - RV85's U1, U4 and N2, and RV82's N3 (B′);
  - RV86's U5 review, read only.
- **Time:** 2026-10-04, 15:09Z to about 15:47Z.
- **Host:**
  - The memory guard (PID 5387) ran throughout.
  - Default toolchain; `--locked --offline`; `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`; one cargo job of mine at a time, each under a perl alarm.
  - I65's own cargo jobs (`targets/i65_g7`) were running on the host at times. I did not touch them.
- **Targets:** WT/targets/i61-u3g2/ only (`reg`, `stale`, `reg-runner`, `unreg`, `base`, `mut`). **WT/f2a-memory's default target was never used, and its profile was not touched.**
- **Git:** no Git writes. Reads used `GIT_OPTIONAL_LOCKS=0`. `git archive` made the scratch trees.
- **Python (U5):** I52's prebuilt checked-JSON and units binaries, as before. Nothing was installed or built.
- **Writes:**
  - five files in WT/f2a-memory, PP only (§1);
  - WT/scratch/i61_u3_grant2_01/;
  - WT/targets/i61-u3g2/;
  - this folder;
  - in `R/I61/u5_reference_01/`, the two-line pin in `_run_records/u5_compare.py`, which the brief's fence allows, plus that file's line in its SHA256SUMS (§5).
- **Other agents' files:** none touched. `retained_memory.rs` is read only and unchanged. Its existing `cfg(test)` reserved-stack override is used as is.

## 1. What changed (PP only; production text unchanged)

| File | Change |
|---|---|
| `PP/src/retained_facade_tests.rs` | +276 lines: the five grant-2 tests (§2) |
| `PP/src/retained_tests_hooks/grant2.rs` | **new**, 95 lines: the test-only seams, as a child module of `retained_tests_hooks` |
| `PP/src/lib.rs` | 2 production lines, each gaining one `#[cfg(test)]` statement (`:2379`, `:3005`), and 8 lines inside the `#[cfg(test)]` seam module |
| `PP/src/retained_product.rs` | 1 production line gaining one `#[cfg(test)]` statement (`:3244`, G-B) |
| `PP/src/retained_wire_tests.rs` | RV82 N3's comment correction: B′ now names its committed test; one assertion label corrected |

**Production text is unchanged, and line-neutral** (`production_equivalence.txt`):
- `lib.rs` and `retained_product.rs` keep their line counts (24,334 and 3,909).
- Removing the added `#[cfg(test)]` fragments restores every production line byte for byte.
- So line-keyed inputs, such as U4's TEXT inventory and audit keys (RV87's re-qualification obligation), see the same production source at the same lines.
- **Warnings are identical to base,** for both the production `--lib` build and the lib test build (`suites/warn_*.warnings`).

**The seams** (test builds only):
- **`ordinary_run_entered`** sits at `run_linear_static_preview_observed`'s first statement, which every ordinary run passes. It is B-1's hook.
- **`at_complete_gate`** sits in `permitted_run` just before `check_complete`. It counts G-C as consulted, then applies any armed observer fault.
- **`before_late_gate`** sits in `prepared_case_source` just before `check_late`.
- **A per-invocation `Tally`** (ordinary runs and G-C consultations) is shared, not copied. `carry_test_hooks` carries it to the reserved-stack thread beside the armed faults.

**The faults:**
- **G-B and G-C** raise the observer's `RustCapacityBytes` record above every bound. U4's actual `check_late` / `check_complete` then refuse, on `LateObservationBytes` and `ObservationBytes`. No refusal is fabricated.
- **Preparation and candidate** reuse the private driver's triggers (a zero diameter; a proof trace fault).
- The existing native, staging, serializer and precommit faults are unchanged.

**No test permit exists** (decision 7). Every permit in these tests comes from `admit` in the registered build.

## 2. The committed tests (deliverables 1–4)

Each test asserts the registered behaviour when the compiled identity is the registered one, and the unchanged ordinary route otherwise. So the same suite is the control in both builds, and each call asserts the report's status (Registered or Stale).

| Test | Registered build | Other builds |
|---|---|---|
| `u3g2_direct_entry_publishes_the_pinned_successor` | Admitted. **B-1:** 1 ordinary run, G-C consulted once. `into_publication()` is the successor; its document has U1's pinned file and receipt hashes, both modes. **B′:** `envelope()` is the plain run's bytes. With `I61_U3G2_OUT` it writes the successors used for U5 | 1 run; publication = plain bytes |
| `u3g2_direct_entry_w1_fallbacks_append_one_notice` | Preparation, native, candidate (maxima; values completion), staging, serializer (receipt-encoding `work_counter_inconsistent`; association), precommit (G1) and precommit binding (G8), both modes. Each gives its typed cause, no successor, 1 run with G-C once, **exactly one** `RETAINED_PRECISION_UNAVAILABLE`, and bytes equal to the plain bytes plus the N1 notice (with C1:68's detail where it applies). The fault fired on the worker | No W1; the fault stays armed on the caller; plain bytes |
| `u3g2_direct_entry_no_w1_refusals_keep_exact_bytes` | **G-A:** two cases and a combination (D1.4), schema 0.3.0 (D1.3), Headless (D1.0). **G-C:** an unattempted solve (`OrdinarySolveNotAttempted`, observed 1, cap 0) and a G-C refusal of the admitted milestone (`ObservationBytes`). **The stack:** spawn failure, giving `StackReservation`, with the armed fault handed back. **Coexistence:** the real exact-selecting fixture `source_blocks/n05-sparse_interactive.request.json`, admitted, giving `Coexistence` with G-C **not** consulted. Each publishes the exact value-route bytes from 1 run | Same bytes; G-A refuses at D1.1 (Headless at D1.0) |
| `u3g2_late_gate_refusal_is_final_and_g_c_is_not_consulted` | **RV85 U4/N2:** G-B refuses (`LateGate`, `LateObservationBytes`), the bytes are exact, **G-C consulted 0 times**. The run would also have failed G-C: SV18 turns the cause into CompleteGate | No permit, no G-B: the fault stays armed |
| `u3g2_no_permit_path_runs_once_without_a_copy` | **B′ on the no-permit path:** the shared value route and a refused Direct entry each run once with no G-C. The no-permit dispatch (`run_linear_static_preview_value_dispatch` through `ordinary_dispatch`) contains no `.clone()`, `to_owned()` or `to_vec()`, makes exactly one ordinary-run call, and G-A borrows the parse's halves | Same |

**A note on "domain".** `permitted_run`'s defensive `W1Fallback::Domain` branch (load-state or exact-pressure) is unreachable with a permit, because D1.3 refuses those families at G-A. The test therefore covers domain refusals at G-A (D1.3, D1.4). `retained_w1`'s own `Domain` branch keeps its private-driver test.

## 3. Controls

| Control | Result |
|---|---|
| **1. Builds without a registration keep every route byte-identical to base** | **Holds.** The 324-output sweep (`sweep/`, this grant's in-crate harness on a scratch copy) was run in two unregistered builds. **(a)** The candidate built **Stale**, using a separate `CARGO_TARGET_DIR` and `RUSTFLAGS=--cfg=i61_u3g2_stale`; its identity records `rustflags=--cfg%3Di61_u3g2_stale`. **(b)** Read-only base `b43378d90a`, where nothing is registered (Missing). Both publish **base `b54caba7ab`'s bytes on all 324 outputs.** That is grant 1's sweep `faed2518…`, compared on the published bytes, because the report's Debug form has gained U4's law record since. Stale's admission records equal (b)'s apart from Missing becoming Stale, and every one of its 140 retained publications is `exact` |
| **2. Registered: only the Direct entry differs, and only as ruled** | **Holds.** Every typed, value and Headless output equals base, and so does every Direct error. Of the 70 Direct publications: **2 successors** (the milestone, both modes); **4 notices** (both `rejected_stress_range` fixtures, both modes, cause `Preparation`; RV89 N-2's fixture rows); **64 exact**, including 18 admitted `Coexistence` runs (n05, n06, their ui copies and `numerical_sensitive_torsion_model`, both modes); the other 2 of the 72 Direct outputs are errors equal to base's. The harness asserts each class's bytes, and that a notice appears exactly when W1 work ran (`compare.json`) |
| **Suites** | **PP, registered,** all targets with `--no-fail-fast`: **704 passed, 1 failed (Mac t13), 10 ignored.** That is ROOT's 699/1/10 plus the 5 new tests; per test it equals I65's registered run, apart from I65's scratch-only sweep. **PP, Stale:** the same 704/1/10, outcome-identical to registered. **runner/headless, registered:** 85 passed, 2 failed (base's own `load_reference` failures), identical to I65's registered run. The runner builds PP without `cfg(test)`, so it sees no change |
| **3. Nothing weakened** | No reader, schema, fixture or existing check changed, and no test permit exists. `retained_memory.rs` is untouched. The only existing-test edits are RV82 N3's comment and one assertion label |
| **4. Mutants, none killed only by a compile error** | **11 of 11 killed** (`mutants.json`; PP `--lib` in the registered build; the base t13 is excluded). Each new test kills at least one, and 4 are killed only by new tests |

**The mutants:**

| Mutant | Killed by |
|---|---|
| SV18: `permitted_run`'s G-B check removed (RV85) | `u3g2_late_gate…`, plus the structural gate-order test |
| SV18b: the check's text kept, branch never taken | **only** `u3g2_late_gate…` (cause CompleteGate; G-C consulted) |
| B-1: a second ordinary run on the permitted path, with no `clone()` spelling | **only** the four permitted-path tests (runs = 2) |
| B′: a second ordinary run on the no-permit path | `u3g2_no_permit…`, `u3g2_…no_w1_refusals…` |
| B′: a custody copy on the no-permit path | **only** `u3g2_no_permit…` |
| B′: the ordinary owner beside the successor altered | `u3g2_direct_entry_publishes…`, plus 3 existing tests |
| Preparation fallback without its notice | `u3g2_…w1_fallbacks…`, plus the private-driver test |
| G-C refusal ignored (W1 runs) | `u3g2_…no_w1_refusals…`, U4's registered G-C test and N9's text guard |
| StackReservation cause lost | `u3g2_…no_w1_refusals…` and N9's text guard |
| Coexistence not checked before G-C | `u3g2_…no_w1_refusals…` (G-C consulted on n05) and the structural gate-order test |
| G-B refusal not recorded (`retained_product.rs`) | **only** `u3g2_late_gate…` |

## 4. B-1 / S-7 and B′

- **B-1 / S-7: exactly one ordinary run per invocation on the permitted path.** The `lib.rs` hook counts the ordinary runs on both threads.
  - The success, every W1 fallback, the G-B, G-C and coexistence refusals, and the stack fallback each show 1 run.
  - G-C is consulted exactly when the run reached it: 1 on success, on W1 fallbacks and on G-C refusals; 0 on G-B refusals, coexistence and the stack fallback.
  - U4's existing `ordinary_dispatch_entered` counter is unchanged and still counts the unpermitted dispatch.
- **B′ (RV82 N3):**
  - On the permit path, the envelope returned beside the successor is the plain run's bytes, in both modes.
  - On the no-permit path, there is one run and no copy.
  - The B′ comment in `retained_wire_tests.rs` now names this test. In the registered build, `u1_ordinary_bytes_unchanged_under_capture`'s Direct call is itself the permitted path, and still equals the plain bytes.

## 5. U5 on the committed facade output (deliverable 6)

**The pin.** `u5_compare.py` takes RV86's two-line extract pin, exactly the change in `u5_compare_extract_variant.diff` (`u5/u5_compare_pin.diff`):
- `EXTRACT_SHA256 = a66a8a49…`;
- the runtime assertion now checks the 7,240-byte extract;
- `RECORD_LOG_SHA256` (`5ced66b5…`) stays in the script and the report.

The full-log hash assertion is kept in the chain. RV86's `make_extract.py`, which asserts I50's full log at `5ced66b5…`, regenerates the committed extract byte for byte.

The script's hash moved from `37343f33…` to `df4684d3…`, and `R/I61/u5_reference_01/SHA256SUMS` has that one line updated (it verifies OK). Nothing else in that record changed.

**The input** is the successor bytes the actual Direct entry published in the registered build, both modes. The committed `u3g2_direct_entry_publishes_the_pinned_successor` wrote them with `I61_U3G2_OUT`: `ac6986b0…` and `6cd1d249…`, the same bytes U5 compared. The reader root is WT/f2a-memory's unchanged Python reader.

**The result.**
- **`u5_report.json` and `u5_run.log` are byte-identical to U5's,** with no stops.
- In each mode, all 97 published class claims pass against both oracle readouts: 25 relative, 69 absolute and 3 input-derived.
- The reader reports `invocation_bound`, not eligible, `needs_recompute`.

**Against RV86's stated limits (§4.4):**
- **Limit 4 is discharged.** The claim no longer rests only on the private-driver and stub bytes: the maintained facade under a real permit publishes the same bytes, and they pass unchanged.
- **Limits 1–3 and 5 stand as stated:**
  - the claim covers the published class claims only;
  - it does not cover the stop rule's sharper bound (7 `StopRuleSharper` rows still fail against the represented readout, as information);
  - it does not cover enclosure of the exact maximum by the extrema interval;
  - the classes and bounds rest on the reader's G5c.
- **Post-merge.** On the merged base, with U6's 07h reader at precommit, ROOT's follow-on reruns this with that reader.

## 6. Items not done

- **Deliverable 5 (D-U6-5) is deferred by ROOT** (scope note). U6's carrier fixtures are not on this branch, so no `include_str!` assertion was added. The transitive relation already holds: the live output's hashes equal U6's fixtures' pinned hashes. The direct assertion and the reruns come in ROOT's follow-on, after NUM, with U6, is merged into this branch.
- **Deliverable 7 (optional, RV85 U1) is not done.** Binding the permit to its invocation needs a field or lifetime on `CapturePermit`, which lives in `retained_memory.rs` (U4's fence). `lib.rs` alone cannot bind it without wrapping the permit in a second type that U4's API does not define. Linearity stays pinned by the existing text test and the single call site (N7's counts).

## Records (`_run_records/`)

All paths are placeholders (WT, R, PP), with no machine paths.
- **Candidate:** `candidate.diff` (against `0c7827b6ad`, including the new file), `candidate_status.txt`, `changed_files_sha256.txt`, `production_equivalence.py` and `.txt`.
- **Suites:** `suites/{reg_pp,stale_pp,reg_runner}.outcomes`, `suites/warn_{base,cand}_{build,test}.warnings`.
- **Sweeps:** `zz_i61_u3g2_sweep.rs` (scratch-only harness), `sweep/sweep_{cand_reg,cand_stale,unreg_b43}.tsv`, `compare_sweeps.py`, `sweep/compare.json`. Base `b54caba7ab`'s sweep is grant 1's `sweep_base.tsv`, with its hash in `outputs_sha256.txt`.
- **Mutants:** `mutants.py`, `mutants.json`.
- **U5:** `u5/run_u5_g2.sh`, `u5/u5_compare_pin.diff`, `u5/u5_report.json`, `u5/u5_run.log`.
- **Scripts:** `run_suites.sh`, `run_sweeps.sh`, `run_final.sh`, `sync_trees.sh`. Outputs: `outputs_sha256.txt`.

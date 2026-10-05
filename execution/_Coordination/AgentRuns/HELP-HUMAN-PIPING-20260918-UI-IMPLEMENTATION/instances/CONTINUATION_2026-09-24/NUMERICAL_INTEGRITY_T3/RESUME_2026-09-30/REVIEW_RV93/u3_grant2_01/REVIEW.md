# RV93: independent review of U3 grant 2 (the permitted path and the live milestone)

**Reviewer:** RV93, TASK (Type 2), dispatched directly by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path. No descendants. I did not write the code under review.

**Brief:** `R/BRIEFS/RV93_U3_GRANT2_REVIEW.md` (NUM `501837c05b`), read in full, with the repository's root AGENTS.md.

**Candidate:** `664f8df7b7` on `codex/piping-f2a-memory-20261004`, against base `0c7827b6ad` (the registration commit). Five files, +387 / −14:
- `PP/src/lib.rs`: two `#[cfg(test)]` statements placed on existing production lines (:2379, :3005), and eight lines inside the `#[cfg(test)]` module `retained_tests_hooks`;
- `PP/src/retained_product.rs`: one `#[cfg(test)]` statement on a production line (:3244, G-B);
- `PP/src/retained_tests_hooks/grant2.rs`: new, 95 lines, a child of that test-only module;
- `PP/src/retained_facade_tests.rs`: the five `u3g2_*` tests (pure addition);
- `PP/src/retained_wire_tests.rs`: a comment and one assertion's message.

**Basis read:** the full diff against `0c7827b6ad`; I61's brief, RETURN.md and `_run_records/`; the rulings the brief names (U3 grants 1–1d, R-1–R-3, RV85's findings, "RV86 on U5: PASS, with stated limits…", D-U6-5, the U4 G6 and registration rulings, RR:8436, ROUTING:98 as cited in RR).

**Copies and host.**
- Fresh `git archive` copies of `664f8df7b7` and `0c7827b6ad` in `WT/rv93/`, plus disposable derivatives: a probe copy, a mutant copy, two sweep copies, and two copies of G7 Pass A's tree `ba1faa1c…` (with and without the grant).
- Targets in `WT/targets/rv93/`; the Stale build in the separate `WT/targets/rv93_stale/`; logs in `WT/scratch/rv93_u3_grant2_01/`.
- Default toolchain (rustc 1.97.1 `8bab26f4f68e`, aarch64-apple-darwin); `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`; one cargo job at a time, each under a perl alarm; the memory guard (PID 5387) checked before every job.
- No Git writes or index operations (reads only, `GIT_OPTIONAL_LOCKS=0`), no installs, no new tooling, nothing in the system temp directory. WT/f2a-memory, the other reviewers' files and I65's files were not touched. I deleted my copies and targets afterwards.

**Oracles (my own; I61's tests were only run):**
- a scratch probe module with **process-global counters**, independent of I61's carried tally, at the ordinary run's first statement, at `solve_load_case_observed`, and inside U4's `check_late` / `check_complete`, recording the thread each was hit on;
- a **byte oracle** for the N1 notice: removing the notice's compact JSON (written out by hand) from the published bytes must give the value route's bytes exactly, with the notice as the last diagnostic;
- the pinned files compared **byte for byte** with NUM's U6 fixture copies, not only by hash;
- the candidate's own Rust and Python readers on the bytes I obtained;
- the actual Direct entry from a **non-test** build (a scratch example binary);
- a **52-input × 5-route × 2-mode sweep** (468 rows, with the full admission report and the private W1 result), in four builds: candidate and base, registered and Stale;
- a permit log compiled **unconditionally** into my probe copy of `admission()`, for the runner-workspace probe;
- I65's G7 TEXT chain and inventory tools, run read-only on Pass A's tree with and without the grant;
- I61's 11 mutants re-run, and 17 of my own.

## Verdict: **PASS**

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 1 |
| NOTE | 6 |

**The headline:**
- **The milestone publishes U1's pinned successor through the actual Direct entry in both modes.** The documents are byte-identical to the pinned files (`ac6986b0…` / `6cd1d249…`; `cmp` against NUM's U6 fixture copies), from exactly one ordinary run, with the ordinary envelope beside them equal to the value route's bytes. A **non-test** build publishes the same bytes.
- **Readers:** Rust **PASS** and Python **PASS** in both modes (98 / 99 classes, invocation-bound, not eligible, needs_recompute). TypeScript was **not executed**, because the candidate carries no WASM; its verdict transfers by byte identity (N-1).
- **U5:** the pinned script on my bytes reproduces U5's report and log **byte for byte**. RV86's limit 4 is discharged; limits 1–3 and 5 stand as stated.
- **Every fallback class** (Preparation, Native, Candidate, Staging, Serializer, Precommit), built by me from hooks and, for Preparation and Candidate, also from **real inputs**, publishes the value route's bytes plus exactly one N1 notice. **Every no-W1 refusal** (G-A, G-B, G-C including `OrdinarySolveNotAttempted`, the stack, coexistence; Domain is unreachable with a permit) publishes exactly the value route's bytes.
- **B-1 / S-7 hold by my own counters:** one ordinary run and one solve per invocation in every case. The hooks fire on the reserved-stack thread.
- **SV18 is killed** by a committed permitted-path test, on two independent assertions, with no compile error.
- **Outside the permitted entry nothing moved.** My sweep is byte-identical to base on all 468 rows, both registered and Stale. Runner/headless is outcome-identical to base (85 / 2), and **no runner test is granted a permit** (0, against positive controls of 42 and 2). Production text is unchanged once the `cfg(test)` statements are removed, and the production build is warning-identical.
- **Mutants:** **28 of 28 killed**, none by a compile error. 9 of them are killed only by the new `u3g2_*` tests.
- **S-1:** U4's D1 inventory tools read the test-only `grant2.rs` as production source. No TEXT value moves, but G7 Pass B will report one added static and two more reachable functions. A rename fixes it.

## Findings

| # | Sev | Where (`664f8df7b7`) | Evidence | Remedy |
|---|---|---|---|---|
| S-1 | SHOULD-FIX | `PP/src/retained_tests_hooks/grant2.rs` (whole file; `:20` `thread_local!`, `:23` `tally`, `:39` `TallyCarry::take`) | **The file is test-only by placement, but its name does not say so.** It is compiled only inside `#[cfg(test)] mod retained_tests_hooks`, yet the file name contains neither `test` nor `_tests.rs`. So all four of U4's D1 inventory tools read it as **production** source: `callgraph_g5.py` (filter `"test" not in f`), `template_inventory` (via g7_pass's `_tests.rs` filter), `statics_list.py` and `g7_linemap.py`. **Measured on Pass A's tree `ba1faa1c…` plus the grant** (`evidence/text/text_check.txt`): every output Pass B compares is identical (TAV 2,150,800,830; 2,814 rows, 0 changed; audit complete; non-candidates "carried"). But **`reachable_fns` rises from 2,770 to 2,772**: `.take(` fan-out reaches `TallyCarry::take` and `tally`. **`statics_list` reports one added production static** (grant2.rs `thread_local!`). And the line map lists grant2.rs as a changed production file. Pass B will show these as deltas, and under QUALIFICATION.md §11 / RR "RV87 confirms…" they read as a change on the D1 call graph. The production compiled code is unchanged (§5). | In I61's D-U6-5 follow-on, **before Pass B:** rename the file so the tools' existing test filters exclude it, e.g. `retained_tests_hooks/grant2_tests.rs` with `mod grant2_tests;`. All edits stay inside the `cfg(test)` module, so the change remains line-neutral. Then re-run `statics_list` and the TEXT chain to confirm `reachable_fns` returns to 2,770. **Or** ROOT records, for Pass B, that these three deltas are test-only. |
| N-1 | NOTE | Review item 1, the TypeScript lane | **The TS reader was not executed.** The brief allows prebuilt WASM only if the candidate carries it, and it does not (`apps/desktop/public/` is untracked). **Verdict by identity:** the bytes equal U1's files exactly (§1). Every TS reader input (apps/desktop, analysis_runs, result_export, schemas, fixtures, serialization, model_operations sources) is byte-identical between `b54caba7ab` and the candidate; `git diff` shows only two R-3 `Cargo.lock` lines. RV82 recorded **TS PASS** (needs_recompute, not eligible, bound, 98 / 99) on exactly these bytes against that reader (`R/REVIEW_RV82/u1_serializer_02/evidence/ts_results.txt`). | **ROOT to decide** whether the identity transfer suffices, or whether the D-U6-5 follow-on runs TS live. On the merged base, U6's Vitest already exercises its fixtures, which are byte copies. |
| N-2 | NOTE | lib.rs:2379, :3005; retained_product.rs:3244 | **Line-neutral style.** Removing each `#[cfg(test)] …;` statement restores the base line exactly, apart from the one space before the fragment (`evidence/prod_text/`). Rust applies a statement attribute to that one statement only, and the non-test example run (§3) confirms that the production `if let … observer.invocation(…)` on :2379 survives a non-test build. **Readability cost:** at :2379 the hook leads the line, so a reader may take the `cfg` as covering the production call after it. Nothing in this repository runs rustfmt on PP, so nothing will reflow these lines silently; a future reflow would move lines and re-trigger U4's line-keyed inputs. | Optional: a short `// cfg(test) hook only; the rest of the line is production` marker, or a grouped hook macro. No action needed for correctness. |
| N-3 | NOTE | retained_memory.rs:2822–2849 (U4's file); lib.rs:2287–2288 | **RV85 U1, optional item 7, not done (outside I61's fence). The residual risk is not real in the registered build as it stands** (§6). The permit is crate-private, linear, built once in `admission`, and handed in the same match arm to the one `permitted_dispatch` call with the request and capture it admitted. It moves into the observer, which `retained_w1` consumes. No output, static or thread-local holds it. | Wider F2a, U4: bind it structurally if a second call site is ever added. The N7 call-site guards flag one today. |
| N-4 | NOTE | retained_facade_tests.rs:533–537 (`REGISTERED_IDENTITY`, `registered()`) | **Only the registered host exercises the permitted path.** In any other build (hosted CI is Stale) the five tests assert the ordinary route, which is by design and passes (5/5 Stale). The registered identity is duplicated as a test constant. If the profile is re-registered without updating the constant, the failure is **loud, not silent**: `direct()` asserts the report's status. | Optional: read the identity from `REGISTERED_PROFILES[0]` (as my probe reads it from source). Keep the registered-host run as the only acceptance evidence for these tests. |
| N-5 | NOTE | RV93 variants (sweep and `zz_rv93_input_fallbacks`) | **W1 fallbacks from real inputs.** The milestone with only its first load (one RX moment; in D1) falls back at **Candidate** with exactly one notice, in both modes. The RX spring at 1e-300 falls back at **Preparation**. Both are byte-identical to base, and both fit ROUTING:98, since W1 work ran. The committed Candidate test relies on a hook alone. | Optional, for U7/U9's corpus: a committed real-input Candidate fallback (this variant) beside the hooked one. |
| N-6 | NOTE | `R/I61/u5_reference_01/_run_records/u5_compare.py:23, :27, :55` | **U5's report records a hash the script no longer checks.** The pin is exactly RV86's two-line variant: the script now asserts the 7,240-byte extract, while `u5_report.json` still records `i50_record_log_sha256: 5ced66b5…`. That keeps the report byte-identical to U5's, as RV86 designed. The full-log assertion now lives only in `make_extract.py`, which I61's and my run scripts execute first; I re-ran it, and it regenerated the extract byte for byte. | None now. The documented entry is the run script (make_extract, then compare). At the next non-byte-identical rerun, consider adding the extract hash to the report. |

There are no BLOCKING findings and no surviving mutants.

## 1. The milestone through the actual Direct entry, both modes

**The registered build.** My default `cargo test` build's build-script output equals the one `REGISTERED_PROFILES` entry's strings byte for byte, read from `retained_memory.rs`'s source (`evidence/identity_check.txt`):
- `OPS_RETAINED_BUILD_IDENTITY`: 363 B;
- `OPS_RETAINED_REVIEWED_INPUTS`: 1,740 B.

The test file's `REGISTERED_IDENTITY` constant also equals it. My Stale build's identity carries `rustflags=--cfg%3Drv93_stale`.

**The run** (`evidence/probe/zz_rv93.rs::zz_rv93_milestone_direct`): `run_linear_static_preview_value_with_retained_direct(RF-SKEW-T-CANT-OFF-122-r1e-04, mode)`, with no fault armed and no tally installed.

| | sparse_interactive | dense_scrutiny |
|---|---|---|
| Admission | Registered, no refusal | Registered, no refusal |
| `into_publication()` | `Successor` | `Successor` |
| Published value (compact) | 113,733 B, `1d9ba709…` | 114,894 B, `7c5fe5c5…` |
| U1 document (`{id, invocation, source}`, pretty) | `ac6986b0…59dc` | `6cd1d249…c9b5` |
| Byte compare with NUM's U6 fixture copy | **identical** (`cmp`) | **identical** (`cmp`) |
| `receipt_sha256` | `efc1a39b…7494` | `3e26499f…ac4a` |
| `envelope()` beside the successor (B′) | = the value route's bytes | = the value route's bytes |
| My counters: runs / solves / G-B / G-C | 1 / 1 / 1 / 1 | 1 / 1 / 1 / 1 |
| Thread of the run and of G-C | the reserved-stack worker (≠ caller) | the worker |
| **Non-test build** (example binary), candidate and base | Successor, `ac6986b0…`, value `1d9ba709…`, envelope `9c7ec1a1…` (= U1's protected ordinary pin) | Successor, `6cd1d249…`, value `7c5fe5c5…`, envelope `21ca629c…` (= the pin) |

**The three F2a readers, as the candidate carries them** (`evidence/readers/readers.txt`):

| Reader | sparse_interactive | dense_scrutiny |
|---|---|---|
| **Rust**: `result_export::retained_precision::validate`, in-process, with the actual invocation | **PASS**: invocation-bound, not eligible, 98 classes, publication `26ef4435…` | **PASS**: 99 classes, publication `487c4cf2…` |
| **Python**: `_validate_draft`, the candidate's `core/analysis_runs/retained_precision.py`, with I52's prebuilt checked-JSON and units CLIs; jsonschema 4.26.0 | **PASS** G0–G8: needs_recompute, not eligible, bound; 98 rows (69 abs, 25 rel, 3 input-derived, 1 non-quantity); 0 schema violations | **PASS**: 99 rows (2 non-quantity); 0 violations |
| Python public entry | Refuses G0 `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`. That is by design on this branch: `_IMPLEMENTATION_COMPLETE = False`, and D-U6-1 belongs to U6. | same |
| Python negative control (one receipt-hash nibble flipped) | refused G1 `RECEIPT_MISMATCH` | refused G1 |
| **TypeScript** | **Not executed** (N-1). The verdict transfers by identity from RV82's TS PASS on exactly these bytes and this reader. | same |

**U5 on these bytes** (`evidence/u5/`):
- **The pin.** `u5_compare.py` (`df4684d3…` in NUM) differs from its pre-pin version (`37343f33…`, from Git history) by exactly RV86's two-line variant.
- **The full-log assertion stays in the chain.** I re-ran RV86's `make_extract.py`, which asserts I50's full log at `5ced66b5…`. It regenerated the committed 7,240-byte extract (`a66a8a49…`) byte for byte.
- **The run.** I ran the pinned script unchanged on my two documents, with my candidate copy as the reader root. **`u5_report.json` and `u5_run.log` are byte-identical to U5's committed ones**, with `STOPS []`.
- **Results, per mode:**
  - 97 class claims checked: 25 relative within 1e-9 on both readouts, 69 absolute within bound, 3 input-derived;
  - the observable checks pass, and both negative controls move to fail;
  - the reader reports needs_recompute, not eligible, bound.
- **Against RV86's stated limits:**
  - **Limit 4 is discharged:** the bytes are now the maintained facade's, under a real permit.
  - **The others stand exactly as stated:**
    - published class claims only;
    - no claim on the stop-rule-sharp bound (7 `represented:info:StopRuleSharper:fail` rows per mode, information only, unchanged);
    - no enclosure claim for the extrema intervals;
    - the classes and bounds rest on the reader's G5c.

## 2. Refusal and coexistence controls on the permitted path

Every row is my own construction: a hook armed by my test on the caller's thread, or an input. Each is checked with my counters and my byte oracle (`evidence/probe/`), in both modes.

**W1 fallbacks: the value route's bytes plus exactly one N1 notice, nothing else changed.**

| Class | How I built it | Cause recorded | Runs / solves / G-B / G-C |
|---|---|---|---|
| Preparation | hook `fail_next_preparation` | `Preparation` | 1 / 1 / 1 / 1 |
| Preparation | **input:** `rejected_stress_range` (each file in its own mode) | `Preparation` | 1 / 1 / 0 / 1 |
| Preparation | **input:** an RV93 variant with the RX spring at 1e-300 | `Preparation` | 1 / 1 / 0 / 1 |
| Native | hook `withdraw_next_native_source` | `Native` | 1 / 1 / 1 / 1 |
| Candidate | hook `fault_next_candidate(Maxima)` | `Candidate` | 1 / 1 / 1 / 1 |
| Candidate | **input:** an RV93 variant, the milestone with only its first load | `Candidate` | 1 / 1 / 1 / 1 |
| Staging | hook `break_next_staging` | `Staging("pipe_stress_extrema[]")` | 1 / 1 / 1 / 1 |
| Serializer | hooks: `WorkCounterRange`, `PublicationHashRange`, `SaturationNotExcluded` | `Serializer(..)`; the notice carries C1:68's reason and the exact detail token | 1 / 1 / 1 / 1 |
| Serializer | hook: `Encoding` | `Serializer(Encoding)`; the plain notice text (F-1) | 1 / 1 / 1 / 1 |
| Precommit | hooks: `corrupt_next_precommit` (G1), `rebind_next_precommit_invocation` (G8) | `Precommit{G1 RECEIPT_MISMATCH}` / `{G8 INVOCATION_MISMATCH}` | 1 / 1 / 1 / 1 |

In every hooked row three things hold:
- the armed fault is gone from the caller afterwards (`armed_names()` empty);
- G-C ran on the worker thread;
- the cause is one the milestone reaches only if the fault fired.

So **the hooks fire on the reserved-stack thread** (RV85 item 5).

**Refusals before W1 work: exactly the value route's bytes, no notice.**

| Refusal | How I built it | Cause | Runs / solves / G-B / G-C |
|---|---|---|---|
| G-A, D1.3 | schema 0.3.0 | none (report clause D1.3) | 1 / 0 / 0 / 0 |
| G-A, D1.4 | a combination | none (D1.4) | 1 / 0 / 0 / 0 |
| G-A, D1.6 | support family `hanger_rigid` | none (D1.6) | 1 / 0 / 0 / 0 |
| G-A, D1.0 | the Headless entry | none (D1.0) | 1 / 1 / 0 / 0 |
| G-B | hook `fail_next_late_gate`: the actual `check_late` refuses `LateObservationBytes` | `LateGate` | 1 / 1 / 1 / **0** |
| G-C, unattempted solve | inputs: an invalid load category; no supports | `CompleteGate(OrdinarySolveNotAttempted, observed 1, cap 0)` | 1 / 1 / 0 / 1 and 1 / 0 / 0 / 1 |
| G-C, bound | hook `fail_next_complete_gate` (`ObservationBytes`) | `CompleteGate` | 1 / 1 / 1 / 1 |
| Stack | `RESERVED_STACK_OVERRIDE` = 2^62, so the spawn fails | `StackReservation`; the run is on the **caller** thread | 1 / 1 / 0 / 0 |
| Coexistence | inputs: `source_blocks/n06-sparse_interactive`, `numerical_sensitive_torsion_model` | `Coexistence` | 1 / 1 / 0 / **0** |
| Domain | unreachable with a permit (below); the domain families refuse at G-A | — | — |

**Domain.** No permitted request can reach either `Domain` branch:
- **`permitted_run`'s** needs `is_load_state` (schema 0.4.0) or `is_exact` (0.3.0/0.4.0 with an exact pressure contract). D1.3 admits only 0.1.0 and 0.2.0, and refuses any pressure contract.
- **`retained_w1`'s** needs a raw custody with ≠ 1 case or a combination. D1.4 refuses those on the same parse.

In my sweep, 34 Direct rows refuse at D1.3 (schema version) with exact bytes. They are 17 inputs in both modes: every load_reference, load_reference_source and physics_source request (0.4.0 / 0.3.0), `exact_pressure_authoring_model`, `physics_thermal_ui_model`, and my 0.3.0 variant. I agree with I61's reading.

**RV85 item 5, hook propagation, by mutants** (§7). Each of these is killed by the committed `u3g2_*` tests, with no compile error:
- the work not wrapped by `carry_test_hooks` (H1);
- the tally not installed on the worker (H2);
- the G-B fault seam made inert (H3).

No fault test can pass vacuously.

## 3. One ordinary run (B-1 / S-7), and B′

**The permitted path.** My global counters show exactly **one** entry to `run_linear_static_preview_observed` and **one** `solve_load_case_observed`, for the success and for every fallback and refusal in §2. The stack fallback's run is on the caller; every other run is on the worker. G-C is consulted exactly when the run reaches it:
- **once** on success, on W1 fallbacks and on G-C refusals;
- **never** after a G-B refusal, coexistence or the stack fallback.

This agrees with I61's tally in every case, by an independent mechanism.

**Every ordinary run passes the hook.** The only callers of `run_linear_static_preview_observed` are:
- `run_linear_static_preview_captured_once` (lib.rs:2369);
- `permitted_run` (:2993);
- the private driver's `prepare_observed` (retained_product.rs:3443, tests only).

W1's native solve does not use `solve_load_case_observed`: solves stayed at 1.

**The no-permit path:**
- For the two-case variant, the shared value route and the refused Direct entry each show one run and no G-B or G-C; the refused Headless entry shows one run with its solve. The two-case variant exits at validation in both I61's test and mine, since its load ids repeat across cases; the Headless and sweep rows cover a no-permit run that solves.
- `ordinary_dispatch`'s text is byte-identical to base (§5), so it adds no run and no copy relative to base.
- I61's text guard pins that going forward. My D1 mutant shows that an extra run spelled without the guard's words is still killed, by the counted tests.

**The dispatch-count hook cannot change a published byte, and production never reaches it.**
- **How it is gated.** All three seams are `#[cfg(test)]` statement attributes, removed before type-checking in any non-test build. `grant2.rs` is declared only inside the `#[cfg(test)]` module `retained_tests_hooks`. So none of it exists in the library that runner/headless, the desktop app or any other dependent crate links: cfg(test) applies only to PP's own unit-test binary.
- **Inert when unarmed.** In the test binary, `ordinary_run_entered` and the G-C count only increment an `Arc<Tally>` when one is installed, and the fault seams act only when armed.
- **Evidence:**
  - my four sweeps are byte-identical between base and candidate in test builds, where the seams are compiled in;
  - the production `--lib` build is warning-identical (13 warning lines);
  - the non-test example publishes the pinned bytes, in both candidate and base;
  - runner/headless is outcome-identical to base.

## 4. SV18 (RV85 U4 / N2)

**SV18 is killed.** I re-ran I61's `SV18_late_check_removed`, which removes `permitted_run`'s G-B check. It is killed by `u3g2_late_gate_refusal_is_final_and_g_c_is_not_consulted`, and also by the structural gate-order test, with no compile error. The test catches it on two independent assertions:
- the cause becomes `CompleteGate`, because the same raised capacity record also exceeds G-C's bound;
- G-C is consulted once instead of zero times.

**SV18b** (the check's text kept, the branch never taken) is killed **only** by that test.

**The test is not vacuous.** My H3 makes the G-B seam inert, so the milestone publishes a successor, and that test alone kills it.

## 5. Controls outside the permitted entry

**Production text.** My own check (`evidence/prod_text/`):
- both files keep their line counts (24,334; 3,909);
- the only changed production lines are lib.rs:2379, :3005 and retained_product.rs:3244. Removing the `#[cfg(test)] …;` statement from each gives the base line exactly, apart from the one space that preceded the fragment;
- every other changed lib.rs line (:3184, :3214, :3216, :3219, :3232, :3247, :3255, :3296) lies inside the `#[cfg(test)]` module, whose extent I took by brace matching.

**The registered sweep** (`evidence/sweep/`): 36 request-shaped fixtures plus 16 RV93 milestone variants, so 52 inputs. Routes: typed default, typed mode, value, retained Headless and Direct, in both modes, giving 468 rows with the full admission report and the private W1 result.
- **Candidate = base on all 468 rows.** The two TSVs are byte-identical (`7955b640…`).
- Base `0c7827b6ad` already carries the registered permitted path, so this includes Direct on in-D1 inputs.
- **Direct publications, registered:**
  - 10 successors: the milestone, schema 0.2.0, loads ×2, a renamed case, and the milestone again as a variant, each in both modes;
  - 8 notices: `rejected_stress_range` ×2 files ×2 modes, the 1e-300 spring, the single-moment variant. My byte oracle checked 6 of these directly (§2); the other 2 are identical to base;
  - 84 exact;
  - 2 errors, equal to the value route's.
- Every typed, value and Headless row is unchanged, and all 204 non-error retained reports read `Registered`.

**The Stale build** (`RUSTFLAGS=--cfg=rv93_stale`, in its own target dir):
- **Candidate = base on all 468 rows** (`d51c84d1…`).
- All 204 non-error retained reports read `Stale`. Every Direct and Headless publication is `Ordinary`, equal to the value route's bytes, with no W1 result.
- PP, all targets: **704 passed, 1 failed (the Mac t13), 10 ignored**, outcome-identical to my registered run. The five `u3g2_*` tests pass via their ordinary-route branch.

**The registered suites:**
- PP, all targets: **704 passed, 1 failed (the Mac t13), 10 ignored**, with the five `u3g2_*` tests passing. This equals ROOT's run.
- runner/headless: **85 passed, 2 failed** (the two `load_reference` tests). The per-test outcomes are identical to base `0c7827b6ad`'s.

**The runner workspace** (my probe copy only). I compiled a permit log **unconditionally** into `admission()`'s permit arm (`evidence/probe/probe_instrumentation.diff`).
- **Positive controls:** my PP probe tests logged 42 permits, and the non-test example logged 2.
- **The whole runner/headless suite against the instrumented PP logged 0 permits**, with outcomes identical to the clean candidate's.

So no runner test reaches W1, as RR "RV89 passes the pre-registration delta…" requires.

**The TEXT and identifier chain (an integration check).** The grant edits three production lines that U4's line-keyed inputs see. So I ran I65's G7 tools read-only on Pass A's tree `ba1faa1c…` and on the same tree plus the grant's five files (`evidence/text/`):
- **The line map carries every rule.** `g7_linemap.py` from 0c7827b6ad to 664f8df7b7 reports **0 remapped and 0 unmapped**: no rule's line is in an edited hunk, so Pass B will not stop at exit 4.
- **The TEXT chain is unchanged.** I ran `run_text_part2.sh`, the enforced identifier audit and the §11 non-candidate sweep. Every output I65's Pass B compares is **identical**: the summary, composite_text, g4_caps, ordinary_caps, producer_caps, profile_tree, t07, t08 and t25. TAV is 2,150,800,830, with 2,814 rows and 0 changed; the audit is complete; the non-candidates are "carried".
- The cfg(test) blanking handles the one-line fragments correctly.
- **But all four tools read `grant2.rs` as production source (S-1):** `reachable_fns` rises from 2,770 to 2,772, and one `thread_local!` static is added.

## 6. RV85 U1 (optional item 7, not done)

I61 did not bind the permit to its invocation. `CapturePermit` is U4's (`retained_memory.rs`), outside I61's fence.

**The remaining risk is not real in the registered build as it stands:**
- **The type is locked down.** It is `pub(super)` (crate-private), neither `Clone` nor `Copy`, with **one** constructor (`admission`, retained_memory.rs:2849), reached only through `admit`.
- **The permit cannot be redirected.** The dispatch's one `admit` call hands it, in the same match arm, to the one `permitted_dispatch` call, with the **same** `request` and `capture` it admitted (lib.rs:2287–2288). I61's N7 guard pins both call sites.
- **No permit outlives its invocation.** The permit moves into the observer (`permitted_probe`), then into `PreparedCase` / `FrozenCandidate`, all of which are consumed inside `retained_w1`. `RetainedPreviewOutput` holds only the envelope, the `Copy` report, and either a `Value` or a fallback cause. No permit is stored in a static or thread-local.
- **No public API** returns, accepts or names a permit.

Reuse would need a new in-crate call site, which the existing structural tests would flag. Binding it structurally remains a U4 item for wider F2a (N-3).

## 7. Mutants

`evidence/mutants/`: PP `--lib` in the registered build, one edit each in my mutant copy, with the pristine file restored and its hash checked after every run. The Mac t13 is excluded.

**I61's 11, re-run: all killed, none by a compile error**, matching I61's record:
- **killed only by `u3g2_*` tests:** SV18b, B-1 permitted second run, B′ no-permit second run, B′ custody copy, G-B refusal not recorded;
- **SV18:** killed by the late-gate test and the gate-order test.

**My 17: all killed, none by a compile error.**

| Mutant (brief's class) | Edit | Killed by |
|---|---|---|
| A1 (successor-or-ordinary choice) | `into_publication` always returns `Ordinary` | `u3g2_…publishes_the_pinned_successor`, `u3_r1_…` |
| A2 (same) | `successor()` hides a real successor | `u3g2_…publishes…`, `u3_r1_…`, U4's registered G-C test |
| B1 (N1 notice count) | the notice pushed twice | the product's own capacity assertion, in `u3g2_…w1_fallbacks…` and others |
| **B1b** (same) | **two notices with no allocation**: the last ordinary diagnostic is overwritten by a copy, then the notice is appended | `u3g2_…w1_fallbacks…` ("exactly one N1 notice": 2), `u3_each_stage…`, `u3_r1_…` |
| B2 (same) | no notice on a serializer fallback | `u3g2_…w1_fallbacks…`, `u3_each_stage…` |
| C1 (the untouched ordinary envelope) | the Native fallback's owner loses a diagnostic, capacity unchanged | `u3g2_…w1_fallbacks…`, `u3_each_stage…` |
| C2 (same) | a G-B refusal's ordinary owner gains a diagnostic | **only** `u3g2_late_gate…` |
| D1 (dispatch count) | an extra no-permit ordinary run, spelled without `.clone()`, `to_owned()` or `to_vec()` | **only** `u3g2_…no_w1_refusals…` and `u3g2_no_permit…` (runs = 2) |
| D2 (same) | an extra ordinary run after the permitted work, on the caller | four `u3g2_*` tests, `u3_n9_…` |
| E1 (G-C unattempted decline) | `OrdinarySolveNotAttempted` never observed (U4's file) | `u3g2_…no_w1_refusals…`, both U4 G-C tests |
| E2 (same) | the decline published with a notice | `u3g2_…no_w1_refusals…`, U4's registered G-C test |
| H1 (RV85 item 5) | work not wrapped by `carry_test_hooks` | four `u3g2_*` tests, `u3_test_hooks_follow…`, `u3_unfired_hooks…` |
| H2 (same) | the tally not installed on the worker | **only** the four permitted-path `u3g2_*` tests |
| H3 (same) | the G-B fault seam inert | **only** `u3g2_late_gate…` |
| S1 | the stack fallback's cause → `Coexistence` | `u3g2_…no_w1_refusals…`, `u3_n9_…` |
| R1 | C1:68's detail dropped from the notice | `u3g2_…w1_fallbacks…`, `u3_each_stage…`, `u3_r2_…` |
| N1 | the notice first instead of last | `u3g2_…w1_fallbacks…`, `u3_each_stage…`, `u3_r1_…` |

**Totals: 28 of 28 killed, 0 survivors, 0 compile-error kills.** Nine (I61's five above, plus C2, D1, H2 and H3) are killed only by the new tests.

## 8. Nothing weakened

- **Only five PP files changed.** `git diff 0c7827b6ad 664f8df7b7` touches only the five PP files above: **no reader, schema, fixture or carrier changed**, and `retained_memory.rs` is unchanged.
- **No existing check was relaxed.** `retained_facade_tests.rs` is pure addition (0 removed lines). In `retained_wire_tests.rs`, only a comment and one assertion's message string changed. The assertion itself (`direct.envelope()` bytes = the plain bytes) is unchanged, and in the registered build it now exercises the permitted path.
- **No test permit.** The production text has one `CapturePermit {` construction (`admission`). Every permit in the new tests comes from `admit`, through the actual Direct entry. The only other `admit` caller is U4's pre-existing law test, which drops its permit.
- **D-U6-5 is deferred by ROOT, not missing.** That is the `include_str!` comparison with U6's carrier fixtures. I compared the live bytes with NUM's fixture copies directly (§1): they are identical.

## For ROOT

1. **S-1:** decide between the rename in the D-U6-5 follow-on, before Pass B, and recording the three test-only deltas (one static, two reachable functions, one changed file) as Pass B's disposition.
2. **N-1:** decide whether TS by identity suffices for grant 2, or whether the follow-on runs TS live on the merged base.
3. **Offered for the follow-on:** I can extend this review to the D-U6-5 delta. My oracles (probe, sweep, mutant harness) are kept as evidence and can be re-applied.

## Evidence (`evidence/`; placeholder paths only)

- `identity_check.txt`: the build-script identity and inputs against the registered entry, and the Stale identity.
- `probe/`: my probe and sweep modules, the non-test example, the instrumentation diff (counters and the permit log), and every probe output line.
- `readers/`: the Rust and Python verdicts, the byte compares, and the hashes of the documents I obtained.
- `u5/`: the run script, the regenerated extract's hash, `u5_report.json` and `u5_run.log`.
- `sweep/`: `cand_reg.tsv` and `cand_stale.tsv` (base's are byte-identical; all four hashes are in `sweep_sha256.txt`), `compare_sweeps.py`, `compare.json` and `run_sweeps.sh`.
- `prod_text/`: the production-text check and its output.
- `text/`: the TEXT, line-map and statics check against Pass A.
- `mutants/`: the harness, results JSON and summary.
- `suites/`: PP registered and Stale outcomes, runner/headless outcomes (candidate, base, probe), the permit positive controls, the build warnings and `SUMMARY.txt`.
- `scripts/`: the cargo wrappers used.

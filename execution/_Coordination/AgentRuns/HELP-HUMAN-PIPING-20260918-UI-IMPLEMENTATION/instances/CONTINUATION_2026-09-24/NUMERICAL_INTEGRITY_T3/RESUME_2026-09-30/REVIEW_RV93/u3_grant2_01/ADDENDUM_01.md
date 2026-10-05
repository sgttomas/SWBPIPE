# RV93 Addendum 01: grant 2 to the final memory head

**Reviewer:** RV93, TASK (Type 2), resumed by ROOT with the same context, harness and oracles as `REVIEW.md` (sha256 `919a0cc5…9c12`). No descendants.

**ROOT's rulings on REVIEW.md are recorded and not re-opened here:**
- S-1 is recorded as test-only, with no rename.
- N-1 is accepted for the merge.
- N-3, N-4 and N-6 are noted; N-2 is kept as a readability note; N-5 is routed to U7's slice L.

**Candidate:** the memory branch head `7f07a2f7b413b37ecaa879f81b3d759a9cde7f13`, against `664f8df7b7`. The first-parent delta is three commits:
1. **`f8ce1eb32b`**, the merge of NUM `0198176dc9`. It brings U6 in, so the 07h Rust reader (F5's exact `diagnostic_refs` list) is now PP's precommit reader.
2. **`f71478696b`**, D-U6-5: one test in `retained_facade_tests.rs` (+31 lines).
3. **`7f07a2f7b4`**, the T17_V4 coefficient line in `retained_memory.rs`: census-20, from 57,880 to 83,625.

I read the PP part of the delta in full. `git diff 664f8df7b7 7f07a2f7b4 -- core/product_physics` touches only those two files: `lib.rs`, `retained_product.rs` and `grant2.rs` are unchanged. I also read the 07h reader's F5 hunk and I61's follow-on records (`R/I61/u3_grant2_02/`).

**Host:** the same rules as REVIEW.md.
- **Copies:** a fresh `git archive` copy of `7f07a2f7b4` in `WT/rv93/head/`, with derivatives `headprobe`, `headsweep` and `headmut`.
- **Targets:** `WT/targets/rv93/*`, plus `WT/targets/rv93_stale/*` for the Stale build.
- **Logs:** `WT/scratch/rv93_u3_grant2_01/ext/`.
- **Cargo:** default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one cargo job at a time.
- **Memory guard:** PID 5387, checked before every job.
- **Never done:** Git writes, installs, or anything in the system temp directory.
- **Cleanup:** I deleted the copies and targets afterwards.

## Verdict: **PASS**

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| NOTE | 0 |

Every check ROOT asked for holds on the final head, and nothing in PP changed behaviour.

## 1. The milestone under the 07h reader at precommit

**The build is still the registered one.** In my head build, the build script's identity and reviewed inputs equal the registered entry (`identity_check_head.txt`). The merge changed no reviewed input; the Stale target still reads `rustflags=--cfg%3Drv93_stale`.

**My probe, unchanged apart from two added tests** (`zz_rv93.rs`, `probe_output.txt`), shows that in both modes the actual Direct entry publishes a `Successor`, not a Precommit fallback:
- **Hashes:** the U1 documents are `ac6986b0…59dc` and `6cd1d249…c9b5`, and the receipts are `efc1a39b…` and `3e26499f…`.
- **Counts:** 1 run, 1 solve, 1 G-B and 1 G-C, on the worker thread.
- **Byte comparisons, both `cmp`-identical:**
  - with the head's own U6 carrier fixtures (`fixtures/results/retained_precision_milestone_successor_{sparse_interactive,dense_scrutiny}.json`), and in-test with `include_str!` (`zz_rv93_milestone_equals_u6_carriers`);
  - with the bytes I obtained on grant 2.
- **A non-test build** (my example binary) of the head publishes the same successor documents. The envelope beside each is U1's protected ordinary pin (`9c7ec1a1…`, `21ca629c…`).

**Readers on these bytes, as the head carries them:**

| Reader | Verdict |
|---|---|
| Rust (07h) | **PASS**: 98 and 99 classes, invocation-bound, not eligible |
| Python, head copy | **PASS**: needs_recompute, not eligible, 0 schema violations. The public entry now admits as well, as D-U6-1 makes it (`_IMPLEMENTATION_COMPLETE` still False, so not eligible). The negative control is refused at G1 |
| U5 (the pinned `u5_compare.py`, with the head's Python reader as root) | `u5_report.json` and `u5_run.log` are **byte-identical to U5's** |

## 2. My 468-row sweep, registered and Stale

On the head with my sweep module (52 inputs × 5 routes × 2 modes, with the full admission report and the private W1 result):

| Build | sha256 | Same as grant 2's (and so base `0c7827b6ad`'s) |
|---|---|---|
| Registered | `7955b640…` | **byte-identical** |
| Stale | `d51c84d1…` | **byte-identical** |

The admission report's law record carries `required`, so this also shows that **the T17_V4 line changes no admission verdict, bound or published byte** for any of the 52 inputs.

## 3. The fallback and refusal classes, and an F5-triggered Precommit fallback

**Every grant-2 row reproduces exactly on the head**, including the published hashes:
- the 22 fallback lines: every class, each built from a hook or an input;
- the 22 refusal lines: G-A, G-B, G-C including `OrdinarySolveNotAttempted`, the stack, and coexistence;
- the input fallbacks: the 1e-300 spring gives Preparation, and the single moment gives Candidate;
- the no-permit counts.

All rows match grant 2's probe output line for line (`probe_output.txt`).

**F5 at precommit (new, `zz_rv93_f5_precommit_fallback`).** A probe-only `#[cfg(test)]` seam in my copy edits the live successor just before precommit, then recomputes `receipt_sha256` with the serializer's own `domain_hash`, so G1 passes. Two edits:
1. swap the first two of the ordinary attempt's four `diagnostic_refs`;
2. drop the last one.

Both keep the refs unique and resolving, which is all that checkpoint A's D6a required before 07h.

| | sparse_interactive | dense_scrutiny |
|---|---|---|
| **Reader alone** on the U6 carrier: untampered / edit 1 / edit 2 | PASS / refused G5 `RETAINED_PRECISION_ATTEMPT_MISMATCH` / refused G5 | same |
| **Actual Direct entry**, edit 1 and edit 2 | `Precommit{G5, ATTEMPT_MISMATCH}`, no successor; **the value route's bytes plus exactly one N1 notice** (my byte oracle); counts 1 / 1 / 1 / 1 | same |

**Control** (`f5_removed_control.txt`): with only F5's exact-list line removed from my copy of the reader, edit 1 is **admitted**. So F5 alone triggers that fallback. Edit 2 may also meet the typed-reference check; edit 1 is the pure F5 case. I then restored the reader and checked it with `cmp`.

## 4. D-U6-5's test

**`u3g2_d_u6_5_carrier_fixtures_are_the_live_successors` is sound:**
- It `include_str!`s both carriers and builds U1's document form from the live successor.
- **In the registered build** that successor comes from the actual Direct entry (`direct()`), which asserts the build status, one run and G-C once, and a `Successor` publication.
- **In any other build** it comes from the private driver.
- It compares the document with the carrier **byte for byte** (`document == carrier`), and checks U1's file and receipt hashes.
- It passes registered and Stale.

**Mutants** (`d_u6_5_mutants.py`, `d_u6_5_mutants_{reg,stale}.json`): **7 of 7 killed, none by a compile error.**

| Mutant | Registered | Stale |
|---|---|---|
| D1: the dense carrier gains a trailing newline | killed, **only** by D-U6-5 | killed, only by D-U6-5 |
| D2: one digit of the sparse carrier changed (`…868e-106` → `…867e-106`) | killed, **only** by D-U6-5 | killed, only by D-U6-5 |
| D3: the two carriers swapped in the test | killed by D-U6-5 | killed by D-U6-5 |
| D4: the published successor loses a diagnostic after precommit (production) | killed by D-U6-5 and both existing successor tests | — |

## 5. Nothing else in PP changed behaviour

- **The PP source delta** is the D-U6-5 test and the T17_V4 coefficient only.
- **PP, all targets:**
  - registered: **705 passed, 1 failed (the Mac t13), 10 ignored**;
  - Stale: the same, outcome-identical to registered;
  - against my grant-2 run, the only difference is the added D-U6-5 test.
- **The in-build profile record is unchanged:** 0.8881 M sparse (max without R 3,508,669,422) and 0.8929 M dense (3,528,379,870), equal to G7 Pass A's. T17_V4 does not set either maximum.
- **runner/headless (registered):** 85 passed, 2 failed (the two `load_reference` tests), per-test identical to grant 2's.
- **The sweep (§2)** is byte-identical in both builds, so U6's production changes in `result_export` (F5, `semantic_contract`, `derivative`) change no PP route's bytes on these inputs.

## Evidence (`evidence/addendum_01/`; placeholder paths only)

- **Probe and outputs:** `zz_rv93.rs` (with the two added tests), `probe_instrumentation.diff` (counters and the F5 seam), `probe_output.txt`.
- **Build identity:** `identity_check_head.txt`.
- **F5 control:** `f5_removed_control.txt`.
- **Sweep:** `sweep_sha256.txt`.
- **Hashes of my outputs:** `head_outputs_sha256.txt`.
- **Readers and U5:** `py_reader_head.txt`, `u5/`.
- **Suites:** `pp_head_reg.tests`, `pp_head_stale.tests`, `runner_head.outcomes`.
- **D-U6-5 mutants:** `d_u6_5_mutants.py`, `d_u6_5_mutants_reg.json`, `d_u6_5_mutants_stale.json`.
- **Scripts and log:** `run_ext.sh`, `run_ext.out`.

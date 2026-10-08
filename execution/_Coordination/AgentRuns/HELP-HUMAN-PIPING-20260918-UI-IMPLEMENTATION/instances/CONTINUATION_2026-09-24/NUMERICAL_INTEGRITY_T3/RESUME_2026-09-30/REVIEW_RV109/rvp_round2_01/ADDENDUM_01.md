# RV109 (RV-P), round 2, addendum 01: I85's I3 step on SP, confirmed

TASK (Type 2), RV109, role RV-P for B1, for ROOT (HELP_HUMAN, Agent 0), through the coordinator's dispatch. I made no delegation. I wrote none of this change. 2026-10-08 UTC.

**The candidate:** `codex/piping-t3-b1-20261007` at **`03f55e71786a58ce9ed61863813e2410a9f53827`** (`WT/b1`), on **I3 = `2ba2f81863`**, which is ROOT's merge of SR-RS (`b1-r` at `b5cb7faaeb`) into `603e238517`. It adds three commits:
- `2fb55b60e2`: I3's pins and fixtures, SF-1, SF-2, and the ordinal fix with its pin;
- `105e1a78c6` and `03f55e7178`: tests only.

**Basis:**
- my round-2 report (`REVIEW.md`, `206fd360…`);
- RR "RV109 passes SP in RV-P round 2; …" and RR "I98's B2-W verified; …", ruling 4 (the nodal-term ordinal);
- C2's CONTRACT_DELTA:104 (`R/I32/f2a_wire_c2/CONTRACT_DELTA.md`);
- the RS and PY readers' nodal-term derivation at the head;
- **after forming my own view:** I85's record `R/I85/b1_sp_01/I3_01.md`. Its sha256 is `5084603d81ac34297a4c…`, and `SHA256SUMS.i3_01` verifies 120 of 120, on NUM `03d4266d40`.

**Placeholders** are as in REVIEW.md. `E` = `evidence/addendum_01/`. My copies (deleted): `I3C` = `2ba2f81863`; `HEADC` = `03f55e7178`; `MUTC` = `03f55e7178` for mutants.

## 0. Verdict

**CONFIRMED.** I85's I3 step does what the I3 record claims. It closes my round-2 SF-1 and SF-2, and the ordinal fix is right, behaviour-neutral for every input authored in canonical order, and pinned.

| BLOCKING | SHOULD-FIX | NOTE |
|---|---|---|
| 0 | 0 | 2 |

| ID | Severity | Path | Evidence | Remedy |
|---|---|---|---|---|
| N-1 | NOTE | PP `retained_wire.rs` `case_source` (`constructor_ordinal: t.original`) | **The fix rests on one nodal term per primitive load.** Inside D1 each admitted load is one force or moment in one direction, so it gives exactly one term, and the authored load index is unique per term.<br>• RS (`retained_precision.rs:3822`) and PY (`retained_precision.py:1533`) derive exactly that: one term per load, with `constructor_ordinal = primitive_load_index = i`.<br>• **The limit:** if a later domain admits a load that yields several terms, its terms would share an ordinal. C2's row ("breaks indistinguishable duplicates") would then need the per-term ordinal, in the producer and all three readers together.<br>• B2's combinations do not hit this: a CombinationSource has no case-shaped `nodal_terms` | For B2 and later widenings: none now |
| N-2 | NOTE | PP `retained_facade_tests.rs` (`b1_sp_w_c2_direct_entry_publishes_the_pinned_successor`) | **The c ≥ 2 notice path is no longer pinned on the Direct entry.** Since W-C2 publishes, the old Direct test, which pinned the ordinary bytes followed by two N1 notices, became the successor pin.<br>• T-12 at c ≥ 2 (|A| notices, detail placement) is still pinned through `retained_w1` with injected serializer and call faults. The Direct entry calls the same `retained_w1`.<br>• At c = 1, Direct's fallbacks keep their `u3g2_*` pins | **Optional:** a Direct variant of W-C2 with an armed serializer fault |

## 1. The W-C2 pins and fixtures, both modes (`E/probe/`)

- **The committed fixtures:** `P/fixtures/results/retained_precision_w_c2_successor_sparse_interactive.json` and `…_dense_scrutiny.json`. Their sha256 are `7922e3e5278d0d87dc5faf79dfbc1f2a384899e97df306cc742355cdacdb6269` and `f2800bd4f2b4c90217918a6e1287f98305a1b6b07893295b5790f387c075d3a3`, which are the pinned document hashes.
- **My own documents match them byte for byte** (`cmp`; `w_c2_documents.sha256`), at I3C and HEADC, in both modes. These were built from my own W-C2 construction (input `ea5c13b1…`) through the actual registered Direct entry (`runs 1, complete_gates 1`).
- **Receipts and published bytes equal the pins:** sparse `cccb9664…` and `c7a18593…`; dense `612e23ca…` and `a77c010b…`. The private driver publishes the same bytes.
- **My re-derivation at the head passes every check in both modes** (31 per three-case row, 29 for (A, A2)): W-C2, (C, B, A) and (A, A2). The accepted reader passes each one (`wc2_head_summary.txt`).
- **T-7 on C** ends at `Precommit { G8, PREPARATION_MISMATCH }`, now asserted. My round-2 trace explains why: the hook's zeroed fact is refused by G8.

## 2. SF-1: N-16 record by record (`E/probe/n16_head.txt`)

**The test** (`n16_batch_against_one_case`):
- it compares terminal facts and record counts;
- it then patches only `shared_built_here` and `verification_shared_built_here` on a copy, and compares whole records with `==`;
- it asserts the noted flags exactly: none at batch position 0, and (p128 `shared_built_here`), (p256 `shared_built_here`), (p256 `verification_shared_built_here`) at every later position.

It runs on W-C2 and on both SF-2 inputs, in both modes.

**My own comparison** uses `AttemptRecord`'s full `Debug` form, which prints every field, including all work and stage counters. On (A, B, C), (C, B, A) and (A, A2), in both modes:
- the first Run equals its one-case Run;
- **the second differs in exactly those three flag values,** with every other field equal.

**Two test mutants show the comparison is live and the exception exact.** Both are killed (`E/mutants/`), in the W-C2 and SF-2 tests:
- **TM1** drops the verification flag's patch. It dies at "record p256 differs beyond the build-provenance flags".
- **TM2** expects no flag. It dies at "C2 §4's shared builds only".

## 3. SF-2: the (C, B, A) and (A, A2) pins (`E/mutants/`)

**The pins match my own constructions exactly**, in both modes (receipt and successor bytes):
- **(C, B, A):** `863d692f…`/`ea9a4846…` sparse; `7aeecbac…`/`c719bd8d…` dense;
- **(A, A2):** `41f33085…`/`529233eb…` sparse; `30001ccf…`/`f4075cdc…` dense.

**My six round-2 patch strings, re-applied unchanged to `MUTC`,** each die in `b1_sp_sf2_selected_not_first_and_two_selected_pins` at the assertion named for them:

| Mutant | First failing assertion |
|---|---|
| M15 | `:2269` "a_a2 …: T-11's staging order (M15)" |
| M16 | `:2259` "c_b_a … request 2: its source's owner and attempt (T-7; M16)" |
| M17 | `:2251` "c_b_a …: product attempt ids in start order (M17)" |
| M19 | `:2257` "c_b_a … request 2: the Run's owner is the request index (N-2; M19)". Also `b1_sp_r3p_1_…` (B, A), through the reader |
| M28 | `:2277` "c_b_a …: the max_displacement headline (M28, M32)" |
| M32 | `:2277` "a_a2 …: the max_displacement headline (M28, M32)" |

**My round-2 N-1 (M11) is closed.** M11 is now killed by five tests: the W-C2 pin, its fixture test, the transaction test (precommit G8), the `retained_w1` test, and SF-2.

**The pristine control** shows only t13 failing. The restored sources equal `03f55e7178` (`restored_src.sha256`).

## 4. The ordinal fix

**The change** is one line in `retained_wire.rs` `case_source`: `"constructor_ordinal": i` becomes `t.original`. This is the authored primitive-load index, the same as `primitive_load_index`, and the array keeps kernel canonical order. It is what both readers derive (N-1).

**c = 1 byte identity** (`E/probe/probe_compare.txt`, `stage_compare.txt`). This uses my round-2 probe on I3C and HEADC, over 66 inputs × 2 modes:
- the 61 earlier inputs: the committed witnesses, `attempted_examples`, 34 committed fixture requests and invocations, and I86's inputs;
- plus five of my own out-of-order inputs.

| Comparison | Identical | Differing |
|---|---|---|
| Probe rows (plain bytes, Direct bytes and cause, driver envelope and successor, sentinel, seam) | **124 of 132:** every committed and canonical-authored input, c = 1 and c ≥ 2 | **8:** exactly my four out-of-order c = 1 inputs × 2 modes (milestone reversed, L = 0 reversed, two-body A reversed, the milestone's first two loads swapped). Each moves from G8 plus one notice to a published successor. Case C reversed is unchanged: it ends at Native, before any receipt |
| Stage rows, c = 1 (adapter counts after the run, preparation, native, candidate, staging and serialization; outcomes; snapshots; staged summary) | **102 of 110** | **8:** the same four inputs, and **only** in the reader's verdict and the successor sha256. Every adapter count is identical |

The c = 1 pins are unchanged: milestone receipts `efc1a39b…` and `3e26499f…`, read on Direct at the head.

**Pinned literals:** every 64-hex literal in PP, RE and the runner (`src` and `tests`) at I3C is still present at HEADC. That is 86 occurrences, 50 distinct, which is I85's 86. The head adds 18 new ones: W-C2 6, SF-2 8, and the ordinal 4. I reproduced every one.

**The reversed-moments input** (`zz_rv109_reversed_orders`, `py_reader_reversed.txt`):
- **At HEADC it publishes in both modes,** through `retained_w1` and the registered Direct entry (`runs 1, complete_gates 1`). The receipts and published bytes are I85's pins: `b79f691a…`/`93aa0435…` sparse, `c6b03683…`/`b70edc6d…` dense. Its terms carry (ordinal, index) = (2, 2), (1, 1), (0, 0), in canonical order.
- **The readers accept it:** the Rust reader with the invocation passes, eligible. So does the Python retained reader at the head, run with its units and checked-JSON helpers built from the head's own sources.
- **So do my other out-of-order inputs** in both readers: swapped loads, two-body A, L = 0, and W-C2 with case A's or case C's loads reversed.
- **At I3C every one of them** fell back at precommit G8 `PREPARATION_MISMATCH`, with one notice per case in A, and **both readers refuse** the successor precommit received there. That is the defect, reproduced.
- The canonical inputs pass both readers at both revisions.
- **Reverting to `i` is killed:** O1 dies at `:2325`, "sparse_interactive: the authored index, in canonical order".

## 5. The suites, I3C against HEADC, test by test (`E/suites/`)

| Suite | I3C | HEADC | Differences |
|---|---|---|---|
| PP registered, all targets | 730 ok, 6 failed, 11 ignored | **738 ok, 1 failed (t13), 11 ignored** | **+4:** `…_direct_entry_publishes_the_pinned_successor` (renamed from `…_direct_entry_counts_one_run_through_g_c`), `…_w_c2_fixtures_are_the_live_successors`, `…_sf2_…_pins` and `…_constructor_ordinal_is_the_authored_index`. **Four now pass** that failed at I3C: `r3p_1`, `…_through_retained_w1_…`, `…_faults_and_abandonment` and `…_outcomes_and_ordinal_mapping`. Nothing else changes |
| PP Stale (`--lib`) | 568 / 5 / 11 | **575 / 1 / 11** | the same delta (I85's 731 → 738 counts all targets) |
| Runner (headless) | 85 ok, 2 failed | identical | 0: the two `load_reference` failures, as at every base |
| Witnesses (`--ignored`) | 10/10 | 10/10 | output identical after normalizing |
| RE carriers | 17/17 | 17/17 | 0 |

**The guards:** s11f (including rule 8), PP-tests' admission guard, and `u1_serializer_reads_no_legacy_work_field` all pass. The step touches none of their files.

**Weakening:**
- **Every removed assertion has a post-I3 replacement.** The G5 expectations become successor and pin assertions. The T-7-on-C expectation becomes G8. The Direct entry's two-notice bytes become the pinned successor (N-2).
- **A1-N-1's capture now precedes validation.** `105e1a78c6` and `03f55e7178` move SF-2's structural assertions onto the successor precommit received, ahead of "ordinary owner untouched".

## 6. The PR-head ledger, extended to `03f55e7178` (`E/ledger/`)

**`603e238517` → `2ba2f81863` (I3):**
- The merge brings only SR-RS's three RE files: `retained_precision.rs`, `source_blocks.rs` and `tests/retained_precision_contract.rs`. They are byte-identical to `b5cb7faaeb`, which RV113 reviewed.
- No SP file changes.

**`2ba2f81863` → `03f55e7178`:** every hunk is SP's and reviewed here.

| # | File (PP) | Hunk | Context | +/− | SP commit(s) | Reviewed |
|---|---|---|---|---|---|---|
| 1 | `retained_wire.rs` | `-969,7 +969,10` | `case_source`: the ordinal | +4/−1 | `2fb55b60e2` | RV109 r2 a1 |
| 2 | `retained_facade_tests.rs` | `-1558,8 +1558,10` | the T-8–T-13 header comment | +4/−2 | `2fb55b60e2` | RV109 r2 a1 |
| 3 | | `-1577,21 +1579,75` | `one_case_run`, `run_records`, `n16_batch_against_one_case` (SF-1) | +61/−7 | `2fb55b60e2` | RV109 r2 a1 |
| 4 | | `-1599,8 +1655,9` | the W-C2 doc (I3, N-16) | +3/−2 | `2fb55b60e2` | RV109 r2 a1 |
| 5 | | `-1612,9 +1669,11` | W-C2: the precommit accepts | +4/−2 | `2fb55b60e2` | RV109 r2 a1 |
| 6 | | `-1673,14 +1732,11` | W-C2: N-16 record by record | +3/−6 | `2fb55b60e2` | RV109 r2 a1 |
| 7 | | `-1693,7 +1749,9` | the faults doc (T-7 on C at G8) | +3/−1 | `2fb55b60e2` | RV109 r2 a1 |
| 8 | | `-1718,7 +1776,7` | T-7 on C: G8 | +1/−1 | `2fb55b60e2` | RV109 r2 a1 |
| 9 | | `-1750,8 +1808,9` | the `retained_w1` test's doc | +3/−2 | `2fb55b60e2` | RV109 r2 a1 |
| 10 | | `-1768,9 +1827,12` | `retained_w1`: the successor | +6/−3 | `2fb55b60e2` | RV109 r2 a1 |
| 11 | | `-1780,18 +1842,34` | `W_C2_PINNED`, `w_c2_document`, the Direct test's head | +23/−7 | `2fb55b60e2` | RV109 r2 a1 |
| 12 | | `-1800,12 +1878,26` | the Direct test: the pins | +19/−5 | `2fb55b60e2` | RV109 r2 a1 |
| 13 | | `-1869,8 +1961,8` | the R3P-1 doc | +2/−2 | `2fb55b60e2` | RV109 r2 a1 |
| 14 | | `-1890,8 +1982,9` | R3P-1: the successor | +2/−1 | `2fb55b60e2` | RV109 r2 a1 |
| 15 | | `-1906,7 +1999,12` | R3P-1: A's one notice under a serializer fault | +6/−1 | `2fb55b60e2` | RV109 r2 a1 |
| 16 | | `-2053,3 +2151,192` | the fixtures test, the SF-2 pins, the reversed milestone and the ordinal test | +189/−0 | `2fb55b60e2`, `105e1a78c6`, `03f55e7178` | RV109 r2 a1 |
| 17 | `P/fixtures/results/retained_precision_w_c2_successor_sparse_interactive.json` | new, 18,583 lines | the W-C2 document | +18583 | `2fb55b60e2` | RV109 r2 a1 (byte-compared, §1) |
| 18 | `P/fixtures/results/retained_precision_w_c2_successor_dense_scrutiny.json` | new, 18,583 lines | the W-C2 document | +18583 | `2fb55b60e2` | RV109 r2 a1 (byte-compared, §1) |

**With REVIEW.md §3, every ST and SP hunk on the branch up to `03f55e7178` is now reviewed by RV-P.**

## 7. Host, cleanup and limits

- **Jobs:** 25, run one at a time by one sequential script through `runjob.sh` → `WT/tools/t3_cargo.sh` (the slotted lock):
  - the suites: 10;
  - the probe: 2;
  - the mutants: 11;
  - two release builds of the Python reader's helpers.

  My jobs never overlapped (`E/cargo_jobs_rv109_a1.log`). I killed no job, and no wait of mine is running.
- **No Git writes; no DEC-025; no installs.** Nothing is in the system temp directory. My shell ran in my scratch.
- **Deleted:** my copies `WT/rv109/a1_{i3,head,mut}` and my targets `WT/targets/rv109-a1-*`.
- **Not touched:** I85's `S/sp03/mut3`, `S/base_i3` and `WT/targets/i85-b1-st*` were neither used nor touched, and are left for ROOT.
- **The records were screened:** no machine path, no symlink, and no host or local-domain name.
- **A scratch slip, disclosed.** My addendum scratch, `S/a1/`, reused my round-1 addendum's folder name. That run's scratch logs and its mutant scratch were moved into `S/logs/r1_addendum_01/` and `S/a1/r1_addendum_01/`. Its pristine source copies, `list.tsv` and `a1.done` in `S/a1/` were overwritten. That round's records in `R` are untouched.
- **Limits:**
  - The TS reader was not run (ROOT checked it).
  - The Python reader ran from the head's sources with helpers I built from them.

## 8. Records

- `ADDENDUM_01.md` (this file) and `SHA256SUMS.addendum_01`, which covers it and every file under `evidence/addendum_01/`.
- **`E/suites/`:** the filtered logs and `suite_diff_I3__03f55e7178.txt`.
- **`E/probe/`:**
  - the probe and shim;
  - both revisions' JSONL outputs and RV109 lines;
  - `probe_compare.txt`, `stage_compare.txt`, `wc2_head_summary.txt`, `n16_head.txt` and `py_reader_reversed.txt`;
  - `w_c2_documents.sha256`.
- **`E/mutants/`:** the list, `summary.md`, the filtered logs and `restored_src.sha256`.
- **`E/ledger/`, `E/tools/`,** and `E/cargo_jobs_rv109_a1.log`.

REVIEW.md, SHA256SUMS and the round-2 evidence are unchanged.

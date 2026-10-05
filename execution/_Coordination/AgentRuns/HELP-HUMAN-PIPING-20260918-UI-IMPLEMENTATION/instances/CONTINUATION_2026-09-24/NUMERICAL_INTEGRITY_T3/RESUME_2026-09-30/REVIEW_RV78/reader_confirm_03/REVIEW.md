# RV78 confirmation 03: snapshot 07c, the `after_rehash` format and joint parity

RV78 is a TASK (Type 2), resumed by ROOT (HELP_HUMAN) for a scoped round (workflow §3: recheck what changed, reuse valid evidence). ROOT is the return path, and RV78 did not delegate. RV78 wrote none of the repairs.

- **Candidate:** READER `a894d9d0bacc98deb0adcab215d1bc9a91ea4373`; the previous head was `b36739112a`. NUM is at `19065f4828`.
- **Shared files:** corpus `d33667719e…` (15 cases, 254 mutations, 19 must-pass entries). The schema `07951edacf…` is unchanged, as are the definition and the semantic table.
- **Run window:** 2026-10-03, 19:10:09 to about 19:20 local, inside the 60-minute box. Nothing in scope is unfinished. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes, no install, no new tooling, and no native, solver or DEC-025 job.
- **Out of scope:** T1/T2 from I61's experiment.

## Verdict: PASS

**Counts:** 0 BLOCKING, 0 SHOULD-FIX, 2 NOTE.

- **Every confirm_02 finding is fixed or recorded** as ROOT disposed it.
- **The joint parity run on 07c is complete:** 288 of 288 entries agree in all three readers and match their expectations.
- **The `after_rehash` format** has the same semantics in all three harnesses and in RV78's own implementation.
- **All 18 new mutations** are contract-faithful correct rejections.
- **No check was weakened.**

## Disposition of the confirm_02 findings

| ID | Disposition | Decision | RV78 evidence on `a894d9d0ba` |
|---|---|---|---|
| S1 | **Fixed** | D23 | The pin `source_member_map_kernel_id_noncanonical` is byte-identical to RV78's probe. Both RV78 D1 probes now give G3 COVERAGE in all three readers. |
| S2 | **Fixed** | D24 | There are four `after_rehash` pins, and each forges exactly one hash. RV78's JCS confirms that the other hashes in each pin verify. T4c is pinned (`source_identity_stale_receipt_rehashed`), and the deferred note is corrected. RV78's own H1/H3/H4/H5 probes still give G1 in all three readers. |
| N1 | **Fixed** | D25 | Python dropped its integral-float rule. Probe F1 (`work.charged = 17.0`) now passes in all three readers. A non-integral counter still fails G2 (Python's test). |
| N2 | **Recorded** | — | "N13 isolation" is in SHARED_SNAPSHOT_07B `deferred`. |
| N3 | **Recorded** | — | Pin overlap, as ruled. |
| N4 | **Fixed** | D21 round (RV78-N4) | I64's OUTCOMES_07B (272 entries) shows SECTION for `g5b_zero_section_area`, and it agrees with RV78 on every overlapping mutation. |
| N5 | **Recorded** | — | ROOT: classes 1 and 2 are the classes that read references. |

**All 52 reader_confirm_02 probes were rerun.** Each gives the outcome its decision sets, in all three readers. The one informational probe (X9) gives G5 ATTEMPT in all three.

## New findings

| ID | Severity | Where | Evidence | Remedy |
|---|---|---|---|---|
| N1 | NOTE | SHARED_SNAPSHOT_07B `deferred` ("D19 Ready direction… needs selected-row/diagnostic rewrites beyond a faithful base") | **The D19 Ready direction can be expressed as a shared pin.** On `two_case_synthetic`, RV78 turned case 1 into an unavailable case with its Ready attempt kept. That took 80 edits: status, removing method/selection/identity, reason, `diagnostic_ref`, the diagnostic code 16 → UNAVAILABLE, and removing the 74 row tokens. Results in all three readers:<br>• under a `facade_failure` cause: G5 PRODUCT_ATTEMPT;<br>• under `prepared_product_failure` naming the Ready attempt: G5 PRODUCT_ATTEMPT;<br>• the control, under `receipt_failure`: passes.<br>Today the rule is pinned only by a Python reader-local test. | Add the two negatives as shared pins (PROBES_CONFIRM_03, batch "D19 Ready direction"). The control may serve as a must-pass entry, but native emission of a per-case `receipt_failure` beside a selected case is not established. It is accepted by D19's text only, so it could stay a reader-logic control. |
| N2 | NOTE | Corpus `unavailable_attempt_under_source_error_cause` | **A latent second defect.** The added `source_decline` says `error: no_nodes` with `constructor_counts.nodes: 2`, which contradicts C2:56. It also sits on a case whose failure was at preparation, before source construction. No reader checks `source_decline` content today, so all three give the intended D19 PRODUCT_ATTEMPT. But a later `source_decline` consistency check would run in class 2 ordinary, which comes before C3 under D17, and would move this pin to ATTEMPT. | Make the `source_decline` self-consistent (for example `nodes: 0` with `no_nodes`, leaving the invocation contradiction to G8), or note the dependency. |

## 1. Shared-file and harness diffs, `b36739112a` → `a894d9d0ba`

**The corpus** (RV78's own diff):
- version, provenance, arithmetic, all 15 cases and all 19 must-pass entries are unchanged;
- all 236 mutations are carried byte-identical in order, with nothing removed;
- **18 are new:** 17 in 07b and 1 in 07c;
- `after_rehash` appears only on the four G1 hash pins;
- every entry uses `rehash:"all"`.

**The schema:** no change.

**The harnesses.** All three apply `after_rehash` literally after `rehash:"all"`, with nothing rehashed afterwards, and treat it as empty when absent:
- Python: `_apply_edits(value, mutation.get("after_rehash") or [])`;
- Rust: `entry["after_rehash"].as_array()…unwrap_or_default()`, after `rehash`;
- TypeScript: `applyEdits(source, m.after_rehash)`, where `applyEdits` accepts undefined.

This matches SHARED_SNAPSHOT_07B `format_change`. RV78 implemented the same semantics independently, and its prepared inputs equal the Python harness on all 273 entries.

**Removed or changed assertions in the test files:**
- the D10 integral-float G2 test, replaced by D25's numbers-are-values test, which still requires G2 for a non-integral counter. The removal is by decision;
- count updates, 236 → 254;
- one TypeScript kernel-scope setup line, corrected from the nonexistent state `'failure'` to `'nonbudget_failure'`. It now also asserts that a build was found, which is stricter (RV81-N2).

**Nothing in the shared files or harnesses is weakened.** RV78 reviewed only the shared files and harnesses here; reader source is RV79–RV81's scope.

## 2. The joint parity run on 07c

| Reader | Command result |
|---|---|
| Python | pytest: 348 passed |
| Rust | default toolchain, no `DEVELOPER_DIR`: 47 passed |
| TypeScript | vitest 409/409; tsc exits 0 |

Per-entry results (PARITY_TABLE.json), using RV78's own edit applier, JCS, rehash and `after_rehash`, with each reader's validate entry called directly:
- **288 of 288 entries** match in all three readers: 15 cases, 254 mutations and 19 must-pass entries. Disagreements: 0. G7 is compared per reader.
- **Hashes:** all 15 base cases' hashes are reproduced.
- **Independent checks:** RV78's invariant checks of the 15 cases and 19 must-pass entries found nothing, and all 5 p512 floors match the exact-rational Φ.
- **Author outcome files:** the latest from each author (I62 PYTHON_OUTCOMES_07C, I63 OUTCOMES_07C, I64 OUTCOMES_07B) agree with RV78's run wherever they overlap: 0 differences.

## 3. The new entries: contract fidelity and emittability

RV78 reviewed all 18 new mutations (NEW_ENTRIES_REVIEW_03.json):
- **Expected gate and code:** RV78's reading agrees with the corpus on all 18.
- **Emittability:** every one is a correct rejection of a receipt the producer cannot emit. The basis for each:
  - D19/S06 §1 for the cause-branch pins;
  - D20/C3:165;
  - D21 (adaptive.rs:4286, 4333);
  - D22/D16;
  - D23/C2:98;
  - D24 for the G1 rows;
  - D26 for the R7 sanity margin, whose arithmetic RV78 did not rederive (RV79's vector);
  - D27/C1:60;
  - D28 (adaptive.rs:4181–4196, where estimate and charge reasons take their quantity, body and kind from one layout row);
  - D29 (FK source.rs:498).
- **Isolation, checked independently by RV78:**
  - **`vbuild_on_escalating_failed_verification`:** RV78's work and build checker finds the added v256 build, its charges and the meter chain consistent, so D21 is the only defect;
  - **`idle_run_exhausted_meter_chain_broken`:** the checker finds exactly the run-1 chaining defect, so D27's recorded-value rule leaves WORK as the only code;
  - **the four `after_rehash` pins:** each forges exactly one hash, as RV78's JCS shows.
- **Isolation, from I62's recorded 07a-Python observations:** the other pins were passing before their rule existed.
- **The one caveat** is N2.

## 4. Method, environment and evidence

**Commands:** as in reader_confirm_02, on the default toolchain without `DEVELOPER_DIR`.

**Disclosed:**
- **`node_modules`:** linked to READER's link target.
- **WASM assets:** READER's prebuilt `public/wasm-engine` and `public/self-weight-engine` were **copied**, not built. The hashes are unchanged from confirm_02:
  - wasm-engine `.js` `5682432840e2…`;
  - wasm-engine `_bg.wasm` `7843297271c6…`;
  - self-weight-engine `.js` `ea2b3fd611a6…`;
  - self-weight-engine `_bg.wasm` `3bc83f88bfef…`.
- **Probe test files:** RV78's two probe test files existed only in `WT/rv78/`, which is deleted.

**Evidence in this folder:**
- **PARITY_TABLE.json:** 288 rows.
- **PROBES_CONFIRM_03.json:** 55 probes. That is the 52 confirm_02 probes rerun, plus the three D19 Ready-direction probes.
- **NEW_ENTRIES_REVIEW_03.json.**
- **INDEPENDENT_CHECKS_03.json.**
- **SHA256SUMS.**

**Scripts and logs** are in `WT/scratch/rv78_reader_confirm3/`. Only `rv78_prepare.py` differs from the confirm_02 copies (it adds `after_rehash`, sha256 prefix `9cf44a05cfbd4237`).

## 5. For ROOT

There are no rulings needed for acceptance. Optionally:
1. **N1:** add the two D19 Ready-direction negatives as shared pins.
2. **N2:** make the `source_decline` in the D19 source_error pin self-consistent.

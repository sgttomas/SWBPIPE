# RV78 confirmation 04: the 07d delta (D31–D33, D32 integral values)

RV78 is a TASK (Type 2), resumed by ROOT (HELP_HUMAN) for a round scoped to the 07d delta (workflow §3). ROOT is the return path, and RV78 did not delegate. RV78 wrote none of the repairs.

- **Candidate:** READER `abcb16fd27d7c3ccd019f2533eb261d4c564fdc7`. The previous head was `a894d9d0ba` (07c). NUM is at `83732c5677`.
- **Shared files:** corpus `12da125d9d…` (15 cases, 259 mutations, 21 must-pass entries). The schema, definition and semantic table are unchanged.
- **Run window:** 2026-10-03, 19:46:18 to about 19:54 local, inside the 45-minute box. Nothing in scope is unfinished. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes, no install, no new tooling, and no native, solver or DEC-025 job.

## Verdict: PASS

**Counts:** 0 BLOCKING, 0 SHOULD-FIX, 1 NOTE.

- **The joint parity run on 07d is complete:** 295 of 295 entries agree in all three readers and match their expectations.
- **The integral-float must-pass entry hashes identically** to its integer-written form, as RV78's own JCS shows.
- **Every new or changed entry is contract-faithful.**
- **RV78's N1 and N2 are both addressed.**
- **No check is weakened** in the shared files or harnesses.

## Disposition of RV78 confirm_03 N1 and N2

| ID | Disposition | Evidence |
|---|---|---|
| N1 (D19 Ready direction) | **Fixed.** Two shared negatives were added; the control stays reader-logic, as RV78 suggested. | `ready_attempt_under_facade_failure_cause` and `ready_attempt_under_prepared_product_failure` carry the same 81-edit set as RV78's D19r probes. Both give G5 PRODUCT_ATTEMPT in all three readers. SHARED_SNAPSHOT_07D `not_added` records why the `receipt_failure` control is not a must-pass entry. |
| N2 (inconsistent `source_decline`) | **Fixed** | `unavailable_attempt_under_source_error_cause` now has `constructor_counts.nodes: 0` with `no_nodes`, consistent with C2:56. The expectation is unchanged, and all three readers give G5 PRODUCT_ATTEMPT. |

## New finding

| ID | Severity | Where | Evidence | Remedy |
|---|---|---|---|---|
| N1 | NOTE | Harness rehash: tests/test_retained_precision_contract.py:106 and :111 (`int(...)`); result_export/tests/retained_precision_contract.rs `index()`; retainedPrecision.test.ts:109–115 | **The three harnesses rehash with different index and precondition semantics.**<br>• Python's `int(...)` truncates (`int(0.5)` = 0) and accepts booleans.<br>• Rust's `index()` applies the D32 value test and skips an unresolved reference.<br>• TypeScript indexes arrays directly. It also recomputes a source's preparation hash even when its attempt has a non-prepared member, which Python and Rust do not.<br>**This is latent.** RV78 scanned every 07d entry: none has a non-integral, negative, −0 or boolean rehash reference, or a sourced attempt with a non-prepared member. RV78's own rehash, which uses the D32 value test and skips unresolved references, equals the Python harness on all 280 entries. | State the rehash semantics in the snapshot format (index by the D32 integral-value test; skip unresolved references; preparation hash only when every member is prepared), and align the three harnesses. Optional; no current entry is affected. |

## 1. Shared-file and harness diffs, `a894d9d0ba` → `abcb16fd27`

**The corpus** (RV78's own diff):
- version, provenance, arithmetic and all 15 cases are unchanged;
- all 254 mutations and 19 must-pass entries are carried in order;
- **exactly one entry changed:** `unavailable_attempt_under_source_error_cause`, the N2 fix with its expectation unchanged;
- **new:** 5 mutations and 2 must-pass entries;
- nothing was removed.

This matches SHARED_SNAPSHOT_07D and ROOT's diff.

**The schema:** no change.

**The harnesses, D32 integral-value indexing:**

| Harness | Change |
|---|---|
| Python | Rehashing indexes `product_attempts` and `sources` by `int(value)` (its D32 change). |
| Rust | Adds `index()`, the value test: finite, integral, at least 0, not −0. It is used for rehash references and for edit paths, including array removal. A source identity whose `source_ref` cannot be addressed is skipped, as at G1. |
| TypeScript | No harness indexing change. JavaScript array indexing already treats `0.0` as `0`. It adds reader-local D31, D32 and D33 tests. |

`after_rehash` and `rehash:"all"` are unchanged. Removed assertions are count updates only (254→259, 19→21). Ruled replacements are in reader source, out of RV78's scope. **No harness check is weakened.**

## 2. The joint parity run on 07d

| Reader | Command result |
|---|---|
| Python | pytest: 358 passed |
| Rust | default toolchain, no `DEVELOPER_DIR`: 53 passed |
| TypeScript | vitest 419/419; tsc exits 0 |

Per-entry results (PARITY_TABLE.json), using RV78's own edit applier, JCS, rehash (indexing by the D32 integral-value test) and `after_rehash`, with each reader's validate entry called directly:
- **295 of 295 entries** match in all three readers: 15 cases, 259 mutations and 21 must-pass entries. Disagreements: 0. G7 is compared per reader.
- **Hashes and the Python harness:** all 15 base hashes are reproduced, and RV78's prepared inputs equal the Python harness on all 280 entries.
- **The integral-float must-pass entry** (`integral_float_integers_and_references`, 32 integer fields and references written as integral floats) has receipt, publication, source-identity and preparation hashes byte-equal to the integer-written base (RV78 JCS). RV78's prepared JSON preserves the `1.0` spelling, so Rust parses floats and TypeScript parses numbers.
- **Independent checks:** RV78's invariant checks of the 15 cases and 21 must-pass entries found nothing (value semantics, D25/D32), and the 5 p512 floors match.
- **Author outcome files:** I62 PYTHON_OUTCOMES_07D, I63 OUTCOMES_07D and I64 OUTCOMES_07D agree with RV78's run wherever they overlap: 0 differences.

**Probes** (PROBES_CONFIRM_04.json, 61 in all):
- **All 55 confirm_03 probes** give their ruled outcome in all three readers. X9 is informational, and all three agree on it.
- **Six D32 edge probes, also all agreeing:**
  - a non-integral `source_ref` (0.5) gives G2 ENCODING;
  - a −0 `attempt_ref` gives G2;
  - a boolean `product_attempt_ref` gives G1;
  - `receipt_version: 1.0` with float-written limits passes;
  - `receipt_version: 2.0` gives G0;
  - an exponent-written counter (`1.7e1`) passes.

## 3. New and changed entries: contract fidelity and emittability

RV78 reviewed all 8 (NEW_ENTRIES_REVIEW_04.json). RV78's reading agrees with the corpus on every one:
- **`model_schema_version_0_4_0_rejected`:** G8 INVOCATION (D31; C1's G8 row, "no 0.4 extension"). It is a correct rejection, and the invocation digest is rebound so that only the version fails.
- **`model_schema_version_0_1_0_accepted`** (must-pass): emittable. The producer handles 0.1.0 and 0.2.0 on one branch (PP pressure_runtime.rs:113–118), and I61's milestone request is 0.1.0.
- **`forged_source_identity_float_source_ref`:** G1 (D32 + D24). It isolates the forged hash: RV78's JCS shows the receipt and publication hashes valid and only the source identity invalid. Its literal receipt hash equals the integer-written pin's, because `0.0` canonicalizes as `0`.
- **`verification_estimate_names_translation_row`:** G5 ATTEMPT (D33; FK verify.rs:880). It isolates the rule, because the quantity resolves with matching body and kind, so D28 passes and only D33 fails.
- **The two D19 Ready negatives:** G5 PRODUCT_ATTEMPT (S06 §1). See N1 in the disposition table.
- **`integral_float_integers_and_references`** (must-pass): emittable, with the same values as the base; see §2.
- **`unavailable_attempt_under_source_error_cause`** (changed): G5 PRODUCT_ATTEMPT. Its `source_decline` is now self-consistent.

## 4. Method, environment and evidence

**Commands:** as in reader_confirm_03, on the default toolchain without `DEVELOPER_DIR`.

**Disclosed:**
- **`node_modules`:** linked to READER's link target.
- **WASM assets:** READER's prebuilt `public/wasm-engine` and `public/self-weight-engine` were **copied**, not built. Hashes are unchanged: `.js` `5682432840e2…`/`ea2b3fd611a6…`, `_bg.wasm` `7843297271c6…`/`3bc83f88bfef…`.
- **Probe test files:** RV78's two probe test files existed only in `WT/rv78/`, which is deleted.

**Evidence in this folder:**
- **PARITY_TABLE.json:** 295 rows.
- **PROBES_CONFIRM_04.json:** 61 probes.
- **NEW_ENTRIES_REVIEW_04.json.**
- **INDEPENDENT_CHECKS_04.json.**
- **SHA256SUMS.**

**Scripts** are in `WT/scratch/rv78_reader_confirm4/`. Changed since round 03:
- `rv78_prepare.py` (`9eb486cb6c47c48c`) adds the D32 integral-value index;
- `rv78_case_checks.py` (`054cbd1c7c527839`) adds value normalization.

## 5. For ROOT

No rulings are needed. Optionally, specify the harness rehash semantics in the snapshot format (N1).

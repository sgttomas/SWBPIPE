# RV92 (U6f): the complete-diff review of U6, carriers and standing

RV92 is a fresh independent reviewer, a TASK (Type 2) dispatched directly by ROOT (HELP_HUMAN, Agent 0) under `BRIEFS/RV92_U6F_COMPLETE_REVIEW.md`. ROOT is the return path; RV92 did not delegate. RV92 had reviewed none of U6. The unit reviews (RV88, RV90, RV91) were used as leads only; every result below was produced by RV92's own runs.

**The candidate:** `76477534f6`, the head of `codex/piping-f2a-carriers-20261004`, diffed against its merge base with NUM, **`7e4f5a51dd`** (40 files, +16,681/−110). It holds U6a, U6c, U6b, U6e (07h), U6d, their repair rounds and the shared case file (format v2). NUM has since changed only four PP files (`a634ac8b53`, U3 1b/1c), disjoint from U6's 40.

## Verdict: PASS, with one SHOULD-FIX and seven NOTEs; nothing BLOCKING

- **Existing behaviour is unchanged** across the whole diff in all three languages (§2): every committed raw envelope, AnalysisRun, canonical results and stress-neutral document gives the same outcome at base and candidate, apart from the known F1 nondeterministic fixture. Every acceptance suite passes.
- **The receipt survives every carrier path, end to end, in every language** (§3): 18 of 18 carrier×reader×mode combinations come back byte-equal, revalidate with the invocation, and read `needs_recompute`.
- **The three-language parity table** (§1) reproduces all 20 shared cases and all four `declared_differences` exactly. **It also shows differences outside the four declared entries:** RV88's N-1 (confirmed, 10 probes), one SHOULD-FIX (S-1, Python's receipt-copy check is not exact), and three NOTE-level groups (N-2, N-3, N-4) that need a ROOT ruling to declare or repair. All of them fail closed: none makes anything eligible. Where one language binds a row that another refuses (N-3, N-5), every language's standing for that envelope is `unsupported`, so each rule gate refuses before binding.
- **The R-1 interface (C-1 to C-3):** U6's carriers are consistent with it. Two documentation and exposure gaps sit in PP, not in U6 (N-6).
- **Cost:** acceptable for the milestone; memoize before native activation (N-7).
- **Merge into NUM** should follow the post-U6f repair round (S-1, N-1, RV88's U6a N-3) and ROOT's rulings on N-2 to N-5.

## Counts

| | New in RV92 | Carried and confirmed |
|---|---|---|
| BLOCKING | 0 | — |
| SHOULD-FIX | 1 (S-1) | — |
| NOTE | 7 (N-2 to N-8) | RV88 N-1 (confirmed, quantified); RV88 U6a N-3 (open, not re-reviewed) |

One line per finding:
- **S-1 (SHOULD-FIX):** Python's AnalysisRun receipt-copy check uses `!=`, so a resealed record whose copy has `false`/`true` for `0`/`1` passes in Python, is refused by TS, and the copy no longer revalidates (compatibility.py:694).
- **N-1 (RV88, confirmed):** TS's header route admits 10 tampered transported successors that Rust refuses; TS's own unused `validateRetainedPrecisionTransport` returns Rust's code on all 10.
- **N-2:** TS's header route reads raw rows, so a non-successor source with a W1 token row is admitted by Rust's and Python's header-only dispatch but refused by TS (28 probes). Undeclared; TS stricter.
- **N-3:** I67-F2's scope is too narrow. TS refuses every row when a registration exists but was refused (8 probes), and TS's summary is empty for every unregistered valid statement (8 probes). Rust and Python bind and summarise by class. Undeclared; fail-closed.
- **N-4:** Differences inherited from the base carriers surface on the successor and are not declared: the G7 dispatch text (Rust vs Python base validators, 4 probes), Rust's header dispatch admitting `carrier_evidence` (2), and the AnalysisRun builders' refusal vocabulary. All are pre-existing at `7e4f5a51dd` (shown).
- **N-5:** TS's source-blocks summary binding refusal disappears for downgraded source-blocks envelopes (5 probes; 56 sweep forms), while Rust and Python still refuse the row. The mechanism is pre-existing; U6's guard extends it.
- **N-6:** R-1: `into_parts()` carries no "ordinary base only" statement (C-1 partly met), and the public `successor()` borrow accessor exposes the successor besides `into_publication()` (C-3 inexact). PP, for U3 grant 2.
- **N-7:** Cost: one validation is 4.3 ms release (40 ms debug); binding every row of a milestone is 0.38 s release (3.7 s debug) and grows with rows². Acceptable for U7; memoize before native activation.
- **N-8:** The recorded U7 preconditions are complete for TS standing, but the whole diff adds six (listed under N-8 in §5).

## Basis and host

- **Copies (Git reads only, `GIT_OPTIONAL_LOCKS=0`):** `git archive` of `76477534f6` (`WT/rv92/cand`) and `7e4f5a51dd` (`WT/rv92/base`) from `WT/f2a-carriers`; WT/f2a-carriers' working tree was not used. A **merge-preview lane** `WT/rv92/merge` is NUM `3e264e2894` (P without the records tree) with the candidate's 40 files copied over. Its 40 files equal the candidate's byte for byte, and its 4 NUM-only PP files differ from the candidate's, as expected.
- **Runtime, disclosed:** `P/node_modules` in both lanes is a symlink to `REPO_ROOT/P/node_modules` (no install). The WASM assets were copied, not built, from `WT/f2a-readers` (`…operation_applier_bg.wasm` `78432972…`, `…self_weight_wasm_bg.wasm` `3bc83f88…`, as RV91 recorded). Node v24.18.0, Vitest 4.1.10, tsc 5.9.3. Python is `REPO_ROOT/P/.venv` with the checked-JSON and units CLIs built from the candidate archive (`--locked --offline --release`) into `WT/targets/rv92/cli`.
- **Cargo:** default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, targets under `WT/targets/rv92/`, `TMPDIR=WT/scratch/rv92_u6f/tmp`. The memory guard (PID 5387) ran throughout and was checked before each job.
- **Disclosure, one host-rule slip:** for about 20 seconds at 13:25Z two of RV92's cargo jobs overlapped (a scripted `--release` run of the 1-test extra check, and the same check run by hand in debug). Both were single small test binaries; the guard logged nothing. Every other cargo job ran alone.
- **A transient write through the link:** Vite's config loader creates a temporary bundled-config file in the resolved `node_modules/.vite-temp`, which is `REPO_ROOT/P/node_modules/.vite-temp` through the link, and deletes it. The directory is empty after the runs (its mtime moved to 13:07Z); nothing persisted. Vitest's own cache went to each lane's `apps/desktop/node_modules/.vite`.
- **Not run:** nothing native, at scale or DEC-025; no install; no Git writes. The review harness files (`zz_rv92_*.rs`, `zzRV92*.test.ts`) were added only to RV92's archive copies, after those lanes' acceptance suites had run.
- **When:** 2026-10-04, about 12:56Z to 13:36Z.

## 1. The three-language parity table

**Inputs: 192 probes** (`_run_records/harness/rv92_probes.py`), the same bytes to all three languages:
- the 20 shared cases and the 4 declared-difference entries (×2 fixtures), from the case file at the candidate;
- 100 RV92 successor probes, both modes: receipt tampering (zeroed hash, unsealed and resealed body edits, `{}`/null/array/string/absent receipt), header edits (profile, component version, schema version, `carrier_evidence`, `source_block_recovery`, missing `contract_evidence`), forged hash-consistent edits (`numerical_quality` rewritten to `checks_passed`, `MODEL_INCOMPLETE`, a removed or foreign row token, an extra member, a `contract_evidence` edit), invocation edits (foreign mode, edited model, `{}`), requested-ref variants, a header-only (no `results`) transport form with and without a zeroed hash, relabels to preview-physics-1, physics-1 and legacy 0.1.0 with the receipt and/or tokens, the projected base with a token on the first/middle/last row, another method string, a null method, a null or real receipt, and R-2's notice;
- 64 probes on the 8 other identities (legacy 0.1.0, precision-1, physics-1, preview-physics-1, source-blocks-1, physics-source-1, load-reference-1, load-reference-source-1): plain, a real receipt, null and string receipts, a token on the first and last row, another method string, and the successor id written into the producer.

**Per language** (`zz_rv92_parity.rs`, `rv92_py_parity.py`, `zzRV92Parity.test.ts`): raw dispatch, transport dispatch, standing with the probe's invocation and requested refs, fresh/standing reason, binding of every row and of the headline rows, the classification summary, the AnalysisRun 0.3 build, validate and receipt mutations, the legacy 0.2 builder and the 0.1.0 wrapper, the derivative with its mutations, and the headless derivative-metadata check. **TS registers as the product does:** only a probe with an invocation calls `registerRetainedPrecision`; standing is `retainedPrecisionStanding` (or `unsupported` on the unsupported route), with the model's load cases equal to the requested refs.

**Check against the shared file:** all **20 of 20** shared cases equal their `expected_standing` and `expected_dispatch` in all three languages, and all **4 declared entries × 2 fixtures** equal their per-language expectations.

| Behaviour (probes) | Rust | Python | TypeScript | Status |
|---|---|---|---|---|
| Raw dispatch, milestone both modes, with and without the invocation | ok | ok | header route `retained_preview_physics`, reader ok | agree |
| Raw dispatch, all 192 | — | R = P on 188 | R and T agree on accept/refuse on 189 | 4 G7 texts differ R/P: **inherited, N-4**; 3 are a harness asymmetry (TS validates source-block and joined receipts at registration and AnalysisRun, not at the header) |
| Transport, a successor whose header and receipt hash are intact (58: the unedited milestones and every probe edited elsewhere) | ok | `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` | ok | **declared F-U6b-2** |
| Transport, a tampered transported successor: zeroed `receipt_sha256`, unsealed body edit, `{}` receipt, header-only with a zeroed hash (10) | G1 `RETAINED_PRECISION_RECEIPT_MISMATCH` or G0 | F-U6b-2 | **ok** (header shape only); the unused `validateRetainedPrecisionTransport` gives Rust's code on 10/10 | **UNDECLARED: RV88 N-1** |
| Transport, a successor with a malformed header (24) | reader G0/G2 code | F-U6b-2 | refused, finding `SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED` | agree (TS finding vocabulary) |
| Transport, a successor with `carrier_evidence` (2) | ok | F-U6b-2 | refused | **inherited, N-4** (Rust's preview header ignores `carrier_evidence` at base too) |
| Transport, a non-successor source with a W1 token row (28) | ok (reads no rows) | ok (reads no rows) | `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` | **UNDECLARED: N-2** |
| Downgrade guard, raw (70): a receipt member (object, null, `{}`, string) or a token row on any of the 8 identities, and the relabels | `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`; standing `unsupported` | same | unsupported route, finding `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` | agree, 70/70 |
| Standing (192) | — | R = P on **192/192** | T = R on 182 | 10 **declared I67-F1** (an invalid statement with no invocation, including the header-only form) |
| Binding, row by row (192) | — | R = P on **192/192** | T = R on 171 | 8 **declared I67-F2**; 8 **UNDECLARED, N-3** (a registration that was refused: foreign mode, edited or empty invocation); 5 **N-5** (downgraded source-blocks) |
| Headline binding, milestone | displacement binds; stress `RULE_QUANTITY_BELOW_VERIFIED_FLOOR` | same | same once registered | agree |
| Classification summary (192) | — | R = P on **192/192** | T = R on 184 | 8 **UNDECLARED, N-3** (an unregistered valid statement: TS `[]`, Rust/Python counts) |
| Fresh set | successor id fresh | fresh | fresh on the successor route | agree on the successor. On 83 refused forms TS's source-based check reads not-fresh where Rust's and Python's id check reads fresh: the API shapes differ (an id against a readable source), pre-existing |
| AnalysisRun 0.3 build and validate, successors (32) | — | built, valid, copy byte-equal | same | agree |
| AnalysisRun receipt mutations: dropped, null, zeroed hash, body +1 (32×4) | — | `ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH` | same | agree |
| A base record seeded with a receipt (19) | — | `ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` | same | agree |
| A **resealed** record whose copy has `false`/`true` for `0`/`1` (4) | — | **ok** | `ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH` | **UNDECLARED: S-1** |
| AnalysisRun 0.3 build refusals (102) | — | the dispatch code (`RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`, reader codes) | `SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED` | both refuse; **inherited vocabulary, N-4**. (2 more legacy probes are a harness asymmetry: Python's router builds 0.2, and the harness called TS's 0.3 builder directly.) |
| Legacy 0.2 builder, legacy 0.1.0 + receipt or token | — | `ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN` | same | agree. Producer-carrying sources: pre-existing vocabulary (`HISTORICAL_ANALYSIS_SOURCE_UNSUPPORTED` in TS) |
| 0.1.0 wrapper (Python only) | — | refuses a receipt or token: `ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` | — | D-U6-9 holds |
| Canonical derivative (Rust only), successors (32) | derive and validate; 69 `retained_precision_absolute_verified` disclosures per milestone; receipt byte-equal | — | — | — |
| Derivative receipt mutations (32×4) | dropped, zeroed, seeded, integral float → `RETAINED_PRECISION_RECEIPT_BINDING_MISMATCH`; class swap or message edit → `DISCLOSURE_SEMANTICS_MISMATCH`; a receipt on another identity's document → `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` (21) | — | — | as ruled |
| Headless derivative-metadata check (runner lib.rs:617–623) on a successor derivative | passes (successor table) | — | — | — |
| Reopen (TS only) | — | — | revalidated, copy equal, no registration, `needs_recompute` | as ruled |
| Desktop outputs (TS only) | — | — | `RETAINED-PRECISION-OUTPUT-NOT-YET-AVAILABLE` on every successor-route probe | as ruled |

The full rows are `_run_records/parity/full_table.jsonl.gz`; the counts are `full_summary.txt`.

## 2. Existing behaviour unchanged

**Acceptance suites, RV92's own runs:**

| Suite | Base `7e4f5a51dd` | Candidate `76477534f6` |
|---|---|---|
| result_export, every target | 149 passed | **167 passed**; all 149 base names present and passing; 18 new |
| Python: the 24-file sweep + `test_retained_precision_schema.py` + `test_retained_precision_carriers.py` | the 24 files: 1,742 passed, 30 skipped, **3 failed** (the three branch-order pins broken since the reader fan-in, which U6c repairs) | **1,843 passed, 30 skipped, 0 failed** (ROOT's figure). Per test: no base test is absent; the only changes are those 3 pins, FAILED → PASSED |
| Python: I66's 5 extra consumer files | 428 passed, 2 skipped, 26 subtests | 428 passed, 2 skipped, 26 subtests (identical counts) |
| Desktop Vitest | 3,258/3,258 (135 files) | **3,466/3,466** (138 files): 0 base outcomes changed; 2 base names absent by ruling (the S-1 fresh-set pin, renamed; the D6a probe, flipped to F5); 210 new |
| `tsc --noEmit` | clean | clean |
| runner/headless | 85 passed, 2 failed | 85 passed, 2 failed: **identical per test**. The 2 are the known platform failures (`cli_load_reference_one_both_modes…`, `load_reference_one_actual_solve…`; HANDOFF_2026-09-30_AUDIT_PAUSE:119–122) |
| PP's U1 and U3 pins (`--lib -- retained_wire_tests retained_facade_tests`) against this result_export | — | **41/41** in the merge lane (NUM's PP with U3 1b/1c), including `u3_permitted_path_publishes_the_pinned_successor`, whose precommit runs the candidate (07h) reader |

**RV92's own sweep, against base, in all three languages** (`rv92_sweep_inputs.py`, base-compatible harnesses run in both lanes): every JSON under the base P (records excluded) yields **84 raw envelopes** (69 existing identities plus 15 successor sources in the reader corpus), **16 canonical results documents, 17 AnalysisRun documents and 14 stress-neutral 0.3 packages**. Each raw envelope also gets 5 injected forms (`{receipt_sha256,body}` receipt, null receipt, a token on the first and last row, another method string): 546 inputs.

| | Rust (for_source, metadata, standing ×3, binding, derive+validate; headless metadata for results documents) | Python (dispatch ×2, standing ×2, binding, AnalysisRun build+validate+schema, v0.2, 0.1.0 wrapper; schema and verify for documents; the stress-neutral validator) | TS (route, standing, current, fresh, reason, binding, output refusal, notices, row labels, table, AnalysisRun build+validate, v0.2; verify for records) |
|---|---|---|---|
| 69 existing raw envelopes | identical, except F1's `source_blocks/rejected_stress_range/sparse_interactive` first code (`…SUMMARY_INPUT_RANGE` vs `…STRESS_OUTPUT_RANGE`, varying at base too) | identical | identical |
| 16 results / 17 AnalysisRun / 14 stress-neutral documents | headless check identical | schema verdicts (base vs candidate schemas), record verification and the stress-neutral validator identical | record verification identical |
| Another method string on any row (67 forms) | identical | identical | identical |
| Receipt or W1 token on an existing identity | refused (`RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`; standing `unsupported`) | refused likewise, on every path | refused (unsupported route, finding `…DOWNGRADE_FORBIDDEN`) |

The only changes are the intended refusals of the injected forms. Their side effects, all on already-refused inputs: Python's and TS's `standing_reason` for precision-1 + receipt becomes null (they gate on dispatch; Rust reads the id, pre-existing); TS's load-reference output reason becomes null (other gates refuse); TS's preview row labels disappear for an unsupported envelope (as for any unsupported envelope at base); and N-5.

**Nothing weakened (every removed line read):** the 94 removed lines outside fixtures are rewritten lists and imports, the Python public entry's G0 `_need` (D-U6-1), D6a's "need not name the case" comments and the D6a test and probes flipped to F5 (U6e, ruled), the snapshot count pins (268→277 etc.), the count-7/"last entry" schema pins replaced by stricter count-8/"last two" pins (ruled), the three fresh-set pins extended by the ruled id with exact equality kept (ruled), the old TS standing text (ruled), the TS evidence predicate moved unchanged into `ordinaryCaseEligible`, and the old four-entry TS unit map replaced by the nine-entry table. No check is narrowed. **The flags stay false** (RS retained_precision.rs:4269, PY retained_precision.py:30, TS retainedPrecision.ts:97). **No T6 file is edited** except the two D-U6-8 reservations (`stress_neutral_export.v0.3.schema.json`, `loadReferenceOutputAvailability.ts`); src-tauri, runner, PP and the T6 panels are untouched.

## 3. The receipt's survival, end to end

Step a, per language: Rust derives and validates each milestone; Python and TS build and validate the AnalysisRun (TS after registering through the product route); TS saves the project as JSON text and reopens it. Step b, per language: each carrier's receipt is put back on the raw source and revalidated with the invocation (`_run_records/survival/`).

| Carrier → reader | Rust | Python | TypeScript |
|---|---|---|---|
| Rust derivative | byte-equal, revalidated, `needs_recompute`; the derivative revalidates | same | same (registered: `needs_recompute`, `…NOT_NUMERICALLY_ELIGIBLE`) |
| Python AnalysisRun | same | same; the record validates | same; **the Python record also validates in TS** |
| TS AnalysisRun | same | same | same; the record validates |

All **18/18** combinations (3 carriers × 3 readers × 2 modes) are byte-equal and revalidate. Without the invocation, standing is `needs_recompute` too. The Rust derivative validates under `results.v0.3`, and both AnalysisRun records under the `analysis_run` dispatcher schema. **TS reopen:** findings `[HISTORICAL_INPUT_MANIFEST_MISSING, RETAINED_PRECISION_VALIDATION_REQUIRED]`, the receipt byte-equal, no registration minted, not eligible.

**Every receipt mutation is refused with its ruled code** (table in §1), with one exception: S-1.

## 4. The R-1 carrier interface (C-1 to C-3), against NUM's PP (`886bef131a` content at `3e264e2894`)

- **C-1, partly met.** `envelope()` (PP lib.rs:2232) is documented as "The ordinary base: the publication unless `successor()` is present", which states it. **`into_parts()` (:2254) has no doc comment**: it returns `(envelope, admission)` and silently drops `retained`. Its one product caller is runner/headless lib.rs:774, where D-2 refuses Headless, so no successor exists there today (the plan's F-6). See N-6.
- **C-2, met.** The successor is validated by the accepted reader before commit (:3146, against the actual invocation). **U6's carriers never trust it:** every Rust carrier (`for_source`, metadata, standing, binding, summary, derivative), every Python carrier and TS registration and AnalysisRun call the reader again on the bytes they are given. No carrier takes a `RetainedPublication` or any "already validated" token; they take JSON values. The 41 PP pins pass with the candidate reader, so the precommit and the carriers use the same 07h reader.
- **C-3, inexact.** `into_publication()` (:2243) has no product caller at NUM `3e264e2894` (git grep), consistent with F-1. But **`successor(&self) -> Option<&Value>` (:2235) is a second public path** to the successor (used only in `retained_facade_tests.rs` today). Once G6 grants a permit, a caller could clone it and also publish `envelope()`, which is two publications where R-1 says one. See N-6.
- **U6's carriers are consistent with the interface:** they accept any JSON value and revalidate; nothing in U6 calls the facade.

## 5. Findings

### S-1 (SHOULD-FIX): Python's AnalysisRun receipt-copy check is not exact

- **Where:** P/core/analysis_runs/compatibility.py:694 (`if run.get("retained_precision") != source["retained_precision"]`).
- **Evidence** (`rv92_py_exact.py`, `zzRV92Exact.test.ts`; `_run_records/survival/{py,ts}_exact.json`): Python's `==` treats `False == 0` and `True == 1`. Changing one integer in the record's receipt copy (`body.builds[0].id` 0 → `false`, or `body.builds[1].id` 1 → `true`) and resealing the record's `analysis_run_record` hash (a checksum, not an authentication):
  - **Python:** `validate_analysis_run_v0_3` passes, in both modes;
  - **TS:** `ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH`, in both modes;
  - **the copy no longer revalidates** in either reader (G1 `RETAINED_PRECISION_RECEIPT_MISMATCH`).
  
  The integral-float case (`0` → `0.0`) is accepted by both, and that copy does revalidate (canonically identical; D25), so it is not a defect.
- **Why it matters:** the plan (§1b) and C1:162 require exact equality, and the receipt must be "copied whole". This is an undeclared Python/TS difference on the AnalysisRun carrier. No standing comes from the record's copy, so nothing becomes eligible.
- **Remedy (post-U6f round, I66):** compare canonical bytes, as TS's `same()` does (for example the checked canonical sha256 of both values), or a type-exact recursive equality. Add the bool-for-int and resealed-record cases to the carrier tests.
- **Pre-existing, not U6's:** the same `!=` pattern guards `source_block_recovery` (:684) and `contract_evidence` (:689). It is for their owner, and ROOT should route it.

### N-1 (RV88's, confirmed): TS's transport route checks shape only

10 probes (both modes): a zeroed `receipt_sha256`, an unsealed body edit, a `{}` receipt, and a header-only form with a zeroed hash. Rust refuses each at G1 or G0, Python refuses all transport (F-U6b-2), and TS's header route admits them. Note that the TS header check is only "a non-array object", so even `{}` passes. **TS's own `validateRetainedPrecisionTransport` (retainedPrecision.ts:1332), which no carrier calls, returns Rust's code on all 10.** ROOT's remedy (TS's transport route calls it, and the entry's text is corrected) would remove the difference.

### N-2 (NOTE): TS's header route reads raw rows

- **Where:** numericalResultQuality.ts:71–79 (`retainedPrecisionDowngrade` inside `sourceContract`).
- **Evidence:** 28 probes (a W1 token on the first, middle or last row of any of the 8 identities, the relabels with tokens only, and the two shared token cases). Rust's `for_source_metadata` and Python's `_source_contract(check_receipt=False)` read no rows and admit; TS's header route refuses. This is the "transport" subject exactly as the F-U6b-2 entry defines it.
- **Impact:** none on reliance. TS is stricter, and raw dispatch agrees in all three. But it is a difference outside the four declared entries.
- **Remedy (ROOT to rule):** declare it, either as a fifth entry or folded into N-1's text correction ("TS has one dispatch, which reads rows when present"). Changing code is not recommended.

### N-3 (NOTE): I67-F2's declared scope is narrower than TS's behaviour

- **Binding:** with an invocation whose registration the reader **refused** (foreign mode, an edited invocation model, `{}`; 8 probes), TS refuses every row with `RULE_QUANTITY_NOT_COVERED`. Rust and Python bind by validated class, because binding validates without the invocation. The declared entry covers only "no invocation".
- **Summary:** for an **unregistered valid** statement (8 probes), TS's `classificationSummary` is `[]` and Rust's and Python's give the counts. No declared entry has a `summary` subject.
- **Impact:** fail-closed; TS's precheck is display-only, and Tauri's binding runs only after the standing gate.
- **Remedy (ROOT to rule):** widen I67-F2 to "no valid registration (none, or refused)" and add the `summary` subject, with shared probes for the foreign-mode case; or align the languages.

### N-4 (NOTE): differences inherited from the base carriers are undeclared

All were shown at base `7e4f5a51dd` (`_run_records/sweep/extra_*`):
- **(a) The G7 dispatch text.** Both carriers forward the G7 detail, as ruled. For the projected base, Rust's preview validator says `SOURCE_PREVIEW_PHYSICS_BLOCKED_ENVELOPE` or `…EVIDENCE_SHAPE`, while Python's says `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID: …`. The readers agree on `(G7, SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID)`. 4 successor probes.
- **(b) `carrier_evidence` at the header.** Rust's header dispatch does not read it, for preview-physics-1 too, so the Rust successor transport admits it; TS (and Python) refuse it. 2 probes.
- **(c) The AnalysisRun builders' refusal vocabulary.** TS's 0.3 builder throws `SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED` for any non-current source, while Python names the dispatch cause (102 probes). TS's 0.2 builder answers `HISTORICAL_ANALYSIS_SOURCE_UNSUPPORTED` for producer-carrying sources (178).

**Remedy (ROOT to rule):** one sentence in the case file stating that differences inherited from the base carriers are not U6 differences, and that G7 parity compares the reader's `(gate, code)`. Otherwise any future shared case that pins a G7 dispatch text will split Rust from Python.

### N-5 (NOTE): TS's source-blocks summary binding refusal on downgraded envelopes

- **Where:** knownSemanticLimitations.ts:97–98 and :121. TS's source-blocks binding branch is gated on `sourceContract(source) === "source_blocks"`. Rust's (semantic_contract.rs:537–548) and Python's are gated on the producer id.
- **Evidence:** U6's downgrade guard makes a source-blocks envelope carrying a receipt or token "unsupported". TS therefore no longer refuses its summary row (5 probes; 14 forms × 4 in the TS sweep), while Rust and Python still return `RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE`.
- **Impact:** none on reliance; `runRuleChecks` refuses the unsupported source first (not fresh). The mechanism is pre-existing: any unsupported source-blocks envelope behaved so at base.
- **Remedy:** declare it under N-4, or key TS's branch on the producer id as Rust does.

### N-6 (NOTE, PP and U3, not U6): the R-1 interface's documentation and exposure

- **C-1:** add "ordinary base only; a successor is dropped" to `into_parts()` (PP lib.rs:2254).
- **C-3:** make `successor()` (:2235) `pub(crate)` or `#[cfg(test)]`, or document it as non-publishing, so that `into_publication()` is the only public successor path before G6 grants a permit.
- **Routing:** U3 grant 2 (I61).

### N-7 (NOTE): cost, item 6 (F7 and RV88 U6a N-2)

Measured on the sparse milestone (`zz_rv92_extra.rs`; parity `binding_ms`):

| | Debug | Release |
|---|---|---|
| One accepted-reader validation | 40 ms | **4.3 ms** |
| `rule_binding_refusal` on all 98 rows (98 validations) | 3.7 s | **0.38 s** |
| `derive_document` + `validate_document` (4 validations) | 0.40 s | 39 ms |
| Python, binding all rows | 2.8–4.1 s (≈35–40 ms per row) | — |
| TS, binding all rows (registered classes, no revalidation) | 0.18 s | — |

- **Scaling:** binding cost is (bound rows) × (validation ∝ rows); binding everything is quadratic.
- **For the milestone and U7:** acceptable. A rule pack binds a few rows; live milestone output is 98–99 rows; F-1 means no product caller binds a successor before native activation.
- **Memoize before native activation, or before any caller binds a statement beyond the milestone size:** validate once per envelope per rule-check call (for example `retained_binding_refusals(envelope) -> map`), and Python likewise. Add a cost pin. It is not needed before U7.

### N-8 (NOTE): the U7 preconditions, item 7

**Recorded so far:** RV91 N-2 (= RV88 U6d S-1): bind TS successor standing to the live native registration and model. RV91 N-5: the T6 panels refuse a successor by an explicit gate.

**These are complete for TS standing. The whole diff adds:**
1. **S-1** fixed before U7 builds AnalysisRun records of live successors.
2. **N-1** (TS transport) repaired, and **N-2 to N-5** declared or repaired, so that the shared file again lists every difference.
3. **U7's live-output rerun** (plan §6: "U7 or U9 replaces the fixture input with a live `into_publication()` output") should rerun **all three** languages' carrier tests and RV92's survival chain, not only U6a and U6b. TS's and the shared file's fixtures are the same pinned bytes.
4. **N-6** before G6's permit makes `successor()` reachable with a real successor.
5. **The post-U7 comparison basis:** TS's `numericalResultStanding` reports an eligible successor as status `integrity_checked` (numericalResultQuality.ts:143), the same mapping as every other TS route (:184), while `retainedPrecisionStanding` returns the token `numerically_eligible`, as Rust and Python do. U6 cannot exercise it (the flags are held). U7's three-language check should compare the token, and pin the mapping.
6. **N-7's memoization**, before native activation (not before U7).

## 6. Limits

- **Post-U7 eligibility** is not reachable (the flags are false). RV92 did not simulate it; the unit reviews' seam tests cover it.
- **TS raw dispatch** for the other identities is compared at the header route only (3 R/T raw rows in §1 are this harness asymmetry).
- **The non-successor class-code check** in `validate_document` was not isolated. Editing a disclosure's `reason_code` trips the derivative's own hash (`DERIVATIVE_HASH_MISMATCH`) first; the results schema refuses the class codes outside the successor branch (U6c).
- **RV88's U6a N-3** (the guard-test scope) was not re-reviewed; it is already routed.
- **Inputs:** the fixtures are PP's pinned producer outputs (D-U6-5), not native Current evidence.

## SHA256SUMS

`SHA256SUMS` covers this report and every file in `_run_records/`. Paths are placeholders only (WT, REPO_ROOT, NUM, P, S).

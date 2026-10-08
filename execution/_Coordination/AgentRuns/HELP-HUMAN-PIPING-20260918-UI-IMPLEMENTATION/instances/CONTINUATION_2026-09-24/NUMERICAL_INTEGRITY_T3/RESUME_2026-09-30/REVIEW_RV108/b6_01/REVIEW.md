# RV108: independent complete-diff review of B6 (the reader items)

RV108 is a TASK (Type 2), an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0) under `R/BRIEFS/RV108_B6_REVIEW.md` (sha256 `95e447d7b7970f86ced59d1e27c991894a4c4b78f2b21b97229b90bf1465c5af`, verified before use). ROOT is the return path. RV108 did not delegate and wrote none of the change. I read NUM's `AGENTS.md` and `agents/AGENT_TASK.md` first, formed my own view from the diff and my own oracles, and read I83's RETURN only afterwards, apart from what the brief points me to for the host (its "Node and the wasm assets" section, and its suite script for the Python file set I83 ran).

**Placeholders:** `WT`, `NUM`, `P`, `T`, `R`, `RR`, `VENV` as in the dispatch; `S` = `WT/scratch/rv108_b6_01`; `DT` = `P/apps/desktop/src`; `RE` = `P/core/reporting/result_export`; `NMS` = the shared `node_modules` that `WT/t6-outputs/P/node_modules` links to; `BASE` = main `bfb26596bf`; `HEAD` = `a7de2a918f` (`a7de2a918ff3332133ce1f9d06e205fb87aae3d9`); `CAND` = my `git archive` copy of HEAD, `BASEC` = of BASE (both at `WT/rv108/{cand,base}`, now deleted).

**Inputs read:** the B6 brief `R/BRIEFS/B6_READER_ITEMS.md` (`a3634160…`); RR's section "I86's SW probe accepted; I83's B6 return verified and ruled; RV108 and I87 dispatched" (RR sha256 at read time `4e4c5ae9…`); `R/I83/b6_01/RETURN.md` (`4e856c31…`, read after my own review); `R/REVIEW_RV92/u6f_01/` (the tampered set); `R/REVIEW_RV78/*` (item 5); `T/IMPLEMENTATION/T6S/CHANGE_RECORD.md` (`1e77547c…`, decision 8).

## Verdict: **PASS**

**Counts: 0 BLOCKING, 0 SHOULD-FIX, 7 NOTE.**

B6 does what its brief and ROOT's rulings say, inside its fence, with no weakening:
- **TS's G7 header refusal** now carries the base header code. Over 984 own single-defect, hash-consistent probes on 7 bases, TS and Rust agree on every header refusal except the declared `carrier_evidence` class; TS and Python agree on every header refusal except a Python defect class that predates B6 (N1). Every statement TS admitted at BASE reads byte-identically at HEAD, and every TS change at HEAD is a G7 refusal whose code moves from `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` to a base header code (631 of my 1,032 probes; 9 of the 339 corpus entries).
- **Python's transport validator** agrees with Rust's and TS's on gate and code for all 14 tampered-receipt forms (RV92's five plus nine of mine), 98 probes. Python's raw path reads identically at BASE and HEAD over the whole 07m corpus and my 1,032 probes. T6S decision 8 holds.
- **07m** is append-only except 277's TS expectation. Entries 286–293 are single hash-consistent edits; all three readers give each its expected code, and all 339 corpus entries meet each reader's own expectation.
- **Suites, BASE → HEAD, test by test:** Python 1,896 → 1,922 (+26 added, 0 removed, 0 changed; 30 skipped in both); Rust `result_export` 177 → 180 (+3, 0 removed); vitest 3,612 → 3,620 (+13, −5: the five removals are three renames and the two dropped F-U6b-2 forms); tsc clean on both.
- **Mutants:** 30 of mine (11 TS reader, 10 Python, 3 Rust `src`, 6 data), beyond I83's 26, plus N0 and three HEAD-tests-at-BASE runs. 23 are killed by the suites. Seven TS sub-condition mutants survive the suites and are killed by my probes (N5). DM5 survives only in Rust, by design (Rust reads its own field). The new tests fail at BASE exactly where they should.

The notes are pre-existing undeclared differences my probes found beyond I83's (N1–N4; ROOT to route), a test-strength gap (N5), precision of two claims (N6) and one inaccuracy in I83's item-5 account that does not change ROOT's ruling (N7).

## Findings

| ID | Severity | Path | Evidence | Remedy |
|---|---|---|---|---|
| N1 | NOTE (undeclared difference, pre-existing; ROOT to route) | `P/core/analysis_runs/compatibility.py` `_source_contract` (the `… not in {…}` membership tests for `numerical_quality.status` and each case's `solve_quality`, `structural_status`, `model_matrix_fidelity`, `accuracy_evidence`) | A **list- or dict-valued** enum is unhashable, so Python's header check raises `TypeError`, not its header code. Python's reader then reports its G7 fallback `SOURCE_PREVIEW_PHYSICS_INVALID` (no detail), raw and in the new transport validator; Rust and TS (HEAD) report `SOURCE_NUMERICAL_CASE_INVALID` or `SOURCE_NUMERICAL_QUALITY_INVALID`. 104 of 984 probes (90 case, 14 quality; 7 bases; with and without invocation). At BASE this was a three-way disagreement (TS `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`). Not declared; the removed N-3 sentence would not have described it either (it named Python `SOURCE_NUMERICAL_CASE_INVALID`). It also makes TS's new doc comment ("so the G7 code agrees across the languages…") and I83 §2.1 ("the case rule is Python's and Rust's exactly") overclaim. Direct check: `_source_contract` on the projected base with `structural_status: []` raises `TypeError: unhashable type: 'list'`. Outside the successor, the same `TypeError` escapes callers that catch `ValueError` only, e.g. the v0.3 packager's validator on a preview-physics-1 package view (unchanged by B6) (`evidence/n1_typeerror_check.txt`, `evidence/three_way_probes.txt`, `evidence/raw_agreement_summary.txt`). | ROOT to rule: either repair Python (guard each membership test with `isinstance(value, str)` so the header `ValueError` is raised; no declaration then needed, and TS's comment becomes true), or declare it with per-reader entries in 07n alongside I83 §7 item 6 (B1's SC). |
| N2 | NOTE (undeclared difference, pre-existing; route) | `P/core/analysis_runs/retained_precision.py` `_g5_ordinary` (via the G5 fallback) | A numerical_quality case **without `solve_quality`**, made hash-consistent: Python G5 `RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH` (a `KeyError` caught by the fail-closed fallback), Rust and TS G5 `RETAINED_PRECISION_ATTEMPT_MISMATCH`. 10 probes. Outside B6's change. | Route with N1 (declare, or have Python report G5 ATTEMPT for a missing quality field). |
| N3 | NOTE (undeclared difference, pre-existing; widen I83 §7 item 6's routing) | Rust base evidence validators (`RE/src/preview_physics_evidence.rs`) against Python's and TS's | The 06b-kind G7 code split is wider than §7 item 6's `limitations` case: a key added to `contract_evidence` (Rust `SOURCE_PREVIEW_PHYSICS_EVIDENCE_SHAPE`) and a non-array `combination_gates` (Rust `SOURCE_PREVIEW_PHYSICS_ARRAY_INVALID`), Python and TS `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`; 7 probes each, hash-consistent. No per-reader entry pins either. On transport, item 6's input (a different non-empty `limitations` list) is refused by Python and TS and admitted by Rust's header-only check; the scope sentence covers that only if its "evidence content" includes the formulation limitations. | When B1's SC adds item 6's per-reader entry, cover these two classes too (one entry each), or state the class in one scope sentence, and name the transport side of item 6. |
| N4 | NOTE (undeclared transport accept/refuse difference, pre-existing; route) | `DT/features/results/retainedPrecision.ts` `projection` (`Object.hasOwn(row, …)` on a null row) | A transported successor whose `results` array **contains `null`** (transport reads no rows): TS refuses at G7 `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID` (a `TypeError` in `projection`, caught as G7's default); Rust admits (its transport reads no rows) and Python now admits. 6 probes (3 bases × 2 forms). At BASE, Python refused every transport, so TS and Python agreed by accident and Rust differed; B6 moves Python to Rust's side. Never eligible either way. | Guard non-object rows in TS's `projection` (as Python's `_transport_g7` does with `type(row) is dict`), or declare. |
| N5 | NOTE (test strength) | `DT/features/results/retainedPrecision.ts` `baseHeaderCode`; corpus 07m | Seven of my TS mutants of `baseHeaderCode`'s sub-conditions **survive the whole retained TS suite** (RT1–RT4, RT6–RT8): dropping the `value_representation`, quality exact-keys, formulation exact-keys, limitations-element or `evidence_refs`-array checks; narrowing `contract_evidence` to null/absent; and changing the `carrier_evidence` branch's code. My probes kill each (2–10 differing probes). The corpus pins one input per branch, as ROOT's ruling 2 records, so a regression to TS's old code in any other sub-condition would pass all three suites. | Append a few entries in B1's 07n (a quality extra member, `value_representation`, a formulation extra member, `limitations: [""]`, `evidence_refs: "x"`, `contract_evidence: []`, and a per-reader `carrier_evidence` entry with Rust's `SOURCE_PREVIEW_PHYSICS_FOREIGN_METHOD_EVIDENCE`). |
| N6 | NOTE (precision of claims) | `P/core/analysis_runs/retained_precision.py` `validate_retained_precision_transport` docstring; the carrier case file's transport scope sentence | (a) On G0–G2 Python's transport validator is the exact twin (98 tampered probes, same gate and code in all three). At the base step it is the **union** of the other two: Rust checks the base header only (labelled G2), TS the preview transport metadata only, Python both. Reader-level codes therefore differ for header and evidence defects (TS's reader-level transport admits an invalid quality case, which TS's carrier refuses through `sourceContract`). (b) The scope sentence's "Rust and Python the reader's G0 code or their base header code" is inexact for N1's inputs (Python: the reader's G7 fallback). Only accept/refuse is compared there, so no pin is wrong. | Wording only, at the next touch: "the reader's G0–G2 code or its base step's code". |
| N7 | NOTE (item 5 account) | `R/I83/b6_01/RETURN.md` §5.1 | The identification is right: RR:9461's RV78-N1 is `carried_artefacts_01`'s N1 (I66 PLAN:196 and :308 give it that content and cascade), and the B6 row's label joins it with `reader_confirm_04`'s rehash rule, fixed in 07e and confirmed by `reader_confirm_05`. One inaccuracy: "RV78 wrote seven reviews, and each has its own N1". `reader_confirm_05` has 0 NOTE and `reader_confirm_06` no findings, so five reviews have an N1 (the table lists those five). | None needed; it does not change ROOT's ruling (b). |

## 1. TS's G7 header refusal (item 2 and its widening)

### 1.1 The change, read against both base readers

`baseHeaderCode` (TS) runs only after `sourceContract(projection)` refuses, and returns codes in Python's `_source_contract` order. Branch by branch, against Python `_source_contract` and Rust `semantic_contract::for_source_metadata` (which `for_source` runs first):

| Branch (TS) | Python | Rust | Reachable at G7? |
|---|---|---|---|
| `carrier_evidence` → `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` | same code, same position | **not checked** for preview-physics-1; `validate_preview_physics_evidence` then gives `SOURCE_PREVIEW_PHYSICS_FOREIGN_METHOD_EVIDENCE` | Yes (declared inherited: "Rust's header dispatch ignoring carrier_evidence") |
| schema version → `SOURCE_SCHEMA_VERSION_UNSUPPORTED` | same (0.1.0 → `LEGACY_SOURCE_METADATA_CONTRADICTION`) | same | **No**: G0 requires `0.2.0` in all three |
| producer → `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` | same | same | Only an extra producer member (G0 reads named fields only); all three agree |
| `contract_evidence` not an object → `…_EVIDENCE_REQUIRED` | same, before `source_block_recovery` | same code, **after** `source_block_recovery` | Yes; order matters only with both defects (T9, known) |
| `source_block_recovery` → `SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN` | same | same | Yes |
| quality → `SOURCE_NUMERICAL_QUALITY_INVALID` | same, except list/dict `status` (N1) | same | Yes |
| case rule → `SOURCE_NUMERICAL_CASE_INVALID` | same, except list/dict enums (N1) | same | Yes (`basis_ref` defects stop at G3; most `solve_quality` defects at G5) |
| formulation → `SOURCE_FORMULATION_BASIS_UNSUPPORTED` | same | same | Yes (`profile_id` is fixed by the projection) |
| fallback → `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` | — | — | **No**: every conjunct of `sourceContract`'s preview route has a branch above; the projection removes the receipt and every row token, so the downgrade guard cannot fire. Its only effect is T10's (the `carrier_evidence` branch returns the same code) |

`baseHeaderCode` cannot throw: every dereference is guarded by the check before it (my RT4 shows what an unguarded one does).

### 1.2 My probe set (beyond I83's 32)

`S/scripts/gen_probes.py` (recorded as `evidence/scripts/`) builds **984** probes on 7 bases: `ordinary_prepared_synthetic`, `two_case_preparation_failure_synthetic` (selected + unavailable), `two_case_synthetic`, `u8_l0_isolated_node_sparse_interactive`, 07j's not_required must-pass entry, and both milestone successors. Each is one edit, rehashed by the shared format's "all" (implemented in my generator with the reader's hash primitive only). Families: each case enum × 8 values (string, empty, null, int, bool, `[]`, `{}`, a list holding a valid value), `evidence_refs` × 8, case members added or removed, the quality level (status × 6, the three constants, members added or removed), formulation (`limitations` × 8, members), `contract_evidence` × 7, `source_block_recovery` × 5, `carrier_evidence` × 4, a producer member, evidence content, and 25 transport forms. Batch 2 (30) adds blocked envelopes, two-defect pairs and edge keys; batch 3 (18) malformed raw rows on transport. Each probe ran through five entry points per language (raw with and without invocation, the transport validator, raw and transport dispatch) in Python HEAD and BASE, Rust, and TS HEAD and BASE.

**Raw path at HEAD** (`evidence/raw_agreement_summary.txt`): all three agree on **821 of 984** (306 at BASE). The 163 disagreements:

| Probes | Python | Rust | TS (HEAD) | Declared? |
|---|---|---|---|---|
| 90 | G7 `SOURCE_PREVIEW_PHYSICS_INVALID` | G7 `SOURCE_NUMERICAL_CASE_INVALID` | G7 `SOURCE_NUMERICAL_CASE_INVALID` | **No** (N1) |
| 28 | G7 `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` | G7 `SOURCE_PREVIEW_PHYSICS_FOREIGN_METHOD_EVIDENCE` | as Python | Yes, inherited (scope) |
| 14 | G7 `SOURCE_PREVIEW_PHYSICS_INVALID` | G7 `SOURCE_NUMERICAL_QUALITY_INVALID` | as Rust | **No** (N1) |
| 10 | G5 `…PRODUCT_ATTEMPT_MISMATCH` | G5 `…ATTEMPT_MISMATCH` | as Rust | **No** (N2) |
| 7 | G7 `…EVIDENCE_INVALID` | G7 `…FORMULATION_BASIS` | as Python | No; known, I83 §7 item 6, routed |
| 7 | G7 `…EVIDENCE_INVALID` | G7 `…EVIDENCE_SHAPE` | as Python | **No** (N3) |
| 7 | G7 `…EVIDENCE_INVALID` | G7 `…ARRAY_INVALID` | as Python | **No** (N3) |

Every header branch that TS changed agrees with Rust on all its probes, and with Python on all but N1's: `contract_evidence` 49/49, `source_block_recovery` 35/35, formulation 63/63, quality 98/112, case 270/360, producer 7/7. Batch 2 confirms the two-defect order (TS follows Python: `contract_evidence` before `source_block_recovery`) and that `carrier_evidence` with a case defect reads `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` in Python and TS (Rust gives `SOURCE_NUMERICAL_CASE_INVALID` there, since its header ignores `carrier_evidence`).

### 1.3 Nothing TS admitted at BASE changes

`evidence/ts_base_vs_head_probes.txt` and `evidence/corpus_three_readers.txt`: TS at BASE and HEAD over the same inputs (my 1,032 probes and the 339 07m corpus entries), five entry points each, admitted results compared by sha256 of a key-sorted serialization.
- Every admitted result is identical: among my probes, 30 raw admissions (each with and without invocation), 695 reader-transport and 79 carrier-transport admissions; in the corpus, 45 raw admissions (17 cases, 28 must-pass entries) and 287 transport admissions.
- The only changes are on the two raw entry points: **631** of my probes (619 in batch 1, 12 in batch 2) and **9** corpus entries (277, 286–293) move from G7 `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` to a G7 base header code. Nothing else moves: no other gate, no detail, no transport, no route.

## 2. Python's transport validator (item 3)

- **The twin, on G0–G2** (`evidence/transport_tamper_table.txt`): RV92's five tampered forms and nine of mine (unsealed and resealed body edits, `{}`, `null`, an empty body, a zeroed hash with and without rows, receipt version 2, a wrong policy, −0, an extra receipt member, an upper-case hash), 7 bases each: **98 of 98** give the same gate and code in the three reader-level validators. The resealed body edit, the header-only form and an edited raw row are admitted by all three, as transport defines.
- **At the base step** the three differ by design (N6): Python checks the header and the preview transport metadata; Rust the header only (gate G2, I83 §7 item 7); TS the preview transport metadata only. At the carrier transport (what the case file compares; `evidence/carrier_transport_accept_refuse.txt`), Python and TS agree on accept/refuse on 1,026 of 1,032 probes; the other six are N4's. Rust admits 70 that Python refuses: `carrier_evidence` (35, declared inherited), `contract_evidence` content (28, declared by the scope sentence) and a different non-empty `limitations` list (14; see N3).
- **The raw path is unchanged** (`evidence/python_base_vs_head_probes.txt`, `evidence/corpus_three_readers.txt`, `evidence/batches23_base_vs_head.txt`): Python BASE and HEAD read identically on the raw entry points over all 339 07m entries and all 1,032 probes (0 differences). The transport dispatch changes only from the old blanket `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`.
- **`compatibility.py` maps truthfully:** `_retained_transport` raises `ValueError(detail or code)`, as `_retained_validation` and Rust's `retained_error` do: G0–G2 codes bare, a G7 base failure with its full text (my PM2 and PM5 are killed by the test that pins this).
- **T6S decision 8 holds** (`evidence/decision8.txt`): at BASE and HEAD, `_transport_contract`, `validate_stress_neutral_export_package_v0_3` (with and without the source envelope) refuse a successor package view, with and without the receipt and as the whole successor, with `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`; `build_…_v0_3` still gives `SN-SOURCE-METHOD-UNSUPPORTED`. At HEAD the refusal comes from the reader's G0 (the metadata view never carries `retained_precision`), behind which `SUPPORTED_METHODS` still excludes the successor. `test_stress_neutral_packager_still_refuses_the_successor` pins it.

## 3. The corpus and the slices (items 1 and 4)

- **07m** (`evidence/corpus_append_only.txt`): sha256 `c21112fd…` (5,521,277 B), BASE 07l `5ac13296…`. Top-level keys, `cases`, `must_pass`, `d37`, `arithmetic`, `provenance` and `version` are identical; mutations 0–285 are identical except 277's `expected_by_reader.typescript` (`SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` → `SOURCE_NUMERICAL_CASE_INVALID`); 286–293 are appended. Both files re-serialize byte for byte with `indent=2`.
- **286–293:** each one edit with `rehash: "all"`, no `expected_by_reader`. All three readers give each its expected code (my own concrete entries; `evidence/corpus_three_readers.txt`). Over the whole corpus, 339 entries × 3 readers meet each reader's own expectation (0 failures).
- **The slices run in all three harnesses and check ids:** Rust `snapshot_07k_mutation_outcomes` and `snapshot_07m_mutation_outcomes` (ids asserted in commit C); Python `test_snapshot_07k_g7_probe_slice` and `test_snapshot_07m_g7_header_slice`; TS `07k and 07m slices (B6)`. DM2 (277 renamed) is killed in all three.
- **Python reads `expected_by_reader.python`:** `_expected` falls back to `expected`, as Rust's `expected_for` and TS's `?? m.expected`. PM10 (reading Rust's field) is killed by entry 139; I83's C1 and C2 cover a wrong Python field.

## 4. The declarations

- **Five entries remain,** each real at HEAD: I67-F1, I67-F2 and D-U7-4 (standing and binding), F5 (shared semantics) and RV92-N2-N5 (transport token rows). Each language's carrier test asserts its own expectation per form and passes at HEAD (§8).
- **The scope sentences** are exact at HEAD for the inputs they name, with N6(b)'s imprecision. The blocked-envelope sentence holds (batch 2: `MODEL_INCOMPLETE`, `MECHANICS_FAILED`, `NOT_RUN` on three bases: Python and TS G7 `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`, Rust `SOURCE_PREVIEW_PHYSICS_BLOCKED_ENVELOPE`, as the sentence states).
- **The removed sentences no longer describe the code:** TS gives `SOURCE_NUMERICAL_CASE_INVALID` for N-3's class (277 and my 270 agreeing probes), and Python admits the unedited successor's transport (all 7 unedited bases, both milestones among them).
- **Undeclared differences:** the two known ones (I83 §7 item 6; T9's two-defect order, confirmed by batch 2), plus N1–N4.

## 5. The fence

- 11 files, +535/−71, three commits over BASE; none under PP, a D1 crate's `src` or the schemas; the only fixtures are the corpus and the carrier case file (ROOT's ruling 3). Main's later changes (to `025c1cf326`) touch none of the 11 files.
- **RV107 A1-N-10:** B6's hunks in `retainedPrecisionIntegration.test.tsx` are at 630–647 and 682–691 (the declared-difference pins). T6S's hunks are at 90 and 889–932 (the T6 panel block). Disjoint. A1-N-10 can close.

## 6. Weakening

I read all 39 removed lines of the non-fixture diff (`evidence/removed_lines.txt`). Each is a grown count (286 → 294), a narrowed slice bound (`[278:]` → `[278:286]`), a changed expectation with its reason (N-3 aligned; F-U6b-2 removed), a replaced pin (N-3 positive → negative; the transport scope phrase), a renamed title, the replaced G7 line, Python's replaced transport refusal, or `raw`-guarded G1 lines. No assertion is dropped.

**The five vitest removals** (`evidence/vitest_base_vs_head.txt`):
1. `RV94 N-3: …` → `RV94 N-3 (B6, PLAN decision 11): …`: same three inputs, new code, and the probe pin strengthened to all three readers' fields.
2. `07l: 17 cases, 286 mutations …` → `… (07m: 294 mutations) …`: count grown, eligibility counts unchanged.
3. and 4. The two `F-U6b-2:python_refuses_transport / unedited` forms: TS's expectation (`ok`) stays pinned by `an untampered transport passes, with or without raw rows` (both modes).
5. `are exactly the six ruled entries…` → `five`.

## 7. Tests and mutants

`evidence/mutants.jsonl`, `evidence/mutants_summary.txt`; harness `evidence/scripts/mutants.py`. Each mutant is one edit to a copy of HEAD, run in its lanes, then restored from CAND: `ts` (the two retained vitest files), `py` (Python's carrier file plus the contract file's corpus and slice tests), `rs` (`--test retained_precision_contract --test retained_precision_carriers`), and `probe` (my TS probe harness on 232 probes, compared with HEAD's outputs).

| Id | Mutant | TS | Python | Rust | TS probes (232) | Outcome |
|---|---|---|---|---|---|---|
| N0 | no mutation (head) | passes | passes | passes | 0 differ | all lanes pass |
| RT1 | TS: drop the value_representation sub-check of the quality branch | survives | — | — | 4 probes differ | **survives the suite**; killed by RV108's probes |
| RT2 | TS: formulation branch without its exact-keys sub-check | survives | — | — | 2 probes differ | **survives the suite**; killed by RV108's probes |
| RT3 | TS: contract_evidence branch weakened to null/absent only | survives | — | — | 10 probes differ | **survives the suite**; killed by RV108's probes |
| RT4 | TS: case branch without the evidence_refs array sub-check | survives | — | — | 9 probes differ | **survives the suite**; killed by RV108's probes |
| RT5 | TS: G7 reads the unprojected successor (baseHeaderCode(s)) | killed (12) | — | — | — | killed |
| RT6 | TS: carrier_evidence branch returns Rust's base code | survives | — | — | 8 probes differ | **survives the suite**; killed by RV108's probes |
| RT7 | TS: formulation branch without the limitations-element sub-check | survives | — | — | 6 probes differ | **survives the suite**; killed by RV108's probes |
| RT8 | TS: quality branch without its exact-keys sub-check | survives | — | — | 2 probes differ | **survives the suite**; killed by RV108's probes |
| RT9 | TS: case branch without its exact-keys sub-check | killed (2) | — | — | — | killed |
| RT10 | TS: accuracy_evidence vocabulary widened by 'estimated' | killed (3) | — | — | — | killed |
| RT11 | TS: QUALITY_STATUSES widened by 'estimated' | killed (2) | — | — | — | killed |
| PM1 | Py: transport G7 code is always the fallback | — | killed (2) | — | — | killed |
| PM2 | Py: transport G7 drops the base text (detail None) | — | killed (2) | — | — | killed |
| PM3 | Py: transport projection keeps the receipt member | — | killed (6) | — | — | killed |
| PM4 | Py: transport G7 runs the raw base dispatch | — | killed (2) | — | — | killed |
| PM5 | Py: transport G7 lets the base ValueError escape to the fallback | — | killed (2) | — | — | killed |
| PM6 | Py: G1 row-shape condition inverted (transport requires rows, raw skips them) | — | killed (8) | — | — | killed |
| PM7 | Py: raw path checks the publication digest only with an invocation | — | killed (6) | — | — | killed |
| PM8 | Py: transport dispatch reports the base preview-physics-1 contract | — | killed (4) | — | — | killed |
| PM9 | Py: transport carrier swallows reader refusals | — | killed (18) | — | — | killed |
| PM10 | Py test helper: _expected reads Rust's per-reader field | — | killed (1) | — | — | killed |
| DM1 | Case file: the transport scope sentence reverted to name Python F-U6b-2's code | killed (1) | killed (1) | killed (1) | — | killed |
| DM2 | Corpus: entry 277 renamed | killed (2) | killed (2) | killed (1) | — | killed |
| DM3 | Corpus: entry 290 re-expected as a case defect | killed (2) | killed (2) | killed (2) | — | killed |
| DM4 | Corpus: entry 293 duplicated at the end | killed (1) | killed (1) | killed (18) | — | killed |
| DM5 | Corpus: 277's TS expectation reverted to the base value | killed (3) | killed (1) | survives | — | killed |
| DM6 | Case file: the F-U6b-2 declared difference restored from base | killed (1) | killed (1) | killed (1) | — | killed |
| BT | Head tests with BASE's TS reader | killed (12) | — | — | — | head tests fail at base (expected) |
| BP | Head tests with BASE's Python reader and carrier | — | killed (18) | — | — | head tests fail at base (expected) |
| BC | Head tests with BASE's corpus and case file | killed (7) | killed (3) | killed (19) | — | head tests fail at base (expected) |
| RM1 | Rust: for_source_metadata admits a successor without the reader's transport checks | — | — | killed (2) | — | killed |
| RM2 | Rust: validate_transport_metadata skips G1 | — | — | killed (2) | — | killed |
| RM3 | Rust: base header reports a bad case as SOURCE_NUMERICAL_QUALITY_INVALID | — | — | killed (3) | — | killed |

**The new tests fail at BASE where they should:**
- **BT** (HEAD's tests, BASE's TS reader): exactly 12 fail: entries 277 and 286–293, the N-3 test and both slices.
- **BP** (HEAD's tests, BASE's Python reader and carrier): exactly 18 fail: the 16 transport tests and the dispatch pin (2). The contract slices pass, as they should: Python's raw path did not change.
- **BC** (HEAD's tests on BASE's corpus and case file): TS 7, Python 3, Rust 19 fail: the counts, the slices, the declared-difference set and the scope pins.

## 8. The suites, BASE against HEAD, test by test

Both ran in my archive copies (`BASEC`, `CAND`); Python with my own builds of the two CLI authorities (`OPENPIPESTRESS_CHECKED_JSON_BIN`, `OPENPIPESTRESS_UNITS_BIN`) and under the lock; vitest and tsc under the lock; cargo through `t3_cargo.sh`.

| Suite | BASE | HEAD | Test by test |
|---|---|---|---|
| Python: I83's 27-file set (`evidence/scripts/suites.sh`) | 1,896 passed, 30 skipped | 1,922 passed, 30 skipped | +26 (16 transport tests, 8 corpus entries, 2 slices); 0 removed; 0 changed outcomes |
| Rust `result_export`, all targets, `--no-fail-fast` | 177 passed | 180 passed | +3 (`b6_transport_refuses_rv92_tampered_successors`, `snapshot_07k_mutation_outcomes`, `snapshot_07m_mutation_outcomes`); 0 removed; 0 changed. The one warning (`derived` unused) is at BASE too |
| vitest, whole desktop suite (141 files) | 3,612 passed | 3,620 passed | +13, −5 (§6); 0 changed outcomes |
| `tsc --noEmit -p tsconfig.json` | rc 0, no output | rc 0, no output | — |

Toolchains: cargo/rustc 1.97.1, node 24.18.0, vitest 4.1.10, tsc 5.9.3, Python 3.13.14, pytest 9.1.1.

## 9. Item 5's checkpoint (RV78-N1)

I83's account is accurate in what matters (N7 is the only error, and it changes nothing):
- RR:9461 ("RV78-N1 is deferred to wider F2a, because of its re-pin cascade"), with RR:9174's routing, is `carried_artefacts_01`'s N1: the retained table does not bind the projection and work policies, the 20B/60B limits, the method token or the canonicalization profile; all three readers enforce them as G0 constants. I66's PLAN (`:196`, `:308`) names the same content and the same cascade.
- The B6 row's "RV78-N1's rehash-index rule (RR:9461)" joins it with `reader_confirm_04`'s N1, which 07e fixed and `reader_confirm_05` §3 confirmed. ROOT's label correction is right.
- §5.2's pin list is right: the table hash `c74742ce…` appears in PP (`retained_memory.rs:952` `reviewed_inputs`, its law tests), RE's `src` (two files), both v0.3 schemas, the readers' and carriers' constants and two Python schema tests, and in no corpus, case file, milestone or golden. So option (a) needs R8-stop files, as ROOT's ruling says, and (b) costs nothing now.

## 10. For ROOT

1. **N1–N4 (undeclared differences my probes found):** route them, with I83's two, to B1's SC, or have N1 (and, if wanted, N4) repaired. N1 is a defect class (a `TypeError` in Python's header check), not a per-language code by design, so a two-line Python guard would remove it without a declaration.
2. **N5:** whether B1's 07n should pin the sub-conditions of each header branch.
3. Nothing in this review blocks the PR.

## 11. Host, disclosures and cleanup

- **Cargo:** every cargo job went through `WT/tools/t3_cargo.sh` with `--locked --offline` (`CARGO_BUILD_JOBS=4`, no RUSTFLAGS), into fresh targets `WT/targets/rv108-b6/{checked-json,units-authority,rx,rx-probe,rx-mut}`.
- **The lock:** the Python and vitest suites and tsc ran under `lockf -k WT/guard/cargo_job.lock`. Light single-file runs ran without it: the TS probe harness (1–25 s), my Python probe scripts, and the mutant lanes for TS (two files) and Python (about 1.5 min). Other T3 jobs (RV109's) held the lock between mine; I waited and killed none of theirs.
- **One stop of my own job:** my first Python suite pair ran in copies without `P/execution/`, and BASE gave 19 failures, all `FileNotFoundError` for the PKG-15 handoff fixtures. I stopped my own suite runner (its pytest, `lockf` and shell; no other process) during HEAD's run, added `P/execution/` without `_Coordination/` to both copies (identical at BASE and HEAD), and reran both. The figures above are from the rerun; the first BASE log is kept as `evidence/py_base_without_execution.tail`.
- **Waits:** one wait per job, each loop also ending when the job's process had gone; none left running.
- **Node and the wasm assets:** `node_modules` links to NMS in my copies, and the eight wasm assets copied (not built) from `WT/sweep-skewpin/P/apps/desktop/public/` (sha256 equal to I71's eight, as I83 recorded). Both lived only in my copies and were deleted with them.
- **Not done:** no DEC-025, evidence sweep, native or solver job, install or Git write. Reads used `GIT_OPTIONAL_LOCKS=0` and `git archive`.
- **Scratch and temp:** `S` only, with `TMPDIR` and pytest's basetemp in it. The agent host writes its own task output under the system temp directory; nothing I ran did.
- **Cleanup:** the five copies under `WT/rv108/` (BASEC, CAND, two probe copies and the mutant copy, with their `node_modules` links and wasm copies) and `WT/targets/rv108-b6/` are deleted. NMS is untouched apart from its pre-existing empty `.vite-temp/`, whose mtime moved. `WT/b6` and the branch are untouched (`a7de2a918f`, clean). `S` keeps only small working files; its probe inputs, outputs and logs are deleted after their records were copied here. No process of mine is running.

## 12. Records in this folder

- `REVIEW.md`, and `SHA256SUMS` over every other file.
- `evidence/`: the tables cited above; `mutants.jsonl` and `mutants_summary.txt`; the suites' test-by-test comparisons and tails (`suites_tails.txt`); `commits.txt`, `diffstat.txt`, `fence.txt`, `removed_lines.txt`; gzipped per-language probe outputs (`outputs/`); and `scripts/` (the generators, the three harnesses, the comparison scripts, the mutant harness, the suite runner and the path sanitizer), with machine paths replaced by placeholders. The probe inputs are not recorded (136 MB); the generators rebuild them from HEAD's corpus and milestones, and `evidence/inputs.sha256` gives their sha256.

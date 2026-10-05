# I61 RETURN: U7 slice A (switch inventory, eligibility oracle, 07i expectations, N-6 text)

**Status: complete, with no stop.** This slice is records only: no tree was edited and nothing was built.
- **Basis:** the U7 branch `codex/piping-f2a-u7-20261004` at HEAD `071eec5c04` (WT/f2a-u7). I read it through `git show HEAD:` because I67's uncommitted TS edits are in that worktree.
- **N-6 is written against the memory head `7f07a2f7b4`,** where slice P lands after the merge (§6). The reader and carrier files at both heads are byte-identical (checked for the three readers and the PY contract test).
- **ROOT's rulings applied:** D-U7-1 to D-U7-6 (NUM `4d6fa4e6f9`). D-U7-3 is amended: documentation only, `successor()` stays `pub`.
- **Placeholders:**
  - P = `projects/chirality-piping`;
  - PY = P/core/analysis_runs;
  - RS = P/core/reporting/result_export;
  - TS = P/apps/desktop/src;
  - PP = P/core/product_physics/src.
- **Line numbers** are at `071eec5c04` unless marked "memory".

**Run facts.**
- **Role:** TASK Type 2 under ROOT, with no descendants.
- **Time:** 2026-10-04, about 16:45Z to 16:57Z.
- **Host:** memguard 5387 was running. No cargo, no Git writes. Git reads used `GIT_OPTIONAL_LOCKS=0`. The oracle is stdlib Python.
- **One disclosure.** One snapshot command first piped a file to `/tmp/x` and deleted it at once. That file was the committed `tests/test_retained_precision_contract.py`, a repository file with no secrets. That was the only write outside WT/scratch/i61_u7_slice_a_01/ and this folder.

## 1. The switch inventory

### 1.1 The three flags and their gates (slice F)

| Reader | Flag | Gate it feeds |
|---|---|---|
| Python | PY/retained_precision.py:30 `_IMPLEMENTATION_COMPLETE = False` → `True` | :1709 `eligible = _IMPLEMENTATION_COMPLETE and invocation is not None and snapshot["status"]["mechanics"]=="MECHANICS_SOLVED" and all(c["status"] in ("selected","not_required") ...)`; :1710 `standing: "eligible"` or `"needs_recompute"` |
| Rust | RS/src/retained_precision.rs:4269 `const IMPLEMENTATION_COMPLETE: bool = false;` → `true` | :4309–4314, the same conjunction into `Validation.numerical_eligible`. `validate_transport_metadata` (:4339) stays `false` |
| TypeScript | TS/features/results/retainedPrecision.ts:97 `const SUMMARY_COVERAGE_COMPLETE = false;` → `true` | :1327, the same conjunction; :1328 `standing: 'eligible'` or `'needs_recompute'`. Transport (:1332ff) is never eligible |

**On the registered build:** RS's line is reached by PP's precommit (`retained_w1` → `validate`), which ignores `numerical_eligible`. Keep every RS edit line-neutral (§1.2), and run Pass B on the U7 head (PLAN row 13).

### 1.2 Stale comments and docs to update with the flip

Keep the RS ones line-neutral: same line count, so U4's TEXT line keys hold.

| Where | Text now | Proposed |
|---|---|---|
| PY/retained_precision.py:27–29 | "…this flag gates eligibility only… It stays false until U7; every gate runs regardless." | "…It is on since U7 (D-U7-5); every gate runs regardless." |
| PY/retained_precision.py:1591–1593 | "…a valid statement reads needs_recompute while it is false." | "…a valid invocation-bound statement of a solved model whose cases are selected or not_required reads eligible." |
| PY/retained_precision.py:1598 | "Unqualified development checks; eligibility remains disabled with the API." | "The ordered checks behind the public entry (D-U6-1)." |
| PY/compatibility.py:322 | "While the reader's eligibility is held, needs_recompute." | Drop that sentence |
| RS/src/retained_precision.rs:4267–4268 | "Eligibility stays held: the reader is unaccepted until the snapshot-07 repair / wave (review RV78-RV81) is confirmed…" | Two lines: "Eligibility is on (U7, D-U7-5): an invocation-bound statement of a solved / model whose cases are selected or not_required is eligible; every gate runs." |
| RS/src/semantic_contract.rs:581–582 | "While the reader's eligibility is held this always / yields `needs_recompute`." | Same two lines: "The reader's eligibility is on since U7; the other conjuncts / still apply." |
| TS/features/results/retainedPrecision.ts:95–96 | "I57 summary-coverage checks are implemented against shared snapshot 04 only. / Eligibility stays held until snapshot 05 controls and independent review." | "Eligibility is on since U7 (D-U7-5); every gate runs regardless." |
| TS/features/results/retainedPrecision.ts:1307 | "pending summary coverage keeps eligibility held" | "eligibility as in C1:160" |
| TS/features/results/retainedPrecisionStanding.ts:13–14 | "While the reader's eligibility is held (until U7) the result is never better than `needs_recompute`;" | Drop that clause (I67 is editing this file in slice T) |

### 1.3 Every eligibility pin and its post-U7 value (slice F)

The **source** column gives where the new value comes from:
- **07i** = the expectation the §4 patch adds;
- **oracle** = `oracle_post_u7.json`.

**Unchanged** means the input has no invocation, is refused, is a transport, or is non-retained.

**Python** (`P/tests/test_retained_precision_contract.py`)

| Line | Now | After U7 | Source |
|---|---|---|---|
| :147–148 (`test_complete_synthetic_draft_control_is_not_qualification`, bases with their invocation) | `numerical_eligible is False`; `standing == "needs_recompute"` | `== fixture["expected"]["numerical_eligible"]` / `["standing"]`: 13 true, 2 false | 07i |
| :150, :153 (no invocation) | False | unchanged | — |
| :193 docstring; :200 (`test_public_entry_equals_the_draft…`, every passing corpus entry) | False / `needs_recompute` | With its invocation, a base's `expected` or an entry's `expected_eligibility`; `:no-invocation` labels False / `needs_recompute`. `_corpus_entries` (:165) yields that expectation | 07i |
| :211 (`test_public_entry_on_the_real_milestone_receipts`) | False / `needs_recompute` for both | With the invocation: True / `"eligible"`; without: unchanged | oracle `milestone` |
| :245 (the refused-coefficient control on `cases[0]`, with its invocation) | False | True (the edit touches `product_attempts` only; the base is eligible) | oracle (base) |
| :280 (must-pass) | False | `== entry["expected_eligibility"]["numerical_eligible"]`: 13 true, 10 false | 07i |
| :624, :631 (`test_snapshot_07_counts_and_entry_format`) | "Snapshot 07h"; allowed keys | "07i"; add `"expected_eligibility"` to the allowed key set | 07i |

**Python** (`P/tests/test_retained_precision_carriers.py`)

| Line | Now | After U7 | Source |
|---|---|---|---|
| :7 module doc; :107 test name | "standing is needs_recompute… U7 owns the switch" | Reword: eligible with the invocation | — |
| :110 (no context) | `needs_recompute` | unchanged | — |
| :111 (with the invocation) | `needs_recompute` | `"numerically_eligible"` | oracle `milestone.with_invocation` |
| :113 | `not validation["numerical_eligible"]` | `validation["numerical_eligible"]` | oracle |
| :219–224 (the shared cases) | reads `expected_standing` | unchanged code; 2 cases change through the patch | patch |
| :236 docstring | "the held withheld count (eligibility is held until U7)" | "the not-Current withheld count (these forms carry no invocation)". Values unchanged | — |
| :269 (scope) | 2 substrings | add `"no carrier authenticates producer origin"` | patch (D-U7-6) |
| :434 (`classification_summary(source, invocation)`) | `== expected` (withheld 97) | `== current` (withheld = absolute, 69) | oracle token |
| :435 (no invocation) | `== expected` | unchanged | — |

**Rust** (`RS/tests/retained_precision_contract.rs`)

| Line | Now | After U7 | Source |
|---|---|---|---|
| :200 (fn `complete_synthetic_controls_keep_eligibility_held`), :207–208 | `!got.numerical_eligible`, "remains held" | Rename; `got.numerical_eligible == case["expected"]["numerical_eligible"]` | 07i |
| :240, :244, :264 | unbound, transport, mutation message | unchanged | — |
| :852 (`shared_must_pass_entries_validate`) | `let same = !got.numerical_eligible && …` | `got.numerical_eligible == entry["expected_eligibility"]["numerical_eligible"].as_bool().unwrap() && …` | 07i |
| :963 doc; :1039 (`publicly_consistent_coverage_attestations_are_not_rejected`, bases `ordinary_prepared_synthetic` and `ordinary_prepared_no_data_synthetic`) | "Eligibility stays held"; `assert!(!got.numerical_eligible)` | `assert!(got.numerical_eligible)` (the edits touch verification attestations only) | oracle (bases) |

**Rust** (`RS/tests/retained_precision_carriers.rs`)

| Line | Now | After U7 | Source |
|---|---|---|---|
| :6 module doc; :183 fn name | "held… U7 owns the switch" | Reword | — |
| :186 (no context) | `needs_recompute` | unchanged | — |
| :187–191 (with the invocation) | `"needs_recompute"`, "eligibility held" | `"numerically_eligible"` | oracle |
| :194 | `!validation.numerical_eligible` | `validation.numerical_eligible` | oracle |
| :314 (the derivative round trip, with the invocation) | `assert!(!after.numerical_eligible)` | `assert!(after.numerical_eligible)` | oracle |
| :318 (transport) | never eligible | unchanged | — |
| :548–549 (`classification_summary(.., Some(&m.invocation))`) | `== expected` (withheld 97); comment "Not Current (eligibility held)" | `== current` (withheld = absolute) | oracle token |
| :550 (None) | `== expected` | unchanged | — |
| :691 (the shared cases) | reads `expected_standing` | unchanged code; 2 cases change through the patch | patch |
| :707 doc ("the held withheld count") | — | "the not-Current withheld count" | — |
| :758 (scope) | substrings | add the D-U7-6 substring | patch |

**PP (memory branch; it lands after the merge)**

| Line | Now | After U7 | Source |
|---|---|---|---|
| PP/retained_wire_tests.rs:122 (memory) (`u1_milestone_successor_both_modes`) | `invocation_bound && !numerical_eligible, "eligibility off"` | `invocation_bound && numerical_eligible` | oracle `milestone` |

PP's facade and grant-2 tests and runner/headless have no successor eligibility pins. Runner `result_envelope_binding.rs:649`, `preview_physics_admission.rs:94/194`, `physics_source_connected.rs` and `load_reference_route_tests.rs` concern non-retained identities.

**TypeScript** (`TS/features/results/retainedPrecision.test.ts`)

| Line | Now | After U7 | Source |
|---|---|---|---|
| :164 (bases) | `toEqual(c.expected)` | unchanged code; 13 bases change through 07i | 07i |
| :171, :174 | unbound, transport | unchanged | — |
| :195 (must-pass) | `toBe(false)` | `toBe(m.expected_eligibility.numerical_eligible)` | 07i |
| :287 (`firstFailure`, 44 call sites, any accepted probe) | `expect(r.numerical_eligible).toBe(false)` | `expect(r.numerical_eligible).toBe(c160(source, invocation))`, where `c160` is R1 below written in the test (invocation present, `MECHANICS_SOLVED`, every case `selected` or `not_required`). It is an independent expectation, not the reader | oracle rule R1 |
| :926 (the milestone with its invocation) | false / `'needs_recompute'` | true / `'eligible'` | oracle |

**TypeScript** (`TS/features/results/retainedPrecisionIntegration.test.tsx`)

| Line | Now | After U7 | Source |
|---|---|---|---|
| :9–17 header doc | "eligibility stays held until U7… `u7.simulate`…" | The flag is on; `u7.simulate` now only reclassifies rows (not_covered, no absolute) | — |
| :199 (registered, with the invocation) | `numerical_eligible false` | `true` | oracle |
| :201–203 | `{status:"needs_recompute", eligible:false, findings:[NOT_NUMERICALLY_ELIGIBLE]}`; `"needs_recompute"` | `{contract:"retained_preview_physics", status:"integrity_checked", eligible:true, findings:[]}`; `"numerically_eligible"` | oracle; RV92 N-8 item 5 (the status mapping pinned beside the token) |
| :204 | `retainedPrecisionInvocation(...)` is null | equals the captured invocation | — |
| :444 describe "(eligibility held)"; :450 | rule checks reject `NOT_NUMERICALLY_ELIGIBLE` | The registered successor passes the gate and reaches the backend with `sourceBlockInvocation` (as the simulated path :399–409 asserts). :451 (a copy) is unchanged | oracle |
| :488 (`classificationSummary(received, model)`) | withheld 97 | withheld 69 | oracle token |
| :581–582 (scope) | 2 regexes | add the D-U7-6 regex | patch |
| :695, :701 (the shared cases) | `toBe(c.expected_standing)`; `eligible toBe(false)` | :695 unchanged code (patch); :701 → `toBe(c.expected_standing === "numerically_eligible")` | patch |
| :172, :218, :282–283, :297, :306, :591, :624, :665, :810 | — | unchanged (no invocation, refused, copied or transport) | — |

**TypeScript** (other files)

| Line | Now | After U7 |
|---|---|---|
| TS/services/retainedPrecisionAnalysisRun.test.ts:87 | `original.numerical_eligible toBe(false)` | `toBe(true)` (the milestone with its invocation) |
| TS/features/results/numericalResultQuality.test.ts | — | **no retained pins** (its two "retained" hits are fixture text) |

### 1.4 Consumers whose behaviour changes

| Consumer | Where | After U7 |
|---|---|---|
| **Standing** | PY compatibility.py:318 (`_retained_standing_from`), :482/:488; RS semantic_contract.rs:584, :647, :731/:736; TS retainedPrecisionStanding.ts:134, :150 | Gives `numerically_eligible` for an eligible statement, bound and requested |
| **TS status** | TS numericalResultQuality.ts:154–158 | `integrity_checked` and `eligible: true` |
| **Summaries** | PY compatibility.py:348; RS semantic_contract.rs:664; TS retainedPrecisionStanding.ts:175, :187 | With an eligible invocation, the withheld count is absolute plus not_covered only |
| **TS rule checks** | TS ruleCheckService.ts:128–135 | A registered successor passes the eligibility gate and is sent with `retainedPrecisionInvocation`. Only reachable with a registered native capture; none exists in product (F-1) |
| **TS notices** | TS knownSemanticLimitations.ts:168 calls `classificationSummary(source)` **without the model** | So it keeps showing the not-Current withheld count after U7. **A finding for slice T:** pass the model, or state that the notice is conservative |
| **Runner binding** | P/core/runner/headless/src/result_envelope_binding.rs:258 (requires `numerically_eligible`) | Would pass for an attested headless successor, but none exists (D-2) |
| **TS export panels** | TS ResultExportPanel.tsx:12–16; StressNeutralExportPanel.tsx:83–86 | The eligibility conjunct now passes for a successor. **Slice T's explicit gate (N-5) must be in before slice F** |
| **No change** | RS derivative.rs:107/:115 (`derive_document`, class-based disclosures); row binding (PY :336/:576, RS :537); transport routes; the stress-neutral packager (`P/core/handoff/stress_neutral/package_v0_3.py:32`, no retained method); D-U6-9 | — |

### 1.5 The D-U7-6 sentences

- **The carrier scope** (`P/fixtures/results/retained_precision_carrier_cases.json`, `scope`): appended by the §4 patch. The existing substrings are kept.

  > A numerically_eligible standing is a property of the supplied statement and its actual invocation, as the accepted reader checks them: no carrier authenticates producer origin, and none claims that a registered producer made the statement (ROOT_RULINGS_V1 "RV93 on U3 grant 2: PASS; RV89 confirms the final basis re-qualified; U7 planned and ruled", D-U7-6).

  Each language asserts the substring "no carrier authenticates producer origin" (PY carriers :269, RS carriers :758, TS integration :581–582).
- **The note** (same file): the sentence "Eligibility is held until U7, so no case expects numerically_eligible." is replaced, in the patch, by:

  > Since U7 the readers' eligibility is switched on: an unedited milestone case with its fixture invocation and the invocation's requested cases expects numerically_eligible, and every other case keeps its expectation.
- **The TS standing text** (TS retainedPrecisionStanding.ts:197, `tail`; for I67 in slice T or F). After "…against the actual invocation and the requested cases.", insert:

  > The reader checks these bytes and their invocation; it does not establish which producer made them.

  The existing substrings that tests read (for example :307's "validated by the retained-precision reader without an invocation") are kept.
- **The reader docs:** RS retained_precision.rs:4270–4271 already says "Hashes bind the supplied statements; they do not establish producer origin." Python's `validate_retained_precision` docstring (:1591) gains the same sentence in the §1.2 rewrite. Keep that rewrite to three lines.

## 2. The oracle (`eligibility_oracle.py`, stdlib only)

**What it reads.** It imports no product, reader or test code. It reads only the shared JSON files:
- the 07h corpus;
- the U6 carrier case file and its 5 fixtures;
- the two milestone successor files, with their invocations.

**Its rules,** derived from C1:160 and :162 (R/I32/f2a_wire_c1/WIRE_CONTRACT.md), D2 §4.9.4 (DESIGN_STANDING/DESIGN.md:595–608) and D-U6-1:
- **R0, G-status.** Taken from the shared expectation, never computed:
  - bases and must-pass entries pass;
  - mutations fail;
  - a carrier case passes only as an unedited milestone, since an unrehashed edit fails G1 or G8 and relabelled or legacy sources are not successors.
- **R1, reader `numerical_eligible`.** The U7 flag, AND G passes, AND an invocation is supplied, AND `MECHANICS_SOLVED`, AND every receipt case is `selected` or `not_required`. The reader label is `eligible` iff R1 holds, else `needs_recompute`.
- **R2, carrier token.** `unsupported` if G fails. Otherwise `numerically_eligible` iff R1 holds, the requested refs equal the receipt's case basis_refs in order, and every not_required case is ordinarily eligible (D2: checks_passed, passive_model_basis, represented_equations_retained, accuracy_evidence not_claimed or reference_verified, non-empty evidence refs resolving to unique ids). Otherwise `needs_recompute`.
- **R3, requested refs.** The invocation model's load cases, or a carrier case's `requested`.
- **R4, edits.** Set or remove on JSON paths (source, invocation, after-rehash). No hash is recomputed, and no rule reads one.

**The self-check.** With the flag held (`--flag false`), the oracle reproduces 07h's 15 base `expected` objects exactly (0 mismatches). It also reproduces every carrier case's current `expected_standing`, except the 2 that U7 moves.

**Post-U7 results** (`oracle_post_u7.json`):

| Set | Result |
|---|---|
| Corpus bases, with their invocation | **13 of 15 eligible.** The two `unavailable`-case bases are not: `two_case_facade_after_certificate_synthetic`, `two_case_preparation_failure_synthetic`. Every eligible base's carrier token is `numerically_eligible` |
| Corpus bases, without an invocation | all `needs_recompute` |
| Must-pass entries | **13 of 23 eligible.** The 10 with an unavailable case are not: `cert_failed_before_summary`, `lane_k_failed`, `lane_source_failed`, `maxima_abandoned`, `prefix_captured`, `cert_failed_after_summary_storage`, `prefix_unattached_old_operand_attested`, `values_failed_separate_completion`, `aliases_abandoned`, `bind_rows_abandoned` |
| Mutations | 277, all `unsupported` |
| U6 carrier cases | 2 of 20 change, `sparse_interactive:invocation` and `dense_scrutiny:invocation`, from `needs_recompute` to `numerically_eligible`. The other 18 keep their expectation. **The 5 declared-difference entries are unchanged:** their standing forms are refused or have no invocation, their summary forms have no invocation, and binding is class-based |
| Live milestone, both modes | With its invocation: `numerically_eligible`. Without: `needs_recompute`. With other requested refs: `needs_recompute` |

## 3. Slice F's application order

1. **Apply the §4 patch** (`patch -p1` from the repository root's `projects/chirality-piping`). It reproduces the staged files byte for byte (checked):
   - 07i: sha256 `1e53ea9c…`;
   - the carrier case file: sha256 `f20a7db0…`.
2. **Flip the three flags** (§1.1).
3. **Rewrite the stale texts** (§1.2).
4. **Change the pins in §1.3.** No pin is deleted; each moves from a constant to its 07i, case-file or oracle value.
5. **Add the scope assertions** (§1.5).

**D-U7-4** (TS N-2's declared difference) goes into `declared_differences` with slice T/F's TS change. It is not in this patch.

## 4. 07i, the records-side patch (`u7_07i_expectations.patch`, 221 changed lines)

**`fixtures/results/retained_precision_cases.json`:**
- **Each base's existing `expected`** `{invocation_bound, numerical_eligible, standing}` takes the oracle's value: 13 become `true` / `"eligible"`.
- **Each must-pass entry** gains `"expected_eligibility": {invocation_bound, numerical_eligible, standing}`, placed after `"expected"`: 13 true.
- **Unchanged (asserted):** cases, mutations, the d37 table, counts (15 / 277 / 23) and every other byte.
- **Left unchanged:** the bases' `qualification` string ("DRAFT: … public API intentionally rejects."). It is now stale wording. ROOT may want it reworded in 07i, but it is outside this patch.

**`fixtures/results/retained_precision_carrier_cases.json`:**
- 2 `expected_standing` values change;
- the note sentence is replaced (§1.5);
- the D-U7-6 scope sentence is appended;
- everything else is unchanged (asserted).

## 5. Notes for ROOT

- **The TS notice** at knownSemanticLimitations.ts:168 does not pass the model, so its withheld count stays not-Current after U7 (§1.4). Slice T should decide whether to pass the model.
- **The base `qualification` strings** are stale (§4).
- **Line-neutral RS comment rewrites** (§1.2) keep TEXT keys. Pass B on the U7 head is still required (PLAN row 13).

## 6. N-6: the documentation text (slice P; `n6_docs_line_neutral.diff`)

**Against the memory head `7f07a2f7b4`:** one inline `#[doc]` on each of `PP/lib.rs:2235` (`successor()`, C-3) and `:2254` (`into_parts()`, C-1). Both functions stay `pub` (D-U7-3 as amended). The file's line count is unchanged (24,333).

```rust
#[doc = "R-1 (RV92 N-6, C-3): a borrowing view of the same successor that `into_publication()` publishes, not a second publication. A caller that publishes this successor does not also publish `envelope()`."] pub fn successor(&self) -> Option<&serde_json::Value> {
#[doc = "R-1 (RV92 N-6, C-1): the ordinary base and the admission report only. A successor that a permitted invocation produced is dropped here, never published; take it with `into_publication()`."] pub fn into_parts(self) -> (MechanicsEnvelope, Option<RetainedAdmissionReport>) {
```

`successor()` keeps its two existing `///` lines; rustdoc joins the doc attributes. It was not compiled here (no builds). Slice P compiles it, and Pass B covers it.

## Records

All paths are placeholders, with no machine paths.
- `eligibility_oracle.py`;
- `oracle_post_u7.json`, `oracle_pre_u7.json`;
- `build_07i.py`;
- `u7_07i_expectations.patch`;
- `n6_docs_line_neutral.diff`;
- `staged_sha256.txt`;
- `SHA256SUMS`.

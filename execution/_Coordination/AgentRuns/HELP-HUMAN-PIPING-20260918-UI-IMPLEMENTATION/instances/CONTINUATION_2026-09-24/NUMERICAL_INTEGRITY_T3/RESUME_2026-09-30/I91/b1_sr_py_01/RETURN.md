# I91 B1 SR-PY: the Python reader (R-D38, F-1 text B, G8 per case, G5's `not_required` rule), with RV108 N1, N2 and N6(a)

TASK (Type 2), I91 (I-PY), for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-07 UTC.

**Briefs, verified before use:** `R/BRIEFS/B1_COMMON.md` sha256 `2d170307516b98c40e8aa2dcf352cf11a13dbdd29aeddd78a4c39f3f15eb2c75` and `R/BRIEFS/B1_SR_PY.md` sha256 `2a902874278ed5890a9179f9faa1ba8b065baf1c7cabeb5c25ccd2ffb180322e`, both matching. I read `NUM/AGENTS.md` and `NUM/agents/AGENT_TASK.md` first.

**Specification read:** PLAN_v2 (`R/I84/b1_plan_01/PLAN_v2.md`, sha256 `c85786b7…9be0`, as B1_COMMON names it) §1, §2.4, §4 and §8; DESIGN_v2 (`R/I78/b0_contract_01/DESIGN_v2.md`, sha256 `5933b90b8c323199050caec4ce0f8bc178f4ea7a33a9808c8a72f769c62d1114`) §1.2, §2 and §3.2–§3.4; `R/REVIEW_RV108/b6_01/REVIEW.md` (N1, N2, N6) with its `evidence/n1_typeerror_check.txt` and `scripts/decision8.py`; RR from "#1107 merged…" to "I89's follow-up verified…; SR-PY dispatched as I91", including "SR-PY prepared; the N1 guard in `_source_contract` ruled in".

**Wider reading, recorded:** to align first failures I read, without editing, the Rust reader's `g8`, `g5_ordinary` and `builds_seen` check and the TS reader's `invocationBinding` and `seenBuilds` check at I1; and SR-RS's (I90) two reader commits `d7c76de43f` and `a4eab1dd01` in `WT/b1-r` (`git show`, read-only, no file of theirs touched). My expectations come from DESIGN_v2; I90's Rust test expectations were a cross-check afterwards (§4.6).

**Placeholders:** WT, NUM, P, PY, T, R, RR, VENV as in the dispatch; `S` = `WT/scratch/i91_b1_sr_py`; `BASE` = I1 `262bd687f0`; `HEAD` = `75132d2673`.

## 0. Summary

- **The census holds (R5).** Over 07m (sha256 `c21112fd…6807`: 17 bases, 294 mutations, 28 must-pass), each entry read raw with its invocation, raw without, and by the transport validator: **0 of 1,017 outcomes differ**, base against the aligned reader, and 0 again with N1 and N2 added. **No 07m entry is moved by N1 or N2.**
- **D38's list (§3):** one PY check assumed a prepared source has a Run: `_g5_stages`' entered-native branch. It is relaxed to (4b). No PY check assumes a Call. Every other source-, Run-, Call-, Group-, Build- or order-touching check is listed with why it does not apply.
- **One missing check found and added (§3.2; for ROOT, §8 item 1):** PY admitted a Build that no record built (an orphan; it even read *eligible*), which Rust (`builds_seen`) and TS (`seenBuilds`) refuse at G5 WORK. That is (4b)'s Build conjunct, so PY could not enforce (4b) without it. It is in the alignment commit, separately mutated, and changes no 07m outcome.
- **G8 (F-1 text B):** a loop over every case in request order: the requested mode, P1, P2–P4, all `PREPARATION_MISMATCH`. **G5:** PY's `not_required` rule was already DESIGN's; it is pinned on W2-published cases.
- **RV108 N1** (`_source_contract`, the five enum membership tests only), **N2** (`_g5_ordinary`) and **N6(a)** (a docstring) are done, with tests.
- **The N1 differential (§5):** 10,307 inputs, base against head. Every one of the 960 inputs that raised `TypeError` in `_source_contract` at base now gives the base header code as a `ValueError`; **every other input reads exactly as before**. At the reader, the only other changes are N1's (1,188: the G7 fallback becomes the header code) and N2's (48). Nothing unexplained.
- **Suites (§6):** I83's 27-file Python set, base against head, test by test: 1,922 → 1,934 passed (30 skipped in both); **+12 added (my new tests), 0 removed, 0 changed**.
- **Mutants (§7):** 30, each killed by an assertion: the D38 restore and each (4b) conjunct, the Build check, Rust's three `not_required` conjuncts restored, the per-case G8 loop (two scopes), the requested mode, P1 (three forms), P2, P3, P4 and Rust's old parity rule, N1's guard on each of the five fields, and N2. Head's new tests on BASE's code: 11 of 12 fail (the `not_required` test passes there, as DESIGN says PY already had the rule).

## 1. Head and commits

Branch `codex/piping-t3-b1-p-20261007` in `WT/b1-p`, **head `75132d2673…`** (full SHA in `_run_records/diff/commits.txt`), six commits over I1 `262bd687f0`, clean. 4 files, +415/−11:

| Commit | What |
|---|---|
| `a514f4ca74` | Align the reader: R-D38 (4b) in `_g5_stages`; every Build referenced by its building record (`_g5_native_checks`); G8's per-case loop (requested mode, P1–P4) |
| `a29949e238` | RV108 N2: a quality case without `solve_quality` reads as no verdict in `_g5_ordinary` |
| `36c0980e4d` | RV108 N1: string guards on `_source_contract`'s five enum membership tests (`compatibility.py`) |
| `5cd239e58e` | RV108 N6(a): `validate_retained_precision_transport`'s docstring |
| `de7d9b9736` | Tests: 6 contract tests (n-case receipts, N2), 3 carrier tests × 2 modes (N1) |
| `75132d2673` | The N1 carrier tests' ValueError-only caller reports an escaping exception by name, so the pre-N1 `TypeError` fails an equality assertion rather than erroring out of the test (§7) |

**The fence.** `PY`; `P/core/analysis_runs/compatibility.py` inside `_source_contract`, only its two enum-test conditions (each membership test gains `not isinstance(value, str) or` in front of it, in place) and two comment lines; `P/tests/test_retained_precision_contract.py`; `P/tests/test_retained_precision_carriers.py`. Nothing else: no fixture, no corpus, no schema, no other base reader, nothing of SP's, SA's or SR-RS's. File sha256 at HEAD: PY `135ff6a2…0a2b`, `compatibility.py` `ec122dd1…ba77`, contract test `9f6964a4…fa0d`, carrier test `c9565e66…3da1`; CORPUS unchanged (`c21112fd…6807`).

## 2. The cascade census (R5)

**Method** (`_run_records/scripts/census.py`). Every 07m entry is built with the contract test's own `apply_entry` semantics (rehash "all") and read through three entry points: `validate_retained_precision` with the entry's invocation, without one, and `validate_retained_precision_transport`. An outcome is `("pass", sha256 of the whole sorted result, classifications included)` or `("refuse", gate, code, detail)`. One process per tree: BASE (a `git archive` of I1) and HEAD (the worktree).

| Run | Reader | Outcomes | Differ from BASE |
|---|---|---|---|
| `census/head_c1.json` | After `a514f4ca74` (the alignment alone) | 1,017 (339 entries × 3) | **0** |
| `census/head_code.json` | After `5cd239e58e` (with N2, N1 and N6(a)) | 1,017 | **0** |

BASE's own tally: bases and must-pass pass at all three entry points; the 294 mutations refuse raw with their invocation (272 refuse and 22 pass without one; 52 refuse and 242 pass in transport). **The entries N1 and N2 would move: none exist in 07m.** No 07m entry reaches either change (none reads an unhashable enum in `_source_contract` or a missing verdict in `_g5_ordinary`).

## 3. D38's obligation `[r1: N-6]`

### 3.1 Every PY check that touches a source, Run, Call, Group, Build or the execution order

Line numbers are HEAD's. "(4b) case" is a case whose attempt failed native before any Run, beside a registered prepared source.

| # | Check (PY at HEAD) | Gate | Assumes a Call or Run for a prepared source? | Disposition |
|---|---|---|---|---|
| 1 | `_validate_draft`: a **selected** case's `source_identity_sha256` (`:1719`) | G1 | No; selected cases only | Does not apply |
| 2 | each source's `preparation.sha256` from its attempt (`:1721`) | G1 | No | Does not apply |
| 3 | every source's `body_membership` non-empty (D29, `:1742`) | G3 | No | Does not apply |
| 4 | attempts' owners; a `complete` old list equals the source's members (`:1752`) | G3 | No | Does not apply |
| 5 | a proof's projection rows and coverage roster | G3 | No; a (4b) attempt has no proof | Does not apply |
| 6 | `execution_order` is a bijection with the cases **that have a Run** (`:1771`) | G3 | No: keyed on cases' Runs, not on sources | Does not apply. It is (4b)'s `execution_order` conjunct (m4) |
| 7 | `_g5_native_checks`: every Call position names a Run of its owner case, that case's `source_ref`, and a source owned by that case (`:611`–`:617`) | G5 | No: it iterates Calls; no rule requires every source to be in a Call | Does not apply. It is (4b)'s Call conjunct (m7) |
| 8 | Groups: their Call exists, their sources are unique and in that Call; the per-call grouping; each group source's stiffness (`:739` on) | G5 | No: it iterates Groups | Does not apply. It is (4b)'s Group conjunct (m7) |
| 9 | Builds: ids, work; a referenced Build's group, slot, origin | G5 WORK | No | Does not apply. **The rule that every Build is referenced by its building record was missing; added (§3.2)** |
| 10 | `charged` equals the sum over Runs | G5 WORK | No; a (4b) case adds nothing | Does not apply |
| 11 | `_g5_ordinary`'s `source_decline` (needs `source_ref` and `run` null) | G5 | No | Does not apply (a (4b) case has no decline) |
| 12 | D4e: `run_ref` non-null ⇒ the case's Run and source; else the case has no Run (`:1025`–`:1026`) | G5 | No; symmetric | Does not apply. It is R-D38 rule 1 |
| 13 | a non-null `source_ref` resolves to a CaseSource owned by the case whose preparation binds this attempt (`:1027` on) | G5 | No | Does not apply. It is (4b)'s source conjunct |
| **14** | **`_g5_stages`: native not `not_entered` ⇒ `run_ref` non-null, a Run, and completed ⇔ selected** | **G5** | **Yes: a native failure needed a Run** | **Relaxed to (4b) (`:870`; `_d38_capture_before_run`)** |
| 15 | `_g5_stages`: preparation completed ⇔ `source_ref` non-null (`:876`) | G5 | No | Does not apply; holds for (4b) |
| 16 | `_g5_coverage`: a complete coverage vector needs the attempt's selected Run | G5 | Not for a prepared source as such: only with a proof's vector, which (4b) has not | Does not apply |
| 17 | a Ready attempt needs its source and a selected Run | G5 | Ready only | Does not apply |
| 18 | D4d: `error` preparation ⇒ no Run; native ⇒ its own non-selected Run; **capture with no Run ⇒ (source_unavailable, preparation)** (`:1096`–`:1108`) | G5 | No; capture without a Run is already mapped | Does not apply; it is (4b)'s reason conjunct |
| 19 | `_g5_typed`: `ERROR_STAGE_RECORDS["capture"]` includes done(1, [failed]) (`:916`) | G5 | No | Does not apply; admits (4b)'s record already |
| 20 | `_g5_numeric`: a non-selected case is checked only with a complete coverage vector (`:1338`); a selected case's source and Run | G5a–G5c | No | Does not apply |
| 21 | G6's row methods | G6 | No | Does not apply |
| 22 | `_g8`: per source, K4SRC/K4STF recomputed from the binding (`_native_source_encoding`), owner case, maps, material basis, sections, topology, supports, constraints, stations, nodal terms, layout, the preparation's binding and operational tuples (`:1524` on) | G8 | No: nothing in G8 reads a Call or Run | Does not apply. S-2's reading (a CaseSource without a Call is checked from its binding) holds |
| 23 | `_g8`: each attempt's old operational tuples (`:1634`) | G8 | No | Does not apply |
| 24 | `_transport_g7` / transport (`raw=False`): G0–G2 and the base step only | — | No | Does not apply |

**So exactly one PY check assumed a prepared source had a Run (row 14), and none assumed a Call.** Every one-case D38 receipt so far failed first at G3 ("no selected case"), so row 14 had never been reached with a D38 shape; the (4b) test below reaches it beside a selected case.

### 3.2 The (4b) predicate, and where its other conjuncts are held

`_d38_capture_before_run(a, case, ai)` (called from row 14's branch when native is `failed` and `run_ref` is null) holds R-D38 (4b)'s conjuncts directly: the case has no Run and the attempt no proof; an unavailable `capture` result; the stage record done(1, [failed]); the case unavailable with `prepared_product_failure` naming this attempt and reason (source_unavailable, preparation); and a non-null `source_ref` **equal to the case's** (r01 N-6, which RS and PY add; TS already had it). The remaining conjuncts are held for every receipt elsewhere: the source binding (row 13), `execution_order` (row 6), Call and Group entries (rows 7 and 8), and **Builds**.

**The Build conjunct needed a new check.** PY never required each Build to be referenced by its building record, so a Build left over from a removed Run was admitted. My probe (`probes/orphan_build_{base,head}.txt`): a copy of each 07m base's last Build appended with a fresh id and rehashed is **admitted at BASE on all 17 bases (15 eligible)**, and refused at HEAD on all 17 with G5 `RETAINED_PRECISION_WORK_MISMATCH`, as Rust's `builds_seen.len() == builds.len()` and TS's `seenBuilds.size === b.builds.length` refuse it. The check (`_g5_native_checks`, `built_refs`) is a class-1 WORK predicate, deferred like the others (D3). I ruled it in as part of (4b), on my own reading; ROOT may rule otherwise (§8 item 1).

## 4. Evidence per item

### 4.1 (4b) in `_g5_stages`

`test_b1_d38_capture_before_any_run_beside_a_selected_case` derives (4b) on `two_case_facade_after_certificate_synthetic` (F_BASE) as SC's `d38_beside_selected` will rewrite W-C2's case C: case 1's Run, `execution_order` entry and Call and Group entries removed, the call's after-value and `charged` recomputed (case 1's Run built nothing; it reused case 0's builds), the cause a typed `CaptureError::Origin` (`{origin: {capacity}}`), every hash resealed. **It is admitted:** G0–G8 pass, standing `needs_recompute`, and case 0's classifications equal the base's. At BASE it is refused (G5 PRODUCT_ATTEMPT; the mutant BASE in §7 shows it).

Refused, each at this reader's first failure (asserted as one dict):

| Edit on the (4b) receipt | First failure |
|---|---|
| m1 `error.kind` native (with `run_ref` 1) | G5 PRODUCT_ATTEMPT |
| m2 native `completed` | G5 PRODUCT_ATTEMPT |
| m3 `run_ref` 1 while the case has no Run | G5 PRODUCT_ATTEMPT |
| m4 `execution_order` still lists case 1 | G3 COVERAGE |
| m5 `proof_start` completed | G5 PRODUCT_ATTEMPT |
| m6 attempt `source_ref` null, preparation completed | G5 PRODUCT_ATTEMPT |
| m7 case 1's source in the Call's `source_refs` | G5 ATTEMPT |
| m7 case 1's source in the Group's `source_refs` | G5 ATTEMPT |
| m8 case `source_ref` 0 ≠ attempt's 1 | G5 PRODUCT_ATTEMPT |
| a Build kept from case 1's removed Run (origin run 1) | G5 WORK |
| both source references null | G5 PRODUCT_ATTEMPT |
| result `ready` | G5 PRODUCT_ATTEMPT |
| preparation failed | G5 PRODUCT_ATTEMPT |
| observables and G5a entered | G5 PRODUCT_ATTEMPT |
| reason code `kernel_unresolved` | G5 PRODUCT_ATTEMPT |
| reason phase `kernel` | G5 PRODUCT_ATTEMPT |
| the cause names attempt 0 | G5 PRODUCT_ATTEMPT |
| the cause a `receipt_failure` | G5 PRODUCT_ATTEMPT |

`test_b1_d38_reader_logic_names_every_conjunct` pins the predicate itself: true on the admitted pair, false for the other attempt index and for each of 13 single breaks.

### 4.2 G5's `not_required` rule

PY's `_g5_ordinary` already had DESIGN §3.3's rule (`product_attempt_ref` null, `initial` not `not_attempted`, verdict `checks_passed`), and keeps its report-outcome equality. `test_b1_g5_not_required_admits_a_w2_published_case`, on 07j's two-case statement (`not_required_second_case_checks_passed`): case 1 made W2-published (T-4's case B), by an evaluation trigger (`structural_failure`/range) and by a formation trigger, verdict `checks_passed`: **admitted and eligible**. Refused: W2-published with verdict `sensitive` (G5 ATTEMPT), `initial` `not_attempted` (G5 ATTEMPT), a report whose outcome differs from the verdict (G5 ATTEMPT), a non-null `product_attempt_ref` (G3 COVERAGE first). Rust's three dropped conjuncts, each restored in PY, are killed by the admitted cases (§7).

### 4.3 G8: the requested mode and P1–P4, per case

The loop sits after the invocation, project and model-scope checks and before the per-source checks, over `body.cases` in request order (G3 binds case i to ordinary attempt i and to the invocation's case i). All four checks are `PREPARATION_MISMATCH`.

`test_b1_g8_mode_row_and_requested_mode_for_every_case`, each G8 PREPARATION (BASE checked none of these, on any case):
- the unavailable case (F_BASE): its mode row valued 2 in sparse; valued 3; two mode rows; its requested mode flipped;
- the not_required case (07j): its mode row valued 2; its requested mode flipped; no mode row (on 07j, whose case 1 has no proof, so no projection index moves; on F_BASE G3 refuses first);
- the invocation's own mode flipped.

`test_b1_g8_parity_rows_p2_to_p4_for_every_case`: 07j's statement made dense (invocation mode, both requested modes, both mode rows; no parity row): admitted and eligible with no parity row on either case (a dense b = 0 case may lack it); admitted with one parity row on the not_required case; admitted with a W2-published not_required case and no parity row. Refused at G8: two parity rows on the not_required case (P2), on the dense selected base (P2); a parity row on a W2-published not_required case (P4) and on a W2-published selected case (P4); a parity row on the sparse unavailable case (P3).

### 4.4 RV108 N1 (`_source_contract`, enum membership tests only)

Each of the five tests (`numerical_quality.status`; each case's `solve_quality`, `structural_status`, `model_matrix_fidelity`, `accuracy_evidence`) now reads `not isinstance(value, str) or value not in {…}`, in place, so the `or` chain's order is unchanged. A string is tested as before; a hashable non-string was never a member and still raises the same `ValueError` at the same point; an unhashable value now raises that `ValueError` instead of `TypeError`.

Carrier tests, both milestones:
- `test_rv108_n1_base_header_refuses_a_non_string_enum_with_its_code`: on the reader's projection (a preview-physics-1 envelope), raw and transport dispatch, each field with `[]`, `{}`, `["sensitive"]`, `{"sensitive": 1}`, `None`, 3, `True`, `"estimated"` and `""`: always the base header code, through a ValueError-only caller.
- `test_rv108_n1_retained_reader_reports_the_base_header_code_at_g7`: hash-consistent successors with an unhashable `status`, `structural_status`, `model_matrix_fidelity` or `accuracy_evidence`: G7 with the header code (code and detail), raw with and without the invocation, by the transport validator, and by the raw and transport dispatch. (An unhashable `solve_quality` stops at G5 ATTEMPT first, as RV108 recorded for Rust and TS; its header code is pinned on the base dispatch and the packager.)
- `test_rv108_n1_stress_neutral_packager_validator_refuses_with_a_value_error`: the v0.3 packager's validator on a preview-physics-1 package view built by the packager from the projection: the unedited view passes; each unhashable enum in any of the five fields is a `ValueError` with the header code. At BASE a `TypeError` escaped it (§7, BASE and the N1 mutants).

### 4.5 RV108 N2 and N6(a)

**N2** (`_g5_ordinary`): the verdict is read as absent when the quality case has no `solve_quality`, so each rule that reads it fails with its own G5 ATTEMPT code. `test_b1_rv108_n2_a_missing_verdict_is_g5_attempt`: G5 ATTEMPT for a selected report case, an unavailable report case, a not_required report case and a W2-published not_required case (each was G5 PRODUCT_ATTEMPT through the fallback at BASE); and for an unavailable case with a structural-failure initial (no rule reads its verdict): admitted with the member, and G7 `SOURCE_NUMERICAL_CASE_INVALID` without it, at BASE and HEAD alike. The differential (§5) moves exactly the 48 removal probes on `solve_quality` at the raw entry points, PRODUCT_ATTEMPT → ATTEMPT; transport, which reads no G5, is unchanged.

**N6(a)**: the docstring now says the validator is the twin of Rust's `validate_transport_metadata` and TS's `validateRetainedPrecisionTransport` on G0–G2 (same checks, gates and codes), and that at the base step it runs both of theirs (the header check, Rust's alone, reported as G2; the preview-physics transport metadata, TS's alone), so a defect there can carry a different reader-level code in each language. Wording only; the census shows no outcome change.

### 4.6 A cross-check with SR-RS

After writing my tests I compared them with I90's Rust tests in `a4eab1dd01`, which derive the same receipts the same way. For every input both of us test (the (4b) base with m1–m8 and the other conjuncts; the W2-published `not_required` cases and their three refusals; G8's P1 and requested-mode cases; P2–P4 with the dense conversion), **my Python first failure equals I90's Rust expectation**. My extra inputs (the kept Build, both source references null, no mode row, the invocation's mode, N2) have no Rust counterpart there. RV-R's own three-language run is the oracle; this is only a reading.

## 5. The `_source_contract` differential (N1)

**Method** (`scripts/n1_diff.py`, `scripts/n1_compare.py`). One process per tree, BASE (archive of I1) and HEAD; every outcome is `ok:<hash of the result>` or `<exception class>:<message>`:
- **A1 (1,356):** every 07m entry (17 bases, 294 mutations, 28 must-pass), raw and as the reader's G7 projection, through `_source_contract` with `check_receipt` true and false;
- **A2 (584):** every JSON object in `P/fixtures` carrying `schema_version`, and every such member one level down, both ways;
- **A3 (3,718):** enum probes on the projection of each 07m base and both milestones: each of the five fields (every case) set to each vocabulary member, `"estimated"`, `""`, `None`, 0, 1.5, `True`, `False`, `[]`, `[member]`, `{}`, `{member: 1}`, and removed, both ways;
- **B (164):** the v0.3 packager's validator on a preview-physics-1 package view of each milestone, unedited and with the same probes;
- **C (4,485):** the retained reader on each 07m base and both milestones with the probes (one member, the 11 others and removal) applied and rehashed: raw with and without the invocation, and transport.

**Result** (`n1_differential/breakdown.txt`, `compare.txt.gz`): 10,307 inputs; **8,111 identical**; 2,196 differ, all explained, **0 unexplained**:

| Section | Differ | Class |
|---|---|---|
| A3 | 920 | BASE `TypeError: unhashable` → HEAD `ValueError` with the header code |
| B | 40 | the same, through the packager's validator |
| C | 1,188 | reader: BASE G7 `SOURCE_PREVIEW_PHYSICS_INVALID` (the fallback, from the `TypeError`) → HEAD G7 header code; every one an unhashable value |
| C | 48 | N2: `solve_quality` removed, raw with and without the invocation: G5 PRODUCT_ATTEMPT → G5 ATTEMPT |

**Every input that did not raise in `_source_contract` reads exactly as before**: A1 and A2 (the corpus and every fixture) are identical; in A3 and B, all 960 BASE `TypeError`s change and nothing else does; no HEAD outcome is a `TypeError`; no BASE outcome was any exception class other than `ValueError`, `RetainedPrecisionError` or `TypeError`. Hashable non-strings (`None`, 0, 1.5, `True`, `False`), invalid strings and removals read identically in every section (except N2's 48 reader removals).

## 6. The Python suites, base against head, test by test

I83's set for B6 (I69's set plus the three retained files: 27 files, `scripts/suites.sh`), in `git archive` copies of BASE and of each head (without `_Coordination/`, with the rest of `P/execution/`), under `lockf -k WT/guard/cargo_job.lock`, with my own builds of the two CLI authorities (`OPENPIPESTRESS_CHECKED_JSON_BIN`, `OPENPIPESTRESS_UNITS_BIN`), VENV's Python 3.13.14. Compared from the junit XML, test by test (`scripts/compare_suites.py`):

| Run | Passed | Skipped | Failed | Against BASE |
|---|---|---|---|---|
| BASE `262bd687f0` | 1,922 | 30 | 0 | — (B6's head count on this set, as RV108 recorded) |
| `de7d9b9736` | 1,934 | 30 | 0 | **+12 added (all pass), 0 removed, 0 changed outcomes** |
| HEAD `75132d2673` | 1,934 | 30 | 0 | **+12 added (all pass), 0 removed, 0 changed**; identical, test by test, to `de7d9b9736`'s run |

The 12 added are exactly the new tests: the 6 contract tests of §4 and the 3 carrier tests × 2 modes. Every count change is an added test.

## 7. Mutants

`scripts/mutants.py`: each mutant is one textual edit (asserted to match once) to a copy of the head tree, then both retained test files run in full (460 tests at head: the contract file's 410 and the carrier file's 50), with junit output; the file is restored byte for byte. pytest used my own CLI builds; no built test binary was run directly. Round 1 ran on `de7d9b9736`; round 2 reran N0, the five N1 mutants and BASE on `75132d2673` after the helper change.

| Id | Mutant | Outcome | Killed by |
|---|---|---|---|
| N0 | none (head) | passes 460/460 | — |
| D38-restore | the relaxed check restored (no (4b) branch in `_g5_stages`) | killed | the (4b) admission test |
| D38-capture | (4b) without `error.kind == capture` | killed | the predicate test (in the reader also held by D4d and `_g5_typed`) |
| D38-stages | (4b) without the later stages `not_entered` | killed | the predicate test (also held by `_g5_stages`' pipeline rule) |
| D38-prep | (4b) without preparation completed and native failed | killed | the predicate test (also held by the pipeline rule and the branch) |
| D38-reason | (4b) without (source_unavailable, preparation) | killed | the predicate test (also held by D4d's mapping) |
| D38-source-eq | (4b) without `a.source_ref == case.source_ref` | killed | the (4b) test (m8) and the predicate test; held nowhere else |
| D38-source-nonnull | (4b) without `a.source_ref` non-null | killed | the predicate test (also held by `_g5_stages`' preparation ⇔ source rule) |
| D38-names-a | (4b) without the cause naming this attempt | killed | the predicate test (also held by D19 and D4c) |
| D38-case-status | (4b) without the case unavailable with `prepared_product_failure` | killed | the predicate test (also held by D19) |
| D38-no-run-no-proof | (4b) without `case.run` and `proof` null | killed | the predicate test (also held by D4e and the proof/stage rules) |
| D38-builds | the every-Build-referenced check removed | killed | the (4b) test (the kept Build) |
| G5-report | `not_required` also needs `initial.kind == report` (Rust's) | killed | the W2-published `not_required` test; the P2–P4 test |
| G5-outcome | … also needs `initial.outcome == checks_passed` (Rust's) | killed | the same two |
| G5-w2 | … also needs `w2.kind == not_triggered` (Rust's) | killed | the same two |
| G8-selected-only | the per-case loop over selected cases only (Rust's old scope) | killed | the P1/requested-mode test; the P2–P4 test |
| G8-first-only | the loop over the first case only | killed | the same two |
| G8-requested | the requested-mode check removed | killed | the P1/requested-mode test |
| P1-removed | P1 removed | killed | the P1/requested-mode test |
| P1-count | P1 checks the first of any number of mode rows | killed | the P1/requested-mode test (two mode rows) |
| P1-code3 | P1 admits mode code 3 in sparse (TS's old rule) | killed | the P1/requested-mode test |
| P2-removed | P2 removed | killed | the P2–P4 test |
| P3-removed | P3 removed | killed | the P2–P4 test |
| P4-removed | P4 removed | killed | the P2–P4 test |
| P2-4-old | P2–P4 replaced by "exactly one parity row iff dense" (Rust's old) | killed | the P2–P4 test (dense b = 0 without a parity row) |
| N1-status | N1's guard removed on `status` | killed (R1: 6; R2: 6) | the base-header, retained-reader and packager tests, both modes |
| N1-solve | … on `solve_quality` | killed (R1: 4; R2: 4) | the base-header and packager tests (a list verdict stops at G5 in the reader) |
| N1-structural | … on `structural_status` | killed (R1: 6; R2: 6) | the three N1 tests, both modes |
| N1-fidelity | … on `model_matrix_fidelity` | killed (R1: 6; R2: 6) | the three N1 tests, both modes |
| N1-accuracy | … on `accuracy_evidence` | killed (R1: 6; R2: 6) | the three N1 tests, both modes |
| N2-removed | N2's explicit read removed (`quality[i]["solve_quality"]` again) | killed | the N2 test |
| BASE | head's tests on BASE's reader and `compatibility.py` | 11 of the 12 new tests fail (R1 and R2) | all new tests except the `not_required` one, which BASE already passes (PY had the rule) |

**30 of 30 mutants are killed by assertions; none survives.** In round 1 the N1 mutants on the base-header and packager tests failed through the pre-N1 `TypeError` escaping the ValueError-only caller; after `75132d2673` that escape is reported by name and fails the equality assertion (round 2). No mutant was killed by a 07m entry, consistent with the census: 07m does not reach these rules. SC's 07n adds the shared entries that do.

## 8. For ROOT

1. **The Build check is my addition, for ROOT to confirm (§3.2).** R-D38 (4b) requires that no Build name the case or its source; PY could not enforce it, because it admitted any Build that no record built (BASE admits such a receipt on all 17 bases, 15 of them eligible). Rust and TS already refuse it at G5 WORK. I added TS's form (every Build referenced by its building record) in the alignment commit `a514f4ca74`; the census is unchanged and mutant D38-builds pins it. **For SC (07n):** a ninth D38 mutation, "case C's Builds kept", would pin this conjunct in all three readers (G5 WORK in each, by my reading of RS and TS).
2. **Pre-existing Python-only admissions found next to SR-PY's scope, not changed** (`probes/gaps_{base,head}.txt`; on 07j's two-case statement and `ordinary_prepared_synthetic`):
   - **(b) G8 step 2, `o.material_basis_ref`, is missing in PY.** DESIGN §3.3's loop has four steps and says "PY: adds the requested-mode check and P1–P4"; but PY never checks an ordinary attempt's `material_basis_ref` for a case without a source: on the `not_required` case, a value of 7 is admitted and eligible. RS (`u(o.material_basis_ref) == index`) and TS (`ordinary.material_basis_ref === bi`) refuse at G8 PREPARATION.
   - **(c)** `material_bases[].case_indices` is not checked for exactness: omitting the `not_required` case is admitted (eligible). RS and TS refuse at G8 PREPARATION.
   - **(d1)** the invocation's member set is not checked: an extra member, rebound, is admitted (eligible). RS and TS refuse at G8 INVOCATION.
   - **(d2)** the invocation's `solver_mode` is not checked for its domain at INVOCATION: `"foo"` was admitted (eligible) at BASE; at HEAD the requested-mode check refuses it at **G8 PREPARATION**, where RS and TS refuse at **G8 INVOCATION**.

   These are outside my brief's items (G8's requested mode and P1–P4), so I left them. ROOT may route them to an SR-PY repair round (G8 step 2 and the material-basis exactness as Rust computes them; the invocation shape before the hash) or to per-reader declarations in 07n.
3. **No 07m entry moves under N1 or N2,** so 07n needs no re-expectation for them; RV108 N1's 104 and N2's 10 probe classes now agree with Rust and TS in Python (§5). TS's B6 comment that the G7 code agrees across the languages becomes true for Python's side of N1.
4. **Not in my brief and still open:** RV108 N6(b) (the carrier case file's scope sentence, a fixture outside my fence); N3–N5 (SC's).
5. **SP's several-notice bytes (T-12)** come later, from SP; my PY check of them is part of SC.
6. **ROOT's restated host rule** (heavy jobs under the lock, including directly run test binaries) reached me mid-run: I ran no built test binary directly; every pytest used my own CLI builds, and the suites also ran under the lock.

## 9. Host, disclosures and cleanup

- **Cargo:** one job, through `WT/tools/t3_cargo.sh` (`--locked --offline`, `CARGO_BUILD_JOBS=4`, no RUSTFLAGS): the two CLI authorities (`canonical_json` with `checked-cli`, `units` with `cli`, both `--release`) into `WT/targets/i91-b1-sr-py/` (its logs and the lock log's lines in `_run_records/host/`). Binary sha256: `openpipestress_jcs_ijson` `549cc2ca…d9a9`, `openpipestress_units` `57064fa9…3a33`.
- **pytest:** the two suite pairs under `lockf -k WT/guard/cargo_job.lock` with my own binaries; the quick single-file runs, the census, the differential, the probes and the mutant lanes with my own binaries, not under the lock (B1_COMMON's rule, as RV108 ran its lanes). `TMPDIR` and `--basetemp` in `S`.
- **Waits:** one per job, each ending when the job's process was gone. Disclosure: twice a foreground wait reached the tool's 10-minute limit and continued in the background beside the job's own completion notice, and a log monitor on the mutant lane would have outlived its job; I stopped those three waiters of mine (no job of mine or anyone else's was stopped) and then used bounded waits of at most 9 minutes. None is left running.
- **Not done:** no DEC-025, evidence sweep, native or solver job, install, Rust or TS job beyond the one build. No other agent's job was touched. Git: commits only on my branch; reads with `GIT_OPTIONAL_LOCKS=0`.
- **The records' paths are placeholders only** (`scripts/sanitize.py` refuses any host form; the screen over this folder, gzipped files included, finds none).
- **Cleanup:** the four `git archive` copies under `S` (BASE, both heads and the mutant copy, which matched the final head byte for byte after the last restore) and `WT/targets/i91-b1-sr-py/` are deleted. `S` keeps only small working files (scripts, census and differential outputs, logs). `WT/b1-p` is clean at `75132d2673`. No process of mine is running; no wait or monitor of mine is left.

## 10. Records in this folder

- `RETURN.md`; `SHA256SUMS` over every other file.
- `_run_records/diff/`: `commits.txt`, `diffstat.txt`, `i91.diff` (BASE..HEAD).
- `_run_records/census/`: `base.json`, `head_c1.json`, `head_code.json`, `compare_c1.txt`, `compare_code.txt`.
- `_run_records/n1_differential/`: `base.json.gz`, `head.json.gz`, `compare.txt.gz`, `breakdown.txt`, `n2_removals.txt`.
- `_run_records/probes/`: the orphan-Build probe and the gap probes, BASE and HEAD; the new inputs through BASE's and HEAD's reader.
- `_run_records/suites/`: junit XML (gzipped), tails and return codes for `py_base` (BASE), `py_head` (`de7d9b9736`) and `py_head2` (HEAD `75132d2673`); `py_compare*` (BASE against each head, and the two heads against each other).
- `_run_records/mutants/`: `mutants_r1.jsonl`, `mutants_r2.jsonl`, their logs.
- `_run_records/host/`: the build logs and binary hashes.
- `_run_records/scripts/`: every script used, with host paths replaced.

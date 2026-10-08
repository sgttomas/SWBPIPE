# RV113 (RV-R): review of SR-PY, the Python reader (R-D38 (4b), F-1 text B, G8 per case, G5 `not_required`, RV108 N1, N2 and N6(a), repair 01)

TASK (Type 2), RV113, an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path; I made no delegation. I wrote none of the change. 2026-10-07 UTC.

**Basis.**
- The coordinator's message: review SR-PY; recommend a gate for I91's (f) and (g); fold in SR-TS's C2 cause-branch question so that ROOT can rule all three-reader alignments together.
- RR "I91's SR-PY verified; the Build check accepted; …" and "I91's SR-PY repair verified; (e) kept; (f) and (g) to RV-R; SR-PY goes to RV113".
- My brief `R/BRIEFS/RV113_RVR_ROUND1.md` (`a753ea5b…cf9cd4`), with its host rules as ROOT restated them.
- SR-PY's brief `R/BRIEFS/B1_SR_PY.md`, sha256 `2a902874…0322e`, verified.

**Order of reading.** I formed my own view first: the census, the probes, the `_source_contract` differential, my D38 audit and my mutant schema. The audit was hashed (`evidence/d38/D38_AUDIT_RV113_BEFORE_RETURN.md`, `8a5f5713…638c`) before I opened I91's records. Then I read I91's `R/I91/b1_sr_py_01/RETURN.md` (`bc7fa865…f50a`) and `REPAIR_01.md` (`f97b46ba…cc7b`). Both were verified, with their SHA256SUMS (56 of 56 and 29 of 29 OK).

**Placeholders.**
- WT, NUM, P, PP, RE, R, RR, VENV and NMS are as in my SR-RS and SR-TS reviews.
- PY = `P/core/analysis_runs/retained_precision.py`; CP = `P/core/analysis_runs/compatibility.py`.
- RS = `RE/src/retained_precision.rs`; TS = `P/apps/desktop/src/features/results/retainedPrecision.ts`.
- My copies are `git archive` copies of P without `execution/`:
  - P@I1 (`262bd687f0`) and P@HEAD (the candidate);
  - P@MUT (the head with my mutant schema);
  - P@RS (`b5cb7faaeb`, SR-RS's repaired head; RS is byte-identical to `cc81e78801`);
  - P@TS (`7e47e51b5d`, SR-TS's head).

**What I reviewed.** `codex/piping-t3-b1-p-20261007` at `11cc14e3e65363790f43943b09933a57f4da28e4` (the branch tip; `WT/b1-p` clean). It is nine commits over I1 `262bd687f0` and touches four files:
- PY +105/−9;
- CP +6/−2;
- the contract tests +352;
- the carrier tests +82.

`diff -rq` between my I1 and head copies lists exactly these four files. Each matches `git show` at its commit, and the corpus is unchanged (07m, `c21112fd…6807`, 17 bases, 294 mutations, 28 must-pass).

## Verdict: PASS. 0 BLOCKING / 4 SHOULD-FIX / 2 NOTE

SR-PY does what its brief and repair round asked, and it is clean:
- **The census:** 0 changes over 07m.
- **(4b), D38 and the orphan-Build check:** (4b) is admitted beside a selected case; m1–m8 and the other conjuncts are refused. My D38 audit agrees with I91's list.
- **G8:** P1–P4 and the requested mode are checked for every case.
- **G5 `not_required`:** this is DESIGN's rule.
- **RV108:** N1, N2 and N6(a) are done.
- **The differential claim holds.**
- **The suites:** +15 added, 0 removed, 0 changed.
- **Mutants:** 34 of my 35 are killed by assertions. The one survivor is S-4 (§9).
- **Probes:** on my 103 probes, PY now agrees with RS on every B1 shape, and with TS except C2.

None of the SHOULD-FIX items is a regression:
- S-1 and S-3 are three-reader differences, already open as (f) and (g), whose gates ROOT asked me to recommend.
- S-2 is a record error in repair 01 that would break SC's 07n as planned.
- S-4 is a missing test row, the same gap SR-RS S-1 found in RS.

| # | Severity | Where | Finding | Recommendation |
|---|---|---|---|---|
| S-1 | SHOULD-FIX | PY `_g8`; RS `g8`; TS `coverage` and `ordinaryAttempts` | **I91's (f) is one member of a family. The receipt-internal references of `material_bases` and `sources` are checked at different gates, and RS and PY do not check them at all on an unbound read.** My probes (§10.1): <br>• a wrong `material_bases[].index`: PY admits it, bound (eligible) and unbound; RS gives G8 PREPARATION bound and admits unbound; TS gives G3 COVERAGE both ways; <br>• a wrong `sources[].index`, or a duplicate or out-of-range `case_indices`: RS and PY give G8 PREPARATION bound and admit unbound; TS gives G3 COVERAGE both ways; <br>• a source owner naming another case's id: PY gives G8 bound and admits unbound; RS gives G5 ATTEMPT both ways; TS gives G3 both ways; <br>• an ordinary attempt whose `material_basis_ref` does not resolve to a basis listing its case: TS gives G5 ATTEMPT both ways; RS and PY give G8 PREPARATION bound and admit unbound. <br>None of these needs the invocation, and no 07m entry pins any of them | **Gate: G3 COVERAGE, in all three readers, for:** <br>• each `material_bases[].index` and `sources[].index` at its position; <br>• `case_indices` unique and in range; <br>• each source's owner a case, in range, with that case's id. <br>This is TS's `coverage` block; RS and PY move or add the checks. <br>**G5 ATTEMPT for the ordinary attempt's basis reference** (TS's `ordinaryAttempts` line; RS and PY add it). <br>G8 keeps only the facts that need the invocation: the selectors, the exact case lists by selector, and the materials. See §10.1 for the alternative and why I prefer this one |
| S-2 | SHOULD-FIX | I91 `REPAIR_01.md` §2 and §4.3; PY tests `test_b1_repair01_g8_material_basis_of_every_case_and_exact_case_indices` and `…_every_material_basis_has_its_materials_checked` | **Repair 01 says (b) and (c) have "the same gate and code in all three readers". For three of its own inputs, TS's first failure is G5 ATTEMPT, not G8 PREPARATION.** TS's ordinary class (`ordinaryAttempts`, `fail(b.material_bases[a.material_basis_ref]?.case_indices.includes(ci))`) refuses them before G8 (§4.2). The three inputs: <br>• 07j's `not_required` case with `material_basis_ref` 7; <br>• its basis omitting case 1; <br>• the missing sourceless basis (the `11cc14e3e6` pin). <br>PY's tests pin G8 PREPARATION, which equals RS only. For the other inputs ((b) beside a second basis, (c) out of order or an extra basis, (d1), (d2), (e)), the three readers agree. PY's code is correct against RS, but **SC's 07n, as REPAIR_01 §5 item 3 proposes it, would fail in TS** | Correct the record. For 07n, either take S-1's ruling first and expect G5 ATTEMPT in all three readers, or use inputs that pass TS's G5 check (e.g. (b) beside a second basis), or declare per reader. Pick one before SC writes the entries |
| S-3 | SHOULD-FIX | PY `_g8`, TS `invocationBinding` and RS `g8`: their model-scope checks | **(g), and the model-scope members around it, differ in every reader.** The producer side: <br>• **`reference_configurations`:** PP's `case_state/resolve.rs` `validate_document` blocks any model document other than 0.4.0 that carries it at all, an explicit null included (`LOAD_STATE_CONTRACT_VERSION_MISMATCH`; `Authored::is_authored`). The readers admit only 0.1.0–0.3.0. D1.3's admission (`retained_memory.rs` `family_clauses`) refuses it too; <br>• **`pressure_contract`, `combinations`, `components`:** PP's typed model (`lib.rs` `PreviewModelWire`: `Option`, and `Vec` with `#[serde(default)]`) accepts `pressure_contract` absent or null, and the two lists absent or as arrays. The readers' own rule then requires no pressure contract and empty lists. <br>My probes (§10.2): <br>• `reference_configurations` null or `[]`: RS gives G8 INVOCATION; TS and PY admit (eligible); <br>• `pressure_contract` `{}`: RS and TS refuse; PY admits; <br>• `pressure_contract` `false`: RS refuses; TS and PY admit; <br>• `combinations` `{"x":1}`: PY refuses; RS and TS admit; <br>• `components` `"x"`: PY and TS refuse; RS admits; <br>• `combinations` null: all three admit, though PP's parse refuses it | **Gate: G8 INVOCATION_MISMATCH, in the model-scope check before any PREPARATION check (as RS orders it). The rule is what PP accepts, intersected with the readers' own rule:** <br>• no `reference_configurations` member, null included (RS's rule; TS and PY add it); <br>• `pressure_contract` absent or null (RS's rule; TS and PY tighten their falsiness tests); <br>• `combinations` and `components` absent or `[]` (all three tighten). <br>Pin each in 07n |
| S-4 | SHOULD-FIX | PY tests: `test_b1_g5_not_required_admits_a_w2_published_case` | **PY's `not_required` conjunct `product_attempt_ref` null is not pinned at its own first failure.** My Q17 drops it and survives all 463 tests of both retained files. With the conjunct dropped, a `not_required` case naming its own product attempt fails at G5 `PRODUCT_ATTEMPT` instead of G5 `ATTEMPT`. RS and TS give `ATTEMPT` (my probe `nr_product_attempt_ref_with_attempt`; PY at the head does too). The test's only non-null row names *another* case's attempt, which G3 refuses first. This is SR-RS S-1's gap, which I90 closed in RS with one row | Add the own-attempt row to the `not_required` test, expecting G5 `ATTEMPT_MISMATCH`, as RS's repair did. Q17 must then die at it |
| N-1 | NOTE | PY `validate_retained_precision_transport` docstring (N6(a)) | **The docstring is right in substance, but it does not say that PY reports its base step at G7.** On transport, PY gives a header defect G7 with the base code, and RS gives G2 with the same code. On 07m, the nine G7 header mutations read G7 in PY, G2 in RS, and admitted or G7 `…EVIDENCE_INVALID` in TS. This is pre-existing (I1 identical) | Add "(reported here at G7)". Rule the transport header scope with SR-TS N-1 (§10.4) |
| N-2 | NOTE | PY `_d38_capture_before_run` | **Some of the predicate's conjuncts are also held elsewhere in the reader.** Those conjuncts' mutants are killed only by the predicate's unit test (§9), as I91 records. They are not dead: they keep (4b)'s shape whole in PY, as in RS and TS | None. Keep them, as RR ruling 3 did for TS |

**Carried, not recounted: SR-TS S-1 (C2 cause branches).** I have now checked it by run. PY behaves exactly as RS does on the six C2 probes and on I92's `receipt_failure` variant. Of the five that TS refuses at G5 ATTEMPT, PY admits three and refuses two at G5 `PRODUCT_ATTEMPT` (§10.3).

## 1. The census over 07m

**Method.** My harness (`evidence/harness/rv113_py_harness.py`) is written from the snapshot format rules (SHARED_SNAPSHOT_06C `format_change`, 07E `format_rule`), not from the candidate's helpers. For each of 339 entries, it records three verdicts:
- `validate_retained_precision` with the invocation (bound);
- the same without the invocation (unbound);
- `validate_retained_precision_transport`.

Each run was one locked job on P@I1 and on P@HEAD, with my own builds of the checked-JSON, binary64 and units authorities.

| Comparison | Result |
|---|---|
| I1 against the head, 339 entries × 3 verdicts | **0 changes**; inputs byte-identical (0 differ); 0 escapes |
| PY's bound verdict against the corpus's Python expectation (`expected_by_reader.python`, else `expected`); must-pass eligibility and standing; bases | **0 misses** at I1 and at the head |
| PY against TS (SR-TS head; my SR-TS census) | bound: **0 differences** in 339 |
| PY against RS (my SR-RS census) | bound: **1 difference**, `g7_maximum_off_enclosure`, which is the corpus's declared per-reader G7 code |
| Transport, three readers | 330 agree; 9 differ: the G7 header mutations, as in N-1 (pre-existing) |
| Unbound, three readers | 338 agree; 1 differs: the same declared G7 split |

**No 07m entry is moved by N1 or N2.** This agrees with I91.

## 2. Probes: PY against RS and TS

My 103 probes are SR-TS's v5 (`gen_probes.py`, published in my SR-TS review; `evidence/probes/probes_v5.json`, `d98be70f…cbaf`). They are the 82 I built for SR-RS plus C2, N4 and N6. RS and TS outputs are the ones already published in my SR-TS review (`probes_rs_head.jsonl` `dcbe5490…`, `probes_ts_head.jsonl` `4eafa0cc…`).

| | Result |
|---|---|
| PY at the head against RS | **101 of 103 agree**. The two that differ are the two declared N6 header classes (`n6_carrier_evidence_with_case_defect`, `n6_contract_evidence_null_and_source_block_recovery`), where PY equals TS |
| PY at the head against TS | **98 of 103 agree**. The five that differ are the C2 cause-branch probes (four `c2_*` probes and I92's `receipt_failure` variant), where PY equals RS (§10.3) |
| PY at I1 against the head | **30 probes change, and each moves to the value both RS and TS give.** They are: <br>• (4b) and its neighbours (6): three are admitted (`d38_base`, `f38_base`, `x_cause_native_unavailable`); the kept Build and the capture accounting now fail first at G5 WORK; the kept recovery method at G6; <br>• G8 P1–P4 and the requested mode per case, including the disclosed limit's P1 row deletion (16); <br>• the eight N6 enum list and dict probes (8): the G7 fallback becomes the header code |

Two more probe sets were run in all three readers, each reader as its own locked job (`evidence/fg/`, `evidence/repair_probes/`). The RS and TS harnesses are those of my SR-TS review: `rv113_census.rs` `c0dadef9…`, `rv113Census.test.ts` `abcc6e36…`.
- **15 probes for (f) and (g) and their neighbours** (§10);
- **17 probes for repair 01's (b)–(e)** (§4).

I built them from the corpus (`gen_probes_fg.py`, `gen_probes_repair.py`), not from I91's tests. Each harness digests its own serialization. So I checked input identity across languages by the publication digest: every admitted row of each probe has one digest in all four runs.

## 3. (4b), D38's audit and the orphan-Build check

**My audit** (`evidence/d38/`, before reading I91) lists 16 checks:
- **One check assumed a Run** for a prepared source: `_g5_stages`' entered-native branch. It is relaxed exactly to (4b), by the new `elif` for native `failed` with `run_ref` null, to `_d38_capture_before_run`.
- **One check was missing, and is now added:** every Build is referenced by its building record.
- **The rest do not apply.** They are Run- or proof-scoped, or (D4d) they already map a capture with no Run to (`source_unavailable`, `preparation`).

**Against I91's 24 rows,** the two lists agree on the relaxation, on the Build check and on every disposition. I91's list is finer (G1 hashes, `_g5_typed`, G6, transport). Mine adds one point. D19 (`_g5_products`) already requires an unavailable attempt to be carried by `prepared_product_failure` naming it. So the predicate's cause-kind and cause-attempt conjuncts duplicate D19 at the same gate and code (N-2).

**The orphan-Build check** (`fail(built_refs == set(range(len(body["builds"]))), "WORK_MISMATCH")`) is a class-1 WORK predicate, deferred like the others, as Rust's `builds_seen` and TS's `seenBuilds` are. My probe `x_builds_of_removed_run_kept` moves from G5 PRODUCT_ATTEMPT at I1 to G5 WORK at the head, as RS and TS give. 07m is unchanged. My Q29 (the check removed) is killed (§9).

**By run:**
- (4b) beside a selected case is admitted with `needs_recompute`, in PY as in RS and TS (`d38_base`, `f38_base`).
- m1–m8 are refused, each at the first failure RS and TS give.

## 4. G8 per case, and repair 01's (b), (c), (d1), (d2) and (e)

### 4.1 The per-case loop (DESIGN_v2 §3.3; F-1 text B)

**Placement.** The loop follows the invocation's shape, digest, project and model-scope checks. It runs over every case in request order and checks, all at `PREPARATION_MISMATCH`:
1. the requested mode;
2. the material basis, numbered by first-seen selector;
3. P1: exactly one mode row, valued with the mode code (int or float, booleans excluded);
4. P2–P4.

The basis list ((c), (e)) and then the per-source checks follow.

**My probes agree with RS and TS** on every G8 per-case shape I built for SR-RS:
- the unavailable and `not_required` cases' mode rows (code 2 in sparse, code 3, duplicated, removed);
- the requested mode flipped;
- P2, P3 and P4 on selected, unavailable and `not_required` cases;
- the disclosed limit.

### 4.2 Repair 01, three readers (`evidence/repair_probes/RP_TABLE.json`)

| Probe | PY at I1 | PY at the head | RS | TS |
|---|---|---|---|---|
| control: 07j unedited | eligible | eligible | eligible | eligible |
| (b) `not_required` case's `material_basis_ref` 7 | eligible | G8 PREPARATION | G8 PREPARATION | **G5 ATTEMPT** |
| (b) basis 1 beside a second base basis listing case 1 | eligible | G8 PREPARATION | G8 PREPARATION | G8 PREPARATION |
| (c) the basis omits case 1 | eligible | G8 PREPARATION | G8 PREPARATION | **G5 ATTEMPT** |
| (c) case list out of order; an extra empty basis | eligible | G8 PREPARATION | G8 PREPARATION | G8 PREPARATION |
| (c) the sourceless case's basis missing (`11cc14e3e6`'s pin) | eligible | G8 PREPARATION | G8 PREPARATION | **G5 ATTEMPT** |
| (d1) an extra invocation member | eligible | G8 INVOCATION | G8 INVOCATION | G8 INVOCATION |
| (d2) `solver_mode` `"foo"`, a list, null, removed | eligible | G8 INVOCATION | G8 INVOCATION | G8 INVOCATION |
| (e) control: case 1 on its own named basis | eligible | eligible | eligible | eligible |
| (e) that basis's E wrong; its selection the base's; another material id; no material | eligible | G8 PREPARATION | G8 PREPARATION | G8 PREPARATION |

**What this shows:**
- **Every repair item is now refused in PY,** at RS's gate and code.
- **(d1), (d2) and (e) agree in all three readers.**
- **Three (b)/(c) inputs reach TS's G5 ordinary check first** (S-2). Unbound, RS and PY admit them all, and TS refuses those three at G5. This is S-1's family.

### 4.3 Code reading

- **(d1) and (d2).** The first check in `_g8` is `type(invocation) is dict and set(invocation) == {"request", "solver_mode"} and solver_mode in (…)`, at INVOCATION. A tuple membership test never raises, whatever the value's type. An unknown mode therefore never reaches `mode_code`'s dict lookup (which would fall back to PREPARATION); my Q13 shows that fallback.
- **(e).** Each basis's materials are selected for the basis's first case. All of a basis's cases share one selector, because (c) is checked first. So this equals TS's per-case form. The per-source materials check stays and is now redundant. It is harmless.

## 5. G5's `not_required` rule

PY's rule (`_g5_ordinary`) is DESIGN §3.3's:
- `product_attempt_ref` null;
- `initial` not `not_attempted`;
- verdict `checks_passed`.

The report-outcome equality is kept. Rust's three dropped conjuncts are absent:
- a report initial;
- its outcome `checks_passed`;
- W2 `not_triggered`.

**By run**, on my 13 `nr_*` probes, PY, RS (SR-RS's head) and TS agree on every one:
- W2-published `not_required` cases, by an evaluation or a formation trigger, are admitted and eligible;
- a W2-failed one with `checks_passed` is admitted and eligible;
- another verdict, `initial` `not_attempted`, a report outcome that differs, and a report with W2 published are each refused at G5 ATTEMPT;
- my SR-RS S-1 shape (`nr_product_attempt_ref_with_attempt`, the case naming its own attempt) is refused at G5 ATTEMPT;
- a dangling reference is refused at G3 COVERAGE.

PY at I1 already gave these verdicts; DESIGN says PY had the rule. Of my mutants (§9):
- Q14–Q16, each of Rust's dropped conjuncts restored, are killed;
- Q18, the verdict conjunct dropped, is killed;
- **Q17, the `product_attempt_ref` null conjunct dropped, survives every test.** It changes the own-attempt shape's first failure to G5 `PRODUCT_ATTEMPT` (S-4).

## 6. RV108 N1: the `_source_contract` differential

**The rule** (B1_SR_PY): every input that did not raise in `_source_contract` reads exactly as before.

**Method** (`evidence/harness/rv113_source_contract_diff.py`). One locked process per copy, P@I1 and P@HEAD. Each outcome is `["return", result]` or `[exception class, message]`. The inputs, 16,563 outcomes in all:
- every 07m entry three ways: raw (`check_receipt` true), the reader's G7 projection (true) and its transport projection (false); 1,017 outcomes;
- on four bases' projections, each of the five enum members (`numerical_quality.status`, and case 0's `solve_quality`, `structural_status`, `model_matrix_fidelity`, `accuracy_evidence`) set to each of 19 values, both modes; 760 outcomes. The values are the vocabulary, `"bogus"`, `""`, null, 0, 1, 1.5, true, false, `[]`, two one-member lists, `{}` and a dict;
- every JSON envelope under `P/fixtures/` (78 files), raw and with the same variants; 14,786 outcomes.

The copies differ only by their root, which the returned contract path names. The comparison removes the root (`compare_py.py`, `norm`).

| I1 outcome | Count | At the head |
|---|---|---|
| `return` | 1,928 | **identical** |
| `ValueError` (not from a caught `TypeError`) | 11,325 | **identical** |
| `LoadReferenceError` | 60 | **identical** |
| `TypeError: unhashable` | 3,150 | `ValueError` with the header code: `SOURCE_NUMERICAL_QUALITY_INVALID` 630, `SOURCE_NUMERICAL_CASE_INVALID` 2,520 |
| `ValueError: SOURCE_PREVIEW_PHYSICS_INVALID` on a retained successor read with `check_receipt` false | 100 | the header code |

**The last row** comprises the four retained successor fixtures, each of the five fields, each unhashable value (5 × 5 × 4). In each, the reader's transport step calls `_source_contract` on its projection. At I1, that inner call raised `TypeError` at the membership test. The reader's fail-closed fallback mapped it to G7 `SOURCE_PREVIEW_PHYSICS_INVALID`, and `_retained_contract` re-raised that as a `ValueError`. So these inputs did raise at the guarded test, and they change as N1 intends. They are I91's class C.

**No other input changed.** Strings read exactly as before, and so do hashable non-strings (always refused with the same code at the same point). **The rule holds.** The edit is `not isinstance(v, str) or v not in {…}` in place, so the `or` chain's order is unchanged. My Q31–Q35 (each guard removed) are killed.

## 7. N2 and N6(a)

**N2.**
- `verdict = quality[i]["solve_quality"] if "solve_quality" in quality[i] else None` replaces the `KeyError`.
- Each rule that reads the verdict now fails with its own G5 ATTEMPT code, as RS's null and TS's undefined do. A case for which no rule reads it reaches G7's header (`SOURCE_NUMERICAL_CASE_INVALID`).
- My Q30 (the old read) is killed by `test_b1_rv108_n2_a_missing_verdict_is_g5_attempt`.

**N6(a).** The docstring now says the validator is the twin of RS and TS on G0–G2, and that at the base step it runs both of their checks. That is right by run (§1, transport row): on G0–G2 the three agree on every 07m entry. It omits PY's gate label (N-1).

## 8. The Python suites, test by test

**The set and the runs.** I83's 27-file set (`evidence/harness/py_sweep_files.txt`), one locked pytest job per copy, with my own authority builds and VENV's Python 3.13.14. Compared from junit XML (`compare_junit.py`).

**A correction to my copies.** They lacked `P/execution/`. Two handoff files read PKG-15's working files from it, so 19 tests failed in both copies (`FileNotFoundError`). I added that folder from `git archive` to both copies; it is identical at I1 and the head, 85 files. Then I reran those two files, one locked job each: 20 of 20 passed in both.

| Run | Passed | Skipped | Failed |
|---|---|---|---|
| I1, whole set | 1,903 | 30 | 19 (the missing folder) |
| Head, whole set | 1,918 | 30 | 19 (the same 19) |
| **I1, with the handoff rerun** | **1,922** | 30 | 0 |
| **Head, with the handoff rerun** | **1,937** | 30 | 0 |

**Against I1:** +15 added (all pass), 0 removed, 0 changed outcomes. The 15 are:
- the nine new contract tests;
- the three carrier tests, each × 2 modes.

The counts equal I91's (1,922 → 1,937).

**Test by test, the new tests are sound.** Each asserts first-failure (gate, code), or admission and standing, on inputs built from shared bases. Each has controls admitted beside it:
- the (4b) test asserts m1–m8, the kept Build, and the other conjuncts as one dict;
- the predicate test breaks each conjunct once;
- the `not_required` test admits W2-published cases by both triggers;
- the G8 tests cover unavailable and `not_required` cases, and dense b = 0 without a parity row;
- the N1 tests read through a ValueError-only caller that names any escape.

One caveat is S-2: three G8 expectations equal RS but not TS.

## 9. Mutants

**Method** (`evidence/harness/make_py_mutants.py`): 35 environment-guarded edits on one copy (P@MUT). `_mx("Qnn")` is true only when `RV113_MUT` names the mutant, and each anchor must match exactly once. Q14–Q18 share one edit of the `not_required` line.

**The runs.** Each mutant ran as one locked job:
- pytest on the 15 B1 and RV108 tests of the two retained test files (`-k "b1_ or rv108"`);
- then my v5 probes and my (f)/(g) probes.

**The control** (`RV113_MUT` unset) passes 15 of 15. Its probe verdicts equal the head's, all 118 probes on all three verdicts. In the full run, it passes all 463 tests of both files. A kill counts only when a test fails by an `AssertionError`. Every failure below is one: there are no errors and no collection failures.

| Id | Mutant | Outcome | Killed by (assertion failures) | Probes moved |
|---|---|---|---|---|
| Q01 | per-case loop scope: requested mode, (b), P1 and P2-P4 checked on selected cases only (I1's sourced scope) | killed | P1/requested mode; P2–P4; repair (b)/(c) | 11 |
| Q02 | requested mode check removed | killed | P1/requested mode | 1 |
| Q03 | P1: at least one mode row instead of exactly one | killed | P1/requested mode | 1 |
| Q04 | P1: mode code 3 (or the other mode's code) accepted | killed | P1/requested mode | 4 |
| Q05 | P2 removed (several parity rows) | killed | P2–P4 | 2 |
| Q06 | P3 removed (a parity row in sparse_interactive) | killed | P2–P4 | 2 |
| Q07 | P4 removed (a parity row on a W2-published case) | killed | P2–P4 | 4 |
| Q08 | (b) removed: the ordinary attempt's material basis per case | killed | repair (b)/(c) | 0 |
| Q09 | (c) count removed: one basis per selector | killed | repair (e) | 0 |
| Q10 | (c) exact case lists removed (selector kept) | killed | repair (b)/(c) | 2 |
| Q11 | (e) removed: a basis's materials checked only through a CaseSource | killed | repair (e) | 0 |
| Q12 | (d1) removed: the invocation's members | killed | repair (d1)/(d2) | 0 |
| Q13 | (d2) removed: the solver mode is one of the two | killed | repair (d1)/(d2) | 0 |
| Q14 | not_required: initial must be a report (Rust's dropped conjunct restored; one shared edit carries Q14-Q18) | killed | P2–P4; not_required | 12 |
| Q15 | not_required: the initial report's outcome must be checks_passed (Rust's dropped conjunct restored) | killed | P2–P4; not_required | 12 |
| Q16 | not_required: W2 must be not_triggered (Rust's dropped conjunct restored) | killed | P2–P4; not_required | 10 |
| Q17 | not_required: product_attempt_ref null dropped | **survives** the 15 B1/RV108 tests and all 463 tests of both files | — | 2 |
| Q18 | not_required: the verdict checks_passed dropped | killed | N2; not_required | 2 |
| Q19 | (4b) branch removed (I1: an entered native stage needs a Run) | killed | (4b) admission | 14 |
| Q20 | (4b) predicate always true (every conjunct dropped) | killed | (4b) admission; predicate | 4 |
| Q21 | (4b): the case's Run and the attempt's proof absent dropped | killed | predicate | 0 |
| Q22 | (4b): an unavailable capture result dropped | killed | predicate | 0 |
| Q23 | (4b): stages after native not_entered dropped | killed | predicate | 0 |
| Q24 | (4b): preparation completed dropped | killed | predicate | 0 |
| Q25 | (4b): the cause naming this attempt dropped | killed | predicate | 0 |
| Q26 | (4b): the reason (source_unavailable, preparation) dropped | killed | predicate | 0 |
| Q27 | (4b): a non-null source reference equal to the case's dropped | killed | (4b) admission; predicate | 4 |
| Q28 | (4b): the case unavailable with prepared_product_failure dropped | killed | predicate | 0 |
| Q29 | the Build check removed (every Build referenced by its building record) | killed | (4b) admission | 2 |
| Q30 | N2 removed: a missing solve_quality read by key (KeyError to the fallback) | killed | N2 | 0 |
| Q31 | N1 guard removed: numerical_quality.status | killed | N1 header; N1 packager; N1 retained (6) | 6 |
| Q32 | N1 guard removed: case solve_quality | killed | N1 header; N1 packager (4) | 0 |
| Q33 | N1 guard removed: case structural_status | killed | N1 header; N1 packager; N1 retained (6) | 6 |
| Q34 | N1 guard removed: case model_matrix_fidelity | killed | N1 header; N1 packager; N1 retained (6) | 6 |
| Q35 | N1 guard removed: case accuracy_evidence | killed | N1 header; N1 packager; N1 retained (6) | 6 |

**Totals: 35 mutants, 34 killed by assertions, 1 survives (Q17).**

- **Q17 is a real gap (S-4).** It drops `product_attempt_ref` null from the `not_required` rule. My SR-RS S-1 shape, a `not_required` case naming its own attempt, then fails at G5 `PRODUCT_ATTEMPT` (the product class) instead of G5 `ATTEMPT`. RS and TS give G5 `ATTEMPT` (probe `nr_product_attempt_ref_with_attempt`). PY's `not_required` test pins only a reference to *another* case's attempt, which G3 refuses first. So no PY test sees the conjunct. This is the gap SR-RS S-1 found in RS, and I90 closed it there with one test row.
- **Q21–Q26 and Q28 are killed only by the predicate's unit test,** and move none of my probes. Each conjunct is also held elsewhere in the reader, as I91 records (D4d, D19, `_g5_typed`, the pipeline and preparation rules). That is N-2.
- **Q08, Q09 and Q11–Q13 move none of my v5 or (f)/(g) probes,** because those probes predate repair 01. My repair probes (§4.2) show the behaviour the tests pin, in all three readers.
- **Q13** shows why the (d2) check must come first. Without it, an unknown mode reaches `mode_code`'s dict lookup, and the fail-closed fallback gives G8 PREPARATION.
- **Agreement with I91.** I91's 40 mutants are all killed. My set adds Q17, which I91 does not have. I91's "G5" mutants restore Rust's three conjuncts (my Q14–Q16), but none drops a kept conjunct (my Q17, Q18).


## 10. The three-reader alignment set, for ROOT to rule together

The set has four items:
- (f), widened to its family (S-1);
- (g), widened to the model-scope members (S-3);
- SR-TS S-1's C2 cause branches;
- SR-TS N-1's transport header scope, now with PY's behaviour.

None of these changes any 07m verdict, and the B1 producer emits none of the shapes.

### 10.1 (f) and its family: recommend G3 COVERAGE, with G5 ATTEMPT for the ordinary attempt's basis

`evidence/fg/FG_TABLE.json`:

| Probe | PY bound / unbound | RS bound / unbound | TS bound / unbound |
|---|---|---|---|
| `material_bases[0].index` 1 (one basis) | eligible / admitted | G8 PREP / admitted | G3 COVERAGE / G3 COVERAGE |
| two bases' indices swapped | eligible / admitted | G8 PREP / admitted | G3 / G3 |
| `sources[0].index` 1; two sources' indices swapped | G8 PREP / admitted | G8 PREP / admitted | G3 / G3 |
| `case_indices` with a duplicate; out of range | G8 PREP / admitted (eligible at I1) | G8 PREP / admitted | G3 / G3 |
| source 1's owner names case 0's id | G8 PREP / admitted | G5 ATTEMPT / G5 ATTEMPT | G3 / G3 |
| an ordinary attempt's basis missing, or not listing its case (§4.2) | G8 PREP / admitted | G8 PREP / admitted | G5 ATTEMPT / G5 ATTEMPT |

**Why G3 rather than Rust's G8.**
1. **These facts are internal to the receipt.** Their check needs no invocation.
2. **G3 runs bound and unbound; G8 runs only bound.** With G8, an unbound read would admit a receipt whose bases or sources are mislabelled, as RS and PY do today.
3. **G3 is where the readers already check the same kind of fact.** PY and RS check product attempts' `id` at their position and the Runs' `execution_order` bijection at G3 COVERAGE. TS already has this whole block at G3, so only RS and PY change, and the change is small.
4. **G8's material-basis checks then mean one thing:** the facts derived from the invocation (each selector, the exact case list by selector, the materials).
5. **For the ordinary attempt's basis reference, G5 ATTEMPT is the ordinary class's reference rule,** like D4b's for the product attempt. TS has it there.

**The alternative.** If ROOT prefers RS's placement (G8 PREPARATION), then TS must move its G3 block and its G5 line to G8. Unbound reads would then admit the whole family in all three readers. That is consistent, but weaker, and I do not recommend it.

**Either way:** PY must add the `material_bases[].index` check, and 07n should pin the members bound and unbound. 07m's `integral_float_integers_and_references` (a `sources[0].index` written as an integral float) stays a must-pass under either gate. RS normalizes it at G2 (D32); TS compares `0 === 0.0`; PY normalizes at G2.

### 10.2 (g) and the model-scope members: recommend G8 INVOCATION with PP's acceptance

| Invocation model edit | PY | RS | TS | PP |
|---|---|---|---|---|
| `reference_configurations` null | eligible | G8 INVOCATION | eligible | blocked (`validate_document`: authored, not 0.4.0) |
| `reference_configurations` `[]` | eligible | G8 INVOCATION | eligible | blocked (the same) |
| `pressure_contract` null (control) | eligible | eligible | eligible | parsed as none |
| `pressure_contract` `{}` | eligible | G8 INVOCATION | G8 INVOCATION | not none: a typed parse failure or a pressure contract, which the readers' rule refuses |
| `pressure_contract` `false` | eligible | G8 INVOCATION | eligible | typed parse failure |
| `combinations` null | eligible | eligible | eligible | typed parse failure (`Vec` with `#[serde(default)]` takes absence, not null) |
| `combinations` `{"x":1}` | G8 INVOCATION | eligible | eligible | typed parse failure |
| `components` `"x"` | G8 INVOCATION | eligible | G8 INVOCATION | typed parse failure |

Unbound, all three readers admit every row, as G8 is bound only. That is right here, because these are invocation facts.

**The recommendation.** G8 INVOCATION_MISMATCH in the model-scope check, before any PREPARATION check (RS's order; PY's and TS's checks already sit there). The rule:
- no `reference_configurations` member, null included;
- `pressure_contract` absent or null;
- `combinations` and `components` absent or `[]`.

The reasons:
- PP blocks or cannot parse every model this rule refuses, so no receipt can bind such an invocation;
- RS's `reference_configurations` rule already reads PP's `is_authored` exactly;
- the gate is the one where every reader already refuses the other namespace facts (schema version, pressure contract).

The fixes are small:
- **PY:** `"reference_configurations" not in model`, `model.get("pressure_contract") is None`, and `model.get(k, []) == []` for the two lists;
- **TS:** the same three in `invocationBinding`'s second `fail`;
- **RS:** replace `list(&model["combinations"]).is_empty()` with "absent or an empty array".

### 10.3 C2 cause branches (SR-TS S-1 and N-2), PY's side

PY is identical to RS on all six C2 probes:
- the branch satisfied: admitted by all three;
- a `receipt_failure` with phase `kernel`, code `facade_certificate`, or phase `preparation`: PY and RS admit (`needs_recompute`), TS gives G5 ATTEMPT;
- a `facade_failure` with phase `kernel`: PY and RS give G5 PRODUCT_ATTEMPT (D19), TS gives G5 ATTEMPT;
- `x_reason_cause_receipt_failure`: PY and RS give G5 PRODUCT_ATTEMPT, TS gives G5 ATTEMPT.

PY names no C2 branch except D4d's `prepared_product_failure` table and D19.

**The recommendation is unchanged from SR-TS:**
- ROOT rules the exact table: TS's set-membership form, or C2's one-to-one form keyed to `check` and `precondition` (SR-TS N-2);
- RS and PY add it at G5 ATTEMPT_MISMATCH in the ordinary class, where TS has it;
- 07n pins each branch, satisfied and broken.

### 10.4 Transport header scope (SR-TS N-1), PY's side

On 07m:
- PY's transport runs the base header check and the preview-physics metadata check, and reports either at G7;
- RS runs the header check alone, at G2;
- TS runs the metadata check alone.

So the nine header mutations read as follows:
- PY: G7 with the base code;
- RS: G2 with the same code;
- TS: six admitted, three G7 `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`.

**The ruling needed:** what transport checks (header, metadata or both) and at which gate. Then align the labels. PY already runs both, so if the ruling is "both, at G2", PY only relabels.

## Host and limits

**Cargo.** The three CLI authorities (`openpipestress_jcs_ijson`, `openpipestress_jcs_binary64`, `openpipestress_units`) were built through `WT/tools/t3_cargo.sh` (`build --locked --offline --release`) into `WT/targets/rv113-pybins`. Every Python job set `OPENPIPESTRESS_CHECKED_JSON_BIN`, `OPENPIPESTRESS_BINARY64_JSON_BIN` and `OPENPIPESTRESS_UNITS_BIN` to these builds.

**Under the lock** (`/usr/bin/lockf -k WT/guard/cargo_job.lock`), each job its own hold:
- the census ×2, the v5 probes ×2 and the differential ×2;
- the suites ×2 and the handoff reruns ×2;
- the (f)/(g) and repair probes on PY ×4;
- the control and 35 mutant runs (pytest and probes in one hold each);
- two full-file runs, the control and Q17.

The RS and TS probe runs:
- **RS:** two cargo jobs through `WT/tools/t3_cargo.sh` (`test --locked --offline --test rv113_census -- rv113_probes --exact`) in P@RS, target `WT/targets/rv113-fgrs`;
- **TS:** two vitest jobs in P@TS, under the lock. NMS was linked at `P/node_modules` and I71's eight wasm assets copied (sha256 verified against my SR-TS record).

I ran no test binary outside the lock. Lock holds are in `evidence/host/`.

**Waits: two slips, disclosed.**
- For the head suite, a foreground wait reached the tool's limit and continued in the background. I then also ran a second short wait on the same job. Both ended when the job ended.
- I launched my first mutant chain with an unsplit id list, so it would have run as one mislabelled control. I stopped it (my own task) while it was still waiting for the lock. It ran nothing, and I relaunched it correctly.

No other job was touched. No wait of mine remains.

**Not done, and not allowed.** No DEC-025, installs or Git writes. I used `git archive` and reads only, with `GIT_OPTIONAL_LOCKS=0`.

**Cleanup.**
- I deleted my copies (`WT/rv113/py-{i1,head,mut}`, `WT/rv113/fg-{rs,ts}`, including the NMS link and the wasm assets) and the targets (`WT/targets/rv113-pybins`, `WT/targets/rv113-fgrs`).
- Scratch is kept in `WT/scratch/rv113_rvr_01/`, and `TMPDIR` pointed there.

**Records.** No record folder is named `build`. Every write used an absolute path. This folder carries placeholder paths only (`sanitize_py.py`; a screen over every file, gzipped ones included, finds no host form).

## For ROOT

1. **SR-PY passes**, with four SHOULD-FIX items, none a PY regression:
   - one PY test row, S-4: a small test-only repair round, as SR-RS's was; I can confirm it as I did RS's;
   - two three-reader alignments, S-1 and S-3;
   - one record correction, S-2.
2. **Rule the alignment set together (§10).**
   - (f) and its family: G3 COVERAGE, with G5 ATTEMPT for the ordinary attempt's basis reference (S-1).
   - (g) and the model-scope members: G8 INVOCATION with PP's acceptance (S-3).
   - C2's table, then RS and PY at G5 ATTEMPT (SR-TS S-1 and N-2).
   - The transport header scope and gate (SR-TS N-1, N-1 here).
3. **Before SC writes 07n's (b)/(c) entries, settle S-2.** With repair 01's inputs, TS gives G5 ATTEMPT where RS and PY give G8 PREPARATION. Either rule S-1 first, or choose inputs that pass TS's G5 check, or declare per reader. REPAIR_01's statement that (b) and (c) have the same gate and code in all three readers should be corrected.
4. **(e) and the Build check are sound.** They agree with RS and TS by run, and each is pinned by a killed mutant.

## Records in this folder

- `REVIEW.md`; `SHA256SUMS` over every file, `REVIEW.md` included.
- `evidence/harness/`:
  - my harness, the differential, the probe generators, the comparators;
  - the mutant schema and table, the run scripts, the sweep list and the sanitizer.
- `evidence/census/`: `PY_CENSUS_07M.json`; `py_i1.jsonl`, `py_head.jsonl`.
- `evidence/probes/`: `probes_v5.json`; `py_i1_v5.jsonl`, `py_head_v5.jsonl`; `PY_PROBE_TABLE.json`.
- `evidence/fg/`: the (f)/(g) probes, the four readers' outputs, `FG_TABLE.json`, and the wasm assets' sha256.
- `evidence/repair_probes/`: the repair-01 probes, the four readers' outputs, `RP_TABLE.json`.
- `evidence/differential/`: `SC_DIFF.json`; `sc_i1.jsonl.gz`, `sc_head.jsonl.gz`.
- `evidence/suites/`: junit XML (gzipped) for both copies and both handoff reruns; `SUITE_COMPARE.json`; the logs' tails.
- `evidence/mutants/`: `MUTANTS.json`, `MUTANT_TABLE_b1.json`, and `runs/b1/<id>/` and `runs/full/{NONE,Q17}/` (run stamps; gzipped pytest log and junit; gzipped probe outputs).
- `evidence/d38/`: my audit, written before reading I91.
- `evidence/host/`:
  - `job_stamps.txt`: every job's queued, start and end stamps;
  - `cargo_holds.txt`: my lines in `WT/guard/cargo_jobs.log`;
  - `authority_binaries.sha256`: my authority builds, which equal I91's `549cc2ca…` and `57064fa9…`.

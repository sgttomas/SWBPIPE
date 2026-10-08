# I92 B1-SR-TS: the TypeScript reader's alignment (R-D38 (4b), F-1 text B per case, G8 per case, G5's `not_required` rule), with RV108 N4 and N6

TASK (Type 2), I92 (I-TS), for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-07 UTC.

**Briefs (verified before work):** `R/BRIEFS/B1_COMMON.md` sha256 `2d170307516b98c40e8aa2dcf352cf11a13dbdd29aeddd78a4c39f3f15eb2c75` and `R/BRIEFS/B1_SR_TS.md` sha256 `9698cd33e6a7fff8f5c08221ec760c0ddcc1fd37e8bd3903d8b42de6342edbfe`, both recomputed on disk and equal to the dispatch. The specification, `R/I84/b1_plan_01/PLAN_v2.md`, is `c85786b704805311485b44ba27c8826ca278c1237d92010f9f997dbf3c919be0` (recomputed). I read NUM's `AGENTS.md` and `agents/AGENT_TASK.md`; PLAN_v2 §0–§2.5 and §4 (R5, R8); DESIGN_v2 (`R/I78/b0_contract_01/DESIGN_v2.md`) §0 header, §1, §2 and §3; RV108's REVIEW (N1–N7) and its N4 probe script; RR's rulings from "Owner decision: SI1c is option D …" to "RV113 passes SR-RS with S-1; …" (the last read when the coordinator relayed its host rule mid-run); I83's RETURN (§0, §6–§8); and I90's RETURN in full. Read-only, for comparison: I90's Rust reader and contract test at `cc81e78801` (`git show`). No other role's instructions were consulted.

**State.** Done. Four commits on `codex/piping-t3-b1-t-20261007` in `WT/b1-t`, over I1 `262bd687f0`. Head **`7e47e51b5d`**. Not pushed; ROOT pushes and merges. The records are in `R/I92/b1_sr_ts_01/`; ROOT commits them.

**Placeholders:** WT, NUM, P, T, R, RR, VENV as in the dispatch; DT = `P/apps/desktop/src`; TS = `DT/features/results/retainedPrecision.ts`; TT = `DT/features/results/retainedPrecision.test.ts`; RS = `P/core/reporting/result_export/src/retained_precision.rs`; PY = `P/core/analysis_runs/retained_precision.py`; S = `WT/scratch/i92_b1_sr_ts`; NMS = the shared `node_modules` that `WT/t6-outputs/P/node_modules` links to; I1 = `262bd687f0`. Code is cited by symbol.


## 0. In brief

| Item | Result |
|---|---|
| **The cascade census (R5)** | **Zero changes.** The aligned reader (TS at `5d7b4447cd`, unchanged since) over 07m (`c21112fd…6807`): all 294 mutations have the same observed first failure as at I1, all 28 must-pass entries are admitted with their stated eligibility and their bases' classifications, and the 17 bases read the same bound, unbound and on transport. The two census logs are byte-identical. No entry moves under N4's repair (§2) |
| (4b) in `productAttempts` | Done (§3.1): the strict "entered native ⇒ Run" check relaxed to R-D38 rules 2, 3, (4a) and (4b); `d38CaptureBeforeRun` carries RS's conjuncts |
| G8's codes to `PREPARATION_MISMATCH`; mode code 3 dropped; every case | Done (§3.2): the requested mode and P1 now report `PREPARATION_MISMATCH`; P1 is 1 sparse / 2 dense; P2–P4 added; the loop covers every case in request order |
| G5's `not_required` rule | TS already had DESIGN's rule (§3.3); now pinned by a test, with the own-attempt form that reaches it |
| D38's obligation `[r1: N-6]` | 24 checks listed (§4): one relaxed; **none extended** (TS already required (4b)'s source equality); the rest shown not to apply or to hold |
| RV108 N4 | Done (§3.4). **TS admits**, by the rule "transport reads no rows": only object rows are projected; a transported successor whose `results` holds a non-object entry is admitted, as in Rust and Python; the full reader still refuses it at G1. Pinned on RV108's 3 bases × 5 forms |
| RV108 N6 | Done (§3.5). `baseHeaderCode`'s doc states each reader's agreement and exception, worded to stay true before and after SR-PY's guard (my choice, recorded). Pinned by a test of every code it names |
| I90's first-failure notes | TS agrees on all of them; one extra variant (a `receipt_failure` cause) differs: TS G5 `ATTEMPT`, RS G5 `PRODUCT_ATTEMPT`, a pre-existing TS-only rule (§9 item 2) |
| Suites, I1 → head | vitest desktop **3,620 → 3,626** (141 files both): +6 added, 0 removed, 0 changed outcomes. `tsc --noEmit` clean on both (§6) |
| Mutants (final head) | **33 run (32 and a supplement): 21 killed, 12 equivalent.** Every mutant the brief names is killed by an assertion; the 12 survivors are (4b)'s redundant conjuncts (§7) |
| Stops (PLAN_v2 §1 fence, R5, R8) | **None fired.** Only TS and TT changed |

## 1. Commits and files

Branch `codex/piping-t3-b1-t-20261007` in `WT/b1-t`, from I1 `262bd687f0`. **Head `7e47e51b5d935fda7a8289d14b21f8979e4f4876`.** Not pushed; ROOT pushes and merges.

| Commit | Content |
|---|---|
| `5d7b4447cd` | TS: R-D38 (4b), F-1 text B per case, RV108 N4 and N6 (the census ran on exactly this TS) |
| `8c0425a3af` | TT: six B1 SR-TS tests, appended |
| `7e5c40f3d5` | TT: one table row, the `not_required` rule on a case owning its own attempt (mutant run 1's G5-4 survivor; §7) |
| `7e47e51b5d` | TT: one table row, P4 on a W2 that was triggered and failed (RV113 N-3's shape, named in ROOT's ruling on RV113; §7) |

`git diff --stat 262bd687f0..7e47e51b5d`: 2 files, +262/−9. Both are in SR-TS's fence (PLAN_v2 §1):

| File | sha256 at I1 | sha256 at the head |
|---|---|---|
| TS | `7f9b47a99e67d78f605dfb8454335446922b3e5fec76b426db4c492b44f25978` | `695f95d5fcc5ff80d24ef4a86b201595e3c69b87098249e1ffce71cccf2e1741` |
| TT | `9d6071c331214cf7e4eb41f1c177df255c4f10fe146ae29ca7c1051bb90d9e5b` | `4756deac8cd48901f8987c7b29d0c36183fa5cefbcad7edea3723c3649de634a` |

TT's diff only adds lines (no removed line). The full diff is `_run_records/diff/sr_ts.diff`.

## 2. The cascade census (R5)

**Method** (`_run_records/census/`). `zzI92Census.test.ts` (scratch only; its harness is TT's `applyEdits`, `rehash` and `applyEntry`, copied verbatim) runs the reader over every 07m entry and writes one JSON line each: for each of the 294 mutations, the observed (gate, code, detail) or the pass record; for each of the 28 must-pass entries, the result (`invocation_bound`, `numerical_eligible`, `standing`, `publication_sha256`, and a key-order-independent sha256 of the classifications); and for each of the 17 bases, the bound, unbound and transport results. It ran in two `git archive` copies of `P` without `execution/`, each with NMS linked and the eight wasm assets copied:
- **base:** I1, before any edit (`S/base`);
- **aligned:** I1 with TS replaced by the aligned reader, sha256 `695f95d5…2e1741`, which is exactly TS at commit `5d7b4447cd` (and at the head). No test file changed.

`census.py` compares the two logs entry by entry against 07m's id lists, and checks each side against the corpus's own expectations (the TypeScript `expected_by_reader` where present).

**Result** (`census.out`): `CENSUS mutations 294 (base lines 294, aligned lines 294); must_pass 28; bases 17; expectation misses base 0 aligned 0; changes 0`. 07m is `c21112fdbfad37dd4832c8dd64d066e1809dd70dce6c89cb920d1d6dd3d46807`. The two logs are **byte-identical**, details included. **R5 does not fire.**
- Every mutation's observed (gate, code) equals its expectation on both sides; the 22 G8 entries are the 2 `INVOCATION_MISMATCH` ones (the invocation digest and model schema 0.4.0, whose codes B1 does not change) and 20 `PREPARATION_MISMATCH`.
- Every must-pass entry is admitted with its stated eligibility (18 eligible, 10 not) and its base's classifications; every base reads as stated, unbound and on transport too.
- **No entry moves under RV108 N4's repair:** 07m has no transported statement with a non-object row (the harness's transport form deletes `results`).

This agrees with I84's static reading and I90's census. Every base's mode row is 1 (sparse) or 2 (dense), and the two dense bases carry their one parity row on a selected b = 0 case. No entry changes a mode row's value, kind or case, adds, removes or moves a row, or reaches (4b). Of the 165 edits under `results` in 07m, 146 set or remove another row's `recovery_method`, one sets another row's `id`, and the 18 on a mode or parity row touch only its `recovery_method` (16), `metadata` (1) or `evidence` (1) (`_run_records/census/results_edits.txt`).

## 3. The changes (commit `5d7b4447cd`, TS only)

### 3.1 R-D38 (DESIGN_v2 §2), in `productAttempts`

Before: `fail((stage.native === 'not_entered') === (a.run_ref === null))`, R-D38's rule 4 without (4b).

After: `fail(stage.native === 'not_entered' ? a.run_ref === null : a.run_ref !== null || (stage.native === 'failed' && d38CaptureBeforeRun(a, c, s)))`, G5 `PRODUCT_ATTEMPT_MISMATCH`:
- not entered ⇒ no Run (rule 2);
- completed ⇒ a Run (rule 3); the existing check below binds a completed stage to a selected terminal;
- failed ⇒ a Run, whose terminal the same check binds to non-selected (4a), **or** `d38CaptureBeforeRun` (4b).

`d38CaptureBeforeRun(a, c, s)` is Rust's `d38_capture_before_run`: an unavailable result with `error.kind == "capture"`; preparation completed and every stage after native not entered; the case unavailable, with cause `prepared_product_failure` naming this attempt and reason (`source_unavailable`, `preparation`); and a resolved source that binds this attempt and equals the case's own. The rest of (4b) holds for every receipt (§4, items 2, 6, 11 and 12). The predicate's doc names the later checks that also enforce each conjunct.

Rule 1 (`(a.run_ref === null) === !c.run`) is unchanged.

### 3.2 F-1 text B and the requested mode in G8 (DESIGN_v2 §3.2–§3.3), in `invocationBinding`

The per-case loop runs over every case of the invocation, in request order (G3 has already bound the receipt's cases to the invocation's `load_cases`), whatever the case's status. For case `ci`, with `o = ordinary_attempts[ci]` (G3 binds `cases[ci].ordinary.attempt_ref == ci`):
1. `o.requested_mode == invocation.solver_mode` — **its code changes from `INVOCATION_MISMATCH` to `PREPARATION_MISMATCH`**;
2. `o.material_basis_ref` — unchanged (already `PREPARATION_MISMATCH`), now before P1 as DESIGN orders it;
3. **P1:** exactly one `linear_solver_mode_basis` row of the case, valued 1 in `sparse_interactive` and 2 in `dense_scrutiny` — **mode code 3 is dropped**, and the code changes from `INVOCATION_MISMATCH` to `PREPARATION_MISMATCH`;
4. **P2** (at most one `sparse_live_path_dense_parity_relative_delta` row of the case), **P3** (none in `sparse_interactive`), **P4** (none when `o.w2.kind == "published"`), each `PREPARATION_MISMATCH` and each its own `fail`, so that each has its own mutant. TS had no parity-row rule before.

An absent parity row in dense mode at b = 0 is admitted (the disclosed limit). P5 is not implemented (deferred, decision 10).

**Order inside G8.** The checks before the loop (invocation keys, digest, project, schema version, model scope: `INVOCATION_MISMATCH`, unchanged) still come first. The material-basis loop and the source and attempt loops follow, all `PREPARATION_MISMATCH`, so the loop's place among them does not change a (gate, code). DESIGN lists the material checks before the loop; as in I90's §3.2, I did not move TS's existing loops.

### 3.3 G5's `not_required` rule (DESIGN_v2 §3.3, decision 9), in `ordinaryAttempts`

No change. DESIGN says TS already has exactly the rule (`product_attempt_ref` null, `initial.kind != "not_attempted"`, verdict `checks_passed`), and it does: `ordinaryAttempts` never had RS's three conjuncts. TS keeps its report-outcome equality (`initial.outcome == q.solve_quality` for a report, with no W2 guard; equivalent under D6c, as DESIGN says). The new test (§5, test 2) pins the rule, and the mutants restoring each of RS's dropped conjuncts in TS are killed (§7).

### 3.4 RV108 N4, in `projection`: TS **admits**, by the rule "transport reads no rows"

Before: `for (const row of p.results) if (Object.hasOwn(row, 'recovery_method')) …` raised `TypeError` on a `null` entry, which the transport validator's G7 catch reported as `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`.

After: only object rows (`isObj(row)`) lose `recovery_method`; any other entry (null, a number, a string, an array) is left as it is. The rule, as the new doc states it:
- **the full reader** has already refused any non-object entry at G1 (`RawRow`), before the projection; nothing changes there;
- **the transport reading reads no rows** (`validatePreviewPhysicsTransportMetadata` reads only `formulation_basis` and `contract_evidence`), so a transported successor whose `results` holds a non-object entry is **admitted**, with the same result as without it. This is Rust's reading (its transport projection, `project(source, false)`, touches no row; its raw projection skips a non-object row, `if let Some(o) = r.as_object_mut()`, which is exactly TS's new guard) and Python's since B6 (`_transport_g7` skips a row unless `type(row) is dict`).

### 3.5 RV108 N6, `baseHeaderCode`'s doc comment: reworded to say what holds now, in a form that stays true after SR-PY

**My choice:** neither "what holds now" alone (it would go stale when SR-PY's guard lands at I4) nor "true after SR-PY" alone (it would be false on `b1-t` until I4). The doc now states each reader's agreement with its exception and its end:
- **Rust** gives the same code on every single-defect header refusal except the declared inherited `carrier_evidence` class, which Rust's header does not read; with both a `contract_evidence` and a `source_block_recovery` defect, Rust reports the latter (I83's T9; inherited). This is RV108 §1.1–§1.2's finding.
- **Python** gives the same code on every header refusal, except that a list- or dict-valued enum raised `TypeError` in `_source_contract` (Python's G7 fallback, `SOURCE_PREVIEW_PHYSICS_INVALID`) **until B1 SR-PY's type guard** (RV108 N1, ruled in for SR-PY in RR "SR-PY prepared; the N1 guard …"); with that guard Python gives this code there too.

The removed sentence was "so the G7 code agrees across the languages". The new test (§5, test 6) pins TS's side of every claim the doc makes.

## 4. D38's list `[r1: N-6]`: every TS check that assumes a prepared source has a Call or a Run

The (4b) shape: an unavailable case whose attempt prepared a registered CaseSource and whose native stage failed before any Run; no Run, Call or Group `source_refs` entry, Build or `execution_order` entry names it. I read every reference to `run`, `run_ref`, `calls`, `groups`, `builds` and `execution_order` in TS, and every check on a case's or attempt's source.

| # | Check (function) | What it assumes | Disposition |
|---|---|---|---|
| 1 | `productAttempts`: `(stage.native === 'not_entered') === (a.run_ref === null)` | an entered native stage has a Run | **Relaxed to (4b)** (§3.1). Rule 2 (not entered ⇒ no Run) and rule 3 (completed ⇒ a Run) are kept in the relaxed form. Restoring the old check is mutant D38-1; dropping rule 3 from the relaxed form is D38-2 |
| 2 | `productAttempts`: `(a.run_ref === null) === !c.run`, and with a Run its id, origin source and owner (C3:167, rule 1) | — | No Call or Run assumption beyond rule 1; for (4b) both are null. Unchanged |
| 3 | `productAttempts`: with a `run_ref`, native completed exactly when the Run is selected | — | Runs only with a Run; does not apply. Unchanged |
| 4 | `productAttempts`: the source association, for every attempt with a source: owner, material basis, `preparation.attempt_ref`, and **`c.source_ref === a.source_ref`** | — | No Run assumption. It already carries (4b)'s source equality for every attempt (DESIGN_v2 §2 `[r01: N-6]`: "TS already requires"), so TS needs **no extension** here (RS's item 2 was one). Unchanged |
| 5 | `productAttempts`: with a source, preparation completed, member counts, `old_coverage` complete | — | No Run assumption; holds for (4b). Unchanged |
| 6 | `productAttempts`: `proof` null exactly when `proof_start` not entered; `proof_start` entered ⇒ native completed; no proof ⇒ every later stage not entered | — | (4b) has no proof; holds. Unchanged |
| 7 | `productAttempts` (I57 §3): a non-null `summary_coverage` needs the attempt's own selected Run | a vector follows a Run | Does not apply: (4b) has no proof |
| 8 | `productAttempts`: Ready ⇒ a selected Run and every stage completed | a Ready result follows a Run | Does not apply: (4b) is unavailable |
| 9 | `productAttempts` (D4d reason table, D30): `preparation` ⇒ no Run; `native` ⇒ the case's own non-selected Run; `capture` with no Run ⇒ (`source_unavailable`, `preparation`); `capture` with a non-selected Run ⇒ kernel; otherwise a selected Run (facade) | — | Already admits (4b) by its capture-without-Run arm. Unchanged |
| 10 | `errorStageRecordAgrees` (D37): `capture` admits `CF--------` ("solve_native before a Run"); the typed check/result pairing allows `native` or `capture` at a failed native stage | — | Already admits (4b). Unchanged |
| 11 | `nativeClass` (G5 class 1): Call ids and kind; `owner_refs`, `source_refs` and `run_refs` of equal length; every position's Run, source and owner its own case's; Group sources within its Call and equal to its Runs' sources; C5 group formation over Runs; every Build built by a record; `charged` the running meter | constraints on the Runs, Calls, Groups and Builds that exist | Does not apply: no check requires a source to appear in a Call or Group. These checks enforce (4b)'s "nothing names it" (m7, call or group: G5 `ATTEMPT_MISMATCH`). Not relaxed |
| 12 | `coverage` (G3): Run ids are positions, and `execution_order` equals the Runs' owners | keyed on `case.run` | Does not apply; it enforces (4b)'s `execution_order` clause (m4: G3 `COVERAGE_MISMATCH`) |
| 13 | `coverage` (G3): an attempt's old member ids against its resolved source's member map; a coverage vector against the source's bodies (only with a vector) | — | No Run assumption; holds |
| 14 | `coverage` (G3, D29): every CaseSource has a non-empty body inventory, its index, and a case owner | — | No Run assumption; holds |
| 15 | `integrity` (G1): the source identity digest of selected cases; the preparation digest of each source whose attempt's members are all prepared | — | No Run assumption; the (4b) source's preparation digest is checked. Holds |
| 16 | `unselectedCoverage` (G5a): a non-selected attempt with a complete roster reads its case's Run's last verification record | a roster follows a Run | Does not apply: (4b) has no proof (item 7 ties a roster to a selected Run) |
| 17 | `numericalCases`, `numericSummaries`, `selectedCoverage`, `numericalScales`, `classifications` (G5a–G5c) | selected cases | Does not apply |
| 18 | G6: a non-selected case's rows carry no `recovery_method` | — | No Run assumption; holds |
| 19 | `ordinaryAttempts` (G5): C2's cause branches for causes other than `prepared_product_failure` (`facade_failure` needs a selected Run, a kernel cause a Run; `source_error` and `unavailable_precondition` no Run); O5's `source_decline` | — | Does not apply: (4b)'s cause is `prepared_product_failure` |
| 20 | `invocationBinding` (G8) source loop: binds every CaseSource to its owner case from the invocation and recomputes `kernel_source_sha256` and `stiffness_sha256` from the binding (`nativeSourceHashes`) | — | No Call needed (DESIGN_v2 §2's S-2 reading: C2 §3's digests are raw sha256 of the source's own encodings); holds |
| 21 | `invocationBinding` (G8) attempt loop: binds old inputs, and with a source its section terms and the new evaluator's values | — | No Run assumption; holds |
| 22 | `diagnostics` (G4): an unavailable case has exactly one `RETAINED_PRECISION_UNAVAILABLE` diagnostic, its `diagnostic_ref` | — | No Run assumption; holds |
| 23 | `nativeRuns` (D8 kernel scope): no `work_accounting` stop or reason in any Run, build or group preparation | the Runs that exist | Does not apply |
| 24 | `validateRetainedPrecision`'s eligibility: every case selected or `not_required` | — | (4b)'s case is unavailable, so the statement is not eligible; standing `needs_recompute` (tested) |

**Outside TS**, `retainedPrecisionStanding.ts` and `retainedPrecisionDisclosure.ts` read only the reader's result; neither names a Run, Call or source (checked). They are outside the fence and unchanged.

**The (4b) predicate.** `d38CaptureBeforeRun` carries Rust's eleven conjuncts (`d38_capture_before_run`), plus the source's binding of the attempt. In TS **every** conjunct is also enforced by a later check of `productAttempts` with the same gate and code (items 4–6, 9, 10 and D19), so each conjunct's mutant is equivalent (§7). In RS the source equality is the one conjunct enforced nowhere else; in TS it is item 4. I kept the predicate whole so that (4b) reads as DESIGN writes it and as RS does (ROOT's ruling 2 on I90), and does not widen if a later check changes.

## 5. The tests (commits `8c0425a3af`, `7e5c40f3d5` and `7e47e51b5d`, TT only; six `it`s appended in one `describe`, no existing test changed)

They are reader-local, on receipts derived from the shared bases and must-pass entries by edits and the file's own full reseal (`applyEntry`, `rehash`), mirroring I90's four `b1_*` Rust tests edit for edit. They are not shared corpus entries; SC's 07n pins those. Each verdict is TS's first failure, or `{admitted: numerical_eligible}`; each table reports every miss at once.

**Test 1: R-D38 (4b) beside a selected case.** On `two_case_facade_after_certificate_synthetic`, case 1 is rewritten into (4b) exactly as Rust's `d38_edits` rewrites it (its Run, `execution_order`, Call and Group entries removed; the call's after-value and `charged` recomputed; case 1's Run built nothing, which the test asserts; a typed `CaptureError::Origin`; every hash resealed). It is admitted with G0–G8 passing: bound, not eligible, `needs_recompute`; unbound, `needs_recompute`; case 0's classifications are the base's; transport admits it. The variants, against Rust's first failures (I90 RETURN §5, test 1, and §9 item 2):

| Variant | TS first failure | RS (I90) | Same? |
|---|---|---|---|
| m1: `error` = `{kind: native, run_ref: 1}` | G5 `PRODUCT_ATTEMPT` | G5 `PRODUCT_ATTEMPT` | yes |
| m1: only `error.kind` set to `native` (the cause stays) | G1 `RECEIPT` | G1 `RECEIPT` (I90 §9 item 2) | yes |
| m2: native `completed` | G5 `PRODUCT_ATTEMPT` | same | yes |
| m3: `run_ref` 1 while the case has no Run | G5 `PRODUCT_ATTEMPT` | same | yes |
| m4: `execution_order` still lists the case | G3 `COVERAGE` | G3 `COVERAGE` | yes |
| m5: `proof_start` completed | G5 `PRODUCT_ATTEMPT` | same | yes |
| m6: `source_ref` null, preparation completed | G5 `PRODUCT_ATTEMPT` | same | yes |
| m7: the case's source in the call's `source_refs` | G5 `ATTEMPT` | G5 `ATTEMPT` | yes |
| m7: in the group's `source_refs` | G5 `ATTEMPT` | G5 `ATTEMPT` | yes |
| m8: `case.source_ref` 0 ≠ the attempt's 1 | G5 `PRODUCT_ATTEMPT` | same | yes |
| result `ready`; preparation `failed`; observables and G5a entered; reason code `kernel_unresolved`; reason phase `kernel`; the cause naming the other attempt | G5 `PRODUCT_ATTEMPT` each | same each | yes |
| the cause a `receipt_failure` (phase still `preparation`) | **G5 `ATTEMPT`** | G5 `PRODUCT_ATTEMPT` | **no** (§9 item 2) |
| the cause a `receipt_failure` with phase `receipt`, code `receipt_encoding` (TS-only variant) | G5 `PRODUCT_ATTEMPT` | not run | — |

**Test 2: G5 `not_required`.** On 07j's must-pass entry (selected, then `not_required`), case 1 is made W2-published with the verdict `checks_passed`, by an evaluation trigger and by a formation trigger (Rust's edits). Each is admitted, bound, **eligible**, standing `eligible`, and `ordinaryAttempts` does not throw. Still refused: W2-published with verdict `sensitive` (G5 `ATTEMPT`); `initial` `not_attempted` (G5 `ATTEMPT`); a report whose outcome differs from the verdict (G5 `ATTEMPT`, the kept equality); `product_attempt_ref` non-null (**G3 `COVERAGE`**, case 0's attempt; as in Rust). All four agree with I90's. Added in `7e5c40f3d5`: the case keeping **its own** attempt (07j's edits without the attempt's removal, and `product_attempt_ref` 1) passes G3 and is refused by the rule itself at **G5 `ATTEMPT`**, before D19's `PRODUCT_ATTEMPT`. This is the form ROOT's ruling on RV113 S-1 names for SC's 07n entry.

**Test 3: G8 P1 and the requested mode, for every case.** Each is refused at G8 `PREPARATION`: on the unavailable case, the dense code in sparse, mode code 3, two mode rows, the requested mode flipped (Rust's four); on the `not_required` case, the dense code in sparse (Rust's), mode code 3, no mode row; on a selected case, mode code 3, the requested mode flipped, and the sparse code in dense mode.

**Test 4: G8 P2–P4, for every case.** 07j's two-case statement is made dense, as in Rust's test. Admitted (eligible): no parity row on either case (so the dense selected case at b = 0 has none, the disclosed limit); one parity row on the `not_required` case at b = 0; a W2-published `not_required` case with none; and (added in `7e47e51b5d`, RV113 N-3's shape) one parity row on a `not_required` case whose W2 was **triggered and failed**, which published nothing, so b = 0. Refused at G8 `PREPARATION`: P2 (two parity rows on the `not_required` case; on the dense selected case), P4 (a parity row on the W2-published `not_required` case; on a W2-published selected case), P3 (a parity row on the sparse unavailable case, Rust's; on the sparse selected case, added). A non-selected case's parity row that keeps `recovery_method` is **G6 `ROW_METHOD`** first, as I90's note says for RS.

**Test 5: RV108 N4.** On RV108's three bases (`ordinary_prepared_synthetic` and both milestone successors), RV108's two forms (`results[0] = null`, `results += null`) and three more non-object entries (`1`, `"row"`, `[]`): the transport validator returns exactly the result it returns for the untouched source, and the full reader refuses each at G1 `RECEIPT`. RV108's other row forms (`results` a string, null, empty) stay admitted on transport.

**Test 6: RV108 N6.** TS's side of each claim in the reworded doc: a list- and a dict-valued `structural_status`, `model_matrix_fidelity` and `accuracy_evidence` give G7 `SOURCE_NUMERICAL_CASE_INVALID`, and `numerical_quality.status` G7 `SOURCE_NUMERICAL_QUALITY_INVALID` (Rust's codes, and Python's with SR-PY's guard); `carrier_evidence` with a case defect gives `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` (the declared class); a null `contract_evidence` with `source_block_recovery` present gives `SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED` (Python's order).

**Against I1's reader** (`_run_records/tests/head_tests_at_I1.txt`; TT as at `8c0425a3af`, run on an archive of I1, before the two rows added later; by reading, not by a rerun, those rows would not change these outcomes): tests 1, 3, 4 and 5 fail, and tests 2 and 6 pass, as they should (TS already had the `not_required` rule and the header codes). At I1, the (4b) receipt is refused at G5 `PRODUCT_ATTEMPT`; mode and requested-mode defects report G8 `INVOCATION_MISMATCH`; mode code 3 is admitted in sparse; no P2–P4 variant is refused; and the null-row transport is refused at G7 `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID` (RV108 N4).

## 6. Evidence per item

### 6.1 Suites, I1 against the head (`_run_records/suites/`)

Each ran under the lock (`scripts/suites.sh`). Base: an archive of I1 (`P` without `execution/`) in `S/base`. Head: `WT/b1-t` at `7e47e51b5d`, a clean tree. Both had NMS linked at `P/node_modules` and the eight wasm assets copied into `P/apps/desktop/public/`.

| Suite | I1 | Head | Test by test |
|---|---|---|---|
| vitest, the whole desktop suite (141 files) | 3,620 passed | **3,626** passed | **+6 added** (the six B1 SR-TS tests, all passed), 0 removed, 0 changed outcomes (`suites_compare.json`) |
| `tsc --noEmit -p tsconfig.json` | rc 0, no output | rc 0, no output | — |

Every count change is an added test. The TS reader's own file, TT, passes in full at the head (490 of 490, mutant N0), including every 07m entry.

### 6.2 Per item

| Item | Evidence |
|---|---|
| Census (R5) | §2; `census/` (both logs, the comparison, the census test and script, and the list of 07m's `results` edits) |
| (4b) | §3.1; test 1 (admitted, needs_recompute; 18 variants with their first failures); D38-1 and D38-2 killed |
| G8 codes, mode code 3, every case, P1–P4 | §3.2; tests 3 and 4; G8-1 to G8-11 killed |
| G5 `not_required` | §3.3; test 2; G5-1 to G5-6 killed |
| D38's list | §4 |
| **N4** | §3.4; test 5 (RV108's three bases; the two null forms and three more; the full reader at G1); N4-1 killed; at I1 the null-row transport is refused at G7 `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`, as RV108 found |
| **N6** | §3.5; test 6 (each code the reworded doc names) |
| I90's first failures | §5, test 1's table; §9 items 2 and 3 |
| The new tests fail where they should at I1 | §5's last paragraph; `tests/head_tests_at_I1.txt` |

## 7. Mutants (`_run_records/mutants/`)

`mutants.py` applies each mutant as one exact string edit (each edit matches TS exactly once) to TS in a scratch archive of the head (`S/mut`: TS sha256 `695f95d5…2e1741`, TT the head's `4756deac…4a`), runs TT (`vitest run src/features/results/retainedPrecision.test.ts`, the TS lane, JSON reporter), then restores TS and checks its sha256. A mutant counts as **killed only when the file loaded (all 490 tests collected) and at least one assertion failed**; none was killed by a load or transform failure (vitest transpiles without type checks, and every mutant collected 490 tests). The whole programme ran as one job under the lock (17:27:32–17:32:31Z). N0 (unmutated) passes 490 of 490; TS's bytes were restored after each mutant and at the end.

**Two runs.** Run 1 (16:36:51–16:41:46Z, on `8c0425a3af`'s TT, `_run_records/mutants/run1/`) left G5-4 surviving. It is **not** equivalent: with the `product_attempt_ref` conjunct gone, a `not_required` case owning its own attempt passes G3 and is refused by D19 at `PRODUCT_ATTEMPT_MISMATCH` instead of the rule's `ATTEMPT_MISMATCH`. Commit `7e5c40f3d5` adds that variant (RV113's S-1 found the same gap in RS). Then ROOT's ruling on RV113 named N-3 (P4 keyed on published, not triggered); commit `7e47e51b5d` adds the TS witness and the programme gained G8-11. Run 2, below, is on the final head.

| Mutant | One edit | Result | Killed by |
|---|---|---|---|
| **D38-1** | the relaxed check restored: `(native == not_entered) == (run_ref == null)` | **killed** | test 1 (the (4b) receipt is refused) |
| **D38-2** | rule 3 dropped from the relaxed form ((4b) admits a completed native stage with no Run) | **killed** | test 1 (m2 is admitted) |
| D38-15 | (4a) dropped from the relaxed check: a failed native stage needs (4b) even with a Run (supplement, after RV113 N-1) | killed | I1's D30 reader-logic test (`productAttempts` passes a native error naming its own non-selected Run) |
| **G8-1** | P1–P4 for case 0 only (the per-case loop narrowed) | **killed** | tests 3 and 4 |
| **G8-2** | P1–P4 for selected cases only | **killed** | tests 3 and 4 |
| **G8-3** | P1 dropped | **killed** | test 3 |
| **G8-4** | P1's "exactly one" dropped (the first row's value only) | **killed** | test 3 (two mode rows) |
| **G8-5** | **P2** dropped | **killed** | test 4 |
| **G8-6** | **P3** dropped | **killed** | test 4 |
| **G8-7** | **P4** dropped | **killed** | test 4 |
| G8-11 | P4 keyed on a triggered W2 instead of a published one (RV113 N-3's shape) | killed | test 4 (the W2-failed row) |
| **G8-8** | **mode code 3 restored** (`[1, 3]` in sparse) | **killed** | test 3 |
| **G8-9** | the requested mode's code back to `INVOCATION_MISMATCH` | **killed** | test 3 |
| **G8-10** | P1's code back to `INVOCATION_MISMATCH` | **killed** | test 3 |
| **G5-1** | `not_required`: RS's `initial.kind == "report"` restored | **killed** | tests 2 and 4 |
| **G5-2** | `not_required`: RS's `initial.outcome == "checks_passed"` restored | **killed** | tests 2 and 4 |
| **G5-3** | `not_required`: RS's `w2.kind == "not_triggered"` restored | **killed** | tests 2 and 4 |
| G5-5 | `not_required`: the kept `initial.kind != "not_attempted"` dropped | killed | test 2 |
| G5-6 | `not_required`: the kept verdict `checks_passed` dropped | killed | test 2 |
| **N4-1** | **the N4 rule removed** (`Object.hasOwn` on every row) | **killed** | test 5 |
| G5-4 | `not_required`: the kept `product_attempt_ref == null` dropped | killed (run 2) | test 2 (the own-attempt row) |
| D38-3 | (4b) result unavailable dropped | survived, equivalent | trivially inside the predicate (`error?.kind` is undefined on a Ready result); and Ready ⇒ a selected Run (§4 item 8) |
| D38-4 | (4b) `error.kind == "capture"` dropped | survived, equivalent | D4d's reason table: `native` needs the case's Run (§4 item 9) |
| D38-5 | (4b) preparation completed dropped | survived, equivalent | an entered native stage needs a completed preparation (stage order) |
| D38-6 | (4b) later stages not entered dropped | survived, equivalent | the proof/stage rules (§4 item 6) |
| D38-7 | (4b) case unavailable dropped | survived, equivalent | D19: an unavailable attempt is reported through its case's `prepared_product_failure` |
| D38-8 | (4b) cause kind dropped | survived, equivalent | D19 |
| D38-9 | (4b) cause names this attempt dropped | survived, equivalent | D4c and D19 |
| D38-10 | (4b) reason code dropped | survived, equivalent | D4d's `capture`-without-Run arm |
| D38-11 | (4b) reason phase dropped | survived, equivalent | the same |
| D38-12 | (4b) source non-null dropped | survived, equivalent | preparation completed ⇔ a source |
| D38-13 | (4b) the source binds the attempt dropped | survived, equivalent | the source association (§4 item 4) |
| D38-14 | (4b) source equality dropped | survived, equivalent | the source association (§4 item 4) |

**Run 2: 33 runs, N0 and 32 mutants: 20 killed, 12 survived, every survivor an equivalent (4b) conjunct.** A one-mutant supplement on the same head (D38-15, 17:40:55–17:41:05Z, under the lock; `mutants/supplement/`) was killed. Every mutant the brief names is killed by an assertion: the per-case G8 loop (G8-1, G8-2), each of G5's three dropped conjuncts restored (G5-1 to G5-3), the relaxed D38 check restored (D38-1), P2–P4 (G8-5 to G8-7), mode code 3 restored (G8-8) and the N4 rule removed (N4-1). For every (4b) survivor, test 1's matching variant is still refused at G5 `PRODUCT_ATTEMPT_MISMATCH` by the later check named, which is why the mutant survives (§4, "The (4b) predicate"). Apart from D38-15, which I1's D30 test kills, no pre-existing test of TT (07m's entries included) kills any of the killed mutants; only the new tests do. This matches the census: 07m does not exercise B1's rules.

## 8. Host and limits

- **No cargo, pytest, DEC-025, evidence sweep, native or solver job, and no installs.** My jobs were vitest, `tsc` and Python scripts (VENV) over committed bytes.
- **The shared lock** (`/usr/bin/lockf -k WT/guard/cargo_job.lock`). Under it: the whole desktop vitest suite and `tsc --noEmit`, for base and for head (four holds), the mutant programme, each run as one hold covering its 33 vitest runs (run 1, 16:36:51–16:41:46Z; run 2, 17:27:32–17:32:31Z); and the one-mutant supplement (17:40:55–17:41:05Z). Timestamps are in `suites/{base,head}_rc.txt` and `mutants/{,run1/,supplement/}mutants_rc.txt`. Ten light vitest runs of one to three files, 1.4–10 s each, ran without the lock, as I83's did: three census runs (the first before the classification digest was made key-order-independent), six runs of the new tests, of TT or of the three retained-precision test files in the aligned scratch copy while writing them, and one of the head's tests against I1. ROOT's restatement of the host rule (RR "RV113 passes SR-RS …", relayed mid-run) arrived after these; every heavy run, the mutant loops included, was already under the lock, and I ran no further vitest outside it. I waited for other jobs and killed none.
- **Waits.** One wait per job. The suites job: a monitor, which I stopped and replaced with a single process wait (`caffeinate -w`) on its first script process. That wait ended when the process had gone; the second script process, in the same background job, ended with the job's own completion notice. Each mutant job, and the supplement: its background completion notice. No wait of mine is running at return.
- **Node and the wasm assets, as I83 did.** For the head's suite, the untracked `WT/b1-t/P/node_modules` → NMS link was created, and the eight wasm assets were copied (not built) from `WT/sweep-skewpin/P/apps/desktop/public/` into `WT/b1-t/P/apps/desktop/public/`. Their sha256 values equal I83's (I71's) eight (`suites/wasm_assets_worktree.sha256`). Both are removed, along with vitest's cache. At return, `git status --short --ignored` shows nothing under P's `apps`, `core`, `tests`, `fixtures` or `node_modules`. NMS's pre-existing `.vite-temp/` is empty; only its mtime moved. The scratch archives (`S/base`, `S/aligned`, `S/mut`) had their own links and copies; I deleted the three archives at the end.
- **Scratch and temp.** Scratch was `S` only, with `TMPDIR` in it for every run. Nothing I ran wrote to the system temp directory. Disclosed: the agent host writes its own background-task output under the system temp directory; that is the harness, not my tools.
- **Writes.** Every write used an absolute path. No record folder is named `build`. Records carry placeholder paths only (§10).
- **Git.** Four commits on `codex/piping-t3-b1-t-20261007` only; no push. Reads used `GIT_OPTIONAL_LOCKS=0`. The worktree is clean at the head. Git prints a pre-existing warning about a symbolic-link loop under `R/REVIEW_RV58/…/fixture/.gitignore`; it is not mine and changes nothing.
- **Fence.** Only TS and TT changed. No FK, schema, base-reader, reviewed-input or D1-visibility change, and no c = 1 byte question (TS is not on the D1 call graph). I touched none of SP's, SA's or SR-PY's files; I read I90's committed Rust files with `git show` only.

## 9. For ROOT

1. **The census is clean (R5 does not fire).** SR-TS can go to RV-R (RV113).
2. **One first-failure difference from RS, outside m1–m8, and pre-existing.** On I90's (4b) variant "the cause is not a prepared product failure" (`receipt_failure`, phase left `preparation`), TS fails at **G5 `ATTEMPT_MISMATCH`** and RS at G5 `PRODUCT_ATTEMPT_MISMATCH`.
   - **Cause:** TS's `ordinaryAttempts` (G5's ordinary class, which runs before the product class in both readers) checks C2's cause branches for every unavailable case whose cause is not `prepared_product_failure`: a `receipt_failure` needs phase `receipt` and a receipt code, a `facade_failure` phase `facade`, and so on. RS and PY have no such branch rule (neither names `receipt_encoding`, `publication_hash_range` or `caller_not_qualified`), so they reach D19 in the product class. With the branch satisfied (phase `receipt`, code `receipt_encoding`), TS too reports D19's `PRODUCT_ATTEMPT_MISMATCH`.
   - **It predates B1** (TS's C2 branch rules are I1's) and does not touch (4b): it is a TS-only refusal rule on a compound defect.
   - **For SC:** a shared 07n entry of that compound form needs per-reader expectations, or the branch-satisfying form. ROOT may also want RV-R to say whether C2's branch rules should exist in RS and PY, or be declared.
3. **I90's other first-failure notes all hold in TS** (§5): m4 at G3 `COVERAGE`, m7 (call and group) at G5 `ATTEMPT`, m1 with the whole error value at G5 `PRODUCT_ATTEMPT` and with `kind` alone at G1 `RECEIPT`, `not_required`'s non-null `product_attempt_ref` at G3 `COVERAGE`, and a non-selected case's parity row keeping `recovery_method` at G6 `ROW_METHOD`.
4. **(4b)'s predicate is wholly redundant in TS** (§4, §7): unlike RS, where the source equality is the one conjunct enforced nowhere else, TS's source association already enforces it for every attempt. I kept the predicate whole for the three-reader shape (ROOT's ruling 2 on I90); RV-R may prefer the minimal form, which here would be only the relaxed check with rule 3 (`native == completed ⇒ run_ref`), behaviour-identical at `validateRetainedPrecision`.
5. **The N6 label.** RV108's own N6 row names Python's transport docstring and the case file's scope sentence; the TS doc-comment overclaim is described in RV108's N1 row. ROOT's ruling and my brief route "TS's doc comment" as N6, and that is what I changed (§3.5). N6(a) and (b) themselves are SR-PY's and SC's.
6. **For SC's 07n expectations,** TS's verdicts on every variant are in §5; they agree with RS's except item 2.
7. **RV113's SR-RS findings, checked in TS** (ROOT's ruling on RV113, read mid-run):
   - **S-1's gap existed in TS too, and is closed.** Mutant run 1 left G5-4 (the `not_required` conjunct `product_attempt_ref == null` dropped) alive; `7e5c40f3d5` pins it with a `not_required` case owning its own attempt, which TS refuses at G5 `ATTEMPT` (the 07n form ROOT named). Killed in run 2.
   - **N-3 (P4 published, not triggered) is pinned in TS** by `7e47e51b5d` (G8-11 killed).
   - **N-1's analogue (a (4a) positive witness).** As in RS, no 07m entry is a (4a) positive witness at `validateRetainedPrecision`: no base or must-pass entry has a native stage that failed with a Run. TS already has one at the reader-logic level: I1's D30 test expects `productAttempts` to pass a native error naming its own non-selected Run. A supplementary mutant, D38-15, drops (4a) from the relaxed check, so that a failed native stage needs (4b) even with a Run. It is **killed by D30 alone** (§7). So TS needs no N-1 repair; 07n's W-C2 entries will add the witness at `validate`.
   - **N-2:** TS keeps DESIGN's `not_required` rule unchanged, as ruled.
8. **TEXT loop rules:** TS is not on the D1 call graph, so SQ's TEXT does not read it. For completeness: G8's per-case loop now filters each case's rows twice (mode rows, then parity rows) for every case; `d38CaptureBeforeRun` scans 8 constant stage names, only for an attempt whose native stage failed with no Run.

## 10. Records in this folder

- `RETURN.md` (this file) and `SHA256SUMS` (every file below and RETURN.md).
- `_run_records/census/`: `zzI92Census.test.ts`, `census.py`, `census_base.jsonl`, `census_aligned.jsonl`, `census.out`, `results_edits.txt`.
- `_run_records/suites/`: `suites_summary.txt`, `suites_compare.json`, `base_rc.txt`, `head_rc.txt`, `wasm_assets_worktree.sha256`, `suites.sh`, `compare_suites.py`.
- `_run_records/mutants/`: run 2 (final head) `mutants.jsonl`, `mutants.out`, `mutants_rc.txt`; `run1/` the same three for run 1; `supplement/` the same three for D38-15, with `mutants_supp.py`; `mutants.py`.
- `_run_records/tests/head_tests_at_I1.txt`.
- `_run_records/diff/`: `sr_ts.diff`, `commits.txt`.
- `_run_records/collect_records.py`: the copy, the placeholder substitution and the screen. In the recorded copy, two lines are redacted: the main checkout's absolute path and the screen's literal pattern, which would otherwise quote the forms it screens for.

Every path is a placeholder (`WT`, `S`, `NMS`, `VENV`, `P`, `R`). The screen looks for a home path, the system temp path, a home-relative form, the worktrees directory and the worktree's name, and finds nothing in this folder, RETURN.md included.

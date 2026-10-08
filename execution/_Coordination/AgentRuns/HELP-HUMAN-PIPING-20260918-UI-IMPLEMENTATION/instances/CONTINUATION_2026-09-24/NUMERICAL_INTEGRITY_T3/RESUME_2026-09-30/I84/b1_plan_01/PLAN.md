# I84 B1-P: B1's implementation plan (documents only)

TASK (Type 2), I84, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. I am a fresh instance. 2026-10-07 UTC (2026-10-06 host local time).

**Brief (verified before work):** `R/BRIEFS/B1_PLAN.md`, sha256 `d9b8bf0dfec96aed4ac728d55b06e7ac97e73f2f94fc7ffee2d37c9f905c6cf0`. I read NUM's root `AGENTS.md` and `agents/AGENT_TASK.md` first.

**Three basis changes during the work.** All are applied here, from ROOT's messages and RR's last three sections:
1. **"Owner decision: memory up to 64 GiB is ROOT's; B1's target reopened".** P1 was no longer settled. It is superseded by items 2 and 3.
2. **"Owner decision: M's practical limit is 12 GiB; target machines".**
   - M ≤ 12 GiB (12,884,901,888 B) is ROOT's; above it is the owner's.
   - The target is 32 GB workstations; the floor is 16 GB workstations still solving in practical time.
   - **New obligation:** measure the actual peak resident memory and the run time of cap-maximal W1 invocations in both modes (§3.6).
3. **"I82's addendum: B1's target is S3, one tier at D1's caps with C = 3"** (`R/I82/b1_cap_study_01/ADDENDUM_01.md`, sha256 `7c155ceb788eeb07e0984cce1a76c89f5952a8e93ab7bb174eb4efd188a00ce2`, read in full).
   - **One tier** at D1's full model caps: n = m = g = 32, Σr = 192, l = 128.
   - **C = 3,** with L = 384 stated and not binding.
   - **M** is selected at G6 as the smallest 256 MiB step that holds at least a 5 % text-error budget in both modes. The pricing expects 10.5 GiB; the ceiling is 12 GiB.
   - **The plan is for one tier:** one cap table, one profile, one set of G-B and G-C bounds. The two-tier variant is dropped, apart from the contingency in §9.

**To avoid a clash with I82's option label "S3",** my slices are named SW, ST, SP, SA, SR, SC, SQ, SG, SB and SK. "Option S3" always means I82's selected shape.

**Placeholders:** WT, NUM, P, PP (= `P/core/product_physics/src`), RE (= `P/core/reporting/result_export`), T, R, RR and VENV as in the dispatch. Also:
- **DESIGN** = `R/I78/b0_contract_01/DESIGN_v2.md`;
- **PROBE** = `R/I81/b1_probe_01/PROBE.md`;
- **STUDY** and **ADD** = `R/I82/b1_cap_study_01/STUDY.md` and `ADDENDUM_01.md`;
- **PLAN** = `R/I61/u8_plan_01/PLAN.md`;
- **CR** and **QUAL** = `T/IMPLEMENTATION/F2A_D1/CHANGE_RECORD.md` and `copies/QUALIFICATION.md`;
- **RS**, **PY**, **TS** = the Rust, Python and TypeScript retained readers: `RE/src/retained_precision.rs`, `P/core/analysis_runs/retained_precision.py`, `P/apps/desktop/src/features/results/retainedPrecision.ts`;
- **CORPUS** = `P/fixtures/results/retained_precision_cases.json`;
- **C** = the case cap (3), **l** = loads per case (128), **L** = total loads (384), **A** = the attempted cases (T-4).

**Basis.**
- **Code:** main `47a3bdfcf5`.
- **NUM** moved `f59542f71b` → `80e88bece4` → `c3556c3519` → `15b94978d1` while I worked, by records-only commits. Outside `P/execution`, NUM equals main plus SI1b's 3 rules-crate files, which B1 does not touch.
- **B6 (I83)** is on `codex/piping-t3-b6-20261007`:
  - committed head `5e38293530`: corpus 07m (17 bases, 294 mutations, 28 must-pass), TS's G7 header change, and the three harnesses;
  - uncommitted work on Python's transport validator (F-U6b-2).
- Code is cited by symbol.

**Limits kept.**
- Documents and code reading only, plus read-only Python (VENV) on committed bytes (`_run_records/`).
- No cargo, vitest, native or solver job; no installs; no Git writes (reads used `GIT_OPTIONAL_LOCKS=0`).
- Scratch in `WT/scratch/i84_b1_plan/`; nothing in the system temp directory.
- The host accepted my writes into this NUM records folder.

## 0. Findings in brief

1. **The producer is single-case throughout, so SP is the largest and least certain slice.**
   - `ProductCapture` holds one `observations`, one `source`, one `case_id` and one `native`.
   - `prepared_case_seen`, `prepared_case_source` and `prepared_observation_custody` refuse `load_cases.len() != 1`.
   - `solve_native` builds `OriginCapacity::for_calls(&[1], &[])`.
   - The serializer's `one_case` and `one_run` refuse more than one case, run, call or source.
   - The kernel side is already n-case (`RecordedInvocation::solve_cases`, `for_calls(&[n], …)`), so **FK should not change.**

   My bottom-up estimate for B1 is **67–102 h agent and 26–41 h review**, above DESIGN §9's 37–59 h and 12–17 h. §10 explains the difference.
2. **The c = 1 behaviour change can land first, as its own small slice (ST).** It covers T-4, decision 21 and `NoTriggeredCase`, written as the n-case classifier and used at c = 1.
   - It changes exactly PROBE §2.4's three witnesses, and pins T-4 on real inputs.
   - Its strongest oracle: **every committed c = 1 successor pin stays byte-identical.**
3. **Under option S3, W2b's replacement and the cap-maximal three-case witness share one model.** Both are at D1's full caps: 32 nodes, 32 members, 32 supports, and 128 loads per case.
   - So one probe (SW) can establish the replacement and the three cases' components on the same model.
   - It can start now, under ROOT's ruled stop rule.
4. **Six committed tests use a two-case milestone as their out-of-domain oracle.** The milestone (2 nodes, 1 member, 4 supports, 3 loads) is now inside the domain.
   - They are in PP's facade tests, the law tests, `retained_memory.rs`'s `tests`, and `P/core/runner/headless/tests/retained_precision_admission.rs`.
   - Each must move to C + 1 = 4 cases, derived from the cap constant, or keep the combination variant. Otherwise the tests silently change meaning.
5. **The reader alignment's re-pin cascade looks empty on 07m, by static reading** (`_run_records/cascade_static_07m.out.txt`).
   - Every base already has exactly one mode row per case, parity rows only in dense bases, and `w2` never `published`.
   - No G8 entry has a per-reader declaration.
   - Every entry that edits a mode or parity row, a requested mode or a material-basis reference fails at G1–G6, before G8.
   - The two entries nearest R-D38's (4b) still fail its conjuncts.

   The mechanical census (each aligned reader over 07m) is SR's precondition, with a stop rule.
6. **W-C2's successor cannot be pinned before RS is aligned.** RS refuses case B's W2-published `not_required` (its extra G5 conjuncts) and dense case A (F-1). So SP develops W-C2 through the private driver, and pins it after SR-RS lands.
7. **One tier keeps admission simple.**
   - `LOAD_CASES` becomes 3, and `cap_rows` gains the per-case and total load rows.
   - G-B and G-C get one set of bounds at C = 3.
   - The generated profile keeps its present shape, with one form set evaluated at c = a = 3.
   - `cap_priced_maximum` and `admission_bound` are unchanged in form.
8. **A consequence for B2, for ROOT to record.**
   - At 12 GiB, option S3's dense margin is 1,848,686,021 B (ADD §1).
   - At D1's caps, one more case-sized owner costs about 3.0–3.3 GB (STUDY §3.3), and a retained combination is priced like one.
   - So **B2 cannot add a D1-cap combination to a three-case invocation within 12 GiB.** It will need its own reduced tier, a lower C for combination-bearing invocations, or the owner. This does not change B1.
9. **Recommended packaging: B6 goes first, as its own compact PR.**
   - It touches no D1 crate `src`, so it needs no Pass B, T9, both-entry gate or re-qualification.
   - B1's reader and corpus slices then start from a main that already carries B6 (§6).
10. **Nothing here changes the breadth order or the owner's F2a order** (§11). The estimates grow, and the B6 packaging and finding 8 are flagged for ROOT.

## 1. The fence: what B1 writes, and what it must not

| Area | Files | Slice and owner |
|---|---|---|
| PP transaction | `PP/lib.rs` (the retained section, and observer call sites inside the ordinary run, only under `if let Some(observer)`); `PP/retained_product.rs`; `PP/retained_receipt.rs`; `PP/retained_wire.rs` | ST, SP: I-P |
| PP tests | `PP/retained_facade_tests.rs`; `PP/retained_product_tests.rs`; `PP/retained_wire_tests.rs` | ST, SP: I-P |
| PP admission | `PP/retained_memory.rs` (with its `tests` module, but not the generated block); `PP/retained_memory_law_tests.rs`; `P/core/runner/headless/tests/retained_precision_admission.rs` | SA: I-A |
| PP qualification | `PP/retained_memory.rs`'s GENERATED PROFILE block and `REGISTERED_PROFILES` (as `registration.diff`, applied by ROOT); `PP/retained_memory_witness_tests.rs` (after ST's edits); `P/core/product_physics/tests/retained_memory_challenge.rs` | SQ: I-A |
| Readers | RS and `RE/tests/retained_precision_contract.rs`; `RE/src/source_blocks.rs` (RV95 N-5's `#[cfg(test)]` test only) | SR-RS: I-RS |
| | PY and `P/tests/test_retained_precision_contract.py` | SR-PY: I-PY |
| | TS and `retainedPrecision.test.ts` | SR-TS: I-TS |
| Corpus and fixtures | CORPUS (07n, append-only after B6's 07m) | SC: I-PY |
| | New `P/fixtures/results/retained_precision_w_c2_successor_{sparse_interactive,dense_scrutiny}.json` | SP: I-P |

**Never touched without a stop and a ROOT ruling:**
- FK (`P/core/solver/frame_kernel`);
- the schema `P/schemas/retained_precision_mp_v2.schema.json`;
- the 13 reviewed statics and `Cargo.lock`, so the D-6 reviewed-input text is unchanged;
- the base (non-retained) readers;
- the ordinary route's behaviour (the observer-free paths);
- the T6S files;
- B6's items.

**So the registration diff changes only three things:** `threshold_bytes`, the generated profile and the cap constants. The identity, the reviewed inputs and `READER_LAYOUTS` stay the same. A change to RS that alters the layout of `Validation`, `ValidationError`, `RowClassification` or `AccuracyClass` is a stop.

## 2. The slices

IDs are roles; ROOT assigns fresh IDs (the next unused are I85 and RV107). Estimates are agent hours.

| Slice | Owner | Content (short) | Depends on | Estimate |
|---|---|---|---|---|
| **SW** | I-W, fresh | W2b's replacement and the three cap-maximal case components (a probe; records only) (§2.0) | R1 | 3–5 h |
| **ST** | I-P | T-4 at c = 1, decision 21, `NoTriggeredCase`, and the re-basings (§2.1) | R1 | 4–6 h |
| **SP** | I-P | The n-case transaction (T-2, T-5 to T-13), W-C2, multi-case faults (§2.2) | ST; the pins after SR-RS | 14–22 h |
| **SA** | I-A | Admission at option S3: D1.4, the census, cap rows, G-B, G-C, T-3 (e), budgets (§2.3) | R1 | 4–6 h |
| **SR-RS** | I-RS | RS: R-D38, F-1 text B, the G8 loop, the G5 `not_required` rule; RV95 N-5's direct test; RV97 R2-N-2 (§2.4) | B6 on main | 7–10 h |
| **SR-PY** | I-PY | PY: R-D38; G8's requested mode and P1–P4 (§2.4) | B6 on main | 4–6 h |
| **SR-TS** | I-TS | TS: R-D38; G8 codes; every case; no mode code 3 (§2.4) | B6 on main | 4–6 h |
| **SC** | I-PY (corpus); I-RS and I-TS (pins) | Corpus 07n and the three harnesses' pins (§2.5) | SP's W-C2 fixtures; SR | 6–8 h |
| **SQ** | I-A | Re-qualification, B1's Pass A: G5, M, G6, the S1 witnesses, the challenge, peak RSS and time, the non-candidate sweep (§3) | ST, SP, SA and SR-RS final | 14–21 h |
| **SG** | I-G, fresh | The Direct-entry gates: pressure, coexistence, the sweep, callers (§3.10) | SQ's registration in the candidate | 2–4 h |
| **SB** | I-A, or a fresh holder of I72's role | Pass B on PR-B1's head against SQ (§3.8) | The PR cut | 2–3 h, plus the build |
| **SK** | I-K, fresh (I77's and I80's role) | PR-B1's package (§6) | SQ, SG, SB | 3–5 h |

### 2.0 SW: W2b's replacement and the cap-maximal case components (a probe; records only)

**Write set:**
- a disposable `git archive` of main `47a3bdfcf5` in `WT/scratch/<id>_b1_w/`;
- the target `WT/targets/<id>-b1-w/`;
- the records `R/<id>/b1_w_probe_01/`: PROBE.md, `_run_records/` and SHA256SUMS.

No maintained file changes. Probe-only `cfg(test)` code goes in the archive, by I81's method (its `_run_records/zz_i81_probe.rs` and `instrumentation.diff`).

**Item 1: W2b's replacement** (RR "I81's B1-0 probe verified…", ruling 1; PROBE §5).
- **The model:** D1's cap-maximal counts. That is 32 nodes, a 32-member ring, 32 supports, 128 loads, 4 + 4 materials with 16 temperature points, and 128-byte identifiers. Under option S3 these are also B1's caps.
- **Construction (a):** scale the ring's sections down until the reciprocal condition estimate falls below √eps while the solve still succeeds.
- **Construction (b):** arrange loads and springs so that K-D5's formation check demotes the report, as it does for the milestone.
- **For each variant,** in both modes, through Direct and through the witness driver at 4 MiB, record:
  - the published verdict;
  - the seed (`initial` and `w2`);
  - `MECHANICS_SOLVED`;
  - the W1 stage reached;
  - the kernel terminal.
- **A variant qualifies** when its verdict is in A (Sensitive), it is solved, and W1 reaches the native stage. Candidate, Native or a successor are all acceptable outcomes. The role to fill is "the full native run at the cap-maximal counts".

**Item 2: the three cap-maximal case components.**
- Three distinct 128-load sets on item 1's model.
- Each is run alone as a one-case request, and each must be Sensitive and reach native. This is I68's and I81's method: a case's native outcome depends only on the model and that case's loads.
- Once SP exists, these become SQ's cap-maximal three-case witness and §3.6's c = 3 input.

**Item 3: an early reading.** Run `/usr/bin/time -l` on each run, recording the maximum resident set size, the peak memory footprint and the wall time. This is informational, ahead of §3.6.

**Controls:**
- the committed W2 and W2b reproduce QUAL §4's outcomes in the same build;
- two runs give identical probe lines.

**Stop rule (ruled).** If neither construction qualifies within its variant ladder, return. I propose at most 6 variants for each construction, all recorded. ROOT then rules option (c): W2b's input stays a `NoTriggeredCase` pin, and the cap-count stack evidence rests on QUAL §4's argument plus case C's full native run.

### 2.1 ST: T-4 at c = 1, decision 21, `NoTriggeredCase`, and the re-basings

**The branch.** ROOT cuts `codex/piping-t3-b1-<date>` from main, with worktree `WT/b1`. ST, SP, SA and SQ work there. SR and SC start after the branch absorbs main with B6 (§4).

**Code (`PP/lib.rs`):**
- Add `W1Fallback::NoTriggeredCase`, the reserved name.
- Add one per-case classifier, written for n cases:
  - `not_required` exactly when `numerical_quality.cases[i].solve_quality == checks_passed`;
  - **excluded** when the seed's `initial` is `StructuralFailure` with tag mechanism, asymmetric or invalid input, and `w2` is not `Published`;
  - **otherwise in A.** That includes an absent entry, `not_assessed`, and **a case with no seed** (ruling 2: W2 stays in A).
- `retained_w1` applies the classifier after coexistence and G-B, and before the notice reservation. If A is empty, the result is `NoTriggeredCase`: exact bytes, no notice.

**Tests (`PP/retained_facade_tests.rs`):**
- **W-C1's variant is re-based on case C alone** (PROBE §4's input, `3649b4dc…`) through Direct, in both modes. It asserts:
  - `Native`;
  - one notice;
  - the bytes equal `with_notice(plain, …)`;
  - `ONE_RUN_THROUGH_G_C`;
  - admitted, with no hooks.

  The kernel reason is recorded, not asserted.
- **`NoTriggeredCase` pins for two-body B** through Direct: exact bytes, no notice, `ONE_RUN_THROUGH_G_C`, no W1 work.
- **I77's two citation renames,** at this touch:
  - `zz_rv93.rs:292–315` becomes `zz_rv93_input_fallbacks`;
  - `retained_memory_witness_tests.rs :181–199` becomes `w6_input()`.

**Tests (`PP/retained_memory_witness_tests.rs`):**
- **W6's stack witness** is re-based on case C, on its private driver at 4 MiB, asserting `Fallback("Native")`.
- **W6's PHYS-R4 input and W2b's committed input** are kept as `NoTriggeredCase` pins (ruling 4).
  - I propose renaming the W2b test to say what it now pins.
  - Its doc line should say that its cap-maximal native role moves to SQ's replacement.
- **W2** is unchanged.

**Acceptance:**
- Every committed c = 1 successor pin is byte-identical: the milestone in both modes, L = 0 in both modes, and U3's pins.
- The registered PP suite differs from base only in the listed tests: PROBE §2.4's table, plus the new pins. `registered_g_c_declines_only_unattempted_solves` is unchanged.
- Unit tests of the classifier with hand-built seeds cover:
  - each verdict;
  - a W2-published Passed case;
  - a report Passed case;
  - each excluded tag, with and without W2;
  - no seed.
- Mutants are killed by assertions, never by compilation:
  - the key moved from the verdict to `initial`;
  - decision 21 dropped;
  - a seedless case treated as excluded;
  - `NoTriggeredCase` appending a notice.
- In a Stale build, every output is the plain bytes.

**A note on decision 21.** Its branch is unreachable from every audited committed input (PROBE §3). So it is pinned only by unit tests, and the owner-information line stays as ROOT wrote it.

### 2.2 SP: the n-case transaction (DESIGN T-1 to T-13)

**Code:**
- **T-2.** `ProductCapture` holds, for each requested case:
  - the seed (already per case);
  - the solver observations;
  - the late old-source capture, with its facts and operational records.

  It stays one owner. Today's one-case scope and custody counters (`case_calls`, `observation_calls`, `prepared_one_case_seen`, `prepared_late_calls`) become per-case, against c = `load_cases.len()` with no combinations.
- **T-3.** The order (a) to (e) in `permitted_run` is kept. The fact behind (e) belongs to SA.
- **T-5.** One `ReservedNotice` per case in A, all reserved before any W1 work. A collision with an existing id or with another reserved id gives `NoticeReservation`.
- **T-6.** Invocation custody is checked once; `bind_observations` runs per case.
- **T-7.** Preparation runs over A in request order.
  - Each case gets one C3 `ProductAttempt`, with ids in actual start order.
  - A preparation failure makes only that case unavailable, with no `CaseSource`.
  - `bind_preparation` names the attempt's own index.
- **T-8.** One `CaseBatchCall` over A's prepared sources:
  - one `RecordedInvocation` with its 60 G limit, and the case limit for each case;
  - `for_calls(&[|prepared|], &[])`.

  A Run that is not selected makes its case unavailable (`kernel_*`). A call failure before any Run leads to T-12.
- **T-9.** One freeze per selected Run. A failure gives `facade_certificate`.
- **T-10 and T-11:**
  - **Staging:** one copy. The selected cases' overlays come first, in request order, then the unavailable cases' diagnostics. `not_required` and excluded cases get nothing.
  - **The receipt body arrays** follow DESIGN T-11. Each snapshot is recorded at its attempt's terminal stage.
  - **Every serializer failure abandons the successor.**
  - **The R-b′ limit (N-5) is kept,** and pinned by a hook test: one `recovery_demoted` seed beside a selected case abandons the successor.
  - **Precommit** runs with the actual invocation. The transfer only moves values.
- **T-12.** The ordinary bytes are published, then one N1 notice per case in A, in request order. The receipt-encoding detail is added only for C1:68's details or `publication_hash_range`, and only on cases that were selected when the successor was abandoned.
- **T-13.** One ordinary run; G-C is reached once.
- **Guidance (decision 19): adopt I82's three choices (STUDY §8.3).** They lower the c² terms that SQ will price:
  - bind observations by case index;
  - collect `diagnostic_refs` in one pass;
  - make the integrity diagnostic loop per case.

**Tests:**
- **W-C2:** three cases on U8's two-body model, in both modes.
  - First through the private driver: A selected; B `not_required`; C unavailable with `kernel_terminal {unresolved, {space: unresolved, tag: ceiling}}`.
  - After SR-RS lands: the successor is pinned by sha256 (the document, the receipt and the published bytes), with the D-U6-5 fixtures and the Rust reader passing with the invocation.
  - Through Direct once SQ's registration is in the candidate (§3.3).
- **Multi-case fault tests,** using hooks:
  - a T-6 custody failure;
  - T-7: one case's preparation fails beside a selected case, giving a successor with that case unavailable and no `CaseSource`;
  - a T-8 call failure: D38's test-hook shape (source withdrawn) stays refused;
  - staging abandonment, each serializer detail, and precommit abandonment.

  Each also checks T-12: the notice count equals |A|, and the detail is placed correctly.
- **T-12's base readers:** `u3_r2_base_readers_accept_the_unavailable_notice` is extended to several N1 notices. Rust's `semantic_contract` runs inside the test. The bytes are written out for PY's and TS's lanes, which SC's owners run.
- **The out-of-domain oracles are re-based** (finding 4). They use four cases, taken from the cap constant, and keep the combination variant:
  - `u3_each_stage_fault_falls_back_to_the_ordinary_bytes` (its two-case `Domain` check);
  - `u3_no_permit_entries_are_the_ordinary_route`;
  - `u3g2_direct_entry_no_w1_refusals_keep_exact_bytes`.
- **`retained_product_tests.rs`'s prepared-scope cases** (`multiple cases`) are restated for n-case custody.
- **The source-text pins** (`u3_n9_single_parse_custody`, `u3_permitted_outputs_keep_the_report_and_gate_order`, `u3_capture_permit_is_linear`) change only in their expected sections, each with its reason.

**Acceptance:**
- c = 1 byte identity holds (ST's pins).
- W-C2's per-case outcomes equal PROBE's one-case outcomes: A selected (dense after F-1 text B), B `not_required`, and C at Ceiling.
- `ONE_RUN_THROUGH_G_C` holds, hooks are empty, and Stale gives the plain bytes.
- Mutants are killed by assertions, for:
  - the staging order;
  - `attempt_ref`;
  - the snapshot record point;
  - the detail placement;
  - the reservation count;
  - each member of decision 5's abandonment set.

**Checkpoint.** After T-2, T-6 and T-7 (capture and preparation per case), I-P re-estimates SP, and ROOT may let RV-P start reading.

### 2.3 SA: admission at option S3

**Caps (`caps`):**
- `LOAD_CASES` = 3;
- a new `TOTAL_LOADS` = 384 = C·l, stated and not binding (ADD §2);
- every other D1.9 and D1.11 cap unchanged: n, m, g = 32, Σr = 192, l = 128.

**Census.** Record the per-case load facts (the maximum length and capacity over cases) and the total. `NestedTypedFacts` grows; today it records `load_cases[0]` only.

**Clauses:**
- **D1.4:** 1 ≤ c ≤ 3, no combinations, no components. c = 0 and c ≥ 4 refuse.
- **D1.5 and D1.7 apply to every case.** Today `family_clauses` reads only `load_cases[0]`.
- **D1.10** already reads every case.

**`cap_rows`:**
- `LoadCasesCapacity` ≤ 3;
- `Loads` and `LoadsCapacity` become per-case maxima ≤ 128;
- a new `TotalLoads` row ≤ 384;
- `CAP_ROWS` grows to match.

**G-B (`late_observations`):**
- `CaseLoads` ≤ 128 per case, plus a running total ≤ 384;
- `LateObservationBytes` ≤ the regenerated T11 − T11_late_capture.

**G-C (`complete_observations`),** one set of bounds at C = 3:
- EnvelopeResults ≤ 3·P_final, with its capacity and text;
- D_env, Text(diag_env), L_PUB and L_DIAGID from the regenerated TEXT;
- the contract-evidence facts × 3;
- ObservationBytes and OrdinarySeedBytes from the regenerated forms;
- **RetainedErrorTextBytes ≤ 3·(3m + 1)·Text(err).** This is I82's stated assumption, and SA checks it against SP's producer.

**T-3 (e).** `ordinary_solve_attempted` holds only with one seed per requested case and every `initial` set.

**Budgets.** `phase_caps()` and `PHASE_BUDGETS` take the regenerated forms. B-6 becomes NOTICE_RESERVE_BYTES × 3; B-1 stays one run.

**Pricing.** `cap_priced_maximum` and `admission_bound` keep their form. Until SQ regenerates the block, the profile is the c = 1 one, so admission at c ≥ 2 rests on a stale price. Decision 17 keeps that state on the branch.

**Law tests:**
- each new row at its cap and at cap + 1;
- c = 0, 1, 2, 3 and 4;
- G-B per case and in total;
- T-3 (e) with an input whose case k < c − 1 blocks: G-C declines, with exact bytes and no notice. Seeds are per case already;
- the re-based out-of-domain tests in `retained_memory.rs`'s `tests`, the law tests and the runner's headless test (finding 4).

**Since option S3 is selected, SA no longer waits for a cap ruling.** It can start at R1.

### 2.4 SR: the three readers

**The common specification:**
- **DESIGN §2:** R-D38 with (4b), and its m1–m8.
- **DESIGN §3.2, text B:** P1–P4. P5 is deferred (DESIGN decision 10).
- **DESIGN §3.3, G8:** a per-case loop over every case, after the invocation, project, model-scope and material checks. In order: the requested mode, the material-basis reference, P1, then P2–P4. All use `RETAINED_PRECISION_PREPARATION_MISMATCH`.
- **DESIGN §3.3, G5's `not_required` rule:** `product_attempt_ref` null, `initial.kind != not_attempted`, and verdict `checks_passed`.

**Precondition for each owner: the cascade census.** Run the aligned reader over 07m's 294 mutations and 28 must-pass entries, and list every outcome that changes. The expected number is zero (finding 5). Any change stops the work and goes to ROOT (R5).

**SR-RS:**
- **(4b), with B1's obligation.** List every RS check that assumes a prepared source has a Call or a Run, and either relax it to (4b) or show it does not apply. The list goes in the RETURN.
- **G8** is widened from selected cases to every case, with P2–P4 replacing "exactly one parity row iff dense".
- **G5:** drop the three conjuncts `initial.kind == report`, `outcome == checks_passed` and `w2 == not_triggered`.
- **Unit tests** on synthetic n-case receipts go in `RE/tests/retained_precision_contract.rs`. Its module doc is reworded at this touch (RV97 R2-N-2).
- **RV95 N-5's direct `#[cfg(test)]` test** goes in `RE/src/source_blocks.rs` (I74 decision 9; I76 item b; RV101 NT-1 and A2-N3). It:
  - calls `integer` at 2^53 − 1, 2^53 and `u64::MAX`;
  - does not rely on the four fields whose schema maximum is tighter;
  - covers `failure.block_order` and the composite physics-source receipt.
- **RS is on the D1 call graph,** so it is frozen before SQ's final G5.

**SR-PY:**
- (4b) in `_g5_stages`;
- G8 adds the requested mode and P1–P4 per case, at that position;
- built on top of B6's F-U6b-2.

**SR-TS:**
- (4b) in `productAttempts`;
- G8's mode-row and requested-mode codes change from `INVOCATION_MISMATCH` to `PREPARATION_MISMATCH`;
- mode code 3 is dropped, and P2–P4 apply to every case;
- built on top of B6's G7 header change.

### 2.5 SC: corpus 07n (one writer), then the harness pins

**Bases,** appended after 07m:
- **`w_c2_sparse_interactive` and `w_c2_dense_scrutiny`:** D-U6-5 copies of SP's pinned successors. Each case's provenance names the producer, the entry and the build.
- **`d38_beside_selected`:** synthetic, labelled "not producer-emittable under T-8". A records script derives it from a W-C2 base and reseals every hash. The script:
  - removes case C's Run, its `execution_order` entry, and the Builds that C's Run originated;
  - removes C's source from the call's and the group's `source_refs`;
  - keeps the shared group and A's builds;
  - recomputes `charged` and the call's after-value;
  - sets the cause to a typed `CaptureError::Origin`.

**Must-pass:**
- the two W-C2 bases;
- `d38_beside_selected` with its invocation: G0–G8 pass, and the standing is `needs_recompute`;
- L = 0's entries, unchanged.

**Mutations:**
- **D38:** m1–m8.
- **F-1,** five entries:
  - dense L = 0's parity row copied onto W-C2 dense case A (P4);
  - dense L = 0's parity row duplicated (P2);
  - a parity row added to sparse L = 0 (P3);
  - mode code 3 in sparse L = 0 (P1);
  - `requested_mode` flipped in one case.
- **`not_required`,** three entries on W-C2 case B:
  - its verdict set to `sensitive`;
  - its `initial` set to `not_attempted`;
  - its `product_attempt_ref` set non-null.

**Expectations.** Each first failure is fixed from the three readers' agreement. A disagreement can be declared per reader only by ROOT's ruling.

**Acceptance:**
- all three readers pass 07n;
- counts move only by added entries;
- 07n's sha256 is recorded;
- the PY and TS lanes also check SP's several-notice bytes.

Then I-RS and I-TS update their harness pins and counts, each running the full 07n.

## 3. Re-qualification: what each obligation must show, and who produces and reviews it

| # | Obligation | Produced by | Reviewed by |
|---|---|---|---|
| 3.1 | G5: TEXT and the profile at option S3 | SQ (I-A) | RV-Q |
| 3.2 | M's selection | ROOT (R6a, R6b) | RV-Q's G6 review |
| 3.3 | G6: `admission_bound`, the pure maximum, the registration diff | SQ; ROOT applies the diff | RV-Q |
| 3.4 | S1 stack witnesses at R/16, both modes | SQ | RV-Q |
| 3.5 | Challenge peaks | SQ | RV-Q |
| 3.6 | **Peak RSS and run time (the new obligation)** | SQ | RV-Q |
| 3.7 | QUAL §11's carry | SQ | RV-Q |
| 3.8 | Pass B | SB | RV-Q confirms |
| 3.9 | The full 40-manifest suite before the freeze | ROOT (DEC-025 on the integrated candidate) | — |
| 3.10 | The Direct-entry gates | SG | ROOT verifies; RV-P reads the sweep delta |
| 3.11 | T9 and the both-entry gate | ROOT | — |
| 3.12 | T6S consistency | The full suite; SK records it | — |

**3.1 G5.**
- **Method.** Re-run TEXT, using `g7_pass.sh`'s chain, on the candidate made of ST, SP, SA and SR-RS.
  - Take I82's census and TEXT variables from `I82_PATCHES.json`: `c`, `a`, `L`, `cases`, `att` and `Pall`.
  - **Do not take its 22 emulated rebinds or its 22 emulated edges.** The real loops replace them.
  - Re-key the line rules with `g7_linemap.py`.
  - Price at c = a = 3 and L = 384.
- **It must show:**
  - TEXT complete: no unmapped loop or argument, no multi-member SCC, and a converged D fixpoint;
  - the identifier audit enforced, with its controls, and B1's new identifier-bearing sites priced by source;
  - the regenerated GENERATED PROFILE block, with `ESTIMATES` = 0 and the forms gate equal;
  - the law test printing `I65_G5_PROFILE` and `I65_G5_PHASE` per mode;
  - **E_mov,max + R** within the text-error budget of ADD §1's 9,747,725,678 B (dense) and 9,688,594,334 B (sparse), or every difference explained, with the c² sites re-derived on the real code;
  - **a c = 1 control run of the same chain:** equal to today's record (3,575,778,286 / 3,595,488,734 B), or different only by named B1 loops.
- **A G5-early dry run** on SP's and SA's first complete code is informational. It catches a memory overrun before the reviews. The final G5 follows the reviews.

**3.2 M.** ROOT selects M by D-7 (RR "I82's addendum…"): **the smallest 256 MiB step at which E_mov,max + R ≤ 0.9 M holds with at least a 5 % text-error budget, in both modes.** The ceiling is 12 GiB, and the pricing expects 10.5 GiB.
- **R6a, after G5:** a provisional M, for G6's tests.
- **R6b, after RV-Q's G6 review:** the final M, recorded as a ruling together with the registration.
- **If no M ≤ 12 GiB holds the rule,** see §9's contingency.

**3.3 G6.** It must show:
- `admission_bound` at M − R − 1, M − R and M − R + 1;
- a pure `maximum` test in which each of the 7 phases is in turn the largest. W4 sits 215 MB below W3 at option S3's pricing (ADD §1);
- the 0.9 M rule in both modes;
- the registration diff (`threshold_bytes`, the generated block, `LOAD_CASES` and `TOTAL_LOADS`), applied in a scratch copy with PP's suite passing, apart from the known Mac `t13`;
- a **QUAL_B1.md** in QUAL's form, with the text-error budget;
- the release build's profile record equal to the dev/test record (QUAL §5's practice; informational until B7).

**3.4 S1 witnesses.** At R/k = 4 MiB, in both modes, one process each, in the dev/test build and, informationally, in the release build. They must show no overflow, abort or panic, with every outcome asserted:
- W1, W2, W2-deep, W3, W4, W7 and headroom: unchanged;
- W6: case C, Native;
- W2b's replacement (from SW), or ROOT's option (c);
- the `NoTriggeredCase` pins;
- **W-C2:** a successor, and again at R/64 = 1 MiB as headroom;
- **the cap-maximal three-case input:** SW's three components on one model, so |A| = 3. Its outcome is asserted. Preparation or Candidate fallbacks are acceptable, as they were for QUAL's W2 and W2b;
- **the deepest call chain,** re-derived on B1's call graph by R4_CALLGRAPH's method. I expect 40 frames, unchanged, because loops over A add no recursion.

**3.5 The challenge (`retained_memory_challenge`).** It must show:
- the measured peak of a permitted multi-case run (W-C2 through Direct, registered) ≤ the in-build W1 phase, or E_mov,max;
- the milestone's peak still within bound. Today it is 3,541,898 B sparse and 2,252,863 B dense.

**3.6 Peak resident memory and run time** (RR "Owner decision: M's practical limit is 12 GiB…"). SQ produces it on this host, by this method.

- **Build:**
  - the registered dev/test PP test binary: `cargo test --lib --no-run` through `WT/tools/t3_cargo.sh`, in a fresh target;
  - the release test binary (`--release`). The release build is Stale at admission, but the private driver runs W1, as QUAL §4's release witnesses did.
- **Run:**
  - each measurement is one process: `/usr/bin/time -l <test-binary> <test> --exact --ignored --test-threads=1 --nocapture`;
  - the binary runs directly, not through cargo;
  - under `lockf -k WT/guard/cargo_job.lock`, with the memory guard up and no other heavy job running.
- **Record:**
  - from `time -l`: the maximum resident set size and the peak memory footprint, in bytes; the real, user and sys seconds;
  - from the test itself: a printed line with `Instant` timings for the ordinary run and for the W1 phases, and the outcome.
- **Inputs, in both modes:**
  - **c = 1 cap-maximal:** W2b's replacement (native reached) and W2 (Preparation);
  - **c = 3 cap-maximal:** SW's three cases at 32/32/32 with 128 loads each;
  - **controls:** the milestone's W1; the same cap-maximal inputs on the ordinary value route only, to isolate W1's increment; and the process floor (the binary running a trivial test).
- **Builds:** dev/test through Direct, with the candidate registered; release through the private driver.
- **Repetitions:** 3 each, reporting the median and the maximum.
- **Output: `RSS_TIME.md`.** It sets the measured peak beside:
  - option S3's priced worst-case heap, E_mov,max = 9,680,616,814 B (9.02 GiB; ADD §3);
  - 16 GB and 32 GB.

  It states that debug-build times are pessimistic, and that the release times are the ones B7 and B8 will read.
- **Non-claims:**
  - no supported-machine statement (owner-held, drafted at B7 and B8);
  - macOS RSS includes shared pages, so the peak footprint is recorded beside it.
- **Expected cost.** W2b's full native run at today's cap counts took 43 s in debug for both modes together (QUAL §4's `witnesses.test.txt`). A three-case cap-maximal input runs three native ladders, so a few minutes per run. In total, about 1.5–2.5 h of machine time for both builds and 3 repetitions.

**3.7 QUAL §11's carry** (RV87 G6r N-1).
- **Recommended:** re-run RV87's by-type non-candidate sweep (`noncand_compare.py`) over B1's new non-candidates, reviewed by type by RV-Q.
- The explicit-row rule is deferred to B2's planning, where it would pay back over B2, B3 and B7 (decision 11).

**3.8 Pass B.** SQ is B1's Pass A: the new reference for TEXT, forms, law, witnesses and challenge. Pass B runs on PR-B1's head with I65's fail-closed `g7_pass.sh`, retargeted as I72 did. It must show:
- the tree;
- the entry equal to the applied registration;
- law, statics, the line map and the premise pins;
- TEXT and forms equal to SQ's;
- every delta row classified, with a reviewed `delta_reviewed.json` entry for each of B1's production hunks;
- the non-candidates equal to SQ's swept set;
- the controls;
- PP and runner outcomes changed only by the listed tests;
- the witnesses and the challenge equal to SQ's.

RV-Q confirms it.

**3.9 The full 40-manifest suite before the freeze.** ROOT runs DEC-025 on the integrated candidate in NUM. Expected deltas:
- added PP, `result_export`, pytest and vitest tests;
- the runner's re-based test, with the same count;
- T6S's suites unchanged.

**3.10 The Direct-entry gates (SG).** By I61's `u9_g8_01` method, on the candidate with the registration applied:
- **Pressure:** any case with pressure regions is refused at D1.5, with exact bytes.
- **Coexistence:** n05 and n06, plus a multi-case input where exact-block selects one case. Exact bytes and no notice (T-3 (c)).
- **The u3g2 fixture sweep, registered and Stale:**
  - Stale is byte-identical to base.
  - Every registered difference is listed row by row and explained. It is explained by T-4 (a Passed one-case input loses its notice), by D1.4's widening (an admitted multi-case fixture gets a successor, one notice per case in A, or `NoTriggeredCase`), or by T-12.
  - An unexplained row is a stop.
- **Callers:** a scan for callers of the retained entries.

**3.11 T9 and the both-entry gate, part 1** (ROOT; U9's G5 and G6 method).
- **Expected:** byte-identical, because the ordinary route is observer-free and Stale builds never reach W1.
- **Recommended anyway,** because `PP/lib.rs`'s ordinary-run bodies host the observer call sites.
- **Part 2** (dense, at scale) runs only if Pass B classifies a hunk as live on the observer-free route (decision 12).

**3.12 T6S consistency** (I74 PLAN §4.3). The c = 1 successors are unchanged (ST, SP), so T6S-2's goldens are not regenerated. The T6S suites run in the full suite. A changed disclosure meaning goes back to ROOT.

## 4. Order, parallelism and ROOT's ruling points

**The host rule:** one heavy job at a time (`WT/guard/cargo_job.lock`). Cargo, pytest under `P/tests` and heavy vitest all take the lock. Parallel lanes interleave their builds; they do not overlap them.

| Phase | Lane 1 (PP transaction) | Lane 2 (admission and qualification) | Lane 3 (readers and corpus) | Lane 4 (probe and gates) |
|---|---|---|---|---|
| 0 | — | — | B6 (I83) finishes; its PR (§6) | R1 |
| 1 | ST, then RV-P round 1 | SA, then RV-Q round 1 | B6's PR is gated and merged; B1's branch absorbs main | SW |
| 2 | SP (checkpoint after T-7) | — | SR-RS, SR-PY, SR-TS (each starts with its census) | — |
| 3 | SP's W-C2 pins (after SR-RS); RV-P round 2 | G5-early (informational) | RV-R on SR; SC after SP's fixtures | — |
| 4 | Repairs | **G5 final → R6a → G6, witnesses, challenge, peak RSS and time → RV-Q → R6b** | RV-R on SC | — |
| 5 | B1 merges into NUM, with no other unmerged slice there → **the full suite** (ROOT) | — | — | SG; T9 and both-entry (ROOT) |
| 6 | PR-B1 cut → SB (Pass B) and RV-Q's confirmation; SK; RV-P's PR-head ledger; CI, dispatch, GEN-8, exact-head DEC-025 → merge | | | |

**The critical path** is ST → SP → SQ (G5, M, G6 and the measurements) → integration and the full suite → the PR gates. That is about 38–55 h of serial agent work plus review latency, roughly 6–9 working sessions. SW, SA, SR and SC run off the critical path.

**ROOT's ruling points:**
- **R1:** this plan, its slices and IDs, and decisions §7.
- **R3:** ST's checkpoint (RV-P round 1), before SP builds on it.
- **R4:** SW's stop rule, if it fires (option (c)).
- **R5:** the cascade census, if any 07m entry changes.
- **R6a and R6b:** M's provisional selection after G5, and its final selection after G6's review.
- **R7:** T9 and both-entry applicability, the freeze, the carry-over if main moves, and the merge.
- **R8, any stop:**
  - an FK, schema, base-reader or reviewed-input change;
  - a c = 1 byte change;
  - an unexplained sweep row;
  - §9's contingency.

The cap ruling is no longer pending: option S3 is selected.

## 5. Reviews (fresh IDs; ROOT assigns them)

**RV-P** (RV93's role). Estimate: round 1 2–3 h; round 2 6–9 h; confirmations 1–2 h; ledger 2–3 h.
- **Scope:**
  - ST (round 1) and SP (round 2), with the same reviewer confirming repairs;
  - the PR-head ledger: every hunk maps to a reviewed commit;
  - SG's sweep delta.
- **Oracles:**
  - DESIGN T-1 to T-13 and its outcome table; C1, C2 and C3.
  - **Its own probe on the B1 head:** case C alone, two-body B, and W6's and W2b's inputs, in both modes, against PROBE §2–§4.
  - The c = 1 pins, byte for byte.
  - **W-C2's receipt, re-derived:** the per-case snapshots, `charged`, `execution_order`, and group and build sharing, from the kernel's record.
  - The three readers on W-C2, with the invocation.
  - Mutants on the T-4 key, decision 21, `NoTriggeredCase`, T-5's count, T-12's placement, and decision 5's set.

**RV-R** (RV78's role). Estimate: 6–9 h, plus 1–2 h to confirm.
- **Scope:** SR (the three readers) and SC (07n).
- **Oracles:**
  - DESIGN §2–§3;
  - **its own cascade census;**
  - its own reseal script for `d38_beside_selected`, byte-compared with SC's;
  - the full 07n in three languages, with first-failure parity;
  - the completeness of the D38 obligation audit, by reading every RS, PY and TS check on `run_ref`, `call_ref` and `source_refs`;
  - F-1 text B's disclosed limit.

**RV-Q** (RV89's and RV87's roles; it may be split in two). Estimate: 8–13 h.
- **Scope:** SA; SQ (G5, G6, the witnesses, the challenge, peak RSS and time, the non-candidate sweep); SB's confirmation.
- **Oracles:**
  - **I82's `b1_eval.py`,** as an independent evaluator of the phases from the profile tree and the atoms;
  - I82's emulated numbers (ADD §1), for each delta;
  - the forms regenerated byte for byte;
  - the boundary tests at M;
  - the witness logs for each identity;
  - the RSS method and its controls.

**The complete-diff review at PR-B1's head** (gate item 2). The slice reviewers cover disjoint parts. RV-P confirms the PR head with the ledger, as RV97 did for U8 and RV101 for T6S. A separate fresh complete-diff reviewer (RV95's role, +5–8 h) is ROOT's option (decision 16).

## 6. PR-B1's packaging

**B6 goes first, as its own compact product PR** (recommended; decision 15).

**Why:**
- **Lighter gates.** B6 touches no D1 crate `src` (B6's brief: PP and every D1 `src` unchanged). So it needs no Pass B, T9, both-entry gate or re-qualification, as with T6S ("Pass B not applicable").
- **It is nearly done,** so its reviewed work reaches main sooner.
- **Shared files.** B1's SR and SC rewrite the same files as B6: TS, PY, the three harnesses and CORPUS. With B6 on main, B1's branch takes B6 by one merge of main, and NUM carries one unmerged slice at a time (RR "U8's full suite passes; … NUM sequencing").
- **PLAN decision 8's purpose is kept.** Breadth still needs only two re-qualification runs, because B6 needs none.
- **Focus.** PR-B1 stays on the D1 call graph.

**The cost:** one more product-PR gate set: B6's reviewer, the full suite, CI with dispatch, GEN-8 and an exact-head DEC-025. That is about 3–5 h of ROOT's wall time and 2 h of machine time.

**The alternative,** B6 riding with PR-B1, saves one DEC-025 cycle. But it holds B6 back for the whole of B1, and makes PR-B1 larger.

**Sequencing:**
1. SI1b (#1106) merges.
2. NUM absorbs main.
3. B6 merges into NUM; its PR is cut, gated and merged.
4. NUM absorbs main.
5. B1's branch merges main (no rebase).
6. SR and SC start.

**PR-B1's gate set** (RR "T3's gate set and Git rules…", items 1–7, applied to B1):
1. A compact cut from main; `source_equality.py` against NUM; `check_citations.py`; the package (SK).
2. Fresh independent reviews (§5), with each repair confirmed by the same reviewer, and the PR-head ledger.
3. **The full 40-manifest suite before the freeze** (§3.9).
4. Hosted CI and the full-SHA dispatch.
5. GEN-8 on the exact head (E-4's method).
6. An exact-final-head Mac DEC-025 against a fresh main baseline: `run_dec025.sh`, counted only at `ALL-DONE`, compared with `compare_suites.py`.
7. **Because the D1 call graph and the registered profile are touched:** Pass B with RV-Q's confirmation (§3.8); T9 and both-entry part 1 (§3.11); the Direct-entry gates (SG).

**B1's evidence in the package:**
- QUAL_B1.md, with M's ruling and the text-error budget;
- the generator and the profile tree;
- `registration.diff`;
- the witness logs (dev/test and release);
- the challenge;
- RSS_TIME.md;
- 07n's parity across the three readers;
- the W-C2 pins;
- RV95 N-5's direct test;
- RV97 R2-N-2.

**Merge:** `gh pr merge --merge --match-head-commit`, after confirming that main has not moved.

## 7. Decisions

None of these is on the work graph's owner-held list. Owner-held items are listed after the table, untouched.

| # | Decision | Recommendation | Decider |
|---|---|---|---|
| 1 | The slice structure, owners and lanes (§2, §4) | **Adopt.** One PP integration owner (I-P) for ST and SP, which share files; one profile owner (I-A) for SA and SQ, for continuity with I65; three reader owners; separate probe and gate TASKs | ROOT |
| 2 | Admission at option S3 (§2.3) | **One cap table, with no tier machinery:** `LOAD_CASES` = 3, a stated `TOTAL_LOADS` = 384, per-case load rows, and G-B and G-C at C = 3. `cap_priced_maximum` and `admission_bound` keep their form | ROOT |
| 3 | Re-basing the two-case out-of-domain oracles (finding 4) | **Four cases, from the cap constant,** keeping the combination variant | ROOT |
| 4 | When W2b's replacement is probed | **Now (SW), at D1's caps,** which are option S3's caps. Item 2 establishes the three cap-maximal cases on the same model | ROOT |
| 5 | W2b's committed witness | **Rename it to what it pins** (`NoTriggeredCase` on an ordinary Passed report), and add the replacement as a new witness | ROOT |
| 6 | T-3 (e)'s fact | **In `retained_memory.rs` (SA):** one seed per requested case, with the requested count taken from the invocation's model | ROOT |
| 7 | W-C2's pin order | **Private driver first;** pinned after SR-RS; through Direct after the registration | ROOT |
| 8 | 07n | **One writer (I-PY).** W-C2 as D-U6-5 copies; D38's base derived by a records script, with labelled provenance | ROOT |
| 9 | The cascade census | **A precondition of SR,** with stop rule R5 | ROOT |
| 10 | Pass A and Pass B | **SQ is B1's Pass A** (the new reference); Pass B runs on the PR head against it | ROOT |
| 11 | QUAL §11's carry | **RV87's by-type sweep now;** the explicit-row rule at B2's planning | ROOT |
| 12 | T9 and the both-entry gate | **Run T9 and part 1.** Part 2 only if Pass B finds a hunk live on the observer-free route | ROOT |
| 13 | The registered sweep | **Required, with every changed row explained** (SG) | ROOT |
| 14 | The peak RSS and run-time method (§3.6), with release-build witnesses and profile record | **Adopt;** produced in SQ. The release build is informational, with no registration; the results feed B7 | ROOT |
| 15 | PR-B1's packaging | **B6 first, separately** (§6) | ROOT |
| 16 | The complete-diff review at the PR head | **RV-P with the ledger;** a separate fresh reviewer is optional | ROOT |
| 17 | Intermediate commits | **No B1 commit reaches NUM or main before SQ,** because the profile is stale until then | ROOT |
| 18 | G5-early | **A dry run after SP and SA,** informational | ROOT |
| 19 | I82's c² guidance (STUDY §8.3) | **Adopt it in SP** | ROOT |
| 20 | M (R6a, R6b) | **The smallest 256 MiB step ≤ 12 GiB** holding E+R ≤ 0.9 M with at least a 5 % text-error budget in both modes (RR, ADD §4); §9's contingency otherwise | ROOT; above 12 GiB, the owner |
| 21 | Finding 8 (B2's room under 12 GiB) | **Record it now** in B2's planning input; no B1 change | ROOT |

**Carried, not re-decided:**
- DESIGN's selected decisions 1–16, 20 and 21;
- RR "I81's B1-0 probe verified…", rulings 1–4;
- RR "I82's addendum…" (option S3, and M's rule);
- I74 decision 9 (N-5's direct test rides with PR-B1);
- I77's renames;
- RV97 R2-N-2;
- the T6S consistency note.

**Owner-held, untouched:**
- M above 12 GiB;
- the formal supported-machine statement (B1 only supplies §3.6's measurement);
- the dense and lane ceilings (F-1 changes neither guard);
- PHYS-R4's named refusal and availability (W6's input is only a test pin);
- observation framing (P5 is deferred);
- KF3 and KF2;
- public meaning;
- the native-app witnesses;
- B8's R-2.

## 8. Risks and stop rules

1. **Memory.**
   - **At option S3's priced 10.5 GiB** (ADD §1), the dense margin is 399,134,558 B (9.53 %), and the sparse margin is 458,265,902 B (10.94 %).
   - **At the 12 GiB ceiling,** the dense margin is 1,848,686,021 B (44.1 % of TAV_W). So B1's real-code G5 can deviate by up to about 44 % of TAV_W within ROOT's authority. These figures are emulated; only G5 measures.
   - **The ladder at R6a:** raise M in 256 MiB steps up to 12 GiB.
   - **Stop:** no M ≤ 12 GiB holds at least 5 % in both modes → §9's contingency. The G5-early dry run gives the warning early.
2. **W2b.**
   - **Stop:** SW's rule (§2.0) → R4, option (c).
   - **A second risk:** the three-case cap-maximal input may not reach native in every case. Its asserted outcome then stands (Preparation or Candidate are acceptable), and §3.6 records the deepest stage it reached.
3. **The reader alignment's re-pin cascade.** I expect none on 07m (finding 5).
   - **Stop:** any changed 07m outcome → R5, with the list.
   - Corpus entries on which the readers disagree are declared per reader only by ruling.
4. **Stack depth with several cases.** Loops over A, the batch native call and the readers' case loops add no recursion, and serde Values keep the grammar's depth. I expect 40 frames, about 1.47 MiB.
   - **Stop:** any overflow, abort or panic at R/16. A change to R or k is a profile change and goes to ROOT.
   - The panic-hook limit stays as QUAL §4 states it.
5. **c = 1 identity.** Any byte change in a committed c = 1 successor, ordinary envelope or Stale output is a stop.
6. **Producer scope.** If SP needs an FK, schema, reviewed-static or base-reader change, it stops. My reading finds none needed.
7. **The stale profile on the branch** (decision 17). Nothing leaves the branch before SQ.
8. **Source-text pins of `PP/lib.rs`.**
   - Inside PP, three facade tests read it.
   - Outside PP, `RE/tests/retained_precision_carriers.rs` names `nonlinear_assembled_loop_context`, and `frame_kernel/tests/k2a_checked_formation.rs` cites a line range. I-P runs both early.
   - A change of expected text needs a stated reason; a weakened assertion is a stop.
9. **Registered sweep surprises,** from real multi-case fixtures newly admitted. Each changed row is explained, or the work stops (SG).
10. **The validity of the RSS measurement.** It needs a quiet host, three repetitions, the process-floor control and the value-route control. Debug times are pessimistic, and the record says so.
11. **NUM sequencing.** B1 integrates only when NUM carries no other unmerged product slice: SI1b and B6 go first.
12. **SP's size.** SP is the largest uncertainty, and the checkpoint after T-7 re-estimates it. If SP runs over 22 h, ROOT may give the serializer (T-11) to a second implementer after the T-7 checkpoint, with I-P owning integration.

## 9. The cap shape: what depends on it, and the contingency

**Selected:** option S3 (RR "I82's addendum…").
- **What depends on it:** SA's cap values and rows, SQ's single profile and form set, and the three-case cap-maximal witness at 32/32/32 with 128 loads per case.
- **What does not:** ST, SP, SR, SC and SG, which have the same content under any shape.
- **The committed witnesses stay in the domain:** W1–W7, W2-deep, W2, W2b's input and headroom, and U8's inputs. W-C2 fits exactly (C = 3).

**The contingency** (RR: if G5 on the real code exceeds 12 GiB at C = 3, ROOT returns to I82's options or to P1, and tells the owner). The plan absorbs it as follows:
- **A trimmed single tier at C = 3** (for example l or m lower; ADD §2): SA changes values only. W2 and W2b leave the domain, and are re-based on a cap-maximal input at the trimmed caps (an SW rerun at those counts, 2–3 h). SQ is unchanged in shape.
- **P1's two tiers** (STUDY §4.1): SA gains tier selection, two cap tables, and G-B, G-C and budget bounds per tier (+3–4 h). SQ gains a second TEXT chain and form set per mode, and the witnesses, maximum tests and RSS runs per tier (+5–6 h). Review grows by +2–3 h. W2 and W2b stay in the domain (tier 1 = D1).
- **The trigger point** is the G5-early dry run, or at the latest G5-final (R6a). Only SA's values and SQ are redone; nothing in lanes 1 or 3 waits.

**Finding 8** (B2's room) follows from the selection. It is a planning input for B2, not a reason to revisit B1.

## 10. Estimates

| | Option S3 |
|---|---|
| **Agent** (SW to SK) | **67–102 h** |
| **Review** (RV-P 11–17 h, RV-R 7–11 h, RV-Q 8–13 h) | **26–41 h** |
| **ROOT:** rulings, return verification, integration, registration, gates, merge | 8–12 h |
| **Machine:** full suite, exact-head DEC-025 with baseline, T9, both-entry part 1, peak RSS and time | 4–6 h |
| **Elapsed** | About 6–9 working sessions, with B6's PR in parallel at the start |
| **If §9's contingency fires with P1** | +8–10 h agent, +2–3 h review |

**Against DESIGN §9** (37–59 h agent, 12–17 h review), the difference comes from four places:
- **SP alone is 14–22 h.** PLAN's 30–45 h was a reading estimate made before the single-case producer was audited (finding 1).
- **The readers take four people:** three implementers plus a corpus owner.
- **Obligations were added since:** the W2b probe as its own slice, the peak RSS and time measurement, the registered-sweep gate, Pass B on a new reference, and the package.
- **Review is counted per reviewer role,** including the PR-head ledger.

These are reading estimates, of the same kind as I61's and I74's.

## 11. Effects on the breadth order and the owner's F2a order

**Neither order changes.** The breadth order is U8 → B0 → B1 and B6 → PR-B1 → B2 and B3 → B4 if ruled → PR-B2 → B7 → B8, with S-I2, F2b and F3 after. For ROOT to weigh:
1. **B6 merges before PR-B1,** as its own PR. This changes PLAN decision 8's packaging, not the order.
2. **Finding 8.** Under option S3 at 12 GiB, B2 cannot add a D1-cap retained combination to a three-case invocation. B2's plan needs its own reduced tier or lower C for combination-bearing invocations, or the owner (above 12 GiB). B4 (cap growth) faces the same ceiling.
3. **B1's estimate grows** (§10). That moves the breadth schedule, not its order.
4. **B7 inherits §3.6's method** for the release identity, under the owner's new obligation.

## 12. What I read, and limits

**Read (sha256):**

| Input | sha256 |
|---|---|
| The brief | `d9b8bf0d…6cf0` |
| DESIGN, whole: T-1 to T-13, §1.3, §1.4, §2, §3, §6, §7, §8, §9 and §10 | `5933b90b…1114` |
| PROBE, whole | `3e32726d…392c` |
| STUDY, whole | `d8b18220…7188` |
| STUDY's `report.json` | `a0a8e2ce…f0` |
| ADD, whole | `7c155ceb…0ce2` |
| PLAN §0–§2.2, §5 and §6 | `f274a614…3def` |
| CR, whole | `f4207994…d476` |
| QUAL, whole | `8edbf4b4…2c29` |
| I74 PLAN §0 and §4–§8 | `0350c918…2ed9` |
| `BRIEFS/B6_READER_ITEMS.md` | `a3634160…3777` |
| `BRIEFS/U8_COMMON.md` | `3146c3e6…223b` |
| `BRIEFS/I68_U8_PROBE_AND_WITNESSES.md` | `5ef6f76a…e8b4` |
| `BRIEFS/I72_U8_PASS_B.md` | `adfc130d…0862` |

**Also read:**
- **RR:**
  - "T3's gate set and Git rules…" and "Owner direction: proportionate CI…" (RR:12179–12304);
  - RR:12805–12912;
  - every section from "Owner decision: ROOT may raise M up to 6.0 GiB…" (RR:13409) to the end, including the three added while I worked.
- **The work graph's T3 section,** with its owner-held list as amended for 12 GiB.
- **I72's Pass B RETURN:** its verdict line and gate table.
- **I65's `u4_g7_06/_run_records/`:** the listing, and `g7_pass.sh`'s non-candidate gate.
- **I61's `u9_plan_01`** gate rows, and **`u9_g8_01`'s RETURN** §0–§2.

**Code read at main `47a3bdfcf5`:**
- **PP `lib.rs`:** `W1Fallback`; the Direct and dispatch functions; `run_linear_static_preview_observed`'s case loop; `permitted_dispatch`, `on_reserved_stack`, `permitted_run`, `ReservedNotice`, `w1_case_id` and `retained_w1`.
- **PP `retained_product.rs`:** `SolverObservations`, `CaptureError`, `ProductCapture`, `OrdinarySeed`, `InitialSeed`, `W2Seed`, `LegacySeed`, `prepared_case_seen`, `prepared_observation_custody`, `prepared_case_source`, `prepare_owned_case` and `PreparedCase::solve_native`.
- **PP `retained_wire.rs`:** `one_case`, `one_run`, `kernel_outcome`, `run_value`, `invocation_arrays`, `ordinary_value`, and the signature of `finish`.
- **PP `retained_memory.rs`:** `caps`, `family_clauses`, `cap_rows`, `provenance_clause`, `domain_clauses`, `REGISTERED_PROFILES`, `build_status`, `cap_priced_maximum`, `priced_maximum`, `bound_admits`, `admission_bound`, `late_observations`, `complete_observations`, `ordinary_solve_attempted`, `phase_caps`, `PHASE_BUDGETS`, `CapturePermit`, the head of `admit`, and the `tests` module.
- **PP tests:**
  - the facade tests: the test list, U8's helpers and test, the R-2 base-reader test, and the two-case sites;
  - the witness tests: the driver, W2, W2-deep and W2b;
  - the law tests, at the registered-admission site;
  - the `multiple cases` site in `retained_product_tests.rs`.
- **`P/core/runner/headless/tests/retained_precision_admission.rs`,** at its oracle.
- **FK `origins.rs`:** the signatures of `for_calls` and `solve_cases`.
- **RS:** its gate list and G8 sites, by grep.
- **The milestone request:** its counts.

**Computed** (VENV, read-only, on committed bytes; in `_run_records/`):
- `cascade_static.py` over 07l and 07m, with its outputs;
- `budget_at_12gib.py` over STUDY's `report.json`, with its output. It agrees with ADD §1 for option S3 at 12 GiB: 1,848,686,021 B dense, 44.1 %; 5 % M = 10.30 GiB;
- `inputs.txt`, the input hashes.

**Limits:**
- **Nothing was built or run.** Every producer and reader claim comes from reading, or is quoted from records.
- **Finding 5 is static.** SR's census is the check.
- **Option S3's figures are I82's emulated prices.** B1's G5 supersedes them.
- **Not audited here:**
  - the RS, PY and TS checks for D38's obligation (SR-RS lists them);
  - the T6S suites;
  - every PP test outside the retained files.
- **B6's uncommitted work was not read:** F-U6b-2, and whether RV78-N1 needs a checkpoint. The plan assumes B6's corpus edits land before 07n, as briefed.
- **The estimates are from reading,** and SP's is the widest.

# I84 B1-P: B1's implementation plan, revision 1 (documents only)

TASK (Type 2), I84, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-07 UTC.

**This revision replaces `PLAN.md` (sha256 `7f9699f3…4bb5c`, kept unchanged beside it) and stands alone.** It answers RV107's review and ROOT's ruling R1 on it:
- the review: `R/REVIEW_RV107/b1_plan_01/REVIEW.md`, sha256 `03c3111d8baec21503b8212e50dc40ccb002a77e79db8127bfbd6467375aa426` (verified): PASS with findings, 0 BLOCKING, 7 SHOULD-FIX, 16 NOTE;
- the ruling: RR "R1: B1's plan ruled with seven amendments; I84 writes PLAN_v2; B6 first as its own PR".

`R1_AMENDMENTS.md` maps each finding to the section that answers it. In this text, **`[r1: X]` marks a passage that answers finding X** (SF-n or N-n), or ruling D15 (decision 15) or D16 (decision 16).

**The brief:** `R/BRIEFS/B1_PLAN.md`, sha256 `d9b8bf0dfec96aed4ac728d55b06e7ac97e73f2f94fc7ffee2d37c9f905c6cf0`.

**The selected shape** (RR "I82's addendum: B1's target is S3, one tier at D1's caps with C = 3"; `R/I82/b1_cap_study_01/ADDENDUM_01.md`, `7c155ceb…0ce2`):
- one tier at D1's full model caps: n = m = g = 32, Σr = 192, l = 128;
- C = 3, with L = 384 stated and not binding;
- M selected at G6 as the smallest 256 MiB step that holds at least a 5 % text-error budget in both modes. The pricing expects 10.5 GiB, and the ceiling is 12 GiB (owner, 2026-10-07).

To avoid a clash with I82's option label "S3", the slices are named SW, ST, SP, SA, SR, SC, SQ, SG, SB and SK. "Option S3" always means I82's shape.

**Placeholders:** WT, NUM, P, PP (= `P/core/product_physics/src`), RE (= `P/core/reporting/result_export`), T, R, RR and VENV as in the dispatch. Also:
- **DESIGN** = `R/I78/b0_contract_01/DESIGN_v2.md`;
- **PROBE** = `R/I81/b1_probe_01/PROBE.md`;
- **STUDY** and **ADD** = `R/I82/b1_cap_study_01/STUDY.md` and `ADDENDUM_01.md`;
- **PLAN** = `R/I61/u8_plan_01/PLAN.md`;
- **CR** and **QUAL** = `T/IMPLEMENTATION/F2A_D1/CHANGE_RECORD.md` and `copies/QUALIFICATION.md`;
- **RS**, **PY**, **TS** = the Rust, Python and TypeScript retained readers: `RE/src/retained_precision.rs`, `P/core/analysis_runs/retained_precision.py`, `P/apps/desktop/src/features/results/retainedPrecision.ts`;
- **CORPUS** = `P/fixtures/results/retained_precision_cases.json`;
- **C** = 3, **l** = 128, **L** = 384; **A** = the attempted cases (T-4);
- **PP-tests** = `P/core/product_physics/tests`.

**Basis.**
- **Code:** main `47a3bdfcf5`. NUM is now `ef43a1d694`. Outside `P/execution`, it still equals main plus SI1b's three rules-crate files (`git diff` from `f59542f71b` is empty there). B1 does not touch those files.
- **B6 (I83)** is at `a79dbd2e4a` on `codex/piping-t3-b6-20261007`. Its corpus 07m is unchanged since `5e38293530`: sha256 `c21112fd…6807`, with 17 bases, 294 mutations and 28 must-pass entries. Its full file set is in §6 `[r1: N-14]`.
- Code is cited by symbol.

**Limits kept.** Documents and code reading only, plus read-only Python (VENV) on committed bytes (`_run_records/`, unchanged from revision 0). No cargo, vitest, native or solver job; no installs; no Git writes (reads used `GIT_OPTIONAL_LOCKS=0`); nothing in the system temp directory.

## 0. Findings in brief

1. **The producer is single-case throughout, so SP is the largest and least certain slice.** RV107 confirmed this in code.
   - `ProductCapture` holds one `source` and one `case_id`.
   - `capture_case_source`, `prepared_case_seen` and `prepared_case_source` refuse `load_cases.len() != 1`.
   - `solve_native` builds `OriginCapacity::for_calls(&[1], &[])`.
   - `one_case` and `one_run` check that there is only one case, run, call and source.
   - The kernel's `solve_cases` is already n-case, so **FK should not change.**
2. **The c = 1 behaviour change lands first, as a small slice (ST).** It is T-4, decision 21 and `NoTriggeredCase`, written as the n-case classifier and used at c = 1.
   - **Its oracle:** every committed c = 1 successor pin stays byte-identical.
   - **ST also carries the SA–SP seam,** so SA can run beside SP `[r1: SF-1, SF-4]`.
3. **W2b's replacement and the cap-maximal three-case witness share one model,** at D1's full caps. One probe (SW) establishes both. It now also looks for a cap-maximal variant that **publishes a successor**, so that the phases where the bound binds (W3–W5) are measured `[r1: SF-5]`.
4. **Seven committed tests use a two-case milestone as their out-of-domain oracle** `[r1: SF-2]`. The milestone (2 nodes, 1 member, 4 supports, 3 loads) is now inside the domain, so all seven are re-based:
   - four facade tests;
   - the law tests' `admit_grants_…`;
   - `retained_memory.rs`'s `actual_retained_entry_dispatches_ordinary_once`;
   - the runner's `explicit_headless_refusal_…`.

   The D1.4 line of `every_family_clause_refuses_with_its_fact` changes too.
5. **The reader alignment's re-pin cascade on 07m is empty, statically.** RV107 confirmed this by rerunning `cascade_static.py`. SR's mechanical census remains the check, with stop rule R5.
6. **W-C2's successor cannot be pinned before RS is aligned** (RV107: TRUE). Until then, the private driver falls back at precommit G5 (case B's `not_required`), and SP reads per-case outcomes through the existing `before_precommit` hook `[r1: N-1]`.
7. **One tier keeps admission simple.**
   - `LOAD_CASES` becomes 3, and the per-case and total load rows are added.
   - G-B and G-C get one set of bounds at C = 3.
   - There is one profile form set.
   - `cap_priced_maximum` and `admission_bound` keep their form.
8. **B2 (likely; the combination's price is not yet computed)** `[r1: N-11]`. Option S3 leaves 1,848,686,021 B of dense margin at 12 GiB (ADD §1). A combination's rows alone grow TAV_W and T16 by about 1.3 and 1.2 GB per case's rows at D1's caps (ADD §1; STUDY §3.3), which is already beyond that margin.
   - So B2 probably needs its own reduced tier, a lower C for combination-bearing invocations, or the owner.
   - B2's study prices it. Nothing in B1 changes.
9. **B6 goes first, as its own compact PR** (ruled at R1, decision 15) `[r1: D15]`.
10. **Nothing here changes the breadth order or the owner's F2a order** (§11).

## 1. The fence `[r1: SF-3, N-4]`

| Area | Files | Slice and owner |
|---|---|---|
| PP transaction | `PP/lib.rs` (the retained section, and observer call sites inside the ordinary run, only under `if let Some(observer)`); `PP/retained_product.rs`; `PP/retained_receipt.rs`; `PP/retained_wire.rs` | ST, SP: I-P |
| PP tests and hooks | `PP/retained_facade_tests.rs`; `PP/retained_product_tests.rs`; `PP/retained_wire_tests.rs`; **`PP/retained_tests_hooks/grant2.rs`** (per-case fault hooks) | ST, SP: I-P |
| **Source-text guards outside `src`** | **`PP-tests/s11f_site_test.rs`** (rule 8's `TABLE` gains a row for each new accumulation site, each with its disposition); **`PP-tests/retained_precision_admission.rs`** (its expected `lib.rs` sections) | SP: I-P |
| The seam's struct fields | `PP/retained_memory.rs`: only the `CompleteFacts` field named in §2.1; `PP/retained_memory_law_tests.rs`: only the existing `CompleteFacts { … }` literals, which gain that field | ST: I-P, before SA starts |
| PP admission | `PP/retained_memory.rs` (with its `tests` module, but not the generated block); `PP/retained_memory_law_tests.rs`; `P/core/runner/headless/tests/retained_precision_admission.rs` | SA: I-A |
| PP qualification | `PP/retained_memory.rs`'s GENERATED PROFILE block and `REGISTERED_PROFILES` (as `registration.diff`, applied by ROOT); the law tests' pinned constants (N-8, §3.3); `PP/retained_memory_witness_tests.rs` (after ST); `PP-tests/retained_memory_challenge.rs` | SQ: I-A |
| Readers | RS and `RE/tests/retained_precision_contract.rs`; `RE/src/source_blocks.rs` (RV95 N-5's `#[cfg(test)]` test only) | SR-RS: I-RS |
| | PY and `P/tests/test_retained_precision_contract.py` | SR-PY: I-PY |
| | TS and `retainedPrecision.test.ts` | SR-TS: I-TS |
| Corpus and fixtures | CORPUS (07n, append-only after B6's 07m) | SC: I-PY |
| | New `P/fixtures/results/retained_precision_w_c2_successor_{sparse_interactive,dense_scrutiny}.json` | SP: I-P |

**Source-text guards that read fenced files** (§8 risk 8) `[r1: SF-3]`:
- **Outside `src`:**
  - `s11f_site_test.rs`, which scans `PP/lib.rs` and `PP/retained_product.rs` with rules 1–6 and rule 8;
  - PP-tests' `retained_precision_admission.rs` (`legacy_native_and_typed_call_graphs_cannot_acquire_retained_admission`), which reads `lib.rs` sections;
  - `RE/tests/retained_precision_carriers.rs`, which names `nonlinear_assembled_loop_context`;
  - FK's `k2a_checked_formation.rs`, a line-range comment.
- **Inside the fence:**
  - `u1_serializer_reads_no_legacy_work_field`, which forbids `.charged()`, `.total()` and the legacy work fields in `retained_wire.rs`'s production code;
  - the law tests that read `admit`'s source;
  - `challenge_bounds_are_the_profile`, which reads the challenge file;
  - `u3_n9_single_parse_custody`, `u3_permitted_outputs_keep_the_report_and_gate_order`, `u3_capture_permit_is_linear` and `u3g2_no_permit_path_runs_once_without_a_copy`.
- **Rule:** I-P runs the outside-`src` guards and RE's carrier test early in ST and in SP. A changed expected text needs a stated reason. A new rule-8 site gets a `TABLE` row with its disposition, reviewed by RV-P. A weakened assertion is a stop.

**Never touched without a stop and a ROOT ruling:**
- FK;
- the schema `P/schemas/retained_precision_mp_v2.schema.json`;
- the 13 reviewed statics and `Cargo.lock`;
- the base (non-retained) readers;
- the ordinary route's behaviour (the observer-free paths);
- the T6S files;
- B6's items;
- **the visibility of any D1 `src` item for a test's sake** `[r1: SF-2]`.

**The registration** changes `threshold_bytes`, the generated profile, and the cap constants `LOAD_CASES` and `TOTAL_LOADS`. It also re-pins the tests listed in §3.3 `[r1: N-8]`. The identity, the reviewed inputs and `READER_LAYOUTS` are unchanged. An RS change to the layout of `Validation`, `ValidationError`, `RowClassification` or `AccuracyClass` is a stop.

## 2. The slices

IDs are roles; ROOT assigns fresh IDs. The next unused are I85 and RV108. Estimates are agent hours, without repair rounds (§10 adds them).

**Worktrees** `[r1: SF-1]`. Every concurrent lane that builds PP or RE has its own worktree, branch and target directory. Each branch forks from the common base named in §4. Python and TS lanes also get their own worktrees, which is cheap and keeps the merges clean. ROOT integrates at the named points I1–I5, with `--no-ff` merges and no rebase.

| Slice | Owner | Worktree / branch | Content | Depends on | Estimate |
|---|---|---|---|---|---|
| **SW** | I-W, fresh | disposable archive (records only) | §2.0 | R1 | 4–7 h |
| **ST** | I-P | `WT/b1` / `codex/piping-t3-b1-<date>` | §2.1 | R1 | 5–7 h |
| **SP** | I-P | `WT/b1` | §2.2 | I1 | 16–25 h |
| **SA** | I-A | `WT/b1-a` / `…-b1-a` from I1 | §2.3 | I1 (ST's seam) | 5–8 h |
| **SR-RS** | I-RS | `WT/b1-r` / `…-b1-r` from I1′ | §2.4 | I1′ (B6 on main) | 7–10 h |
| **SR-PY** | I-PY | `WT/b1-p` / `…-b1-p` from I1′ | §2.4 | I1′; a free implementer slot | 5–7 h |
| **SR-TS** | I-TS | `WT/b1-t` / `…-b1-t` from I1′ | §2.4 | I1′; a free implementer slot | 5–7 h |
| **SC** | I-PY (corpus); I-RS, I-TS (pins) | `WT/b1` after I4 | §2.5 | I4; RV-P round 2 settled | 6–8 h |
| **SQ** | I-A | `WT/b1` after I5 | §3 | I5 (the code freeze) | 17–25 h |
| **SG** | I-G, fresh | an archive of the integrated candidate | §3.10 | SQ's registration applied | 2–4 h |
| **SB** | I-A, or a fresh holder of I72's role | an archive of the PR head | §3.8 | The PR cut | 2–3 h, plus the build |
| **SK** | I-K, fresh | records only | §6 | SQ, SG, SB | 3–5 h |

### 2.0 SW: W2b's replacement, a publishing cap-maximal input, and the three case components (a probe; records only)

**Write set:**
- a disposable `git archive` of main `47a3bdfcf5` in `WT/scratch/<id>_b1_w/`;
- the target `WT/targets/<id>-b1-w/`;
- the records `R/<id>/b1_w_probe_01/`: PROBE.md, `_run_records/` and SHA256SUMS.

No maintained file changes. Probe-only `cfg(test)` code goes in the archive, by I81's method.

**Item 1: W2b's replacement** (RR "I81's B1-0 probe verified…", ruling 1; PROBE §5).
- **The model:** D1's cap-maximal counts. That is 32 nodes, a 32-member ring, 32 supports, 128 loads, 4 + 4 materials with 16 temperature points, and 128-byte identifiers.
- **Constructions:**
  - (a) scale the ring's sections down until the reciprocal condition estimate falls below √eps while the solve still succeeds;
  - (b) arrange loads and springs so that K-D5's formation check demotes the report.
- **For each variant,** in both modes, through Direct and through the witness driver at 4 MiB, record:
  - the published verdict and the seed;
  - `MECHANICS_SOLVED`;
  - the W1 stage reached and the kernel terminal.
- **A variant qualifies** when its verdict is in A (Sensitive), it is solved, and W1 reaches the native stage.

**Item 2: a cap-maximal input that publishes a successor** `[r1: SF-5]`. This is preferred for the measurement, because the priced bound binds at W3, with W4 next (ADD §1).
- **Construction (c):** replicate the milestone's body, which publishes, as disconnected loaded copies.
  - Fill the remaining node, member and support counts with unloaded, fully restrained bodies. Their rows are exact zeros, as the L = 0 base and W-C2's case A show.
  - Spread up to 128 milestone-type moments across the copies.
- **Variants of (a) and (b)** that reach a successor also count.
- **The stop:** if no variant within its ladder publishes, the record says so. **§3.6 then states plainly that W3–W5 are unmeasured at the caps,** and are measured only on W-C2 and the milestone.

**Item 3: the three cap-maximal case components.**
- Three distinct 128-load sets on one cap-maximal model. Each is run alone as a one-case request, and each must be Sensitive and reach native (I68's and I81's method). Use item 2's model if it publishes, otherwise item 1's.
- **Stresses** `[r1: N-7]`:
  - a quote and a backslash in every provenance;
  - a raw value of depth 16 (STUDY §6 item 3; W2's helpers).
- **The domain** is checked with `assess` on the assembled three-case request. RV107's census estimates 6,384 of 16,384 raw values, and 49,428 raw string bytes (57,043 escaped) of 65,536, so it is close. If the request falls outside D1, provenance is shortened until `assess` admits it, and the margin is recorded.

**Item 4: an early reading.** Run `/usr/bin/time -l` on each run, one mode per process. This is informational.

**Controls:**
- the committed W2 and W2b reproduce QUAL §4 in the same build;
- two runs give identical probe lines.

**Stop rule (ruled).**
- **Item 1:** if neither (a) nor (b) qualifies within at most 6 variants each, return. ROOT rules option (c): W2b's input stays a `NoTriggeredCase` pin, and the cap-count stack evidence rests on QUAL §4's argument plus case C's full native run.
- **Item 2:** its stop is stated above.

### 2.1 ST: T-4 at c = 1, decision 21, `NoTriggeredCase`, the re-basings, and the SA–SP seam

**The branch:** ROOT cuts `codex/piping-t3-b1-<date>` from main, with worktree `WT/b1`.

**Code (`PP/lib.rs`):**
- Add `W1Fallback::NoTriggeredCase`.
- Add one per-case classifier, written for n cases. **The published verdict is looked up by `numerical_quality.cases[].basis_ref.ref_id`, not by position, as `ordinary_value` does** `[r1: N-3]`.
  - `not_required` exactly when that verdict is `checks_passed`.
  - **Excluded** when the seed's `initial` is `StructuralFailure` with tag mechanism, asymmetric or invalid input, and `w2` is not `Published`.
  - **Otherwise in A.** That includes an absent entry, `not_assessed`, and a case with no seed (ruling 2).
- `retained_w1` applies the classifier after coexistence and G-B, and before the notice reservation. If A is empty, the result is `NoTriggeredCase`: exact bytes, no notice.
- `w1_case_id` is unchanged at c = 1. SP replaces it (§2.2).

**The seam for SA and SP** (behaviour-neutral at c = 1) `[r1: SF-4]`. I-P adds both fields before SA forks:
- **`ProductCapture::late_loads_total: usize`** (`retained_product.rs`). `prepared_case_source` adds `case.primitive_loads.len()` to it, with a checked add (`CountRange` on overflow), immediately before calling `check_late`. G-B at case k therefore sees Σ_{i≤k} l_i.
- **`CompleteFacts::requested_cases: usize`** (`retained_memory.rs`'s struct). `permitted_run` (`lib.rs`) sets it from the typed request's `model.load_cases.len()`, read before the request moves into the observed run. No allocation.
  - The law tests build `CompleteFacts` literally at seven sites today. ST adds `requested_cases: 1` to each, so the crate still compiles. This is the only ST edit to SA's files, and it lands before SA forks.
  - `LateFacts` is unchanged: the total travels in the capture.
- **Neither field is read in ST.** `ordinary_solve_attempted` and G-B's observation list stay as they are; SA reads the fields after I1.

**Tests (`PP/retained_facade_tests.rs`):**
- **W-C1's variant is re-based on case C alone** (PROBE §4's input, `3649b4dc…`) through Direct, in both modes. It asserts `Native`, one notice, `with_notice(plain, …)`, `ONE_RUN_THROUGH_G_C`, admitted, and no hooks. The kernel reason is recorded, not asserted.
- **`NoTriggeredCase` pins for two-body B** through Direct: exact bytes, no notice, `ONE_RUN_THROUGH_G_C`, no W1 work.
- **The four facade out-of-domain oracles** `[r1: SF-2]` now take `crate::retained_memory::caps::LOAD_CASES + 1` cases. The constant is visible inside the crate. The tests are:
  - `u3_each_stage_fault_falls_back_to_the_ordinary_bytes`;
  - `u3_no_permit_entries_are_the_ordinary_route`;
  - `u3g2_direct_entry_no_w1_refusals_keep_exact_bytes`;
  - `u3g2_no_permit_path_runs_once_without_a_copy`.

  Each keeps its combination variant where it has one. Before SA they still use 2 cases, which is today's behaviour; after SA they use 4. Their labels say "C + 1 cases".
- **I77's two citation renames:**
  - `zz_rv93.rs:292–315` becomes `zz_rv93_input_fallbacks`;
  - `retained_memory_witness_tests.rs :181–199` becomes `w6_input()`.

**Tests (`PP/retained_memory_witness_tests.rs`):**
- **W6's stack witness** is re-based on case C, on its private driver at 4 MiB, asserting `Fallback("Native")`.
- **W6's PHYS-R4 input and W2b's committed input** stay as `NoTriggeredCase` pins (ruling 4). W2b's test is renamed to say what it now pins, and its doc line says that its cap-maximal native role moves to SQ.
- **W2** is unchanged.

**Acceptance:**
- **Byte identity:** every committed c = 1 successor pin is byte-identical: the milestone, L = 0 and U3's pins, in both modes.
- **The registered PP suite** differs from base only in the listed tests: PROBE §2.4's table, plus the new pins and the renames.
- **The source-text guards** (§1) pass. Their expected text changes only with reasons.
- **Unit tests of the classifier** with hand-built seeds:
  - each verdict;
  - W2-published Passed;
  - report Passed;
  - each excluded tag, with and without W2;
  - no seed;
  - a quality entry out of request order, for the id lookup.
- **Mutants,** each killed by an assertion, never by compilation:
  - keying on `initial`;
  - looking the verdict up by position;
  - dropping decision 21;
  - a seedless case excluded;
  - `NoTriggeredCase` appending a notice.
- **Stale:** every output is the plain bytes.
- **Decision 21's branch** is pinned only by unit tests, because no audited committed input reaches it (PROBE §3).

### 2.2 SP: the n-case transaction (DESIGN T-1 to T-13)

**Code:**
- **T-2.** `ProductCapture` holds, for each requested case: the seed (already per case), the solver observations, and the late old-source capture with its facts and operational records. It stays one owner. The one-case scope and custody counters become per-case.
- **T-3.** The order (a) to (e) in `permitted_run` is kept. Fact (e) belongs to SA, through the seam.
- **The domain re-check in `retained_w1`** `[r1: SF-2]`. `w1_case_id`'s replacement returns the request's case ids, and **`retained_w1` refuses with `W1Fallback::Domain` when c = 0, c > `LOAD_CASES`, or there is any combination.** This keeps `u3_each_stage_fault_falls_back_to_the_ordinary_bytes` meaningful on the private driver.
- **T-5.** One `ReservedNotice` per case in A, all reserved before any W1 work. A collision gives `NoticeReservation`.
- **T-6.** Invocation custody is checked once. `bind_observations` runs per case. **When it binds by case index (decision 19), it keeps its envelope-binding checks.**
- **T-7.** Preparation runs over A in request order. Each case gets one C3 `ProductAttempt`, with ids in actual start order. A preparation failure makes only that case unavailable, with no `CaseSource`. `bind_preparation` names the attempt's own index.
- **T-8.** One `CaseBatchCall`: one `RecordedInvocation` (60 G), the case limit for each case, and `for_calls(&[|prepared|], &[])`.
  - **The kernel names owners by batch ordinal** (`NativeOwner::Case(used_cases + position)`). The serializer maps each ordinal to its request index, so `calls[].owner_refs` and each Run's `origin.owner_ref` name request indices. In W-C2, the batch is {A, C}, so the ordinals are {0, 1} and the request indices are {0, 2} `[r1: N-2]`.
  - A Run that is not selected makes its case unavailable (`kernel_*`). A call failure before any Run leads to T-12.
- **T-9.** One freeze per selected Run. A failure gives `facade_certificate`.
- **T-10 and T-11:**
  - **Staging:** one copy. The selected cases' overlays come first, in request order, then the unavailable cases' diagnostics. `not_required` and excluded cases get nothing.
  - **The receipt body** follows DESIGN T-11, with snapshots recorded at each attempt's terminal stage.
  - **Every serializer failure abandons the successor.**
  - **The R-b′ limit is kept,** and pinned by a hook test.
  - **Precommit** runs with the actual invocation. The transfer only moves values.
  - **The serializer's production code** still names none of `u1_serializer_reads_no_legacy_work_field`'s forbidden reads `[r1: SF-3]`.
- **T-12.** The ordinary bytes, then one N1 notice per case in A, in request order. The receipt-encoding detail is added only for C1:68's details or `publication_hash_range`, and only on cases that were selected when the successor was abandoned.
- **T-13.** One ordinary run; G-C is reached once.
- **The seam:** SP keeps `late_loads_total` and `requested_cases` as ST defined them.
- **I82's c² guidance** (STUDY §8.3; decision 19): bind observations by case index; collect `diagnostic_refs` in one pass; make the integrity diagnostic loop per case.
- **New accumulation sites in `lib.rs`'s non-test code:** prefer shapes outside rule 8 (checked arithmetic through named helpers). Otherwise add each to s11f's `TABLE` with its disposition `[r1: SF-3]`.

**Tests:**
- **W-C2:** three cases on U8's two-body model, in both modes.
  - **Before SR-RS** `[r1: N-1]`: through the private driver, assert `Precommit { gate: "G5", code: "RETAINED_PRECISION_ATTEMPT_MISMATCH" }` (case B's `not_required` fails first). Read the per-case outcomes from the serialized successor, which the existing `retained_tests_hooks::before_precommit` hook hands to the test:
    - A `selected`;
    - B `not_required`;
    - C `unavailable` with `kernel_terminal {unresolved, {space: unresolved, tag: ceiling}}`.
  - **After SR-RS is merged (I3):** the successor is pinned by sha256 (the document, the receipt and the published bytes), with the D-U6-5 fixtures, and the Rust reader passes with the invocation.
  - **Through Direct,** once SQ's registration is in the candidate (§3.3).
  - **The batch against the one-case runs** `[r1: N-16]`: A, B and C should match PROBE's one-case outcomes. Any difference inside the one `CaseBatchCall`, with its shared group builds and one meter, is recorded with its cause and returned to ROOT as a finding. It is not absorbed into the pin.
- **The ordinal mapping** `[r1: N-2]`: W-C2's receipt names request indices {0, 2} for A's and C's Runs and the call's `owner_refs`.
- **Multi-case coexistence** `[r1: N-5]`: a committed pin where exact-block selects one case or more of a multi-case input. It is built from the n05 source-blocks request with a second case added. The pin asserts:
  - `Coexistence`;
  - exact ordinary bytes;
  - no notice and no reservation;
  - counts `{runs: 1, complete_gates: 0}`;
  - in both modes.

  If n05 with a second case does not select, I-P records the variant tried and builds another from W3's fixtures, or returns.
- **Multi-case fault tests,** using hooks. Each checks T-12's notice count (= |A|) and the detail placement:
  - T-6 custody;
  - T-7: one case's preparation fails beside a selected case, giving a successor with that case unavailable and no `CaseSource`. **This needs a case-targeted hook,** proposed as `fail_preparation_of_case(index)` in `grant2.rs`. Today's `fail_next_preparation` would fail the first attempted case, which is W-C2's A `[r1: N-4]`;
  - a T-8 call failure (D38's test-hook shape stays refused);
  - staging, each serializer detail, and precommit.
- **T-12's base readers:** `u3_r2_base_readers_accept_the_unavailable_notice` is extended to several N1 notices. The bytes are written out for PY's and TS's lanes.
- **The rest:**
  - `retained_product_tests.rs`'s prepared-scope cases (`multiple cases`) are restated for n-case custody;
  - the in-crate source-text pins change only in their expected sections, with reasons;
  - the outside-`src` guards (§1) are run at SP's checkpoint and at its end.

**Acceptance:**
- c = 1 byte identity holds.
- W-C2's outcomes hold (or are returned under N-16).
- `ONE_RUN_THROUGH_G_C` holds, hooks are empty, and Stale gives the plain bytes.
- **Mutants,** killed by assertions:
  - the staging order;
  - `attempt_ref`;
  - the snapshot record point;
  - the detail placement;
  - the reservation count;
  - each member of decision 5's abandonment set;
  - **the ordinal-to-request mapping;**
  - **the domain re-check (c = C + 1; one combination).**

**Checkpoint (R3′):** after T-2, T-6 and T-7, I-P re-estimates SP, and RV-P may start reading.

### 2.3 SA: admission at option S3

SA runs in `WT/b1-a`, forked from I1, beside SP. It is split from SP by file `[r1: SF-1]`.

**Caps:** `LOAD_CASES` = 3; a new `TOTAL_LOADS` = 384, stated and not binding (ADD §2). Every other D1.9 and D1.11 cap is unchanged.

**Census:** the per-case load facts (the maximum length and capacity over cases) and the total.

**Clauses:**
- **D1.4:** 1 ≤ c ≤ 3, no combinations, no components.
- **D1.5 and D1.7** apply to every case.
- **D1.10** already reads every case.

**`cap_rows`:**
- `LoadCasesCapacity` ≤ 3;
- per-case `Loads` and `LoadsCapacity` ≤ 128;
- a new `TotalLoads` ≤ 384.

**G-B (`late_observations`),** using the seam `[r1: SF-4]`:
- `CaseLoads` ≤ 128, from `LateFacts.case`, as today;
- **a new `CaseLoadsTotal` ≤ 384, from `LateFacts.capture.late_loads_total`.** `LATE_FACTS` goes from 9 to 10;
- `LateObservationBytes` ≤ T11 − T11_late_capture, from the regenerated forms.

**G-C (`complete_observations`),** at C = 3:
- **EnvelopeResults ≤ 3·P_final,** with its capacity and text `[r1: N-13]`;
- D_env, Text(diag_env), L_PUB and L_DIAGID from the regenerated TEXT;
- the contract-evidence facts × 3;
- ObservationBytes and OrdinarySeedBytes from the regenerated forms;
- RetainedErrorTextBytes ≤ 3·(3m + 1)·Text(err). This is I82's assumption. I-A checks it against SP's producer when SP is complete (phase 4), before I5.

**T-3 (e),** using the seam: `ordinary_solve_attempted(capture, requested)` holds exactly when `requested ≥ 1`, `capture.ordinary.len() == requested`, and every seed's `initial` is set. `requested` comes from `CompleteFacts.requested_cases`.

**Budgets:** `phase_caps()` and `PHASE_BUDGETS` take the regenerated forms; B-6 is NOTICE_RESERVE_BYTES × 3.

**Pricing:** `cap_priced_maximum` and `admission_bound` keep their form. Nothing leaves the branch before SQ (decision 17).

**Law tests:**
- each new row at its cap and at cap + 1;
- c = 0, 1, 2, 3 and 4;
- G-B per case and in total;
- the EnvelopeResults bound `[r1: N-13]`;
- T-3 (e) with an input whose case k < c − 1 blocks: G-C declines, with exact bytes and no notice;
- **the D1.4 line of `every_family_clause_refuses_with_its_fact`:** C + 1 cases and 0 cases refuse with `(Invocation, LoadCases)` `[r1: SF-2]`.

**Values that depend on the regenerated profile are re-pinned at SQ** `[r1: SF-4]`. These are the G-B and G-C byte bounds that read `profile::F_T11`, `F_T11_LATE_CAPTURE`, `F_T11_ORDINARY_SEED` and `text_atoms::*`.
- SA's tests of these facts assert the expressions: the bound equals the named form or atom times its count.
- Their numeric values are fixed by SQ. This is a back edge from SA to SQ.
- RV-Q round 1 reviews SA's expressions, not their values.

**Out-of-domain oracles** `[r1: SF-2]`:
- **In the crate:** the law tests' `admit_grants_…` site and `retained_memory.rs`'s `actual_retained_entry_dispatches_ordinary_once` take `caps::LOAD_CASES + 1` cases.
- **The runner's `explicit_headless_refusal_…`** cannot read `pub(crate) caps::LOAD_CASES`, and **no D1 `src` visibility changes for a test.** So:
  - it uses a literal 4, with a comment naming `product_physics::retained_memory::caps::LOAD_CASES` + 1;
  - it asserts the observable refusal: exact ordinary bytes, no successor, `typed.load_cases.length == 4`;
  - a new PP law test ties the literal to the producer. It asserts `caps::LOAD_CASES + 1 == 4` and that `admit` refuses `LOAD_CASES + 1` cases at D1.4 with `(Invocation, LoadCases)`, and its comment names the runner test.

**Mutants** `[r1: N-13]`, killed by assertions, never by compilation:
- D1.4's bound (≤ C against < C or ≤ C + 1);
- the per-case load rows (case 0 only, against every case);
- G-B's running total (dropped; current case only);
- T-3 (e)'s requested count (seeds only; `requested` ignored; `≥` instead of `==`);
- the EnvelopeResults bound (P_final instead of 3·P_final).

### 2.4 SR: the three readers

**The common specification:**
- DESIGN §2: R-D38 with (4b), and m1–m8;
- DESIGN §3.2, text B: P1–P4 (P5 deferred);
- DESIGN §3.3's G8 per-case loop, with `RETAINED_PRECISION_PREPARATION_MISMATCH`;
- DESIGN §3.3's G5 `not_required` rule.

**Precondition for each owner: the cascade census.** Run the aligned reader over 07m's 294 mutations and 28 must-pass entries. Expected: zero changes. Any change stops the work and goes to ROOT (R5).

**D38's obligation, for each reader** `[r1: N-6]`. List every check that assumes a prepared source has a Call or a Run. Relax each to (4b), or show it does not apply. The list goes in each owner's RETURN, and RV-R checks all three.

**SR-RS** (its own worktree `WT/b1-r`; RS is on the D1 call graph and PP compiles it for precommit) `[r1: SF-1]`:
- G8 is widened to every case, with P2–P4.
- G5 drops the three conjuncts (`initial.kind == report`, `outcome == checks_passed`, `w2 == not_triggered`).
- Unit tests on synthetic n-case receipts. The module doc of `RE/tests/retained_precision_contract.rs` is reworded (RV97 R2-N-2).
- RV95 N-5's direct `#[cfg(test)]` test in `RE/src/source_blocks.rs`, as I74 decision 9 and RV101 NT-1 and A2-N3 specify.
- RS is frozen at I5, before SQ's final G5.

**SR-PY:** (4b) in `_g5_stages`, and G8's requested mode and P1–P4 per case. It builds on B6's F-U6b-2, which is on main by I1′.

**SR-TS:** (4b) in `productAttempts`; G8's codes change to `PREPARATION_MISMATCH`; mode code 3 is dropped; checks apply to every case. It builds on B6's G7 header change.

### 2.5 SC: corpus 07n (one writer), then the harness pins

SC starts only after **RV-P round 2 and its repairs have settled W-C2's bytes**, so that 07n's copies and its reseal are made once `[r1: N-9]`.

**Bases,** appended after 07m:
- `w_c2_sparse_interactive` and `w_c2_dense_scrutiny` (D-U6-5 copies);
- `d38_beside_selected`: synthetic, labelled "not producer-emittable under T-8". A records script derives it from a W-C2 base:
  - remove case C's Run, its `execution_order` entry and the Builds that C's Run originated;
  - remove C's source from the call's and the group's `source_refs`;
  - keep the shared group and A's builds;
  - recompute `charged` and the call's after-value;
  - set the cause to a typed `CaptureError::Origin`;
  - reseal every hash.

**Must-pass:**
- the two W-C2 bases;
- `d38_beside_selected` with its invocation (G0–G8 pass; standing `needs_recompute`);
- L = 0's entries, unchanged.

**Mutations:**
- D38: m1–m8;
- F-1, five entries:
  - dense L = 0's parity row onto W-C2 dense case A (P4);
  - the row duplicated (P2);
  - a parity row in sparse L = 0 (P3);
  - mode code 3 (P1);
  - `requested_mode` flipped;
- `not_required` on W-C2 case B, three entries:
  - its verdict set to `sensitive`;
  - `initial` set to `not_attempted`;
  - `product_attempt_ref` set non-null.

**Expectations:** each first failure is fixed from the three readers' agreement; a disagreement is declared per reader only by ruling.

**Acceptance:**
- all three readers pass 07n;
- counts move only by added entries;
- 07n's sha256 is recorded;
- PY and TS check SP's several-notice bytes.

Then I-RS and I-TS update their harness pins and counts.

## 3. Re-qualification: what each obligation must show, and who produces and reviews it

| # | Obligation | Produced by | Reviewed by |
|---|---|---|---|
| 3.1 | G5 | SQ (I-A) | RV-Q |
| 3.2 | M | ROOT (R6a, R6b) | RV-Q's G6 review |
| 3.3 | G6 and the registration's re-pins | SQ; ROOT applies the diff | RV-Q |
| 3.4 | S1 witnesses | SQ | RV-Q |
| 3.5 | Challenge | SQ | RV-Q |
| 3.6 | **Peak RSS and run time, with the counting-allocator peak** | SQ | RV-Q; ROOT reads it at R9 |
| 3.7 | QUAL §11's carry | SQ | RV-Q |
| 3.8 | Pass B | SB | RV-Q confirms |
| 3.9 | **The full 40-manifest suite and the src-tauri suite** before the freeze | ROOT | — |
| 3.10 | Direct-entry gates | SG | ROOT; RV-P reads the sweep delta |
| 3.11 | T9 and both-entry part 1 | ROOT | — |
| 3.12 | T6S consistency | The full suite; SK records it | — |

### 3.1 G5

**Method.** Re-run TEXT (`g7_pass.sh`'s chain) on the frozen code at I5: ST, SP, SA and SR-RS, with every review repair confirmed `[r1: N-9]`.
- Take I82's census and TEXT variables (`I82_PATCHES.json`: `c`, `a`, `L`, `cases`, `att`, `Pall`), and price at c = a = 3 and L = 384.
- **Loop rules:** `g7_linemap.py` only re-keys rules that already exist. So **I-A writes TEXT loop rules for every new or changed B1 loop** `[r1: N-16]`, using I82's 22 rebinds and 22 added edge multiplicities as the checklist of sites that must have one. RV-Q compares the real multiplicities with I82's emulated ones, site by site.

**It must show:**
- TEXT complete: no unmapped loop or argument, no multi-member SCC, a converged D fixpoint;
- the identifier audit enforced, with its controls;
- the regenerated GENERATED PROFILE block, with `ESTIMATES` = 0 and the forms gate equal;
- the law test printing `I65_G5_PROFILE` and `I65_G5_PHASE` per mode;
- **E_mov,max + R** within the text-error budget of ADD §1's 9,747,725,678 B (dense) and 9,688,594,334 B (sparse), or every difference explained, with the c² sites re-derived;
- **a c = 1 control run:** equal to today's record (3,575,778,286 / 3,595,488,734 B), or different only by named B1 loops.

**G5-early** is an informational dry run in phase 4, once SP is complete and SA and SR-RS are merged.

### 3.2 M

ROOT selects M by D-7: **the smallest 256 MiB step with E_mov,max + R ≤ 0.9 M and at least a 5 % text-error budget, in both modes.** The ceiling is 12 GiB, and the pricing expects 10.5 GiB.
- **R6a, after G5:** a provisional M.
- **R6b, after RV-Q's G6 review:** the final M, recorded as a ruling with the registration.
- **Otherwise:** §9's contingency.

### 3.3 G6, and what the registration re-pins `[r1: N-8]`

**G6 must show:**
- `admission_bound` at M − R − 1, M − R and M − R + 1;
- a pure `maximum` test in which each of the 7 phases is in turn the largest (W4 sits 215 MB below W3 at option S3's pricing);
- the 0.9 M rule in both modes;
- `registration.diff`, applied in a scratch copy, with PP's suite passing apart from the known Mac `t13`;
- **QUAL_B1.md,** in QUAL's form;
- the release build's profile record equal to the dev/test record (informational).

**SQ owns these re-pins, in the same reviewed change:**
- **`retained_memory_law_tests.rs`:**
  - `const M`;
  - `the_registered_profile_is_the_only_permit_source`'s `threshold_bytes` assertion;
  - the two `required <= 3_623_878_656` margin assertions, which become 0.9 M at the new M;
  - `profile_in_build_record`'s ratio denominator and `PINNED_RECORD`;
  - `challenge_bounds_are_the_profile`, extended to the challenge's new phase constants (§3.5);
  - SA's form-valued gate tests (§2.3).
- **The challenge file:** `W1_PHASE_BYTES`, `MAX_PHASE_BYTES`, the new phase constants and `CAP_BYTES` (§3.5).

### 3.4 The S1 witnesses

They run at R/k = 4 MiB, in the dev/test build and, informationally, the release build. **Each witness has one test entry point per mode, run as its own process** `[r1: SF-5]`. They must show no overflow, abort or panic, with every outcome asserted:
- W1, W2, W2-deep, W3, W4, W7 and headroom: unchanged;
- W6: case C, Native;
- W2b's replacement (from SW), or option (c);
- the `NoTriggeredCase` pins;
- **W-C2:** a successor, and again at R/64 = 1 MiB as headroom;
- **SW item 2's publishing cap-maximal input,** if one exists: a successor;
- **the cap-maximal three-case input** (SW item 3's components; |A| = 3):
  - every provenance escaped, with a raw value of depth 16;
  - **an assertion, through `assess`, that the input is inside D1,** because the private driver skips admission `[r1: N-7]`;
  - the outcome asserted (Preparation or Candidate fallbacks are acceptable);
- **the deepest call chain,** re-derived on B1's call graph. I expect 40 frames.

### 3.5 The challenge (`PP-tests/retained_memory_challenge.rs`)

The binary's counting allocator measures requested heap, which is like-for-like with E_mov,max `[r1: SF-5]`. Changes:
- **`CAP_BYTES`** (the abort cap; 6 GiB today) is raised to 16 GiB. That is above the regenerated E_mov,max and within T3's host allowance of up to 64 GiB.
- **The bound is chosen by the furthest phase reached,** not only by `successor().is_some()`. The outcome maps to its phase:

  | Outcome | Phase |
  |---|---|
  | Preparation, Native or Candidate | W2 |
  | Staging or Serializer | W3 |
  | Precommit | W4 |
  | A successor | E_mov,max |

  The phase constants are pinned to the profile by `challenge_bounds_are_the_profile`. A permitted run's outcome is read from the public surface: a successor, or the notice's detail and count. It is printed beside the peak.
- **Inputs, one test entry point per mode:**
  - the milestone;
  - W-C2 (Direct, registered);
  - the c = 1 cap-maximal inputs: W2b's replacement and SW item 2's publishing input, if any;
  - the cap-maximal three-case input.
- **It must show:** each peak ≤ its phase bound, and the milestone's peak still within bound. Today the milestone peaks at 3,541,898 B sparse and 2,252,863 B dense.

### 3.6 Peak resident memory and run time

This answers RR "Owner decision: M's practical limit is 12 GiB…" `[r1: SF-5]`.

**Build:**
- the registered dev/test PP test binaries, built through `WT/tools/t3_cargo.sh` in a fresh target;
- the release binaries (`--release`). The release build is Stale at admission, so it is measured through the private driver's witness tests.

**Run:**
- one process per measurement, **and one mode per process:** `/usr/bin/time -l <binary> <test> --exact --ignored --test-threads=1 --nocapture`;
- the binary runs directly, not through cargo;
- under `lockf -k WT/guard/cargo_job.lock`, with the memory guard up and no other heavy job.

**Record, for each run:**
- from `time -l`: the maximum resident set size and the peak memory footprint; real, user and sys seconds;
- **the counting-allocator peak.** The dev/test measurements run the challenge binary itself, so RSS and the requested-heap peak come from the same process;
- the test's printed `Instant` timings for the ordinary run and for W1's phases;
- **the outcome, and the furthest W1 phase reached** (the mapping in §3.5).

**Inputs, both modes:**
- the c = 1 cap-maximal inputs: W2b's replacement, SW item 2's publishing input, and W2 (Preparation);
- the cap-maximal three-case input;
- W-C2.
- **Controls:**
  - the milestone's W1;
  - the same inputs on the ordinary value route only (W1's increment);
  - the process floor.

**Repetitions:** 3, reporting the median and the maximum.

**The output, `RSS_TIME.md`.** It sets the measured peaks beside:
- option S3's priced E_mov,max (9,680,616,814 B = 9.02 GiB; ADD §3);
- the furthest phase each run reached;
- 16 GB and 32 GB.

**It states:**
- **phase coverage:** if no cap-maximal input reached W3–W5, it says plainly that **W3–W5 are unmeasured at the caps**, and are measured only on W-C2 and the milestone;
- **host:** this host has 128 GB and runs under no memory pressure. Behaviour on a 16 GB machine (compression, swap) is **not observed**. The 16 GB judgement rests on the measured peak and run time;
- **build:** debug-build times are pessimistic, and the release times are the ones B7 and B8 will read;
- **RSS:** macOS RSS includes shared pages, so the peak footprint is recorded beside it;
- **non-claims:** no supported-machine statement, which is owner-held.

**Cost:** about 2–3.5 h of machine time, with per-mode processes and the added inputs.

**Then R9** (§4): ROOT reads RSS_TIME.md against the owner's direction (target 32 GB; floor 16 GB "solving within practical timeframes") and reports it to the owner `[r1: SF-5, N-15]`.

### 3.7 QUAL §11's carry

Re-run RV87's by-type non-candidate sweep (`noncand_compare.py`) over B1's new non-candidates, reviewed by type by RV-Q. The explicit-row rule waits for B2's planning.

### 3.8 Pass B

SQ is B1's Pass A. Pass B runs on PR-B1's head with I65's fail-closed `g7_pass.sh`, retargeted as I72 did. **It must show:**
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

### 3.9 The full 40-manifest suite and the src-tauri suite, before the freeze `[r1: SF-6]`

ROOT runs, on the integrated candidate in NUM:
- **DEC-025:** expected deltas are added PP, `result_export`, pytest and vitest tests; the runner's re-based test at the same count; T6S's suites unchanged.
- **The src-tauri suite** (U9 G7's method, RR "U9 gates G7 and G8 pass…"). `P/apps/desktop/src-tauri` builds PP and is not among DEC-025's 40 manifests. The method:
  - `git archive` copies of the base (current main) and the candidate;
  - `cargo test --offline --locked --no-fail-fast` in each, under the lock;
  - per-test outcomes identical. #1082 had 116 against 116; today's count is whatever the base gives.

### 3.10 The Direct-entry gates (SG)

These use I61's `u9_g8_01` method, on the candidate with the registration applied:
- **Pressure:** refused at D1.5, with exact bytes.
- **Coexistence:** n05 and n06, and a multi-case input (the SP pin's input). Exact bytes, no notice.
- **The u3g2 sweep, registered and Stale:**
  - Stale is byte-identical to base;
  - every registered difference is listed row by row and explained (T-4, D1.4's widening or T-12);
  - an unexplained row is a stop.
- **Callers:** a scan for callers of the retained entries.

### 3.11 T9 and the both-entry gate, part 1 (ROOT)

The expected result is byte-identical. They run because `PP/lib.rs`'s ordinary-run bodies host the observer call sites. Part 2 runs only if Pass B classifies a hunk as live on the observer-free route.

### 3.12 T6S consistency

The c = 1 successors are unchanged, so T6S-2's goldens are not regenerated. The T6S suites run in the full suite. A changed disclosure meaning goes back to ROOT.

## 4. Order, worktrees and ROOT's ruling points `[r1: SF-1, N-9, N-15]`

**Host rules:**
- one heavy job at a time (`WT/guard/cargo_job.lock`);
- **at most three implementers at once,** plus reviewers (RR "RV98 confirms U8's Pass B; operating adjustments…", item 4). A repair round takes its lane's slot back, and ROOT schedules repairs ahead of new starts.

**The integration points.** ROOT merges with `--no-ff` on the B1 branch only; nothing reaches NUM before step 8.
- **I1:** ST, its repairs and RV-P round 1's confirmation, committed on `b1`.
- **I1′:** `b1` absorbs main once B6 has merged. If B6 merges before I1, I1 and I1′ coincide.
- **I2:** SA (`b1-a`) merged into `b1`, after RV-Q round 1.
- **I3:** SR-RS (`b1-r`) merged into `b1`, after RV-R. SP then pins W-C2.
- **I4:** SR-PY and SR-TS (`b1-p`, `b1-t`) merged, after RV-R.
- **I5:** the code freeze. SP's round-2 repairs are confirmed and SC is merged. Only SQ's registration and re-pins follow.

**The phases.** Implementers are named per phase; there are never more than three at once.

1. **Phase 0, now.**
   - RV107 confirms PLAN_v2.
   - I83 returns B6. **B6's PR:** a compact cut, review, the full suite, CI with dispatch, GEN-8, an exact-head DEC-025, and the merge (§6).
2. **Phase 1. Implementers: I-P (ST), I-W (SW).**
   - ST in `WT/b1`, including the seam. RV-P round 1, repairs and confirmation, then **I1**.
   - SW from the archive.
   - **I1′** when B6 is on main.
3. **Phase 2. Implementers: I-P (SP), I-A (SA), I-RS (SR-RS).**
   - SP in `WT/b1`, with the checkpoint after T-7 (**R3′**).
   - SA in `WT/b1-a`, forked from I1. RV-Q round 1 reviews its expressions, then **I2**. SA's check of RetainedErrorTextBytes against SP's producer waits for SP's completion (phase 4).
   - SR-RS in `WT/b1-r`, forked from I1′. It starts with its census (R5). Then RV-R, then **I3**.
4. **Phase 3. Implementers: I-P (SP), plus I-PY and I-TS as I-A's and I-RS's slots free.**
   - SP continues. After I3, SP pins W-C2's successor and fixtures.
   - **SR-PY** (`WT/b1-p`, from I1′) starts when SA returns. **SR-TS** (`WT/b1-t`, from I1′) starts when SR-RS returns. Each starts with its census.
   - RV-R reviews SR-PY and SR-TS, then **I4**.
   - **The SR chain is near-critical** `[r1: N-9]`. It runs from B6's PR gates through SR-RS, RV-R and I3 to SP's pins. ROOT starts B6's PR first for this reason.
5. **Phase 4. Implementers: I-A (G5-early), I-PY (SC), then I-RS and I-TS (pins).**
   - When SP is complete: **RV-P round 2**, repairs and confirmation. W-C2's bytes are then settled.
   - **G5-early** (I-A) runs on `b1`, informational.
   - **SC** starts only after RV-P round 2's repairs (N-9). I-PY writes 07n. Once 07n is written, I-RS and I-TS update their pins together; with I-A that is three implementers.
   - RV-R reviews SC, then **I5**.
6. **Phase 5. Implementer: I-A (SQ).** **Every repair and confirmation lands before G5-final.**
   - G5-final → **R6a** → G6, witnesses, challenge, peak RSS and time, and the non-candidate sweep → RV-Q, with its repair rounds → **R6b**.
   - ROOT applies the registration.
   - **R9:** ROOT reads RSS_TIME.md against the target-machine direction and reports it to the owner.
7. **Phase 6. Implementer: I-G (SG).**
   - `b1` merges into NUM, with no other unmerged product slice there.
   - ROOT runs **the full suite and the src-tauri suite**, and T9 and both-entry part 1.
   - SG runs the Direct-entry gates.
8. **Phase 7. Implementers: I-A or a Pass B holder (SB), I-K (SK).**
   - PR-B1 is cut. SB runs Pass B, and RV-Q confirms it. SK writes the package.
   - **The fresh complete-diff reviewer (RV-X)** reviews the whole PR, with RV-P's ledger as input. Repairs follow, confirmed by the same reviewer.
   - CI and dispatch, GEN-8, an exact-head DEC-025 and the src-tauri suite on the head (§6). Then **R7**, and the merge.

**The critical path:** ST → RV-P round 1 → SP → RV-P round 2 and its repairs → SQ → RV-Q and its repairs → integration and the full suite → PR-B1's cut, SB and RV-X with its repairs → the PR gates.
- That is about **44–66 h of serial agent work** (ST 5–7, SP 16–25, SQ 17–25, SB 2–3, plus repairs about 4–6).
- Serial review is about **25–41 h** (RV-P rounds 1 and 2 with confirmations, 10–16 h; RV-Q on SQ with its rounds, 9–15 h; RV-X with its confirmation, 6–10 h).
- Machine time is about 4–6 h.
- **Roughly 8–12 working sessions.**

**ROOT's ruling points:**
- **R1:** done. R2 (the cap ruling) is retired, because option S3 is selected.
- **R3:** ST's checkpoint, before I1.
- **R3′:** SP's checkpoint after T-7: re-estimate, and possibly split the serializer.
- **R4:** SW's stop rule, if it fires.
- **R5:** a census change in any SR lane.
- **R6a and R6b:** M.
- **R7:** T9 and both-entry applicability, the freeze, the carry-over if main moves, and the merge.
- **R8, any stop:**
  - an FK, schema, base-reader, reviewed-input or D1 visibility change;
  - a c = 1 byte change;
  - an unexplained sweep row;
  - a batch outcome differing from PROBE's (N-16);
  - §9's contingency.
- **R9 [new]:** after RV-Q passes SQ and before PR-B1 merges, ROOT reads RSS_TIME.md against the owner's target-machine direction and reports it to the owner `[r1: SF-5, N-15]`. The formal supported-machine statement stays owner-held.

## 5. Reviews (fresh IDs; ROOT assigns them)

**RV-P** (RV93's role). Estimate: rounds 1 and 2: 8–12 h; confirmations over two rounds: 2–4 h; the ledger: 2–3 h.
- **Scope:** ST (round 1) and SP (round 2), with the same reviewer confirming each repair. Also the PR-head ledger (every hunk maps to a reviewed commit) and SG's sweep delta.
- **Oracles:**
  - DESIGN T-1 to T-13 and its outcome table; C1, C2 and C3;
  - **its own probe on the B1 head:** case C alone, two-body B, and W6's and W2b's inputs, in both modes, against PROBE §2–§4;
  - the c = 1 pins, byte for byte;
  - **W-C2's receipt re-derived:** snapshots, `charged`, `execution_order`, group and build sharing, and the **ordinal-to-request mapping** (N-2);
  - the readers on W-C2;
  - s11f's new `TABLE` rows and their dispositions (SF-3);
  - `bind_observations`' envelope binding under decision 19;
  - **mutants:** the T-4 key, the id lookup (N-3), decision 21, `NoTriggeredCase`, T-5's count, T-12's placement, decision 5's set, and the domain re-check (SF-2).

**RV-R** (RV78's role). Estimate: 6–9 h, plus 2–3 h of confirmations.
- **Scope:** SR's three lanes and SC.
- **Oracles:**
  - DESIGN §2–§3;
  - **its own cascade census;**
  - its own reseal of `d38_beside_selected`, byte-compared with SC's;
  - the full 07n in three languages, with first-failure parity;
  - **the three readers' D38 audits** (N-6);
  - F-1 text B's disclosed limit.

**RV-Q** (RV89's and RV87's roles; it may be split in two). Estimate: SA 2–3 h; SQ 7–11 h; SB's confirmation 1–2 h; two repair confirmations 2–4 h.
- **Scope:** SA's expressions (round 1); SQ (G5, G6, the witnesses, the challenge, RSS and time, the non-candidate sweep, the re-pins); SB's confirmation.
- **Oracles:**
  - I82's `b1_eval.py`, as an independent evaluator;
  - I82's emulated numbers (ADD §1), with **the real loop multiplicities against I82's 22 rebinds and edges** (N-16);
  - the forms regenerated byte for byte;
  - the boundary tests at M;
  - the witness logs for each identity;
  - the RSS method, its controls, and the phase-coverage statement (SF-5).

**RV-X** (RV95's role), **new and fresh: PR-B1's complete-diff reviewer** `[r1: SF-7, D16]`. Estimate: 5–8 h, plus a confirmation of 1–2 h.
- **Scope:** the whole PR-B1 diff against main, as RV95 reviewed #1082. This is in addition to the slice reviews, with the same reviewer confirming each repair.
- **Oracles:**
  - RV-P's ledger, as input;
  - DESIGN, C1–C3 and the gate records;
  - its own reading of every hunk.

**Decision 16 is overruled.** Gate item 2 is met by RV-X.

**RV107** confirms this revision, outside B1's own budget.

## 6. Packaging

**B6 goes first, as its own compact product PR (ruled at R1)** `[r1: D15, N-14]`.

**B6's actual change set,** at `a79dbd2e4a` against `bfb26596bf`: 11 files, none a D1 crate `src`. B6's package lists them:
- `apps/desktop/src/features/results/retainedPrecision.ts`;
- `retainedPrecision.test.ts`;
- `retainedPrecisionIntegration.test.tsx`;
- `core/analysis_runs/compatibility.py`;
- `core/analysis_runs/retained_precision.py`;
- `RE/tests/retained_precision_carriers.rs`;
- `RE/tests/retained_precision_contract.rs`;
- `fixtures/results/retained_precision_carrier_cases.json`;
- `fixtures/results/retained_precision_cases.json` (07m, `c21112fd…`);
- `tests/test_retained_precision_carriers.py`;
- `tests/test_retained_precision_contract.py`.

B6's brief said its fixtures would be unchanged, but two fixtures changed. That is for B6's reviewer.

**B6's gates:** the product set without item 7. That is the compact cut, source equality, citations and the package (item 1); a fresh review; the full suite before the freeze; CI with dispatch; GEN-8; and an exact-head DEC-025. B6 touches no PP `src`, so the src-tauri suite is not needed for it.

**Sequencing:**
1. SI1b (#1106) merges.
2. NUM absorbs main.
3. B6 merges into NUM, and its PR is cut, gated and merged.
4. NUM absorbs main.
5. `b1` absorbs main (I1′).

**PR-B1's gate set** (RR "T3's gate set and Git rules…", items 1–7, as amended at R1):
1. A compact cut from main; `source_equality.py` against NUM; `check_citations.py`; the package (SK).
2. **A fresh, independent complete-diff review of the whole PR (RV-X), with the same reviewer confirming each repair** `[r1: SF-7]`. The slice reviews (RV-P, RV-R, RV-Q) and RV-P's ledger come in addition.
3. **The full 40-manifest suite before the freeze, and the src-tauri suite** (§3.9) `[r1: SF-6]`.
4. Hosted CI and the full-SHA dispatch.
5. GEN-8 on the exact head (E-4's method).
6. An exact-final-head Mac DEC-025 against a fresh main baseline (`run_dec025.sh`; counted only at `ALL-DONE`; `compare_suites.py`), **and the src-tauri suite on the exact head against the same main** `[r1: SF-6]`.
7. Because the D1 call graph and the registered profile are touched: Pass B with RV-Q's confirmation (§3.8); T9 and both-entry part 1 (§3.11); the Direct-entry gates (SG).

**B1's evidence in the package:**
- QUAL_B1.md, with M's ruling and the text-error budget;
- the generator and the profile tree;
- `registration.diff`;
- the witness logs;
- the challenge;
- **RSS_TIME.md, with R9's ruling;**
- 07n's parity;
- the W-C2 pins;
- RV95 N-5's direct test;
- RV97 R2-N-2;
- **the src-tauri comparison.**

**Merge:** `gh pr merge --merge --match-head-commit`, after confirming main has not moved.

## 7. Decisions, as ruled at R1

ROOT selected decisions 1–14 and 17–21 as amended, ruled decision 15, and overruled decision 16. Decisions 22–26 are new in this revision, for RV107's confirmation and ROOT. None is on the work graph's owner-held list.

| # | Decision | As ruled or proposed | Decider |
|---|---|---|---|
| 1 | Slices, owners, worktrees and lanes | **Amended (SF-1):** at most three implementers at once. Each concurrent PP- or RE-building lane has its own worktree and branch. ST precedes SA. Integration is at I1–I5 (§4) | ROOT (ruled) |
| 2 | Admission at option S3 | One cap table; `LOAD_CASES` = 3; a stated `TOTAL_LOADS` = 384; G-B and G-C at C = 3; the pricing functions unchanged in form | ROOT (ruled) |
| 3 | The out-of-domain oracles | **Amended (SF-2):** seven tests plus the D1.4 line. Inside the crate, `LOAD_CASES + 1`. In the runner, a literal 4 with a PP-side tie test. No visibility change | ROOT (ruled) |
| 4 | W2b's replacement probed now (SW) | **Amended (SF-5, N-7):** SW also looks for a publishing cap-maximal input, and the three cases carry the escaping and depth stresses | ROOT (ruled) |
| 5 | W2b's committed witness renamed to what it pins | As proposed | ROOT (ruled) |
| 6 | T-3 (e)'s fact | **Amended (SF-4):** `CompleteFacts.requested_cases`, set in `permitted_run` (ST's seam); SA's predicate uses it | ROOT (ruled) |
| 7 | W-C2's pin order | **Amended (N-1):** private driver first, asserting the precommit G5 fallback, with outcomes read through `before_precommit`; pinned after I3; through Direct after the registration | ROOT (ruled) |
| 8 | 07n: one writer; D38's base derived by a records script | **Amended (N-9):** SC starts after RV-P round 2 settles W-C2 | ROOT (ruled) |
| 9 | The cascade census as SR's precondition | As proposed | ROOT (ruled) |
| 10 | SQ as Pass A; Pass B against it | As proposed | ROOT (ruled) |
| 11 | RV87's sweep now; the explicit-row rule at B2 | As proposed | ROOT (ruled) |
| 12 | T9 and both-entry part 1; part 2 conditional | As proposed, with SF-6's src-tauri suite beside them | ROOT (ruled) |
| 13 | The registered sweep, every row explained | As proposed | ROOT (ruled) |
| 14 | The RSS and time method | **Amended (SF-5):** furthest phase recorded; the counting-allocator peak; `CAP_BYTES` raised; bounds by phase; one mode per process; the host statement; R9 | ROOT (ruled) |
| 15 | PR-B1's packaging | **Ruled at R1: B6 first, as its own compact PR** | ROOT (ruled) |
| 16 | The complete-diff review | **Overruled at R1 (SF-7):** a fresh complete-diff reviewer (RV-X) for the whole PR, in addition to the slice reviews | ROOT (ruled) |
| 17 | No B1 commit reaches NUM or main before SQ | As proposed | ROOT (ruled) |
| 18 | G5-early | As proposed, in phase 4, once SP is complete | ROOT (ruled) |
| 19 | I82's c² guidance in SP | As proposed; `bind_observations` keeps its envelope binding | ROOT (ruled) |
| 20 | M's rule | The smallest 256 MiB step ≤ 12 GiB with at least 5 % in both modes | ROOT (ruled); above 12 GiB, the owner |
| 21 | The B2 consequence | **Amended (N-11):** "likely; the combination's price is not yet computed"; B2's study prices it | ROOT (ruled) |
| 22 **[new]** | Where the SA–SP seam lands | **In ST, by I-P,** before SA forks: `ProductCapture.late_loads_total` and `CompleteFacts.requested_cases`, with the law tests' seven `CompleteFacts` literals given `requested_cases: 1`. Both fields are behaviour-neutral at c = 1 (§2.1) | ROOT |
| 23 **[new]** | The per-case preparation fault hook | **`fail_preparation_of_case(index)` in `grant2.rs`** (a proposed name; collision check by I-P), carried across the reserved-stack hop like the others | ROOT |
| 24 **[new]** | The challenge's abort cap and bounds | **`CAP_BYTES` = 16 GiB;** the bound chosen by the furthest phase reached, with the phase constants pinned to the profile | ROOT |
| 25 **[new]** | SW's publishing construction | **(c) milestone-body replication,** with unloaded restrained bodies filling the counts. Its stop leaves W3–W5 stated as unmeasured at the caps | ROOT |
| 26 **[new]** | The multi-case coexistence pin's input | **n05 with a second case added;** if it does not select, another W3 fixture; or return | ROOT |

**Carried, not re-decided:**
- DESIGN's selected decisions 1–16, 20 and 21;
- RR "I81's B1-0 probe verified…", rulings 1–4;
- RR "I82's addendum…";
- RR "R1: …";
- I74 decision 9;
- I77's renames;
- RV97 R2-N-2;
- the T6S consistency note.

**Owner-held, untouched:**
- M above 12 GiB;
- the formal supported-machine statement. B1 supplies RSS_TIME.md, which ROOT reports at R9;
- the dense and lane ceilings;
- PHYS-R4's named refusal and availability;
- observation framing (P5 deferred);
- KF3 and KF2;
- public meaning;
- the native-app witnesses;
- B8's R-2.

## 8. Risks and stop rules

1. **Memory.**
   - **The priced margins:** at 10.5 GiB, 399,134,558 B dense (9.53 %) and 458,265,902 B sparse (10.94 %). At 12 GiB, 1,848,686,021 B dense (44.1 % of TAV_W). These figures are emulated.
   - **The ladder at R6a:** raise M in 256 MiB steps up to 12 GiB.
   - **Stop:** no M ≤ 12 GiB holds at least 5 % → §9. G5-early gives warning in phase 4.
2. **W2b and the publishing input.**
   - **Stop:** SW's rules (§2.0) → R4.
   - If the three-case input does not reach native in every case, its asserted outcome stands, and §3.6 records the furthest phase.
3. **The re-pin cascade.** I expect no changes. **Stop:** any changed 07m outcome → R5.
4. **Stack depth.** I expect 40 frames. **Stop:** any overflow, abort or panic at R/16.
5. **c = 1 identity.** Any byte change is a stop.
6. **Producer scope.** If SP needs an FK, schema, reviewed-static, base-reader or D1-visibility change, it stops.
7. **The stale profile** (decision 17). Nothing leaves `b1` before SQ.
8. **Source-text guards** `[r1: SF-3]`: the full list is in §1. A changed expected text needs a reason. A new rule-8 site gets a reviewed `TABLE` row. A weakened assertion is a stop.
9. **Registered sweep surprises.** An unexplained row is a stop (SG).
10. **The validity of the RSS measurement** `[r1: SF-5]`:
    - phase coverage is stated;
    - one mode per process, three repetitions, the floor and value-route controls;
    - no memory pressure on the host, stated;
    - debug times are pessimistic, stated.
11. **NUM sequencing.** SI1b and B6 go first.
12. **SP's size.** The checkpoint R3′ re-estimates it. If SP runs over 25 h, ROOT may give T-11 (the serializer) to a second implementer, in its own worktree from the checkpoint, within the three-implementer cap.
13. **Batch outcomes** `[r1: N-16]`. A difference from PROBE's one-case outcomes inside one `CaseBatchCall` is returned as a finding, with its cause (R8).
14. **Integration conflicts** `[r1: SF-1]`. Lanes are split by file, so I1–I5 should merge cleanly. Any conflict outside the seam's two struct fields is a stop at that integration point, and is returned to ROOT.

## 9. The cap shape, and the contingency

**Selected:** option S3.
- **What depends on it:** SA's values and rows, SQ's single profile and form set, and the three-case cap-maximal witness at 32/32/32 with 128 loads per case.
- **What does not:** ST, SP, SR, SC and SG.
- **The committed witnesses** all stay in the domain, and W-C2 fits exactly.

**The contingency** (RR: if G5 on the real code exceeds 12 GiB at C = 3, ROOT returns to I82's options or to P1, and tells the owner):
- **A trimmed single tier at C = 3:** STUDY §3.2's C = 3 points, for example `t_c3_m12` and `t_c3_k20_l128` `[r1: N-12]`. ADD §2's trims are priced at C = 4.
  - SA changes values only.
  - W2 and W2b leave the domain, and are re-based by an SW rerun at the trimmed counts (2–3 h).
- **P1's two tiers:**
  - SA gains tier selection and per-tier bounds (+3–4 h);
  - SQ gains a second form set, and per-tier witnesses and measurements (+5–6 h);
  - review grows by +2–3 h;
  - W2 and W2b stay in the domain.
- **The trigger** is G5-early (phase 4), or at the latest R6a. Only SA's values and SQ are redone.

## 10. Estimates `[r1: N-10]`

**Slice work** (agent hours, without repairs): SW 4–7, ST 5–7, SP 16–25, SA 5–8, SR-RS 7–10, SR-PY 5–7, SR-TS 5–7, SC 6–8, SQ 17–25, SG 2–4, SB 2–3, SK 3–5. **Total: 77–116 h.**

**Repair rounds, calibrated against U4's G5–G7.** That work had two NOT CONFIRMED rounds from RV87, on S-2 in G5 part 2 and on the identifier audit in G6. RV89 also routed items into the next step three times, at G5 part 1, at G5 part 2, and at G6 with the registration. The profile lane therefore gets two rounds.

| Reviewer | Rounds | Agent | Review |
|---|---|---|---|
| RV-Q (SA, SQ) | 2 | 4–8 h | 2–4 h |
| RV-P (ST, SP) | 1–2 | 3–6 h | 2–4 h |
| RV-R (SR, SC) | 1 | 2–3 h | 1–2 h |
| RV-X (the PR) | 1 | 2–4 h | 1–2 h |
| **Total** | | **11–21 h** | **6–12 h** |

The review column is already inside the reviewer totals below; it is not added a second time.

| | Option S3 |
|---|---|
| **Agent** (slices plus repairs) | **88–137 h** |
| **Review** (RV-P 12–19, RV-R 8–12, RV-Q 12–20, RV-X 6–10, with confirmations) | **38–61 h** |
| **ROOT:** rulings, five integration points, verification, registration, R9, gates, merge; and B6's PR | 10–15 h |
| **Machine:** the full suite and src-tauri (candidate and head, each against main), T9, both-entry part 1, peak RSS and time | 6–9 h |
| **Elapsed** | About 8–12 working sessions, with B6's PR at the start |
| **If §9's contingency fires with P1** | +8–10 h agent, +2–3 h review |

**Against revision 0** (67–102 h agent, 26–41 h review), the increase comes from:
- the repair rounds (N-10);
- RV-X (SF-7);
- the measurement changes (SF-5);
- the seam, the oracles, the guards and the hooks (SF-2 to SF-4, N-4, N-5);
- the per-reader D38 audits (N-6);
- three-implementer sequencing, which lengthens elapsed time (SF-1).

**Against DESIGN §9** (37–59 h, 12–17 h), the main cause is unchanged: SP's single-case refactor.

**A calibration caveat** (N-10). U8 and T6S each went from dispatch to merge in about one wall day, against planned 1.5–2.5 sessions and 3–5 days. B1 is larger, and is on the D1 call graph with a re-registration, as U4 was. These remain reading estimates.

## 11. Effects on the breadth order and the owner's F2a order

**Neither order changes.** For ROOT to weigh:
1. **B6 merges before PR-B1** (ruled). This is a packaging change to PLAN decision 8.
2. **B2's room under 12 GiB** (likely; not yet priced). B2's plan needs a reduced tier or a lower C for combinations, or the owner. B4 faces the same ceiling.
3. **B1's estimate grows** (§10). This moves the schedule, not the order.
4. **B7 inherits §3.6's method,** and R9's report to the owner feeds B7's and B8's supported-machine statement.

## 12. What I read for this revision, and limits

**Read** (sha256):

| Input | sha256 |
|---|---|
| RV107's REVIEW, with `evidence/checks.txt` and `raw_census_s3.*` cited by it | `03c3111d…a426` |
| RR "I84's B1 plan returned…" and "R1: B1's plan ruled with seven amendments…", at NUM `ef43a1d694` | — |
| RR "RV98 confirms U8's Pass B; operating adjustments…" (item 4) and "U9 gates G7 and G8 pass…" | — |
| RR's U4 G2–G7 section headings and verdicts (RR:8865–10836), for N-10's calibration | — |
| ADD (whole) | `7c155ceb…0ce2` |
| PLAN.md (revision 0, unchanged) | `7f9699f3…4bb5c` |

**Code read at main `47a3bdfcf5`** (unchanged at NUM `ef43a1d694` outside `P/execution`), in addition to revision 0's list:
- `retained_facade_tests.rs`: `u3g2_no_permit_path_runs_once_without_a_copy`;
- `retained_memory_law_tests.rs`: `const M`, the threshold and margin assertions, `every_family_clause_refuses_with_its_fact`, `profile_in_build_record`, `challenge_bounds_are_the_profile`, and its seven `CompleteFacts { … }` literals;
- `retained_memory.rs`: `LateFacts` and `CompleteFacts`;
- `retained_tests_hooks/grant2.rs`: its hook list;
- `retained_wire_tests.rs`: `u1_serializer_reads_no_legacy_work_field`;
- PP-tests: `retained_memory_challenge.rs` (its constants, counting allocator and bound selection); `s11f_site_test.rs` (rules 1–8, `TABLE`, `RULE8_FILES`); `retained_precision_admission.rs` (its `lib.rs` section guard).

**B6:** `git diff --name-only bfb26596bf a79dbd2e4a`, and 07m's blob at both heads (`47d8b327…`, sha256 `c21112fd…6807`).

**Limits:**
- **Nothing was built or run.** No new computation; `_run_records/` is unchanged.
- **Unestablished until their slices run:**
  - whether a publishing cap-maximal input exists (SW item 2);
  - whether n05 with a second case selects exact-block (SP's coexistence pin);
  - the batch outcomes (N-16).
- **Option S3's figures are emulated.** G5 supersedes them.
- **The estimates are reading estimates,** with repair rounds calibrated against U4's records only.

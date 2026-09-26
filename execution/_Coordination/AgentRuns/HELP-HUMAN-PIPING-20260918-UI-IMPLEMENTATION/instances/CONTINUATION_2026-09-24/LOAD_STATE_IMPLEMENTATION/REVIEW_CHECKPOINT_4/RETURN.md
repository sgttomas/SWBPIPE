# Independent review: load/reference-state checkpoint 4

**Verdict: FINDINGS.** One should-fix and four notes. Nothing blocking.

I found no route by which a 0.4.0 invocation still loses its ordinary results when a selected join cannot finalize. Every probe I built publishes `load-reference-1` with rows bit-identical to the uncaptured ordinary route. That includes public-limit budget failures, a composite failure, invocation-limit exhaustion and an injected derived-row failure. The fallback publishes no stale selected state, no receipt and no joined record. It cannot recurse. Pre-0.4 output is byte-identical to the base on 106 pre-0.4 runs.

The should-fix is about the replay reservation, not lost results. The reservation does not do what ROOT's selection and the records say. A case can pass it at the public limit and still fail captured replay. Replay can also cost more than the live charge. The fallback catches both, so the published outcome is still ordinary. But "a case is selected only when replay can complete" and "replay is bounded by the live charge" are both false.

## Scope and identity

- **Reviewer.** A Type 2 TASK acting as a fresh-context, non-author reviewer, spawned by HELP_HUMAN (ROOT) at the request of the T1 WORKING_ITEMS manager. I wrote none of the reviewed bytes, delegated nothing and made no Git writes.
- **Candidate.** `codex/piping-load-states-20260925` at `14a74b793`, which was HEAD when I started and when I finished. I reviewed the frozen diff `a68326039..14a74b793`: 9 commits and 80 files, listed with sha256 in `_run_records/prehash_worktree.txt`. The posthash is equal.
- **Basis.**
  - `REVIEW_CHECKPOINT_3/RETURN.md` and `ROOT_DISPOSITION.md`;
  - ROOT's later confirmation of the SF-1 approach, as given in my brief;
  - `CHECKPOINT_4.md` §2 (the N-2 readers disposition);
  - `CP4_WIRE_ADDENDUM.md`;
  - the returns in `CP4_JOIN_TESTS/` and `CP4_READERS/`.
- **Paths.** Paths are relative to WORKING_ROOT = `projects/chirality-piping/`. `LSI` is the LOAD_STATE_IMPLEMENTATION instance folder. Line numbers are at `14a74b793`. Commands and logs are in `_run_records/`, indexed by `RUNS.md`.

## Findings

| # | Severity | Location | Concrete failure scenario | Suggested repair |
|---|---|---|---|---|
| SF-1R | should-fix | `core/product_physics/src/source_recovery.rs:190–213`: `reserve_captured_replay` and its doc comment ("bounded by the live charge").<br>`core/product_physics/src/lib.rs:2372–2375`, with the comment "a case is selected only when that replay fits".<br>`LSI/CHECKPOINT_4.md:25` ("Replay is therefore bounded by the live charge").<br>`LSI/CP4_WIRE_ADDENDUM.md:30–44` (§1.1).<br>`ROOT_DISPOSITION.md` SF-1 bullet 2 ("so a case is selected only when replay can complete"). | **The reservation `c ≤ L − c` does not guarantee that captured replay completes**, for two reasons.<br>**(i) Other charges come first.** Derived recipes and the finalization reservation are debited from the same ledger after selection and *before* replay: 432k–549k on every input I measured. §1.1 calls these "later finalization stages", but they run earlier.<br>*Public-limit witness:* the committed `eigen_motion` witness plus one declared 1e-6 N·m tip `RY` moment.<br>• The live charge is c = 3,992,928, which passes the reservation (L = 8M).<br>• The pre-replay charges are 521,510.<br>• Captured replay then fails with `Exact(Budget)` (`charged 7,999,978, rejected 43`). Completing it would need 8,507,226.<br>• The fallback then republishes ordinarily, with "invocation join withheld … case case:join: captured source replay …".<br>Both modes behave this way (`probes_public.log`, "RY@tip"; `probes_replay_measure.log`).<br>**(ii) Replay can exceed the live charge.** On the committed `fields` witness, replay costs 3,196,145 against a live charge of 3,196,032 (+113), in both modes. On every other input I measured, replay was 105–203 below the live charge. The "therefore" rests on 4 inputs of one witness.<br>**Consequence.** No results are lost: §1.2 catches both cases. But the published cause in the band c ∈ (≈3.74M, 4.0M] is a withheld join rather than the reservation refusal. The invocation executes about twice the work. ROOT's selection text and three records overstate the guarantee.<br>**Tests.** The committed scenario-(a) test uses UZ/UY/RY loads (c ≥ 4.06M), which are always declined by the reservation. The post-reservation fallback is pinned only under a private 7.0M limit. | Have ROOT choose one of the following. Either way, add a public-limit test pinning the `RY@tip` input, and correct `CHECKPOINT_4` §1, ADDENDUM §1.1 and the two code comments in a new record, not by rewriting.<br>• **(a) Strengthen the reservation.** Reserve at least c plus the pre-replay stages, with a margin for replay exceeding c. The finalization reservation formula (`composite.rs:739–742`) and the derived-recipe count are computable from the selected response's rows and supports.<br>• **(b) Accept a screen.** Record the reservation as a screen that makes most budget cliffs a direct `unavailable`, with the §1.2 fallback as the only guarantee. |
| N-1 | note | `LSI/_run_records/session3/cp4_sf1_mutations.{py,log}`<br>`LSI/CHECKPOINT_4.md:37–43` | **The SF-1 mutant record was produced on pre-final bytes.** The script asserts each anchor occurs exactly once. The anchors of SF1-M3, M4 and M5 are single-line strings that occur 0 times in the committed `lib.rs`, because rustfmt split those lines. So the logged run, including "all restored and sha256-verified", cannot have run on `6235f6b43` or later. This is the same class as CP3 N-4, and it is not disclosed. I re-anchored all six to the frozen bytes and reran them: all six are killed by `load_state_fallback_tests` (`mutations.log`). | In the next record, state that the logged run used pre-rustfmt bytes, and cite the rerun. No code change is needed. |
| N-2 | note | `lib.rs:832–840` (`withholding_load_state_join`)<br>`LSI/CP4_WIRE_ADDENDUM.md:55` ("Executed work is not refunded") | **The invocation limit no longer bounds executed work for a captured 0.4.0 invocation.** The republication gets a fresh ledger with the full limits. Probe: ten copies of the witness exhaust the first run's 64M invocation budget (cause "invocation publication reservation"). The fallback then charges another 33.6M, re-running every attempt only to decline it. Publication is correct, with 900 or 910 rows and no blocking diagnostic, so this matters only as a resource statement. | Record the bound as about 2× the invocation limit. Alternatively, ROOT may prefer that the republication skip re-running retained-source attempts and reuse the first run's attempt facts. |
| N-3 | note | `lib.rs:2369–2371` and `:2381–2384`<br>`LSI/CP4_WIRE_ADDENDUM.md:58` | **The fallback can mask a case's own refusal.** In the republication, every successful attempt is declined as "invocation join withheld" *before* the reservation is checked. Probe: case A is the selectable witness; case B is the witness plus a UZ load, with c = 4,063,366, which fails its own reservation in the first run. After the fallback, B's published diagnostic says its join was withheld because of the invocation, with no reservation refusal. The rows are correct. This matches the addendum's wording ("any successful attempt is declined"), so it is a truthfulness nuance, not a contract breach. | Optional: in the republication, check the reservation before the withheld branch, so that a case's own refusal is published. Or record that the withheld stage takes precedence. |
| N-4 | note | `LSI/CHECKPOINT_4.md:98`<br>commit `ad6da6880`<br>`LSI/TASK_BRIEFS/CP4_REVIEW.md:46`<br>`LSI/_run_records/session3/cp4_integrated_python_named.log` | **Small record inaccuracies.**<br>• §3 says the path edit changed `CP4_READERS.md` and `CP4_JOIN_TESTS.md`. `ad6da6880` also edited `CP4_REVIEW.md`, which was not yet dispatched.<br>• The placeholder substitution left garbled text: "the venv \`the session DEC-025 venv (location given in the spawn request)\`".<br>• The integrated pytest log records neither the command nor the file list behind "957 passed". I reproduced 957/1 with the five files named in RUNS.md, and I assume this is the same set. | Correct these in the next record. |

## What I verified, and how

### 1. Suites (brief "Checks")

| Check | Result | Log |
|---|---|---|
| product_physics | 415 passed, 1 ignored; lib 324 | `product_physics_all_tests.log` |
| result_export | 64/64 | `result_export_tests.log` |
| the five named pytest files | 957 passed, 1 skipped | `pytest_named.log` |
| `cp4_regen_compare.py` | 34/34, byte-identical to the committed log | `regen_compare.log` |
| rustfmt 1.8.0-stable | 0 hunks on the three result_export files, `source_recovery.rs` and both new test modules. `lib.rs` own hunks go from 47 (at `a68326039`) to 48, as claimed | `rustfmt_check.log` |

### 2. SF-1 (brief item 1): code reading, probes (`review4_probes.rs.txt`) and mutants

**Lost results.** No route found.
- **Every path records its failure.** Both recording sites (`lib.rs:1956–1962`, `:3431–3436`) cover every selected-join failure path. The wrapper (`:1428–1450`) checks after *any* return of the first run, including early `blocked_envelope` returns.
- **The one uncovered blocking diagnostic.** The only selected-case blocking diagnostic not recorded directly is `SOURCE_BLOCK_RECOVERY_DERIVED_UNAVAILABLE`, from a failed support norm (`:2725–2732`). I injected that failure on a scratch copy. The missing support rows then make the per-case finalization fail ("source functional row binding is not unique"), which is recorded, so the invocation falls back to ordinary publication in both modes (`probes_support_norm_injection.log`). This is covered transitively, not directly.
- **Public-limit outcomes.** Every public-limit variant published `load-reference-1` with `MECHANICS_SOLVED` and quality `Sensitive`, in both modes. That covers 17 single and paired extra loads, the mixed reservation case and the ten-case exhaustion (`probes_public.log`). Where I compared rows (mixed reservation), they were bit-identical to the typed route.

**Only the ordinary route is published.**
- **A fresh run.** The republication is a fresh run from a clone of the typed request. The only carried state is the cause string and the two limits. `CapturedInvocation` holds no interior mutability, and the only thread-local is test-only.
- **Nothing can be selected.** With `load_state_join_withheld` set, no 0.4.0 case can be selected (`:2369`). So `source_selected` is false, and the label, profile, `source_block_recovery: None`, the `not_joined` records and the ordinary `recovery_method` follow from the unchanged ordinary path.
- **Mutants.** SF1-M3 (the withheld attempt still selects) and R3–R5 are killed.

**The reservation.** See SF-1R. At the committed witness (c = 3,360,029) it holds with margin, and `load-reference-source-1` is still selected and qualified.

**The ledger.**
- **No double debit.** Each attempt is debited once: a reservation refusal debits c, a withheld attempt debits its live work, and a finalization failure debits `error.1`.
- **Nothing published is refunded.** The first run's ledger is discarded, and nothing from it is published. The fresh ledger has the resource effect described in N-2.

**Pre-0.4 unchanged.**
- **By reading.** Every change is gated by `load_state` or `joined_load_state`, and the wrapper by `is_load_state`. The pre-0.4 `and_then` arm returns `Ok(recovery)`, and its diagnostic suffix is empty.
- **Differential run.** 120 value-route runs, 106 of them pre-0.4: every committed request in the four producer-fixture folders, plus invented pre-0.4 stress variants (1–3 extra tip loads, and P12). Stdout, exit code and stderr are identical between the base `a68326039` producer and the candidate (`pre04_differential_*.json`).
- **P12 unchanged.** The P12 `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")` is unchanged.
- **The pre-0.4 guards.** Removing both outer pre-0.4 guards (mutants R1, R2) survives every suite. They are output-equivalent: a pre-0.4 republication ignores the withheld flag, re-selects and reproduces the same bytes, so the guards only avoid a redundant run.

**No recursion.** The wrapper calls `run_linear_static_preview_captured_once` at most twice. It clones only when `load_state_join_withheld` is `None`, and the fallback ledger always has it set.

**Diagnostics.** They are truthful, except for SF-1R's attribution and N-3. The withheld suffix is appended to every attempt diagnostic in the republication, including the pressure case's own refusal. That statement is true: the invocation does publish ordinarily.

**Tests.**
- Both reviewer scenarios are pinned in both modes, with row bit-equality against the uncaptured route and record and diagnostic checks. My reruns kill all six manager mutants and R3, R4 and R5.
- Survivors other than R1 and R2:
  - R6 (strict `<`, a boundary case not exercised);
  - R7 (the first recorded cause against the last one).
- The gap is the public-limit post-reservation path (SF-1R).

### 3. N-1 join tests (brief item 2)

- **Independent expectations.** The expectations are closed forms: N = E·A·(δ/L − ε*), with ε* = (1+ε_th)(1+ε_fit) − 1, g + θ·L, 2.5·T/k and −k·g, compared at 1e-9 relative. The witness constants are asserted against the fixture, so they are not taken from the producer. P11 compares against the typed ordinary route at the CP3 reviewer's 1e-6 criterion for route comparison, plus closed forms. No protected criterion is weakened.
- **P5.** The test now requires the live closure to accept each perturbation, so each refusal belongs to captured replay alone.
- **K8 and K9, reproduced on frozen bytes** with the CP3 anchors verbatim (`mutations.log`). Each is killed by exactly its named test, both in the module and in the whole crate:
  - K8 by `a_primitive_load_colliding_…`;
  - K9 by `the_mixed_joined_envelope_binds_…`.
- **K5, K7 and K10.** I did not rerun them. The test bodies match the CP4_JOIN_TESTS kill table: evidence_only; the motion-ownership test and explicit_zero_motion; strain_split_same_total.
- **The K9 route mirror is faithful for its purpose.** `route_cases` follows the product's 0.4.0 loop:
  - validate, normalize, resolve every case;
  - build each case's pair model and stiffness;
  - `solve_load_case` with the value-route budget.

  It skips only product validators that do not affect the finalized cases. Its byte-equal reproduction of the published receipt, in both modes, guards against drift. The tampers are then checked by the product's own `validate_publication` and `finalize_composite`.

### 4. N-2 and N-3 readers (brief item 3)

- **Same rules, same order, same codes.** In both readers each segment is checked in this order:
  1. shape;
  2. `use`;
  3. two indices;
  4. two numbers;
  5. `upper == lower + 1` (Rust `checked_add`, which is equivalent at `u64::MAX`), then `start == end` or `start < end` → `LAW_SEGMENT`;
  6. duplicate within the list → `LAW_SEGMENT_DUPLICATE`.

  Duplicates compare by binary64 value, so `0.0` equals `-0.0` in both languages.
- **Number handling.** Python `_as_f64` is used in `_number`, `_num_eq` and `_same`, the fit binding and the factor. It matches serde_json `as_f64`: `as f64` for i64/u64 and the correctly rounded parse beyond them. `float_roundtrip` is enabled in `result_export/Cargo.toml`. `U64_MAX` now bounds indices only.
- **Independent parity check.** I appended 8 cases of my own to a *scratch* copy of the shared file. Their expectations come from my own exact round-to-nearest-even arithmetic. They cover:
  - `u64::MAX` against `u64::MAX + 1` (both 2^64);
  - ties to even, down and up, at 2^54 and beyond u64 at 2^65;
  - wrong-neighbour controls;
  - three integer spellings of one binary64.

  All 8 pass in both harnesses. A control with one deliberately wrong expectation fails in both, so the harnesses discriminate (`reader_boundary_cases.log`). The scratch file was restored and compared byte-equal to the candidate.
- **No divergence.** I found no case that diverges between the languages. The only language-specific cases are the declared overflow and non-finite ones.
- **No extra tightening.** There is none beyond the disposition. `[T_lower, T_upper]` is not bound, and integers above 2^53 are admitted by the readers and refused by the carriers, as ROOT decided.
- **Frozen fixtures.** The four frozen envelopes are still accepted: the pytest and result_export suites pass.

### 5. N-5 (brief item 4)

In `13f752115`, `load_reference.rs` and `load_reference_contract.rs` are token-equal to their parents apart from whitespace and trailing commas. `semantic_contract.rs` differs only by line breaks and the reorder of two `pub use` items. This is formatting only.

### 6. N-4 and the records (brief item 5)

- **Hashes match.** Every hash I checked matches:
  - the CP2/CP3 wire files and `ROOT_DISPOSITION.md` in the addendum table;
  - `load_state_join_tests.rs`;
  - `57f12bcb…` for `CP3_READERS/RETURN.md` and `767f0e99…` for `CP3_ROOT_SELECTIONS.json`, both at `485cc2ed0`.
- **Earlier records not rewritten.** The only pre-existing files modified in the range are the ten code and test files. Every record change is a new file, apart from the in-range TASK_BRIEFS path edit (N-4).
- **No machine paths.** I found no absolute machine path in any file added or changed in the range.
- **Claims.** The claims in CHECKPOINT_4 match the logs I reproduced, except for SF-1R and N-1. CHECKPOINT_4 §5 is still unfilled, as intended.

### 7. `LSI/T1_PLAN.md` (brief item 6, read only)

The plan's statements about current state hold where I checked them:
- 13 reference cases;
- no 0.4.0 fields in desktop `types.ts`;
- native migration refuses 0.4.0 with the quoted message;
- no 0.4.0 test in the headless runner;
- readers refuse the joined envelope.

It keeps the protected 1e-9 criterion and states no new tolerance. No constraint is misstated. One point to carry forward: WP4's "SF-1 fallback through the runner" and native witness 8 should also cover the post-reservation fallback (the SF-1R `RY` input), not only the budget-cliff input, which the reservation declines outright.

## Limits

- **Not run:**
  - the dependent crates (headless, operation_applier, self_weight_wasm); I read the manager's logs instead;
  - the full piping pytest sweep and the DEC-025 sweep;
  - desktop, native and browser lanes;
  - hosted CI;
  - the reader mutants of CP4_READERS, and K5, K7 and K10 (I reran K8 and K9 as the brief requires).
- **The support-norm route is argued, not reached.** Its coverage rests on one scratch fault injection. I found no public input that makes a support norm fail.
- **The replay measurements are a sample.** They cover the five committed load_reference_source witnesses, two cases and seven single-load variants. SF-1R's replay-above-live example is the committed `fields` witness.
- **Writes and cleanup.**
  - Probes, mutants and the base comparison ran only on a single `git archive` scratch copy. Its `product_physics/src` was replaced by `a68326039`'s for the differential run.
  - The checkout was never mutated. I wrote only inside `LSI/REVIEW_CHECKPOINT_4/`.
  - The scratch copy and the cargo target were deleted at the end.
- **Status.** This review closes no M10, M16 or M29 finding. It is not acceptance, and it does not activate `load-reference-source-1`.

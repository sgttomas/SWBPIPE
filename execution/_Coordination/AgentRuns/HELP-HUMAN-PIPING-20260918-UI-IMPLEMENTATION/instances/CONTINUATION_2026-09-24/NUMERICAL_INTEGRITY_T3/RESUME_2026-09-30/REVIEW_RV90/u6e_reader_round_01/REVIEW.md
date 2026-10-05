# RV90: independent review of U6e, the reader round (F5, RV79-N1, RV80-N2; snapshot 07g)

RV90 is a TASK (Type 2) dispatched directly by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path, and nothing was delegated.

- **Brief:** `R/BRIEFS/RV90_U6E_READER_ROUND_REVIEW.md`, at NUM `3f07f1d927`.
- **Candidate:** `5e1e2625ac` on `codex/piping-f2a-readers-round-20261004`.
- **Base:** `844448112f` (U6a).
- **Independence:** I did not write this code or the corpus. My oracles are my own derivations, applier, rehash, probes and outcome dumps. I used the author's tests and tables only as objects under review.

## Verdict

**PASS, with 1 SHOULD-FIX and 4 NOTEs. Nothing is BLOCKING.** Under D36, none of the findings gates the round. ROOT rules on S1.

- **F5 is A2's rule, implemented the same way in all three readers for well-formed envelopes.**
  - It uses the same gate and code (G5 `RETAINED_PRECISION_ATTEMPT_MISMATCH`) and runs at the same point: the class-2 ordinary pass, per case, after D6a's check (kept) and before the typed references.
  - The two real milestone receipts pass all three readers unchanged, with classes 25/69/3/1 and 25/69/3/2, standing `needs_recompute`, not eligible.
  - The producer's serializer builds the list with the same predicate over the final envelope. No legitimate producer output is refused.
- **The 15 base repairs are confirmed.** Only `ordinary_attempts[*].diagnostic_refs` and `retained_precision.receipt_sha256` changed. Every new list is A2's list by my own computation, and every repaired base reseals to itself under my own rehash.
- **My own D37 table matches the corpus `d37`.** I derived it from PP source and C3, and it agrees on all 9 kinds and all 25 records. Each reader's D37 test reads the corpus table: perturbing the corpus table fails exactly that test in all three readers. TypeScript's `errorStageRecordAgrees` is a verbatim move.
- **RV80-N2's claim holds as scoped.** In Rust, the whole-statement widening (I61's RS05) is killed only by the new in-crate test. TypeScript has no normalization. Python's scope is already observable through the corpus.
- **No unintended outcome change, in any of the three readers.**
  - The 07f readers on 07f against the 07g readers on 07g: 0 of 305 outcomes changed.
  - The 07f readers against the 07g readers on the 07g corpus: exactly the 6 new F5 mutations differ.
  - The two flipped tests flip only because of F5.
- **All suites pass on 07g.**
  - Python: 384 (372 contract and 12 schema).
  - Rust `result_export`: 163.
  - TypeScript: vitest 444, and `tsc` exits 0.
  - Every reader matches the corpus expectation on all 311 entries. The completeness flags are still false.
- **Mutants.** I61's 22 are all killed. I added 14 of my own, run 18 times.
  - 6 are killed, including both corpus-table perturbations in all three readers.
  - 8 survive:
    - 6 are real gaps (S1, N1, N2);
    - 1 is equivalent (D6a is now implied by F5);
    - 1 is a designed control (the D37 test fed from the reader's table cannot kill M35).

## Counts

**0 BLOCKING · 1 SHOULD-FIX · 4 NOTE**

| ID | Severity | Finding |
|---|---|---|
| S1 | SHOULD-FIX (D36: tracked, not gating) | Python's F5 membership test is a substring or key test on a non-array `affected_refs`. Rust and TypeScript treat a non-array as naming no case. The base readers agree at the gate on 6 such probes; the candidate readers diverge on all 6 (5 at the gate, 1 in the code). |
| N1 | NOTE (D36: test-only) | No shared entry pins F5 on any case but the first. "F5 for case 0 only" survives the full suite in all three readers, yet admits two second-case lists that every pristine reader refuses. |
| N2 | NOTE (D36: test-only) | No shared entry pins a strict-prefix list. TypeScript's "length check dropped" and Rust's `exact.starts_with(refs)` survive the suite, yet admit a strict prefix that every pristine reader refuses. |
| N3 | NOTE (test quality) | Rust's flipped D6a probe `[integrity, other]` is refused under F5 even without `other`, because it is also 07f's relaxed form. It does not isolate the other-scope element. |
| N4 | NOTE (record wording) | The corpus's `d37.basis` does not state the facade-order premise (`PP/lib.rs:2962–2974`) that excludes `capture` at `C---------`. The table is correct for the producer as it is. |

## 1. F5 against A2

**The rule (decision 2, A2, RR:8821).**
- Each case's ordinary `diagnostic_refs` lists exactly the diagnostics whose `affected_refs` name the case.
- Each is listed once, in envelope order.
- `RETAINED_PRECISION_*` diagnostics and the legacy disclosure omitted under T1 (a) are excluded.
- Invocation-level diagnostics are attributed to no case.

D-U6-7 makes this amend checkpoint A's D6a (RR:8117), which required only unique and resolving references.

| Reader | D6a, kept | F5 | Gate and code |
|---|---|---|---|
| Python | `P/core/analysis_runs/retained_precision.py:909` | `:914`, list equality with the filtered envelope ids | `fail` in `_g5_ordinary` gives G5 `ATTEMPT_MISMATCH` |
| Rust | `P/core/reporting/result_export/src/retained_precision.rs:4133–4135` | `:4140–4145`, `Vec<&Value>` equality | `fail` in `g5_ordinary` gives G5 `ATTEMPT_MISMATCH` |
| TypeScript | `P/apps/desktop/src/features/results/retainedPrecision.ts:1261` | `:1265–1266`, a length check plus element-wise `===` | `fail` in `ordinaryAttempts` gives G5 `ATTEMPT_MISMATCH` |

**Same point in all three readers.**
- Each reader runs `g5_native`, then the ordinary pass, then the product pass, so F5 runs in class 2, before product association (D17) and the typed (class 3) and work (class 4) checks.
- F5 sits inside the per-case loop, after D6a and before the typed references.
- Every check in that loop shares gate and code, so ordering inside the pass cannot change a first failure. The one exception is a Python exception fallback (see S1).

**D6a is kept, but it is now implied.**
- G4 already refuses duplicate diagnostic ids.
- So A2's list is unique and resolving, and any list equal to it passes D6a.
- Removing D6a is an equivalent mutant (RV90:PY03 survives). This is the ruled state, so no action is needed.

**The real milestone receipts.** I ran both U6a fixtures (`P/fixtures/results/retained_precision_milestone_successor_{sparse_interactive,dense_scrutiny}.json`, sha256 `ac6986b0…` and `6cd1d249…`) through all six readers: base and candidate, in each of the three languages.
- **The receipts as committed:** 25/69/3/1 and 25/69/3/2 everywhere, not eligible, `needs_recompute`. Each receipt's list equals my A2 list, which is 3 load diagnostics plus the integrity diagnostic.
- **Resealed:** under my own rehash each receipt reseals to identical bytes.
- **Refused by the candidate readers at G5 ATTEMPT, but admitted by the 07f readers** (9 resealed variants per receipt):
  - M09 (every diagnostic naming the case);
  - M10 (every non-`RETAINED_PRECISION_*` diagnostic);
  - the first element dropped;
  - the first two swapped;
  - an invocation-level diagnostic appended.
- **Refused by both reader versions:** `[]`, the last element dropped (the typed report diagnostic) and a duplicate.
- **M20 (row method "other")** is G6 `ROW_METHOD_MISMATCH` everywhere. The full table is in `_run_records/probe_table.md`.

**The producer's list.**
- `PP/retained_wire.rs:1424–1428` (`ordinary_value`) filters `env["diagnostics"]` by `affected_refs` containing the case id, excluding codes that start with `RETAINED_PRECISION_`. This is the readers' predicate.
- It runs after `successor_envelope` (`:740–770`), which omits the T1 (a) legacy disclosure and pushes the SELECTED diagnostic, and after `unavailable_envelope` (`:1577–1589`).
- `finish` (`:1436`) only inserts `retained_precision`, so the list is computed over the published diagnostics.
- In the milestone domain the producer is one-case. `serialize_unavailable` is not a publication; it is refused at G3.

**Conclusion.** No legitimate producer output is refused.

## 2. The base repairs

I ran my own deep diff of each 07f base against its 07g base (`_run_records/base_diff.py`, output in `base_repairs_rv90.txt`). All 15 change only these paths:
- `retained_precision.body.ordinary_attempts[i].diagnostic_refs`, for every case (4 two-case bases have two);
- `retained_precision.receipt_sha256`.

The publication hash, source identities, preparation hashes, classifications and every other byte are unchanged.

**Every new list is A2's list, by my own filter over the envelope.**
- The bases carry `RETAINED_PRECISION_SELECTED` and `RETAINED_PRECISION_UNAVAILABLE` diagnostics that name the case, and a model-level diagnostic with `affected_refs: null`. So both exclusions are exercised.
- Each 07f list held only the integrity diagnostic.
- Each 07g list holds 6 or 2 load diagnostics, then the integrity diagnostic, in envelope order.

**The receipt hash is correctly recomputed.** My own implementation of the 07e rehash rule (`_run_records/helpers.py`) reseals all 15 bases to themselves. The 07f bases also reseal to themselves.

The other top-level keys are unchanged: `version`, `provenance` and `arithmetic`, plus mutations 0–267 and all 22 must-pass entries, which are byte-identical in JSON. `d37` is the only new key.

## 3. RV79-N1: the D37 table, derived independently

**My derivation** (`_run_records/rv90_d37.py`, output in `rv90_d37_out.txt`) uses only these sources:
- `PP/retained_receipt.rs:45–54`: `enter`, `completed` and `checked` set Entered, Completed, and Completed or Failed; `fail_entered` turns every Entered stage into Failed. `project` (`:162–164`) refuses any record still Entered.
- `PP/retained_product.rs`:
  - `prepare_owned_case` (`:3309–3433`);
  - `solve_native` (`:3449–3468`);
  - `freeze_candidate` (`:3650–3760`).
- The error kind each exit publishes, from `PP/retained_wire.rs:1206–1236` (`public_error`). This includes D38's `capture` when `capture.native` is None.
- C3's PublicFailure kinds (`R/I52/prepared_public_contract_02/C3_DELTA.md:277`).

**Every exit point and the record it leaves:**

| Exit | Record | Kind |
|---|---|---|
| Any error in `prepare_owned_case` | `F---------` | preparation |
| `solve_native`, a nonselected outcome after `o.native=Some` | `CF--------` | native |
| `solve_native`, an error before any Run (source, origin, duplicate) | `CF--------` | capture (D38) |
| `freeze_candidate` before `enter(ProofStart)`: `bind_rows`, `prepared_specs`, MapWrite (and, unreachably, consumed, missing owner or nonselected) | `CC--------` | capture |
| `begin_prepared_product` | `CCF-------` | proof |
| `draft.project()` | `CCCF------` | proof |
| `prepared_maxima` | `CCCCF-----` | abandoned |
| `complete_maxima` | `CCCCCF----` | values |
| `prepared_alias` | `CCCCCCF---` | abandoned |
| `bind_rows_view` after `completed(Aliases)` (`fail_entered` has nothing to fail) | `CCCCCCC---` | abandoned |
| `certify_final` fails; the verdict copy fails or does not cover the rows | `CCCCCCCF--` | proof |
| `certify_final` fails; Observables and G5a entered and checked | `CCCCCCCF` with `CC`, `CF`, `FC` or `FF` | proof |
| Certified, then `matches_values` is false or the verdict copy fails | `CCCCCCCC--` | capture |
| `!pass` with an observable error | `CCCCCCCCFC`, `CCCCCCCCFF` | observable |
| `!pass`, observables passed, a G5a error | `CCCCCCCCCF` | g5a |
| `!pass` with both checks passed: `numeric_pass` false, or `full_case_passed` false through `error`, adapter fault or `final_calls` | `CCCCCCCCCC` | numeric |
| `pass`, then the commit's CountRange or `adapter.require` fails | `CCCCCCCCCC` | capture |

**Comparison with the corpus `d37`, entry by entry.** All 9 kinds are equal as sets, record for record. My 25-record universe equals `d37.records`. The 6 universe records that no exit leaves (`----------`, `C---------`, `CCC-------`, `CCCC------`, `CCCCC-----` and `CCCCCC----`) appear under no kind. `unknown_kinds` is `["storage"]`.

**One premise is not written down (N4).**
- `capture` at `C---------` (`freeze_candidate` with no `solve_native`) is unreachable only because the facade calls `freeze_candidate` after `solve_native` returns Ok (`PP/lib.rs:2962–2974`).
- The corpus `basis` cites the stage lines but not this call order. The exclusion is stricter, and correct as the producer stands.

**Each reader's test reads the corpus table.**
- Python `test_error_kind_agrees_with_stage_record_d37` (`P/tests/test_retained_precision_contract.py:743`). With `proof=None`, only the D37 check in `_g5_typed` can refuse. The test asserts the refusal code.
- Rust `u6e_reader_round_tests::d37_error_stages_matches_the_corpus_table_rv79_n1` (`RS:4357`) calls `error_stages` directly.
- TypeScript, the test "RV79-N1: D37 agrees with the corpus table…" (`retainedPrecision.test.ts:911`), calls `errorStageRecordAgrees` directly.
- Each test iterates 10 kinds × 25 records and accepts a record exactly when the corpus lists it.

**Evidence that the corpus is the oracle:**
- Widening the corpus (`abandoned` plus `CCCCCCCC--`) fails exactly that test, and only that test, in each of the three readers. So does narrowing it (`abandoned` minus `CCCCCCC---`). These are RV90:ALL01 and ALL02.
- Pointing the Python test's expectations at `rp.ERROR_STAGE_RECORDS` while applying M35 survives the whole suite (RV90:PY04). So the corpus table is what kills M35 and M37, and RV79-N1 is closed.

**`productAttempts` is unchanged.**
- `errorStageRecordAgrees` (`TS:652–655`) is the removed expression moved verbatim, with `a.result.error.kind` and `a.stages` passed in. `productAttempts` calls it at `TS:785`, in the same place.
- The TypeScript outcomes on 07f and 07g are identical between the base and candidate readers, apart from the 6 F5 entries.
- Removing the call from `productAttempts` (RV90:TS03) is killed by the shared `numeric_error_with_checks_not_entered` and by 07f's D37 test. The Rust counterpart (RV90:RS03) is also killed.
- The Python reader's own table is kept and still pinned (`…_d37_reader_table_pins`).
- `PublicError.kind` is closed at G1 by the receipt schema, so an unknown kind such as `constructor` never reaches the object lookup.

## 4. RV80-N2: `integral_receipt`'s scope

**Rust** (`RS:251–290`).
- The function copies and normalizes the statement only when `retained_precision` holds a float, and it normalizes only that member.
- The new in-crate test (`RS:4384`) pins three facts: a receipt float becomes an integer, every other top-level member is byte-identical, and a row's `17.0` stays a float.
- I61's RS05 (normalize the whole statement) is killed by that test alone. None of the other 162 tests, and none of the 311 shared entries, sees it.
- I also looked for statement-to-receipt `Value` equality on numbers in G3–G5 (`basis_ref`, `rows_for`, quality binding). The comparisons are of strings.
- **Conclusion:** no shared gate, code or classification observable distinguishes the scope in Rust. `ValidationError.detail` text is not compared for parity and was not examined.

**Python** (`PY:569–578`, `:1638`).
- `_normalize_integrals(receipt)` runs after G2 and D34, which leave only U or I32 integral values in the receipt.
- A statement-scope widening is observable through the shared corpus: I61's PY_M50 fails 130 tests. I61's RETURN §3 says 197; its own `mutants_py.json` says 130, which is a wording slip only.
- The new monkeypatch test pins the outermost call to the receipt.

**TypeScript** has no normalization.

**The claim, "no shared observable distinguishes the scope in Rust or TypeScript", is sound.** Keeping `integral_float_integers_and_references` as the only shared D32 entry is consistent with it.

## 5. No unintended outcome change

**My own applier and dumps.**
- I applied every corpus entry myself (`_run_records/helpers.py` and `apply_corpus.py`): the edits, `invocation_edits`, the invocation digest, my own 07e rehash, then `after_rehash`.
- I fed each applied document to the base (07f) and candidate (07g) readers in all three languages: Python through `validate_retained_precision`, Rust through `rp::validate` (the review-copy harness `rv90_dump.rs`), and TypeScript through `validateRetainedPrecision` (`rv90_dump.test.ts`).
- Twelve outcome files are in `_run_records/outcomes/`, and the comparison is in `compare_out.txt`.

| Comparison | Python | Rust | TypeScript |
|---|---|---|---|
| 07f reader on 07f against 07g reader on 07g: changed outcomes of 07f entries | 0/305 | 0/305 | 0/305 |
| New 07g entries | 6, each G5 ATTEMPT | 6, each G5 ATTEMPT | 6, each G5 ATTEMPT |
| 07f reader against 07g reader, both on the 07g corpus | only the 6 F5 mutations (07f passes, 07g G5 ATTEMPT) | same | same |
| 07g reader on the unrepaired 07f corpus | 15 bases, 22 must-pass and 136 mutations changed | same | same |
| 07g reader on 07g against corpus `expected` (and `expected_by_reader`) | 0 misses of 311 | 0 | 0 |
| Any `numerical_eligible: true` | none | none | none |

This confirms `OUTCOME_DELTA_07F_07G.json` (which is Python-only) in all three readers, including its `unrepaired_07f_corpus_under_f5` counts of 15, 136 and 22. Classification counts of passing entries are unchanged.

**The flipped tests flip only for F5.** With F5 removed:
- I61:RS01 fails Rust `d6_d7_ordinary_and_diagnostic_relations` exactly at "F5: a listed diagnostic of another scope is refused" (`P/core/reporting/result_export/tests/retained_precision_contract.rs:2430–2435`), with `left: Null`, which is 07f's outcome.
- I61:TS01 fails TypeScript's "D6a as amended by F5…" (`retainedPrecision.test.ts:376`) with `'pass'`.

**N3.** The Rust probe lists `[integrity, other]`. That list is refused under F5 even without `other`, because it is also 07f's relaxed form. The TypeScript probe appends to the exact list, so it isolates the element; the shared `f5_ordinary_refs_list_invocation_level_m10` does too. Coverage holds.

## 6. Parity and suites

**Suites on 07g** (`_run_records/test_runs.txt`):

| Reader | 07g | Base, 07f |
|---|---|---|
| Python | 384 passed (contract 372, schema 12) | 374 |
| Rust `result_export` | 163 passed (12 targets) | 159 |
| TypeScript vitest | 444 passed | 436 |
| TypeScript `tsc --noEmit` | exit 0 | — |

**Every reader asserts the corpus outcome on every entry:**
- Python `test_shared_draft_first_failure_controls` (parametrized over mutations), the must-pass test and the bases;
- Rust `shared_rehashed_first_failure_mutations`, `shared_must_pass_entries_validate` and the slice tallies, now including `snapshot_07g_mutation_outcomes` for 268–273;
- TypeScript, one named test per mutation and per must-pass entry.

I61's F5-removed mutants are killed by each of the six F5 entries in every reader.

**Cross-reader parity on 07g.** Every entry agrees, except the ruled per-reader G7 entry `g7_maximum_off_enclosure`.

**The completeness flags are unchanged and false:**
- `_IMPLEMENTATION_COMPLETE = False` (`PY:30`);
- `IMPLEMENTATION_COMPLETE: bool = false` (`RS:4269`);
- `SUMMARY_COVERAGE_COMPLETE = false` (`TS:97`).

**S1, the F5 parity on non-array `affected_refs`.**
- Before G7, no reader validates envelope diagnostics against the envelope schema.
- Python's `name in (d.get("affected_refs") or [])` (`PY:914`) behaves differently by type: a string is a substring test, an object is a key test, and a number raises `TypeError`, which falls back to G5 `PRODUCT_ATTEMPT_MISMATCH`.
- Rust's `list()` (`RS:4142`) and TypeScript's `Array.isArray` (`TS:1265`) treat any non-array as naming nothing.

I set the first load diagnostic's `affected_refs` to the case id as a string, to `5` and to `{case: 1}`, each with the id listed and unlisted, and resealed:

| Probe | Python 07g | Rust 07g | TypeScript 07g | All three 07f |
|---|---|---|---|---|
| string, listed | G7 | G5 ATTEMPT | G5 ATTEMPT | G7 |
| string, unlisted | G5 ATTEMPT | G7 | G7 | G7 |
| number, listed | G5 PRODUCT_ATTEMPT | G5 ATTEMPT | G5 ATTEMPT | G7 |
| number, unlisted | G5 PRODUCT_ATTEMPT | G7 | G7 | G7 |
| object, listed | G7 | G5 ATTEMPT | G5 ATTEMPT | G7 |
| object, unlisted | G5 ATTEMPT | G7 | G7 | G7 |

The G7 codes are the readers' accepted per-language G7 codes (Rust `ARRAY_INVALID`, Python and TypeScript `EVIDENCE_INVALID`).

- The base readers agree at the gate on all 6 probes. The candidate readers diverge on all 6, 5 at the gate and 1 in the code (number, listed), and Python is the odd reader out each time. Rust and TypeScript agree with each other on all 6.
- TypeScript's mutant without the `Array.isArray` guard (RV90:TS04) reproduces Python's behaviour and survives the suite. No entry pins the semantics.
- A related probe, the typed integrity diagnostic with a string `affected_refs`, already diverged at base (Rust G5, Python and TypeScript G7). In the candidate, Python alone stays at G7.
- **Remedy:**
  - Change `PY:914` to `isinstance(d.get("affected_refs"), list) and name in d["affected_refs"]`. With that change applied in my mutant lane, every probe agrees across the three readers at gate and code (G7 codes aside), and the 07g corpus outcomes are unchanged (0 of 311; `out_pymut_remedy_07g.json`, `out_pymut_PY_list_only_membership_remedy_probes.json`).
  - Add a shared mutation (for example, string `affected_refs`, listed) to pin it.

These inputs are malformed and refused by every reader in every version. So under D36 S1 is tracked, not gating. But it is a parity regression this candidate introduces, in the check under review.

## 7. Mutants

**Method.**
- Each mutant is one textual edit, applied in a pristine lane made by `git archive 5e1e2625ac` (`WT/rv90/mut`).
- Each runs that language's full suite: Python's two contract files, Rust's whole `result_export`, or TypeScript's `retainedPrecision.test.ts`.
- The pristine bytes are restored and compared after every run.
- No mutant failed to compile or collect. The runner and the per-mutant results are `_run_records/mutants.py` and `mutants/*.json`.

**I61's 22, re-run: 22 of 22 killed.**
- Python 12: PY01–PY04, M33–M39 and PY_M50.
- Rust 5: RS01–RS05.
- TypeScript 5: TS01–TS05.
- The Python kill counts match I61's exactly. For Rust and TypeScript, I61 recorded failing lists truncated to 8; mine contain those tests.
- RS03, RS04, M35 and M37 are killed by the corpus-table D37 tests. RS05 is killed only by the RV80-N2 test.

**RV90's own: 14 mutants, 18 runs.**

| Mutant | Result | Notes |
|---|---|---|
| PY01, RS01, TS02: F5 for case 0 only | **survive** (all 3) | **N1.** Non-equivalent: each admits my `second_case_swapped` and `second_case_relaxed_form`, which every pristine reader refuses |
| TS01: F5 length check dropped | **survives** | **N2.** Admits `envelope_reordered_strict_prefix` |
| RS02: F5 as `exact.starts_with(refs)` | **survives** | **N2.** Same probe |
| TS04: F5 `Array.isArray` guard dropped | **survives** | **S1.** Matches Python's semantics; unpinned |
| PY03: D6a's unique-and-resolve check removed | survives | Equivalent: F5 together with G4's unique ids implies D6a |
| PY04: Python D37 test fed from `rp.ERROR_STAGE_RECORDS`, plus M35 | survives | Designed control: without the corpus table, M35 survives |
| PY02, RS04: exclusion only of `RETAINED_PRECISION_SELECTED` | killed | Bases with an UNAVAILABLE diagnostic |
| RS03, TS03: D37 not enforced in the product pass | killed | Shared `numeric_error_with_checks_not_entered`, 07f's D37 tests |
| ALL01: corpus `d37` widened | killed (all 3) | Only by each reader's corpus-table D37 test |
| ALL02: corpus `d37` narrowed | killed (all 3) | Only by each reader's corpus-table D37 test |

**The probe for N2.** I moved a naming load diagnostic after the integrity diagnostic in the envelope and resealed.
- The exact list passes in all six readers.
- The list without its last element passes the 07f readers and is refused by every 07g reader.

**Remedy for N1 and N2.** Add two shared mutations: a second case's list in 07f's relaxed form, and the strict prefix above.

## For ROOT

1. **S1: is it repaired now, or tracked?**
   - The fix is one line in Python plus one shared mutation. It changes no 07g outcome and restores gate-level parity on every probe.
   - Under D36 it is not gating. But it is a parity regression in the check this round introduces, so I recommend repairing it in this round or in U6f.
2. **N1 and N2: should their shared mutations be a U7 condition, as RV79-N1 was?** F5 sits on the eligible path, and today nothing in the corpus would catch a regression that skips later cases or accepts a strict prefix.
3. **N4: should the corpus `d37.basis` record the facade-order premise?** This would be a wording addition in the next snapshot, with no outcome change.

## Host and method disclosures

**Host.**
- **Memory guard:** `WT/guard/memguard.sh`, PID 5387, was running throughout. Each Rust mutant run checked it first.
- **Cargo:**
  - Default toolchain (cargo and rustc 1.97.1), `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`.
  - One cargo job at a time. Targets are under `WT/targets/rv90/` (`canonical_json`, `units`, `re_cand`, `re_base`, `mut`).
- **Python:**
  - The checked-JSON (`openpipestress_jcs_ijson`, `openpipestress_jcs_binary64`) and units (`openpipestress_units`) CLIs were built from the candidate archive with `--locked --offline --release` and the `checked-cli` and `cli` features.
  - The interpreter is the existing project venv (Python 3.13.14, pytest 9.1.1).
  - `TMPDIR` pointed into `WT/scratch/rv90_u6e/`.

**TypeScript.**
- **Disclosure:** `P/node_modules` in each review copy is a symlink to REPO_ROOT's `P/node_modules`. The prebuilt `apps/desktop/public/wasm-engine` and `self-weight-engine` were **copied** from `WT/f2a-readers`.
  - The engine sha256s are `78432972…` for the operation applier and `3bc83f88…` for self-weight.
  - Nothing was built or installed.
- A real, empty `apps/desktop/node_modules` was created in each copy so that the vite caches stay in the copy. REPO_ROOT's `node_modules/.vite-temp` was not modified.
- The tools are node v24.18.0, vitest 4.1.10 and tsc 5.9.3.

**Review-copy harnesses.** These files were added only to `WT/rv90/{cand,base}`, never to the mutant lane during suite runs:
- `tests/rv90_dump.rs` and `src/features/results/rv90_dump.test.ts`. Both copies are in `_run_records/`.
- They validate pre-applied JSONL documents and were run separately from the suites. The suite counts above come from runs made before the harnesses were added, or without them.

**Writes.** Only these locations were written:
- `WT/rv90/` (cand, base, mut);
- `WT/targets/rv90/`;
- `WT/scratch/rv90_u6e/`;
- this folder.

Nothing went to the system temp directory.

**Not done.** No Git writes or index operations: Git reads only, with `GIT_OPTIONAL_LOCKS=0`. No installs, no new tooling, and no solver, native or DEC-025 jobs.

**Copies.** The copies and targets are deleted after this report (see the return).

**Time.** 2026-10-04, from 03:24 to about 04:15 local (MDT).

## Records (`_run_records/`)

**Scripts:**
- `helpers.py`: RV90's own edit and rehash applier.
- `apply_corpus.py`, `py_validate.py`, `rv90_dump.rs`, `rv90_dump.test.ts`: application and the three validators.
- `compare.py`, `base_diff.py`, `rv90_d37.py`.
- `apply_probes.py`, `apply_probes2.py`, `apply_probes3.py`, `probe_table.py`.
- `mutants.py`, `mutsum.py`, `ts_mut_probe.py`, `rs_mut_probe.py`, `py_mut_probe.py`.

The applied JSONL documents (about 38 MB each) are not kept. `apply_corpus.py` regenerates them deterministically.

**Outputs:**
- `compare_out.txt`, `compare_report.json`;
- `base_repairs_rv90.txt`, `rv90_d37_out.txt`;
- `probe_table.md`, `test_runs.txt`;
- `outcomes/*.json`: per-reader, per-version outcomes for 07f, 07g and the probes, plus the mutant probes;
- `mutants/*.json`.

`SHA256SUMS` covers every file in this folder.

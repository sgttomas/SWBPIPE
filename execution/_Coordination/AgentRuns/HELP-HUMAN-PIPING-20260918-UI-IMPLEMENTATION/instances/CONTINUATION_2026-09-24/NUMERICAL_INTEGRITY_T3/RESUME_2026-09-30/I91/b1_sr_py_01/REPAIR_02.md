# I91 B1 SR-PY, repair 02: the three-reader alignment set (items 1–3), RV113's S-4, N-1, N-2 and S-2; item 4 held under the stop rule

TASK (Type 2), I91 (I-PY), for ROOT (HELP_HUMAN, Agent 0), the return path. No delegation. 2026-10-08 UTC.

**Brief, verified before use:** `R/BRIEFS/B1_SR_PY_REPAIR_02.md`, sha256 `6fcb24e431b067b8680fce9ca77e695c2f0630a680d7d2b6f31e05e64ad793e3`. Its rules and B1_COMMON's and B1_SR_PY's still hold. **Read:** RR "RV113's three returns verified; I3 made at `2ba2f81863`; the three-reader alignment set ruled" (items 1–6, the specification); RV113's SR-PY review `R/REVIEW_RV113/rvr_sr_py_01/REVIEW.md` (`d8611e59…`), with its `evidence/fg/`, `evidence/probes/`, `evidence/repair_probes/`, `evidence/census/` and harness; RV113's SR-TS review `rvr_sr_ts_01/REVIEW.md` (`d44dec19…`): S-1, N-1, N-2. To mirror TS's placements I read, without editing, TS's `coverage` and `ordinaryAttempts` and Rust's `g8` at I1.

**Placeholders** as in `RETURN.md`: WT, NUM, P, PY, T, R, RR, VENV; `S` = `WT/scratch/i91_b1_sr_py`; `R01` = `11cc14e3e6` (repair 01's head, the baseline); `HEAD` = `70d4a68bd7`.

## 0. Summary

- **Items 1, 2 and 3 are in PY,** each at its ruled gate and code, with tests and mutants killed by assertions. On RV113's probes PY now gives TS's present verdict on every item-1 and item-3 probe, bound and unbound (TS already has both), and the ruled verdict on every item-2 probe (TS and RS move in their own rounds).
- **Items 5 and 6 (S-4, N-1, N-2)** are done: Q17's own-attempt row; the docstring states PY's transport gate; a stage-rule test pins the (4b) conjuncts held elsewhere.
- **Item 7 (S-2)** is corrected in §6.
- **Item 4 is held, not committed (§5).** Relabelling the transport header check from G7 to G2 changes the transport verdict's gate of nine 07m mutations (277 and 286–293), with the code, the detail and the bound and unbound verdicts unchanged. The brief allows no change to any 07m verdict and says to stop and return if one would change. The change is exactly what item 4 rules, so it is returned as a ready, tested patch for ROOT to rule on.
- **The census over 07m at HEAD:** 339 entries × 3 verdicts (bound, unbound, transport) through RV113's harness: **0 changes against R01 and 0 misses** against the corpus's Python expectations. My own census agrees (0 of 1,017 outcomes differ).
- **Suites against R01:** 1,937 → 1,942 passed, 30 skipped, rc 0: 5 tests added, 0 removed, 0 changed in outcome.
- **Mutants:** 30 in all. Each new or moved check has one, and so do each C2 branch, Q17, each (4b) conjunct and repair 01's count. **29 are killed, every one by an assertion.** The 30th (F-src-kind) cannot be killed, because the schema's own constant refuses first (§11.3). N0 passes, and PRE (the head tests on R01's reader) fails exactly the 7 new or changed tests.

## 1. Head and commits

**HEAD = `70d4a68bd78dae3fd60196e1085bd48944e1cbf4`** on `codex/piping-t3-b1-p-20261007` in `WT/b1-p`, three commits on R01 (`11cc14e3e65363790f43943b09933a57f4da28e4`), each ending with the agent trailer. Not pushed: ROOT pushes.
- `a721e58483`: items 1–3; S-4 and N-2's tests; the rows whose first failure the placements move.
- `5f267db91d`: N-1, the transport docstring (wording only).
- `70d4a68bd7`: the owner-out-of-range row asserts G3's own refusal (D16).

The diff (`diff/repair_02.diff`, R01..HEAD): `retained_precision.py` +47 −6; `tests/test_retained_precision_contract.py` +247 −7. No other file. Item 4 is not in HEAD (§5).

## 2. Item 1: the receipt's own references at G3, the ordinary attempt's basis at G5

**The change** (`a721e58483`):
- **G3 COVERAGE, bound and unbound** (`_validate_draft`'s G3, after D29), as TS's `coverage` block: each `sources[si].index == si`; each source's owner a case, its index an integer in range, and the receipt's case id at that index equal to the owner's `case_id`; each `material_bases[mi].index == mi` (**new: PY had no check of it at all**); each basis's `case_indices` unique and each below the case count.
- **G8 drops what moved:** the per-source `s["index"] == si and s["owner"]["kind"] == "case"` and the owner-id comparison. G8 keeps the invocation's facts: the selectors and step 2, the exact case lists by selector (repair 01's (c)), the materials (repair 01's (e)), and each source's binding to its request case (`pressure_regions`, `equivalent_static`, maps, sections, …).
- **G5 ATTEMPT_MISMATCH, the ordinary class's reference rule** (`_g5_ordinary`, first per case, as TS's `ordinaryAttempts`): an ordinary attempt's `material_basis_ref` resolves to a basis whose `case_indices` lists its case.

**Tests:** `test_b1_repair02_receipt_references_at_g3_and_the_ordinary_basis_at_g5` (RV113's (f) probes and four more; bound and unbound; the owner out of range is G3's own refusal, D16) and the (b)/(c) rows of repair 01's test, re-expected (§6).

## 3. Item 2: the model scope at G8 INVOCATION

**The change** (`a721e58483`, `_g8`'s model-scope check, after the invocation shape, digest and project, before any PREPARATION check): `model.get("pressure_contract") is None`, `model.get("combinations", []) == []`, `model.get("components", []) == []` and `"reference_configurations" not in model` (RV113 §10.2's three expressions; the member test refuses null too).

**Test** `test_b1_repair02_model_scope_at_g8_invocation`: `reference_configurations` null and `[]`; `pressure_contract` `{}` and `false`; `combinations` null, `{"x": 1}` and `[{}]`; `components` `"x"` and null: each G8 INVOCATION. `pressure_contract` null, `combinations` `[]` and `components` `[]`: admitted (eligible).

## 4. Item 3: C2's cause table at G5 ATTEMPT

**The change** (`a721e58483`, `_g5_ordinary`, after the `not_required` and `selected` rules): for every unavailable case whose cause is not `prepared_product_failure`, exactly the ruled table (`RECEIPT_FAILURE_CODES`, `PRECONDITION_CODES`):
- `source_error`: phase `preparation`, code `source_unavailable`, no Run, and a `source_decline` whose error equals the cause's;
- `unavailable_precondition`: phase `routing` or `preparation`, no Run, and the code keyed by `precondition` (`caller` → `caller_not_qualified`; `resource_admission` → `resource_admission_not_available`; `upstream_no_wrap` → `upstream_no_wrap_not_established`; `capture`, `source_family` → `source_unavailable`);
- `receipt_failure`: phase `receipt`, code one of `receipt_encoding`, `publication_hash_range`, `invocation_not_representable`;
- `facade_failure`: phase `facade`, code `facade_certificate`, a Run whose terminal is `selected`, and `owner_ref` the case itself;
- any other cause (a kernel reason): phase `kernel`, a Run, code `kernel_<terminal kind>`, and the cause equal to the Run's terminal reason.

**Tests:** `test_b1_repair02_c2_cause_table_reader_logic` (`_g5_ordinary` alone: every branch satisfied, with all four precondition keys in both phases and all three receipt codes; then 22 single breaks, each G5 ATTEMPT) and `test_b1_repair02_c2_cause_table_in_the_reader` (RV113's six `c2_*` probes through the whole reader: the receipt branch satisfied is admitted, `needs_recompute`; broken, G5 ATTEMPT; the facade branch satisfied reaches D19, G5 PRODUCT_ATTEMPT; broken, G5 ATTEMPT). The D38 test's "cause a `receipt_failure`" row moves from G5 PRODUCT_ATTEMPT to G5 ATTEMPT (I92's variant).

## 5. Item 4: held

**The patch** (`_run_records/repair_02/item4/item4_held.diff`, sha256 `357098ba93dfe3a3972477f83ede8c20eab4ad634807f36e1347b121a4d54682`, applies cleanly to HEAD): `_transport_g7` becomes `_transport_base`. It still calls `_source_contract(projection, check_receipt=False)`, which runs the header check and then the preview-physics metadata check. A failure with `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID` (the only code the metadata check raises; the header check never raises it) or with the no-code fallback `SOURCE_PREVIEW_PHYSICS_INVALID` is reported at G7. Every other code is reported at **G2**. The docstring states it (N-1 after item 4). The carrier N1 test's transport row expects G2. A new test pins the header at G2 and the metadata at G7 on transport, with both at G7 raw. With the patch applied, those tests pass (8). The four retained-reader test files pass in full (551, `item4/tests_*.log`).

**Its census** (`_run_records/repair_02/item4/census_*`, run with the patch applied): 339 × 3 verdicts, **9 changes against R01, all on transport, all G7 → G2 with the same code and detail**: mutations 277 (`g7_not_required_quality_enum_invalid`) and 286–293 (B6's 07m G7 slice). Bound and unbound are unchanged, and the corpus's Python expectations (all bound) still match (0 misses). On RV113's v5 probes, transport agreement with Rust rises from 93 to 101 of 103 (the other two are the declared N6 classes).

**Why it is held.** The brief: "No change to any 07m verdict is allowed. If one would change, stop and return", and the census evidence it asks for is "0 changes against your repair 01 head `11cc14e3e6` on all three verdicts". Item 4's own ruling moves exactly these nine transport gates (RV113 §10.4 names them), so the two instructions conflict. I did not commit item 4. **ROOT to rule:** either the nine transport relabels are item 4's intended effect (then apply the patch, or have me commit it; the census above is its evidence), or item 4 waits.

## 6. Item 7: S-2, REPAIR_01's (b)/(c) claim corrected

REPAIR_01 §0 and §2 said that (b) and (c) give "the same gate and code in all three readers". **That was wrong for three of its own inputs.** RV113's run (`evidence/repair_probes/RP_TABLE.json`), each reader's first failure, bound:

| Input | PY at R01 | RS (`b5cb7faaeb`) | TS (`7e47e51b5d`) | PY at HEAD |
|---|---|---|---|---|
| (b) 07j's `not_required` case's `material_basis_ref` 7 | G8 PREPARATION | G8 PREPARATION | **G5 ATTEMPT** | G5 ATTEMPT |
| (c) its basis omitting case 1 | G8 PREPARATION | G8 PREPARATION | **G5 ATTEMPT** | G5 ATTEMPT |
| (c) the sourceless case's basis missing (`11cc14e3e6`'s pin) | G8 PREPARATION | G8 PREPARATION | **G5 ATTEMPT** | G5 ATTEMPT |

Unbound, PY at R01 and RS admitted all three, and TS refused them at G5 ATTEMPT. TS's ordinary class (`ordinaryAttempts`: `fail(b.material_bases[a.material_basis_ref]?.case_indices.includes(ci))`) refuses them before G8. **The other repair-01 inputs** ((b) beside a second basis, (c) out of order and an extra empty basis, (d1), (d2), (e)) do give the same gate and code in all three readers. **Since item 1,** PY gives G5 ATTEMPT for the three, as TS does, bound and unbound; RS moves in I90's round. SC's 07n (b) and (c) entries wait for this set, as RR item 5 says.

## 7. Items 5 and 6: S-4, N-1, N-2

- **S-4** (item 5): `test_b1_g5_not_required_admits_a_w2_published_case` gains RV113's `nr_product_attempt_ref_with_attempt` row: 07j's edits keeping attempt 1, with case 1 naming it. G3 passes, and the rule's null-attempt conjunct is the first failure: **G5 ATTEMPT**, as Rust and TS. Q17 dies at it (§9).
- **N-1** (item 6): the docstring of `validate_retained_precision_transport` now states that PY reports its base step at G7, and that item 4 moves the header check to G2 (`5f267db91d`). The held patch rewrites it to the post-item-4 statement.
- **N-2** (item 6): `test_b1_d38_stage_rule_refuses_each_conjunct_held_elsewhere` calls `_g5_stages` as the reader does, on the admitted (4b) pair, and breaks each conjunct that other checks also hold (the case's Run, the capture kind, the reason's code and phase, the case's status, the cause's kind, the cause's attempt): each is refused by `_g5_stages` itself, G5 PRODUCT_ATTEMPT. So those conjuncts are pinned where the reader applies them, not only through the predicate's boolean unit test.

## 8. The census and the probes

**The census** (`census/head_70d4a68bd7/`). It ran RV113's harness, as published, on `S/head5`: the archive of HEAD without `execution/_Coordination`. It ran as one job under the lock, with my own builds. The corpus is 07m (sha256 `c21112fd…6807`): 17 bases, 294 mutations and 28 must-pass entries. Each entry was read bound, unbound and through the transport validator.
- **Against R01: 339 × 3 = 1,017 verdicts, 0 changes.** The baseline is RV113's published PY census at R01 (`evidence/census/py_head.jsonl`), compared entry by entry, detail included.
- **Against the corpus: 0 misses.** The bound verdict was checked against each mutation's `expected_by_reader.python` (else `expected`), each must-pass entry's eligibility and each base's.
- **My own census agrees.** `census_i91.json` uses the contract test's `apply_entry`. 0 of its 1,017 outcomes differ from R01's run (`compare_i91_census.txt`).
- **At `a721e58483`** (`census/a721e58483/`), the result is the same: 0 changes and 0 misses.

**RV113's probes** (`probes/`). `compare.json` holds each row against PY at R01, RS and TS as RV113 ran them. `inputs.txt` traces every input to RV113's published file by sha256.
- **v5 (103 probes): 5 change, bound and unbound, each to G5 ATTEMPT, which is TS's verdict.**
  - Previously G5 PRODUCT_ATTEMPT: `x_reason_cause_receipt_failure` and `c2_facade_phase_kernel`.
  - Previously admitted: `c2_receipt_phase_kernel`, `c2_receipt_code_facade` and `c2_receipt_phase_preparation`.
  - Bound and unbound now equal TS on 103 of 103, and RS on 96.
  - Transport is unchanged (103 same), since item 4 is held.
- **fg (15): the seven (f) rows are now G3 COVERAGE, bound and unbound, which is TS's verdict.** At R01, bound, five gave G8 PREPARATION and `f_mb_index_1` and `f_mb_index_swapped` were admitted. Unbound, all seven were admitted.
- **fg, the eight (g) rows, bound:**
  - G8 INVOCATION: `reference_configurations` null and `[]`, `pressure_contract` `{}` and `false`, `combinations` null and an object, and `components` a string.
  - Admitted, as ruled: `pressure_contract` null.
  - TS still admits five of them (`reference_configurations` null and `[]`, `pressure_contract` `false`, `combinations` null and an object). RS admits three (`combinations` null and an object, `components` a string). Each moves in its own round.
  - Unbound there is no invocation and no G8, so every reader admits all eight.
- **rp (17): three rows are G5 ATTEMPT, bound and unbound, which is TS's verdict:** `r_b_basis_ref_7`, `r_c_basis_omits_case_1` and `r_c_missing_sourceless_basis`. At R01 and in RS they were G8 PREPARATION bound and admitted unbound. The other 14 rows are unchanged and equal in all three readers.
- **Transport is unchanged on all three sets.**

## 9. Mutants

`scripts/mutants_r02.py` runs on `S/mut`, a copy of `S/head5`. Each mutant makes one textual edit, asserted unique, then restores the file byte for byte (`mut` equals `head5` afterwards).
- **Test scope:** each mutant runs the B1, repair and N1 tests of both retained test files (`-k "b1_ or rv108_n1"`, 20 tests). N0 and PRE run both files in full (468 tests).
- **How the runs were held:**
  - N0 and F-mb-index ran first, each under `lockf -k WT/guard/cargo_job.lock`.
  - After ROOT's slot note I stopped my own lane between runs. The rest each ran as one `WT/tools/t3_slot.sh` job. The script's only change is that wrapper.
- **Kill evidence:** `mutants/kinds.txt` (`scripts/mutant_kinds.py`) reads every junit report. **Every kill is a `failure` ending in an `AssertionError` at a line of the test file. No kill is an error or a crash.**

| Item | Mutant | Killed by |
|---|---|---|
| 1, G3 | F-mb-index: a basis's index unchecked (new) | `repair02_receipt_references…` |
| 1, G3 | F-mb-unique: a basis's case list may repeat a case | `repair02_receipt_references…` |
| 1, G3 | F-mb-range: a basis's case list may name a case out of range | `repair02_receipt_references…` |
| 1, G3 | F-src-index: a source's index unchecked (moved from G8) | `repair02_receipt_references…` |
| 1, G3 | F-src-kind: a source owner's kind unchecked (moved from G8) | **not killable:** `CaseSource.owner.kind` is `const "case"`, so G1 refuses any other kind first |
| 1, G3 | F-src-range: an owner index out of range unchecked | `repair02_receipt_references…` (D16: the fallback's refusal carries a cause, so the explicit-refusal assertion fails) |
| 1, G3 | F-src-id: an owner's `case_id` unchecked (moved from G8) | `repair02_receipt_references…` |
| 1, G5 | G5-basis-off: the ordinary attempt's basis reference unchecked | `repair02_receipt_references…`, both repair-01 (b)/(c) and (e) tests |
| 1, G5 | G5-basis-resolve: the basis need only resolve, not list its case | `repair02_receipt_references…`, repair 01's (b)/(c) test |
| 2, G8 | G-refconf: `reference_configurations` unchecked | `repair02_model_scope…` |
| 2, G8 | G-pressure-falsy, G-combinations-falsy, G-components-falsy: each checked by falsiness (the pre-repair rule) | `repair02_model_scope…` (each) |
| 3, G5 | C2-off: the table removed | `repair02_c2_…reader_logic`, `repair02_c2_…in_the_reader`, the D38 capture test |
| 3, G5 | C2-source-decline, C2-receipt-phase, C2-facade-run, C2-facade-owner, C2-precondition-set (TS's unkeyed set of four), C2-precondition-run, C2-kernel-code, C2-kernel-cause: one branch's conjunct each | `repair02_c2_…reader_logic` (each) |
| 3, G5 | C2-receipt-code | `repair02_c2_…reader_logic`, `repair02_c2_…in_the_reader` |
| 5 (S-4) | **Q17:** `not_required` without its `product_attempt_ref` null conjunct | `g5_not_required_admits_a_w2_published_case` (S-4's row) |
| 6 (N-2) | D38-capture, D38-reason, D38-names-a, D38-case-status, D38-no-run-no-proof: each (4b) conjunct dropped | `d38_reader_logic_names_every_conjunct` **and the new `d38_stage_rule_refuses_each_conjunct_held_elsewhere`** (each). Before N-2, only the predicate's unit test killed them. |
| repair 01 (c) | R-c-count: the basis count unchecked | repair 01's (b)/(c) test, at its new explicit-count assertion (D16) |

- **N0** (head): 468 of 468 pass.
- **PRE** (the head tests on R01's reader, a baseline, not a mutant) fails exactly 7 tests, each new or changed in this repair:
  - the D38 capture test;
  - both repair-01 tests;
  - `receipt_references`, `model_scope`, `c2_…in_the_reader` (assertions);
  - `c2_…reader_logic` (an `AttributeError`: R01 has no `PRECONDITION_CODES`).

  The other 461 pass, so nothing else in the tested scope depends on repair 02.
- **Item 4's own tests** are in the held patch, so they have no HEAD mutant. I ran them on `S/mut` with the patch applied, then reverted the patch (`mut` equals `head5` again).
  - The N1 and item-4 tests: 8 passed (`item4/tests_n1_and_item4.log`).
  - The four retained-reader files in full (contract, carriers, schema, dispatcher): 551 passed, rc 0 (`item4/tests_retained_files.log`).

## 10. The Python suites against R01

I83's Python set (27 files, `scripts/suites.sh`) ran on `S/head5` under the lock, with my own builds.
- **Result: 1,942 passed, 30 skipped, rc 0** (`suites/py_r02.*`; the junit's `hostname` attribute removed).
- **Against R01's run (`repair_01/suites/py_r01b`, 1,937 passed, 30 skipped): +5 tests, 0 removed, 0 changed in outcome** (`py_compare_r02.*`).
- **Added:**
  - `test_b1_d38_stage_rule_refuses_each_conjunct_held_elsewhere`;
  - `test_b1_repair02_receipt_references_at_g3_and_the_ordinary_basis_at_g5`;
  - `test_b1_repair02_model_scope_at_g8_invocation`;
  - `test_b1_repair02_c2_cause_table_reader_logic`;
  - `test_b1_repair02_c2_cause_table_in_the_reader`.
- **Changed in content, same id, passing at both revisions:**
  - `test_b1_d38_capture_before_any_run_beside_a_selected_case` (the `receipt_failure` row, §4);
  - `test_b1_g5_not_required_admits_a_w2_published_case` (S-4's row, §7);
  - `test_b1_repair01_g8_material_basis_of_every_case_and_exact_case_indices` and `test_b1_repair01_g8_every_material_basis_has_its_materials_checked` (re-expected, §6).

## 11. For ROOT

1. **Rule on item 4 (§5).** The patch is ready and tested; its only 07m effect is the nine transport gate relabels the ruling intends.
2. **For SC (07n):** the item-1, item-2 and item-3 probes in my tests and RV113's are ready inputs. With PY at HEAD, PY equals TS's present verdict on every item-1 and item-3 probe; on item 2, PY gives the ruled verdict, which TS reaches after I92's round and RS after I90's.
3. **A schema-equivalent check:** the moved `s["owner"]["kind"] == "case"` conjunct is also the schema's constant (`CaseSource.owner.kind`), so G1 refuses any other value first and its mutant cannot be killed (§9). It is kept for parity with TS's `coverage`.

## 12. Host

- **Cargo:** the three CLI authorities (`openpipestress_jcs_ijson`, `openpipestress_jcs_binary64`, `openpipestress_units`) built through `WT/tools/t3_cargo.sh` into `WT/targets/i91-b1-sr-py/`; every Python job set `OPENPIPESTRESS_CHECKED_JSON_BIN`, `OPENPIPESTRESS_BINARY64_JSON_BIN` and `OPENPIPESTRESS_UNITS_BIN` to them. Sha256 in `_run_records/repair_02/host/`.
- **Heavy jobs, each its own hold, one of mine at a time:**
  - under `lockf -k WT/guard/cargo_job.lock` (slot 1): the three census-and-probes jobs, the suite run, and the first two mutant runs (N0, F-mb-index);
  - after ROOT's slot note, through `WT/tools/t3_slot.sh`: the remaining 29 mutant runs and PRE, and the two test runs with item 4's patch applied, each pytest its own slot job (`host/lock_log.txt` lists the mutant runs and the builds).
  - Quick single-file test runs used my own builds.
- **Waits:** bounded, one per job, each ending when the job's process had gone; none left.
- **Records:** placeholder paths only (every file copied through `scripts/sanitize.py`). The junit XML has its `hostname` attribute removed. There is no symlink and no `build` folder. Every file was screened, gzipped files decompressed, for the strict pattern (the four host-path forms the brief names), the machine's three host-name forms and any junit `hostname` attribute: 0 hits. This record names none of the host forms literally.
- **Not done:** no DEC-025, install, or other agent's job touched. Commits on my branch only.
- **Cleanup:**
  - deleted: `S/head5` and `S/mut` (the tree copies), `WT/targets/i91-b1-sr-py/` (the three builds; their sha256 are in `host/binaries.txt`) and the temporary folders under `S/tmp/`;
  - kept: `S` holds only scripts, logs and run outputs (23 MB);
  - `WT/b1-p` is clean at HEAD, and no process or wait of mine remains.

## 13. Records

`REPAIR_02.md` and `SHA256SUMS.repair_02` (over `_run_records/repair_02/`). Earlier records unchanged.
- `diff/`: commits, diffstat, `repair_02.diff` (R01..HEAD).
- `census/head_70d4a68bd7/` and `census/a721e58483/`: RV113's harness census (`census_rv113.jsonl`), my census (`census_i91.json`), the comparisons (`compare.txt`, `compare_i91_census.txt`; `a721e58483/compare.json`), the job's log and stamps.
- `probes/`: RV113's three probe sets through PY at HEAD (`probes_v5.jsonl`, `probes_fg.jsonl`, `probes_rp.jsonl`). `compare.json` holds the full report: the census part, and each probe against PY at R01, RS and TS. `inputs.txt` gives the published RV113 inputs and their sha256.
- `item4/`:
  - the held patch (`item4_held.diff`);
  - with the patch applied: its census, probe runs and comparisons;
  - the two test logs.
- `suites/`: `py_r02.xml.gz` (hostname removed), `py_r02.tail`, `py_r02.rc.txt`, and `py_compare_r02.{txt,json}` (against R01's run).
- `mutants/`: `mutants_r02_part1.{jsonl,log}` (N0, F-mb-index), `mutants_r02_part2.{jsonl,log}` (the rest and PRE), and `kinds.txt` (each failing test and its failure kind).
- `scripts/`: `mutants_r02.py`, `mutant_kinds.py`, `job_r02.sh`, `r02_compare.py`, `census.py`, `census_compare.py`, `suites.sh`, `compare_suites.py`, `build_bins.sh` and `sanitize.py`. RV113's harness is used as published (`R/REVIEW_RV113/rvr_sr_py_01/evidence/harness/rv113_py_harness.py`; `probes/inputs.txt`).
- `host/`: the binaries' sha256 (`binaries.txt`) and my lock-log lines (`lock_log.txt`).
